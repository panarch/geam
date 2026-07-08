use super::bind::bind_arguments;
use super::steps::execute_steps;
use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, ExecutionPlan, FloatFunctionFunctionId,
    FloatFunctionId, FunctionFunctionFunctionId, FunctionFunctionValue, IntFunctionFunctionId,
    IntFunctionId, ListFunctionFunctionId, ListFunctionId, ListFunctionValue, ListReturn,
    ListValue, NilFunctionFunctionId, NilFunctionId, ReturnBody, ReturnBodyKind,
    StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId, TupleFunctionId, Value,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_list_function_expr,
    eval_nil_expr, eval_nil_function_expr, eval_string_expr, eval_string_function_expr,
    eval_tuple_expr, eval_tuple_function_expr, eval_typed_list_expr,
};
use crate::runtime::frame::Frame;
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
enum ReturnOutcome<'a, Value, Function> {
    Value(Value),
    TailCall {
        function: Function,
        args: &'a [CallArg],
    },
}

fn eval_return_body<'a, Expression, Function, Value>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    body: &'a ReturnBody<Expression, Function>,
    eval_expression: fn(&ExecutionPlan, &mut Frame, &Expression) -> ExecutionResult<Value>,
) -> ExecutionResult<ReturnOutcome<'a, Value, Function>>
where
    Function: Clone,
{
    match body.kind() {
        ReturnBodyKind::Expr(expression) => {
            eval_expression(plan, frame, expression).map(ReturnOutcome::Value)
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
        ReturnBodyKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_return_body(plan, frame, branch, eval_expression);
                }
            }
            eval_return_body(plan, frame, fallback, eval_expression)
        }
        ReturnBodyKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
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

pub(super) fn run_int_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_float_loop(
    plan: &ExecutionPlan,
    mut function: FloatFunctionId,
    mut frame: Frame,
) -> ExecutionResult<f64> {
    loop {
        let runtime_function = plan.float_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_float_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.float_function(next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_string_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bool_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_nil_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_tuple_loop(
    plan: &ExecutionPlan,
    mut function: TupleFunctionId,
    mut frame: Frame,
) -> ExecutionResult<Vec<Value>> {
    loop {
        let runtime_function = plan.tuple_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_tuple_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.tuple_function(next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_list_loop(
    plan: &ExecutionPlan,
    mut function: ListFunctionId,
    mut frame: Frame,
) -> ExecutionResult<ListValue> {
    loop {
        let runtime_function = plan.list_function(&function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let outcome = eval_list_return_body(plan, &mut frame, runtime_function.return_())?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.list_function(&next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

fn eval_list_return_body<'a>(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    body: &'a ListReturn,
) -> ExecutionResult<ReturnOutcome<'a, ListValue, ListFunctionId>> {
    match body {
        ListReturn::Int(body) => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            ListFunctionId::Int,
        ),
        ListReturn::Float(body) => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            ListFunctionId::Float,
        ),
        ListReturn::String(body) => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            ListFunctionId::String,
        ),
        ListReturn::Bool(body) => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            ListFunctionId::Bool,
        ),
        ListReturn::Nil(body) => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            ListFunctionId::Nil,
        ),
        ListReturn::Tuple { item_type, body } => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            |id| ListFunctionId::Tuple {
                id,
                item_type: item_type.clone(),
            },
        ),
        ListReturn::List { item_type, body } => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            |id| ListFunctionId::List {
                id,
                item_type: item_type.clone(),
            },
        ),
        ListReturn::Function { item_type, body } => map_list_return_outcome(
            eval_return_body(plan, frame, body, eval_typed_list_expr)?,
            |id| ListFunctionId::Function {
                id,
                item_type: item_type.clone(),
            },
        ),
    }
}

fn map_list_return_outcome<'a, Function>(
    outcome: ReturnOutcome<'a, ListValue, Function>,
    function: impl FnOnce(Function) -> ListFunctionId,
) -> ExecutionResult<ReturnOutcome<'a, ListValue, ListFunctionId>> {
    Ok(match outcome {
        ReturnOutcome::Value(value) => ReturnOutcome::Value(value),
        ReturnOutcome::TailCall { function: id, args } => ReturnOutcome::TailCall {
            function: function(id),
            args,
        },
    })
}

