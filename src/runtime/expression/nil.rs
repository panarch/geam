use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, NilExpr, NilExprKind, RuntimeFunctionId};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_nil_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilExpr,
) {
    match expression.kind() {
        NilExprKind::Value => {}
        NilExprKind::LocalGet { local, .. } => frame.get_nil(*local),
        NilExprKind::Call { function, args } => {
            function::run_nil_call(plan, *function, args, frame)
        }
        NilExprKind::FunctionCall { function, args } => {
            let function = super::eval_function_expr(plan, frame, function);
            match function.runtime_id() {
                RuntimeFunctionId::Nil(function_id) => {
                    function::run_dynamic_nil_call(plan, function_id, &function, args, frame);
                }
                RuntimeFunctionId::Int(_)
                | RuntimeFunctionId::String(_)
                | RuntimeFunctionId::Bool(_)
                | RuntimeFunctionId::Function(_) => {}
            }
        }
        NilExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_nil_expr(plan, frame, true_);
            } else {
                eval_nil_expr(plan, frame, false_);
            }
        }
        NilExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_expr(plan, frame, branch);
                }
            }
            eval_nil_expr(plan, frame, fallback);
        }
        NilExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_nil_expr(plan, frame, return_);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_nil_expr;
    use crate::plan::{
        BoolFunctionId, ExecutionPlan, Expr, FunctionExpr, FunctionFunctionId, FunctionId,
        FunctionPlan, FunctionType, FunctionValue, IntFunctionId, NilExpr, RuntimeFunctionId,
        StringFunctionId, ValueType,
    };
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
    fn eval_invalid_function_call_return_shape() {
        let plan = empty_plan();
        for runtime_id in [
            RuntimeFunctionId::Int(IntFunctionId(0)),
            RuntimeFunctionId::String(StringFunctionId(0)),
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            RuntimeFunctionId::Function(FunctionFunctionId(0)),
        ] {
            let function = FunctionExpr::value(FunctionValue::new(
                FunctionType::new(Vec::new(), ValueType::Int),
                runtime_id,
                Vec::new(),
            ));
            let expression = NilExpr::function_call(function, Vec::new());

            assert_eq!(eval_nil_expr(&plan, &mut Frame::default(), &expression), ());
        }
    }

    fn empty_plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                Expr::nil(NilExpr::value()),
            ),
            Vec::new(),
        )
    }
}
