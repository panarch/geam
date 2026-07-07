use crate::plan::{
    ExecutionPlan, FunctionValueKind, NilFunctionExpr, NilFunctionExprKind, NilFunctionValue,
    Value, ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::expression::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_nil_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &NilFunctionExpr,
) -> Result<NilFunctionValue, ExecutionError> {
    match expression.kind() {
        NilFunctionExprKind::Value(value) => Ok(value.clone()),
        NilFunctionExprKind::Closure {
            runtime_id,
            params,
            captures,
        } => Ok(NilFunctionValue::new_with_captures(
            *runtime_id,
            params.clone(),
            function::eval_capture_args(plan, frame, captures)?,
        )),
        NilFunctionExprKind::LocalGet { local, .. } => Ok(frame.get_nil_function(*local)),
        NilFunctionExprKind::Call { function, args, .. } => {
            function::run_nil_function_returning_function_call(plan, *function, args, frame)
        }
        NilFunctionExprKind::FunctionCall {
            function: callee,
            args,
            ..
        } => function::run_nil_function_function_call(plan, callee, args, frame),
        NilFunctionExprKind::TupleIndex {
            tuple,
            index,
            type_,
        } => {
            let expected = ValueType::Function(Box::new(type_.clone()));
            let value = project_tuple_expr(plan, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type();
            match value {
                Value::Function(function) => match function.kind() {
                    FunctionValueKind::Nil(value) => Ok(value.clone()),
                    _ => Err(ExecutionError::tuple_index_family_mismatch(
                        expected, actual,
                    )),
                },
                _ => Err(ExecutionError::tuple_index_family_mismatch(
                    expected, actual,
                )),
            }
        }
        NilFunctionExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        NilFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_nil_function_expr(plan, frame, true_)
            } else {
                eval_nil_function_expr(plan, frame, false_)
            }
        }
        NilFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, frame, branch);
                }
            }
            eval_nil_function_expr(plan, frame, fallback)
        }
        NilFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, frame, branch);
                }
            }
            eval_nil_function_expr(plan, frame, fallback)
        }
        NilFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_nil_function_expr(plan, frame, branch);
                }
            }
            eval_nil_function_expr(plan, frame, fallback)
        }
        NilFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_nil_function_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_nil_function_expr;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, CaptureArg, ExecutionPlan,
        Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionId,
        NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
        NilFunctionValue, NilLocalId, PanicExpr, PanicSite, ParamLocal, ReturnExpr, Step,
        StringExpr, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};

    #[test]
    fn eval_nil_function_local_closure_call_function_call_and_block() {
        let plan = plan();
        let mut frame = Frame::default();
        frame.set_nil_function(NilFunctionLocalId(0), function_runtime_value());

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::local_get(NilFunctionLocalId(0), "value".into(), type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::closure(
                    NilFunctionId(0),
                    vec![ParamLocal::nil(NilLocalId(0))],
                    Vec::new(),
                    type_(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::call(NilFunctionFunctionId(0), Vec::new(), type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::function_call(
                    FunctionFunctionExpr::value(FunctionFunctionValue::new(
                        FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                        Vec::new(),
                        type_(),
                    )),
                    Vec::new(),
                    type_(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_panic_returns_error() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown()), type_()),
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
    fn eval_nil_function_bool_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    function_value(),
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::bool_case(
                    BoolExpr::value(false),
                    other_function_value(),
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_int_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), function_value())],
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_function_value())],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_string_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), function_value())],
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), other_function_value())],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_float_case_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, function_value())],
                    other_function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, other_function_value())],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_block() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                    function_value(),
                ),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );
    }

    #[test]
    fn eval_nil_function_expr_propagates_operand_errors() {
        assert_tuple_index_error(
            ValueType::Nil,
            NilFunctionExpr::closure(
                NilFunctionId(0),
                vec![ParamLocal::nil(NilLocalId(0))],
                vec![CaptureArg::nil(NilLocalId(0), error_nil_expr())],
                type_(),
            ),
        );
        assert_function_tuple_index_error(NilFunctionExpr::tuple_index(empty_tuple(), 0, type_()));
        assert_tuple_index_error(
            ValueType::Bool,
            NilFunctionExpr::bool_case(error_bool_expr(), function_value(), other_function_value()),
        );
        assert_tuple_index_error(
            ValueType::Int,
            NilFunctionExpr::int_case(
                error_int_expr(),
                vec![(1.into(), function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::String,
            NilFunctionExpr::string_case(
                error_string_expr(),
                vec![("hit".into(), function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Float,
            NilFunctionExpr::float_case(
                error_float_expr(),
                vec![(1.0, function_value())],
                other_function_value(),
            ),
        );
        assert_tuple_index_error(
            ValueType::Nil,
            NilFunctionExpr::block(
                vec![Step::evaluate(Expr::nil(error_nil_expr()))],
                function_value(),
            ),
        );
    }

    #[test]
    fn eval_nil_function_tuple_index() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::nil(function_value()))],
            vec![ValueType::Function(Box::new(type_()))],
        );

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::tuple_index(tuple, 0, type_()),
            )
            .expect("expression should evaluate")
            .runtime_id(),
            NilFunctionId(0),
        );

        let mismatch_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::bool(BoolFunctionExpr::value(
                BoolFunctionValue::new(BoolFunctionId(0), Vec::new()),
            )))],
            vec![ValueType::Function(Box::new(mismatch_type.clone()))],
        );

        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::tuple_index(tuple, 0, type_()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Function(Box::new(mismatch_type)),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            eval_nil_function_expr(
                &plan,
                &mut frame,
                &NilFunctionExpr::tuple_index(tuple, 0, type_()),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(type_())),
                ValueType::Int,
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
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            vec![FunctionPlan::new(
                FunctionId::new(1),
                "get_nil_value".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::nil_function(NilFunctionFunctionId(0), function_value()),
            )],
        )
    }

    fn function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(function_runtime_value())
    }

    fn function_runtime_value() -> NilFunctionValue {
        NilFunctionValue::new(NilFunctionId(0), vec![ParamLocal::nil(NilLocalId(0))])
    }

    fn other_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(1),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn type_() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn assert_tuple_index_error(expected: ValueType, expression: NilFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(expected)),
        );
    }

    fn assert_function_tuple_index_error(expression: NilFunctionExpr) {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_nil_function_expr(&plan, &mut frame, &expression),
            Err(tuple_index_error(ValueType::Function(Box::new(type_())))),
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

    fn error_nil_expr() -> crate::plan::NilExpr {
        crate::plan::NilExpr::tuple_index(empty_tuple(), 0)
    }
}
