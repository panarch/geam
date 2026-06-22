use crate::plan::{
    BoolExpr, BoolExprKind, ExecutionPlan, Expr, ExprKind, IntExpr, IntExprKind, NilExpr,
    NilExprKind, StringExpr, StringExprKind, Value,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn eval_expr(plan: &ExecutionPlan, frame: &mut Frame, expression: &Expr) -> Value {
    match expression.kind() {
        ExprKind::Int(expression) => Value::Int(eval_int_expr(plan, frame, expression)),
        ExprKind::String(expression) => Value::String(eval_string_expr(plan, frame, expression)),
        ExprKind::Bool(expression) => Value::Bool(eval_bool_expr(plan, frame, expression)),
        ExprKind::Nil(expression) => {
            eval_nil_expr(plan, frame, expression);
            Value::Nil
        }
    }
}

pub(super) fn eval_int_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &IntExpr,
) -> BigInt {
    match expression.kind() {
        IntExprKind::Value(value) => value.clone(),
        IntExprKind::LocalGet { local, .. } => frame.get_int(*local),
        IntExprKind::Call { function, args } => {
            function::run_int_call(plan, *function, args, frame)
        }
        IntExprKind::Add { left, right } => {
            eval_int_expr(plan, frame, left) + eval_int_expr(plan, frame, right)
        }
        IntExprKind::Sub { left, right } => {
            eval_int_expr(plan, frame, left) - eval_int_expr(plan, frame, right)
        }
        IntExprKind::Mult { left, right } => {
            eval_int_expr(plan, frame, left) * eval_int_expr(plan, frame, right)
        }
        IntExprKind::Div { left, right } => eval_div_int(
            eval_int_expr(plan, frame, left),
            eval_int_expr(plan, frame, right),
        ),
        IntExprKind::Remainder { left, right } => eval_remainder_int(
            eval_int_expr(plan, frame, left),
            eval_int_expr(plan, frame, right),
        ),
        IntExprKind::Negate(value) => -eval_int_expr(plan, frame, value),
    }
}

pub(super) fn eval_string_expr(
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
    }
}

pub(super) fn eval_bool_expr(
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
    }
}

pub(super) fn eval_nil_expr(plan: &ExecutionPlan, frame: &mut Frame, expression: &NilExpr) {
    match expression.kind() {
        NilExprKind::Value => {}
        NilExprKind::LocalGet { local, .. } => frame.get_nil(*local),
        NilExprKind::Call { function, args } => {
            function::run_nil_call(plan, *function, args, frame)
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
    use super::super::{Value, int, run_src};

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
    }

    #[test]
    fn eval_primitive_values() {
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
}
