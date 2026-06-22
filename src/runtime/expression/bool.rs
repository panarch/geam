use super::{eval_expr, eval_int_expr};
use crate::plan::{BoolExpr, BoolExprKind, ExecutionPlan};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_bool_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolExpr,
) -> bool {
    match expression.kind() {
        BoolExprKind::Value(value) => *value,
        BoolExprKind::LocalGet { local, .. } => frame.get_bool(*local),
        BoolExprKind::Call { function, args } => {
            function::run_bool_call(plan, *function, args, frame)
        }
        BoolExprKind::Not(value) => !eval_bool_expr(plan, frame, value),
        BoolExprKind::LtInt { left, right } => {
            eval_int_expr(plan, frame, left) < eval_int_expr(plan, frame, right)
        }
        BoolExprKind::LtEqInt { left, right } => {
            eval_int_expr(plan, frame, left) <= eval_int_expr(plan, frame, right)
        }
        BoolExprKind::GtInt { left, right } => {
            eval_int_expr(plan, frame, left) > eval_int_expr(plan, frame, right)
        }
        BoolExprKind::GtEqInt { left, right } => {
            eval_int_expr(plan, frame, left) >= eval_int_expr(plan, frame, right)
        }
        BoolExprKind::Equal { left, right } => {
            eval_expr(plan, frame, left) == eval_expr(plan, frame, right)
        }
        BoolExprKind::NotEqual { left, right } => {
            eval_expr(plan, frame, left) != eval_expr(plan, frame, right)
        }
        BoolExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_bool_expr(plan, frame, true_)
            } else {
                eval_bool_expr(plan, frame, false_)
            }
        }
        BoolExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, frame, branch);
                }
            }
            eval_bool_expr(plan, frame, fallback)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_integer_comparisons() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 < 2
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  2 <= 2
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  2 > 1
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 >= 2
}
"#,
            ),
            Value::Bool(false),
        );
    }

    #[test]
    fn eval_bool_values() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  True != False
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  !False
}
"#,
            ),
            Value::Bool(true),
        );
    }

    #[test]
    fn eval_bool_case_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = True
  case value {
    True -> False
    False -> True
  }
}
"#,
            ),
            Value::Bool(false),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = False
  case value {
    True -> False
    False -> True
  }
}
"#,
            ),
            Value::Bool(true),
        );
    }

    #[test]
    fn eval_int_case_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 1 {
    1 -> True
    _ -> False
  }
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 2 {
    1 -> True
    _ -> False
  }
}
"#,
            ),
            Value::Bool(false),
        );
    }
}
