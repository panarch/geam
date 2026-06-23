use super::{eval_bool_expr, eval_int_expr};
use crate::plan::{
    ExecutionPlan, FunctionExpr, FunctionExprKind, FunctionValue, RuntimeFunctionId,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> FunctionValue {
    match expression.kind() {
        FunctionExprKind::Value(value) => value.clone(),
        FunctionExprKind::LocalGet { local, .. } => frame.get_function(*local),
        FunctionExprKind::Call { function, args } => {
            function::run_function_call(plan, *function, args, frame)
        }
        FunctionExprKind::FunctionCall { function, args } => {
            let function = eval_function_expr(plan, frame, function);
            match function.runtime_id() {
                RuntimeFunctionId::Function(function_id) => {
                    function::run_dynamic_function_call(plan, function_id, &function, args, frame)
                }
                RuntimeFunctionId::Int(_)
                | RuntimeFunctionId::String(_)
                | RuntimeFunctionId::Bool(_)
                | RuntimeFunctionId::Nil(_) => invalid_function_value(),
            }
        }
        FunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject) {
                eval_function_expr(plan, frame, true_)
            } else {
                eval_function_expr(plan, frame, false_)
            }
        }
        FunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject);
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_expr(plan, frame, branch);
                }
            }
            eval_function_expr(plan, frame, fallback)
        }
        FunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame);
            eval_function_expr(plan, frame, return_)
        }
    }
}

fn invalid_function_value() -> FunctionValue {
    FunctionValue::new(
        crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Nil),
        RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
        Vec::new(),
    )
}

#[cfg(test)]
mod tests {
    use super::eval_function_expr;
    use crate::plan::{
        ExecutionPlan, Expr, FunctionExpr, FunctionId, FunctionPlan, FunctionType, FunctionValue,
        IntFunctionId, RuntimeFunctionId, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_function_local_get() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_direct_call() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get()
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_value_call() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn get_getter() {
  get
}

pub fn main() {
  get_getter()()
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_bool_case() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case True {
    True -> add_one
    False -> add_one
  }
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_bool_case_false_branch() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case False {
    True -> add_one
    False -> add_one
  }
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_int_case() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case 1 {
    1 -> add_one
    _ -> add_one
  }
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_int_case_fallback() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case 2 {
    1 -> add_one
    _ -> add_one
  }
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_function_block() {
        let function = run_function_src(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  {
    1
    add_one
  }
}
"#,
        );

        assert_int_function(function);
    }

    #[test]
    fn eval_invalid_function_call_return_shape() {
        let plan = empty_plan();
        let function = FunctionExpr::value(FunctionValue::new(
            FunctionType::new(Vec::new(), ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
        ));
        let expression = FunctionExpr::function_call(
            function,
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Nil),
        );

        assert_eq!(
            eval_function_expr(&plan, &mut Frame::default(), &expression)
                .type_()
                .return_(),
            &ValueType::Nil,
        );
    }

    #[test]
    #[should_panic(expected = "main should return a function value")]
    fn run_function_src_panics_on_non_function_value() {
        run_function_src(
            r#"
pub fn main() {
  1
}
"#,
        );
    }

    fn run_function_src(src: &str) -> FunctionValue {
        let Value::Function(function) = run_src(src) else {
            panic!("main should return a function value");
        };

        function
    }

    fn assert_int_function(function: FunctionValue) {
        assert_eq!(function.type_().arguments(), &[ValueType::Int]);
        assert_eq!(function.type_().return_(), &ValueType::Int);
    }

    fn empty_plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                Expr::function(FunctionExpr::value(FunctionValue::new(
                    FunctionType::new(Vec::new(), ValueType::Nil),
                    RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                    Vec::new(),
                ))),
            ),
            Vec::new(),
        )
    }
}
