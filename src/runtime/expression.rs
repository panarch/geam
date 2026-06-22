use crate::plan::{BoolExpr, Expr, IntExpr, ModulePlan, NilExpr, StringExpr, Value};
use crate::runtime::error::RuntimeError;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn eval_expr(
    plan: &ModulePlan,
    frame: &mut Frame,
    expression: &Expr,
) -> Result<Value, RuntimeError> {
    match expression {
        Expr::Int(expression) => Ok(Value::Int(eval_int_expr(plan, frame, expression)?)),
        Expr::String(expression) => Ok(Value::String(eval_string_expr(plan, frame, expression)?)),
        Expr::Bool(expression) => Ok(Value::Bool(eval_bool_expr(plan, frame, expression)?)),
        Expr::Nil(expression) => {
            eval_nil_expr(plan, frame, expression)?;
            Ok(Value::Nil)
        }
    }
}

pub(super) fn eval_int_expr(
    plan: &ModulePlan,
    frame: &mut Frame,
    expression: &IntExpr,
) -> Result<BigInt, RuntimeError> {
    match expression {
        IntExpr::Value(value) => Ok(value.clone()),
        IntExpr::LocalGet { local, .. } => frame.get_int(*local),
        IntExpr::Call { function, args } => function::run_int_call(plan, *function, args, frame),
        IntExpr::Add { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? + eval_int_expr(plan, frame, right)?)
        }
        IntExpr::Sub { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? - eval_int_expr(plan, frame, right)?)
        }
        IntExpr::Mult { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? * eval_int_expr(plan, frame, right)?)
        }
        IntExpr::Div { left, right } => eval_div_int(
            eval_int_expr(plan, frame, left)?,
            eval_int_expr(plan, frame, right)?,
        ),
        IntExpr::Remainder { left, right } => eval_remainder_int(
            eval_int_expr(plan, frame, left)?,
            eval_int_expr(plan, frame, right)?,
        ),
        IntExpr::Negate(value) => Ok(-eval_int_expr(plan, frame, value)?),
    }
}

pub(super) fn eval_string_expr(
    plan: &ModulePlan,
    frame: &mut Frame,
    expression: &StringExpr,
) -> Result<EcoString, RuntimeError> {
    match expression {
        StringExpr::Value(value) => Ok(value.clone()),
        StringExpr::LocalGet { local, .. } => frame.get_string(*local),
        StringExpr::Call { function, args } => {
            function::run_string_call(plan, *function, args, frame)
        }
        StringExpr::Concatenate { left, right } => Ok(format!(
            "{}{}",
            eval_string_expr(plan, frame, left)?,
            eval_string_expr(plan, frame, right)?,
        )
        .into()),
    }
}

pub(super) fn eval_bool_expr(
    plan: &ModulePlan,
    frame: &mut Frame,
    expression: &BoolExpr,
) -> Result<bool, RuntimeError> {
    match expression {
        BoolExpr::Value(value) => Ok(*value),
        BoolExpr::LocalGet { local, .. } => frame.get_bool(*local),
        BoolExpr::Call { function, args } => function::run_bool_call(plan, *function, args, frame),
        BoolExpr::Not(value) => Ok(!eval_bool_expr(plan, frame, value)?),
        BoolExpr::LtInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? < eval_int_expr(plan, frame, right)?)
        }
        BoolExpr::LtEqInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? <= eval_int_expr(plan, frame, right)?)
        }
        BoolExpr::GtInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? > eval_int_expr(plan, frame, right)?)
        }
        BoolExpr::GtEqInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? >= eval_int_expr(plan, frame, right)?)
        }
        BoolExpr::Equal { left, right } => {
            Ok(eval_expr(plan, frame, left)? == eval_expr(plan, frame, right)?)
        }
        BoolExpr::NotEqual { left, right } => {
            Ok(eval_expr(plan, frame, left)? != eval_expr(plan, frame, right)?)
        }
    }
}

pub(super) fn eval_nil_expr(
    plan: &ModulePlan,
    frame: &mut Frame,
    expression: &NilExpr,
) -> Result<(), RuntimeError> {
    match expression {
        NilExpr::Value => Ok(()),
        NilExpr::LocalGet { local, .. } => frame.get_nil(*local),
        NilExpr::Call { function, args } => function::run_nil_call(plan, *function, args, frame),
    }
}

fn eval_div_int(left: BigInt, right: BigInt) -> Result<BigInt, RuntimeError> {
    // Gleam defines Int division by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return Ok(BigInt::from(0));
    }

    Ok(left / right)
}

fn eval_remainder_int(left: BigInt, right: BigInt) -> Result<BigInt, RuntimeError> {
    // Gleam defines Int remainder by zero as 0 across its targets.
    if right == BigInt::from(0) {
        return Ok(BigInt::from(0));
    }

    Ok(left % right)
}

#[cfg(test)]
mod tests {
    use super::super::{Value, int, run_src};
    use super::{eval_bool_expr, eval_int_expr};
    use crate::plan::{BoolExpr, IntExpr, ModulePlan};
    use crate::runtime::frame::Frame;

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
    fn eval_negate_int() {
        let plan = ModulePlan {
            module: "main".into(),
            main: crate::FunctionId(0),
            functions: Vec::new(),
        };
        let mut frame = Frame::default();

        assert_eq!(
            eval_int_expr(
                &plan,
                &mut frame,
                &IntExpr::Negate(Box::new(IntExpr::Value(3.into())))
            ),
            Ok((-3).into()),
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

    #[test]
    fn eval_bool_not() {
        let plan = ModulePlan {
            module: "main".into(),
            main: crate::FunctionId(0),
            functions: Vec::new(),
        };
        let mut frame = Frame::default();

        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::Not(Box::new(BoolExpr::Value(true)))
            ),
            Ok(false),
        );
    }
}
