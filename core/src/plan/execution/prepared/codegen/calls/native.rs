use super::local::values;
use super::shape::{CallFunction, CallLocal};
use super::{
    CallContractTarget, CallFamily, CallGroupCodegen, CallTarget, Code, ExecutionGraphProfile,
    Rust, target_family, target_id,
};
use crate::plan::HostCallSite;

impl CallFamily {
    pub(super) fn native_prefix(self) -> &'static str {
        match self {
            Self::Custom => "custom_native",
            Self::Tuple => "tuple_native",
            _ => "native",
        }
    }
}

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn is_native(&self, target: CallTarget, arguments: &[CallLocal]) -> bool {
        self.native_targets.contains(&target.key())
            && arguments.iter().all(CallLocal::native_argument)
    }

    pub(super) fn native_families(&self) -> Vec<CallFamily> {
        [CallFamily::String, CallFamily::Custom, CallFamily::Tuple]
            .into_iter()
            .filter(|family| {
                self.functions.iter().any(|function| {
                    let calls = function.shape.calls.iter().any(|call| {
                        let CallContractTarget::Static(target) = call.target else {
                            return false;
                        };
                        target_family(target) == *family && self.is_native(target, &call.args)
                    });
                    calls
                        || function.shape.tails.iter().any(|tail| {
                            target_family(tail.target) == *family
                                && self.is_native(tail.target, &tail.args)
                        })
                })
            })
            .collect()
    }

    pub(super) fn has_native(&self) -> bool {
        !self.native_families().is_empty()
    }

    pub(super) fn write_native_request(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        target: CallTarget,
        site: &HostCallSite,
        arguments: &[CallLocal],
        caller: &str,
    ) {
        let result = format!(
            "FunctionStep::{}Native {{ function: {}, site: {}, arguments: {}, caller: {caller} }}",
            target_family(target),
            target_id(target),
            Rust::expression(site),
            values(arguments, true)
        );
        source.push_str(&format!("return {};\n", self.body_result(function, result)));
    }

    // Delivery restores only owned state. Source work and root publication
    // stay in advance, without a result mailbox checked by every ordinary call.
    pub(super) fn write_native_resume(&self, source: &mut Code, family: CallFamily) {
        source.open(&format!(
            "impl {family}NativeExecution for FunctionExecution {{\n"
        ));
        source.open(&format!(
            "fn resume_native(mut self: Box<Self>, value: {}) -> Box<dyn CallExecution> {{\n",
            family.value_type()
        ));
        source.open(&format!(
            "let active = if let Some(caller) = self.{}_caller.take().or_else(|| self.{}.pop()) {{\n",
            family.native_prefix(),
            family.return_stack()
        ));
        source.push_str("caller.small(value)\n");
        source.alternative("} else {\n");
        source.push_str(&format!(
            "FunctionState::{family}NativeComplete {{ value }}\n"
        ));
        source.close("};\n");
        source.push_str(&format!(
            "self.active = Some({});\nself\n",
            self.active("active")
        ));
        source.close("}\n");
        source.close("}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CallCodegen, CallFamily, CallGroupCodegen, Code};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomFieldListEnd, HostCustomIndex0, HostCustomSchema, HostCustomType, HostProfile,
        HostProvider, HostProviderModule, HostProviderSet, HostTupleType, HostTypeList,
        HostTypeListEnd, ModuleSource, PackageSource, StringValue,
    };
    use num_bigint::BigInt;

    #[test]
    fn each_synchronous_native_family_emits_exact_owned_caller_and_root_restoration() {
        struct Profile;
        impl HostProfile for Profile {
            type RunState = Vec<&'static str>;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl HostProvider<Profile> for Profile {
            type State = Vec<&'static str>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        struct MarkerSchema;
        struct Found;
        impl HostCustomSchema for MarkerSchema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "example";
            const NAME: &'static str = "Marker";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = HostCustomConstructorList<Found, HostCustomConstructorListEnd>;
        }
        impl HostCustomConstructorDefinition for Found {
            const NAME: &'static str = "Found";
            type Fields = HostCustomFieldListEnd;
        }
        type Marker = HostCustomType<MarkerSchema, HostTypeListEnd>;
        type MarkerFound = HostCustomConstructorAt<Marker, HostCustomIndex0, Found>;
        type Single = HostTupleType<HostTypeList<BigInt, HostTypeListEnd>>;
        fn mark<'call>(
            mut call: HostCall<'call, Profile, Profile, Marker>,
        ) -> Result<HostCallCompletion<'call, Marker>, HostCallError> {
            call.state().push("mark");
            Ok(call.return_custom::<MarkerFound>(()))
        }
        fn single<'call>(
            mut call: HostCall<'call, Profile, Profile, Single>,
        ) -> Result<HostCallCompletion<'call, Single>, HostCallError> {
            call.state().push("single");
            Ok(call.return_tuple((BigInt::from(7), ())))
        }
        let source = r#"
