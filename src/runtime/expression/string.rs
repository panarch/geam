use super::{eval_bool_expr, eval_float_expr, eval_int_expr, project_tuple_expr};
use crate::plan::{ExecutionPlan, StringExpr, StringExprKind, Value, ValueType};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;

pub(in crate::runtime) fn eval_string_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringExpr,
) -> Result<EcoString, ExecutionError> {
    match expression.kind() {
        StringExprKind::Value(value) => Ok(value.clone()),
        StringExprKind::LocalGet { local, .. } => Ok(frame.get_string(*local)),
        StringExprKind::Call { function, args } => {
            function::run_string_call(plan, *function, args, frame)
        }
        StringExprKind::FunctionCall { function, args } => {
            function::run_string_function_call(plan, function, args, frame)
        }
        StringExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, frame, tuple, *index, ValueType::String)? {
                Value::String(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::String,
                    other.value_type(),
                )),
            }
        }
        StringExprKind::Concatenate { left, right } => Ok(format!(
            "{}{}",
            eval_string_expr(plan, frame, left)?,
            eval_string_expr(plan, frame, right)?,
        )
        .into()),
        StringExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
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
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_string_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_string_expr;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, ExecutionPlan, Expr, FloatExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionId, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, TupleExpr, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;
    use crate::runtime::{Value, run_src};

    #[test]
    fn tuple_index_family_mismatch_returns_error() {
        let plan = crate::runtime::plan_src(r#"pub fn main() { "one" }"#);
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::string(StringExpr::value("one".into()))],
            vec![ValueType::String],
        );

        assert_eq!(
            eval_string_expr(&plan, &mut frame, &StringExpr::tuple_index(tuple, 0)),
            Ok("one".into()),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );

        assert_eq!(
            eval_string_expr(&plan, &mut frame, &StringExpr::tuple_index(tuple, 0)),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::String,
                ValueType::Int,
            )),
        );
    }

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

    #[test]
    fn eval_string_case_string() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case "one" {
    "one" -> "one"
    _ -> "other"
  }
  value
}
"#,
            ),
            Value::String("one".into()),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case "many" {
    "one" -> "one"
    _ -> "other"
  }
  value
}
"#,
            ),
            Value::String("other".into()),
        );
    }

    #[test]
    fn eval_float_case_string() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 1.0 {
    1.0 -> "one"
    _ -> "other"
  }
  value
}
"#,
            ),
            Value::String("one".into()),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let value = case 2.0 {
    1.0 -> "one"
    _ -> "other"
  }
  value
}
"#,
            ),
            Value::String("other".into()),
        );
    }

    #[test]
    fn eval_block_string() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    "ignored"
    "geam"
  }
}
"#,
            ),
            Value::String("geam".into()),
        );
    }

    #[test]
    fn eval_string_expr_local_and_calls() {
        assert_eq!(
            run_src(
                r#"
fn called() {
  "called"
}

fn get_called() {
  called
}

pub fn main() {
  let value = "local"
  value <> called() <> get_called()()
}
"#,
            ),
            Value::String("localcalledcalled".into()),
        );
    }

    #[test]
    fn eval_string_expr_propagates_operand_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::concatenate(
                    error_string_expr(),
                    crate::plan::StringExpr::value("right".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::concatenate(
                    crate::plan::StringExpr::value("left".into()),
                    error_string_expr(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::bool_case(
                    error_bool_expr(),
                    crate::plan::StringExpr::value("true".into()),
                    crate::plan::StringExpr::value("false".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), crate::plan::StringExpr::value("one".into()))],
                    crate::plan::StringExpr::value("other".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::string_case(
                    error_string_expr(),
                    vec![("one".into(), crate::plan::StringExpr::value("one".into()))],
                    crate::plan::StringExpr::value("other".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::float_case(
                    error_float_expr(),
                    vec![(1.0, crate::plan::StringExpr::value("one".into()))],
                    crate::plan::StringExpr::value("other".into()),
                ),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Float,
                ValueType::Tuple(Vec::new()),
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &crate::plan::StringExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    crate::plan::StringExpr::value("return".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
    }

    #[test]
    fn eval_string_expr_propagates_return_expression_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &StringExpr::bool_case(
                    BoolExpr::value(true),
                    error_string_expr(),
                    StringExpr::value("false".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &StringExpr::bool_case(
                    BoolExpr::value(false),
                    StringExpr::value("true".into()),
                    error_string_expr(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &StringExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), error_string_expr())],
                    StringExpr::value("other".into()),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &StringExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), StringExpr::value("one".into()))],
                    error_string_expr(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
        assert_eq!(
            eval_string_expr(
                &plan,
                &mut frame,
                &StringExpr::block(Vec::new(), error_string_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
            )),
        );
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

    fn error_string_expr() -> crate::plan::StringExpr {
        crate::plan::StringExpr::function_call(
            StringFunctionExpr::function_call(
                bool_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            Vec::new(),
        )
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::function_call(
            BoolFunctionExpr::function_call(
                string_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            Vec::new(),
        )
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::function_call(
            IntFunctionExpr::function_call(
                string_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            Vec::new(),
        )
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(TupleExpr::value(Vec::new(), Vec::new()), 0)
    }

    fn string_function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        ))
    }

    fn bool_function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Bool(crate::plan::BoolFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Bool),
        ))
    }

    fn function_return_family_error_value(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> ExecutionError {
        ExecutionError::function_return_family_mismatch(expected, actual)
    }
}
