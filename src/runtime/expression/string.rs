use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{ExecutionPlan, RuntimeFunctionId, StringExpr, StringExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;

pub(in crate::runtime) fn eval_string_expr(
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
        StringExprKind::FunctionCall { function, args } => {
            let function = super::eval_function_expr(plan, frame, function);
            match function.runtime_id() {
                RuntimeFunctionId::String(function_id) => {
                    function::run_dynamic_string_call(plan, function_id, &function, args, frame)
                }
                RuntimeFunctionId::Int(_)
                | RuntimeFunctionId::Bool(_)
                | RuntimeFunctionId::Nil(_)
                | RuntimeFunctionId::Function(_) => EcoString::default(),
            }
        }
        StringExprKind::Concatenate { left, right } => format!(
            "{}{}",
            eval_string_expr(plan, frame, left),
            eval_string_expr(plan, frame, right),
        )
        .into(),
        StringExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
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
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_string_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_string_expr;
    use crate::plan::{
        BoolFunctionId, ExecutionPlan, Expr, FunctionExpr, FunctionFunctionId, FunctionId,
        FunctionPlan, FunctionType, FunctionValue, IntFunctionId, NilFunctionId, RuntimeFunctionId,
        StringExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{Value, run_src};

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
    fn eval_invalid_function_call_return_shape() {
        let plan = empty_plan();
        for runtime_id in [
            RuntimeFunctionId::Int(IntFunctionId(0)),
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            RuntimeFunctionId::Function(FunctionFunctionId(0)),
        ] {
            let function = FunctionExpr::value(FunctionValue::new(
                FunctionType::new(Vec::new(), ValueType::Int),
                runtime_id,
                Vec::new(),
            ));
            let expression = StringExpr::function_call(function, Vec::new());

            assert_eq!(
                eval_string_expr(&plan, &mut Frame::default(), &expression),
                "",
            );
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
                Expr::string(StringExpr::value("".into())),
            ),
            Vec::new(),
        )
    }
}
