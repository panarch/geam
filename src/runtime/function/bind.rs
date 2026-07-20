use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{CallArg, CallArgKind, CaptureArg, CaptureArgKind, FrameLayout};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bit_array_expr, eval_bit_array_function_expr, eval_bit_array_list_expr, eval_bool_expr,
    eval_bool_function_expr, eval_bool_list_expr, eval_custom_expr, eval_custom_function_expr,
    eval_custom_list_expr, eval_float_expr, eval_float_function_expr, eval_float_list_expr,
    eval_function_function_expr, eval_function_list_expr, eval_generic_function_expr,
    eval_int_expr, eval_int_function_expr, eval_int_list_expr, eval_list_function_expr,
    eval_list_list_expr, eval_never_function_expr, eval_nil_expr, eval_nil_function_expr,
    eval_nil_list_expr, eval_parameter_list_expr, eval_parameter_list_list_expr, eval_string_expr,
    eval_string_function_expr, eval_string_list_expr, eval_tuple_expr, eval_tuple_function_expr,
    eval_tuple_list_expr, eval_typed_custom_function_expr, eval_typed_function_expr,
    eval_utf_codepoint_expr, eval_utf_codepoint_function_expr, eval_utf_codepoint_list_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture, EvaluatedValue,
};

pub(super) fn bind_arguments(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: &FrameLayout,
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout, state);
    bind_arguments_into(plan, state, args, caller_frame, &mut frame)?;
    Ok(frame)
}

pub(super) fn bind_function_value_arguments(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: &FrameLayout,
    captures: &[EvaluatedCapture],
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout, state);
    bind_captures(&mut frame, captures);
    bind_arguments_into(plan, state, args, caller_frame, &mut frame)?;
    Ok(frame)
}

fn bind_arguments_into(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for arg in args {
        match arg.kind() {
            CallArgKind::Int { local, value } => {
                let value = eval_int_expr(plan, state, caller_frame, value)?;
                frame.set_int(*local, value);
            }
            CallArgKind::String { local, value } => {
                let value = eval_string_expr(plan, state, caller_frame, value)?;
                frame.set_string(*local, value);
            }
            CallArgKind::BitArray { local, value } => {
                let value = eval_bit_array_expr(plan, state, caller_frame, value)?;
                frame.set_bit_array(*local, value);
            }
            CallArgKind::UtfCodepoint { local, value } => {
                let value = eval_utf_codepoint_expr(plan, state, caller_frame, value)?;
                frame.set_utf_codepoint(*local, value);
            }
            CallArgKind::Custom(binding) => {
                let value = eval_custom_expr(plan, state, caller_frame, binding.value())?;
                frame.set_custom(binding.local(), value);
            }
            CallArgKind::Float { local, value } => {
                let value = eval_float_expr(plan, state, caller_frame, value)?;
                frame.set_float(*local, value);
            }
            CallArgKind::Bool { local, value } => {
                let value = eval_bool_expr(plan, state, caller_frame, value)?;
                frame.set_bool(*local, value);
            }
            CallArgKind::Nil { local, value } => {
                eval_nil_expr(plan, state, caller_frame, value)?;
                frame.set_nil(*local);
            }
            CallArgKind::Tuple { local, value } => {
                let value = eval_tuple_expr(plan, state, caller_frame, value)?;
                frame.set_tuple(*local, value);
            }
            CallArgKind::List(value) => {
                bind_list_argument(plan, state, caller_frame, frame, value)?
            }
            CallArgKind::IntFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_int_function_expr,
                )?;
                frame.set_int_function(*local, value);
            }
            CallArgKind::StringFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_string_function_expr,
                )?;
                frame.set_string_function(*local, value);
            }
            CallArgKind::BitArrayFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_bit_array_function_expr,
                )?;
                frame.set_bit_array_function(*local, value);
            }
            CallArgKind::UtfCodepointFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_utf_codepoint_function_expr,
                )?;
                frame.set_utf_codepoint_function(*local, value);
            }
            CallArgKind::CustomFunction { local, value } => {
                let value = eval_typed_custom_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_custom_function_expr,
                )?;
                frame.set_custom_function(local, value);
            }
            CallArgKind::GenericFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_generic_function_expr,
                )?;
                frame.set_generic_function(local, value);
            }
            CallArgKind::NeverFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_never_function_expr,
                )?;
                frame.set_never_function(local, value);
            }
            CallArgKind::FloatFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_float_function_expr,
                )?;
                frame.set_float_function(*local, value);
            }
            CallArgKind::BoolFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_bool_function_expr,
                )?;
                frame.set_bool_function(*local, value);
            }
            CallArgKind::NilFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_nil_function_expr,
                )?;
                frame.set_nil_function(*local, value);
            }
            CallArgKind::TupleFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_tuple_function_expr,
                )?;
                frame.set_tuple_function(*local, value);
            }
            CallArgKind::ListFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_list_function_expr,
                )?;
                frame.set_list_function(local.clone(), value);
            }
            CallArgKind::FunctionFunction { local, value } => {
                let value = eval_typed_function_expr(
                    plan,
                    state,
                    caller_frame,
                    value,
                    eval_function_function_expr,
                )?;
                frame.set_function_function(local, value);
            }
        }
    }

    Ok(())
}

