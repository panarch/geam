use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::NeverFunctionId;
use crate::plan::execution::graph::{NeverFunctionLocal, ParamLocal};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum NeverCallTarget {
    Direct(NeverFunctionId),
    Value(NeverFunctionLocal),
}

#[derive(Clone)]
pub struct NeverCall {
    pub function: NeverCallTarget,
    pub args: Table<ParamLocal>,
    pub site: crate::plan::HostCallSite,
}

impl NeverCall {
    pub(in crate::plan::execution) fn new(
        function: NeverCallTarget,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self {
            function,
            args,
            site,
        }
    }

    pub(crate) fn function(&self) -> &NeverCallTarget {
        &self.function
    }

    pub(crate) fn args(&self) -> &[ParamLocal] {
        &self.args
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }
}

impl Explain for NeverCall {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("never_call ");
        match self.function() {
            NeverCallTarget::Direct(function) => {
                FunctionLabel::new("never", function.0).write(context.output());
            }
            NeverCallTarget::Value(function) => context.write(function),
        }
        context.push_str(" args=");
        context.write_list(self.args(), |context, argument| context.write(argument));
    }
}

impl Emit for NeverCallTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Direct(field_0) => output.call("graph::NeverCallTarget::Direct", &[field_0]),
            Self::Value(field_0) => output.call("graph::NeverCallTarget::Value", &[field_0]),
        }
    }
}

impl Emit for NeverCall {
    fn emit(&self, output: &mut Rust) {
        let Self {
            function,
            args,
            site,
        } = self;
        output.structure(
            "graph::NeverCall",
            &[("function", function), ("args", args), ("site", site)],
        );
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        NeverCall, NeverCallTarget, NeverFunctionId, NeverFunctionLocal, ParamLocal, Rust,
    };
    use crate::plan::execution::graph::{IntLocalId, NeverFunctionLocalId};
    use crate::plan::execution::type_::{
        FunctionShape, FunctionType, GenericFunctionType, ValueShapeId, ValueType,
    };
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_direct_and_function_valued_never_calls_with_the_source_site() {
        let target = NeverCallTarget::Direct(NeverFunctionId(2));
        assert_eq!(
            Rust::expression(&target),
            "data::graph::NeverCallTarget::Direct(data::function::NeverFunctionId(2))"
        );
        let signature = FunctionType::new(
            Vec::new(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let function = NeverCallTarget::Value(NeverFunctionLocal {
            id: NeverFunctionLocalId(3),
            type_: GenericFunctionType::from_shapes(
                signature.clone(),
                FunctionShape::new(ValueShapeId(1), signature),
            ),
        });
        assert_eq!(
            Rust::expression(&function),
            r#"
data::graph::NeverCallTarget::Value(data::graph::NeverFunctionLocal {
    id: data::graph::NeverFunctionLocalId(3),
    type_: data::type_::GenericFunctionType {
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
        },
        shape: data::type_::FunctionShape {
            shape_id: data::type_::ValueShapeId(1),
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
        },
    },
})"#.trim_start_matches('\n')
        );
        let call = NeverCall::new(
            target,
            vec![ParamLocal::Int(IntLocalId(1))].into(),
            HostCallSite::from_static("example", "main", SourceSpan::new(3, 12)),
        );
        assert_eq!(
            Rust::expression(&call),
            r#"
data::graph::NeverCall {
    function: data::graph::NeverCallTarget::Direct(data::function::NeverFunctionId(2)),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
}"#.trim_start_matches('\n')
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::NeverCall;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_direct_never_call() {
        let source = r#"
fn stop(value: Int) -> value { panic }

pub fn main() -> Int {
  let _ = stop(1)
  1
}
"#;
        let expected = "never_call never#0 args=[%int#0]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_function_value_never_call() {
        let source = r#"
fn stop(value: Int) -> value { panic }

pub fn main() -> Int {
  let function = stop
  let _ = function(1)
  1
}
"#;
        let expected = "never_call %function.never#0 args=[%int#0]";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one Never call")]
    fn never_call_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            never_call(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Never call")]
    fn never_call_uniqueness_guard_is_visible() {
        let source = r#"
fn stop() -> value { panic }

pub fn main() -> Int {
  let _ = stop()
  1
}
"#;
        explain::with_execution_plan(source, |plan| {
            let call = never_call(&terminators(plan));
            never_call_from_nodes(&[call, call]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .blocks()
            .map(|block| block.terminator())
            .collect()
    }

    fn never_call<'a>(terminators: &[&'a Terminator]) -> &'a NeverCall {
        let calls = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::NeverCall(call) => Some(call),
                _ => None,
            })
            .collect::<Vec<_>>();
        never_call_from_nodes(&calls)
    }

    fn never_call_from_nodes<'a>(calls: &[&'a NeverCall]) -> &'a NeverCall {
        let mut calls = calls.iter().copied();
        let Some(call) = calls.next() else {
            panic!("source should lower one Never call");
        };
        if calls.next().is_some() {
            panic!("source should lower one Never call");
        }
        call
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let call = never_call(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(call);
        });
    }
}
