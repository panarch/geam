use crate::plan::{
    CallArg, CallArgKind, CaptureArg, CaptureArgKind, CaptureValue, CaptureValueKind,
    ExecutionPlan, FrameLayout,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_list_expr,
    eval_list_function_expr, eval_nil_expr, eval_nil_function_expr, eval_string_expr,
    eval_string_function_expr, eval_tuple_expr, eval_tuple_function_expr,
};
use crate::runtime::frame::Frame;

pub(super) fn bind_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: FrameLayout,
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout);
    bind_arguments_into(plan, args, caller_frame, &mut frame)?;
    Ok(frame)
}

pub(super) fn bind_function_value_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: FrameLayout,
    captures: &[CaptureValue],
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout);
    bind_captures(&mut frame, captures);
    bind_arguments_into(plan, args, caller_frame, &mut frame)?;
    Ok(frame)
}

fn bind_arguments_into(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for arg in args {
        match arg.kind() {
            CallArgKind::Int { local, value } => {
                let value = eval_int_expr(plan, caller_frame, value)?;
                frame.set_int(*local, value);
            }
            CallArgKind::String { local, value } => {
                let value = eval_string_expr(plan, caller_frame, value)?;
                frame.set_string(*local, value);
            }
            CallArgKind::Float { local, value } => {
                let value = eval_float_expr(plan, caller_frame, value)?;
                frame.set_float(*local, value);
            }
            CallArgKind::Bool { local, value } => {
                let value = eval_bool_expr(plan, caller_frame, value)?;
                frame.set_bool(*local, value);
            }
            CallArgKind::Nil { local, value } => {
                eval_nil_expr(plan, caller_frame, value)?;
                frame.set_nil(*local);
            }
            CallArgKind::Tuple { local, value } => {
                let value = eval_tuple_expr(plan, caller_frame, value)?;
                frame.set_tuple(*local, value);
            }
            CallArgKind::List { local, value } => {
                let value = eval_list_expr(plan, caller_frame, value)?;
                frame.set_list(*local, value);
            }
            CallArgKind::IntFunction { local, value } => {
                let value = eval_int_function_expr(plan, caller_frame, value)?;
                frame.set_int_function(*local, value);
            }
            CallArgKind::StringFunction { local, value } => {
                let value = eval_string_function_expr(plan, caller_frame, value)?;
                frame.set_string_function(*local, value);
            }
            CallArgKind::FloatFunction { local, value } => {
                let value = eval_float_function_expr(plan, caller_frame, value)?;
                frame.set_float_function(*local, value);
            }
            CallArgKind::BoolFunction { local, value } => {
                let value = eval_bool_function_expr(plan, caller_frame, value)?;
                frame.set_bool_function(*local, value);
            }
            CallArgKind::NilFunction { local, value } => {
                let value = eval_nil_function_expr(plan, caller_frame, value)?;
                frame.set_nil_function(*local, value);
            }
            CallArgKind::TupleFunction { local, value } => {
                let value = eval_tuple_function_expr(plan, caller_frame, value)?;
                frame.set_tuple_function(*local, value);
            }
            CallArgKind::ListFunction { local, value } => {
                let value = eval_list_function_expr(plan, caller_frame, value)?;
                frame.set_list_function(*local, value);
            }
            CallArgKind::FunctionFunction { local, value } => {
                let value = eval_function_function_expr(plan, caller_frame, value)?;
                frame.set_function_function(*local, value);
            }
        }
    }

    Ok(())
}

