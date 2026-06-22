use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, StringExpr, StringExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;

pub(in crate::runtime) fn eval_string_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringExpr,
) -> EcoString {
    match expression.kind() {
        StringExprKind::Value(value) => value.clone(),
        StringExprKind::LocalGet { local, .. } => frame.get_string(*local),
        StringExprKind::Call { function, args } => {
            function::run_string_call(plan, *function, args, frame)
        }
        StringExprKind::Concatenate { left, right } => format!(
            "{}{}",
            eval_string_expr(plan, frame, left),
            eval_string_expr(plan, frame, right),
        )
        .into(),
        StringExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_string_expr(plan, frame, true_)
            } else {
                eval_string_expr(plan, frame, false_)
            }
        }
        StringExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_string_concatenation() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  "hello, " <> "geam"
}
"#,
            ),
            Value::String("hello, geam".into()),
        );
    }

    #[test]
    fn eval_bool_case_string() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case True {
    True -> "yes"
    False -> "no"
  }
}
"#,
            ),
            Value::String("yes".into()),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case False {
    True -> "yes"
    False -> "no"
  }
}
"#,
            ),
            Value::String("no".into()),
        );
    }

    #[test]
    fn eval_int_case_string() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 1 {
    1 -> "one"
    _ -> "other"
  }
}
"#,
            ),
            Value::String("one".into()),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 2 {
    1 -> "one"
    _ -> "other"
  }
}
"#,
            ),
            Value::String("other".into()),
        );
    }
}
