use crate::plan::{ExecutionPlan, StepKind};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_nil_expr,
    eval_nil_function_expr, eval_string_expr, eval_string_function_expr,
};
use crate::runtime::frame::Frame;

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
            StepKind::LetFloat { local, value, .. } => {
                let value = eval_float_expr(plan, frame, value)?;
                frame.set_float(*local, value);
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
            StepKind::LetFloatFunction { local, value, .. } => {
                let value = eval_float_function_expr(plan, frame, value)?;
                frame.set_float_function(*local, value);
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

#[cfg(test)]
mod tests {
    use super::execute_steps;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolLocalId, ExecutionPlan,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionValue,
        FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionLocalId, IntLocalId, NilExpr, NilFunctionExpr,
        NilFunctionLocalId, NilLocalId, ReturnExpr, Step, StringExpr, StringFunctionExpr,
        StringFunctionLocalId, StringLocalId, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

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