pub type Marker { Found }
@external(erlang, "native", "mark")
fn mark() -> Marker
@external(erlang, "native", "single")
fn single() -> #(Int)
@external(erlang, "native", "text")
fn text() -> String
pub fn marker() { mark() }
pub fn tuple() { single() }
pub fn main() { let _ = marker() let _ = tuple() text() }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "example",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::new("application", "example")
                .unwrap()
                .with_scoped_function::<Profile, (), Marker, _>("mark", mark)
                .unwrap()
                .with_scoped_function::<Profile, (), Single, _>("single", single)
                .unwrap()
                .with_function::<(), StringValue, _>("text", || "kept".into())
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let execution = hosted.execution();
        let codegen = CallCodegen::new(
            &execution.program.functions,
            &execution.program.common.custom_types,
            &execution.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            codegen.functions.iter().collect(),
            Vec::new(),
            codegen.native_targets.clone(),
        );
        assert!(matches!(
            group.native_families().as_slice(),
            [CallFamily::String, CallFamily::Custom, CallFamily::Tuple]
        ));
        assert!(!group.has_native_calls());
        for (family, expected) in [
            (
                CallFamily::String,
                r#"impl StringNativeExecution for FunctionExecution {
    fn resume_native(mut self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
        let active = if let Some(caller) = self.native_caller.take().or_else(|| self.string_returns.pop()) {
            caller.small(value)
        } else {
            FunctionState::StringNativeComplete { value }
        };
        self.active = Some(active);
        self
    }
}
"#,
            ),
            (
                CallFamily::Custom,
                r#"impl CustomNativeExecution for FunctionExecution {
    fn resume_native(mut self: Box<Self>, value: CallCustom) -> Box<dyn CallExecution> {
        let active = if let Some(caller) = self.custom_native_caller.take().or_else(|| self.custom_returns.pop()) {
            caller.small(value)
        } else {
            FunctionState::CustomNativeComplete { value }
        };
        self.active = Some(active);
        self
    }
}
"#,
            ),
            (
                CallFamily::Tuple,
                r#"impl TupleNativeExecution for FunctionExecution {
    fn resume_native(mut self: Box<Self>, value: CallTuple) -> Box<dyn CallExecution> {
        let active = if let Some(caller) = self.tuple_native_caller.take().or_else(|| self.tuple_returns.pop()) {
            caller.small(value)
        } else {
            FunctionState::TupleNativeComplete { value }
        };
        self.active = Some(active);
        self
    }
}
"#,
            ),
        ] {
            let mut rendered = Code::default();
            group.write_native_resume(&mut rendered, family);
            assert_eq!(rendered.as_str(), expected);
        }
        let mut effects = Vec::new();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut effects, &mut echo).unwrap(),
            crate::Value::String("kept".into())
        );
        assert_eq!(effects, ["mark", "single"]);
        assert!(echo.is_empty());
    }
}