pub(super) fn run_int_function_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_float_function_loop(
    plan: &ExecutionPlan,
    mut function: FloatFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::FloatFunctionValue> {
    loop {
        let runtime_function = plan.float_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_float_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.float_function_function(next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_string_function_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_bool_function_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_nil_function_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_tuple_function_loop(
    plan: &ExecutionPlan,
    mut function: TupleFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<crate::plan::TupleFunctionValue> {
    loop {
        let runtime_function = plan.tuple_function_function(function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_tuple_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.tuple_function_function(next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_list_function_loop(
    plan: &ExecutionPlan,
    mut function: ListFunctionFunctionId,
    mut frame: Frame,
) -> ExecutionResult<ListFunctionValue> {
    loop {
        let runtime_function = plan.list_function_function(&function);
        execute_steps(plan, runtime_function.steps(), &mut frame)?;
        let eval = eval_list_function_expr;
        let outcome = eval_return_body(plan, &mut frame, runtime_function.return_(), eval)?;
        match outcome {
            ReturnOutcome::Value(value) => return Ok(value),
            ReturnOutcome::TailCall {
                function: next,
                args,
            } => {
                let frame_layout = plan.list_function_function(&next).frame_layout();
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

pub(super) fn run_function_function_loop(
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
                frame = bind_arguments(plan, args, &mut frame, frame_layout)?;
                function = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ReturnOutcome, eval_list_return_body, run_bool_function_loop, run_bool_loop,
        run_float_function_loop, run_float_loop, run_function_function_loop, run_int_function_loop,
        run_int_loop, run_list_function_loop, run_list_loop, run_nil_function_loop, run_nil_loop,
        run_string_function_loop, run_string_loop, run_tuple_function_loop, run_tuple_loop,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionValue,
        BoolListFunctionId, CallArg, ExecutionPlan, Expr, FloatExpr, FloatFunctionExpr,
        FloatFunctionFunctionId, FloatFunctionId, FloatFunctionValue, FloatListFunctionId,
        FunctionExpr, FunctionExprKind, FunctionFunctionExpr, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionValue, FunctionId,
        FunctionListFunctionId, FunctionPlan, FunctionReturnFamily, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionValue, IntListFunctionId,
        ListExpr, ListFunctionExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionValue,
        ListListFunctionId, ListReturn, ListValue, NilExpr, NilFunctionExpr, NilFunctionFunctionId,
        NilFunctionId, NilFunctionValue, NilListFunctionId, PanicExpr, PanicSite, ReturnBody,
        ReturnExpr, Step, StringExpr, StringFunctionExpr, StringFunctionFunctionId,
        StringFunctionId, StringFunctionValue, StringListFunctionId, TupleExpr, TupleFunctionExpr,
        TupleFunctionFunctionId, TupleFunctionId, TupleFunctionValue, TupleListFunctionId, Value,
        ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use num_bigint::BigInt;

    #[test]
    fn primitive_return_loops_propagate_step_errors() {
        let plan = primitive_function_plan_with_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_loop(&plan, IntFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_string_loop(
            &plan,
            StringFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_loop(
            &plan,
            FloatFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_loop(&plan, BoolFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_nil_loop(&plan, NilFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_tuple_loop(
            &plan,
            TupleFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_loop(
            &plan,
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            Frame::default(),
        ));
    }

    #[test]
    fn function_return_loops_propagate_step_errors() {
        let plan = plan_with_function_function_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_function_loop(
            &plan,
            IntFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_loop(
            &plan,
            StringFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_function_loop(
            &plan,
            FloatFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_loop(
            &plan,
            BoolFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_loop(
            &plan,
            NilFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_loop(
            &plan,
            TupleFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_loop(
            &plan,
            ListFunctionFunctionId::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int,
            ),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_loop(
            &plan,
            FunctionFunctionFunctionId(0),
            Frame::default(),
        ));
    }

    #[test]
    fn return_body_cases_and_block_choose_expected_branches() {
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::expr(IntExpr::value(1.into())),
                    ReturnBody::expr(IntExpr::value(2.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(1.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::bool_case(
                    BoolExpr::value(false),
                    ReturnBody::expr(IntExpr::value(1.into())),
                    ReturnBody::expr(IntExpr::value(2.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(2.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::int_case(
                    IntExpr::value(2.into()),
                    vec![(BigInt::from(2), ReturnBody::expr(IntExpr::value(3.into())))],
                    ReturnBody::expr(IntExpr::value(4.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(3.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::int_case(
                    IntExpr::value(5.into()),
                    vec![(BigInt::from(2), ReturnBody::expr(IntExpr::value(3.into())))],
                    ReturnBody::expr(IntExpr::value(4.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(4.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::float_case(
                    FloatExpr::value(1.5),
                    vec![(1.5, ReturnBody::expr(IntExpr::value(5.into())))],
                    ReturnBody::expr(IntExpr::value(6.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(5.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::float_case(
                    FloatExpr::value(2.5),
                    vec![(1.5, ReturnBody::expr(IntExpr::value(5.into())))],
                    ReturnBody::expr(IntExpr::value(6.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(6.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), ReturnBody::expr(IntExpr::value(7.into())))],
                    ReturnBody::expr(IntExpr::value(8.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(7.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), ReturnBody::expr(IntExpr::value(7.into())))],
                    ReturnBody::expr(IntExpr::value(8.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(8.into()),
        );
        assert_eq!(
            run_int_loop(
                &int_return_body_plan(ReturnBody::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(0.into())))],
                    ReturnBody::expr(IntExpr::value(9.into())),
                )),
                IntFunctionId(0),
                Frame::default(),
            ),
            Ok(9.into()),
        );
        assert_expected_function_got_int(run_int_loop(
            &int_return_body_plan(ReturnBody::block(
                vec![failing_step()],
                ReturnBody::expr(IntExpr::value(10.into())),
            )),
            IntFunctionId(0),
            Frame::default(),
        ));
    }

    #[test]
    fn return_body_case_subject_errors_propagate() {
        assert_expected_function_got_int(run_int_loop(
            &int_return_body_plan(ReturnBody::bool_case(
                failing_bool_expr(),
                ReturnBody::expr(IntExpr::value(1.into())),
                ReturnBody::expr(IntExpr::value(2.into())),
            )),
            IntFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_int_loop(
            &int_return_body_plan(ReturnBody::int_case(
                failing_int_expr(),
                vec![(BigInt::from(1), ReturnBody::expr(IntExpr::value(1.into())))],
                ReturnBody::expr(IntExpr::value(2.into())),
            )),
            IntFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_int_loop(
            &int_return_body_plan(ReturnBody::float_case(
                failing_float_expr(),
                vec![(1.5, ReturnBody::expr(IntExpr::value(1.into())))],
                ReturnBody::expr(IntExpr::value(2.into())),
            )),
            IntFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_int_loop(
            &int_return_body_plan(ReturnBody::string_case(
                failing_string_expr(),
                vec![("hit".into(), ReturnBody::expr(IntExpr::value(1.into())))],
                ReturnBody::expr(IntExpr::value(2.into())),
            )),
            IntFunctionId(0),
            Frame::default(),
        ));
    }

    #[test]
    fn primitive_return_loops_propagate_return_body_errors() {
        let plan = primitive_function_plan_with_return_body_errors();

        assert_expected_function_got_int(run_int_loop(&plan, IntFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_string_loop(
            &plan,
            StringFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_loop(
            &plan,
            FloatFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_loop(&plan, BoolFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_nil_loop(&plan, NilFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_tuple_loop(
            &plan,
            TupleFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_loop(
            &plan,
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            Frame::default(),
        ));
    }

    #[test]
    fn function_return_loops_propagate_return_body_errors() {
        let plan = function_function_plan_with_return_body_errors();

        assert_expected_function_got_int(run_int_function_loop(
            &plan,
            IntFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_loop(
            &plan,
            StringFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_function_loop(
            &plan,
            FloatFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_loop(
            &plan,
            BoolFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_loop(
            &plan,
            NilFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_loop(
            &plan,
            TupleFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_loop(
            &plan,
            ListFunctionFunctionId::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int,
            ),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_loop(
            &plan,
            FunctionFunctionFunctionId(0),
            Frame::default(),
        ));
    }

    #[test]
    fn primitive_return_loops_follow_tail_calls() {
        let plan = primitive_tail_call_plan(Vec::new());

        assert_eq!(
            run_int_loop(&plan, IntFunctionId(0), Frame::default()),
            Ok(2.into()),
        );
        assert_eq!(
            run_string_loop(&plan, StringFunctionId(0), Frame::default()),
            Ok("done".into()),
        );
        assert_eq!(
            run_float_loop(&plan, FloatFunctionId(0), Frame::default()),
            Ok(2.5),
        );
        assert_eq!(
            run_bool_loop(&plan, BoolFunctionId(0), Frame::default()),
            Ok(false),
        );
        assert_eq!(
            run_nil_loop(&plan, NilFunctionId(0), Frame::default()),
            Ok(())
        );
        assert_eq!(
            run_tuple_loop(&plan, TupleFunctionId(0), Frame::default()),
            Ok(vec![Value::Int(2.into())]),
        );
        assert_eq!(
            run_list_loop(
                &plan,
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                Frame::default()
            ),
            Ok(ListValue::int(vec![2.into()])),
        );
    }

    #[test]
    fn function_return_loops_follow_tail_calls() {
        let plan = function_tail_call_plan(Vec::new());

        assert_eq!(
            run_int_function_loop(&plan, IntFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(IntFunctionId(0)),
        );
        assert_eq!(
            run_string_function_loop(&plan, StringFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(StringFunctionId(0)),
        );
        assert_eq!(
            run_float_function_loop(&plan, FloatFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(FloatFunctionId(0)),
        );
        assert_eq!(
            run_bool_function_loop(&plan, BoolFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(BoolFunctionId(0)),
        );
        assert_eq!(
            run_nil_function_loop(&plan, NilFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(NilFunctionId(0)),
        );
        assert_eq!(
            run_tuple_function_loop(&plan, TupleFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(TupleFunctionId(0)),
        );
        assert_eq!(
            run_list_function_loop(
                &plan,
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                ),
                Frame::default()
            )
            .map(|value| value.runtime_id()),
            Ok(ListFunctionId::from_item_type(
                0,
                crate::plan::ValueType::Int
            )),
        );
        assert_eq!(
            run_function_function_loop(&plan, FunctionFunctionFunctionId(0), Frame::default())
                .map(|value| value.runtime_id()),
            Ok(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
        );
    }

    #[test]
    fn list_return_body_dispatches_compound_item_family_values_and_tail_calls() {
        let plan = int_return_body_plan(ReturnBody::expr(IntExpr::value(1.into())));
        let mut frame = Frame::default();
        let tuple_item = vec![ValueType::Int];
        let list_item = ValueType::String;
        let function_item = FunctionType::new(Vec::new(), ValueType::Bool);

        for (body, expected) in [
            (
                ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Nil)),
                ListValue::nil(0),
            ),
            (
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::Tuple(tuple_item.clone()),
                )),
                ListValue::tuple(tuple_item.clone(), Vec::new()),
            ),
            (
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::List(Box::new(list_item.clone())),
                )),
                ListValue::list(list_item.clone(), Vec::new()),
            ),
            (
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(function_item.clone())),
                )),
                ListValue::function(function_item.clone(), Vec::new()),
            ),
        ] {
            assert_eq!(
                eval_list_return_body(&plan, &mut frame, &body),
                Ok(ReturnOutcome::Value(expected)),
            );
        }

        for (body, expected) in [
            (
                ListReturn::tail_call(ListFunctionId::Int(IntListFunctionId(0)), Vec::new()),
                ListFunctionId::Int(IntListFunctionId(0)),
            ),
            (
                ListReturn::tail_call(ListFunctionId::Float(FloatListFunctionId(0)), Vec::new()),
                ListFunctionId::Float(FloatListFunctionId(0)),
            ),
            (
                ListReturn::tail_call(ListFunctionId::String(StringListFunctionId(0)), Vec::new()),
                ListFunctionId::String(StringListFunctionId(0)),
            ),
            (
                ListReturn::tail_call(ListFunctionId::Bool(BoolListFunctionId(0)), Vec::new()),
                ListFunctionId::Bool(BoolListFunctionId(0)),
            ),
            (
                ListReturn::tail_call(ListFunctionId::Nil(NilListFunctionId(0)), Vec::new()),
                ListFunctionId::Nil(NilListFunctionId(0)),
            ),
            (
                ListReturn::tail_call(
                    ListFunctionId::Tuple {
                        id: TupleListFunctionId(0),
                        item_type: tuple_item.clone(),
                    },
                    Vec::new(),
                ),
                ListFunctionId::Tuple {
                    id: TupleListFunctionId(0),
                    item_type: tuple_item,
                },
            ),
            (
                ListReturn::tail_call(
                    ListFunctionId::List {
                        id: ListListFunctionId(0),
                        item_type: Box::new(list_item.clone()),
                    },
                    Vec::new(),
                ),
                ListFunctionId::List {
                    id: ListListFunctionId(0),
                    item_type: Box::new(list_item),
                },
            ),
            (
                ListReturn::tail_call(
                    ListFunctionId::Function {
                        id: FunctionListFunctionId(0),
                        item_type: function_item.clone(),
                    },
                    Vec::new(),
                ),
                ListFunctionId::Function {
                    id: FunctionListFunctionId(0),
                    item_type: function_item,
                },
            ),
        ] {
            assert_eq!(
                eval_list_return_body(&plan, &mut frame, &body),
                Ok(ReturnOutcome::TailCall {
                    function: expected,
                    args: &[],
                }),
            );
        }
    }

    #[test]
    fn list_return_body_dispatches_item_family_evaluation_errors() {
        let plan = int_return_body_plan(ReturnBody::expr(IntExpr::value(1.into())));
        let item_types = vec![
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        ];

        for item_type in item_types {
            let body = ListReturn::expr(ListExpr::panic(
                PanicExpr::panic_at(None, PanicSite::unknown()),
                item_type,
            ));
            let mut frame = Frame::default();

            assert_eq!(
                eval_list_return_body(&plan, &mut frame, &body),
                Err(ExecutionError::source_panic(
                    None,
                    PanicKind::Panic,
                    None,
                    PanicSite::unknown(),
                )),
            );
        }
    }

    #[test]
    fn primitive_return_loops_propagate_tail_call_binding_errors() {
        let plan = primitive_tail_call_binding_error_plan();

        assert_expected_function_got_int(run_int_loop(&plan, IntFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_string_loop(
            &plan,
            StringFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_loop(
            &plan,
            FloatFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_loop(&plan, BoolFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_nil_loop(&plan, NilFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_tuple_loop(
            &plan,
            TupleFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_loop(
            &plan,
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            Frame::default(),
        ));
    }

    #[test]
    fn function_return_loops_propagate_tail_call_binding_errors() {
        let plan = function_tail_call_binding_error_plan();

        assert_expected_function_got_int(run_int_function_loop(
            &plan,
            IntFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_string_function_loop(
            &plan,
            StringFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_float_function_loop(
            &plan,
            FloatFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_function_loop(
            &plan,
            BoolFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_nil_function_loop(
            &plan,
            NilFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_tuple_function_loop(
            &plan,
            TupleFunctionFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_list_function_loop(
            &plan,
            ListFunctionFunctionId::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int,
            ),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_function_function_loop(
            &plan,
            FunctionFunctionFunctionId(0),
            Frame::default(),
        ));
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
            function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0))),
            Vec::new(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        )
    }

    fn failing_step() -> Step {
        Step::evaluate(Expr::function(FunctionExpr::function(
            failing_function_function_expr(),
        )))
    }

    fn failing_function_function_arg() -> CallArg {
        CallArg::function_function(FunctionFunctionLocalId(0), failing_function_function_expr())
    }

    fn int_return_body_plan(body: ReturnBody<IntExpr, IntFunctionId>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_body(IntFunctionId(0), body),
            ),
            Vec::new(),
        )
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
                function_plan(3, "float_function", steps.clone(), float_function_expr()),
                function_plan(4, "bool_function", steps.clone(), bool_function_expr()),
                function_plan(5, "nil_function", steps.clone(), nil_function_expr()),
                function_plan(6, "tuple_function", steps.clone(), tuple_function_expr()),
                function_plan(7, "list_function", steps.clone(), list_function_expr()),
                function_plan(
                    8,
                    "function_function",
                    steps,
                    function_function_expr_value(),
                ),
            ],
        )
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
                    "float".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(1.5)),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "bool".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::bool(BoolFunctionId(0), BoolExpr::value(true)),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "nil".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::nil(NilFunctionId(0), NilExpr::value()),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "tuple".into(),
                    Vec::new(),
                    steps.clone(),
                    ReturnExpr::tuple(
                        TupleFunctionId(0),
                        TupleExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            vec![ValueType::Int],
                        ),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "list".into(),
                    Vec::new(),
                    steps,
                    ReturnExpr::list_body(
                        ListFunctionId::from_item_type(0, ValueType::Int),
                        ListReturn::expr(ListExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            ValueType::Int,
                        )),
                    ),
                ),
            ],
        )
    }

    fn primitive_function_plan_with_return_body_errors() -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_body(IntFunctionId(0), failing_int_return_body()),
            ),
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "string".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string_body(StringFunctionId(0), failing_string_return_body()),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "float".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_body(FloatFunctionId(0), failing_float_return_body()),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "bool".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool_body(BoolFunctionId(0), failing_bool_return_body()),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "nil".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil_body(NilFunctionId(0), failing_nil_return_body()),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "tuple".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_body(
                        TupleFunctionId(0),
                        vec![ValueType::Int],
                        failing_tuple_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "list".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_body(
                        ListFunctionId::from_item_type(0, ValueType::Int),
                        failing_list_return_body(),
                    ),
                ),
            ],
        )
    }

    fn function_function_plan_with_return_body_errors() -> ExecutionPlan {
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
                FunctionPlan::new(
                    FunctionId::new(1),
                    "int_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::int_function_body(
                        IntFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Int),
                        failing_int_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "string_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string_function_body(
                        StringFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::String),
                        failing_string_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "float_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_function_body(
                        FloatFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Float),
                        failing_float_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "bool_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool_function_body(
                        BoolFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Bool),
                        failing_bool_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "nil_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil_function_body(
                        NilFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Nil),
                        failing_nil_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "tuple_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_function_body(
                        TupleFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Tuple(vec![ValueType::Int])),
                        failing_tuple_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(7),
                    "list_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_function_body(
                        ListFunctionFunctionId::from_item_type(
                            0,
                            zero_arg_function_type(ValueType::List(Box::new(ValueType::Int))),
                            ValueType::Int,
                        ),
                        failing_list_function_return_body(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(8),
                    "function_function".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::function_function_body(
                        FunctionFunctionFunctionId(0),
                        function_function_type(),
                        failing_function_function_return_body(),
                    ),
                ),
            ],
        )
    }

    fn primitive_tail_call_binding_error_plan() -> ExecutionPlan {
        primitive_tail_call_plan(vec![failing_function_function_arg()])
    }

    fn primitive_tail_call_plan(args: Vec<CallArg>) -> ExecutionPlan {
        ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_body(
                    IntFunctionId(0),
                    ReturnBody::tail_call(IntFunctionId(1), args.clone()),
                ),
            ),
            vec![
                FunctionPlan::new(
                    FunctionId::new(1),
                    "int_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::int(IntFunctionId(1), IntExpr::value(2.into())),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "string_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string_body(
                        StringFunctionId(0),
                        ReturnBody::tail_call(StringFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "string_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string(StringFunctionId(1), StringExpr::value("done".into())),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "float_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_body(
                        FloatFunctionId(0),
                        ReturnBody::tail_call(FloatFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "float_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float(FloatFunctionId(1), FloatExpr::value(2.5)),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "bool_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool_body(
                        BoolFunctionId(0),
                        ReturnBody::tail_call(BoolFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(7),
                    "bool_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool(BoolFunctionId(1), BoolExpr::value(false)),
                ),
                FunctionPlan::new(
                    FunctionId::new(8),
                    "nil_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil_body(
                        NilFunctionId(0),
                        ReturnBody::tail_call(NilFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(9),
                    "nil_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil(NilFunctionId(1), NilExpr::value()),
                ),
                FunctionPlan::new(
                    FunctionId::new(10),
                    "tuple_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_body(
                        TupleFunctionId(0),
                        vec![ValueType::Int],
                        ReturnBody::tail_call(TupleFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(11),
                    "tuple_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple(TupleFunctionId(1), tuple_value_expr_with_int(2)),
                ),
                FunctionPlan::new(
                    FunctionId::new(12),
                    "list_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_body(
                        ListFunctionId::from_item_type(0, ValueType::Int),
                        ListReturn::tail_call(
                            ListFunctionId::from_item_type(1, ValueType::Int),
                            args.clone(),
                        ),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(13),
                    "list_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_body(
                        ListFunctionId::from_item_type(1, ValueType::Int),
                        ListReturn::expr(list_value_expr_with_int(2)),
                    ),
                ),
            ],
        )
    }

    fn function_tail_call_binding_error_plan() -> ExecutionPlan {
        function_tail_call_plan(vec![failing_function_function_arg()])
    }

    fn function_tail_call_plan(args: Vec<CallArg>) -> ExecutionPlan {
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
                FunctionPlan::new(
                    FunctionId::new(1),
                    "int_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::int_function_body(
                        IntFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Int),
                        ReturnBody::tail_call(IntFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(2),
                    "int_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::int_function(IntFunctionFunctionId(1), int_function_value_expr()),
                ),
                FunctionPlan::new(
                    FunctionId::new(3),
                    "string_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string_function_body(
                        StringFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::String),
                        ReturnBody::tail_call(StringFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(4),
                    "string_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::string_function(
                        StringFunctionFunctionId(1),
                        string_function_value_expr(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(5),
                    "float_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_function_body(
                        FloatFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Float),
                        ReturnBody::tail_call(FloatFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(6),
                    "float_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::float_function(
                        FloatFunctionFunctionId(1),
                        float_function_value_expr(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(7),
                    "bool_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool_function_body(
                        BoolFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Bool),
                        ReturnBody::tail_call(BoolFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(8),
                    "bool_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::bool_function(
                        BoolFunctionFunctionId(1),
                        bool_function_value_expr(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(9),
                    "nil_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil_function_body(
                        NilFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Nil),
                        ReturnBody::tail_call(NilFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(10),
                    "nil_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::nil_function(NilFunctionFunctionId(1), nil_function_value_expr()),
                ),
                FunctionPlan::new(
                    FunctionId::new(11),
                    "tuple_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_function_body(
                        TupleFunctionFunctionId(0),
                        zero_arg_function_type(ValueType::Tuple(vec![ValueType::Int])),
                        ReturnBody::tail_call(TupleFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(12),
                    "tuple_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::tuple_function(
                        TupleFunctionFunctionId(1),
                        tuple_function_value_expr(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(13),
                    "list_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_function_body(
                        ListFunctionFunctionId::from_item_type(
                            0,
                            zero_arg_function_type(ValueType::List(Box::new(ValueType::Int))),
                            ValueType::Int,
                        ),
                        ReturnBody::tail_call(
                            ListFunctionFunctionId::from_item_type(
                                1,
                                zero_arg_function_type(ValueType::List(Box::new(ValueType::Int))),
                                ValueType::Int,
                            ),
                            args.clone(),
                        ),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(14),
                    "list_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::list_function(
                        ListFunctionFunctionId::from_item_type(
                            1,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        list_function_value_expr(),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(15),
                    "function_function_tail".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::function_function_body(
                        FunctionFunctionFunctionId(0),
                        function_function_type(),
                        ReturnBody::tail_call(FunctionFunctionFunctionId(1), args.clone()),
                    ),
                ),
                FunctionPlan::new(
                    FunctionId::new(16),
                    "function_function_done".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::function_function(
                        FunctionFunctionFunctionId(1),
                        function_function_value_expr_typed(),
                    ),
                ),
            ],
        )
    }

    fn failing_int_return_body() -> ReturnBody<IntExpr, IntFunctionId> {
        ReturnBody::expr(IntExpr::block(
            vec![failing_step()],
            IntExpr::value(1.into()),
        ))
    }

    fn failing_string_return_body() -> ReturnBody<StringExpr, StringFunctionId> {
        ReturnBody::expr(StringExpr::block(
            vec![failing_step()],
            StringExpr::value("geam".into()),
        ))
    }

    fn failing_float_return_body() -> ReturnBody<FloatExpr, FloatFunctionId> {
        ReturnBody::expr(FloatExpr::block(
            vec![failing_step()],
            FloatExpr::value(1.5),
        ))
    }

    fn failing_bool_return_body() -> ReturnBody<BoolExpr, BoolFunctionId> {
        ReturnBody::expr(BoolExpr::block(vec![failing_step()], BoolExpr::value(true)))
    }

    fn failing_nil_return_body() -> ReturnBody<NilExpr, NilFunctionId> {
        ReturnBody::expr(NilExpr::block(vec![failing_step()], NilExpr::value()))
    }

    fn failing_tuple_return_body() -> ReturnBody<TupleExpr, TupleFunctionId> {
        ReturnBody::expr(TupleExpr::block(vec![failing_step()], tuple_value_expr()))
    }

    fn failing_list_return_body() -> ListReturn {
        ListReturn::expr(ListExpr::block(vec![failing_step()], list_value_expr()))
    }

    fn failing_int_function_return_body() -> ReturnBody<IntFunctionExpr, IntFunctionFunctionId> {
        ReturnBody::expr(IntFunctionExpr::block(
            vec![failing_step()],
            int_function_value_expr(),
        ))
    }

    fn failing_string_function_return_body()
    -> ReturnBody<StringFunctionExpr, StringFunctionFunctionId> {
        ReturnBody::expr(StringFunctionExpr::block(
            vec![failing_step()],
            string_function_value_expr(),
        ))
    }

    fn failing_float_function_return_body() -> ReturnBody<FloatFunctionExpr, FloatFunctionFunctionId>
    {
        ReturnBody::expr(FloatFunctionExpr::block(
            vec![failing_step()],
            float_function_value_expr(),
        ))
    }

    fn failing_bool_function_return_body() -> ReturnBody<BoolFunctionExpr, BoolFunctionFunctionId> {
        ReturnBody::expr(BoolFunctionExpr::block(
            vec![failing_step()],
            bool_function_value_expr(),
        ))
    }

    fn failing_nil_function_return_body() -> ReturnBody<NilFunctionExpr, NilFunctionFunctionId> {
        ReturnBody::expr(NilFunctionExpr::block(
            vec![failing_step()],
            nil_function_value_expr(),
        ))
    }

    fn failing_tuple_function_return_body() -> ReturnBody<TupleFunctionExpr, TupleFunctionFunctionId>
    {
        ReturnBody::expr(TupleFunctionExpr::block(
            vec![failing_step()],
            tuple_function_value_expr(),
        ))
    }

    fn failing_list_function_return_body() -> ReturnBody<ListFunctionExpr, ListFunctionFunctionId> {
        ReturnBody::expr(ListFunctionExpr::block(
            vec![failing_step()],
            list_function_value_expr(),
        ))
    }

    fn failing_function_function_return_body()
    -> ReturnBody<FunctionFunctionExpr, FunctionFunctionFunctionId> {
        ReturnBody::expr(FunctionFunctionExpr::block(
            vec![failing_step()],
            function_function_value_expr_typed(),
        ))
    }

    fn failing_int_expr() -> IntExpr {
        IntExpr::block(vec![failing_step()], IntExpr::value(1.into()))
    }

    fn failing_string_expr() -> StringExpr {
        StringExpr::block(vec![failing_step()], StringExpr::value("geam".into()))
    }

    fn failing_float_expr() -> FloatExpr {
        FloatExpr::block(vec![failing_step()], FloatExpr::value(1.5))
    }

    fn failing_bool_expr() -> BoolExpr {
        BoolExpr::block(vec![failing_step()], BoolExpr::value(true))
    }

    fn tuple_value_expr() -> TupleExpr {
        tuple_value_expr_with_int(1)
    }

    fn tuple_value_expr_with_int(value: i64) -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(value.into()))],
            vec![ValueType::Int],
        )
    }

    fn list_value_expr() -> ListExpr {
        list_value_expr_with_int(1)
    }

    fn list_value_expr_with_int(value: i64) -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(value.into()))],
            ValueType::Int,
        )
    }

    fn int_function_value_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(IntFunctionId(0), Vec::new()))
    }

    fn string_function_value_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(StringFunctionId(0), Vec::new()))
    }

    fn float_function_value_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(FloatFunctionId(0), Vec::new()))
    }

    fn bool_function_value_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new()))
    }

    fn nil_function_value_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new()))
    }

    fn tuple_function_value_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            Vec::new(),
            vec![ValueType::Int],
        ))
    }

    fn list_function_value_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            Vec::new(),
        ))
    }

    fn function_function_value_expr_typed() -> FunctionFunctionExpr {
        function_function_expr(FunctionFunctionId::Int(IntFunctionFunctionId(0)))
    }

    fn zero_arg_function_type(return_: ValueType) -> FunctionType {
        FunctionType::new(Vec::new(), return_)
    }

    fn function_function_type() -> FunctionType {
        zero_arg_function_type(ValueType::Function(Box::new(zero_arg_function_type(
            ValueType::Int,
        ))))
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
            FunctionExprKind::Float(return_) => {
                ReturnExpr::float_function(FloatFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Bool(return_) => {
                ReturnExpr::bool_function(BoolFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Nil(return_) => {
                ReturnExpr::nil_function(NilFunctionFunctionId(0), return_)
            }
            FunctionExprKind::Tuple(return_) => {
                ReturnExpr::tuple_function(TupleFunctionFunctionId(0), return_)
            }
            FunctionExprKind::List(return_) => ReturnExpr::list_function(
                crate::plan::ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                    ),
                    crate::plan::ValueType::Int,
                ),
                return_,
            ),
            FunctionExprKind::Function(return_) => {
                ReturnExpr::function_function(FunctionFunctionFunctionId(0), return_)
            }
        }
    }

    fn function_function_expr(runtime_id: FunctionFunctionId) -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            runtime_id,
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ))
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

    fn float_function_expr() -> FunctionExpr {
        FunctionExpr::float(FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
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

    fn tuple_function_expr() -> FunctionExpr {
        FunctionExpr::tuple(TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            Vec::new(),
            vec![ValueType::Int],
        )))
    }

    fn list_function_expr() -> FunctionExpr {
        FunctionExpr::list(ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            Vec::new(),
        )))
    }

    fn function_function_expr_value() -> FunctionExpr {
        FunctionExpr::function(function_function_expr(FunctionFunctionId::Int(
            IntFunctionFunctionId(0),
        )))
    }
}
