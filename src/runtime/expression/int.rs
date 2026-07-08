use super::{
    eval_bool_expr, eval_float_expr, eval_panic_expr, eval_string_expr, project_list_expr,
    project_tuple_expr,
};
use crate::plan::{ExecutionPlan, IntExpr, IntExprKind, Value, ValueType};
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
        IntExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, frame, tuple, *index, ValueType::Int)? {
                Value::Int(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::Int,
                    other.value_type(),
                )),
            }
        }
        IntExprKind::ListIndex { list, index } => {
            match project_list_expr(plan, frame, list, *index, ValueType::Int)? {
                Value::Int(value) => Ok(value),
                other => Err(ExecutionError::list_index_family_mismatch(
                    ValueType::Int,
                    other.value_type(),
                )),
            }
        }
        IntExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
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
        IntExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_int_expr(plan, frame, branch);
                }
            }
            eval_int_expr(plan, frame, fallback)
        }
        IntExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
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
    use super::eval_int_expr;
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, IntExpr, ListExpr, PanicExpr, PanicSite, Step, StringExpr,
        TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use crate::runtime::{int, run_src};

    #[test]
    fn tuple_index_family_mismatch_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );

        assert_eq!(
            eval_int_expr(&plan, &mut frame, &IntExpr::tuple_index(tuple, 0)),
            Ok(1.into()),
        );

        let tuple = TupleExpr::value(
            vec![Expr::string(StringExpr::value("one".into()))],
            vec![ValueType::String],
        );

        assert_eq!(
            eval_int_expr(&plan, &mut frame, &IntExpr::tuple_index(tuple, 0)),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn list_index_family_mismatch_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let mut frame = Frame::default();
        let list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);

        assert_eq!(
            eval_int_expr(&plan, &mut frame, &IntExpr::list_index(list, 0)),
            Ok(1.into()),
        );

        let list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        assert_eq!(
            eval_int_expr(&plan, &mut frame, &IntExpr::list_index(list, 1)),
            Err(ExecutionError::list_index_family_mismatch(
                ValueType::Int,
                ValueType::List(Box::new(ValueType::Int)),
            )),
        );

        let list = ListExpr::value(
            vec![Expr::string(StringExpr::value("one".into()))],
            ValueType::String,
        );

        assert_eq!(
            eval_int_expr(&plan, &mut frame, &IntExpr::list_index(list, 0)),
            Err(ExecutionError::list_index_family_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn eval_int_panic_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let mut frame = Frame::default();

        assert_eq!(
            eval_int_expr(
                &plan,
                &mut frame,
                &IntExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ),
            Err(ExecutionError::source_panic(
                None,
                PanicKind::Panic,
                None,
                PanicSite::unknown()
            )),
        );
    }

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
    fn eval_string_case_int() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case "one" {
    "one" -> 10
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
  let value = case "many" {
    "one" -> 10
    _ -> 0
  }
  value
}
"#,
            ),
            int(0),
        );
    }

    #[test]
    fn eval_float_case_int() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1.0 {
    1.0 -> 10
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
  let value = case 2.0 {
    1.0 -> 10
    _ -> 0
  }
  value
}
"#,
            ),
            int(0),
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

    #[test]
    fn eval_int_expr_propagates_operand_errors() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let mut frame = Frame::default();

        for (expression, expected) in [
            (
                IntExpr::add(error_int_expr(), IntExpr::value(1.into())),
                ValueType::Int,
            ),
            (
                IntExpr::add(IntExpr::value(1.into()), error_int_expr()),
                ValueType::Int,
            ),
            (
                IntExpr::sub(error_int_expr(), IntExpr::value(1.into())),
                ValueType::Int,
            ),
            (
                IntExpr::sub(IntExpr::value(1.into()), error_int_expr()),
                ValueType::Int,
            ),
            (
                IntExpr::mult(error_int_expr(), IntExpr::value(1.into())),
                ValueType::Int,
            ),
            (
                IntExpr::mult(IntExpr::value(1.into()), error_int_expr()),
                ValueType::Int,
            ),
            (
                IntExpr::div(error_int_expr(), IntExpr::value(1.into())),
                ValueType::Int,
            ),
            (
                IntExpr::div(IntExpr::value(1.into()), error_int_expr()),
                ValueType::Int,
            ),
            (
                IntExpr::remainder(error_int_expr(), IntExpr::value(1.into())),
                ValueType::Int,
            ),
            (
                IntExpr::remainder(IntExpr::value(1.into()), error_int_expr()),
                ValueType::Int,
            ),
            (IntExpr::negate(error_int_expr()), ValueType::Int),
            (
                IntExpr::bool_case(
                    error_bool_expr(),
                    IntExpr::value(1.into()),
                    IntExpr::value(0.into()),
                ),
                ValueType::Bool,
            ),
            (
                IntExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), IntExpr::value(1.into()))],
                    IntExpr::value(0.into()),
                ),
                ValueType::Int,
            ),
            (
                IntExpr::string_case(
                    error_string_expr(),
                    vec![("one".into(), IntExpr::value(1.into()))],
                    IntExpr::value(0.into()),
                ),
                ValueType::String,
            ),
            (
                IntExpr::float_case(
                    error_float_expr(),
                    vec![(1.0, IntExpr::value(1.into()))],
                    IntExpr::value(0.into()),
                ),
                ValueType::Float,
            ),
            (
                IntExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    IntExpr::value(1.into()),
                ),
                ValueType::Bool,
            ),
        ] {
            assert_eq!(
                eval_int_expr(&plan, &mut frame, &expression),
                Err(tuple_index_error(expected)),
            );
        }
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(empty_tuple(), 0)
    }

    fn empty_tuple() -> TupleExpr {
        TupleExpr::value(Vec::new(), Vec::new())
    }

    fn tuple_index_error(expected: ValueType) -> ExecutionError {
        ExecutionError::tuple_index_family_mismatch(expected, ValueType::Tuple(Vec::new()))
    }
}
