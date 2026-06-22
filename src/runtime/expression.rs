use crate::plan::{BinOp, Expr, FunctionRef, ModulePlan, Value};
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
        Expr::Value(value) => Ok(value.clone()),
        Expr::LocalGet { local, .. } => frame.get(*local),
        Expr::Call { function, args } => {
            let args = args
                .iter()
                .map(|argument| eval_expr(plan, frame, argument))
                .collect::<Result<Vec<_>, _>>()?;
            match function {
                FunctionRef::Local(name) => function::run_function(plan, name.as_str(), args),
            }
        }
        Expr::BinOp { op, left, right } => {
            let left = eval_expr(plan, frame, left)?;
            let right = eval_expr(plan, frame, right)?;
            eval_bin_op(*op, left, right)
        }
        Expr::NegateInt(value) => {
            let value = eval_expr(plan, frame, value)?;
            Ok(Value::Int(-expect_int(value)?))
        }
        Expr::NegateBool(value) => {
            let value = eval_expr(plan, frame, value)?;
            Ok(Value::Bool(!expect_bool(value)?))
        }
    }
}

fn eval_bin_op(op: BinOp, left: Value, right: Value) -> Result<Value, RuntimeError> {
    match op {
        BinOp::AddInt => Ok(Value::Int(expect_int(left)? + expect_int(right)?)),
        BinOp::SubInt => Ok(Value::Int(expect_int(left)? - expect_int(right)?)),
        BinOp::MultInt => Ok(Value::Int(expect_int(left)? * expect_int(right)?)),
        BinOp::Eq => Ok(Value::Bool(left == right)),
        BinOp::NotEq => Ok(Value::Bool(left != right)),
        BinOp::Concatenate => Ok(Value::String(
            format!("{}{}", expect_string(left)?, expect_string(right)?).into(),
        )),
    }
}

fn expect_int(value: Value) -> Result<BigInt, RuntimeError> {
    match value {
        Value::Int(value) => Ok(value),
        other => Err(RuntimeError::TypeMismatch {
            expected: "Int",
            actual: other.kind(),
        }),
    }
}

fn expect_bool(value: Value) -> Result<bool, RuntimeError> {
    match value {
        Value::Bool(value) => Ok(value),
        other => Err(RuntimeError::TypeMismatch {
            expected: "Bool",
            actual: other.kind(),
        }),
    }
}

fn expect_string(value: Value) -> Result<EcoString, RuntimeError> {
    match value {
        Value::String(value) => Ok(value),
        other => Err(RuntimeError::TypeMismatch {
            expected: "String",
            actual: other.kind(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::frame::Frame;
    use super::super::{RuntimeError, Value, int, run_src};
    use super::{eval_bin_op, eval_expr};
    use crate::plan::{BinOp, Expr, ModulePlan};

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
    fn eval_negate_int() {
        let plan = ModulePlan {
            module: "main".into(),
            functions: Vec::new(),
        };
        let mut frame = Frame::default();
        let expression = Expr::NegateInt(Box::new(Expr::Value(int(3))));

        assert_eq!(eval_expr(&plan, &mut frame, &expression), Ok(int(-3)));
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
    fn report_type_mismatch() {
        assert_eq!(
            eval_bin_op(BinOp::AddInt, Value::String("bad".into()), int(1)),
            Err(RuntimeError::TypeMismatch {
                expected: "Int",
                actual: "String",
            }),
        );

        assert_eq!(
            eval_bin_op(
                BinOp::Concatenate,
                Value::Bool(true),
                Value::String("value".into()),
            ),
            Err(RuntimeError::TypeMismatch {
                expected: "String",
                actual: "Bool",
            }),
        );

        let plan = ModulePlan {
            module: "main".into(),
            functions: Vec::new(),
        };
        let mut frame = Frame::default();
        let expression = Expr::NegateBool(Box::new(Expr::Value(Value::Nil)));

        assert_eq!(
            eval_expr(&plan, &mut frame, &expression),
            Err(RuntimeError::TypeMismatch {
                expected: "Bool",
                actual: "Nil",
            }),
        );
    }
}
