use crate::execution::ExecutionPlan;
use crate::plan::{
    FunctionFunctionExpr, FunctionFunctionExprKind, FunctionFunctionValue, FunctionReturnFamily,
    FunctionValueKind, Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_function_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionFunctionExpr,
) -> Result<FunctionFunctionValue, ExecutionError> {
    match expression.kind() {
        FunctionFunctionExprKind::Value(value) => Ok(value.clone()),
        FunctionFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
            return_type,
        } => Ok(FunctionFunctionValue::new_with_captures(
            runtime_id.clone(),
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
            return_type.clone(),
        )),
        FunctionFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_function_function(*local)),
        FunctionFunctionExprKind::Call { function, args, .. } => {
            function::run_function_function_returning_function_call(plan, *function, args, frame)
        }
        FunctionFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_function_function_function_call(plan, callee, args, frame),
        FunctionFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Function(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        FunctionFunctionExprKind::ListIndex { list, index, type_ } => {
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::Function(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Function,
                    function.kind().family(),
                )),
            }
        }
        FunctionFunctionExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_function_function_expr(plan, frame, true_)
            } else {
                eval_function_function_expr(plan, frame, false_)
            }
        }
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_function_function_expr(plan, frame, branch);
                }
            }
            eval_function_function_expr(plan, frame, fallback)
        }
        FunctionFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_function_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_function_function_expr;
    use crate::execution::ExecutionPlan;
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionValue, IntLocalId, ListElements, ListExpr, PanicExpr, PanicSite, ReturnExpr,
        Step, StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use crate::runtime::{Value, run_src};

    #[test]
    fn eval_function_function_value() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get
}
"#,
        );
    }

    #[test]
    fn eval_function_function_local_get() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  let getter = get
  getter
}
"#,
        );
    }

    #[test]
    fn eval_function_function_panic_returns_error() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    returned_int_function_type(),
                ),
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
    fn eval_function_function_direct_call() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn select() {
  get
}

pub fn main() {
  select()
}
"#,
        );
    }

    #[test]
    fn eval_function_function_value_call() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn select() {
  get
}

pub fn main() {
  let selector = select
  selector()
}
"#,
        );
    }

    #[test]
    fn eval_function_function_bool_case_branches() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case True {
    True -> get
    False -> get_other
  }
}
"#,
        );

        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case False {
    True -> get_other
    False -> get
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_int_case_branches() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 1 {
    1 -> get
    _ -> get_other
  }
}
"#,
        );

        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 2 {
    1 -> get_other
    _ -> get
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_float_case_branches() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 1.0 {
    1.0 -> get
    _ -> get_other
  }
}
"#,
        );

        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn get() {
  add_one
}

fn get_other() {
  add_two
}

