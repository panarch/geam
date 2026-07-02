use super::{eval_expr, eval_float_expr, eval_int_expr, eval_string_expr};
use crate::plan::{BoolExpr, BoolExprKind, ExecutionPlan};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_bool_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &BoolExpr,
) -> Result<bool, ExecutionError> {
    match expression.kind() {
        BoolExprKind::Value(value) => Ok(*value),
        BoolExprKind::LocalGet { local, .. } => Ok(frame.get_bool(*local)),
        BoolExprKind::Call { function, args } => {
            function::run_bool_call(plan, *function, args, frame)
        }
        BoolExprKind::FunctionCall { function, args } => {
            function::run_bool_function_call(plan, function, args, frame)
        }
        BoolExprKind::Not(value) => Ok(!eval_bool_expr(plan, frame, value)?),
        BoolExprKind::LtInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? < eval_int_expr(plan, frame, right)?)
        }
        BoolExprKind::LtEqInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? <= eval_int_expr(plan, frame, right)?)
        }
        BoolExprKind::GtInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? > eval_int_expr(plan, frame, right)?)
        }
        BoolExprKind::GtEqInt { left, right } => {
            Ok(eval_int_expr(plan, frame, left)? >= eval_int_expr(plan, frame, right)?)
        }
        BoolExprKind::LtFloat { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? < eval_float_expr(plan, frame, right)?)
        }
        BoolExprKind::LtEqFloat { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? <= eval_float_expr(plan, frame, right)?)
        }
        BoolExprKind::GtFloat { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? > eval_float_expr(plan, frame, right)?)
        }
        BoolExprKind::GtEqFloat { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? >= eval_float_expr(plan, frame, right)?)
        }
        BoolExprKind::Equal { left, right } => {
            Ok(eval_expr(plan, frame, left)? == eval_expr(plan, frame, right)?)
        }
        BoolExprKind::NotEqual { left, right } => {
            Ok(eval_expr(plan, frame, left)? != eval_expr(plan, frame, right)?)
        }
        BoolExprKind::And { left, right } => {
            let left = eval_bool_expr(plan, frame, left)?;
            eval_and(left, || eval_bool_expr(plan, frame, right))
        }
        BoolExprKind::Or { left, right } => {
            let left = eval_bool_expr(plan, frame, left)?;
            eval_or(left, || eval_bool_expr(plan, frame, right))
        }
        BoolExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
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
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, frame, branch);
                }
            }
            eval_bool_expr(plan, frame, fallback)
        }
        BoolExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, frame, branch);
                }
            }
            eval_bool_expr(plan, frame, fallback)
        }
        BoolExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bool_expr(plan, frame, branch);
                }
            }
            eval_bool_expr(plan, frame, fallback)
        }
        BoolExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_bool_expr(plan, frame, return_)
        }
    }
}

fn eval_and(
    left: bool,
    right: impl FnOnce() -> Result<bool, ExecutionError>,
) -> Result<bool, ExecutionError> {
    if left { right() } else { Ok(false) }
}

fn eval_or(
    left: bool,
    right: impl FnOnce() -> Result<bool, ExecutionError>,
) -> Result<bool, ExecutionError> {
    if left { Ok(true) } else { right() }
}

