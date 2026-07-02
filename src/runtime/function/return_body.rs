use super::bind::bind_arguments;
use super::steps::execute_steps;
use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, ExecutionPlan, FunctionFunctionFunctionId,
    FunctionFunctionValue, IntFunctionFunctionId, IntFunctionId, NilFunctionFunctionId,
    NilFunctionId, ReturnBody, ReturnBodyKind, StringFunctionFunctionId, StringFunctionId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_function_function_expr, eval_int_expr,
    eval_int_function_expr, eval_nil_expr, eval_nil_function_expr, eval_string_expr,
    eval_string_function_expr,
};
use crate::runtime::frame::Frame;
use ecow::EcoString;
use num_bigint::BigInt;

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
    Function: Copy,
{
    match body.kind() {
        ReturnBodyKind::Expr(expression) => {
            eval_expression(plan, frame, expression).map(ReturnOutcome::Value)
        }
        ReturnBodyKind::TailCall { function, args } => Ok(ReturnOutcome::TailCall {
            function: *function,
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
        run_bool_function_loop, run_bool_loop, run_function_function_loop, run_int_function_loop,
        run_int_loop, run_nil_function_loop, run_nil_loop, run_string_function_loop,
        run_string_loop,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionValue,
        ExecutionPlan, Expr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionValue, FunctionId,
        FunctionPlan, FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionValue, NilExpr, NilFunctionExpr,
        NilFunctionFunctionId, NilFunctionId, NilFunctionValue, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionValue,
        ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn primitive_return_loops_propagate_step_errors() {
        let plan = primitive_function_plan_with_steps(vec![failing_step()]);

        assert_expected_function_got_int(run_int_loop(&plan, IntFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_string_loop(
            &plan,
            StringFunctionId(0),
            Frame::default(),
        ));
        assert_expected_function_got_int(run_bool_loop(&plan, BoolFunctionId(0), Frame::default()));
        assert_expected_function_got_int(run_nil_loop(&plan, NilFunctionId(0), Frame::default()));
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
}
