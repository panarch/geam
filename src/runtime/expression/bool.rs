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
        BoolExprKind::FunctionCall { function, args } => {
            function::run_bool_function_call(plan, function, args, frame)
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
        BoolExprKind::And { left, right } => {
            let left = eval_bool_expr(plan, frame, left);
            eval_and(left, || eval_bool_expr(plan, frame, right))
        }
        BoolExprKind::Or { left, right } => {
            let left = eval_bool_expr(plan, frame, left);
            eval_or(left, || eval_bool_expr(plan, frame, right))
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
        BoolExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_bool_expr(plan, frame, return_)
        }
    }
}

fn eval_and(left: bool, right: impl FnOnce() -> bool) -> bool {
    if left { right() } else { false }
}

fn eval_or(left: bool, right: impl FnOnce() -> bool) -> bool {
    if left { true } else { right() }
}

#[cfg(test)]
mod tests {
    use super::{eval_and, eval_or};
    use crate::runtime::{Value, run_src};
    use std::cell::Cell;

    thread_local! {
        static RIGHT_CALLED: Cell<bool> = const { Cell::new(false) };
    }

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
    fn eval_bool_operators() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  True && True
}
"#,
            ),
            Value::Bool(true),
        );
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  False && True
}
"#,
            ),
            Value::Bool(false),
        );
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  True || False
}
"#,
            ),
            Value::Bool(true),
        );
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  False || False
}
"#,
            ),
            Value::Bool(false),
        );
    }

    #[test]
    fn eval_and_short_circuits_false_left() {
        reset_called();
        let actual = eval_and(false, mark_called_true);

        assert!(!actual);
        assert!(!right_called());
    }

    #[test]
    fn eval_and_evaluates_true_left() {
        reset_called();
        let actual = eval_and(true, mark_called_true);

        assert!(actual);
        assert!(right_called());
    }

    #[test]
    fn eval_or_short_circuits_true_left() {
        reset_called();
        let actual = eval_or(true, mark_called_false);

        assert!(actual);
        assert!(!right_called());
    }

    #[test]
    fn eval_or_evaluates_false_left() {
        reset_called();
        let actual = eval_or(false, mark_called_false);

        assert!(!actual);
        assert!(right_called());
    }

    fn reset_called() {
        RIGHT_CALLED.set(false);
    }

    fn right_called() -> bool {
        RIGHT_CALLED.get()
    }

    fn mark_called_true() -> bool {
        mark_called(true)
    }

    fn mark_called_false() -> bool {
        mark_called(false)
    }

    fn mark_called(value: bool) -> bool {
        RIGHT_CALLED.set(true);
        value
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

    #[test]
    fn eval_block_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    1
    True
  }
}
"#,
            ),
            Value::Bool(true),
        );
    }
}
