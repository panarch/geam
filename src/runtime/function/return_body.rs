use super::bind::bind_arguments;
use super::steps::execute_steps;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CallArg, FloatFunctionFunctionId, FloatFunctionId,
    FloatListFunctionId, FunctionFunctionFunctionId, FunctionListFunctionId, IntFunctionFunctionId,
    IntFunctionId, IntListFunctionId, ListFunctionFunctionId, ListFunctionId, ListListFunctionId,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionId, ReturnBody, ReturnBodyKind,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionId, TupleFunctionFunctionId,
    TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bit_array_expr, eval_bit_array_function_expr, eval_bit_array_list_expr, eval_bool_expr,
    eval_bool_function_expr, eval_bool_list_expr, eval_float_expr, eval_float_function_expr,
    eval_float_list_expr, eval_function_function_expr, eval_function_list_expr, eval_int_expr,
    eval_int_function_expr, eval_int_list_expr, eval_list_function_expr, eval_list_list_expr,
    eval_nil_expr, eval_nil_function_expr, eval_nil_list_expr, eval_string_expr,
    eval_string_function_expr, eval_string_list_expr, eval_tuple_expr, eval_tuple_function_expr,
    eval_tuple_list_expr, eval_utf_codepoint_expr, eval_utf_codepoint_function_expr,
    eval_utf_codepoint_list_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, FloatListValueId, FunctionListValueId, IntListValueId,
    ListListValueId, ListValueId, NilListValueId, RuntimeState, StringListValueId,
    TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use ecow::EcoString;
use num_bigint::BigInt;

enum ReturnOutcome<'a, Value, Function> {
    Value(Value),
    TailCall {
        function: Function,
        args: &'a [CallArg],
    },
}

fn finish_return<Value>(
    state: &mut RuntimeState,
    frame: Frame,
    value: Value,
) -> ExecutionResult<Value> {
    drop(frame);
    state.drain_releases();
    Ok(value)
}

fn bind_tail_arguments(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    args: &[CallArg],
    mut frame: Frame,
    frame_layout: &crate::plan::execution::FrameLayout,
) -> ExecutionResult<Frame> {
    let next = bind_arguments(plan, state, args, &mut frame, frame_layout)?;
    drop(frame);
    state.drain_releases();
    Ok(next)
}

fn eval_return_body<'a, Expression, Function, Value>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    body: &'a ReturnBody<Expression, Function>,
    eval_expression: fn(
        &ExecutionPlan,
        &mut RuntimeState,
        &mut Frame,
        &Expression,
    ) -> ExecutionResult<Value>,
) -> ExecutionResult<ReturnOutcome<'a, Value, Function>>
where
    Function: Clone,
{
    match body.kind() {
        ReturnBodyKind::Expr(expression) => {
            eval_expression(plan, state, frame, expression).map(ReturnOutcome::Value)
        }
        ReturnBodyKind::TailCall { function, args } => Ok(ReturnOutcome::TailCall {
            function: function.clone(),
            args,
        }),
        ReturnBodyKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_return_body(plan, state, frame, true_, eval_expression)
            } else {
                eval_return_body(plan, state, frame, false_, eval_expression)
            }
        }
        ReturnBodyKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_return_body(plan, state, frame, branch, eval_expression);
                }
            }
            eval_return_body(plan, state, frame, fallback, eval_expression)
        }
        ReturnBodyKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_return_body(plan, state, frame, branch, eval_expression);
                }
            }
            eval_return_body(plan, state, frame, fallback, eval_expression)
        }
        ReturnBodyKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_return_body(plan, state, frame, branch, eval_expression);
                }
            }
            eval_return_body(plan, state, frame, fallback, eval_expression)
        }
        ReturnBodyKind::Block { steps, return_ } => {
            execute_steps(plan, state, steps, frame)?;
            eval_return_body(plan, state, frame, return_, eval_expression)
        }
    }
}

