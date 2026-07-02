use crate::plan::{ExecutionPlan, StepKind};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_nil_expr,
    eval_nil_function_expr, eval_string_expr, eval_string_function_expr, eval_tuple_expr,
    eval_tuple_function_expr,
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
            StepKind::LetTuple { local, value, .. } => {
                let value = eval_tuple_expr(plan, frame, value)?;
                frame.set_tuple(*local, value);
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
            StepKind::LetTupleFunction { local, value, .. } => {
                let value = eval_tuple_function_expr(plan, frame, value)?;
                frame.set_tuple_function(*local, value);
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
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
        BoolLocalId, ExecutionPlan, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionId,
        FloatFunctionLocalId, FloatFunctionValue, FloatLocalId, FrameLayout, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionValue, FunctionId,
        FunctionPlan, FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId,
        NilExpr, NilFunctionExpr, NilFunctionId, NilFunctionLocalId, NilFunctionValue, NilLocalId,
        ReturnExpr, Step, StringExpr, StringFunctionExpr, StringFunctionId, StringFunctionLocalId,
        StringFunctionValue, StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionId,
        TupleFunctionLocalId, TupleFunctionValue, TupleLocalId, Value, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn execute_steps_propagates_let_value_evaluation_errors() {
        let plan = plan();

        let steps = [
            Step::let_int(IntLocalId(0), "x".into(), failing_int_expr()),
            Step::let_string(StringLocalId(0), "x".into(), failing_string_expr()),
            Step::let_float(FloatLocalId(0), "x".into(), failing_float_expr()),
            Step::let_bool(BoolLocalId(0), "x".into(), failing_bool_expr()),
            Step::let_nil(NilLocalId(0), "x".into(), failing_nil_expr()),
            Step::let_tuple(TupleLocalId(0), "x".into(), failing_tuple_expr()),
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
            Step::let_float_function(
                FloatFunctionLocalId(0),
                "x".into(),
                failing_float_function_expr(),
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
            Step::let_tuple_function(
                TupleFunctionLocalId(0),
                "x".into(),
                failing_tuple_function_expr(),
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
    fn execute_steps_evaluates_and_binds_all_let_families() {
        let plan = plan();
        let mut frame = Frame::new(all_family_layout());
        let steps = [
            Step::let_int(IntLocalId(0), "x".into(), IntExpr::value(1.into())),
            Step::let_string(
                StringLocalId(0),
                "x".into(),
                StringExpr::value("one".into()),
            ),
            Step::let_float(FloatLocalId(0), "x".into(), FloatExpr::value(1.5)),
            Step::let_bool(BoolLocalId(0), "x".into(), BoolExpr::value(true)),
            Step::let_nil(NilLocalId(0), "x".into(), NilExpr::value()),
            Step::let_tuple(TupleLocalId(0), "x".into(), tuple_expr()),
            Step::let_int_function(IntFunctionLocalId(0), "x".into(), int_function_expr()),
            Step::let_string_function(StringFunctionLocalId(0), "x".into(), string_function_expr()),
            Step::let_float_function(FloatFunctionLocalId(0), "x".into(), float_function_expr()),
            Step::let_bool_function(BoolFunctionLocalId(0), "x".into(), bool_function_expr()),
            Step::let_nil_function(NilFunctionLocalId(0), "x".into(), nil_function_expr()),
            Step::let_tuple_function(TupleFunctionLocalId(0), "x".into(), tuple_function_expr()),
            Step::let_function_function(
                FunctionFunctionLocalId(0),
                "x".into(),
                function_function_expr(),
            ),
        ];

        execute_steps(&plan, &steps, &mut frame).expect("steps should execute");

        assert_eq!(frame.get_int(IntLocalId(0)), 1.into());
        assert_eq!(frame.get_string(StringLocalId(0)), "one");
        assert_eq!(frame.get_float(FloatLocalId(0)), 1.5);
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_tuple(TupleLocalId(0)), vec![Value::Int(1.into())]);
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

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        )
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(IntFunctionId(0), Vec::new()))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(StringFunctionId(0), Vec::new()))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(FloatFunctionId(0), Vec::new()))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new()))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new()))
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            Vec::new(),
            vec![ValueType::Int],
        ))
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ))
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

    fn failing_tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
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

    fn all_family_layout() -> FrameLayout {
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        layout.include_string(StringLocalId(0));
        layout.include_float(FloatLocalId(0));
        layout.include_bool(BoolLocalId(0));
        layout.include_nil(NilLocalId(0));
        layout.include_tuple(TupleLocalId(0));
        layout.include_int_function(IntFunctionLocalId(0));
        layout.include_string_function(StringFunctionLocalId(0));
        layout.include_float_function(FloatFunctionLocalId(0));
        layout.include_bool_function(BoolFunctionLocalId(0));
        layout.include_nil_function(NilFunctionLocalId(0));
        layout.include_tuple_function(TupleFunctionLocalId(0));
        layout.include_function_function(FunctionFunctionLocalId(0));
        layout
    }
}