pub fn main() {
  case 2.0 {
    1.0 -> get_other
    _ -> get
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_float_case_branches_direct() {
        let plan = plan();
        let mut frame = Frame::default();

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, function_function_value())],
                other_function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::float_case(
                FloatExpr::value(2.0),
                vec![(1.0, other_function_function_value())],
                function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );
    }

    #[test]
    fn eval_function_function_direct_closure_case_and_block_paths() {
        let plan = plan();
        let mut frame = Frame::default();

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
                Vec::new(),
                function_function_type(),
                returned_int_function_type(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_function_value(),
                other_function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::bool_case(
                BoolExpr::value(false),
                other_function_function_value(),
                function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_function_value())],
                other_function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::int_case(
                IntExpr::value(2.into()),
                vec![(1.into(), other_function_function_value())],
                function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::string_case(
                StringExpr::value("hit".into()),
                vec![("hit".into(), function_function_value())],
                other_function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::string_case(
                StringExpr::value("miss".into()),
                vec![("hit".into(), other_function_function_value())],
                function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_function_value(),
            ),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0))
        );
    }

    #[test]
    fn eval_function_function_expr_propagates_operand_errors() {
        assert_tuple_index_error(
            ValueType::Int,
            FunctionFunctionExpr::closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
                vec![CaptureArg::int(IntLocalId(0), error_int_expr())],
                function_function_type(),
                returned_int_function_type(),
            ),
        );
        assert_function_tuple_index_error(FunctionFunctionExpr::tuple_index(
            empty_tuple(),
            0,
            returned_int_function_type(),
        ));
        assert_tuple_index_error(
            ValueType::Bool,
            FunctionFunctionExpr::bool_case(
                error_bool_expr(),
                function_function_value(),
                other_function_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Int,
            FunctionFunctionExpr::int_case(
                error_int_expr(),
                vec![(1.into(), function_function_value())],
                other_function_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::String,
            FunctionFunctionExpr::string_case(
                error_string_expr(),
                vec![("hit".into(), function_function_value())],
                other_function_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Float,
            FunctionFunctionExpr::float_case(
                error_float_expr(),
                vec![(1.0, function_function_value())],
                other_function_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Int,
            FunctionFunctionExpr::block(
                vec![Step::evaluate(Expr::int(error_int_expr()))],
                function_function_value(),
            ),
        );
    }

    #[test]
    fn eval_function_function_block() {
        assert_returns_function_returning_int(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  {
    let getter = get
    getter
  }
}
"#,
        );
    }

    #[test]
    fn eval_function_function_tuple_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::function(
                function_function_value(),
            ))],
            vec![ValueType::Function(Box::new(returned_int_function_type()))],
        );

        let function = eval_function_function_expr(
            &plan,
            &mut frame,
            &FunctionFunctionExpr::tuple_index(tuple, 0, returned_int_function_type()),
        )
        .expect("expression should evaluate");
        assert_eq!(
            function.runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
        );

        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            )))],
            vec![ValueType::Function(Box::new(int_function_type.clone()))],
        );

        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::tuple_index(tuple, 0, returned_int_function_type()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(returned_int_function_type())),
                ValueType::Function(Box::new(int_function_type)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::tuple_index(tuple, 0, returned_int_function_type()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(returned_int_function_type())),
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn eval_function_function_list_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::function(
                function_function_value(),
            ))],
            ValueType::Function(Box::new(function_function_value().type_().clone())),
        );
        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::list_index(
                    list,
                    0,
                    function_function_value().type_().clone()
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
        );

        let list = ListExpr::from_elements(ListElements::Function {
            item_type: function_function_value().type_().clone(),
            values: vec![FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            ))],
        });
        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::list_index(
                    list,
                    0,
                    function_function_value().type_().clone()
                ),
            ),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                FunctionReturnFamily::Int,
            )),
        );

        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::function(
                function_function_value(),
            ))],
            ValueType::Function(Box::new(function_function_value().type_().clone())),
        );
        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::list_index(
                    list,
                    1,
                    function_function_value().type_().clone()
                ),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(function_function_value().type_().clone())),
                1,
                1,
            )),
        );

        let list = ListExpr::tuple_index(
            empty_tuple(),
            0,
            ValueType::Function(Box::new(returned_int_function_type())),
        );
        assert_eq!(
            eval_function_function_expr(
                &plan,
                &mut frame,
                &FunctionFunctionExpr::list_index(list, 0, returned_int_function_type()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Function(Box::new(
                    returned_int_function_type(),
                )))),
                ValueType::Tuple(Vec::new()),
            )),
        );
    }

    fn assert_returns_function_returning_int(src: &str) {
        assert_eq!(
            run_src(src),
            Value::Function(
                FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    returned_int_function_type(),
                )
                .into(),
            ),
        );
    }

    fn returned_int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn function_function_type() -> FunctionType {
        FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(returned_int_function_type())),
        )
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            returned_int_function_type(),
        ))
    }

    fn other_function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)),
            Vec::new(),
            returned_int_function_type(),
        ))
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::from_module_plan(crate::plan::ModulePlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
            ),
            Vec::new(),
        ))
    }

    fn assert_tuple_index_error(expected: ValueType, expression: FunctionFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_function_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(expected)),
        );
    }

    fn assert_function_tuple_index_error(expression: FunctionFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_function_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(ValueType::Function(Box::new(
                returned_int_function_type()
            )))),
        );
    }

    fn tuple_index_error(expected: ValueType) -> ExecutionError {
        ExecutionError::tuple_index_family_mismatch(expected, ValueType::Tuple(Vec::new()))
    }

    fn empty_tuple() -> TupleExpr {
        TupleExpr::value(Vec::new(), Vec::new())
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(empty_tuple(), 0)
    }
}
