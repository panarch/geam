use crate::plan::{FunctionPlan, ModulePlan, Step, Value};
use crate::runtime::error::RuntimeError;
use crate::runtime::expression::eval_expr;
use crate::runtime::frame::Frame;

pub(super) fn run_function(
    plan: &ModulePlan,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, RuntimeError> {
    let function = find_function(plan, name)?;
    execute_function(plan, function, args)
}

fn find_function<'a>(plan: &'a ModulePlan, name: &str) -> Result<&'a FunctionPlan, RuntimeError> {
    plan.functions
        .iter()
        .find(|function| function.name == name)
        .ok_or_else(|| RuntimeError::MissingFunction { name: name.into() })
}

fn execute_function(
    plan: &ModulePlan,
    function: &FunctionPlan,
    args: Vec<Value>,
) -> Result<Value, RuntimeError> {
    if function.params.len() != args.len() {
        return Err(RuntimeError::ArityMismatch {
            name: function.name.clone(),
            expected: function.params.len(),
            got: args.len(),
        });
    }

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
    use super::super::{RuntimeError, int, plan_src, run_src};
    use super::run_function;

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
    fn run_function_with_arguments() {
        let plan = plan_src(
            r#"
pub fn double(value: Int) {
  value * 2
}
"#,
        );

        assert_eq!(run_function(&plan, "double", vec![int(3)]), Ok(int(6)));
    }

    #[test]
    fn report_arity_mismatch() {
        let plan = plan_src(
            r#"
pub fn double(value: Int) {
  value * 2
}
"#,
        );

        assert_eq!(
            run_function(&plan, "double", Vec::new()),
            Err(RuntimeError::ArityMismatch {
                name: "double".into(),
                expected: 1,
                got: 0,
            }),
        );
    }
}