#[cfg(test)]
mod tests {
    use super::{eval_and, eval_bool_expr, eval_or};
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, ExecutionPlan, Expr, FloatExpr, FloatFunctionExpr,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue, FunctionId, FunctionPlan,
        FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId, ReturnExpr,
        Step, StringExpr, StringFunctionExpr, StringFunctionFunctionId, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;
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
    fn eval_string_case_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case "yes" {
    "yes" -> True
    _ -> False
  }
  value
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case "no" {
    "yes" -> True
    _ -> False
  }
  value
}
"#,
            ),
            Value::Bool(false),
        );
    }

    #[test]
    fn eval_float_case_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1.0 {
    1.0 -> True
    _ -> False
  }
  value
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 2.0 {
    1.0 -> True
    _ -> False
  }
  value
}
"#,
            ),
            Value::Bool(false),
        );
    }

    #[test]
    fn eval_and_short_circuits_false_left() {
        reset_called();
        let actual = eval_and(false, mark_called_true_result).expect("and should evaluate");

        assert!(!actual);
        assert!(!right_called());
    }

    #[test]
    fn eval_and_evaluates_true_left() {
        reset_called();
        let actual = eval_and(true, mark_called_true_result).expect("and should evaluate");

        assert!(actual);
        assert!(right_called());
    }

    #[test]
    fn eval_or_short_circuits_true_left() {
        reset_called();
        let actual = eval_or(true, mark_called_false_result).expect("or should evaluate");

        assert!(actual);
        assert!(!right_called());
    }

    #[test]
    fn eval_or_evaluates_false_left() {
        reset_called();
        let actual = eval_or(false, mark_called_false_result).expect("or should evaluate");

        assert!(!actual);
        assert!(right_called());
    }

    #[test]
    fn eval_and_propagates_right_error() {
        let actual = eval_and(true, function_return_family_error);

        assert_eq!(
            actual.err().map(|error| error.to_string()),
            Some(
                "execution invariant failed: function return family mismatch (expected Bool, got Int)"
                    .into()
            ),
        );
    }

    #[test]
    fn eval_or_propagates_right_error() {
        let actual = eval_or(false, function_return_family_error);

        assert_eq!(
            actual.err().map(|error| error.to_string()),
            Some(
                "execution invariant failed: function return family mismatch (expected Bool, got Int)"
                    .into()
            ),
        );
    }

    #[test]
    fn eval_bool_expr_propagates_operand_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_bool_expr(&plan, &mut frame, &BoolExpr::not(error_bool_expr())),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lt_int(error_int_expr(), IntExpr::value(1.into())),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lt_int(IntExpr::value(1.into()), error_int_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lte_int(error_int_expr(), IntExpr::value(1.into())),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lte_int(IntExpr::value(1.into()), error_int_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gt_int(error_int_expr(), IntExpr::value(1.into())),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gt_int(IntExpr::value(1.into()), error_int_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gte_int(error_int_expr(), IntExpr::value(1.into())),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gte_int(IntExpr::value(1.into()), error_int_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lt_float(error_float_expr(), FloatExpr::value(1.0)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lt_float(FloatExpr::value(1.0), error_float_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lte_float(error_float_expr(), FloatExpr::value(1.0)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::lte_float(FloatExpr::value(1.0), error_float_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gt_float(error_float_expr(), FloatExpr::value(1.0)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gt_float(FloatExpr::value(1.0), error_float_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gte_float(error_float_expr(), FloatExpr::value(1.0)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::gte_float(FloatExpr::value(1.0), error_float_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::equal(
                    Expr::bool(error_bool_expr()),
                    Expr::bool(BoolExpr::value(true))
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::equal(
                    Expr::bool(BoolExpr::value(true)),
                    Expr::bool(error_bool_expr()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::not_equal(
                    Expr::bool(error_bool_expr()),
                    Expr::bool(BoolExpr::value(true)),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::not_equal(
                    Expr::bool(BoolExpr::value(true)),
                    Expr::bool(error_bool_expr()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::and(error_bool_expr(), BoolExpr::value(true)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::string_case(
                    error_string_expr(),
                    vec![("hit".into(), BoolExpr::value(true))],
                    BoolExpr::value(false),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Int,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::float_case(
                    error_float_expr(),
                    vec![(1.0, BoolExpr::value(true))],
                    BoolExpr::value(false),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::or(error_bool_expr(), BoolExpr::value(false)),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::bool_case(
                    error_bool_expr(),
                    BoolExpr::value(true),
                    BoolExpr::value(false),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), BoolExpr::value(true))],
                    BoolExpr::value(false),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_bool_expr(
                &plan,
                &mut frame,
                &BoolExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    BoolExpr::value(true),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
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

    fn mark_called_true_result() -> Result<bool, ExecutionError> {
        Ok(mark_called_true())
    }

    fn mark_called_false_result() -> Result<bool, ExecutionError> {
        Ok(mark_called_false())
    }

    fn function_return_family_error() -> Result<bool, ExecutionError> {
        Err(function_return_family_error_value(
            FunctionReturnFamily::Bool,
            FunctionReturnFamily::Int,
        ))
    }

    fn function_return_family_error_value(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> ExecutionError {
        ExecutionError::function_return_family_mismatch(expected, actual)
    }

    fn mark_called(value: bool) -> bool {
        RIGHT_CALLED.set(true);
        value
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
            ),
            Vec::new(),
        )
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::function_call(
            BoolFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            Vec::new(),
        )
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::function_call(
            IntFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            Vec::new(),
        )
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::function_call(
            StringFunctionExpr::function_call(
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(crate::plan::IntFunctionFunctionId(0)),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                )),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            Vec::new(),
        )
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::function_call(
            FloatFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Float),
            ),
            Vec::new(),
        )
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        ))
    }

    #[test]
    fn eval_bool_case_bool() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = True
  let result = case value {
    True -> False
    False -> True
  }
  result
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
  let result = case value {
    True -> False
    False -> True
  }
  result
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
  let value = case 1 {
    1 -> True
    _ -> False
  }
  value
}
"#,
            ),
            Value::Bool(true),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 2 {
    1 -> True
    _ -> False
  }
  value
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
  let value = {
    1
    True
  }
  value
}
"#,
            ),
            Value::Bool(true),
        );
    }
}
