use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, NilExpr, NilExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_nil_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilExpr,
) {
    match expression.kind() {
        NilExprKind::Value => {}
        NilExprKind::LocalGet { local, .. } => frame.get_nil(*local),
        NilExprKind::Call { function, args } => {
            function::run_nil_call(plan, *function, args, frame)
        }
        NilExprKind::FunctionCall { function, args } => {
            function::run_nil_function_call(plan, function, args, frame)
        }
        NilExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_nil_expr(plan, frame, true_);
            } else {
                eval_nil_expr(plan, frame, false_);
            }
        }
        NilExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, frame, branch);
                }
            }
            eval_nil_expr(plan, frame, fallback);
        }
        NilExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_nil_expr(plan, frame, return_);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_nil_value() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  Nil
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_bool_case_nil() {
        assert_eq!(
            run_src(
                r#"
fn flag() {
  True
}

pub fn main() {
  case flag() {
    True -> Nil
    False -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );

        assert_eq!(
            run_src(
                r#"
fn flag() {
  False
}

pub fn main() {
  case flag() {
    True -> Nil
    False -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_int_case_nil() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 1 {
    1 -> Nil
    _ -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 2 {
    1 -> Nil
    _ -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_block_nil() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    1
    Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }
}