pub(super) fn run_int_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: IntFunctionId,
    mut frame: Frame,
) -> ExecutionResult<BigInt> {
    loop {
        let runtime_function = plan.int_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_int_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.int_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_float_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: FloatFunctionId,
    mut frame: Frame,
) -> ExecutionResult<f64> {
    loop {
        let runtime_function = plan.float_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_float_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.float_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_string_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: StringFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EcoString> {
    loop {
        let runtime_function = plan.string_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_string_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.string_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bit_array_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BitArrayFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedBitArray> {
    loop {
        let runtime_function = plan.bit_array_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_bit_array_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bit_array_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_utf_codepoint_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: UtfCodepointFunctionId,
    mut frame: Frame,
) -> ExecutionResult<char> {
    loop {
        let runtime_function = plan.utf_codepoint_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_utf_codepoint_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.utf_codepoint_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bool_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BoolFunctionId,
    mut frame: Frame,
) -> ExecutionResult<bool> {
    loop {
        let runtime_function = plan.bool_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_bool_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bool_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_nil_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: NilFunctionId,
    mut frame: Frame,
) -> ExecutionResult<()> {
    loop {
        let runtime_function = plan.nil_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_nil_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(()) => return finish_return(state, frame, ()),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.nil_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_tuple_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: TupleFunctionId,
    mut frame: Frame,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    loop {
        let runtime_function = plan.tuple_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_tuple_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.tuple_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    function: ListFunctionId,
    frame: Frame,
) -> ExecutionResult<ListValueId> {
    match function {
        ListFunctionId::Int(function) => {
            run_int_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::String(function) => {
            run_string_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::BitArray(function) => {
            run_bit_array_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::UtfCodepoint(function) => {
            run_utf_codepoint_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::Float(function) => {
            run_float_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::Bool(function) => {
            run_bool_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::Nil(function) => {
            run_nil_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::Tuple(function) => {
            run_tuple_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::List(function) => {
            run_list_list_loop(plan, state, function, frame).map(Into::into)
        }
        ListFunctionId::Function(function) => {
            run_function_list_loop(plan, state, function, frame).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn run_int_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: IntListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<IntListValueId> {
    loop {
        let runtime_function = plan.int_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_int_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.int_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_string_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: StringListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<StringListValueId> {
    loop {
        let runtime_function = plan.string_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_string_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.string_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bit_array_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BitArrayListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<BitArrayListValueId> {
    loop {
        let runtime_function = plan.bit_array_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_bit_array_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bit_array_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_utf_codepoint_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: UtfCodepointListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<UtfCodepointListValueId> {
    loop {
        let runtime_function = plan.utf_codepoint_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_utf_codepoint_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.utf_codepoint_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_float_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: FloatListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<FloatListValueId> {
    loop {
        let runtime_function = plan.float_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_float_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.float_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bool_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BoolListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<BoolListValueId> {
    loop {
        let runtime_function = plan.bool_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_bool_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bool_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_nil_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: NilListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<NilListValueId> {
    loop {
        let runtime_function = plan.nil_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_nil_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.nil_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_tuple_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: TupleListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<TupleListValueId> {
    loop {
        let runtime_function = plan.tuple_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_tuple_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.tuple_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_list_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: ListListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<ListListValueId> {
    loop {
        let runtime_function = plan.list_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_list_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.list_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_function_list_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: FunctionListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<FunctionListValueId> {
    loop {
        let runtime_function = plan.function_list_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_function_list_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.function_list_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_int_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: IntFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedIntFunction> {
    loop {
        let runtime_function = plan.int_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_int_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.int_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_float_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: FloatFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedFloatFunction> {
    loop {
        let runtime_function = plan.float_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_float_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.float_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_string_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: StringFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedStringFunction> {
    loop {
        let runtime_function = plan.string_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_string_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.string_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bit_array_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BitArrayFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    loop {
        let runtime_function = plan.bit_array_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_bit_array_function_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bit_array_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_utf_codepoint_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: UtfCodepointFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    loop {
        let runtime_function = plan.utf_codepoint_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let outcome = eval_return_body(
            plan,
            state,
            &mut frame,
            runtime_function.return_(),
            eval_utf_codepoint_function_expr,
        )?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.utf_codepoint_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bool_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: BoolFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedBoolFunction> {
    loop {
        let runtime_function = plan.bool_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_bool_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.bool_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_nil_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: NilFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedNilFunction> {
    loop {
        let runtime_function = plan.nil_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_nil_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.nil_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_tuple_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: TupleFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedTupleFunction> {
    loop {
        let runtime_function = plan.tuple_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_tuple_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.tuple_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_list_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: ListFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedListFunction> {
    loop {
        let runtime_function = plan.list_function_function(&function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_list_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.list_function_function(&next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_function_function_loop(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    mut function: FunctionFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    loop {
        let runtime_function = plan.function_function_function(function);
        execute_steps(plan, state, runtime_function.steps(), &mut frame)?;
        let eval = eval_function_function_expr;
        let outcome = eval_return_body(plan, state, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return finish_return(state, frame, value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.function_function_function(next).frame_layout();
                frame = bind_tail_arguments(plan, state, args, frame, frame_layout)?;
                function = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        IntFunctionId as RuntimeIntFunctionId, IntLocalId, ParamLocal, RuntimeFunctionId,
    };
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionId,
        ModulePlan, PanicExpr, PanicSite, ReturnBody, ReturnExpr, Step, StringExpr, ValueType,
    };
    use crate::runtime::{FunctionValue, ListValue, Value, run_main};

    #[test]
    fn main_materializes_compound_list_return_families() {
        assert_eq!(
            crate::runtime::run_src("pub fn main() { [#(1)] }"),
            Value::List(ListValue::from_evaluated_tuple(
                vec![ValueType::Int],
                vec![vec![Value::Int(1.into())]],
            )),
        );
        assert_eq!(
            crate::runtime::run_src("pub fn main() { [[1]] }"),
            Value::List(ListValue::from_evaluated_list(
                ValueType::Int,
                vec![ListValue::int(vec![1.into()])],
            )),
        );

        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            crate::runtime::run_src("fn one(value: Int) { value + 1 } pub fn main() { [one] }"),
            Value::List(ListValue::from_evaluated_function(
                function_type.clone(),
                vec![FunctionValue::new(
                    RuntimeFunctionId::Int(RuntimeIntFunctionId(0)),
                    vec![ParamLocal::Int(IntLocalId(0))],
                    function_type,
                )],
            )),
        );
    }

    #[test]
    fn return_body_subject_and_block_errors_propagate() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let value = || ReturnBody::expr(IntExpr::value(0.into()));
        let bodies = [
            ReturnBody::bool_case(BoolExpr::panic(panic()), value(), value()),
            ReturnBody::int_case(IntExpr::panic(panic()), Vec::new(), value()),
            ReturnBody::float_case(FloatExpr::panic(panic()), Vec::new(), value()),
            ReturnBody::string_case(StringExpr::panic(panic()), Vec::new(), value()),
            ReturnBody::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                value(),
            ),
        ];

        for body in bodies {
            let main = FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_body(IntFunctionId(0), body),
            );
            let module = ModulePlan::new("main".into(), main, Vec::new());
            let plan = crate::ExecutionPlan::from_module_plan(module);

            assert_eq!(
                run_main(&plan)
                    .expect_err("return body operand should fail")
                    .to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn source_tail_calls_preserve_every_return_family() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/block_case_tail_call.gleam"
                ),
                Value::Int(10_000.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/float_tail_recursion.gleam"
                ),
                Value::Float(5_000.0),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/function_returning_tail_call.gleam"
                ),
                Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/function_returning_tail_call_families.gleam"
                ),
                Value::String("okokok".into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/list_tail_recursion.gleam"
                ),
                Value::List(ListValue::int(vec![1.into(), 2.into(), 3.into()])),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/list_tail_recursion_item_families.gleam"
                ),
                Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/mutual_tail_recursion_bool.gleam"
                ),
                Value::Bool(false),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/string_nil_tail_recursion.gleam"
                ),
                Value::String("done".into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/tail_recursion_int.gleam"
                ),
                Value::Int(10_000.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/tail_call/tuple_tail_recursion.gleam"
                ),
                Value::Int(20_000.into()),
            ),
            (
                r#"
fn codepoint() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value }
fn recurse(count: Int) -> UtfCodepoint { case count { 0 -> codepoint() _ -> recurse(count - 1) } }
pub fn main() { recurse(3) }
"#,
                Value::UtfCodepoint('A'),
            ),
            (
                r#"
fn codepoint() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value }
fn recurse(count: Int) -> List(UtfCodepoint) { case count { 0 -> [codepoint()] _ -> recurse(count - 1) } }
pub fn main() { recurse(3) }
"#,
                Value::List(ListValue::utf_codepoint(vec!['A'])),
            ),
            (
                r#"
fn codepoint() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value }
fn recurse(count: Int) -> fn() -> UtfCodepoint { case count { 0 -> fn() { codepoint() } _ -> recurse(count - 1) } }
pub fn main() { recurse(3)() }
"#,
                Value::UtfCodepoint('A'),
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src(source), expected);
        }
    }

    #[test]
    fn function_loops_propagate_step_errors_for_every_return_family() {
        let return_shapes = [
            ("Int", "0"),
            ("String", "\"\""),
            ("BitArray", "<<>>"),
            ("UtfCodepoint", "panic"),
            ("Float", "0.0"),
            ("Bool", "False"),
            ("Nil", "Nil"),
            ("#(Int)", "#(0)"),
            ("List(Int)", "[]"),
            ("List(String)", "[]"),
            ("List(BitArray)", "[]"),
            ("List(UtfCodepoint)", "[]"),
            ("List(Float)", "[]"),
            ("List(Bool)", "[]"),
            ("List(Nil)", "[]"),
            ("List(#(Int))", "[]"),
            ("List(List(Int))", "[]"),
            ("List(fn() -> Int)", "[]"),
            ("fn() -> Int", "fn() { 0 }"),
            ("fn() -> String", "fn() { \"\" }"),
            ("fn() -> BitArray", "fn() { <<>> }"),
            ("fn() -> UtfCodepoint", "fn() { panic }"),
            ("fn() -> Float", "fn() { 0.0 }"),
            ("fn() -> Bool", "fn() { False }"),
            ("fn() -> Nil", "fn() { Nil }"),
            ("fn() -> #(Int)", "fn() { #(0) }"),
            ("fn() -> List(Int)", "fn() { [] }"),
            ("fn() -> fn() -> Int", "fn() { fn() { 0 } }"),
        ];

        for (return_type, return_value) in return_shapes {
            let source = format!(
                "pub fn main() -> {return_type} {{ let value: Int = panic as \"step\" let _ = value {return_value} }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: step",
            );
        }
    }

    #[test]
    fn function_loops_propagate_tail_argument_errors_for_every_return_family() {
        let return_types = [
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
            "fn() -> fn() -> Int",
        ];

        for return_type in return_types {
            let source = format!(
                "fn recurse(value: Int) -> {return_type} {{ recurse(panic as \"tail\") }} pub fn main() -> {return_type} {{ recurse(0) }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: tail",
            );
        }
    }

    #[test]
    fn generic_list_loop_materializes_compound_values() {
        assert_eq!(
            crate::runtime::run_src("pub fn main() -> List(#(Int)) { [#(1)] }"),
            Value::List(ListValue::from_evaluated_tuple(
                vec![crate::plan::ValueType::Int],
                vec![vec![Value::Int(1.into())]],
            )),
        );
        assert_eq!(
            crate::runtime::run_src("pub fn main() -> List(List(Int)) { [[1]] }"),
            Value::List(ListValue::from_evaluated_list(
                crate::plan::ValueType::Int,
                vec![ListValue::int(vec![1.into()])],
            )),
        );
        assert_eq!(
            crate::runtime::run_src("pub fn main() -> List(fn() -> Int) { [fn() { 1 }] }"),
            Value::List(ListValue::from_evaluated_function(
                crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Int),
                vec![crate::runtime::FunctionValue::from(
                    crate::runtime::IntFunctionValue::new(
                        crate::plan::execution::IntFunctionId(0),
                        Vec::new(),
                        crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Int,),
                    ),
                )],
            )),
        );
    }

    #[test]
    fn list_return_loops_propagate_source_panics_for_every_item_family() {
        let sources = [
            "pub fn main() -> List(Int) { panic }",
            "pub fn main() -> List(String) { panic }",
            "pub fn main() -> List(BitArray) { panic }",
            "pub fn main() -> List(UtfCodepoint) { panic }",
            "pub fn main() -> List(Float) { panic }",
            "pub fn main() -> List(Bool) { panic }",
            "pub fn main() -> List(Nil) { panic }",
            "pub fn main() -> List(#(Int)) { panic }",
            "pub fn main() -> List(List(Int)) { panic }",
            "pub fn main() -> List(fn() -> Int) { panic }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("panic expression should fail execution");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
    }
}
