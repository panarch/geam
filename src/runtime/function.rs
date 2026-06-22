use crate::plan::{FunctionId, FunctionPlan, ModulePlan, Step, Value};
use crate::runtime::error::RuntimeError;
use crate::runtime::expression::eval_expr;
use crate::runtime::frame::Frame;

pub(super) fn run_function(
    plan: &ModulePlan,
    function: FunctionId,
    args: Vec<Value>,
) -> Result<Value, RuntimeError> {
    let function = &plan.functions[function.0];
    execute_function(plan, function, args)
}

fn execute_function(
    plan: &ModulePlan,
    function: &FunctionPlan,
    args: Vec<Value>,
) -> Result<Value, RuntimeError> {
    let mut frame = Frame::default();
    for (param, value) in function.params.iter().zip(args) {
        frame.set(param.local, value);
    }

    for step in &function.steps {
        match step {
            Step::Let { local, value, .. } => {
                let value = eval_expr(plan, &mut frame, value)?;
                frame.set(*local, value);
            }
            Step::Evaluate(expression) => {
                let _ = eval_expr(plan, &mut frame, expression)?;
            }
        }
    }

    eval_expr(plan, &mut frame, &function.return_)
}

#[cfg(test)]
mod tests {
    use super::super::{int, run_src};

    #[test]
    fn execute_let_binding() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let x = 1
  x + 2
}
"#,
            ),
            int(3),
        );
    }

    #[test]
    fn execute_expression_steps() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 == 1
  5
}
"#,
            ),
            int(5),
        );
    }

    #[test]
    fn execute_function_with_arguments() {
        assert_eq!(
            run_src(
                r#"
fn double(value: Int) {
  value * 2
}

pub fn main() {
  double(3)
}
"#,
            ),
            int(6),
        );
    }
}
