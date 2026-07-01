use super::eval_bool_expr;
use crate::plan::{ExecutionPlan, IntExpr, IntExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use num_bigint::BigInt;

pub(in crate::runtime) fn eval_int_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntExpr,
) -> Result<BigInt, ExecutionError> {
    match expression.kind() {
        IntExprKind::Value(value) => Ok(value.clone()),
        IntExprKind::LocalGet { local, .. } => Ok(frame.get_int(*local)),
        IntExprKind::Call { function, args } => {
            function::run_int_call(plan, *function, args, frame)
        }
        IntExprKind::FunctionCall { function, args } => {
            function::run_int_function_call(plan, function, args, frame)
        }
        IntExprKind::Add { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? + eval_int_expr(plan, frame, right)?)
        }
        IntExprKind::Sub { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? - eval_int_expr(plan, frame, right)?)
        }
        IntExprKind::Mult { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? * eval_int_expr(plan, frame, right)?)
        }
        IntExprKind::Div { left, right } => Ok(eval_div_int(
            eval_int_expr(plan, frame, left)?,
            eval_int_expr(plan, frame, right)?,
        )),
        IntExprKind::Remainder { left, right } => Ok(eval_remainder_int(
            eval_int_expr(plan, frame, left)?,
            eval_int_expr(plan, frame, right)?,
        )),
        IntExprKind::Negate(value) => Ok(-eval_int_expr(plan, frame, value)?),
        IntExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_int_expr(plan, frame, true_)
            } else {
                eval_int_expr(plan, frame, false_)
            }
        }
        IntExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_expr(plan, frame, branch);
                }
            }
            eval_int_expr(plan, frame, fallback)
        }
        IntExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_int_expr(plan, frame, return_)
        }
    }
}

fn eval_div_int(left: BigInt, right: BigInt) -> BigInt {
    // Gleam defines Int division by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return BigInt::from(0);
    }

    left / right
}

fn eval_remainder_int(left: BigInt, right: BigInt) -> BigInt {
    // Gleam defines Int remainder by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return BigInt::from(0);
    }

    left % right
}

#[cfg(test)]
mod tests {
    use crate::runtime::{int, run_src};

    #[test]
    fn eval_integer_arithmetic() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 + 2 * 3
}
"#,
            ),
            int(7),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  7 - 2
}
"#,
            ),
            int(5),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  -3
}
"#,
            ),
            int(-3),
        );

        assert_eq!(
            run_src(
                r#"
fn negate(value: Int) {
  -value
}

pub fn main() {
  negate(3)
}
"#,
            ),
            int(-3),
        );
    }

    #[test]
    fn eval_integer_division() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  11 / 3
}
"#,
            ),
            int(3),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  -11 / 3
}
"#,
            ),
            int(-3),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  11 / -3
}
"#,
            ),
            int(-3),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  -11 / -3
}
"#,
            ),
            int(3),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 / 0
}
"#,
            ),
            int(0),
        );
    }

    #[test]
    fn eval_integer_remainder() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  11 % 3
}
"#,
            ),
            int(2),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  -11 % 3
}
"#,
            ),
            int(-2),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  11 % -3
}
"#,
            ),
            int(2),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  -11 % -3
}
"#,
            ),
            int(-2),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1 % 0
}
"#,
            ),
            int(0),
        );
    }

    #[test]
    fn eval_local_function_call() {
        assert_eq!(
            run_src(
                r#"
fn one() {
  1
}

pub fn main() {
  one()
}
"#,
            ),
            int(1),
        );

        assert_eq!(
            run_src(
                r#"
fn one() {
  1
}

pub fn main() {
  let value = one()
  value
}
"#,
            ),
            int(1),
        );
    }

    #[test]
    fn eval_bool_case_int() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case True {
    True -> 1
    False -> 0
  }
  value
}
"#,
            ),
            int(1),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case False {
    True -> 1
    False -> 0
  }
  value
}
"#,
            ),
            int(0),
        );
    }

    #[test]
    fn eval_int_case_int() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1 {
    1 -> 10
    _ -> 0
  }
  value
}
"#,
            ),
            int(10),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 9 {
    1 -> 10
    _ -> 0
  }
  value
}
"#,
            ),
            int(0),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1 {
    _ -> 7
    1 -> 10
  }
  value
}
"#,
            ),
            int(7),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1 {
    1 -> 10
    1 -> 20
    _ -> 0
  }
  value
}
"#,
            ),
            int(10),
        );
    }

    #[test]
    fn eval_block_int() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    let x = 1
    x + 2
  }
}
"#,
            ),
            int(3),
        );
    }

    #[test]
    fn eval_block_shadowing() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let x = 1
  {
    let x = 2
    x + 1
  }
  x
}
"#,
            ),
            int(1),
        );
    }
}
