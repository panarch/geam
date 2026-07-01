use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, CallArgKind, CaptureArg, CaptureArgKind,
    CaptureValue, CaptureValueKind, ExecutionPlan, FrameLayout, FunctionFunctionFunctionId,
    FunctionFunctionValue, FunctionReturnFamily, FunctionValue, IntFunctionFunctionId,
    IntFunctionId, NilFunctionFunctionId, NilFunctionId, ReturnBody, ReturnBodyKind,
    RuntimeFunctionId, StepKind, StringFunctionFunctionId, StringFunctionId, Value,
};
use crate::runtime::ExecutionError;
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_expr, eval_function_function_expr, eval_int_expr,
    eval_int_function_expr, eval_nil_expr, eval_nil_function_expr, eval_string_expr,
    eval_string_function_expr,
};
use crate::runtime::frame::Frame;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn run_main(plan: &ExecutionPlan) -> ExecutionResult<Value> {
    let mut caller_frame = Frame::default();
    match plan.main_runtime() {
        RuntimeFunctionId::Int(function) => {
            run_int_call(plan, function, &[], &mut caller_frame).map(Value::Int)
        }
        RuntimeFunctionId::String(function) => {
            run_string_call(plan, function, &[], &mut caller_frame).map(Value::String)
        }
        RuntimeFunctionId::Bool(function) => {
            run_bool_call(plan, function, &[], &mut caller_frame).map(Value::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            run_nil_call(plan, function, &[], &mut caller_frame).map(|_| Value::Nil)
        }
        RuntimeFunctionId::Function { id, .. } => {
            run_function_returning_function_call(plan, id, &[], &mut caller_frame)
                .map(Value::Function)
        }
    }
}

pub(super) fn run_int_call(
    plan: &ExecutionPlan,
    function: IntFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.int_function(function).frame_layout(),
    )?;
    run_int_loop(plan, function, frame)
}

pub(super) fn run_string_call(
    plan: &ExecutionPlan,
    function: StringFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.string_function(function).frame_layout(),
    )?;
    run_string_loop(plan, function, frame)
}

pub(super) fn run_bool_call(
    plan: &ExecutionPlan,
    function: BoolFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_function(function).frame_layout(),
    )?;
    run_bool_loop(plan, function, frame)
}

pub(super) fn run_nil_call(
    plan: &ExecutionPlan,
    function: NilFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_function(function).frame_layout(),
    )?;
    run_nil_loop(plan, function, frame)
}

enum ReturnOutcome<Value, Function> {
    Value(Value),
    TailCall {
        function: Function,
        args: Vec<CallArg>,
    },
}

fn eval_return_body<Expression, Function, Value>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    body: &ReturnBody<Expression, Function>,
    eval_expression: fn(&ExecutionPlan, &mut Frame, &Expression) -> ExecutionResult<Value>,
) -> ExecutionResult<ReturnOutcome<Value, Function>>
where
    Function: Copy,
{
    match body.kind() {
        ReturnBodyKind::Expr(expression) => {
            eval_expression(plan, frame, expression).map(ReturnOutcome::Value)
        }
        ReturnBodyKind::TailCall { function, args } => Ok(ReturnOutcome::TailCall {
            function: *function,
            args: args.clone(),
        }),
        ReturnBodyKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_return_body(plan, frame, true_, eval_expression)
            } else {
                eval_return_body(plan, frame, false_, eval_expression)
            }
        }
        ReturnBodyKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_return_body(plan, frame, branch, eval_expression);
                }
            }
            eval_return_body(plan, frame, fallback, eval_expression)
        }
        ReturnBodyKind::Block { steps, return_ } => {
            execute_steps(plan, steps, frame)?;
            eval_return_body(plan, frame, return_, eval_expression)
        }
    }
}

