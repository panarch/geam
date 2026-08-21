use super::super::function;
use super::super::specialization::{
    SpecializationKey, SpecializedFunctionShape, ValueInhabitation,
};
use super::super::{LoweredExecution, LoweringCompletion, LoweringContext, SpecializationOutcome};
use super::{parameter, return_, sealing};
use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::execution::host::{
    HostFunctionTables, HostSpecializationError, HostedExecutionProfile, HostedFunction,
    HostedFunctionMetadata, HostedNeverFunction, HostedValueFunction,
};
use crate::plan::{HostFunctionTemplate, HostImplementationBinding};
use std::collections::HashMap;
use std::sync::Arc;

type HostedLoweredExecution = LoweredExecution<HostedExecutionProfile>;

pub(super) struct HostFunctionRegistry<Profile: HostProfile> {
    functions: HashMap<crate::plan::FunctionTemplateId, RegisteredHostFunction<Profile>>,
}

struct RegisteredHostFunction<Profile: HostProfile> {
    constructions: crate::host::RegisteredHostConstructions,
    implementation: Arc<RegisteredHostFunctionImplementation<Profile>>,
}

pub(super) struct HostFunctionLowering<'registry, Profile: HostProfile> {
    registered: &'registry HostFunctionRegistry<Profile>,
    value_functions: Vec<HostedValueFunction<Profile>>,
    never_functions: Vec<HostedNeverFunction<Profile>>,
    additional: function::ProfiledFunctionEntries<HostedExecutionProfile>,
}

impl<Profile: HostProfile> HostFunctionRegistry<Profile> {
    pub(super) fn new(implementation_bindings: Vec<HostImplementationBinding<Profile>>) -> Self {
        Self {
            functions: implementation_bindings
                .into_iter()
                .map(|binding| {
                    let (template, constructions, implementation) = binding.into_parts();
                    (
                        template,
                        RegisteredHostFunction {
                            constructions,
                            implementation,
                        },
                    )
                })
                .collect(),
        }
    }

    pub(super) fn lowering(&self) -> HostFunctionLowering<'_, Profile> {
        HostFunctionLowering {
            registered: self,
            value_functions: Vec::new(),
            never_functions: Vec::new(),
            additional: function::ProfiledFunctionEntries {
                never: Vec::new(),
                custom: Vec::new(),
                external: Vec::new(),
                int: Vec::new(),
                float: Vec::new(),
                string: Vec::new(),
                bit_array: Vec::new(),
                utf_codepoint: Vec::new(),
                bool: Vec::new(),
                nil: Vec::new(),
                tuple: Vec::new(),
                parameter_list: Vec::new(),
                int_list: Vec::new(),
                string_list: Vec::new(),
                bit_array_list: Vec::new(),
                utf_codepoint_list: Vec::new(),
                custom_list: Vec::new(),
                external_list: Vec::new(),
                float_list: Vec::new(),
                bool_list: Vec::new(),
                nil_list: Vec::new(),
                tuple_list: Vec::new(),
                parameter_list_list: Vec::new(),
                list_list: Vec::new(),
                function_list: Vec::new(),
                int_function_functions: Vec::new(),
                float_function_functions: Vec::new(),
                string_function_functions: Vec::new(),
                bit_array_function_functions: Vec::new(),
                utf_codepoint_function_functions: Vec::new(),
                custom_function_functions: Vec::new(),
                external_function_functions: Vec::new(),
                bool_function_functions: Vec::new(),
                nil_function_functions: Vec::new(),
                tuple_function_functions: Vec::new(),
                generic_function_functions: Vec::new(),
                never_function_functions: Vec::new(),
                parameter_list_function_functions: Vec::new(),
                parameter_list_list_function_functions: Vec::new(),
                int_list_function_functions: Vec::new(),
                string_list_function_functions: Vec::new(),
                bit_array_list_function_functions: Vec::new(),
                utf_codepoint_list_function_functions: Vec::new(),
                custom_list_function_functions: Vec::new(),
                external_list_function_functions: Vec::new(),
                float_list_function_functions: Vec::new(),
                bool_list_function_functions: Vec::new(),
                nil_list_function_functions: Vec::new(),
                tuple_list_function_functions: Vec::new(),
                list_list_function_functions: Vec::new(),
                function_list_function_functions: Vec::new(),
                function_function_functions: Vec::new(),
            },
        }
    }
}