pub(in crate::runtime) fn eval_call_argument_values(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    args: &[CallArg],
    frame: &mut Frame,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        values.push(match arg.kind() {
            CallArgKind::Int { value, .. } => {
                EvaluatedValue::Int(eval_int_expr(plan, state, frame, value)?)
            }
            CallArgKind::Float { value, .. } => {
                EvaluatedValue::Float(eval_float_expr(plan, state, frame, value)?)
            }
            CallArgKind::String { value, .. } => {
                EvaluatedValue::String(eval_string_expr(plan, state, frame, value)?)
            }
            CallArgKind::BitArray { value, .. } => {
                EvaluatedValue::BitArray(eval_bit_array_expr(plan, state, frame, value)?)
            }
            CallArgKind::UtfCodepoint { value, .. } => {
                EvaluatedValue::UtfCodepoint(eval_utf_codepoint_expr(plan, state, frame, value)?)
            }
            CallArgKind::Custom(binding) => {
                EvaluatedValue::Custom(eval_custom_expr(plan, state, frame, binding.value())?)
            }
            CallArgKind::Bool { value, .. } => {
                EvaluatedValue::Bool(eval_bool_expr(plan, state, frame, value)?)
            }
            CallArgKind::Nil { value, .. } => {
                eval_nil_expr(plan, state, frame, value)?;
                EvaluatedValue::Nil
            }
            CallArgKind::Tuple { value, .. } => {
                EvaluatedValue::Tuple(eval_tuple_expr(plan, state, frame, value)?)
            }
            CallArgKind::List(value) => {
                EvaluatedValue::List(eval_list_local_expr(plan, state, frame, value)?)
            }
            CallArgKind::IntFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_int_function_expr)?.into(),
            ),
            CallArgKind::FloatFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_float_function_expr)?
                    .into(),
            ),
            CallArgKind::StringFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_string_function_expr)?
                    .into(),
            ),
            CallArgKind::BitArrayFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_bit_array_function_expr)?
                    .into(),
            ),
            CallArgKind::UtfCodepointFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(
                    plan,
                    state,
                    frame,
                    value,
                    eval_utf_codepoint_function_expr,
                )?
                .into(),
            ),
            CallArgKind::CustomFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_custom_function_expr(
                    plan,
                    state,
                    frame,
                    value,
                    eval_custom_function_expr,
                )?
                .into(),
            ),
            CallArgKind::GenericFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_generic_function_expr)?
                    .into(),
            ),
            CallArgKind::NeverFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_never_function_expr)?
                    .into(),
            ),
            CallArgKind::BoolFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_bool_function_expr)?
                    .into(),
            ),
            CallArgKind::NilFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_nil_function_expr)?.into(),
            ),
            CallArgKind::TupleFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_tuple_function_expr)?
                    .into(),
            ),
            CallArgKind::ListFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_list_function_expr)?
                    .into(),
            ),
            CallArgKind::FunctionFunction { value, .. } => EvaluatedValue::Function(
                eval_typed_function_expr(plan, state, frame, value, eval_function_function_expr)?
                    .into(),
            ),
        });
    }
    Ok(values)
}

