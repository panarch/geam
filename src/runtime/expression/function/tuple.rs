use crate::plan::{
    ExecutionPlan, FunctionReturnFamily, FunctionValueKind, TupleFunctionExpr,
    TupleFunctionExprKind, TupleFunctionValue, Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_tuple_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &TupleFunctionExpr,
) -> Result<TupleFunctionValue, ExecutionError> {
    match expression.kind() {
        TupleFunctionExprKind::Value(value) => Ok(value.clone()),
        TupleFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
            return_type,
        } => Ok(TupleFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
            return_type.clone(),
        )),
        TupleFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_tuple_function(*local)),
        TupleFunctionExprKind::Call { function, args, .. } => {
            function::run_tuple_function_returning_function_call(plan, *function, args, frame)
        }
        TupleFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_tuple_function_function_call(plan, callee, args, frame),
        TupleFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            match project_tuple_expr(
                plan,
                frame,
                tuple,
                *index,
                ValueType::Function(Box::new(type_.clone())),
            )? {
                Value::Function(function) => match function.kind() {
                    crate::plan::FunctionValueKind::Tuple(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        ValueType::Function(Box::new(type_.clone())),
                        Value::Function(function).value_type(),
                    )),
                },
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::Function(Box::new(type_.clone())),
                    other.value_type(),
                )),
            }
        }
        TupleFunctionExprKind::ListIndex { list, index, type_ } => {
            let function = project_function_list_expr(plan, frame, list, *index, type_)?;
            match function.kind() {
                FunctionValueKind::Tuple(value) => Ok(value.clone()),
                _ => Err(ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Tuple,
                    function.kind().family(),
                )),
            }
        }
        TupleFunctionExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        TupleFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_tuple_function_expr(plan, frame, true_)
            } else {
                eval_tuple_function_expr(plan, frame, false_)
            }
        }
        TupleFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, frame, fallback)
        }
        TupleFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, frame, fallback)
        }
        TupleFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_function_expr(plan, frame, branch);
                }
            }
            eval_tuple_function_expr(plan, frame, fallback)
        }
        TupleFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_tuple_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_tuple_function_expr;
    use crate::plan::FrameLayout;
    use crate::plan::{
        BoolExpr, CaptureArg, ExecutionPlan, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionValue, FunctionId, FunctionListLocalId, FunctionPlan,
        FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionValue, ListElements, ListExpr, ListLocal, ListValue, PanicExpr, PanicSite,
        ParamLocal, ReturnExpr, Step, StringExpr, TupleExpr, TupleFunctionExpr,
        TupleFunctionFunctionId, TupleFunctionId, TupleFunctionLocalId, TupleFunctionValue,
        TupleLocalId, Value, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::run_src;
    use crate::runtime::{ExecutionError, PanicKind};

    #[test]
    fn eval_tuple_function_value_local_call_function_call_block_and_closure() {
        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

pub fn main() {
  make
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

pub fn main() {
  let f = make
  f
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn get() {
  make
}

pub fn main() {
  get()
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn get() {
  make
}

pub fn main() {
  let getter = get
  getter()
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

pub fn main() {
  {
    make
  }
}
"#,
        );

        assert_returns_tuple_function(
            r#"
pub fn main() {
  fn(value: Int) { #(value, "ok") }
}
"#,
        );
    }

    #[test]
    fn eval_tuple_function_panic_returns_error() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    tuple_function_type(),
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
    fn eval_tuple_function_case_and_tuple_index_paths() {
        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn other(value: Int) {
  #(value, "other")
}

pub fn main() {
  case True {
    True -> make
    False -> other
  }
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn other(value: Int) {
  #(value, "other")
}

pub fn main() {
  case 2 {
    1 -> other
    _ -> make
  }
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn other(value: Int) {
  #(value, "other")
}

pub fn main() {
  case "hit" {
    "hit" -> make
    _ -> other
  }
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn other(value: Int) {
  #(value, "other")
}

pub fn main() {
  case 1.0 {
    2.0 -> other
    _ -> make
  }
}
"#,
        );

        assert_returns_tuple_function(
            r#"
fn make(value: Int) {
  #(value, "ok")
}

fn other(value: Int) {
  #(value, "other")
}

pub fn main() {
  let functions = #(make, other)
  functions.0
}
"#,
        );
    }

    #[test]
    fn eval_tuple_function_expr_propagates_operand_errors() {
        assert_tuple_index_error(
            ValueType::Tuple(tuple_type()),
            TupleFunctionExpr::closure(
                TupleFunctionId(0),
                vec![ParamLocal::tuple(TupleLocalId(0), tuple_type())],
                vec![CaptureArg::tuple(TupleLocalId(0), error_tuple_expr())],
                tuple_function_type(),
                tuple_type(),
            ),
        );
        assert_function_tuple_index_error(TupleFunctionExpr::tuple_index(
            empty_tuple(),
            0,
            tuple_function_type(),
        ));
        assert_tuple_index_error(
            ValueType::Bool,
            TupleFunctionExpr::bool_case(
                error_bool_expr(),
                tuple_function_expr(),
                other_tuple_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Int,
            TupleFunctionExpr::int_case(
                error_int_expr(),
                vec![(1.into(), tuple_function_expr())],
                other_tuple_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::String,
            TupleFunctionExpr::string_case(
                error_string_expr(),
                vec![("hit".into(), tuple_function_expr())],
                other_tuple_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Float,
            TupleFunctionExpr::float_case(
                error_float_expr(),
                vec![(1.0, tuple_function_expr())],
                other_tuple_function_expr(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Tuple(tuple_type()),
            TupleFunctionExpr::block(
                vec![Step::evaluate(Expr::tuple(error_tuple_expr()))],
                tuple_function_expr(),
            ),
        );
    }

    #[test]
    fn eval_tuple_function_direct_expression_paths() {
        let plan = plan();
        let mut frame = Frame::default();
        frame.set_tuple_function(TupleFunctionLocalId(0), tuple_function_value());

        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::closure(
                    TupleFunctionId(0),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                    Vec::new(),
                    tuple_function_type(),
                    tuple_type(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::local_get(
                    TupleFunctionLocalId(0),
                    "make".into(),
                    tuple_function_type(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::call(
                    TupleFunctionFunctionId(0),
                    Vec::new(),
                    tuple_function_type(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::function_call(
                    FunctionFunctionExpr::value(FunctionFunctionValue::new(
                        FunctionFunctionId::Tuple(TupleFunctionFunctionId(0)),
                        Vec::new(),
                        tuple_function_type(),
                    )),
                    Vec::new(),
                    tuple_function_type(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::tuple_index(
                    TupleExpr::value(
                        vec![Expr::function(FunctionExpr::tuple(tuple_function_expr()))],
                        vec![ValueType::Function(Box::new(tuple_function_type()))],
                    ),
                    0,
                    tuple_function_type(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    tuple_function_expr(),
                    other_tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_tuple_function_expr(),
                    tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), tuple_function_expr())],
                    other_tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_tuple_function_expr())],
                    tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), tuple_function_expr())],
                    other_tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), other_tuple_function_expr())],
                    tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, tuple_function_expr())],
                    other_tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, other_tuple_function_expr())],
                    tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    tuple_function_expr(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );
    }

    #[test]
    fn tuple_function_projection_invariant_error() {
        let plan = plan();
        let mut frame = Frame::default();
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let tuple_function_type = tuple_function_expr().type_().clone();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            )))],
            vec![ValueType::Function(Box::new(int_function_type.clone()))],
        );

        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::tuple_index(tuple, 0, tuple_function_type.clone()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(tuple_function_type.clone())),
                ValueType::Function(Box::new(int_function_type)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::tuple_index(tuple, 0, tuple_function_type.clone()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(tuple_function_type.clone())),
                ValueType::Int,
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::tuple_index(tuple, 1, tuple_function_type.clone()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(tuple_function_type)),
                ValueType::Tuple(vec![ValueType::Int]),
            )),
        );
    }

    #[test]
    fn tuple_function_list_projection() {
        let plan = plan();
        let mut frame = Frame::default();
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let tuple_function_type = tuple_function_expr().type_().clone();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::tuple(tuple_function_expr()))],
            ValueType::Function(Box::new(tuple_function_type.clone())),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::list_index(list, 0, tuple_function_type.clone()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            TupleFunctionId(0),
        );

        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            )))],
            ValueType::Function(Box::new(int_function_type.clone())),
        );

        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::list_index(list, 0, tuple_function_type.clone()),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(tuple_function_type.clone())),
                ValueType::Function(Box::new(int_function_type)),
            )),
        );

        let list = ListExpr::from_elements(ListElements::Function {
            item_type: tuple_function_type.clone(),
            values: vec![FunctionExpr::int(IntFunctionExpr::value(
                IntFunctionValue::new(IntFunctionId(0), Vec::new()),
            ))],
        });
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::list_index(list, 0, tuple_function_type.clone()),
            ),
            Err(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::Int,
            )),
        );

        let mut layout = FrameLayout::default();
        layout.include_list(ListLocal::function(
            FunctionListLocalId(0),
            tuple_function_type.clone(),
        ));
        let mut frame = Frame::new(layout);
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            frame.set_list(
                &ListLocal::function(FunctionListLocalId(0), tuple_function_type.clone()),
                ListValue::function(
                    tuple_function_type.clone(),
                    vec![IntFunctionValue::new(IntFunctionId(0), Vec::new()).into()],
                ),
            ),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(tuple_function_type.clone())),
                ValueType::Function(Box::new(int_function_type)),
            )),
        );

        let mut frame = Frame::default();
        let list = ListExpr::value(
            vec![Expr::function(FunctionExpr::tuple(tuple_function_expr()))],
            ValueType::Function(Box::new(tuple_function_type.clone())),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::list_index(list, 1, tuple_function_type.clone()),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Function(Box::new(tuple_function_type.clone())),
                1,
                1,
            )),
        );

        let list = ListExpr::tuple_index(
            empty_tuple(),
            0,
            ValueType::Function(Box::new(tuple_function_type.clone())),
        );
        assert_eq!(
            eval_tuple_function_expr(
                &plan,
                &mut frame,
                &TupleFunctionExpr::list_index(list, 0, tuple_function_type.clone()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Function(Box::new(
                    tuple_function_type.clone(),
                )))),
                ValueType::Tuple(Vec::new()),
            )),
        );
    }

    fn assert_returns_tuple_function(src: &str) {
        assert_eq!(
            run_src(src),
            Value::Function(
                TupleFunctionValue::new(
                    TupleFunctionId(0),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                    tuple_type(),
                )
                .into(),
            ),
        );
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int, ValueType::String]
    }

    fn tuple_function_value() -> TupleFunctionValue {
        TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            tuple_type(),
        )
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Tuple(tuple_type()))
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(tuple_function_value())
    }

    fn other_tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(1),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            tuple_type(),
        ))
    }

    fn assert_tuple_index_error(expected: ValueType, expression: TupleFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_tuple_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(expected)),
        );
    }

    fn assert_function_tuple_index_error(expression: TupleFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_tuple_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(ValueType::Function(Box::new(
                tuple_function_type()
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

    fn error_tuple_expr() -> TupleExpr {
        TupleExpr::tuple_index(empty_tuple(), 0, tuple_type())
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
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "make".into(),
                    vec![crate::plan::Param::named(
                        ParamLocal::int(crate::plan::IntLocalId(0)),
                        "value".into(),
                    )],
                    Vec::new(),
                    ReturnExpr::tuple(
                        TupleFunctionId(0),
                        TupleExpr::value(
                            vec![
                                Expr::int(IntExpr::local_get(
                                    crate::plan::IntLocalId(0),
                                    "value".into(),
                                )),
                                Expr::string(StringExpr::value("ok".into())),
                            ],
                            tuple_type(),
                        ),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "get".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_function(TupleFunctionFunctionId(0), tuple_function_expr()),
                ),
            ],
        )
    }
}
