use crate::plan::{
    AssertBinding, AssertPattern, BoolFunctionLocalId, BoolFunctionValue, BoolLocalId,
    ExecutionPlan, FloatFunctionLocalId, FloatFunctionValue, FloatLocalId, FunctionFunctionLocalId,
    FunctionFunctionValue, FunctionValue, FunctionValueKind, IntFunctionLocalId, IntFunctionValue,
    IntLocalId, ListAssertPattern, ListAssertTail, ListFunctionLocalId, ListFunctionValue,
    ListLocalId, ListValue, NilFunctionLocalId, NilFunctionValue, NilLocalId, ParamLocal, StepKind,
    StringFunctionLocalId, StringFunctionValue, StringLocalId, TupleFunctionLocalId,
    TupleFunctionValue, TupleLocalId, Value, ValueType,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_expr, eval_float_expr, eval_float_function_expr,
    eval_function_function_expr, eval_int_expr, eval_int_function_expr, eval_list_expr,
    eval_list_function_expr, eval_nil_expr, eval_nil_function_expr, eval_string_expr,
    eval_string_function_expr, eval_tuple_expr, eval_tuple_function_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::{ExecutionError, PanicKind};
use ecow::EcoString;
use num_bigint::BigInt;

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
            StepKind::LetList { local, value, .. } => {
                let value = eval_list_expr(plan, frame, value)?;
                frame.set_list(*local, value);
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
            StepKind::LetListFunction { local, value, .. } => {
                let value = eval_list_function_expr(plan, frame, value)?;
                frame.set_list_function(*local, value);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                let value = eval_function_function_expr(plan, frame, value)?;
                frame.set_function_function(*local, value);
            }
            StepKind::AssertList {
                local,
                pattern,
                message,
                site,
                pattern_span,
            } => {
                let value = frame.get_list(*local);
                let mut bindings = Vec::new();
                if match_assert_pattern(pattern, &Value::List(value.clone()), &mut bindings)
                    .is_none()
                {
                    let message = match message {
                        Some(message) => Some(eval_string_expr(plan, frame, message)?),
                        None => None,
                    };
                    return Err(ExecutionError::let_assert_panic(
                        plan.source_context(),
                        message,
                        site.clone(),
                        Value::List(value),
                        *pattern_span,
                    ));
                }
                for binding in bindings {
                    frame_set_binding(frame, binding);
                }
            }
            StepKind::AssertBool {
                condition,
                message,
                site,
            } => {
                let message = match message {
                    Some(message) => Some(eval_string_expr(plan, frame, message)?),
                    None => None,
                };
                if !eval_bool_expr(plan, frame, condition)? {
                    return Err(ExecutionError::source_panic(
                        plan.source_context(),
                        PanicKind::Assert,
                        message,
                        site.clone(),
                    ));
                }
            }
            StepKind::Evaluate(expression) => {
                let _ = eval_expr(plan, frame, expression)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum PendingBinding {
    Int(IntLocalId, BigInt),
    Float(FloatLocalId, f64),
    String(StringLocalId, EcoString),
    Bool(BoolLocalId, bool),
    Nil(NilLocalId),
    Tuple(TupleLocalId, Vec<Value>),
    List(ListLocalId, ListValue),
    IntFunction(IntFunctionLocalId, IntFunctionValue),
    FloatFunction(FloatFunctionLocalId, FloatFunctionValue),
    StringFunction(StringFunctionLocalId, StringFunctionValue),
    BoolFunction(BoolFunctionLocalId, BoolFunctionValue),
    NilFunction(NilFunctionLocalId, NilFunctionValue),
    TupleFunction(TupleFunctionLocalId, TupleFunctionValue),
    ListFunction(ListFunctionLocalId, ListFunctionValue),
    FunctionFunction(FunctionFunctionLocalId, FunctionFunctionValue),
}

fn match_list_assert_pattern(
    pattern: &ListAssertPattern,
    value: &ListValue,
) -> Option<Vec<PendingBinding>> {
    if let Some(tail) = pattern.tail() {
        if value.values().len() < pattern.elements().len() {
            return None;
        }

        let mut bindings = match_prefix_assert_patterns(pattern.elements(), value.values())?;
        if let ListAssertTail::Bind(binding) = tail {
            let tail_values = value.values()[pattern.elements().len()..].to_vec();
            bindings.push(PendingBinding::List(
                binding.local(),
                ListValue::new(pattern.element_type().clone(), tail_values),
            ));
        }
        Some(bindings)
    } else {
        if value.values().len() != pattern.elements().len() {
            return None;
        }

        match_prefix_assert_patterns(pattern.elements(), value.values())
    }
}

fn match_prefix_assert_patterns(
    patterns: &[AssertPattern],
    values: &[Value],
) -> Option<Vec<PendingBinding>> {
    let mut bindings = Vec::new();
    for (pattern, value) in patterns.iter().zip(values) {
        match_assert_pattern(pattern, value, &mut bindings)?;
    }
    Some(bindings)
}

fn match_assert_pattern(
    pattern: &AssertPattern,
    value: &Value,
    bindings: &mut Vec<PendingBinding>,
) -> Option<()> {
    match pattern {
        AssertPattern::Bind(binding) => {
            bindings.push(pending_binding(binding, value)?);
            Some(())
        }
        AssertPattern::Discard => Some(()),
        AssertPattern::Tuple(patterns) => {
            let Value::Tuple(values) = value else {
                return None;
            };
            if patterns.len() != values.len() {
                return None;
            }
            for (pattern, value) in patterns.iter().zip(values) {
                match_assert_pattern(pattern, value, bindings)?;
            }
            Some(())
        }
        AssertPattern::List(pattern) => {
            let Value::List(value) = value else {
                return None;
            };
            bindings.extend(match_list_assert_pattern(pattern, value)?);
            Some(())
        }
        AssertPattern::Alias { pattern, binding } => {
            match_assert_pattern(pattern, value, bindings)?;
            bindings.push(pending_binding(binding, value)?);
            Some(())
        }
    }
}

fn pending_binding(target: &AssertBinding, value: &Value) -> Option<PendingBinding> {
    match (target.local(), value) {
        (ParamLocal::Int(local), Value::Int(value)) => {
            Some(PendingBinding::Int(*local, value.clone()))
        }
        (ParamLocal::Float(local), Value::Float(value)) => {
            Some(PendingBinding::Float(*local, *value))
        }
        (ParamLocal::String(local), Value::String(value)) => {
            Some(PendingBinding::String(*local, value.clone()))
        }
        (ParamLocal::Bool(local), Value::Bool(value)) => Some(PendingBinding::Bool(*local, *value)),
        (ParamLocal::Nil(local), Value::Nil) => Some(PendingBinding::Nil(*local)),
        (ParamLocal::Tuple { local, .. }, Value::Tuple(value))
            if target.local().value_type()
                == ValueType::Tuple(value.iter().map(Value::value_type).collect()) =>
        {
            Some(PendingBinding::Tuple(*local, value.clone()))
        }
        (ParamLocal::List { local, .. }, Value::List(value))
            if target.local().value_type()
                == ValueType::List(Box::new(value.element_type().clone())) =>
        {
            Some(PendingBinding::List(*local, value.clone()))
        }
        (_, Value::Function(value)) => pending_function_binding(target.local(), value),
        _ => None,
    }
}

fn pending_function_binding(target: &ParamLocal, value: &FunctionValue) -> Option<PendingBinding> {
    if target.value_type() != ValueType::Function(Box::new(value.type_())) {
        return None;
    }

    match (target, value.kind()) {
        (ParamLocal::IntFunction { local, .. }, FunctionValueKind::Int(value)) => {
            Some(PendingBinding::IntFunction(*local, value.clone()))
        }
        (ParamLocal::FloatFunction { local, .. }, FunctionValueKind::Float(value)) => {
            Some(PendingBinding::FloatFunction(*local, value.clone()))
        }
        (ParamLocal::StringFunction { local, .. }, FunctionValueKind::String(value)) => {
            Some(PendingBinding::StringFunction(*local, value.clone()))
        }
        (ParamLocal::BoolFunction { local, .. }, FunctionValueKind::Bool(value)) => {
            Some(PendingBinding::BoolFunction(*local, value.clone()))
        }
        (ParamLocal::NilFunction { local, .. }, FunctionValueKind::Nil(value)) => {
            Some(PendingBinding::NilFunction(*local, value.clone()))
        }
        (ParamLocal::TupleFunction { local, .. }, FunctionValueKind::Tuple(value)) => {
            Some(PendingBinding::TupleFunction(*local, value.clone()))
        }
        (ParamLocal::ListFunction { local, .. }, FunctionValueKind::List(value)) => {
            Some(PendingBinding::ListFunction(*local, value.clone()))
        }
        (ParamLocal::FunctionFunction { local, .. }, FunctionValueKind::Function(value)) => {
            Some(PendingBinding::FunctionFunction(*local, value.clone()))
        }
        _ => None,
    }
}

fn frame_set_binding(frame: &mut Frame, binding: PendingBinding) {
    match binding {
        PendingBinding::Int(local, value) => frame.set_int(local, value),
        PendingBinding::Float(local, value) => frame.set_float(local, value),
        PendingBinding::String(local, value) => frame.set_string(local, value),
        PendingBinding::Bool(local, value) => frame.set_bool(local, value),
        PendingBinding::Nil(local) => frame.set_nil(local),
        PendingBinding::Tuple(local, value) => frame.set_tuple(local, value),
        PendingBinding::List(local, value) => frame.set_list(local, value),
        PendingBinding::IntFunction(local, value) => frame.set_int_function(local, value),
        PendingBinding::FloatFunction(local, value) => frame.set_float_function(local, value),
        PendingBinding::StringFunction(local, value) => frame.set_string_function(local, value),
        PendingBinding::BoolFunction(local, value) => frame.set_bool_function(local, value),
        PendingBinding::NilFunction(local, value) => frame.set_nil_function(local, value),
        PendingBinding::TupleFunction(local, value) => frame.set_tuple_function(local, value),
        PendingBinding::ListFunction(local, value) => frame.set_list_function(local, value),
        PendingBinding::FunctionFunction(local, value) => {
            frame.set_function_function(local, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PendingBinding, execute_steps, frame_set_binding, match_assert_pattern,
        match_list_assert_pattern, pending_binding,
    };
    use crate::plan::{
        AssertBinding, AssertPattern, BoolExpr, BoolFunctionExpr, BoolFunctionId,
        BoolFunctionLocalId, BoolFunctionValue, BoolLocalId, ExecutionPlan, Expr, FloatExpr,
        FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue, FloatLocalId,
        FrameLayout, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId,
        FunctionFunctionValue, FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType,
        IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, ListAssertPattern, ListAssertTail, ListExpr,
        ListFunctionExpr, ListFunctionId, ListFunctionLocalId, ListFunctionValue, ListLocalId,
        ListValue, NilExpr, NilFunctionExpr, NilFunctionId, NilFunctionLocalId, NilFunctionValue,
        NilLocalId, PanicExpr, PanicSite, ParamLocal, ReturnExpr, SourceSpan, Step, StringExpr,
        StringFunctionExpr, StringFunctionId, StringFunctionLocalId, StringFunctionValue,
        StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionId, TupleFunctionLocalId,
        TupleFunctionValue, TupleLocalId, Value, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};

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
            Step::let_list(ListLocalId(0), "x".into(), failing_list_expr()),
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
            Step::let_list_function(
                ListFunctionLocalId(0),
                "x".into(),
                failing_list_function_expr(),
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
            Step::let_list(ListLocalId(0), "x".into(), list_expr()),
            Step::let_int_function(IntFunctionLocalId(0), "x".into(), int_function_expr()),
            Step::let_string_function(StringFunctionLocalId(0), "x".into(), string_function_expr()),
            Step::let_float_function(FloatFunctionLocalId(0), "x".into(), float_function_expr()),
            Step::let_bool_function(BoolFunctionLocalId(0), "x".into(), bool_function_expr()),
            Step::let_nil_function(NilFunctionLocalId(0), "x".into(), nil_function_expr()),
            Step::let_tuple_function(TupleFunctionLocalId(0), "x".into(), tuple_function_expr()),
            Step::let_list_function(ListFunctionLocalId(0), "x".into(), list_function_expr()),
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
            frame.get_list(ListLocalId(0)),
            ListValue::new(ValueType::Int, vec![Value::Int(1.into())]),
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
    fn execute_steps_assert_list_binds_elements_and_tail_after_match() {
        let plan = plan();
        let mut frame = Frame::new(assert_list_layout());
        frame.set_list(
            ListLocalId(0),
            ListValue::new(
                ValueType::Int,
                vec![Value::Int(1.into()), Value::Int(2.into())],
            ),
        );

        execute_steps(
            &plan,
            &[Step::assert_list(ListLocalId(0), list_pattern(), None)],
            &mut frame,
        )
        .expect("assert should match");

        assert_eq!(frame.get_int(IntLocalId(0)), 1.into());
        assert_eq!(
            frame.get_list(ListLocalId(1)),
            ListValue::new(ValueType::Int, vec![Value::Int(2.into())]),
        );
    }

    #[test]
    fn execute_steps_assert_bool_continues_after_true_condition() {
        let plan = plan();

        assert_eq!(
            execute_steps(
                &plan,
                &[Step::assert_bool(
                    BoolExpr::value(true),
                    Some(StringExpr::value("ok".into())),
                )],
                &mut Frame::default(),
            ),
            Ok(()),
        );
    }

    #[test]
    fn execute_steps_assert_bool_returns_assert_panic_after_false_condition() {
        let plan = plan();

        assert_eq!(
            execute_steps(
                &plan,
                &[Step::assert_bool(BoolExpr::value(false), None)],
                &mut Frame::default(),
            ),
            Err(ExecutionError::panic(PanicKind::Assert)),
        );
        assert_eq!(
            execute_steps(
                &plan,
                &[Step::assert_bool(
                    BoolExpr::value(false),
                    Some(StringExpr::value("nope".into())),
                )],
                &mut Frame::default(),
            ),
            Err(ExecutionError::source_panic(
                None,
                PanicKind::Assert,
                Some("nope".into()),
                PanicSite::unknown(),
            )),
        );
    }

    #[test]
    fn execute_steps_assert_bool_evaluates_message_before_condition() {
        let plan = plan();

        assert_eq!(
            execute_steps(
                &plan,
                &[Step::assert_bool(
                    BoolExpr::panic(PanicExpr::todo(None)),
                    Some(StringExpr::panic(PanicExpr::panic(None))),
                )],
                &mut Frame::default(),
            ),
            Err(ExecutionError::panic(PanicKind::Panic)),
        );
    }

    #[test]
    fn execute_steps_assert_bool_propagates_message_error_before_condition() {
        let plan = plan();

        assert_expected_function_got_int(execute_steps(
            &plan,
            &[Step::assert_bool(
                failing_bool_expr(),
                Some(failing_string_expr()),
            )],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn execute_steps_assert_bool_propagates_condition_error_after_message() {
        let plan = plan();

        assert_expected_function_got_int(execute_steps(
            &plan,
            &[Step::assert_bool(
                failing_bool_expr(),
                Some(StringExpr::value("checked".into())),
            )],
            &mut Frame::default(),
        ));
    }

    #[test]
    fn execute_steps_assert_list_does_not_evaluate_message_after_match() {
        let plan = plan();
        let mut frame = Frame::new(assert_list_layout());
        frame.set_list(
            ListLocalId(0),
            ListValue::new(
                ValueType::Int,
                vec![Value::Int(1.into()), Value::Int(2.into())],
            ),
        );

        execute_steps(
            &plan,
            &[Step::assert_list(
                ListLocalId(0),
                list_pattern(),
                Some(failing_string_expr()),
            )],
            &mut frame,
        )
        .expect("matching assert should not evaluate message");

        assert_eq!(frame.get_int(IntLocalId(0)), 1.into());
    }

    #[test]
    fn execute_steps_assert_list_propagates_message_error_before_let_assert_panic() {
        let plan = plan();
        let mut frame = Frame::new(assert_list_layout());
        frame.set_list(ListLocalId(0), ListValue::new(ValueType::Int, Vec::new()));

        assert_expected_function_got_int(execute_steps(
            &plan,
            &[Step::assert_list(
                ListLocalId(0),
                list_pattern(),
                Some(failing_string_expr()),
            )],
            &mut frame,
        ));
        assert_eq!(frame.get_int(IntLocalId(0)), 0.into());
    }

    #[test]
    fn execute_steps_assert_list_returns_let_assert_panic_without_message() {
        let plan = plan();
        let mut frame = Frame::new(assert_list_layout());
        frame.set_list(ListLocalId(0), ListValue::new(ValueType::Int, Vec::new()));

        let actual = execute_steps(
            &plan,
            &[Step::assert_list(ListLocalId(0), list_pattern(), None)],
            &mut frame,
        );

        assert_eq!(
            actual,
            Err(ExecutionError::let_assert_panic(
                None,
                None,
                PanicSite::unknown(),
                Value::List(ListValue::new(ValueType::Int, Vec::new())),
                SourceSpan::new(0, 0),
            )),
        );
        assert_eq!(frame.get_int(IntLocalId(0)), 0.into());
    }

    #[test]
    fn execute_steps_assert_list_does_not_commit_partial_nested_bindings() {
        let plan = plan();
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocalId(0));
        layout.include_int(IntLocalId(0));
        layout.include_int(IntLocalId(1));
        let mut frame = Frame::new(layout);
        frame.set_list(
            ListLocalId(0),
            ListValue::new(
                ValueType::List(Box::new(ValueType::Int)),
                vec![
                    Value::List(ListValue::new(ValueType::Int, vec![Value::Int(1.into())])),
                    Value::List(ListValue::new(ValueType::Int, Vec::new())),
                ],
            ),
        );

        let actual = execute_steps(
            &plan,
            &[Step::assert_list(
                ListLocalId(0),
                AssertPattern::list(ListAssertPattern::new(
                    ValueType::List(Box::new(ValueType::Int)),
                    vec![
                        AssertPattern::list(ListAssertPattern::new(
                            ValueType::Int,
                            vec![AssertPattern::Bind(AssertBinding::new(
                                ParamLocal::int(IntLocalId(0)),
                                "first".into(),
                            ))],
                            None,
                        )),
                        AssertPattern::list(ListAssertPattern::new(
                            ValueType::Int,
                            vec![AssertPattern::Bind(AssertBinding::new(
                                ParamLocal::int(IntLocalId(1)),
                                "second".into(),
                            ))],
                            None,
                        )),
                    ],
                    None,
                )),
                None,
            )],
            &mut frame,
        );

        assert_eq!(
            actual,
            Err(ExecutionError::let_assert_panic(
                None,
                None,
                PanicSite::unknown(),
                Value::List(ListValue::new(
                    ValueType::List(Box::new(ValueType::Int)),
                    vec![
                        Value::List(ListValue::new(ValueType::Int, vec![Value::Int(1.into())])),
                        Value::List(ListValue::new(ValueType::Int, Vec::new())),
                    ],
                )),
                SourceSpan::new(0, 0),
            )),
        );
        assert_eq!(frame.get_int(IntLocalId(0)), 0.into());
        assert_eq!(frame.get_int(IntLocalId(1)), 0.into());
    }

    #[test]
    fn execute_steps_assert_list_accepts_empty_pattern_without_bindings() {
        let plan = plan();
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocalId(0));
        let mut frame = Frame::new(layout);
        frame.set_list(ListLocalId(0), ListValue::new(ValueType::Int, Vec::new()));

        execute_steps(
            &plan,
            &[Step::assert_list(
                ListLocalId(0),
                AssertPattern::list(ListAssertPattern::new(ValueType::Int, Vec::new(), None)),
                None,
            )],
            &mut frame,
        )
        .expect("empty assert list pattern should match an empty list");

        assert_eq!(
            frame.get_list(ListLocalId(0)),
            ListValue::new(ValueType::Int, Vec::new()),
        );
    }

    #[test]
    fn match_assert_pattern_rejects_wrong_value_shapes_without_bindings() {
        let int_binding = AssertBinding::new(ParamLocal::int(IntLocalId(0)), "int".into());
        let string_binding =
            AssertBinding::new(ParamLocal::string(StringLocalId(0)), "string".into());
        let list_pattern = AssertPattern::list(ListAssertPattern::new(
            ValueType::Int,
            vec![AssertPattern::Bind(int_binding.clone())],
            None,
        ));
        let alias_pattern = AssertPattern::alias(AssertPattern::Discard, int_binding.clone());

        let cases = [
            (
                AssertPattern::Bind(int_binding),
                Value::String("wrong".into()),
            ),
            (
                AssertPattern::list(ListAssertPattern::new(
                    ValueType::Int,
                    vec![AssertPattern::Bind(AssertBinding::new(
                        ParamLocal::int(IntLocalId(0)),
                        "int".into(),
                    ))],
                    Some(ListAssertTail::Ignore),
                )),
                Value::List(ListValue::new(
                    ValueType::Int,
                    vec![Value::String("wrong".into())],
                )),
            ),
            (
                AssertPattern::Tuple(vec![AssertPattern::Bind(string_binding.clone())]),
                Value::Int(1.into()),
            ),
            (
                AssertPattern::Tuple(vec![
                    AssertPattern::Bind(string_binding),
                    AssertPattern::Discard,
                ]),
                Value::Tuple(vec![Value::String("only one".into())]),
            ),
            (
                AssertPattern::Tuple(vec![AssertPattern::Bind(AssertBinding::new(
                    ParamLocal::int(IntLocalId(0)),
                    "int".into(),
                ))]),
                Value::Tuple(vec![Value::String("wrong".into())]),
            ),
            (list_pattern, Value::Int(1.into())),
            (
                AssertPattern::alias(
                    AssertPattern::Bind(AssertBinding::new(
                        ParamLocal::int(IntLocalId(0)),
                        "int".into(),
                    )),
                    AssertBinding::new(ParamLocal::int(IntLocalId(1)), "alias".into()),
                ),
                Value::String("wrong".into()),
            ),
            (alias_pattern, Value::String("wrong".into())),
        ];

        for (pattern, value) in cases {
            let mut bindings = Vec::new();

            assert_eq!(match_assert_pattern(&pattern, &value, &mut bindings), None);
            assert_eq!(bindings.len(), 0);
        }
    }

    #[test]
    fn match_assert_pattern_collects_nested_bindings_after_full_match() {
        let int_binding = AssertBinding::new(ParamLocal::int(IntLocalId(0)), "int".into());
        let tuple_alias = AssertBinding::new(
            ParamLocal::tuple(
                TupleLocalId(0),
                vec![ValueType::Int, ValueType::List(Box::new(ValueType::Int))],
            ),
            "tuple".into(),
        );
        let pattern = AssertPattern::alias(
            AssertPattern::Tuple(vec![
                AssertPattern::Bind(int_binding),
                AssertPattern::list(ListAssertPattern::new(
                    ValueType::Int,
                    vec![AssertPattern::Discard],
                    Some(ListAssertTail::bind(ListLocalId(0), "tail".into())),
                )),
            ]),
            tuple_alias,
        );
        let tuple_value = vec![
            Value::Int(1.into()),
            Value::List(ListValue::new(
                ValueType::Int,
                vec![Value::Int(2.into()), Value::Int(3.into())],
            )),
        ];
        let value = Value::Tuple(tuple_value.clone());
        let mut bindings = Vec::new();

        assert_eq!(
            match_assert_pattern(&pattern, &value, &mut bindings),
            Some(())
        );

        assert_eq!(bindings.len(), 3);
        assert_eq!(bindings[0], PendingBinding::Int(IntLocalId(0), 1.into()),);
        assert_eq!(
            bindings[1],
            PendingBinding::List(
                ListLocalId(0),
                ListValue::new(ValueType::Int, vec![Value::Int(3.into())]),
            ),
        );
        assert_eq!(
            bindings[2],
            PendingBinding::Tuple(TupleLocalId(0), tuple_value)
        );
    }

    #[test]
    fn match_list_assert_pattern_handles_tail_bind_and_ignore() {
        let first_binding = AssertBinding::new(ParamLocal::int(IntLocalId(0)), "first".into());
        let value = ListValue::new(
            ValueType::Int,
            vec![
                Value::Int(1.into()),
                Value::Int(2.into()),
                Value::Int(3.into()),
            ],
        );
        let bind_tail = ListAssertPattern::new(
            ValueType::Int,
            vec![AssertPattern::Bind(first_binding.clone())],
            Some(ListAssertTail::bind(ListLocalId(0), "tail".into())),
        );
        let ignore_tail = ListAssertPattern::new(
            ValueType::Int,
            vec![AssertPattern::Bind(first_binding)],
            Some(ListAssertTail::Ignore),
        );

        let bound = match_list_assert_pattern(&bind_tail, &value).expect("tail bind should match");

        assert_eq!(bound.len(), 2);
        assert_eq!(bound[0], PendingBinding::Int(IntLocalId(0), 1.into()));
        assert_eq!(
            bound[1],
            PendingBinding::List(
                ListLocalId(0),
                ListValue::new(
                    ValueType::Int,
                    vec![Value::Int(2.into()), Value::Int(3.into())],
                ),
            ),
        );

        let ignored =
            match_list_assert_pattern(&ignore_tail, &value).expect("tail ignore should match");

        assert_eq!(ignored.len(), 1);
        assert_eq!(ignored[0], PendingBinding::Int(IntLocalId(0), 1.into()));
    }

    #[test]
    fn frame_set_binding_writes_all_value_families() {
        let mut frame = Frame::new(all_family_layout());
        let tuple_value = vec![Value::Int(1.into())];
        let list_value = ListValue::new(ValueType::Int, vec![Value::Int(2.into())]);
        let int_function = IntFunctionValue::new(IntFunctionId(0), Vec::new());
        let string_function = StringFunctionValue::new(StringFunctionId(0), Vec::new());
        let float_function = FloatFunctionValue::new(FloatFunctionId(0), Vec::new());
        let bool_function = BoolFunctionValue::new(BoolFunctionId(0), Vec::new());
        let nil_function = NilFunctionValue::new(NilFunctionId(0), Vec::new());
        let tuple_function =
            TupleFunctionValue::new(TupleFunctionId(0), Vec::new(), vec![ValueType::Int]);
        let list_function = ListFunctionValue::new(ListFunctionId(0), Vec::new(), ValueType::Int);
        let function_function = FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        );
        let int_binding = AssertBinding::new(ParamLocal::int(IntLocalId(0)), "int".into());
        let string_binding =
            AssertBinding::new(ParamLocal::string(StringLocalId(0)), "string".into());
        let float_binding = AssertBinding::new(ParamLocal::float(FloatLocalId(0)), "float".into());
        let bool_binding = AssertBinding::new(ParamLocal::bool(BoolLocalId(0)), "bool".into());
        let nil_binding = AssertBinding::new(ParamLocal::nil(NilLocalId(0)), "nil".into());
        let tuple_binding = AssertBinding::new(
            ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int]),
            "tuple".into(),
        );
        let list_binding = AssertBinding::new(
            ParamLocal::list(ListLocalId(0), ValueType::Int),
            "list".into(),
        );
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let string_function_type = FunctionType::new(Vec::new(), ValueType::String);
        let float_function_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_function_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_function_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let tuple_function_type =
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_function_type =
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        let int_function_binding = AssertBinding::new(
            ParamLocal::int_function(IntFunctionLocalId(0), int_function_type),
            "int_function".into(),
        );
        let string_function_binding = AssertBinding::new(
            ParamLocal::string_function(StringFunctionLocalId(0), string_function_type),
            "string_function".into(),
        );
        let float_function_binding = AssertBinding::new(
            ParamLocal::float_function(FloatFunctionLocalId(0), float_function_type),
            "float_function".into(),
        );
        let bool_function_binding = AssertBinding::new(
            ParamLocal::bool_function(BoolFunctionLocalId(0), bool_function_type),
            "bool_function".into(),
        );
        let nil_function_binding = AssertBinding::new(
            ParamLocal::nil_function(NilFunctionLocalId(0), nil_function_type),
            "nil_function".into(),
        );
        let tuple_function_binding = AssertBinding::new(
            ParamLocal::tuple_function(TupleFunctionLocalId(0), tuple_function_type),
            "tuple_function".into(),
        );
        let list_function_binding = AssertBinding::new(
            ParamLocal::list_function(ListFunctionLocalId(0), list_function_type),
            "list_function".into(),
        );
        let function_function_binding = AssertBinding::new(
            ParamLocal::function_function(FunctionFunctionLocalId(0), function_function_type),
            "function_function".into(),
        );

        assert_eq!(
            pending_binding(&int_binding, &Value::String("wrong".into())),
            None,
        );
        assert_eq!(
            pending_binding(
                &int_function_binding,
                &Value::Function(string_function.clone().into()),
            ),
            None,
        );
        assert_eq!(
            pending_binding(
                &AssertBinding::new(
                    ParamLocal::int_function(
                        IntFunctionLocalId(1),
                        FunctionType::new(Vec::new(), ValueType::String),
                    ),
                    "malformed_int_function".into(),
                ),
                &Value::Function(string_function.clone().into()),
            ),
            None,
        );

        let bindings = [
            pending_binding(&int_binding, &Value::Int(11.into())),
            pending_binding(&string_binding, &Value::String("value".into())),
            pending_binding(&float_binding, &Value::Float(1.5)),
            pending_binding(&bool_binding, &Value::Bool(true)),
            pending_binding(&nil_binding, &Value::Nil),
            pending_binding(&tuple_binding, &Value::Tuple(tuple_value.clone())),
            pending_binding(&list_binding, &Value::List(list_value.clone())),
            pending_binding(
                &int_function_binding,
                &Value::Function(int_function.clone().into()),
            ),
            pending_binding(
                &string_function_binding,
                &Value::Function(string_function.clone().into()),
            ),
            pending_binding(
                &float_function_binding,
                &Value::Function(float_function.clone().into()),
            ),
            pending_binding(
                &bool_function_binding,
                &Value::Function(bool_function.clone().into()),
            ),
            pending_binding(
                &nil_function_binding,
                &Value::Function(nil_function.clone().into()),
            ),
            pending_binding(
                &tuple_function_binding,
                &Value::Function(tuple_function.clone().into()),
            ),
            pending_binding(
                &list_function_binding,
                &Value::Function(list_function.clone().into()),
            ),
            pending_binding(
                &function_function_binding,
                &Value::Function(function_function.clone().into()),
            ),
        ];
        for binding in bindings {
            frame_set_binding(
                &mut frame,
                binding.expect("pending binding should match target family"),
            );
        }

        assert_eq!(frame.get_int(IntLocalId(0)), 11.into());
        assert_eq!(frame.get_string(StringLocalId(0)), "value");
        assert_eq!(frame.get_float(FloatLocalId(0)), 1.5);
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_tuple(TupleLocalId(0)), tuple_value);
        assert_eq!(frame.get_list(ListLocalId(0)), list_value);
        assert_eq!(frame.get_int_function(IntFunctionLocalId(0)), int_function);
        assert_eq!(
            frame.get_string_function(StringFunctionLocalId(0)),
            string_function,
        );
        assert_eq!(
            frame.get_float_function(FloatFunctionLocalId(0)),
            float_function,
        );
        assert_eq!(
            frame.get_bool_function(BoolFunctionLocalId(0)),
            bool_function
        );
        assert_eq!(frame.get_nil_function(NilFunctionLocalId(0)), nil_function);
        assert_eq!(
            frame.get_tuple_function(TupleFunctionLocalId(0)),
            tuple_function,
        );
        assert_eq!(
            frame.get_list_function(ListFunctionLocalId(0)),
            list_function,
        );
        assert_eq!(
            frame.get_function_function(FunctionFunctionLocalId(0)),
            function_function,
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

    fn failing_list_expr() -> ListExpr {
        ListExpr::function_call(failing_list_function_expr(), Vec::new(), ValueType::Int)
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
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

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId(0),
            Vec::new(),
            ValueType::Int,
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

    fn failing_list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::function_call(
            failing_function_function_expr(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
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
        layout.include_list(ListLocalId(0));
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

    fn assert_list_layout() -> FrameLayout {
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocalId(0));
        layout.include_int(IntLocalId(0));
        layout.include_list(ListLocalId(1));
        layout
    }

    fn list_pattern() -> AssertPattern {
        AssertPattern::list(ListAssertPattern::new(
            ValueType::Int,
            vec![AssertPattern::Bind(AssertBinding::new(
                ParamLocal::int(IntLocalId(0)),
                "first".into(),
            ))],
            Some(ListAssertTail::bind(ListLocalId(1), "rest".into())),
        ))
    }
}
