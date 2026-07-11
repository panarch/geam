use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    AssertBinding, AssertPattern, BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId,
    FloatLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId, ListAssertPattern,
    ListAssertTail, ListFunctionLocal, NilFunctionLocalId, NilLocalId, ParamLocal, StepKind,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_bool_list_expr, eval_expr, eval_float_expr,
    eval_float_function_expr, eval_float_list_expr, eval_function_function_expr,
    eval_function_list_expr, eval_int_expr, eval_int_function_expr, eval_int_list_expr,
    eval_list_function_expr, eval_list_list_expr, eval_nil_expr, eval_nil_function_expr,
    eval_nil_list_expr, eval_string_expr, eval_string_function_expr, eval_string_list_expr,
    eval_tuple_expr, eval_tuple_function_expr, eval_tuple_list_expr, get_list_value,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::{ListValueId, RuntimeState};
use crate::runtime::{
    EvaluatedBoolFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedIntFunction, EvaluatedListCapture,
    EvaluatedListFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedValue,
};
use crate::runtime::{ExecutionError, PanicKind};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) fn execute_steps(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    steps: &[crate::plan::execution::Step],
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for step in steps {
        match step.kind() {
            StepKind::LetInt { local, value, .. } => {
                let value = eval_int_expr(plan, state, frame, value)?;
                frame.set_int(*local, value);
            }
            StepKind::LetString { local, value, .. } => {
                let value = eval_string_expr(plan, state, frame, value)?;
                frame.set_string(*local, value);
            }
            StepKind::LetFloat { local, value, .. } => {
                let value = eval_float_expr(plan, state, frame, value)?;
                frame.set_float(*local, value);
            }
            StepKind::LetBool { local, value, .. } => {
                let value = eval_bool_expr(plan, state, frame, value)?;
                frame.set_bool(*local, value);
            }
            StepKind::LetNil { local, value, .. } => {
                eval_nil_expr(plan, state, frame, value)?;
                frame.set_nil(*local);
            }
            StepKind::LetTuple { local, value, .. } => {
                let value = eval_tuple_expr(plan, state, frame, value)?;
                frame.set_tuple(*local, value);
            }
            StepKind::LetList { value, .. } => execute_let_list(plan, state, frame, value)?,
            StepKind::LetIntFunction { local, value, .. } => {
                let value = eval_int_function_expr(plan, state, frame, value)?;
                frame.set_int_function(*local, value);
            }
            StepKind::LetStringFunction { local, value, .. } => {
                let value = eval_string_function_expr(plan, state, frame, value)?;
                frame.set_string_function(*local, value);
            }
            StepKind::LetFloatFunction { local, value, .. } => {
                let value = eval_float_function_expr(plan, state, frame, value)?;
                frame.set_float_function(*local, value);
            }
            StepKind::LetBoolFunction { local, value, .. } => {
                let value = eval_bool_function_expr(plan, state, frame, value)?;
                frame.set_bool_function(*local, value);
            }
            StepKind::LetNilFunction { local, value, .. } => {
                let value = eval_nil_function_expr(plan, state, frame, value)?;
                frame.set_nil_function(*local, value);
            }
            StepKind::LetTupleFunction { local, value, .. } => {
                let value = eval_tuple_function_expr(plan, state, frame, value)?;
                frame.set_tuple_function(*local, value);
            }
            StepKind::LetListFunction { local, value, .. } => {
                let value = eval_list_function_expr(plan, state, frame, value)?;
                frame.set_list_function(local.clone(), value);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                let value = eval_function_function_expr(plan, state, frame, value)?;
                frame.set_function_function(*local, value);
            }
            StepKind::AssertList {
                local,
                pattern,
                message,
                site,
                pattern_span,
            } => {
                let value = get_list_value(frame, local);
                let mut bindings = Vec::new();
                if match_assert_pattern(
                    plan,
                    state,
                    pattern,
                    &EvaluatedValue::List(value.clone()),
                    &mut bindings,
                )
                .is_none()
                {
                    let message = match message {
                        Some(message) => Some(eval_string_expr(plan, state, frame, message)?),
                        None => None,
                    };
                    return Err(ExecutionError::let_assert_panic(
                        plan.source_context(),
                        message,
                        site.clone(),
                        crate::runtime::materialize::value(
                            plan,
                            state,
                            EvaluatedValue::List(value),
                        ),
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
                    Some(message) => Some(eval_string_expr(plan, state, frame, message)?),
                    None => None,
                };
                if !eval_bool_expr(plan, state, frame, condition)? {
                    return Err(ExecutionError::source_panic(
                        plan.source_context(),
                        PanicKind::Assert,
                        message,
                        site.clone(),
                    ));
                }
            }
            StepKind::Evaluate(expression) => {
                let _ = eval_expr(plan, state, frame, expression)?;
            }
        }
    }

    state.drain_releases();
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum PendingBinding {
    Int(IntLocalId, BigInt),
    Float(FloatLocalId, f64),
    String(StringLocalId, EcoString),
    Bool(BoolLocalId, bool),
    Nil(NilLocalId),
    Tuple(TupleLocalId, Vec<EvaluatedValue>),
    List(EvaluatedListCapture),
    IntFunction(IntFunctionLocalId, EvaluatedIntFunction),
    FloatFunction(FloatFunctionLocalId, EvaluatedFloatFunction),
    StringFunction(StringFunctionLocalId, EvaluatedStringFunction),
    BoolFunction(BoolFunctionLocalId, EvaluatedBoolFunction),
    NilFunction(NilFunctionLocalId, EvaluatedNilFunction),
    TupleFunction(TupleFunctionLocalId, EvaluatedTupleFunction),
    ListFunction(ListFunctionLocal, EvaluatedListFunction),
    FunctionFunction(FunctionFunctionLocalId, EvaluatedFunctionFunction),
}

fn match_list_assert_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    pattern: &ListAssertPattern,
    value: &ListValueId,
) -> Option<Vec<PendingBinding>> {
    let values = state.evaluated_values(plan, value);
    if let Some(tail) = pattern.tail() {
        if values.len() < pattern.elements().len() {
            return None;
        }

        let mut bindings = match_prefix_assert_patterns(plan, state, pattern.elements(), &values)?;
        if let ListAssertTail::Bind(binding) = tail {
            bindings.push(PendingBinding::List(pending_list_binding(
                binding.local().clone(),
                state.drop_first(value, pattern.elements().len()),
            )?));
        }
        Some(bindings)
    } else {
        if values.len() != pattern.elements().len() {
            return None;
        }

        match_prefix_assert_patterns(plan, state, pattern.elements(), &values)
    }
}

fn match_prefix_assert_patterns(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    patterns: &[AssertPattern],
    values: &[EvaluatedValue],
) -> Option<Vec<PendingBinding>> {
    let mut bindings = Vec::new();
    for (pattern, value) in patterns.iter().zip(values) {
        match_assert_pattern(plan, state, pattern, value, &mut bindings)?;
    }
    Some(bindings)
}

fn match_assert_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    pattern: &AssertPattern,
    value: &EvaluatedValue,
    bindings: &mut Vec<PendingBinding>,
) -> Option<()> {
    match pattern {
        AssertPattern::Bind(binding) => {
            bindings.push(pending_binding(plan, binding, value)?);
            Some(())
        }
        AssertPattern::Discard => Some(()),
        AssertPattern::Tuple(patterns) => {
            let EvaluatedValue::Tuple(values) = value else {
                return None;
            };
            if patterns.len() != values.len() {
                return None;
            }
            for (pattern, value) in patterns.iter().zip(values) {
                match_assert_pattern(plan, state, pattern, value, bindings)?;
            }
            Some(())
        }
        AssertPattern::List(pattern) => {
            let EvaluatedValue::List(value) = value else {
                return None;
            };
            bindings.extend(match_list_assert_pattern(plan, state, pattern, value)?);
            Some(())
        }
        AssertPattern::Alias { pattern, binding } => {
            match_assert_pattern(plan, state, pattern, value, bindings)?;
            bindings.push(pending_binding(plan, binding, value)?);
            Some(())
        }
    }
}

fn pending_binding(
    plan: &ExecutionPlan,
    target: &AssertBinding,
    value: &EvaluatedValue,
) -> Option<PendingBinding> {
    match (target.local(), value) {
        (ParamLocal::Int(local), EvaluatedValue::Int(value)) => {
            Some(PendingBinding::Int(*local, value.clone()))
        }
        (ParamLocal::Float(local), EvaluatedValue::Float(value)) => {
            Some(PendingBinding::Float(*local, *value))
        }
        (ParamLocal::String(local), EvaluatedValue::String(value)) => {
            Some(PendingBinding::String(*local, value.clone()))
        }
        (ParamLocal::Bool(local), EvaluatedValue::Bool(value)) => {
            Some(PendingBinding::Bool(*local, *value))
        }
        (ParamLocal::Nil(local), EvaluatedValue::Nil) => Some(PendingBinding::Nil(*local)),
        (ParamLocal::Tuple { local, .. }, EvaluatedValue::Tuple(value))
            if plan.value_type(&target.local().value_type())
                == ValueType::Tuple(value.iter().map(|value| value.value_type(plan)).collect()) =>
        {
            Some(PendingBinding::Tuple(*local, value.clone()))
        }
        (ParamLocal::List(local), EvaluatedValue::List(value)) => {
            pending_list_binding(local.clone(), value.clone()).map(PendingBinding::List)
        }
        (_, EvaluatedValue::Function(value)) => {
            pending_function_binding(plan, target.local(), value)
        }
        _ => None,
    }
}

fn pending_function_binding(
    plan: &ExecutionPlan,
    target: &ParamLocal,
    value: &EvaluatedFunctionValue,
) -> Option<PendingBinding> {
    if plan.value_type(&target.value_type())
        != ValueType::Function(Box::new(plan.function_type(value.type_())))
    {
        return None;
    }

    match (target, value.kind()) {
        (ParamLocal::IntFunction { local, .. }, EvaluatedFunctionValueKind::Int(value)) => {
            Some(PendingBinding::IntFunction(*local, value.clone()))
        }
        (ParamLocal::FloatFunction { local, .. }, EvaluatedFunctionValueKind::Float(value)) => {
            Some(PendingBinding::FloatFunction(*local, value.clone()))
        }
        (ParamLocal::StringFunction { local, .. }, EvaluatedFunctionValueKind::String(value)) => {
            Some(PendingBinding::StringFunction(*local, value.clone()))
        }
        (ParamLocal::BoolFunction { local, .. }, EvaluatedFunctionValueKind::Bool(value)) => {
            Some(PendingBinding::BoolFunction(*local, value.clone()))
        }
        (ParamLocal::NilFunction { local, .. }, EvaluatedFunctionValueKind::Nil(value)) => {
            Some(PendingBinding::NilFunction(*local, value.clone()))
        }
        (ParamLocal::TupleFunction { local, .. }, EvaluatedFunctionValueKind::Tuple(value)) => {
            Some(PendingBinding::TupleFunction(*local, value.clone()))
        }
        (ParamLocal::ListFunction(local), EvaluatedFunctionValueKind::List(value)) => {
            Some(PendingBinding::ListFunction(local.clone(), value.clone()))
        }
        (
            ParamLocal::FunctionFunction { local, .. },
            EvaluatedFunctionValueKind::Function(value),
        ) => Some(PendingBinding::FunctionFunction(*local, value.clone())),
        _ => None,
    }
}

fn pending_list_binding(
    local: crate::plan::execution::ListLocal,
    value: ListValueId,
) -> Option<EvaluatedListCapture> {
    match (local, value) {
        (crate::plan::execution::ListLocal::Int { local, .. }, ListValueId::Int(value)) => {
            Some(EvaluatedListCapture::Int { local, value })
        }
        (crate::plan::execution::ListLocal::String { local, .. }, ListValueId::String(value)) => {
            Some(EvaluatedListCapture::String { local, value })
        }
        (crate::plan::execution::ListLocal::Float { local, .. }, ListValueId::Float(value)) => {
            Some(EvaluatedListCapture::Float { local, value })
        }
        (crate::plan::execution::ListLocal::Bool { local, .. }, ListValueId::Bool(value)) => {
            Some(EvaluatedListCapture::Bool { local, value })
        }
        (crate::plan::execution::ListLocal::Nil { local, .. }, ListValueId::Nil(value)) => {
            Some(EvaluatedListCapture::Nil { local, value })
        }
        (crate::plan::execution::ListLocal::Tuple { local, .. }, ListValueId::Tuple(value)) => {
            Some(EvaluatedListCapture::Tuple { local, value })
        }
        (crate::plan::execution::ListLocal::List { local, .. }, ListValueId::List(value)) => {
            Some(EvaluatedListCapture::List { local, value })
        }
        (
            crate::plan::execution::ListLocal::Function { local, .. },
            ListValueId::Function(value),
        ) => Some(EvaluatedListCapture::Function { local, value }),
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
        PendingBinding::List(value) => frame_set_list_binding(frame, value),
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

fn execute_let_list(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<()> {
    match value {
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            let value = eval_int_list_expr(plan, state, frame, value)?;
            frame.set_int_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            let value = eval_string_list_expr(plan, state, frame, value)?;
            frame.set_string_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            let value = eval_float_list_expr(plan, state, frame, value)?;
            frame.set_float_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            let value = eval_bool_list_expr(plan, state, frame, value)?;
            frame.set_bool_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            let value = eval_nil_list_expr(plan, state, frame, value)?;
            frame.set_nil_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Tuple { local, value, .. } => {
            let value = eval_tuple_list_expr(plan, state, frame, value)?;
            frame.set_tuple_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::List { local, value, .. } => {
            let value = eval_list_list_expr(plan, state, frame, value)?;
            frame.set_list_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Function { local, value, .. } => {
            let value = eval_function_list_expr(plan, state, frame, value)?;
            frame.set_function_list(*local, value);
        }
    }
    Ok(())
}

fn frame_set_list_binding(frame: &mut Frame, value: EvaluatedListCapture) {
    match value {
        EvaluatedListCapture::Int { local, value } => frame.set_int_list(local, value),
        EvaluatedListCapture::String { local, value } => frame.set_string_list(local, value),
        EvaluatedListCapture::Float { local, value } => frame.set_float_list(local, value),
        EvaluatedListCapture::Bool { local, value } => frame.set_bool_list(local, value),
        EvaluatedListCapture::Nil { local, value } => frame.set_nil_list(local, value),
        EvaluatedListCapture::Tuple { local, value } => frame.set_tuple_list(local, value),
        EvaluatedListCapture::List { local, value } => frame.set_list_list(local, value),
        EvaluatedListCapture::Function { local, value } => frame.set_function_list(local, value),
    }
}

#[cfg(test)]
mod tests {
    use super::{PendingBinding, match_assert_pattern, match_list_assert_pattern};
    use crate::plan::execution::{
        AssertPattern, FunctionFunctionId, IntFunctionFunctionId, IntFunctionId, IntListLocalId,
        IntLocalId, ListAssertPattern, StepKind,
    };
    use crate::runtime::state::{IntListValueId, ListValueId};
    use crate::runtime::{
        EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedListCapture, EvaluatedValue,
        ListValue,
    };

    #[test]
    fn source_steps_bind_and_assert_exact_values() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution/values/list_expression_item_families.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_return_shapes.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!("../../../tests/fixtures/execution/bindings/expression_steps.gleam"),
                crate::runtime::Value::Int(5.into()),
            ),
            (
                include_str!("../../../tests/fixtures/execution/statements/assert_statement.gleam"),
                crate::runtime::Value::Int(1.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_list_destructuring.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_fixed_list.gleam"
                ),
                crate::runtime::Value::Int(3.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_empty_list.gleam"
                ),
                crate::runtime::Value::List(ListValue::int(Vec::new())),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/nested_pattern_alias_assignment.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_discard_alias.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src(source), expected);
        }
    }

    #[test]
    fn let_assert_binds_every_function_return_family() {
        let function_shapes = [
            ("Int", "1"),
            ("String", "\"one\""),
            ("Float", "1.0"),
            ("Bool", "True"),
            ("Nil", "Nil"),
            ("#(Int)", "#(1)"),
            ("List(Int)", "[1]"),
            ("fn() -> Int", "fn() { 1 }"),
        ];

        for (return_type, return_value) in function_shapes {
            let source = format!(
                r#"
fn target() -> {return_type} {{ {return_value} }}
pub fn main() {{
  let assert [function] = [target]
  let _ = function()
  42
}}
"#,
            );

            assert_eq!(
                crate::runtime::run_src(&source),
                crate::runtime::Value::Int(42.into()),
            );
        }
    }

    #[test]
    fn source_assert_steps_return_default_and_explicit_panics() {
        let cases = [
            (
                "pub fn main() { let values: List(Int) = [] let assert [first] = values first }",
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                "pub fn main() { let values: List(Int) = [] let assert [first] = values as \"missing\" first }",
                "let_assert: missing",
            ),
            (
                "pub fn main() { assert False Nil }",
                "assert: Assertion failed.",
            ),
            (
                "pub fn main() { assert False as \"checked\" Nil }",
                "assert: checked",
            ),
            (
                "fn fail_message() -> String { panic as \"message\" } pub fn main() { assert True as fail_message() Nil }",
                "panic: message",
            ),
            (
                "fn fail_condition() -> Bool { panic as \"condition\" } pub fn main() { assert fail_condition() as \"checked\" Nil }",
                "panic: condition",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src_error(source).to_string(), expected);
        }
    }

    #[test]
    fn source_let_errors_propagate_for_every_value_family() {
        let value_types = [
            "Int",
            "String",
            "Float",
            "Bool",
            "Nil",
            "#(Int)",
            "List(Int)",
            "List(String)",
            "List(Float)",
            "List(Bool)",
            "List(Nil)",
            "List(#(Int))",
            "List(List(Int))",
            "List(fn() -> Int)",
            "fn() -> Int",
            "fn() -> String",
            "fn() -> Float",
            "fn() -> Bool",
            "fn() -> Nil",
            "fn() -> #(Int)",
            "fn() -> List(Int)",
            "fn() -> fn() -> Int",
        ];

        for value_type in value_types {
            let source = format!(
                "pub fn main() {{ let value: {value_type} = panic as \"step\" let _ = value Nil }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: step",
            );
        }
    }

    #[test]
    fn let_assert_message_errors_propagate_after_mismatch() {
        let source = r#"
fn fail_message() -> String { panic as "message" }
pub fn main() {
  let values: List(Int) = []
  let assert [first, ..] = values as fail_message()
  first
}
"#;

        assert_eq!(
            crate::runtime::run_src_error(source).to_string(),
            "panic: message",
        );
    }

    #[test]
    fn source_bound_tail_prefix_mismatch_returns_the_let_assert_error() {
        assert_eq!(
            crate::runtime::run_src_error(include_str!(
                "../../../tests/fixtures/execution_errors/patterns/let_assert_bound_tail_prefix.gleam"
            ))
            .to_string(),
            "let_assert: Pattern match failed, no pattern matched the value.",
        );
    }

    #[test]
    fn let_assert_matcher_rejects_direct_mutated_value_shapes_without_bindings() {
        let mut state = crate::runtime::RuntimeState::new();
        let tuple_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = tuple_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &tuple_plan,
                &mut state,
                tuple_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());
        assert_eq!(
            match_assert_pattern(
                &tuple_plan,
                &mut state,
                tuple_pattern,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let list_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [[value]] = [[1]]
  value
}
"#,
        );
        let function = list_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let nested_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        assert_eq!(
            match_assert_pattern(
                &list_plan,
                &mut state,
                nested_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let binding_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [value] = [1]
  value
}
"#,
        );
        let function = binding_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let binding = &expect_list_assert_pattern(pattern).elements()[0];
        assert_eq!(
            match_assert_pattern(
                &binding_plan,
                &mut state,
                binding,
                &EvaluatedValue::String("wrong".into()),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn list_assert_tail_binding_preserves_the_typed_local_and_value() {
        let mut state = crate::runtime::RuntimeState::new();
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [first, ..rest] = [1, 2, 3]
  rest
}
"#,
        );
        let function = plan.int_list_function(plan.int_list_function_id(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let pattern = expect_list_assert_pattern(pattern);

        let ignored_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [first, ..] = [1, 2, 3]
  first
}
"#,
        );
        let function = ignored_plan.int_function(IntFunctionId(0));
        let ignored_tail = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let ignored_tail = expect_list_assert_pattern(ignored_tail);
        let value = state.int(
            plan.int_list_function_id(0).type_id(),
            vec![1.into(), 2.into(), 3.into()],
        );

        let bindings = match_list_assert_pattern(&plan, &mut state, pattern, &value.clone().into())
            .expect("list pattern should match");
        assert_eq!(bindings[0], PendingBinding::Int(IntLocalId(0), 1.into()));
        assert_eq!(int_list_binding(&bindings[0]), None);
        let (local, tail) = int_list_binding(&bindings[1]).expect("tail must bind List(Int)");
        assert_eq!(local, IntListLocalId(1));
        assert_eq!(state.int_values(tail), &[2.into(), 3.into()]);
        assert_eq!(
            match_list_assert_pattern(&ignored_plan, &mut state, ignored_tail, &value.into(),),
            Some(vec![PendingBinding::Int(IntLocalId(0), 1.into())]),
        );
    }

    #[test]
    fn nested_and_alias_assert_patterns_propagate_binding_mismatches() {
        let mut state = crate::runtime::RuntimeState::new();
        let nested_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = nested_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &nested_plan,
                &mut state,
                tuple_pattern,
                &EvaluatedValue::Tuple(vec![
                    EvaluatedValue::Int(1.into()),
                    EvaluatedValue::String("wrong".into())
                ]),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings.len(), 1);

        let alias_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right) as pair] = [#(1, 2)]
  pair.0 + left + right
}
"#,
        );
        let function = alias_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let alias_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        bindings.clear();
        assert_eq!(
            match_assert_pattern(
                &alias_plan,
                &mut state,
                alias_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let function_alias_plan = crate::runtime::plan_src(
            r#"
fn target() { 1 }
pub fn main() {
  let assert [_ as function] = [target]
  function()
}
"#,
        );
        let function = function_alias_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let alias_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let wrong_kind = EvaluatedFunctionValue::from(EvaluatedFunctionFunction::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        ));
        assert_eq!(
            match_assert_pattern(
                &function_alias_plan,
                &mut state,
                alias_pattern,
                &EvaluatedValue::Function(wrong_kind),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn list_assert_binding_rejects_direct_mutated_list_and_function_metadata() {
        let mut state = crate::runtime::RuntimeState::new();
        let plan = crate::runtime::plan_src(
            r#"
fn strings() -> List(String) { [] }
fn target() { 1 }
pub fn main() {
  let assert [..rest] = [1]
  let assert [values] = [[1]]
  let assert [function] = [target]
  #(rest, values, function())
}
"#,
        );
        let function = plan.tuple_function(crate::plan::execution::TupleFunctionId(0));
        let patterns = function
            .steps()
            .iter()
            .filter_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(expect_list_assert_pattern(pattern)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(patterns.len(), 3);
        let wrong_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["wrong".into()],
        );

        assert_eq!(
            match_list_assert_pattern(
                &plan,
                &mut state,
                patterns[0],
                &ListValueId::String(wrong_list.clone()),
            ),
            None,
        );

        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &patterns[1].elements()[0],
                &EvaluatedValue::List(ListValueId::String(wrong_list)),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let wrong_function =
            EvaluatedFunctionValue::from(crate::runtime::EvaluatedIntFunction::new(
                IntFunctionId(0),
                Vec::new(),
                Vec::new(),
                crate::plan::execution::FunctionType::new(
                    Vec::new(),
                    crate::plan::execution::ValueType::String,
                ),
            ));
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &patterns[2].elements()[0],
                &EvaluatedValue::Function(wrong_function),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    #[should_panic(expected = "expected a list assert pattern")]
    fn list_assert_pattern_shape_guard_rejects_tuple_patterns() {
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];

        let _ = expect_list_assert_pattern(tuple_pattern);
    }

    fn expect_list_assert_pattern(pattern: &AssertPattern) -> &ListAssertPattern {
        match pattern {
            AssertPattern::List(pattern) => pattern,
            _ => panic!("expected a list assert pattern"),
        }
    }

    fn int_list_binding(binding: &PendingBinding) -> Option<(IntListLocalId, &IntListValueId)> {
        match binding {
            PendingBinding::List(EvaluatedListCapture::Int { local, value }) => {
                Some((*local, value))
            }
            _ => None,
        }
    }
}