pub(in crate::runtime) fn eval_capture_args(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    args: &[CaptureArg],
) -> ExecutionResult<Vec<CaptureValue>> {
    let mut captures = Vec::with_capacity(args.len());
    for arg in args {
        captures.push(match arg.kind() {
            CaptureArgKind::Int { local, value } => {
                CaptureValue::int(*local, eval_int_expr(plan, frame, value)?)
            }
            CaptureArgKind::String { local, value } => {
                CaptureValue::string(*local, eval_string_expr(plan, frame, value)?)
            }
            CaptureArgKind::Float { local, value } => {
                CaptureValue::float(*local, eval_float_expr(plan, frame, value)?)
            }
            CaptureArgKind::Bool { local, value } => {
                CaptureValue::bool(*local, eval_bool_expr(plan, frame, value)?)
            }
            CaptureArgKind::Nil { local, value } => {
                eval_nil_expr(plan, frame, value)?;
                CaptureValue::nil(*local)
            }
            CaptureArgKind::Tuple { local, value } => {
                CaptureValue::tuple(*local, eval_tuple_expr(plan, frame, value)?)
            }
            CaptureArgKind::List { local, value } => {
                CaptureValue::list(*local, eval_list_expr(plan, frame, value)?)
            }
            CaptureArgKind::IntFunction { local, value } => {
                CaptureValue::int_function(*local, eval_int_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::StringFunction { local, value } => CaptureValue::string_function(
                *local,
                eval_string_function_expr(plan, frame, value)?,
            ),
            CaptureArgKind::FloatFunction { local, value } => {
                CaptureValue::float_function(*local, eval_float_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::BoolFunction { local, value } => {
                CaptureValue::bool_function(*local, eval_bool_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::NilFunction { local, value } => {
                CaptureValue::nil_function(*local, eval_nil_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::TupleFunction { local, value } => {
                CaptureValue::tuple_function(*local, eval_tuple_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::ListFunction { local, value } => {
                CaptureValue::list_function(*local, eval_list_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::FunctionFunction { local, value } => CaptureValue::function_function(
                *local,
                eval_function_function_expr(plan, frame, value)?,
            ),
        });
    }

    Ok(captures)
}

fn bind_captures(frame: &mut Frame, captures: &[CaptureValue]) {
    for capture in captures {
        match capture.kind() {
            CaptureValueKind::Int { local, value } => frame.set_int(*local, value.clone()),
            CaptureValueKind::String { local, value } => frame.set_string(*local, value.clone()),
            CaptureValueKind::Float { local, value } => frame.set_float(*local, *value),
            CaptureValueKind::Bool { local, value } => frame.set_bool(*local, *value),
            CaptureValueKind::Nil { local } => frame.set_nil(*local),
            CaptureValueKind::Tuple { local, value } => frame.set_tuple(*local, value.clone()),
            CaptureValueKind::List { local, value } => frame.set_list(*local, value.clone()),
            CaptureValueKind::IntFunction { local, value } => {
                frame.set_int_function(*local, value.clone());
            }
            CaptureValueKind::StringFunction { local, value } => {
                frame.set_string_function(*local, value.clone());
            }
            CaptureValueKind::FloatFunction { local, value } => {
                frame.set_float_function(*local, value.clone());
            }
            CaptureValueKind::BoolFunction { local, value } => {
                frame.set_bool_function(*local, value.clone());
            }
            CaptureValueKind::NilFunction { local, value } => {
                frame.set_nil_function(*local, value.clone());
            }
            CaptureValueKind::TupleFunction { local, value } => {
                frame.set_tuple_function(*local, value.clone());
            }
            CaptureValueKind::ListFunction { local, value } => {
                frame.set_list_function(*local, value.clone());
            }
            CaptureValueKind::FunctionFunction { local, value } => {
                frame.set_function_function(*local, value.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{bind_arguments, bind_function_value_arguments, eval_capture_args};
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
        BoolLocalId, CallArg, CaptureArg, CaptureValue, ExecutionPlan, Expr, FloatExpr,
        FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue, FloatLocalId,
        FrameLayout, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType,
        IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, ListExpr, ListFunctionExpr, ListFunctionId,
        ListFunctionLocalId, ListFunctionValue, ListLocalId, ListValue, NilExpr, NilFunctionExpr,
        NilFunctionId, NilFunctionLocalId, NilFunctionValue, NilLocalId, ReturnExpr, StringExpr,
        StringFunctionExpr, StringFunctionId, StringFunctionLocalId, StringFunctionValue,
        StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionId, TupleFunctionLocalId,
        TupleFunctionValue, TupleLocalId, Value, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn bind_arguments_propagates_typed_argument_evaluation_errors() {
        let plan = plan();

        let cases = [
            CallArg::int(IntLocalId(0), failing_int_expr()),
            CallArg::string(StringLocalId(0), failing_string_expr()),
            CallArg::float(FloatLocalId(0), failing_float_expr()),
            CallArg::bool(BoolLocalId(0), failing_bool_expr()),
            CallArg::nil(NilLocalId(0), failing_nil_expr()),
            CallArg::tuple(TupleLocalId(0), failing_tuple_expr()),
            CallArg::list(ListLocalId(0), failing_list_expr()),
            CallArg::int_function(IntFunctionLocalId(0), failing_int_function_expr()),
            CallArg::string_function(StringFunctionLocalId(0), failing_string_function_expr()),
            CallArg::float_function(FloatFunctionLocalId(0), failing_float_function_expr()),
            CallArg::bool_function(BoolFunctionLocalId(0), failing_bool_function_expr()),
            CallArg::nil_function(NilFunctionLocalId(0), failing_nil_function_expr()),
            CallArg::tuple_function(TupleFunctionLocalId(0), failing_tuple_function_expr()),
            CallArg::list_function(ListFunctionLocalId(0), failing_list_function_expr()),
            CallArg::function_function(
                FunctionFunctionLocalId(0),
                failing_function_function_expr(),
            ),
        ];

        for arg in cases {
            assert_expected_function_got_int(bind_arguments(
                &plan,
                &[arg],
                &mut Frame::default(),
                FrameLayout::default(),
            ));
        }
    }

    #[test]
    fn bind_arguments_evaluates_and_binds_all_value_families() {
        let plan = plan();
        let frame = bind_arguments(
            &plan,
            &[
                CallArg::int(IntLocalId(0), IntExpr::value(1.into())),
                CallArg::string(StringLocalId(0), StringExpr::value("one".into())),
                CallArg::float(FloatLocalId(0), FloatExpr::value(1.5)),
                CallArg::bool(BoolLocalId(0), BoolExpr::value(true)),
                CallArg::nil(NilLocalId(0), NilExpr::value()),
                CallArg::tuple(TupleLocalId(0), tuple_expr()),
                CallArg::list(ListLocalId(0), list_expr()),
                CallArg::int_function(IntFunctionLocalId(0), int_function_expr()),
                CallArg::string_function(StringFunctionLocalId(0), string_function_expr()),
                CallArg::float_function(FloatFunctionLocalId(0), float_function_expr()),
                CallArg::bool_function(BoolFunctionLocalId(0), bool_function_expr()),
                CallArg::nil_function(NilFunctionLocalId(0), nil_function_expr()),
                CallArg::tuple_function(TupleFunctionLocalId(0), tuple_function_expr()),
                CallArg::list_function(ListFunctionLocalId(0), list_function_expr()),
                CallArg::function_function(FunctionFunctionLocalId(0), function_function_expr()),
            ],
            &mut Frame::default(),
            all_family_layout(),
        )
        .expect("arguments should bind");

        assert_eq!(frame.get_int(IntLocalId(0)), 1.into());
        assert_eq!(frame.get_string(StringLocalId(0)), "one");
        assert_eq!(frame.get_float(FloatLocalId(0)), 1.5);
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_tuple(TupleLocalId(0)), vec![Value::Int(1.into())]);
        assert_eq!(
            frame.get_list(ListLocalId(0)),
            ListValue::int(vec![1.into()]),
        );
        assert_eq!(
            frame.get_int_function(IntFunctionLocalId(0)).runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            frame
                .get_string_function(StringFunctionLocalId(0))
                .runtime_id(),
            StringFunctionId(0),
        );
        assert_eq!(
            frame
                .get_float_function(FloatFunctionLocalId(0))
                .runtime_id(),
            FloatFunctionId(0),
        );
        assert_eq!(
            frame.get_bool_function(BoolFunctionLocalId(0)).runtime_id(),
            BoolFunctionId(0),
        );
        assert_eq!(
            frame.get_nil_function(NilFunctionLocalId(0)).runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            frame
                .get_tuple_function(TupleFunctionLocalId(0))
                .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            frame.get_list_function(ListFunctionLocalId(0)).runtime_id(),
            ListFunctionId(0),
        );
        assert_eq!(
            frame
                .get_function_function(FunctionFunctionLocalId(0))
                .runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
        );
    }

    #[test]
    fn tuple_function_arguments_are_evaluated_and_bound() {
        let plan = plan();
        let frame = bind_arguments(
            &plan,
            &[CallArg::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_expr(),
            )],
            &mut Frame::default(),
            FrameLayout::default(),
        )
        .expect("arguments should bind");

        assert_eq!(
            frame
                .get_tuple_function(TupleFunctionLocalId(0))
                .runtime_id(),
            TupleFunctionId(0),
        );
    }

    #[test]
    fn bind_function_value_arguments_propagates_argument_evaluation_errors_after_captures() {
        let plan = plan();
        let captures = [CaptureValue::int(IntLocalId(1), 10.into())];
        let mut frame_layout = FrameLayout::default();
        frame_layout.include_int(IntLocalId(1));

        assert_expected_function_got_int(bind_function_value_arguments(
            &plan,
            &[CallArg::int(IntLocalId(0), failing_int_expr())],
            &mut Frame::default(),
            frame_layout,
            &captures,
        ));
    }

    #[test]
    fn eval_capture_args_propagates_typed_argument_evaluation_errors() {
        let plan = plan();

        let cases = [
            CaptureArg::int(IntLocalId(0), failing_int_expr()),
            CaptureArg::string(StringLocalId(0), failing_string_expr()),
            CaptureArg::float(FloatLocalId(0), failing_float_expr()),
            CaptureArg::bool(BoolLocalId(0), failing_bool_expr()),
            CaptureArg::nil(NilLocalId(0), failing_nil_expr()),
            CaptureArg::tuple(TupleLocalId(0), failing_tuple_expr()),
            CaptureArg::list(ListLocalId(0), failing_list_expr()),
            CaptureArg::int_function(IntFunctionLocalId(0), failing_int_function_expr()),
            CaptureArg::string_function(StringFunctionLocalId(0), failing_string_function_expr()),
            CaptureArg::float_function(FloatFunctionLocalId(0), failing_float_function_expr()),
            CaptureArg::bool_function(BoolFunctionLocalId(0), failing_bool_function_expr()),
            CaptureArg::nil_function(NilFunctionLocalId(0), failing_nil_function_expr()),
            CaptureArg::tuple_function(TupleFunctionLocalId(0), failing_tuple_function_expr()),
            CaptureArg::list_function(ListFunctionLocalId(0), failing_list_function_expr()),
            CaptureArg::function_function(
                FunctionFunctionLocalId(0),
                failing_function_function_expr(),
            ),
        ];

        for arg in cases {
            assert_expected_function_got_int(eval_capture_args(
                &plan,
                &mut Frame::default(),
                &[arg],
            ));
        }
    }

    #[test]
    fn eval_capture_args_evaluates_all_capture_families() {
        let plan = plan();
        let captures = eval_capture_args(
            &plan,
            &mut Frame::default(),
            &[
                CaptureArg::int(IntLocalId(0), IntExpr::value(1.into())),
                CaptureArg::string(StringLocalId(0), StringExpr::value("one".into())),
                CaptureArg::float(FloatLocalId(0), FloatExpr::value(1.5)),
                CaptureArg::bool(BoolLocalId(0), BoolExpr::value(true)),
                CaptureArg::nil(NilLocalId(0), NilExpr::value()),
                CaptureArg::tuple(TupleLocalId(0), tuple_expr()),
                CaptureArg::list(ListLocalId(0), list_expr()),
                CaptureArg::int_function(IntFunctionLocalId(0), int_function_expr()),
                CaptureArg::string_function(StringFunctionLocalId(0), string_function_expr()),
                CaptureArg::float_function(FloatFunctionLocalId(0), float_function_expr()),
                CaptureArg::bool_function(BoolFunctionLocalId(0), bool_function_expr()),
                CaptureArg::nil_function(NilFunctionLocalId(0), nil_function_expr()),
                CaptureArg::tuple_function(TupleFunctionLocalId(0), tuple_function_expr()),
                CaptureArg::list_function(ListFunctionLocalId(0), list_function_expr()),
                CaptureArg::function_function(FunctionFunctionLocalId(0), function_function_expr()),
            ],
        )
        .expect("capture args should evaluate");

        assert_eq!(
            captures,
            vec![
                CaptureValue::int(IntLocalId(0), 1.into()),
                CaptureValue::string(StringLocalId(0), "one".into()),
                CaptureValue::float(FloatLocalId(0), 1.5),
                CaptureValue::bool(BoolLocalId(0), true),
                CaptureValue::nil(NilLocalId(0)),
                CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]),
                CaptureValue::list(ListLocalId(0), ListValue::int(vec![1.into()]),),
                CaptureValue::int_function(IntFunctionLocalId(0), int_function_value()),
                CaptureValue::string_function(StringFunctionLocalId(0), string_function_value()),
                CaptureValue::float_function(FloatFunctionLocalId(0), float_function_value()),
                CaptureValue::bool_function(BoolFunctionLocalId(0), bool_function_value()),
                CaptureValue::nil_function(NilFunctionLocalId(0), nil_function_value()),
                CaptureValue::tuple_function(TupleFunctionLocalId(0), tuple_function_value()),
                CaptureValue::list_function(ListFunctionLocalId(0), list_function_value()),
                CaptureValue::function_function(
                    FunctionFunctionLocalId(0),
                    function_function_value(),
                ),
            ],
        );
    }

    #[test]
    fn bind_function_value_arguments_binds_all_capture_families() {
        let plan = plan();
        let frame = bind_function_value_arguments(
            &plan,
            &[],
            &mut Frame::default(),
            all_family_layout(),
            &[
                CaptureValue::int(IntLocalId(0), 1.into()),
                CaptureValue::string(StringLocalId(0), "one".into()),
                CaptureValue::float(FloatLocalId(0), 1.5),
                CaptureValue::bool(BoolLocalId(0), true),
                CaptureValue::nil(NilLocalId(0)),
                CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]),
                CaptureValue::list(ListLocalId(0), ListValue::int(vec![1.into()])),
                CaptureValue::int_function(IntFunctionLocalId(0), int_function_value()),
                CaptureValue::string_function(StringFunctionLocalId(0), string_function_value()),
                CaptureValue::float_function(FloatFunctionLocalId(0), float_function_value()),
                CaptureValue::bool_function(BoolFunctionLocalId(0), bool_function_value()),
                CaptureValue::nil_function(NilFunctionLocalId(0), nil_function_value()),
                CaptureValue::tuple_function(TupleFunctionLocalId(0), tuple_function_value()),
                CaptureValue::list_function(ListFunctionLocalId(0), list_function_value()),
                CaptureValue::function_function(
                    FunctionFunctionLocalId(0),
                    function_function_value(),
                ),
            ],
        )
        .expect("captures should bind");

        assert_eq!(frame.get_int(IntLocalId(0)), 1.into());
        assert_eq!(frame.get_string(StringLocalId(0)), "one");
        assert_eq!(frame.get_float(FloatLocalId(0)), 1.5);
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_tuple(TupleLocalId(0)), vec![Value::Int(1.into())]);
        assert_eq!(
            frame.get_list(ListLocalId(0)),
            ListValue::int(vec![1.into()]),
        );
        assert_eq!(
            frame.get_int_function(IntFunctionLocalId(0)).runtime_id(),
            IntFunctionId(0),
        );
        assert_eq!(
            frame
                .get_string_function(StringFunctionLocalId(0))
                .runtime_id(),
            StringFunctionId(0),
        );
        assert_eq!(
            frame
                .get_float_function(FloatFunctionLocalId(0))
                .runtime_id(),
            FloatFunctionId(0),
        );
        assert_eq!(
            frame.get_bool_function(BoolFunctionLocalId(0)).runtime_id(),
            BoolFunctionId(0),
        );
        assert_eq!(
            frame.get_nil_function(NilFunctionLocalId(0)).runtime_id(),
            NilFunctionId(0),
        );
        assert_eq!(
            frame
                .get_tuple_function(TupleFunctionLocalId(0))
                .runtime_id(),
            TupleFunctionId(0),
        );
        assert_eq!(
            frame.get_list_function(ListFunctionLocalId(0)).runtime_id(),
            ListFunctionId(0),
        );
        assert_eq!(
            frame
                .get_function_function(FunctionFunctionLocalId(0))
                .runtime_id(),
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
        );
    }

    fn assert_expected_function_got_int<T>(actual: Result<T, ExecutionError>) {
        let error = actual.err().expect("call should fail");

        assert_eq!(
            error,
            ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                FunctionReturnFamily::Int,
            ),
        );
    }

    fn failing_function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::function_call(
            FunctionFunctionExpr::value(FunctionFunctionValue::new(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            )),
            Vec::new(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        )
    }

    fn failing_int_expr() -> IntExpr {
        IntExpr::function_call(failing_int_function_expr(), Vec::new())
    }

    fn failing_string_expr() -> StringExpr {
        StringExpr::function_call(failing_string_function_expr(), Vec::new())
    }

    fn failing_float_expr() -> FloatExpr {
        FloatExpr::function_call(failing_float_function_expr(), Vec::new())
    }

    fn failing_bool_expr() -> BoolExpr {
        BoolExpr::function_call(failing_bool_function_expr(), Vec::new())
    }

    fn failing_nil_expr() -> NilExpr {
        NilExpr::function_call(failing_nil_function_expr(), Vec::new())
    }

    fn failing_tuple_expr() -> TupleExpr {
        TupleExpr::function_call(
            failing_tuple_function_expr(),
            Vec::new(),
            vec![ValueType::Int],
        )
    }

    fn failing_list_expr() -> ListExpr {
        ListExpr::function_call(failing_list_function_expr(), Vec::new(), ValueType::Int)
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        )
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(int_function_value())
    }

    fn int_function_value() -> IntFunctionValue {
        IntFunctionValue::new(IntFunctionId(0), Vec::new())
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(string_function_value())
    }

    fn string_function_value() -> StringFunctionValue {
        StringFunctionValue::new(StringFunctionId(0), Vec::new())
    }

    fn failing_int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }

    fn failing_string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        )
    }

    fn failing_float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Float),
        )
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(float_function_value())
    }

    fn float_function_value() -> FloatFunctionValue {
        FloatFunctionValue::new(FloatFunctionId(0), Vec::new())
    }

    fn failing_bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Bool),
        )
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(bool_function_value())
    }

    fn bool_function_value() -> BoolFunctionValue {
        BoolFunctionValue::new(BoolFunctionId(0), Vec::new())
    }

    fn failing_nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Nil),
        )
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(nil_function_value())
    }

    fn nil_function_value() -> NilFunctionValue {
        NilFunctionValue::new(NilFunctionId(0), Vec::new())
    }

    fn failing_tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
        )
    }

    fn failing_list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
        )
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(tuple_function_value())
    }

    fn tuple_function_value() -> TupleFunctionValue {
        TupleFunctionValue::new(TupleFunctionId(0), Vec::new(), vec![ValueType::Int])
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(list_function_value())
    }

    fn list_function_value() -> ListFunctionValue {
        ListFunctionValue::new(ListFunctionId(0), Vec::new(), ValueType::Int)
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(function_function_value())
    }

    fn function_function_value() -> FunctionFunctionValue {
        FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }

    fn all_family_layout() -> FrameLayout {
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        layout.include_string(StringLocalId(0));
        layout.include_float(FloatLocalId(0));
        layout.include_bool(BoolLocalId(0));
        layout.include_nil(NilLocalId(0));
        layout.include_tuple(TupleLocalId(0));
        layout.include_list(ListLocalId(0), ValueType::Int);
        layout.include_int_function(IntFunctionLocalId(0));
        layout.include_string_function(StringFunctionLocalId(0));
        layout.include_float_function(FloatFunctionLocalId(0));
        layout.include_bool_function(BoolFunctionLocalId(0));
        layout.include_nil_function(NilFunctionLocalId(0));
        layout.include_tuple_function(TupleFunctionLocalId(0));
        layout.include_list_function(ListFunctionLocalId(0));
        layout.include_function_function(FunctionFunctionLocalId(0));
        layout
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(crate::plan::IntFunctionId(0), IntExpr::value(1.into())),
            ),
            Vec::new(),
        )
    }
}