fn run_int_loop(
    plan: &ExecutionPlan,
    mut function: IntFunctionId,
    mut frame: Frame,
) -> ExecutionResult<BigInt> {
    loop {
        let runtime_function = plan.int_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_int_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.int_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_string_loop(
    plan: &ExecutionPlan,
    mut function: StringFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EcoString> {
    loop {
        let runtime_function = plan.string_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_string_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.string_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_bool_loop(
    plan: &ExecutionPlan,
    mut function: BoolFunctionId,
    mut frame: Frame,
) -> ExecutionResult<bool> {
    loop {
        let runtime_function = plan.bool_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_bool_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bool_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_nil_loop(
    plan: &ExecutionPlan,
    mut function: NilFunctionId,
    mut frame: Frame,
) -> ExecutionResult<()> {
    loop {
        let runtime_function = plan.nil_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_nil_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(()) => return Ok(()),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.nil_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_int_function_loop(
    plan: &ExecutionPlan,
    mut function: IntFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::IntFunctionValue> {
    loop {
        let runtime_function = plan.int_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_int_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.int_function_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_string_function_loop(
    plan: &ExecutionPlan,
    mut function: StringFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::StringFunctionValue> {
    loop {
        let runtime_function = plan.string_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_string_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.string_function_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_bool_function_loop(
    plan: &ExecutionPlan,
    mut function: BoolFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::BoolFunctionValue> {
    loop {
        let runtime_function = plan.bool_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_bool_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bool_function_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_nil_function_loop(
    plan: &ExecutionPlan,
    mut function: NilFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::NilFunctionValue> {
    loop {
        let runtime_function = plan.nil_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_nil_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.nil_function_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn run_function_function_loop(
    plan: &ExecutionPlan,
    mut function: FunctionFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<FunctionFunctionValue> {
    loop {
        let runtime_function = plan.function_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_function_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.function_function_function(next).frame_layout();
                frame = bind_arguments(plan, &args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(in crate::runtime) fn execute_steps(
    plan: &ExecutionPlan,
    steps: &[crate::plan::Step],
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for step in steps {
        match step.kind() {
            StepKind::LetInt { local, value, .. } => {
                let value = eval_int_expr(plan, frame, value)?;
                frame.set_int(*local, value);
            }
            StepKind::LetString { local, value, .. } => {
                let value = eval_string_expr(plan, frame, value)?;
                frame.set_string(*local, value);
            }
            StepKind::LetBool { local, value, .. } => {
                let value = eval_bool_expr(plan, frame, value)?;
                frame.set_bool(*local, value);
            }
            StepKind::LetNil { local, value, .. } => {
                eval_nil_expr(plan, frame, value)?;
                frame.set_nil(*local);
            }
            StepKind::LetIntFunction { local, value, .. } => {
                let value = eval_int_function_expr(plan, frame, value)?;
                frame.set_int_function(*local, value);
            }
            StepKind::LetStringFunction { local, value, .. } => {
                let value = eval_string_function_expr(plan, frame, value)?;
                frame.set_string_function(*local, value);
            }
            StepKind::LetBoolFunction { local, value, .. } => {
                let value = eval_bool_function_expr(plan, frame, value)?;
                frame.set_bool_function(*local, value);
            }
            StepKind::LetNilFunction { local, value, .. } => {
                let value = eval_nil_function_expr(plan, frame, value)?;
                frame.set_nil_function(*local, value);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                let value = eval_function_function_expr(plan, frame, value)?;
                frame.set_function_function(*local, value);
            }
            StepKind::Evaluate(expression) => {
                let _ = eval_expr(plan, frame, expression)?;
            }
        }
    }

    Ok(())
}

fn bind_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: FrameLayout,
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout);
    bind_arguments_into(plan, args, caller_frame, &mut frame)?;
    Ok(frame)
}

fn bind_function_value_arguments(
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
            CaptureValueKind::Bool { local, value } => frame.set_bool(*local, *value),
            CaptureValueKind::Nil { local } => frame.set_nil(*local),
            CaptureValueKind::IntFunction { local, value } => {
                frame.set_int_function(*local, value.clone());
            }
            CaptureValueKind::StringFunction { local, value } => {
                frame.set_string_function(*local, value.clone());
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

pub(in crate::runtime) fn run_int_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::IntFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<BigInt> {
    let function = eval_int_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.int_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_int_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_string_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::StringFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<EcoString> {
    let function = eval_string_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.string_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_string_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_bool_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::BoolFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<bool> {
    let function = eval_bool_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.bool_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_bool_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_nil_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::NilFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<()> {
    let function = eval_nil_function_expr(plan, caller_frame, function)?;
    let runtime_function = plan.nil_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_nil_loop(plan, function.runtime_id(), frame)
}

pub(in crate::runtime) fn run_int_function_returning_function_call(
    plan: &ExecutionPlan,
    function: IntFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::IntFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.int_function_function(function).frame_layout(),
    )?;
    run_int_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_string_function_returning_function_call(
    plan: &ExecutionPlan,
    function: StringFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::StringFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.string_function_function(function).frame_layout(),
    )?;
    run_string_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_bool_function_returning_function_call(
    plan: &ExecutionPlan,
    function: BoolFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::BoolFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.bool_function_function(function).frame_layout(),
    )?;
    run_bool_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_nil_function_returning_function_call(
    plan: &ExecutionPlan,
    function: NilFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::NilFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.nil_function_function(function).frame_layout(),
    )?;
    run_nil_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_function_function_returning_function_call(
    plan: &ExecutionPlan,
    function: FunctionFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionFunctionValue> {
    let frame = bind_arguments(
        plan,
        args,
        caller_frame,
        plan.function_function_function(function).frame_layout(),
    )?;
    run_function_function_loop(plan, function, frame)
}

pub(in crate::runtime) fn run_int_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::IntFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .int()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Int,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.int_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_int_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_string_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::StringFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .string()
            .ok_or(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::String,
                runtime_id.family(),
            ))?;
    let runtime_function = plan.string_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_string_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_bool_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::BoolFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .bool()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Bool,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.bool_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_bool_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_nil_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<crate::plan::NilFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id = runtime_id
        .nil()
        .ok_or(ExecutionError::function_return_family_mismatch(
            FunctionReturnFamily::Nil,
            runtime_id.family(),
        ))?;
    let runtime_function = plan.nil_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_nil_function_loop(plan, function_id, frame)
}

pub(in crate::runtime) fn run_function_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionFunctionValue> {
    let function = eval_function_function_expr(plan, caller_frame, function)?;
    let runtime_id = function.runtime_id();
    let function_id =
        runtime_id
            .function()
            .ok_or(ExecutionError::function_return_family_mismatch(
                FunctionReturnFamily::Function,
                runtime_id.family(),
            ))?;
    let runtime_function = plan.function_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let frame =
        bind_function_value_arguments(plan, args, caller_frame, frame_layout, function.captures())?;
    run_function_function_loop(plan, function_id, frame)
}
fn run_function_returning_function_call(
    plan: &ExecutionPlan,
    function: crate::plan::FunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> ExecutionResult<FunctionValue> {
    match function {
        crate::plan::FunctionFunctionId::Int(function) => {
            run_int_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::String(function) => {
            run_string_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Bool(function) => {
            run_bool_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Nil(function) => {
            run_nil_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
        crate::plan::FunctionFunctionId::Function(function) => {
            run_function_function_returning_function_call(plan, function, args, caller_frame)
                .map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        execute_steps, run_bool_call, run_bool_function_call, run_bool_function_function_call,
        run_bool_function_returning_function_call, run_function_function_function_call,
        run_function_function_returning_function_call, run_int_call, run_int_function_call,
        run_int_function_function_call, run_int_function_returning_function_call, run_main,
        run_nil_call, run_nil_function_call, run_nil_function_function_call,
        run_nil_function_returning_function_call, run_string_call, run_string_function_call,
        run_string_function_function_call, run_string_function_returning_function_call,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
        BoolFunctionValue, BoolLocalId, CallArg, ExecutionPlan, Expr, FunctionExpr,
        FunctionExprKind, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionFunctionValue, FunctionId, FunctionPlan,
        FunctionReturnFamily, FunctionType, FunctionValue, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId,
        NilExpr, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
        NilFunctionValue, NilLocalId, ParamLocal, ReturnExpr, RuntimeFunctionId, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
        StringFunctionValue, StringLocalId, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, Value, run_src};

    #[test]
    fn function_function_call_returns_execution_error_on_return_family_mismatch() {
        let plan = plan();

        assert_function_return_family_mismatch(
            run_int_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Int,
            FunctionReturnFamily::String,
        );
        assert_function_return_family_mismatch(
            run_string_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::String,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_bool_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Bool,
            FunctionReturnFamily::Int,
        );
        assert_function_return_family_mismatch(
            run_nil_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            ),
            FunctionReturnFamily::Nil,
            FunctionReturnFamily::Int,
        );
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_function_call_returns_function_values() {
        assert_eq!(
            run_src(
                r#"
fn int_identity(value: Int) {
  value
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

fn get_int() {
  int_identity
}

fn get_string() {
  string_identity
}

fn get_bool() {
  bool_identity
}

fn get_nil() {
  nil_identity
}

fn get_get_int() {
  get_int
}

fn get_get_get_int() {
  get_get_int
}

pub fn main() {
  let int_ok = get_int()(1) + get_get_get_int()()()(2) == 3
  let bool_ok = get_bool()(True)

  get_nil()(Nil)

  case int_ok && bool_ok {
    True -> get_string()("ge") <> get_string()("am")
    False -> "bad"
  }
}
"#,
            ),
            Value::String("geam".into()),
        );
    }

    #[test]
    fn run_src_returns_function_value_shapes() {
        assert_eq!(
            run_src(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                vec![ParamLocal::int(IntLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::String(StringFunctionId(0)),
                vec![ParamLocal::string(StringLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                vec![ParamLocal::bool(BoolLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                vec![ParamLocal::nil(NilLocalId(0))],
            )),
        );
        assert_eq!(
            run_src(
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
            ),
            Value::Function(FunctionValue::new(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Vec::new(),
            )),
        );
    }

    #[test]
    fn function_function_projection_returns_typed_function_values() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_eq!(
            run_int_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Int),
        );
        assert_eq!(
            run_string_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::String),
        );
        assert_eq!(
            run_bool_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Bool(BoolFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Bool),
        );
        assert_eq!(
            run_nil_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Nil(NilFunctionFunctionId(0))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Nil),
        );
        assert_eq!(
            run_function_function_function_call(
                &plan,
                &function_function_expr(FunctionFunctionId::Function(FunctionFunctionFunctionId(
                    0
                ))),
                &[],
                &mut Frame::default(),
            )
            .map(|value| value.type_().return_().clone()),
            Ok(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))),
        );
    }

    #[test]
    fn function_function_call_propagates_callee_evaluation_error() {
        let plan = plan();
        let expression = failing_function_function_expr();

        assert_expected_function_got_int(run_int_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &expression,
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_function_call_propagates_argument_binding_error() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_expected_function_got_int(run_int_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Bool(BoolFunctionFunctionId(0))),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Nil(NilFunctionFunctionId(0))),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Function(FunctionFunctionFunctionId(0))),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_function_call_propagates_step_error() {
        let plan = plan_with_function_function_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::String(StringFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Bool(BoolFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Nil(NilFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_function_call(
            &plan,
            &function_function_expr(FunctionFunctionId::Function(FunctionFunctionFunctionId(0))),
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn run_main_propagates_function_body_error_by_return_family() {
        let steps = vec![failing_step()];

        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::string(StringFunctionId(0), StringExpr::value("geam".into())),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::bool(BoolFunctionId(0), BoolExpr::value(true)),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps.clone(),
            ReturnExpr::nil(NilFunctionId(0), NilExpr::value()),
        )));
        assert_expected_function_got_int(run_main(&plan_with_main(
            steps,
            function_return_expr(function_function_expr_value()),
        )));
    }

    #[test]
    fn primitive_function_call_propagates_argument_binding_error() {
        let plan = primitive_function_plan();

        assert_expected_function_got_int(run_int_call(
            &plan,
            IntFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_call(
            &plan,
            StringFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_call(
            &plan,
            BoolFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_call(
            &plan,
            NilFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn primitive_function_call_propagates_typed_argument_evaluation_errors() {
        let plan = primitive_function_plan();

        let cases = [
            CallArg::int(IntLocalId(0), failing_int_expr()),
            CallArg::string(StringLocalId(0), failing_string_expr()),
            CallArg::bool(BoolLocalId(0), failing_bool_expr()),
            CallArg::nil(NilLocalId(0), failing_nil_expr()),
            CallArg::int_function(IntFunctionLocalId(0), failing_int_function_expr()),
            CallArg::string_function(StringFunctionLocalId(0), failing_string_function_expr()),
            CallArg::bool_function(BoolFunctionLocalId(0), failing_bool_function_expr()),
            CallArg::nil_function(NilFunctionLocalId(0), failing_nil_function_expr()),
        ];

        for arg in cases {
            assert_expected_function_got_int(run_int_call(
                &plan,
                IntFunctionId(0),
                &[arg],
                &mut Frame::default(),
            ));
        }
    }

    #[test]
    fn execute_steps_propagates_let_value_evaluation_errors() {
        let plan = plan();

        let steps = [
            Step::let_int(IntLocalId(0), "x".into(), failing_int_expr()),
            Step::let_string(StringLocalId(0), "x".into(), failing_string_expr()),
            Step::let_bool(BoolLocalId(0), "x".into(), failing_bool_expr()),
            Step::let_nil(NilLocalId(0), "x".into(), failing_nil_expr()),
            Step::let_int_function(
                IntFunctionLocalId(0),
                "x".into(),
                failing_int_function_expr(),
            ),
            Step::let_string_function(
                StringFunctionLocalId(0),
                "x".into(),
                failing_string_function_expr(),
            ),
            Step::let_bool_function(
                BoolFunctionLocalId(0),
                "x".into(),
                failing_bool_function_expr(),
            ),
            Step::let_nil_function(
                NilFunctionLocalId(0),
                "x".into(),
                failing_nil_function_expr(),
            ),
            Step::let_function_function(
                FunctionFunctionLocalId(0),
                "x".into(),
                failing_function_function_expr(),
            ),
        ];

        for step in steps {
            assert_expected_function_got_int(execute_steps(&plan, &[step], &mut Frame::default()));
        }
    }

    #[test]
    fn primitive_function_value_call_propagates_callee_evaluation_error() {
        let plan = primitive_function_plan();

        assert_expected_function_got_int(run_int_function_call(
            &plan,
            &IntFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_call(
            &plan,
            &StringFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_call(
            &plan,
            &BoolFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_call(
            &plan,
            &NilFunctionExpr::function_call(
                failing_function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Nil),
            ),
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn primitive_function_value_call_propagates_argument_binding_error() {
        let plan = primitive_function_plan();

        assert_expected_function_got_int(run_int_function_call(
            &plan,
            &IntFunctionExpr::value(IntFunctionValue::new(IntFunctionId(0), Vec::new())),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_call(
            &plan,
            &StringFunctionExpr::value(StringFunctionValue::new(StringFunctionId(0), Vec::new())),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_call(
            &plan,
            &BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new())),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_call(
            &plan,
            &NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new())),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn primitive_function_value_call_propagates_step_error() {
        let plan = primitive_function_plan_with_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_function_call(
            &plan,
            &IntFunctionExpr::value(IntFunctionValue::new(IntFunctionId(0), Vec::new())),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_call(
            &plan,
            &StringFunctionExpr::value(StringFunctionValue::new(StringFunctionId(0), Vec::new())),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_call(
            &plan,
            &BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new())),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_call(
            &plan,
            &NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new())),
            &[],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_returning_function_call_propagates_argument_binding_error() {
        let plan = plan_with_function_function_steps(Vec::new());

        assert_expected_function_got_int(run_int_function_returning_function_call(
            &plan,
            IntFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_returning_function_call(
            &plan,
            StringFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_returning_function_call(
            &plan,
            BoolFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_returning_function_call(
            &plan,
            NilFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_returning_function_call(
            &plan,
            FunctionFunctionFunctionId(0),
            &[failing_function_function_arg()],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn function_returning_function_call_propagates_step_error() {
        let plan = plan_with_function_function_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_function_returning_function_call(
            &plan,
            IntFunctionFunctionId(0),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_returning_function_call(
            &plan,
            StringFunctionFunctionId(0),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_returning_function_call(
            &plan,
            BoolFunctionFunctionId(0),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_returning_function_call(
            &plan,
            NilFunctionFunctionId(0),
            &[],
            &mut Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_returning_function_call(
            &plan,
            FunctionFunctionFunctionId(0),
            &[],
            &mut Frame::default(),
        ));
    }

    fn assert_expected_function_got_int<T>(actual: Result<T, ExecutionError>) {
        assert_function_return_family_mismatch(
            actual,
            FunctionReturnFamily::Function,
            FunctionReturnFamily::Int,
        );
    }

    fn assert_function_return_family_mismatch<T>(
        actual: Result<T, ExecutionError>,
        expected: FunctionReturnFamily,
        actual_family: FunctionReturnFamily,
    ) {
        let error = actual.err().expect("call should fail");

        assert_eq!(
            error,
            ExecutionError::function_return_family_mismatch(expected, actual_family),
        );
    }

    fn function_function_expr(runtime_id: FunctionFunctionId) -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            runtime_id,
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ))
    }

    fn failing_function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::function_call(
            function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            Vec::new(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        )
    }

    fn failing_function_function_arg() -> CallArg {
        CallArg::function_function(FunctionFunctionLocalId(0), failing_function_function_expr())
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

    fn failing_step() -> Step {
        Step::evaluate(Expr::function(FunctionExpr::function(
            failing_function_function_expr(),
        )))
    }

    fn plan_with_function_function_steps(steps: Vec<Step>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            vec![
                function_plan(1, "int_function", steps.clone(), int_function_expr()),
                function_plan(2, "string_function", steps.clone(), string_function_expr()),
                function_plan(3, "bool_function", steps.clone(), bool_function_expr()),
                function_plan(4, "nil_function", steps.clone(), nil_function_expr()),
                function_plan(
                    5,
                    "function_function",
                    steps,
                    function_function_expr_value(),
                ),
            ],
        )
    }

    fn plan_with_main(steps: Vec<Step>, return_: ReturnExpr) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                steps,
                return_,
            ),
            vec![function_plan(
                1,
                "int_function",
                Vec::new(),
                int_function_expr(),
            )],
        )
    }

    fn primitive_function_plan() -> ExecutionPlan {
        primitive_function_plan_with_steps(Vec::new())
    }

    fn primitive_function_plan_with_steps(steps: Vec<Step>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                steps.clone(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
            ),
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "string".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::string(StringFunctionId(0), StringExpr::value("geam".into())),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "bool".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::bool(BoolFunctionId(0), BoolExpr::value(true)),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "nil".into(),
                    Vec::new(),
                    steps,
                    ReturnExpr::nil(NilFunctionId(0), NilExpr::value()),
                ),
            ],
        )
    }

    fn function_plan(
        id: usize,
        name: &str,
        steps: Vec<Step>,
        return_: FunctionExpr,
    ) -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(id),
            name.into(),
            Vec::new(),
            steps,
            function_return_expr(return_),
        )
    }

    fn function_return_expr(return_: FunctionExpr) -> ReturnExpr {
        match return_.into_kind() {
            FunctionExprKind::Int(return_) => {
                ReturnExpr::int_function(IntFunctionFunctionId(0), return_)
            }
            FunctionExprKind::String(return_) => {
                ReturnExpr::string_function(StringFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Bool(return_) => {
                ReturnExpr::bool_function(BoolFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Nil(return_) => {
                ReturnExpr::nil_function(NilFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Function(return_) => {
                ReturnExpr::function_function(FunctionFunctionFunctionId(0), return_)
            }
        }
    }

    fn int_function_expr() -> FunctionExpr {
        FunctionExpr::int(IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            Vec::new(),
        )))
    }

    fn string_function_expr() -> FunctionExpr {
        FunctionExpr::string(StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            Vec::new(),
        )))
    }

    fn bool_function_expr() -> FunctionExpr {
        FunctionExpr::bool(BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            Vec::new(),
        )))
    }

    fn nil_function_expr() -> FunctionExpr {
        FunctionExpr::nil(NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            Vec::new(),
        )))
    }

    fn function_function_expr_value() -> FunctionExpr {
        FunctionExpr::function(function_function_expr(FunctionFunctionId::Int(
            IntFunctionFunctionId(0),
        )))
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
            Vec::new(),
        )
    }
}
