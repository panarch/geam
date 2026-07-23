use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::NeverFunctionId;
use crate::plan::execution::graph::{NeverFunctionLocal, ParamLocal};

pub(crate) enum NeverCallTarget {
    Direct(NeverFunctionId),
    Value(NeverFunctionLocal),
}

pub(crate) struct NeverCall {
    function: NeverCallTarget,
    args: Box<[ParamLocal]>,
}

impl NeverCall {
    pub(in crate::plan::execution) fn new(
        function: NeverCallTarget,
        args: Box<[ParamLocal]>,
    ) -> Self {
        Self { function, args }
    }

    pub(crate) fn function(&self) -> &NeverCallTarget {
        &self.function
    }

    pub(crate) fn args(&self) -> &[ParamLocal] {
        &self.args
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
            .iter()
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