impl<Profile: HostProfile> HostFunctionLowering<'_, Profile> {
    pub(super) fn lower_specialized(
        &mut self,
        template: &HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), HostSpecializationError> {
        let index = context.specialization_index(key);
        let shape =
            SpecializedFunctionShape::instantiate(template.signature().shape(), key.substitution());
        let parameters = context.specialization_parameters(key).to_vec();
        let type_arguments = key
            .substitution()
            .arguments()
            .iter()
            .map(|argument| argument.to_module_shape().value_type())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let registered = &self.registered.functions[&template.id()];
        let implementation = Arc::clone(&registered.implementation);
        let return_ = context.representations.inhabitation(shape.return_());

        match implementation.as_ref() {
            RegisteredHostFunctionImplementation::Value(implementation) => {
                let ValueInhabitation::Inhabited(return_) = return_ else {
                    return Err(HostSpecializationError::undetermined_return_storage(
                        template.package().clone(),
                        template.site().module().clone(),
                        template.site().function().clone(),
                        shape.to_module_shape().type_(),
                    ));
                };
                sealing::seal_callbacks(template, key, &shape, &context.representations, true)?;
                let constructions =
                    sealing::seal_host_types(template, &registered.constructions, key, context);
                let parameters =
                    parameter::lower_host_parameters(&parameters, template.layout(), context);
                let type_ = context.lower_concrete_function_type(&shape);
                let host_index = self.value_functions.len();
                self.value_functions.push(HostedFunction::new(
                    HostedFunctionMetadata::new(
                        template.package().clone(),
                        template.site().clone(),
                        shape.to_module_shape().type_(),
                        type_arguments,
                        parameters,
                        constructions,
                        type_,
                    ),
                    implementation.clone(),
                ));
                return_::lower_host_return(
                    index,
                    key,
                    return_,
                    return_::HostTargetIndex::Value(host_index),
                    &mut self.additional,
                    context,
                );
            }
            RegisteredHostFunctionImplementation::Never(implementation) => {
                sealing::seal_callbacks(template, key, &shape, &context.representations, false)?;
                let constructions =
                    sealing::seal_host_types(template, &registered.constructions, key, context);
                let parameters =
                    parameter::lower_host_parameters(&parameters, template.layout(), context);
                let type_ = context.lower_concrete_function_type(&shape);
                let host_index = self.never_functions.len();
                self.never_functions.push(HostedFunction::new(
                    HostedFunctionMetadata::new(
                        template.package().clone(),
                        template.site().clone(),
                        shape.to_module_shape().type_(),
                        type_arguments,
                        parameters,
                        constructions,
                        type_,
                    ),
                    implementation.clone(),
                ));
                match return_ {
                    ValueInhabitation::Inhabited(return_) => return_::lower_host_return(
                        index,
                        key,
                        return_,
                        return_::HostTargetIndex::Never(host_index),
                        &mut self.additional,
                        context,
                    ),
                    ValueInhabitation::Uninhabited(_) => {
                        return_::lower_uninhabited_never_return(
                            index,
                            key,
                            host_index,
                            &mut self.additional,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn finish(
        self,
        context: LoweringContext,
    ) -> (
        LoweringCompletion<HostedLoweredExecution>,
        HostFunctionTables<Profile>,
    ) {
        let Self {
            registered: _,
            value_functions,
            never_functions,
            additional,
        } = self;
        let completion = context.finish_hosted(additional);
        let tables = HostFunctionTables::new(
            value_functions.into_boxed_slice(),
            never_functions.into_boxed_slice(),
        );
        (completion, tables)
    }
}

impl LoweringContext {
    fn finish_hosted(
        self,
        additional: function::ProfiledFunctionEntries<HostedExecutionProfile>,
    ) -> LoweringCompletion<HostedLoweredExecution> {
        let Self {
            constant_templates,
            constants,
            types,
            representations,
            functions,
            erased_specializations,
            ..
        } = self;
        let outcome = functions
            .finish_hosted(additional)
            .zip_with(
                SpecializationOutcome::Complete(constants.finish_hosted()),
                |functions, constants| {
                    let (list_types, custom_types, external_types, value_shapes) =
                        types.into_tables();
                    Box::new(LoweredExecution {
                        constants,
                        functions: *functions,
                        list_types,
                        custom_types,
                        external_types,
                        value_shapes,
                    })
                },
            )
            .include_prior_erasure(erased_specializations);
        (constant_templates, representations, outcome)
    }
}

#[cfg(test)]
mod tests {
    use crate::host::{HostFailure, StatelessHostProfile};
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId, ValueFunctionEntry};
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId};
    use crate::plan::execution::host::{
        HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionTarget,
    };
    use crate::plan::execution::type_::{
        FunctionType as ExecutionFunctionType, ValueType as ExecutionValueType,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::{
        HostModule, HostProviderModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn assembles_value_callbacks_in_first_use_order_and_prunes_unused_registrations() {
        fn positive(value: BigInt) -> bool {
            value > BigInt::from(0)
        }

        assert!(positive(BigInt::from(1)));
        assert!(!positive(BigInt::from(-1)));
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("subtract", <BigInt as std::ops::Sub>::sub)
            .expect("host function should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("unused", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("positive", positive)
            .expect("host function should be valid")
            .with_function("unused_predicate", || false)
            .expect("host function should be valid");
        let execution = hosted_execution(
            r#"
import host/math

pub fn main() {
  let added = math.add(1, 2)
  #(math.subtract(added, 1), math.positive(added))
}
"#,
            vec!["host_support"],
            HostProviderSet::new([math]).expect("host modules should be unique"),
        );

        let functions = execution.host_functions.value_functions();
        assert_eq!(functions.len(), 3);
        assert!(execution.host_functions.never_functions().is_empty());
        assert_host_metadata(
            &functions[0],
            "host_support",
            "host/math",
            "add",
            FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
            &[],
            ExecutionFunctionType::new(
                vec![ExecutionValueType::Int, ExecutionValueType::Int],
                ExecutionValueType::Int,
            ),
        );
        assert_host_metadata(
            &functions[1],
            "host_support",
            "host/math",
            "subtract",
            FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
            &[],
            ExecutionFunctionType::new(
                vec![ExecutionValueType::Int, ExecutionValueType::Int],
                ExecutionValueType::Int,
            ),
        );
        assert_host_metadata(
            &functions[2],
            "host_support",
            "host/math",
            "positive",
            FunctionType::new(vec![ValueType::Int], ValueType::Bool),
            &[],
            ExecutionFunctionType::new(vec![ExecutionValueType::Int], ExecutionValueType::Bool),
        );

        assert!(matches!(
            execution.program.functions.int_function(IntFunctionId(0)),
            ValueFunctionEntry::Host(HostedFunctionTarget::Value(target))
                if *target == HostFunctionId::new(0, IntLocalId(0))
        ));
        assert!(matches!(
            execution.program.functions.int_function(IntFunctionId(1)),
            ValueFunctionEntry::Host(HostedFunctionTarget::Value(target))
                if *target == HostFunctionId::new(1, IntLocalId(0))
        ));
        assert!(matches!(
            execution.program.functions.bool_function(BoolFunctionId(0)),
            ValueFunctionEntry::Host(HostedFunctionTarget::Value(target))
                if *target == HostFunctionId::new(2, BoolLocalId(0))
        ));
    }

    #[test]
    fn assembles_never_callbacks_for_each_first_use_specialization() {
        fn stop(_: BigInt) -> Result<Infallible, HostFailure> {
            Err(HostFailure::new("stopped"))
        }

        assert_eq!(stop(BigInt::from(0)), Err(HostFailure::new("stopped")),);
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_fallible_function("stop", stop)
            .expect("host function should be valid")
            .with_fallible_function("unused_stop", stop)
            .expect("host function should be valid");
        let providers = HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique");
        let execution = hosted_execution(
            r#"
@external(erlang, "host", "stop")
fn stop(value: Int) -> result

@external(erlang, "host", "unused_stop")
fn unused_stop(value: Int) -> result

pub fn main() {
  let int_stop: fn(Int) -> Int = stop
  let bool_stop: fn(Int) -> Bool = stop
  #(int_stop == int_stop, bool_stop == bool_stop)
}
"#,
            Vec::new(),
            providers,
        );

        assert!(execution.host_functions.value_functions().is_empty());
        let functions = execution.host_functions.never_functions();
        assert_eq!(functions.len(), 2);
        assert_host_metadata(
            &functions[0],
            "application",
            "main",
            "stop",
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            &[ValueType::Int],
            ExecutionFunctionType::new(vec![ExecutionValueType::Int], ExecutionValueType::Int),
        );
        assert_host_metadata(
            &functions[1],
            "application",
            "main",
            "stop",
            FunctionType::new(vec![ValueType::Int], ValueType::Bool),
            &[ValueType::Bool],
            ExecutionFunctionType::new(vec![ExecutionValueType::Int], ExecutionValueType::Bool),
        );

        assert!(matches!(
            execution.program.functions.int_function(IntFunctionId(0)),
            ValueFunctionEntry::Host(HostedFunctionTarget::Never(target))
                if *target == HostNeverFunctionId::new(0)
        ));
        assert!(matches!(
            execution.program.functions.bool_function(BoolFunctionId(0)),
            ValueFunctionEntry::Host(HostedFunctionTarget::Never(target))
                if *target == HostNeverFunctionId::new(1)
        ));
    }

    fn assert_host_metadata<Implementation>(
        function: &HostedFunction<Implementation>,
        package: &str,
        module: &str,
        name: &str,
        signature: FunctionType,
        type_arguments: &[ValueType],
        type_: ExecutionFunctionType,
    ) {
        assert_eq!(function.package(), package);
        assert_eq!(function.module(), module);
        assert_eq!(function.name(), name);
        assert_eq!(function.metadata().signature(), &signature);
        assert_eq!(function.type_arguments(), type_arguments);
        assert_eq!(function.type_(), &type_);
    }

    fn hosted_execution(
        source: &str,
        dependencies: Vec<&str>,
        hosts: HostProviderSet<StatelessHostProfile>,
    ) -> HostedExecution<StatelessHostProfile> {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                dependencies,
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal")
    }
}
