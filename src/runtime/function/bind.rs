use crate::plan::{
    CallArg, CallArgKind, CaptureArg, CaptureArgKind, CaptureValue, CaptureValueKind,
    ExecutionPlan, FrameLayout,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_nil_expr,
    eval_nil_function_expr, eval_string_expr, eval_string_function_expr,
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
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolLocalId, CallArg, CaptureArg,
        ExecutionPlan, FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionValue, FrameLayout, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionFunctionValue, FunctionId, FunctionPlan,
        FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionLocalId, IntLocalId, NilExpr, NilFunctionExpr, NilFunctionLocalId, NilLocalId,
        ReturnExpr, StringExpr, StringFunctionExpr, StringFunctionLocalId, StringLocalId,
        ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn bind_arguments_propagates_typed_argument_evaluation_errors() {
        let plan = plan();

        let cases = [
            CallArg::int(IntLocalId(0), failing_int_expr()),
            CallArg::string(StringLocalId(0), failing_string_expr()),
            CallArg::bool(BoolLocalId(0), failing_bool_expr()),
            CallArg::nil(NilLocalId(0), failing_nil_expr()),
            CallArg::int_function(IntFunctionLocalId(0), failing_int_function_expr()),
            CallArg::string_function(StringFunctionLocalId(0), failing_string_function_expr()),
            CallArg::float_function(FloatFunctionLocalId(0), failing_float_function_expr()),
            CallArg::bool_function(BoolFunctionLocalId(0), failing_bool_function_expr()),
            CallArg::nil_function(NilFunctionLocalId(0), failing_nil_function_expr()),
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
    fn float_function_captures_are_evaluated_and_bound() {
        let plan = plan();
        let captures = eval_capture_args(
            &plan,
            &mut Frame::default(),
            &[CaptureArg::float_function(
                FloatFunctionLocalId(0),
                float_function_expr(),
            )],
        )
        .expect("capture args should evaluate");
        let frame = bind_function_value_arguments(
            &plan,
            &[],
            &mut Frame::default(),
            FrameLayout::default(),
            &captures,
        )
        .expect("captures should bind");

        assert_eq!(
            frame
                .get_float_function(FloatFunctionLocalId(0))
                .runtime_id(),
            FloatFunctionId(0),
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

    fn failing_bool_expr() -> BoolExpr {
        BoolExpr::function_call(failing_bool_function_expr(), Vec::new())
    }

    fn failing_nil_expr() -> NilExpr {
        NilExpr::function_call(failing_nil_function_expr(), Vec::new())
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
        FloatFunctionExpr::value(FloatFunctionValue::new(FloatFunctionId(0), Vec::new()))
    }

    fn failing_bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Bool),
        )
    }

    fn failing_nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Nil),
        )
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