fn eval_list_local_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<crate::runtime::state::ListValueId> {
    match value {
        crate::plan::execution::ListLocalExpr::Parameter { value, .. } => {
            eval_parameter_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::ParameterList { value, .. } => {
            eval_parameter_list_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Int { value, .. } => {
            eval_int_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::String { value, .. } => {
            eval_string_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::BitArray { value, .. } => {
            eval_bit_array_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::UtfCodepoint { value, .. } => {
            eval_utf_codepoint_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Custom { value, .. } => {
            eval_custom_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Float { value, .. } => {
            eval_float_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Bool { value, .. } => {
            eval_bool_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Nil { value, .. } => {
            eval_nil_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Tuple { value, .. } => {
            eval_tuple_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::List { value, .. } => {
            eval_list_list_expr(plan, state, frame, value).map(Into::into)
        }
        crate::plan::execution::ListLocalExpr::Function { value, .. } => {
            eval_function_list_expr(plan, state, frame, value).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn eval_capture_args(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    args: &[CaptureArg],
) -> ExecutionResult<Vec<EvaluatedCapture>> {
    let mut captures = Vec::with_capacity(args.len());
    for arg in args {
        captures.push(match arg.kind() {
            CaptureArgKind::Int { local, value } => {
                EvaluatedCapture::int(*local, eval_int_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::String { local, value } => {
                EvaluatedCapture::string(*local, eval_string_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::BitArray { local, value } => {
                EvaluatedCapture::bit_array(*local, eval_bit_array_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::UtfCodepoint { local, value } => EvaluatedCapture::utf_codepoint(
                *local,
                eval_utf_codepoint_expr(plan, state, frame, value)?,
            ),
            CaptureArgKind::Custom(binding) => EvaluatedCapture::custom(
                binding.local(),
                eval_custom_expr(plan, state, frame, binding.value())?,
            ),
            CaptureArgKind::Float { local, value } => {
                EvaluatedCapture::float(*local, eval_float_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::Bool { local, value } => {
                EvaluatedCapture::bool(*local, eval_bool_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::Nil { local, value } => {
                eval_nil_expr(plan, state, frame, value)?;
                EvaluatedCapture::nil(*local)
            }
            CaptureArgKind::Tuple { local, value } => {
                EvaluatedCapture::tuple(*local, eval_tuple_expr(plan, state, frame, value)?)
            }
            CaptureArgKind::List(value) => eval_list_capture(plan, state, frame, value)?,
            CaptureArgKind::IntFunction { local, value } => EvaluatedCapture::int_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_int_function_expr)?,
            ),
            CaptureArgKind::StringFunction { local, value } => EvaluatedCapture::string_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_string_function_expr)?,
            ),
            CaptureArgKind::BitArrayFunction { local, value } => {
                EvaluatedCapture::bit_array_function(
                    *local,
                    eval_typed_function_expr(
                        plan,
                        state,
                        frame,
                        value,
                        eval_bit_array_function_expr,
                    )?,
                )
            }
            CaptureArgKind::UtfCodepointFunction { local, value } => {
                EvaluatedCapture::utf_codepoint_function(
                    *local,
                    eval_typed_function_expr(
                        plan,
                        state,
                        frame,
                        value,
                        eval_utf_codepoint_function_expr,
                    )?,
                )
            }
            CaptureArgKind::CustomFunction { local, value } => EvaluatedCapture::custom_function(
                local.clone(),
                eval_typed_custom_function_expr(
                    plan,
                    state,
                    frame,
                    value,
                    eval_custom_function_expr,
                )?,
            ),
            CaptureArgKind::GenericFunction { local, value } => EvaluatedCapture::generic_function(
                local.clone(),
                eval_typed_function_expr(plan, state, frame, value, eval_generic_function_expr)?,
            ),
            CaptureArgKind::NeverFunction { local, value } => EvaluatedCapture::never_function(
                local.clone(),
                eval_typed_function_expr(plan, state, frame, value, eval_never_function_expr)?,
            ),
            CaptureArgKind::FloatFunction { local, value } => EvaluatedCapture::float_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_float_function_expr)?,
            ),
            CaptureArgKind::BoolFunction { local, value } => EvaluatedCapture::bool_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_bool_function_expr)?,
            ),
            CaptureArgKind::NilFunction { local, value } => EvaluatedCapture::nil_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_nil_function_expr)?,
            ),
            CaptureArgKind::TupleFunction { local, value } => EvaluatedCapture::tuple_function(
                *local,
                eval_typed_function_expr(plan, state, frame, value, eval_tuple_function_expr)?,
            ),
            CaptureArgKind::ListFunction { local, value } => EvaluatedCapture::list_function(
                local.clone(),
                eval_typed_function_expr(plan, state, frame, value, eval_list_function_expr)?,
            ),
            CaptureArgKind::FunctionFunction { local, value } => {
                EvaluatedCapture::function_function(
                    local.clone(),
                    eval_typed_function_expr(
                        plan,
                        state,
                        frame,
                        value,
                        eval_function_function_expr,
                    )?,
                )
            }
        });
    }

    Ok(captures)
}

fn bind_captures(frame: &mut Frame, captures: &[EvaluatedCapture]) {
    for capture in captures {
        match capture.kind() {
            EvaluatedCaptureKind::Int { local, value } => frame.set_int(*local, value.clone()),
            EvaluatedCaptureKind::String { local, value } => {
                frame.set_string(*local, value.clone())
            }
            EvaluatedCaptureKind::BitArray { local, value } => {
                frame.set_bit_array(*local, value.clone())
            }
            EvaluatedCaptureKind::UtfCodepoint { local, value } => {
                frame.set_utf_codepoint(*local, *value)
            }
            EvaluatedCaptureKind::Custom { local, value } => {
                frame.set_custom(*local, value.clone())
            }
            EvaluatedCaptureKind::Float { local, value } => frame.set_float(*local, *value),
            EvaluatedCaptureKind::Bool { local, value } => frame.set_bool(*local, *value),
            EvaluatedCaptureKind::Nil { local } => frame.set_nil(*local),
            EvaluatedCaptureKind::Tuple { local, value } => frame.set_tuple(*local, value.clone()),
            EvaluatedCaptureKind::List(value) => bind_list_capture(frame, value),
            EvaluatedCaptureKind::IntFunction { local, value } => {
                frame.set_int_function(*local, value.clone());
            }
            EvaluatedCaptureKind::StringFunction { local, value } => {
                frame.set_string_function(*local, value.clone());
            }
            EvaluatedCaptureKind::BitArrayFunction { local, value } => {
                frame.set_bit_array_function(*local, value.clone());
            }
            EvaluatedCaptureKind::UtfCodepointFunction { local, value } => {
                frame.set_utf_codepoint_function(*local, value.clone());
            }
            EvaluatedCaptureKind::CustomFunction { local, value } => {
                frame.set_custom_function(local, value.clone());
            }
            EvaluatedCaptureKind::GenericFunction { local, value } => {
                frame.set_generic_function(local, value.clone());
            }
            EvaluatedCaptureKind::NeverFunction { local, value } => {
                frame.set_never_function(local, value.clone());
            }
            EvaluatedCaptureKind::FloatFunction { local, value } => {
                frame.set_float_function(*local, value.clone());
            }
            EvaluatedCaptureKind::BoolFunction { local, value } => {
                frame.set_bool_function(*local, value.clone());
            }
            EvaluatedCaptureKind::NilFunction { local, value } => {
                frame.set_nil_function(*local, value.clone());
            }
            EvaluatedCaptureKind::TupleFunction { local, value } => {
                frame.set_tuple_function(*local, value.clone());
            }
            EvaluatedCaptureKind::ListFunction { local, value } => {
                frame.set_list_function(local.clone(), value.clone());
            }
            EvaluatedCaptureKind::FunctionFunction { local, value } => {
                frame.set_function_function(local, value.clone());
            }
        }
    }
}

fn bind_list_argument(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    caller_frame: &mut Frame,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<()> {
    match value {
        crate::plan::execution::ListLocalExpr::Parameter { local, value } => {
            let value = eval_parameter_list_expr(plan, state, caller_frame, value)?;
            frame.set_parameter_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::ParameterList { local, value } => {
            let value = eval_parameter_list_list_expr(plan, state, caller_frame, value)?;
            frame.set_parameter_list_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            let value = eval_int_list_expr(plan, state, caller_frame, value)?;
            frame.set_int_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            let value = eval_string_list_expr(plan, state, caller_frame, value)?;
            frame.set_string_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::BitArray { local, value } => {
            let value = eval_bit_array_list_expr(plan, state, caller_frame, value)?;
            frame.set_bit_array_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::UtfCodepoint { local, value } => {
            let value = eval_utf_codepoint_list_expr(plan, state, caller_frame, value)?;
            frame.set_utf_codepoint_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Custom { local, value } => {
            let value = eval_custom_list_expr(plan, state, caller_frame, value)?;
            frame.set_custom_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            let value = eval_float_list_expr(plan, state, caller_frame, value)?;
            frame.set_float_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            let value = eval_bool_list_expr(plan, state, caller_frame, value)?;
            frame.set_bool_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            let value = eval_nil_list_expr(plan, state, caller_frame, value)?;
            frame.set_nil_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Tuple { local, value, .. } => {
            let value = eval_tuple_list_expr(plan, state, caller_frame, value)?;
            frame.set_tuple_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::List { local, value, .. } => {
            let value = eval_list_list_expr(plan, state, caller_frame, value)?;
            frame.set_list_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Function { local, value, .. } => {
            let value = eval_function_list_expr(plan, state, caller_frame, value)?;
            frame.set_function_list(*local, value);
        }
    }
    Ok(())
}

fn eval_list_capture(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<EvaluatedCapture> {
    Ok(match value {
        crate::plan::execution::ListLocalExpr::Parameter { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Parameter {
                local: *local,
                value: eval_parameter_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::ParameterList { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::ParameterList {
                local: *local,
                value: eval_parameter_list_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Int {
                local: *local,
                value: eval_int_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::String {
                local: *local,
                value: eval_string_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::BitArray { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::BitArray {
                local: *local,
                value: eval_bit_array_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::UtfCodepoint { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::UtfCodepoint {
                local: *local,
                value: eval_utf_codepoint_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Custom { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Custom {
                local: *local,
                value: eval_custom_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Float {
                local: *local,
                value: eval_float_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Bool {
                local: *local,
                value: eval_bool_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Nil {
                local: *local,
                value: eval_nil_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Tuple { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                local: *local,
                value: eval_tuple_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::List { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::List {
                local: *local,
                value: eval_list_list_expr(plan, state, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Function { local, value } => {
            EvaluatedCapture::list(EvaluatedListCapture::Function {
                local: *local,
                value: eval_function_list_expr(plan, state, frame, value)?,
            })
        }
    })
}

fn bind_list_capture(frame: &mut Frame, value: &EvaluatedListCapture) {
    match value {
        EvaluatedListCapture::Parameter { local, value } => {
            frame.set_parameter_list(*local, *value);
        }
        EvaluatedListCapture::ParameterList { local, value } => {
            frame.set_parameter_list_list(*local, value.clone());
        }
        EvaluatedListCapture::Int { local, value } => {
            frame.set_int_list(*local, value.clone());
        }
        EvaluatedListCapture::String { local, value } => {
            frame.set_string_list(*local, value.clone());
        }
        EvaluatedListCapture::BitArray { local, value } => {
            frame.set_bit_array_list(*local, value.clone());
        }
        EvaluatedListCapture::UtfCodepoint { local, value } => {
            frame.set_utf_codepoint_list(*local, value.clone());
        }
        EvaluatedListCapture::Custom { local, value } => {
            frame.set_custom_list(*local, value.clone());
        }
        EvaluatedListCapture::Float { local, value } => {
            frame.set_float_list(*local, value.clone());
        }
        EvaluatedListCapture::Bool { local, value } => {
            frame.set_bool_list(*local, value.clone());
        }
        EvaluatedListCapture::Nil { local, value } => {
            frame.set_nil_list(*local, value.clone());
        }
        EvaluatedListCapture::Tuple { local, value } => {
            frame.set_tuple_list(*local, value.clone());
        }
        EvaluatedListCapture::List { local, value } => {
            frame.set_list_list(*local, value.clone());
        }
        EvaluatedListCapture::Function { local, value } => {
            frame.set_function_list(*local, value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{bind_captures, bind_list_capture};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayListExpr,
        BitArrayListLocalId, BitArrayLocalId, BoolExpr, BoolFunctionExpr, BoolFunctionLocalId,
        BoolListExpr, BoolListLocalId, CaptureArg, CustomConstructorDefinition, CustomExpr,
        CustomFieldDefinition, CustomFunctionExpr, CustomFunctionLocal, CustomFunctionLocalId,
        CustomFunctionType, CustomListLocalId, CustomLocal, CustomLocalId, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypePublicity, CustomTypeTemplate, Expr,
        FloatExpr, FloatFunctionExpr, FloatFunctionLocalId, FloatListExpr, FloatListLocalId,
        FunctionExpr, FunctionFunctionExpr, FunctionFunctionLocal, FunctionFunctionLocalId,
        FunctionFunctionType, FunctionListLocalId, FunctionTemplate, FunctionTemplateId,
        FunctionType, GenericFunctionExpr, GenericFunctionLocal, GenericFunctionLocalId,
        GenericFunctionType, GenericListLocalId, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntFunctionLocalId, IntListExpr, IntListLocalId, IntLocalId, ListExpr,
        ListFunctionExpr, ListFunctionLocal, ListListExpr, ListListLocalId, ListLocal,
        ListLocalExpr, ModulePlan, NilExpr, NilFunctionExpr, NilFunctionLocalId, NilListExpr,
        NilListLocalId, NilLocalId, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionLocalId, StringListExpr, StringListLocalId,
        StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionLocalId, TupleListExpr,
        TupleListLocalId, TupleLocalId, TypeParameterId, TypedFunctionExpr, UtfCodepointExpr,
        UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId, UtfCodepointListExpr,
        UtfCodepointListLocalId, UtfCodepointLocalId, ValueShape, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{EvaluatedValue, ExecutionError, InvariantError, run_main};

    #[test]
    fn source_arguments_and_captures_preserve_every_value_family() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/argument/list_boundary_item_families.gleam"
                ),
                crate::runtime::Value::Int(1.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_value_families.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_return_shapes.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src(source), expected);
        }

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_list_function.gleam"
            )),
            crate::runtime::Value::List(crate::runtime::ListValue::int(vec![1.into(), 2.into()])),
        );

        assert_eq!(
            crate::runtime::run_src(
                r#"
fn int_value() { 1 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn float_value() { 1.0 }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

pub fn main() {
  let int = 1
  let string = "one"
  let bit_array = <<1>>
  let float = 1.0
  let bool = True
  let nil = Nil
  let tuple = #(1)
  let int_list = [1]
  let string_list = ["one"]
  let bit_array_list = [<<1>>]
  let float_list = [1.0]
  let bool_list = [True]
  let nil_list = [Nil]
  let tuple_list = [#(1)]
  let list_list = [[1]]
  let function_list = [int_value]
  let int_function = int_value
  let string_function = string_value
  let bit_array_function = bit_array_value
  let float_function = float_value
  let bool_function = bool_value
  let nil_function = nil_value
  let tuple_function = tuple_value
  let list_function = list_value
  let function_function = function_value

  let closure = fn() {
    assert int == 1
    assert string == "one"
    assert bit_array == <<1>>
    assert float == 1.0
    assert bool
    nil
    assert tuple == #(1)
    assert int_list == [1]
    assert string_list == ["one"]
    assert bit_array_list == [<<1>>]
    assert float_list == [1.0]
    assert bool_list == [True]
    assert nil_list == [Nil]
    assert tuple_list == [#(1)]
    assert list_list == [[1]]
    assert case function_list { [function] -> function() == 1 _ -> False }
    assert int_function() == 1
    assert string_function() == "one"
    assert bit_array_function() == <<1>>
    assert float_function() == 1.0
    assert bool_function()
    nil_function()
    assert tuple_function() == #(1)
    assert list_function() == [1]
    assert function_function()() == 1
    42
  }

  closure()
}
"#,
            ),
            crate::runtime::Value::Int(42.into()),
        );

        assert_eq!(
            crate::runtime::run_src(
                r#"
fn int_value() { 1 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn float_value() { 1.0 }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

fn accept_int(function: fn() -> Int) { function() }
fn accept_string(function: fn() -> String) { function() }
fn accept_bit_array(function: fn() -> BitArray) { function() }
fn accept_float(function: fn() -> Float) { function() }
fn accept_bool(function: fn() -> Bool) { function() }
fn accept_nil(function: fn() -> Nil) { function() }
fn accept_tuple(function: fn() -> #(Int)) { function() }
fn accept_list(function: fn() -> List(Int)) { function() }
fn accept_function(function: fn() -> fn() -> Int) { function()() }

pub fn main() {
  assert accept_int(int_value) == 1
  assert accept_string(string_value) == "one"
  assert accept_bit_array(bit_array_value) == <<1>>
  assert accept_float(float_value) == 1.0
  assert accept_bool(bool_value)
  accept_nil(nil_value)
  assert accept_tuple(tuple_value) == #(1)
  assert accept_list(list_value) == [1]
  assert accept_function(function_value) == 1
  42
}
"#,
            ),
            crate::runtime::Value::Int(42.into()),
        );
    }

    #[test]
    fn source_argument_errors_propagate_for_every_value_family() {
        let parameter_types = [
            "Int",
            "String",
            "BitArray",
            "UtfCodepoint",
            "Float",
            "Bool",
            "Nil",
            "#(Int)",
            "List(Int)",
            "List(String)",
            "List(BitArray)",
            "List(UtfCodepoint)",
            "List(Float)",
            "List(Bool)",
            "List(Nil)",
            "List(#(Int))",
            "List(List(Int))",
            "List(value)",
            "List(List(value))",
            "List(fn() -> Int)",
            "fn() -> Int",
            "fn() -> String",
            "fn() -> BitArray",
            "fn() -> UtfCodepoint",
            "fn() -> Float",
            "fn() -> Bool",
            "fn() -> Nil",
            "fn() -> #(Int)",
            "fn() -> List(Int)",
            "fn() -> List(BitArray)",
            "fn() -> List(UtfCodepoint)",
            "fn() -> fn() -> Int",
            "fn(value) -> value",
            "fn(Int) -> value",
        ];

        for parameter_type in parameter_types {
            let source = format!(
                "fn callee(value: {parameter_type}) -> Nil {{ Nil }} pub fn main() {{ callee(panic as \"argument\") }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: argument",
            );
        }

        for parameter_type in ["Boxed", "List(Boxed)", "fn() -> Boxed"] {
            let source = format!(
                "pub type Boxed {{ Boxed(Int) }} fn callee(value: {parameter_type}) -> Nil {{ Nil }} pub fn main() {{ callee(panic as \"argument\") }}",
            );
            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: argument",
            );
        }

        for source in [
            r#"
fn fail_condition() -> Bool { panic as "argument" }
fn callee(_function: fn(value) -> value) { Nil }
pub fn main() {
  callee(case fail_condition() {
    True -> fn(value) { value }
    False -> fn(value) { value }
  })
}
"#,
            r#"
fn fail_condition() -> Bool { panic as "argument" }
fn callee(_function: fn(Int) -> value) { Nil }
pub fn main() {
  callee(case fail_condition() {
    True -> fn(_value) { panic }
    False -> fn(_value) { panic }
  })
}
"#,
        ] {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: argument",
            );
        }
    }

    #[test]
    fn generic_function_argument_projection_errors_propagate() {
        let cases = [
            (
                r#"
fn consume(_function: fn(value) -> value) { Nil }
fn caller(functions: #(fn(value) -> value)) { consume(functions.0) }
pub fn main() { caller }
"#,
                FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(0))],
                    ValueType::Parameter(TypeParameterId(0)),
                ),
            ),
            (
                r#"
fn consume(_function: fn(Int) -> value) { Nil }
fn caller(functions: #(fn(Int) -> value)) { consume(functions.0) }
pub fn main() { caller }
"#,
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Parameter(TypeParameterId(0)),
                ),
            ),
        ];

        for (source, expected_function_type) in cases {
            let plan = crate::runtime::plan_src(source);
            let function_id = crate::plan::execution::NilFunctionId(0);
            let function = plan.nil_function(function_id);
            let mut state = crate::runtime::RuntimeState::new();
            let mut frame = Frame::new(function.frame_layout(), &mut state);
            frame.set_tuple(
                crate::plan::execution::TupleLocalId(0),
                vec![EvaluatedValue::Int(1.into())],
            );

            assert_eq!(
                super::super::return_graph::run_nil_loop(&plan, &mut state, function_id, frame,),
                Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: ValueType::Function(Box::new(expected_function_type)),
                        actual: ValueType::Int,
                    }
                )),
            );
        }
    }

    #[test]
    fn diverging_argument_prefix_propagates_generic_function_projection_errors() {
        let cases = [
            (
                r#"
fn fail() -> other { panic }
fn consume(_function: fn(value) -> value, _other: other) { Nil }
fn caller(functions: #(fn(value) -> value)) { consume(functions.0, fail()) }
pub fn main() { caller }
"#,
                FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(0))],
                    ValueType::Parameter(TypeParameterId(0)),
                ),
            ),
            (
                r#"
fn fail() -> other { panic }
fn consume(_function: fn(Int) -> value, _other: other) { Nil }
fn caller(functions: #(fn(Int) -> value)) { consume(functions.0, fail()) }
pub fn main() { caller }
"#,
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Parameter(TypeParameterId(0)),
                ),
            ),
        ];

        for (source, expected_function_type) in cases {
            let plan = crate::runtime::plan_src(source);
            let function_id = crate::plan::execution::NilFunctionId(0);
            let function = plan.nil_function(function_id);
            let mut state = crate::runtime::RuntimeState::new();
            let mut frame = Frame::new(function.frame_layout(), &mut state);
            frame.set_tuple(
                crate::plan::execution::TupleLocalId(0),
                vec![EvaluatedValue::Int(1.into())],
            );

            assert_eq!(
                super::super::return_graph::run_nil_loop(&plan, &mut state, function_id, frame,),
                Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: ValueType::Function(Box::new(expected_function_type)),
                        actual: ValueType::Int,
                    }
                )),
            );
        }
    }

    #[test]
    fn source_generic_and_never_function_arguments_bind_without_evaluation() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
fn identity(value) { value }
fn diverge(_value: Int) -> value { panic }
fn take_generic(_function: fn(value) -> value) { Nil }
fn take_never(_function: fn(Int) -> value) { Nil }

pub fn main() {
  #(take_generic(identity), take_never(diverge))
}
"#,
            ),
            crate::runtime::Value::Tuple(vec![
                crate::runtime::Value::Nil,
                crate::runtime::Value::Nil,
            ]),
        );
    }

    #[test]
    fn unresolved_generic_and_never_function_arguments_preserve_evaluation_order() {
        for (function_type, function, expected) in [
            (
                "fn(value) -> value",
                "fn identity(value) { value }",
                "identity",
            ),
            (
                "fn(Int) -> value",
                "fn diverge(_value: Int) -> value { panic }",
                "diverge",
            ),
        ] {
            let source = format!(
                r#"
{function}
fn fail() -> other {{ panic as "argument failed" }}
fn consume(_function: {function_type}, _other: other) {{ Nil }}
pub fn main() {{ consume({expected}, fail()) }}
"#,
            );
            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: argument failed",
            );
        }
    }

    #[test]
    fn generic_and_never_function_argument_errors_propagate_from_the_value() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution_errors/functions/generic_symbolic_function_argument_failure.gleam"
                ),
                "panic: symbolic function argument failed",
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution_errors/functions/generic_never_function_value_argument_failure.gleam"
                ),
                "panic: never function value argument failed",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src_error(source).to_string(), expected);
        }
    }

    #[test]
    fn source_utf_codepoint_scalar_list_and_function_captures_bind_exact_values() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn identity(value: UtfCodepoint) { value }

pub fn main() {
  let scalar = codepoint()
  let values = [scalar]
  let function = identity
  let closure = fn() {
    let assert [value] = values
    case scalar == value {
      True -> function(value)
      False -> panic
    }
  }
  closure()
}
"#,
            ),
            crate::runtime::Value::UtfCodepoint('A'),
        );
    }

    #[test]
    fn source_custom_scalar_list_and_function_arguments_and_captures_bind_exact_values() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub type Boxed {
  Boxed(Int)
}

fn boxed(value: Int) -> Boxed {
  Boxed(value)
}

fn apply(
  value: Boxed,
  values: List(Boxed),
  function: fn(Int) -> Boxed,
) -> Int {
  case value, values, function(3) {
    Boxed(one), [Boxed(two)], Boxed(three) -> one + two + three
    _, _, _ -> 0
  }
}

pub fn main() {
  let value = Boxed(1)
  let values = [Boxed(2)]
  let function = boxed
  let closure = fn() { apply(value, values, function) }
  closure()
}
"#,
            ),
            crate::runtime::Value::Int(6.into()),
        );
    }

    #[test]
    fn evaluated_custom_function_and_utf_codepoint_list_captures_bind_exact_slots() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }
fn boxed() { Boxed(1) }
fn boxes() { [Boxed(1)] }
fn codepoints() -> List(UtfCodepoint) {
  let assert <<value:utf8_codepoint>> = <<65>>
  [value]
}
pub fn main() {
  let function = boxed
  let values = codepoints()
  #(function, values)
}
"#,
        );
        let function = plan.tuple_function(crate::plan::execution::TupleFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let custom_function_local = function.frame_layout().custom_functions()[0].clone();
        let custom_function = crate::runtime::EvaluatedCustomFunction::reference(
            plan.custom_function_id(0),
            Vec::new(),
            Vec::new(),
            custom_function_local.type_().to_function_type(),
        );
        bind_captures(
            &mut frame,
            &[crate::runtime::EvaluatedCapture::custom_function(
                custom_function_local.clone(),
                custom_function.clone(),
            )],
        );
        assert_eq!(
            frame.get_custom_function(&custom_function_local),
            custom_function,
        );

        let codepoints =
            state.utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['A']);
        bind_list_capture(
            &mut frame,
            &crate::runtime::EvaluatedListCapture::UtfCodepoint {
                local: crate::plan::execution::UtfCodepointListLocalId(0),
                value: codepoints.clone(),
            },
        );
        assert_eq!(
            frame.get_utf_codepoint_list(crate::plan::execution::UtfCodepointListLocalId(0)),
            codepoints,
        );
    }

    #[test]
    fn module_capture_errors_propagate_for_every_value_family() {
        let panic = || {
            PanicExpr::panic_at(
                Some(StringExpr::value("capture".into())),
                PanicSite::unknown(),
            )
        };
        let function_type = |return_: ValueType| FunctionType::new(Vec::new(), return_);
        let int_function_type = function_type(ValueType::Int);
        let list_function_type = function_type(ValueType::List(Box::new(ValueType::Int)));
        let function_function_type =
            FunctionFunctionType::new(Vec::new(), int_function_type.clone());
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type.clone());
        let captures = [
            CaptureArg::int(IntLocalId(0), IntExpr::panic(panic())),
            CaptureArg::string(StringLocalId(0), StringExpr::panic(panic())),
            CaptureArg::bit_array(BitArrayLocalId(0), BitArrayExpr::panic(panic())),
            CaptureArg::utf_codepoint(UtfCodepointLocalId(0), UtfCodepointExpr::panic(panic())),
            CaptureArg::custom(
                CustomLocalId(0),
                CustomExpr::panic(panic(), custom_type.clone()),
            ),
            CaptureArg::float(crate::plan::FloatLocalId(0), FloatExpr::panic(panic())),
            CaptureArg::bool(crate::plan::BoolLocalId(0), BoolExpr::panic(panic())),
            CaptureArg::nil(NilLocalId(0), NilExpr::panic(panic())),
            CaptureArg::tuple(
                TupleLocalId(0),
                TupleExpr::panic(panic(), vec![ValueType::Int]),
            ),
            CaptureArg::list(ListLocalExpr::Int {
                local: IntListLocalId(0),
                value: IntListExpr::from(ListExpr::panic(panic(), ValueType::Int)),
            }),
            CaptureArg::list(ListLocalExpr::String {
                local: StringListLocalId(0),
                value: StringListExpr::from(ListExpr::panic(panic(), ValueType::String)),
            }),
            CaptureArg::list(ListLocalExpr::BitArray {
                local: BitArrayListLocalId(0),
                value: BitArrayListExpr::from(ListExpr::panic(panic(), ValueType::BitArray)),
            }),
            CaptureArg::list(ListLocalExpr::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: UtfCodepointListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::UtfCodepoint,
                )),
            }),
            CaptureArg::list(ListLocalExpr::Custom {
                local: CustomListLocalId(0),
                item_type: custom_type.clone(),
                value: ListExpr::panic(panic(), ValueType::Custom(custom_type.clone()))
                    .into_custom()
                    .expect("custom list panic must preserve its item family"),
            }),
            CaptureArg::list(ListLocalExpr::Float {
                local: FloatListLocalId(0),
                value: FloatListExpr::from(ListExpr::panic(panic(), ValueType::Float)),
            }),
            CaptureArg::list(ListLocalExpr::Bool {
                local: BoolListLocalId(0),
                value: BoolListExpr::from(ListExpr::panic(panic(), ValueType::Bool)),
            }),
            CaptureArg::list(ListLocalExpr::Nil {
                local: NilListLocalId(0),
                value: NilListExpr::from(ListExpr::panic(panic(), ValueType::Nil)),
            }),
            CaptureArg::list(ListLocalExpr::Tuple {
                local: TupleListLocalId(0),
                item_type: vec![ValueType::Int],
                value: TupleListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Tuple(vec![ValueType::Int]),
                )),
            }),
            CaptureArg::list(ListLocalExpr::List {
                local: ListListLocalId(0),
                item_type: Box::new(ValueType::Int),
                value: ListListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::List(Box::new(ValueType::Int)),
                )),
            }),
            CaptureArg::list(ListLocalExpr::Function {
                local: FunctionListLocalId(0),
                item_type: int_function_type.clone(),
                value: crate::plan::FunctionListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(int_function_type.clone())),
                )),
            }),
            CaptureArg::list(ListLocalExpr::Generic {
                local: GenericListLocalId(0),
                parameter: TypeParameterId(0),
                value: ListExpr::panic(panic(), ValueType::Parameter(TypeParameterId(0)))
                    .into_generic()
                    .expect("generic list panic should retain its item parameter"),
            }),
            CaptureArg::list(ListLocalExpr::ParameterList {
                local: ListListLocalId(1),
                parameter: TypeParameterId(0),
                value: ListExpr::panic(
                    panic(),
                    ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
                )
                .into_parameter_list()
                .expect("nested generic list panic should retain its item parameter"),
            }),
            CaptureArg::int_function(
                IntFunctionLocalId(0),
                IntFunctionExpr::panic(panic(), int_function_type.clone()),
            ),
            CaptureArg::string_function(
                StringFunctionLocalId(0),
                StringFunctionExpr::panic(panic(), function_type(ValueType::String)),
            ),
            CaptureArg::bit_array_function(
                BitArrayFunctionLocalId(0),
                BitArrayFunctionExpr::panic(panic(), function_type(ValueType::BitArray)),
            ),
            CaptureArg::utf_codepoint_function(
                UtfCodepointFunctionLocalId(0),
                UtfCodepointFunctionExpr::panic(panic(), function_type(ValueType::UtfCodepoint)),
            ),
            CaptureArg::custom_function(
                CustomFunctionLocalId(0),
                CustomFunctionExpr::panic(panic(), custom_function_type),
            ),
            CaptureArg::float_function(
                FloatFunctionLocalId(0),
                FloatFunctionExpr::panic(panic(), function_type(ValueType::Float)),
            ),
            CaptureArg::bool_function(
                BoolFunctionLocalId(0),
                BoolFunctionExpr::panic(panic(), function_type(ValueType::Bool)),
            ),
            CaptureArg::nil_function(
                NilFunctionLocalId(0),
                NilFunctionExpr::panic(panic(), function_type(ValueType::Nil)),
            ),
            CaptureArg::tuple_function(
                TupleFunctionLocalId(0),
                TupleFunctionExpr::panic(
                    panic(),
                    function_type(ValueType::Tuple(vec![ValueType::Int])),
                ),
            ),
            CaptureArg::list_function(
                ListFunctionLocal::from_item_type(0, list_function_type.clone(), ValueType::Int),
                ListFunctionExpr::panic(panic(), list_function_type, ValueType::Int),
            ),
            CaptureArg::function_function(
                FunctionFunctionLocalId(0),
                FunctionFunctionExpr::panic(panic(), function_function_type),
            ),
            CaptureArg::generic_function_expr(
                GenericFunctionLocal::new(
                    GenericFunctionLocalId(0),
                    GenericFunctionType::new(
                        vec![ValueShape::Parameter(TypeParameterId(0))],
                        TypeParameterId(0),
                    ),
                ),
                TypedFunctionExpr::new(
                    crate::plan::FunctionShape::new(
                        vec![ValueShape::Parameter(TypeParameterId(0))],
                        ValueShape::Parameter(TypeParameterId(0)),
                    ),
                    GenericFunctionExpr::panic(
                        panic(),
                        GenericFunctionType::new(
                            vec![ValueShape::Parameter(TypeParameterId(0))],
                            TypeParameterId(0),
                        ),
                    ),
                ),
            ),
            CaptureArg::generic_function_expr(
                GenericFunctionLocal::new(
                    GenericFunctionLocalId(1),
                    GenericFunctionType::new(vec![ValueShape::Int], TypeParameterId(0)),
                ),
                TypedFunctionExpr::new(
                    crate::plan::FunctionShape::new(
                        vec![ValueShape::Int],
                        ValueShape::Parameter(TypeParameterId(0)),
                    ),
                    GenericFunctionExpr::panic(
                        panic(),
                        GenericFunctionType::new(vec![ValueShape::Int], TypeParameterId(0)),
                    ),
                ),
            ),
        ];

        for capture in captures {
            assert_eq!(run_module_capture(capture).to_string(), "panic: capture",);
        }
    }

    #[test]
    fn custom_constructor_argument_errors_propagate_for_every_value_family() {
        for field_type in [
            "Int",
            "Float",
            "String",
            "BitArray",
            "UtfCodepoint",
            "Nested",
            "Bool",
            "Nil",
            "#(Int)",
            "List(Int)",
            "fn() -> Int",
            "fn() -> Float",
            "fn() -> String",
            "fn() -> BitArray",
            "fn() -> UtfCodepoint",
            "fn() -> Nested",
            "fn() -> Bool",
            "fn() -> Nil",
            "fn() -> #(Int)",
            "fn() -> List(Int)",
            "fn() -> fn() -> Int",
        ] {
            let source = format!(
                "pub type Nested {{ Nested }} pub type Boxed {{ Boxed({field_type}) }} pub fn main() {{ let constructor = Boxed constructor(panic as \"argument\") }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: argument",
            );
        }
    }

    fn run_module_capture(capture: CaptureArg) -> ExecutionError {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let list_function_type =
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type.clone());
        let function_function_type = FunctionFunctionType::new(Vec::new(), function_type.clone());
        let generic_function_type = GenericFunctionType::new(
            vec![ValueShape::Parameter(TypeParameterId(0))],
            TypeParameterId(0),
        );
        let never_function_type =
            GenericFunctionType::new(vec![ValueShape::Int], TypeParameterId(0));
        let target_values = vec![
            Expr::int(IntExpr::local_get(IntLocalId(0), "capture".into())),
            Expr::string(StringExpr::local_get(StringLocalId(0), "capture".into())),
            Expr::bit_array(BitArrayExpr::local_get(
                BitArrayLocalId(0),
                "capture".into(),
            )),
            Expr::utf_codepoint(UtfCodepointExpr::local_get(
                UtfCodepointLocalId(0),
                "capture".into(),
            )),
            Expr::custom(CustomExpr::local_get(
                CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                "capture".into(),
            )),
            Expr::float(FloatExpr::local_get(
                crate::plan::FloatLocalId(0),
                "capture".into(),
            )),
            Expr::bool(BoolExpr::local_get(
                crate::plan::BoolLocalId(0),
                "capture".into(),
            )),
            Expr::nil(NilExpr::local_get(NilLocalId(0), "capture".into())),
            Expr::tuple(TupleExpr::local_get(
                TupleLocalId(0),
                "capture".into(),
                vec![ValueType::Int],
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::int(IntListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::string(StringListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::bit_array(BitArrayListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::utf_codepoint(UtfCodepointListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::custom(CustomListLocalId(0), custom_type.clone()),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::float(FloatListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::bool(BoolListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::nil(NilListLocalId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::tuple(TupleListLocalId(0), vec![ValueType::Int]),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::list(ListListLocalId(0), ValueType::Int),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::function(FunctionListLocalId(0), function_type.clone()),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::generic(GenericListLocalId(0), TypeParameterId(0)),
                "capture".into(),
            )),
            Expr::list(ListExpr::local_get(
                ListLocal::list(ListListLocalId(1), ValueType::Parameter(TypeParameterId(0))),
                "capture".into(),
            )),
            Expr::function(FunctionExpr::int(IntFunctionExpr::local_get(
                IntFunctionLocalId(0),
                "capture".into(),
                function_type.clone(),
            ))),
            Expr::function(FunctionExpr::string(StringFunctionExpr::local_get(
                StringFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::String),
            ))),
            Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::BitArray),
            ))),
            Expr::function(FunctionExpr::utf_codepoint(
                UtfCodepointFunctionExpr::local_get(
                    UtfCodepointFunctionLocalId(0),
                    "capture".into(),
                    FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                ),
            )),
            Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
                CustomFunctionLocal::new(CustomFunctionLocalId(0), custom_function_type.clone()),
                "capture".into(),
            ))),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::local_get(
                FloatFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::Float),
            ))),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::local_get(
                BoolFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ))),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::local_get(
                NilFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::Nil),
            ))),
            Expr::function(FunctionExpr::tuple(TupleFunctionExpr::local_get(
                TupleFunctionLocalId(0),
                "capture".into(),
                FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
            ))),
            Expr::function(FunctionExpr::list(ListFunctionExpr::local_get(
                ListFunctionLocal::from_item_type(0, list_function_type.clone(), ValueType::Int),
                "capture".into(),
            ))),
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                FunctionFunctionLocal::new(FunctionFunctionLocalId(0), function_function_type),
                "capture".into(),
            ))),
            Expr::function(FunctionExpr::generic(GenericFunctionExpr::local_get(
                GenericFunctionLocal::new(GenericFunctionLocalId(0), generic_function_type),
                "capture".into(),
            ))),
            Expr::function(FunctionExpr::generic(GenericFunctionExpr::local_get(
                GenericFunctionLocal::new(GenericFunctionLocalId(1), never_function_type),
                "capture".into(),
            ))),
        ];
        let target_types = target_values.iter().map(Expr::value_type).collect();
        let target = FunctionTemplate::new(
            FunctionTemplateId::new(1),
            "target".into(),
            Vec::new(),
            vec![Step::evaluate(Expr::tuple(TupleExpr::value(
                target_values,
                target_types,
            )))],
            ReturnExpr::int(
                IntFunctionId(0),
                IntExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ),
        );
        let expression = IntFunctionExpr::closure(
            crate::plan::monomorphic_function_instantiation(
                1,
                crate::plan::FunctionShape::from_function_type(function_type.clone()),
            ),
            Vec::new(),
            vec![capture],
            function_type,
        );
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int_function(IntFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, vec![target]).with_custom_types(vec![
            CustomTypeDefinition::new(
                CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                CustomTypePublicity::Public,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Boxed".into(),
                    0,
                    vec![CustomFieldDefinition::new(None, CustomTypeTemplate::Int)],
                )],
            ),
        ]);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("capture expression should fail at runtime")
    }
}
