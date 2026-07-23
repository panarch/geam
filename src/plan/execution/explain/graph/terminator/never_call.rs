use super::super::super::label::FunctionLabel;
use super::super::super::value::{ExplainLocal, write_locals};
use crate::plan::execution::{NeverCallTarget, ParamLocal};

pub(super) fn write_never_call(
    output: &mut String,
    function: &NeverCallTarget,
    args: &[ParamLocal],
) {
    output.push_str("never_call ");
    match function {
        NeverCallTarget::Direct(function) => {
            FunctionLabel::new("never", function.0).push_to(output);
        }
        NeverCallTarget::Value(function) => function.write_local(output),
    }
    output.push_str(" args=");
    write_locals(output, args);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_direct_never_call() {
        let source = r#"
fn stop() -> value { panic }

pub fn main() -> Int {
  let _ = stop()
  1
}
"#;
        let expected = "never_call never#0 args=[]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_function_value_never_call() {
        let source = r#"
fn stop() -> value { panic }

pub fn main() -> Int {
  let function = stop
  let _ = function()
  1
}
"#;
        let expected = "never_call %function.never#0 args=[]";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let (function, args) = never_call(&terminators);
            super::write_never_call(output, function, args);
        });
    }

    fn never_call<'a>(
        terminators: &[&'a Terminator],
    ) -> (
        &'a crate::plan::execution::NeverCallTarget,
        &'a [crate::plan::execution::ParamLocal],
    ) {
        let mut calls = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::NeverCall { function, args } => Some((function, args.as_ref())),
                _ => None,
            });
        let Some(call) = calls.next() else {
            panic!("source should lower one Never call");
        };
        if calls.next().is_some() {
            panic!("source should lower one Never call");
        }
        call
    }

    #[test]
    #[should_panic(expected = "source should lower one Never call")]
    fn never_call_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            never_call(&terminators);
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
        super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::NeverCall { .. }))
                .expect("source should lower a Never call");
            never_call(&[terminator, terminator]);
        });
    }
}
