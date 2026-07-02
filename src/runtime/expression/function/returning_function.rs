use crate::plan::{
    ExecutionPlan, FunctionFunctionExpr, FunctionFunctionExprKind, FunctionFunctionValue,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{eval_bool_expr, eval_int_expr, eval_string_expr};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionFunctionExpr,
) -> Result<FunctionFunctionValue, ExecutionError> {
    match expression.kind() {
        FunctionFunctionExprKind::Value(value) => Ok(value.clone()),
        FunctionFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
            return_type,
        } => Ok(FunctionFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
            return_type.clone(),
        )),
        FunctionFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_function_function(*local)),
        FunctionFunctionExprKind::Call { function, args, .. } => {
            function::run_function_function_returning_function_call(plan, *function, args, frame)
        }
        FunctionFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_function_function_function_call(plan, callee, args, frame),
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_function_function_expr(plan, frame, true_)
            } else {
                eval_function_function_expr(plan, frame, false_)
            }
        }
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_function_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        FunctionFunctionId, FunctionFunctionValue, FunctionType, IntFunctionFunctionId, ValueType,
    };
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_function_function_value() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get
}
"#,
        );
    }

    #[test]
    fn eval_function_function_local_get() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  let getter = get
  getter
}
"#,
        );
    }

    #[test]
    fn eval_function_function_direct_call() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn select() {
  get
}

pub fn main() {
  select()
}
"#,
        );
    }

    #[test]
    fn eval_function_function_value_call() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn select() {
  get
}

pub fn main() {
  let selector = select
  selector()
}
"#,
        );
    }

    #[test]
    fn eval_function_function_bool_case_branches() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case True {
    True -> get
    False -> get_other
  }
}
"#,
        );

        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case False {
    True -> get_other
    False -> get
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_int_case_branches() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 1 {
    1 -> get
    _ -> get_other
  }
}
"#,
        );

        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 2 {
    1 -> get_other
    _ -> get
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_block() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  {
    let getter = get
    getter
  }
}
"#,
        );
    }

    fn assert_returns_function_returning_int(src: &str) {
        assert_eq!(
            run_src(src),
            Value::Function(
                FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    returned_int_function_type(),
                )
                .into(),
            ),
        );
    }

    fn returned_int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }
}
