use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::NeverFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use std::convert::Infallible;

pub(in crate::runtime) fn run_never(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NeverFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    #[test]
    fn an_actual_empty_return_preserves_the_source_panic() {
        let source = r#"
pub type Empty
pub fn main() -> Empty { panic as "never result" }
"#;
        let error = crate::runtime::run_src_error(source);
        assert_eq!(error.to_string(), "panic: never result");
    }
}
