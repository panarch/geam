use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, NilExpr, NilExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_nil_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilExpr,
) -> Result<(), ExecutionError> {
    match expression.kind() {
        NilExprKind::Value => Ok(()),
        NilExprKind::LocalGet { local, .. } => {
            frame.get_nil(*local);
            Ok(())
        }
        NilExprKind::Call { function, args } => {
            function::run_nil_call(plan, *function, args, frame)
        }
        NilExprKind::FunctionCall { function, args } => {
            function::run_nil_function_call(plan, function, args, frame)
        }
        NilExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_nil_expr(plan, frame, true_)
            } else {
                eval_nil_expr(plan, frame, false_)
            }
        }
        NilExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, frame, branch);
                }
            }
            eval_nil_expr(plan, frame, fallback)
        }
        NilExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_nil_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_nil_expr;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, ExecutionPlan, Expr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType,
        IntExpr, IntFunctionExpr, IntFunctionId, NilFunctionExpr, ReturnExpr, Step,
        StringFunctionFunctionId, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_nil_value() {
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
    fn eval_bool_case_nil() {
        assert_eq!(
            run_src(
                r#"
fn flag() {
  True
}

pub fn main() {
  case flag() {
    True -> Nil
    False -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );

        assert_eq!(
            run_src(
                r#"
fn flag() {
  False
}

pub fn main() {
  case flag() {
    True -> Nil
    False -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_int_case_nil() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 1 {
    1 -> Nil
    _ -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 2 {
    1 -> Nil
    _ -> Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_block_nil() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    1
    Nil
  }
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_nil_expr_local_and_calls() {
        assert_eq!(
            run_src(
                r#"
fn called() {
  Nil
}

fn get_called() {
  called
}

pub fn main() {
  let value = Nil
  value
  called()
  get_called()()
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn eval_nil_expr_propagates_operand_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::bool_case(
                    error_bool_expr(),
                    crate::plan::NilExpr::value(),
                    crate::plan::NilExpr::value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), crate::plan::NilExpr::value())],
                    crate::plan::NilExpr::value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    crate::plan::NilExpr::value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool
            )),
        );
    }

    #[test]
    fn eval_nil_expr_propagates_return_expression_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::bool_case(
                    BoolExpr::value(true),
                    error_nil_expr(),
                    crate::plan::NilExpr::value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Nil
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::bool_case(
                    BoolExpr::value(false),
                    crate::plan::NilExpr::value(),
                    error_nil_expr(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Nil
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), error_nil_expr())],
                    crate::plan::NilExpr::value(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Nil
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), crate::plan::NilExpr::value())],
                    error_nil_expr(),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Nil
            )),
        );
        assert_eq!(
            eval_nil_expr(
                &plan,
                &mut frame,
                &crate::plan::NilExpr::block(Vec::new(), error_nil_expr()),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Nil
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

    fn error_nil_expr() -> crate::plan::NilExpr {
        crate::plan::NilExpr::function_call(
            NilFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Nil),
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

    fn function_return_family_error_value(expected: FunctionReturnFamily) -> ExecutionError {
        ExecutionError::function_return_family_mismatch(expected, FunctionReturnFamily::String)
    }
}
