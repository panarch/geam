use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    AssertBinding, AssertPattern, BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolLocalId, CustomBindingPattern, CustomConstructorId, CustomFunctionLocalId, CustomLocalId,
    FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId,
    ListAssertPattern, ListAssertTail, ListFunctionLocal, NilFunctionLocalId, NilLocalId,
    ParamLocal, StepKind, StringFunctionLocalId, StringLocalId, TotalBindingPattern,
    TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bit_array_expr, eval_bit_array_function_expr, eval_bit_array_list_expr, eval_bool_expr,
    eval_bool_function_expr, eval_bool_list_expr, eval_custom_expr, eval_custom_function_expr,
    eval_custom_list_expr, eval_expr, eval_float_expr, eval_float_function_expr,
    eval_float_list_expr, eval_function_function_expr, eval_function_list_expr, eval_int_expr,
    eval_int_function_expr, eval_int_list_expr, eval_list_function_expr, eval_list_list_expr,
    eval_nil_expr, eval_nil_function_expr, eval_nil_list_expr, eval_string_expr,
    eval_string_function_expr, eval_string_list_expr, eval_tuple_expr, eval_tuple_function_expr,
    eval_tuple_list_expr, eval_utf_codepoint_expr, eval_utf_codepoint_function_expr,
    eval_utf_codepoint_list_expr, get_list_value,
};
use crate::runtime::frame::Frame;
use crate::runtime::state::{ListValueId, RuntimeState};
use crate::runtime::{
    BitArrayValue, EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
    Value,
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
            StepKind::LetBitArray { local, value, .. } => {
                let value = eval_bit_array_expr(plan, state, frame, value)?;
                frame.set_bit_array(*local, value);
            }
            StepKind::LetUtfCodepoint { local, value, .. } => {
                let value = eval_utf_codepoint_expr(plan, state, frame, value)?;
                frame.set_utf_codepoint(*local, value);
            }
            StepKind::LetCustom { local, value, .. } => {
                let value = eval_custom_expr(plan, state, frame, value)?;
                frame.set_custom(*local, value);
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
            StepKind::LetBitArrayFunction { local, value, .. } => {
                let value = eval_bit_array_function_expr(plan, state, frame, value)?;
                frame.set_bit_array_function(*local, value);
            }
            StepKind::LetUtfCodepointFunction { local, value, .. } => {
                let value = eval_utf_codepoint_function_expr(plan, state, frame, value)?;
                frame.set_utf_codepoint_function(*local, value);
            }
            StepKind::LetCustomFunction { local, value, .. } => {
                let value = eval_custom_function_expr(plan, state, frame, value)?;
                frame.set_custom_function(*local, value);
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
                    frame,
                    pattern,
                    &EvaluatedValue::List(value.clone()),
                    &mut bindings,
                )?
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
                        )?,
                        *pattern_span,
                    ));
                }
                for binding in bindings {
                    frame_set_binding(frame, binding);
                }
            }
            StepKind::AssertBitArray {
                local,
                pattern,
                message,
                site,
                pattern_span,
            } => {
                let value = frame.get_bit_array(*local);
                let mut bindings = Vec::new();
                if match_bit_array_assert_pattern(frame, pattern, &value, &mut bindings).is_none() {
                    let message = match message {
                        Some(message) => Some(eval_string_expr(plan, state, frame, message)?),
                        None => None,
                    };
                    return Err(ExecutionError::let_assert_panic(
                        plan.source_context(),
                        message,
                        site.clone(),
                        Value::BitArray(BitArrayValue::from_evaluated(value.bits())),
                        *pattern_span,
                    ));
                }
                for binding in bindings {
                    frame_set_binding(frame, binding);
                }
            }
            StepKind::AssertCustom {
                local,
                pattern,
                message,
                site,
                pattern_span,
            } => {
                let value = frame.get_custom(*local);
                let mut bindings = Vec::new();
                if match_assert_pattern(
                    plan,
                    state,
                    frame,
                    pattern,
                    &EvaluatedValue::Custom(value.clone()),
                    &mut bindings,
                )?
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
                            EvaluatedValue::Custom(value),
                        )?,
                        *pattern_span,
                    ));
                }
                for binding in bindings {
                    frame_set_binding(frame, binding);
                }
            }
            StepKind::BindCustomFields { local, pattern } => {
                let value = frame.get_custom(*local);
                let bindings = bind_custom_fields(plan, pattern, &value)?;
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

fn match_bit_array_assert_pattern(
    frame: &mut Frame,
    pattern: &crate::plan::execution::BitArrayAssertPattern,
    value: &EvaluatedBitArray,
    bindings: &mut Vec<PendingBinding>,
) -> Option<()> {
    match pattern {
        crate::plan::execution::BitArrayAssertPattern::Pattern(pattern) => {
            crate::runtime::pattern::match_bit_array_pattern(frame, value, pattern).then_some(())
        }
        crate::plan::execution::BitArrayAssertPattern::Alias { pattern, local } => {
            match_bit_array_assert_pattern(frame, pattern, value, bindings)?;
            bindings.push(PendingBinding::BitArray(*local, value.clone()));
            Some(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PendingBinding {
    Int(IntLocalId, BigInt),
    Float(FloatLocalId, f64),
    String(StringLocalId, EcoString),
    BitArray(BitArrayLocalId, EvaluatedBitArray),
    UtfCodepoint(UtfCodepointLocalId, char),
    Custom(CustomLocalId, EvaluatedCustomValue),
    Bool(BoolLocalId, bool),
    Nil(NilLocalId),
    Tuple(TupleLocalId, Vec<EvaluatedValue>),
    List(EvaluatedListCapture),
    IntFunction(IntFunctionLocalId, EvaluatedIntFunction),
    FloatFunction(FloatFunctionLocalId, EvaluatedFloatFunction),
    StringFunction(StringFunctionLocalId, EvaluatedStringFunction),
    BitArrayFunction(BitArrayFunctionLocalId, EvaluatedBitArrayFunction),
    UtfCodepointFunction(UtfCodepointFunctionLocalId, EvaluatedUtfCodepointFunction),
    CustomFunction(CustomFunctionLocalId, EvaluatedCustomFunction),
    BoolFunction(BoolFunctionLocalId, EvaluatedBoolFunction),
    NilFunction(NilFunctionLocalId, EvaluatedNilFunction),
    TupleFunction(TupleFunctionLocalId, EvaluatedTupleFunction),
    ListFunction(ListFunctionLocal, EvaluatedListFunction),
    FunctionFunction(FunctionFunctionLocalId, EvaluatedFunctionFunction),
}

pub(in crate::runtime) fn match_and_apply_assert_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    pattern: &AssertPattern,
    value: &EvaluatedValue,
) -> ExecutionResult<bool> {
    let mut bindings = Vec::new();
    if match_assert_pattern(plan, state, frame, pattern, value, &mut bindings)?.is_none() {
        return Ok(false);
    }
    for binding in bindings {
        frame_set_binding(frame, binding);
    }
    Ok(true)
}

fn match_list_assert_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    pattern: &ListAssertPattern,
    value: &ListValueId,
) -> ExecutionResult<Option<Vec<PendingBinding>>> {
    let values = state.evaluated_values(plan, value);
    if let Some(tail) = pattern.tail() {
        if values.len() < pattern.elements().len() {
            return Ok(None);
        }

        let Some(mut bindings) =
            match_prefix_assert_patterns(plan, state, frame, pattern.elements(), &values)?
        else {
            return Ok(None);
        };
        if let ListAssertTail::Bind(binding) = tail {
            let Some(binding) = pending_list_binding(
                binding.local().clone(),
                state.drop_first(value, pattern.elements().len()),
            ) else {
                return Ok(None);
            };
            bindings.push(PendingBinding::List(binding));
        }
        Ok(Some(bindings))
    } else {
        if values.len() != pattern.elements().len() {
            return Ok(None);
        }

        match_prefix_assert_patterns(plan, state, frame, pattern.elements(), &values)
    }
}

fn match_prefix_assert_patterns(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    patterns: &[AssertPattern],
    values: &[EvaluatedValue],
) -> ExecutionResult<Option<Vec<PendingBinding>>> {
    let mut bindings = Vec::new();
    for (pattern, value) in patterns.iter().zip(values) {
        if match_assert_pattern(plan, state, frame, pattern, value, &mut bindings)?.is_none() {
            return Ok(None);
        }
    }
    Ok(Some(bindings))
}

fn match_assert_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    pattern: &AssertPattern,
    value: &EvaluatedValue,
    bindings: &mut Vec<PendingBinding>,
) -> ExecutionResult<Option<()>> {
    match pattern {
        AssertPattern::Bind(binding) => {
            let Some(binding) = pending_binding(plan, binding, value) else {
                return Ok(None);
            };
            bindings.push(binding);
            Ok(Some(()))
        }
        AssertPattern::Discard => Ok(Some(())),
        AssertPattern::Int(pattern) => match value {
            EvaluatedValue::Int(value) if value == pattern => Ok(Some(())),
            _ => Ok(None),
        },
        AssertPattern::Float(pattern) => match value {
            EvaluatedValue::Float(value) if value == pattern => Ok(Some(())),
            _ => Ok(None),
        },
        AssertPattern::String(pattern) => match value {
            EvaluatedValue::String(value) if value == pattern => Ok(Some(())),
            _ => Ok(None),
        },
        AssertPattern::Bool(pattern) => match value {
            EvaluatedValue::Bool(value) if value == pattern => Ok(Some(())),
            _ => Ok(None),
        },
        AssertPattern::Nil => match value {
            EvaluatedValue::Nil => Ok(Some(())),
            _ => Ok(None),
        },
        AssertPattern::Tuple(patterns) => {
            let EvaluatedValue::Tuple(values) = value else {
                return Ok(None);
            };
            if patterns.len() != values.len() {
                return Ok(None);
            }
            for (pattern, value) in patterns.iter().zip(values) {
                if match_assert_pattern(plan, state, frame, pattern, value, bindings)?.is_none() {
                    return Ok(None);
                }
            }
            Ok(Some(()))
        }
        AssertPattern::List(pattern) => {
            let EvaluatedValue::List(value) = value else {
                return Ok(None);
            };
            let Some(list_bindings) =
                match_list_assert_pattern(plan, state, frame, pattern, value)?
            else {
                return Ok(None);
            };
            bindings.extend(list_bindings);
            Ok(Some(()))
        }
        AssertPattern::BitArray(pattern) => {
            let EvaluatedValue::BitArray(value) = value else {
                return Ok(None);
            };
            if crate::runtime::pattern::match_bit_array_pattern(frame, value, pattern) {
                Ok(Some(()))
            } else {
                Ok(None)
            }
        }
        AssertPattern::Custom(pattern) => {
            let EvaluatedValue::Custom(value) = value else {
                return Ok(None);
            };
            let constructor_id = pattern.constructor();
            if value.constructor() != constructor_id {
                return Ok(None);
            }
            let constructor = plan.custom_constructor(constructor_id);
            ensure_custom_field_arity(plan, value)?;
            for (field_index, field_pattern) in pattern.fields().iter().enumerate() {
                let field = &constructor.fields()[field_index];
                let value = &value.fields()[field_index];
                ensure_custom_field_type(plan, constructor_id, field_index, field.type_(), value)?;
                if match_assert_pattern(plan, state, frame, field_pattern, value, bindings)?
                    .is_none()
                {
                    return Ok(None);
                }
            }
            Ok(Some(()))
        }
        AssertPattern::StringPrefix {
            prefix,
            left,
            right,
        } => {
            let EvaluatedValue::String(value) = value else {
                return Ok(None);
            };
            let Some(suffix) = value.strip_prefix(prefix.as_str()) else {
                return Ok(None);
            };
            if let Some(binding) = left {
                let Some(binding) =
                    pending_binding(plan, binding, &EvaluatedValue::String(prefix.clone()))
                else {
                    return Ok(None);
                };
                bindings.push(binding);
            }
            if let Some(binding) = right {
                let Some(binding) =
                    pending_binding(plan, binding, &EvaluatedValue::String(suffix.into()))
                else {
                    return Ok(None);
                };
                bindings.push(binding);
            }
            Ok(Some(()))
        }
        AssertPattern::Alias { pattern, binding } => {
            if match_assert_pattern(plan, state, frame, pattern, value, bindings)?.is_none() {
                return Ok(None);
            }
            let Some(binding) = pending_binding(plan, binding, value) else {
                return Ok(None);
            };
            bindings.push(binding);
            Ok(Some(()))
        }
    }
}

fn bind_custom_fields(
    plan: &ExecutionPlan,
    pattern: &CustomBindingPattern,
    value: &EvaluatedCustomValue,
) -> ExecutionResult<Vec<PendingBinding>> {
    let mut bindings = Vec::new();
    append_custom_field_bindings(plan, pattern, value, &mut bindings)?;
    Ok(bindings)
}

fn append_custom_field_bindings(
    plan: &ExecutionPlan,
    pattern: &CustomBindingPattern,
    value: &EvaluatedCustomValue,
    bindings: &mut Vec<PendingBinding>,
) -> ExecutionResult<()> {
    let constructor_id = pattern.constructor();
    if value.constructor() != constructor_id {
        let expected = plan.custom_constructor(constructor_id);
        let actual = plan.custom_constructor(value.constructor());
        return Err(ExecutionError::CustomFieldDiscriminantMismatch {
            expected_type: plan.custom_value_type(constructor_id.type_id()),
            expected_constructors: vec![expected.name().clone()],
            actual_type: plan.custom_value_type(value.type_id()),
            actual_constructor: actual.name().clone(),
        });
    }
    ensure_custom_field_arity(plan, value)?;
    for (field_index, pattern) in pattern.fields().iter().enumerate() {
        let value = &value.fields()[field_index];
        append_total_bindings(plan, constructor_id, field_index, value, pattern, bindings)?;
    }
    Ok(())
}

fn ensure_custom_field_arity(
    plan: &ExecutionPlan,
    value: &EvaluatedCustomValue,
) -> ExecutionResult<()> {
    let constructor = plan.custom_constructor(value.constructor());
    if constructor.fields().len() == value.fields().len() {
        Ok(())
    } else {
        Err(ExecutionError::CustomFieldArityMismatch {
            custom_type: plan.custom_value_type(value.type_id()),
            constructor: constructor.name().clone(),
            expected: constructor.fields().len(),
            actual: value.fields().len(),
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn append_total_bindings(
    plan: &ExecutionPlan,
    constructor: CustomConstructorId,
    field_index: usize,
    field_value: &EvaluatedValue,
    pattern: &TotalBindingPattern,
    bindings: &mut Vec<PendingBinding>,
) -> ExecutionResult<()> {
    match pattern.kind() {
        crate::plan::execution::TotalBindingPatternKind::Bind(binding) => {
            let Some(binding) = pending_binding(plan, binding, field_value) else {
                let descriptor = plan.custom_constructor(constructor);
                return Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index,
                    expected: plan.value_type(pattern.type_()),
                    actual: field_value.value_type(plan),
                });
            };
            bindings.push(binding);
        }
        crate::plan::execution::TotalBindingPatternKind::Discard => {}
        crate::plan::execution::TotalBindingPatternKind::Tuple(patterns) => {
            let EvaluatedValue::Tuple(values) = field_value else {
                let descriptor = plan.custom_constructor(constructor);
                return Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index,
                    expected: plan.value_type(pattern.type_()),
                    actual: field_value.value_type(plan),
                });
            };
            for (pattern, value) in patterns.iter().zip(values) {
                append_total_bindings(plan, constructor, field_index, value, pattern, bindings)?;
            }
        }
        crate::plan::execution::TotalBindingPatternKind::List(tail) => {
            if let ListAssertTail::Bind(binding) = tail {
                let EvaluatedValue::List(value) = field_value else {
                    let descriptor = plan.custom_constructor(constructor);
                    return Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index,
                        expected: plan.value_type(pattern.type_()),
                        actual: field_value.value_type(plan),
                    });
                };
                let Some(binding) = pending_list_binding(binding.local().clone(), value.clone())
                else {
                    let descriptor = plan.custom_constructor(constructor);
                    return Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index,
                        expected: plan.value_type(pattern.type_()),
                        actual: field_value.value_type(plan),
                    });
                };
                bindings.push(PendingBinding::List(binding));
            }
        }
        crate::plan::execution::TotalBindingPatternKind::Custom(custom_pattern) => {
            let EvaluatedValue::Custom(value) = field_value else {
                let descriptor = plan.custom_constructor(constructor);
                return Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index,
                    expected: plan.value_type(pattern.type_()),
                    actual: field_value.value_type(plan),
                });
            };
            append_custom_field_bindings(plan, custom_pattern, value, bindings)?;
        }
        crate::plan::execution::TotalBindingPatternKind::Alias {
            pattern: inner,
            binding,
        } => {
            append_total_bindings(plan, constructor, field_index, field_value, inner, bindings)?;
            let Some(binding) = pending_binding(plan, binding, field_value) else {
                let descriptor = plan.custom_constructor(constructor);
                return Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index,
                    expected: plan.value_type(pattern.type_()),
                    actual: field_value.value_type(plan),
                });
            };
            bindings.push(binding);
        }
    }
    Ok(())
}

fn ensure_custom_field_type(
    plan: &ExecutionPlan,
    constructor: CustomConstructorId,
    field_index: usize,
    expected: &crate::plan::execution::ValueType,
    value: &EvaluatedValue,
) -> ExecutionResult<()> {
    if plan.value_type(expected) == value.value_type(plan) {
        Ok(())
    } else {
        let descriptor = plan.custom_constructor(constructor);
        Err(ExecutionError::CustomFieldFamilyMismatch {
            custom_type: plan.custom_value_type(constructor.type_id()),
            constructor: descriptor.name().clone(),
            field_index,
            expected: plan.value_type(expected),
            actual: value.value_type(plan),
        })
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
        (ParamLocal::BitArray(local), EvaluatedValue::BitArray(value)) => {
            Some(PendingBinding::BitArray(*local, value.clone()))
        }
        (ParamLocal::UtfCodepoint(local), EvaluatedValue::UtfCodepoint(value)) => {
            Some(PendingBinding::UtfCodepoint(*local, *value))
        }
        (ParamLocal::Custom { local, type_id }, EvaluatedValue::Custom(value))
            if *type_id == value.type_id() =>
        {
            Some(PendingBinding::Custom(*local, value.clone()))
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
        (
            ParamLocal::BitArrayFunction { local, .. },
            EvaluatedFunctionValueKind::BitArray(value),
        ) => Some(PendingBinding::BitArrayFunction(*local, value.clone())),
        (
            ParamLocal::UtfCodepointFunction { local, .. },
            EvaluatedFunctionValueKind::UtfCodepoint(value),
        ) => Some(PendingBinding::UtfCodepointFunction(*local, value.clone())),
        (ParamLocal::CustomFunction { local, .. }, EvaluatedFunctionValueKind::Custom(value)) => {
            Some(PendingBinding::CustomFunction(*local, value.clone()))
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
        (
            crate::plan::execution::ListLocal::BitArray { local, .. },
            ListValueId::BitArray(value),
        ) => Some(EvaluatedListCapture::BitArray { local, value }),
        (
            crate::plan::execution::ListLocal::UtfCodepoint { local, .. },
            ListValueId::UtfCodepoint(value),
        ) => Some(EvaluatedListCapture::UtfCodepoint { local, value }),
        (crate::plan::execution::ListLocal::Custom { local, .. }, ListValueId::Custom(value)) => {
            Some(EvaluatedListCapture::Custom { local, value })
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
        PendingBinding::BitArray(local, value) => frame.set_bit_array(local, value),
        PendingBinding::UtfCodepoint(local, value) => frame.set_utf_codepoint(local, value),
        PendingBinding::Custom(local, value) => frame.set_custom(local, value),
        PendingBinding::Bool(local, value) => frame.set_bool(local, value),
        PendingBinding::Nil(local) => frame.set_nil(local),
        PendingBinding::Tuple(local, value) => frame.set_tuple(local, value),
        PendingBinding::List(value) => frame_set_list_binding(frame, value),
        PendingBinding::IntFunction(local, value) => frame.set_int_function(local, value),
        PendingBinding::FloatFunction(local, value) => frame.set_float_function(local, value),
        PendingBinding::StringFunction(local, value) => frame.set_string_function(local, value),
        PendingBinding::BitArrayFunction(local, value) => {
            frame.set_bit_array_function(local, value)
        }
        PendingBinding::UtfCodepointFunction(local, value) => {
            frame.set_utf_codepoint_function(local, value)
        }
        PendingBinding::CustomFunction(local, value) => frame.set_custom_function(local, value),
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
        crate::plan::execution::ListLocalExpr::BitArray { local, value } => {
            let value = eval_bit_array_list_expr(plan, state, frame, value)?;
            frame.set_bit_array_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::UtfCodepoint { local, value } => {
            let value = eval_utf_codepoint_list_expr(plan, state, frame, value)?;
            frame.set_utf_codepoint_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Custom { local, value } => {
            let value = eval_custom_list_expr(plan, state, frame, value)?;
            frame.set_custom_list(*local, value);
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
        EvaluatedListCapture::BitArray { local, value } => frame.set_bit_array_list(local, value),
        EvaluatedListCapture::UtfCodepoint { local, value } => {
            frame.set_utf_codepoint_list(local, value)
        }
        EvaluatedListCapture::Custom { local, value } => frame.set_custom_list(local, value),
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
    use super::{
        PendingBinding, bind_custom_fields, execute_steps, match_and_apply_assert_pattern,
        match_assert_pattern, match_list_assert_pattern,
    };
    use crate::plan::ValueType;
    use crate::plan::execution::{
        AssertBinding, AssertPattern, CustomListLocalId, CustomLocalId, FunctionFunctionId,
        IntFunctionFunctionId, IntFunctionId, IntListLocalId, IntLocalId, ListAssertPattern,
        ListLocal, ParamLocal, Step, StepKind, StringLocalId,
    };
    use crate::runtime::expression::eval_custom_expr;
    use crate::runtime::frame::Frame;
    use crate::runtime::state::{IntListValueId, ListValueId};
    use crate::runtime::{
        EvaluatedCustomValue, EvaluatedFunctionFunction, EvaluatedFunctionValue,
        EvaluatedListCapture, EvaluatedValue, ExecutionError, ListValue,
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
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_bit_array_patterns.gleam"
                ),
                crate::runtime::Value::Tuple(vec![
                    crate::runtime::Value::Int(16.into()),
                    crate::runtime::Value::BitArray(crate::BitArrayValue::from_bytes(vec![1, 2])),
                    crate::runtime::Value::BitArray(crate::BitArrayValue::from_bytes(vec![3])),
                    crate::runtime::Value::BitArray(crate::BitArrayValue::from_bytes(vec![4])),
                    crate::runtime::Value::BitArray(crate::BitArrayValue::from_bytes(vec![
                        16, 1, 2, 3, 4,
                    ])),
                    crate::runtime::Value::Int(1.into()),
                    crate::runtime::Value::BitArray(crate::BitArrayValue::from_bytes(vec![2])),
                ]),
            ),
            (
                include_str!("../../../tests/fixtures/execution/bindings/custom_let_assert.gleam"),
                crate::runtime::Value::Int(6.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/custom_total_binding.gleam"
                ),
                crate::runtime::Value::Int(28.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/custom_field_families.gleam"
                ),
                crate::runtime::Value::Int(22.into()),
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
            ("BitArray", "<<1>>"),
            (
                "UtfCodepoint",
                "{ let assert <<value:utf8_codepoint>> = <<65>> value }",
            ),
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
    fn let_assert_binds_utf_codepoint_list_elements_and_tails() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

pub fn main() {
  let assert [value, ..rest] = [codepoint(), codepoint()]
  #(value, rest)
}
"#,
            ),
            crate::runtime::Value::Tuple(vec![
                crate::runtime::Value::UtfCodepoint('A'),
                crate::runtime::Value::List(crate::runtime::ListValue::utf_codepoint(vec!['A'])),
            ]),
        );
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
                "pub fn main() { let assert <<1>> = <<2>> 0 }",
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                "pub fn main() { let assert [<<1>>] = [<<2>>] 0 }",
                "let_assert: Pattern match failed, no pattern matched the value.",
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
            (
                "pub type Choice { Empty Full(Int) } pub fn main() { let assert Full(value) = Empty value }",
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                "pub type Choice { Empty Full(Int) } pub fn main() { let assert Full(value) = Empty as \"expected full\" value }",
                "let_assert: expected full",
            ),
            (
                "pub fn main() { let assert <<1>> as whole = <<2>> whole }",
                "let_assert: Pattern match failed, no pattern matched the value.",
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
            "fn() -> List(BitArray)",
            "fn() -> List(UtfCodepoint)",
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

        for value_type in ["Boxed", "List(Boxed)", "fn() -> Boxed"] {
            let source = format!(
                "pub type Boxed {{ Boxed(Int) }} pub fn main() {{ let value: {value_type} = panic as \"step\" let _ = value Nil }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: step",
            );
        }
    }

    #[test]
    fn let_assert_message_errors_propagate_after_mismatch() {
        let list_source = r#"
fn fail_message() -> String { panic as "message" }
pub fn main() {
  let values: List(Int) = []
  let assert [first, ..] = values as fail_message()
  first
}
"#;

        assert_eq!(
            crate::runtime::run_src_error(list_source).to_string(),
            "panic: message",
        );
        assert_eq!(
            crate::runtime::run_src_error(
                r#"
fn fail_message() -> String { panic as "message" }
pub fn main() {
  let assert <<1>> = <<2>> as fail_message()
  1
}
"#,
            )
            .to_string(),
            "panic: message",
        );
        assert_eq!(
            crate::runtime::run_src_error(
                r#"
pub type Choice { Empty Full(Int) }
fn fail_message() -> String { panic as "message" }
pub fn main() {
  let assert Full(value) = Empty as fail_message()
  value
}
"#,
            )
            .to_string(),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &tuple_plan,
                &mut state,
                &mut frame,
                tuple_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings
            ),
            Ok(None),
        );
        assert_eq!(bindings, Vec::new());
        assert_eq!(
            match_assert_pattern(
                &tuple_plan,
                &mut state,
                &mut frame,
                tuple_pattern,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &mut bindings,
            ),
            Ok(None),
        );
        assert_eq!(bindings, Vec::new());

        let bit_array_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [<<1>>] = [<<1>>]
  1
}
"#,
        );
        let function = bit_array_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let bit_array_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            match_assert_pattern(
                &bit_array_plan,
                &mut state,
                &mut frame,
                bit_array_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            Ok(None),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            match_assert_pattern(
                &list_plan,
                &mut state,
                &mut frame,
                nested_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            Ok(None),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        assert_eq!(
            match_assert_pattern(
                &binding_plan,
                &mut state,
                &mut frame,
                binding,
                &EvaluatedValue::String("wrong".into()),
                &mut bindings,
            ),
            Ok(None),
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn literal_prefix_and_alias_assert_patterns_reject_mismatched_values_without_bindings() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let function = plan.int_function(IntFunctionId(0));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let cases = [
            (
                AssertPattern::Int(1.into()),
                EvaluatedValue::String("one".into()),
            ),
            (AssertPattern::Float(1.0), EvaluatedValue::Int(1.into())),
            (
                AssertPattern::String("one".into()),
                EvaluatedValue::Int(1.into()),
            ),
            (AssertPattern::Bool(true), EvaluatedValue::Int(1.into())),
            (AssertPattern::Nil, EvaluatedValue::Int(1.into())),
            (
                AssertPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: None,
                },
                EvaluatedValue::Int(1.into()),
            ),
            (
                AssertPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: None,
                },
                EvaluatedValue::String("suffix".into()),
            ),
            (
                AssertPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: Some(AssertBinding::new(ParamLocal::Int(IntLocalId(0)))),
                    right: None,
                },
                EvaluatedValue::String("prefix".into()),
            ),
            (
                AssertPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: Some(AssertBinding::new(ParamLocal::Int(IntLocalId(0)))),
                },
                EvaluatedValue::String("prefix".into()),
            ),
            (
                AssertPattern::Alias {
                    pattern: Box::new(AssertPattern::Discard),
                    binding: AssertBinding::new(ParamLocal::Int(IntLocalId(0))),
                },
                EvaluatedValue::String("wrong".into()),
            ),
        ];

        for (pattern, value) in cases {
            let mut bindings = Vec::new();
            assert_eq!(
                match_assert_pattern(
                    &plan,
                    &mut state,
                    &mut frame,
                    &pattern,
                    &value,
                    &mut bindings,
                ),
                Ok(None),
            );
            assert_eq!(bindings, Vec::new());
        }

        let pattern = AssertPattern::StringPrefix {
            prefix: "pre".into(),
            left: None,
            right: Some(AssertBinding::new(ParamLocal::String(StringLocalId(0)))),
        };
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                &pattern,
                &EvaluatedValue::String("prefix".into()),
                &mut bindings,
            ),
            Ok(Some(())),
        );
        assert_eq!(
            bindings,
            vec![PendingBinding::String(StringLocalId(0), "fix".into())],
        );

        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                &AssertPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: None,
                },
                &EvaluatedValue::String("prefix".into()),
                &mut bindings,
            ),
            Ok(Some(())),
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn assert_steps_propagate_custom_field_family_mismatches() {
        let custom_plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() {
  let assert Boxed(value) = Boxed(1)
  value
}
"#,
        );
        let function = custom_plan.int_function(IntFunctionId(0));
        let (assert_index, custom_local) = function
            .steps()
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::AssertCustom { local, .. } => Some((index, *local)),
                _ => None,
            })
            .expect("source should lower an assert-custom step");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        execute_steps(
            &custom_plan,
            &mut state,
            &function.steps()[..assert_index],
            &mut frame,
        )
        .expect("custom setup should execute");
        let constructor_id = frame.get_custom(custom_local).constructor();
        frame.set_custom(
            custom_local,
            EvaluatedCustomValue::new(constructor_id, vec![EvaluatedValue::String("wrong".into())]),
        );
        assert_eq!(
            execute_steps(
                &custom_plan,
                &mut state,
                &function.steps()[assert_index..=assert_index],
                &mut frame,
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: custom_plan.custom_value_type(constructor_id.type_id()),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );

        let list_plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() {
  let assert [Boxed(value)] = [Boxed(1)]
  value
}
"#,
        );
        let function = list_plan.int_function(IntFunctionId(0));
        let (assert_index, list_local, list_type) = function
            .steps()
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::AssertList {
                    local: ListLocal::Custom { local, type_id },
                    ..
                } => Some((index, *local, *type_id)),
                _ => None,
            })
            .expect("source should lower a custom-list assert step");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        execute_steps(
            &list_plan,
            &mut state,
            &function.steps()[..assert_index],
            &mut frame,
        )
        .expect("custom-list setup should execute");
        let constructor_id =
            state.custom_values(&frame.get_custom_list(list_local))[0].constructor();
        let wrong = state.custom(
            list_type,
            vec![EvaluatedCustomValue::new(
                constructor_id,
                vec![EvaluatedValue::String("wrong".into())],
            )],
        );
        frame.set_custom_list(list_local, wrong);
        assert_eq!(
            execute_steps(
                &list_plan,
                &mut state,
                &function.steps()[assert_index..=assert_index],
                &mut frame,
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: list_plan.custom_value_type(constructor_id.type_id()),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    #[test]
    fn custom_assert_reports_direct_mutated_field_family_mismatch() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Choice {
  Boxed(Int)
  Other
}

pub fn main() {
  let assert Boxed(value) = Boxed(1)
  value
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let (pattern, custom_pattern) = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertCustom {
                    pattern: pattern @ AssertPattern::Custom(custom_pattern),
                    ..
                } => Some((pattern, custom_pattern)),
                _ => None,
            })
            .expect("source should lower an assert-custom step");
        let constructor_id = custom_pattern.constructor();
        let constructor = plan.custom_constructor(constructor_id);
        let value = EvaluatedValue::Custom(EvaluatedCustomValue::new(
            constructor_id,
            vec![EvaluatedValue::String("wrong".into())],
        ));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let mut bindings = Vec::new();

        assert_eq!(
            match_and_apply_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                pattern,
                &EvaluatedValue::Int(1.into()),
            ),
            Ok(false),
        );

        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                pattern,
                &value,
                &mut bindings,
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor_id.type_id()),
                constructor: constructor.name().clone(),
                field_index: 0,
                expected: crate::plan::ValueType::Int,
                actual: crate::plan::ValueType::String,
            }),
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn custom_assert_reports_direct_mutated_field_arity_mismatch() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Choice { Boxed(Int) Other }
pub fn main() {
  let assert Boxed(value) = Boxed(1)
  value
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let (pattern, custom_pattern) = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertCustom {
                    pattern: pattern @ AssertPattern::Custom(custom_pattern),
                    ..
                } => Some((pattern, custom_pattern)),
                _ => None,
            })
            .expect("source should lower an assert-custom step");
        let constructor_id = custom_pattern.constructor();
        let constructor = plan.custom_constructor(constructor_id);
        let value = EvaluatedValue::Custom(EvaluatedCustomValue::new(constructor_id, Vec::new()));
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let mut bindings = Vec::new();

        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                pattern,
                &value,
                &mut bindings,
            ),
            Err(ExecutionError::CustomFieldArityMismatch {
                custom_type: plan.custom_value_type(constructor_id.type_id()),
                constructor: constructor.name().clone(),
                expected: 1,
                actual: 0,
            }),
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn assert_panic_materialization_propagates_custom_field_arity_mismatch() {
        let list_plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() {
  let values = [Boxed(1)]
  let assert [] = values
  0
}
"#,
        );
        let list_function = list_plan.int_function(IntFunctionId(0));
        assert_eq!(list_function.steps().len(), 3);
        let mut list_state = crate::runtime::RuntimeState::new();
        let mut list_frame = Frame::new(list_function.frame_layout(), &mut list_state);
        execute_steps(
            &list_plan,
            &mut list_state,
            &list_function.steps()[..2],
            &mut list_frame,
        )
        .expect("list value should evaluate");
        let list_local = CustomListLocalId(1);
        let list_type = list_function.frame_layout().custom_lists()[1];
        let original = list_frame.get_custom_list(list_local);
        let constructor = list_state.custom_values(&original)[0].constructor();
        let descriptor = list_plan.custom_constructor(constructor);
        let malformed = EvaluatedCustomValue::new(constructor, Vec::new());
        let list = list_state.custom(list_type, vec![malformed]);
        list_frame.set_custom_list(list_local, list);
        assert_eq!(
            execute_steps(
                &list_plan,
                &mut list_state,
                &list_function.steps()[2..],
                &mut list_frame,
            ),
            Err(ExecutionError::CustomFieldArityMismatch {
                custom_type: list_plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                expected: 1,
                actual: 0,
            }),
        );

        let custom_plan = crate::runtime::plan_src(
            r#"
pub type Choice { Boxed(Int) Other }
pub fn main() {
  let assert Boxed(value) = Other
  value
}
"#,
        );
        let custom_function = custom_plan.int_function(IntFunctionId(0));
        assert_eq!(custom_function.steps().len(), 2);
        let mut custom_state = crate::runtime::RuntimeState::new();
        let mut custom_frame = Frame::new(custom_function.frame_layout(), &mut custom_state);
        execute_steps(
            &custom_plan,
            &mut custom_state,
            &custom_function.steps()[..1],
            &mut custom_frame,
        )
        .expect("custom value should evaluate");
        let custom_local = CustomLocalId(0);
        let original = custom_frame.get_custom(custom_local);
        let constructor = original.constructor();
        let descriptor = custom_plan.custom_constructor(constructor);
        custom_frame.set_custom(
            custom_local,
            EvaluatedCustomValue::new(constructor, vec![EvaluatedValue::Int(1.into())]),
        );
        assert_eq!(
            execute_steps(
                &custom_plan,
                &mut custom_state,
                &custom_function.steps()[1..],
                &mut custom_frame,
            ),
            Err(ExecutionError::CustomFieldArityMismatch {
                custom_type: custom_plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                expected: 0,
                actual: 1,
            }),
        );
    }

    #[test]
    fn custom_total_binding_reports_discriminant_and_arity_mismatches() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Choice { Boxed(Int) Other(Int) }
pub fn main() {
  let other = Other(2)
  let Boxed(value) = Boxed(1)
  value
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let custom_exprs = function
            .steps()
            .iter()
            .filter_map(|step| match step.kind() {
                StepKind::LetCustom { value, .. } => Some(value),
                _ => None,
            })
            .collect::<Vec<_>>();
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::BindCustomFields { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower a total custom binding");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let other = eval_custom_expr(&plan, &mut state, &mut frame, custom_exprs[0])
            .expect("other constructor should evaluate");
        let boxed = eval_custom_expr(&plan, &mut state, &mut frame, custom_exprs[1])
            .expect("boxed constructor should evaluate");
        let expected = plan.custom_constructor(pattern.constructor());
        let actual = plan.custom_constructor(other.constructor());

        assert_eq!(
            bind_custom_fields(&plan, pattern, &other),
            Err(ExecutionError::CustomFieldDiscriminantMismatch {
                expected_type: plan.custom_value_type(pattern.constructor().type_id()),
                expected_constructors: vec![expected.name().clone()],
                actual_type: plan.custom_value_type(other.type_id()),
                actual_constructor: actual.name().clone(),
            }),
        );

        let missing = EvaluatedCustomValue::new(boxed.constructor(), Vec::new());
        assert_eq!(
            bind_custom_fields(&plan, pattern, &missing),
            Err(ExecutionError::CustomFieldArityMismatch {
                custom_type: plan.custom_value_type(boxed.type_id()),
                constructor: expected.name().clone(),
                expected: 1,
                actual: 0,
            }),
        );

        let extra = EvaluatedCustomValue::new(
            boxed.constructor(),
            vec![EvaluatedValue::Int(1.into()), EvaluatedValue::Int(2.into())],
        );
        assert_eq!(
            bind_custom_fields(&plan, pattern, &extra),
            Err(ExecutionError::CustomFieldArityMismatch {
                custom_type: plan.custom_value_type(boxed.type_id()),
                constructor: expected.name().clone(),
                expected: 1,
                actual: 2,
            }),
        );
    }

    #[test]
    fn nested_custom_assert_errors_propagate_through_list_tuple_and_alias_wrappers() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Inner { Inner(Int) }
pub type Outer { Outer(Inner) }
pub fn main() {
  let ignored = 0
  let value = Outer(Inner(1))
  let assert [#(Outer(Inner(number)) as alias), ..rest] = [#(value)]
  number
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let custom_local = expect_let_custom_local(&function.steps()[1]);
        let (assert_index, list_type, list_pattern) = function
            .steps()
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::AssertList {
                    local: ListLocal::Tuple { type_id, .. },
                    pattern: AssertPattern::List(pattern),
                    ..
                } => Some((index, *type_id, pattern)),
                _ => None,
            })
            .expect("source should lower a tuple-list assert step");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        execute_steps(
            &plan,
            &mut state,
            &function.steps()[..assert_index],
            &mut frame,
        )
        .expect("nested custom setup should execute");
        let outer = frame.get_custom(custom_local);
        let inner = expect_custom_value(&outer.fields()[0]);
        let malformed = EvaluatedCustomValue::new(
            outer.constructor(),
            vec![EvaluatedValue::Custom(EvaluatedCustomValue::new(
                inner.constructor(),
                vec![EvaluatedValue::String("wrong".into())],
            ))],
        );
        let expected = ExecutionError::CustomFieldFamilyMismatch {
            custom_type: plan.custom_value_type(inner.constructor().type_id()),
            constructor: plan.custom_constructor(inner.constructor()).name().clone(),
            field_index: 0,
            expected: ValueType::Int,
            actual: ValueType::String,
        };
        let tuple_pattern = &list_pattern.elements()[0];

        assert_eq!(
            match_and_apply_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                tuple_pattern,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Custom(malformed.clone())]),
            ),
            Err(expected.clone()),
        );

        let malformed_list = ListValueId::Tuple(
            state.tuple(list_type, vec![vec![EvaluatedValue::Custom(malformed)]]),
        );
        assert_eq!(
            match_list_assert_pattern(&plan, &mut state, &mut frame, list_pattern, &malformed_list,),
            Err(expected),
        );
    }

    #[test]
    fn custom_assert_pattern_rejects_wrong_subject_and_nested_literal_without_bindings() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Choice { Boxed(Int) }
pub fn main() {
  let assert Boxed(1) = Boxed(1)
  1
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let (pattern, constructor_id) = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertCustom {
                    pattern: pattern @ AssertPattern::Custom(custom_pattern),
                    ..
                } => Some((pattern, custom_pattern.constructor())),
                _ => None,
            })
            .expect("source should lower an assert-custom step");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let mut bindings = Vec::new();

        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            Ok(None),
        );
        assert_eq!(bindings, Vec::new());
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                pattern,
                &EvaluatedValue::Custom(EvaluatedCustomValue::new(
                    constructor_id,
                    vec![EvaluatedValue::Int(2.into())],
                )),
                &mut bindings,
            ),
            Ok(None),
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn custom_assert_binding_stores_nested_custom_values() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub type Inner { Inner(Int) }
pub type Outer { Outer(Inner) }
pub fn main() {
  let assert Outer(inner) = Outer(Inner(1))
  case inner { Inner(value) -> value }
}
"#,
            ),
            crate::runtime::Value::Int(1.into()),
        );
    }

    #[test]
    fn custom_total_binding_reports_each_direct_mutated_field_family_mismatch() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Inner { Inner(Int) }
pub type Fields {
  Fields(Int, Float, String, BitArray, UtfCodepoint, Inner, Bool, Nil, #(Int), List(Int), fn() -> Int)
}
fn one() { 1 }
pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let Fields(int, float, string, bits, scalar, inner, bool, nil, tuple, list, function) =
    Fields(1, 1.0, "one", <<1>>, codepoint, Inner(2), True, Nil, #(3), [4], one)
  int
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let steps = function.steps();
        let (let_index, custom_local, value) = steps
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::LetCustom { local, value } => Some((index, *local, value)),
                _ => None,
            })
            .expect("source should lower a custom value step");
        let (bind_index, pattern) = steps
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::BindCustomFields { pattern, .. } => Some((index, pattern)),
                _ => None,
            })
            .expect("source should lower a custom field binding step");

        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        execute_steps(&plan, &mut state, &steps[..let_index], &mut frame)
            .expect("codepoint setup should execute");
        let value = eval_custom_expr(&plan, &mut state, &mut frame, value)
            .expect("custom constructor should evaluate");
        let constructor_id = value.constructor();
        let constructor = plan.custom_constructor(constructor_id);

        for field_index in 0..constructor.fields().len() {
            let expected = plan.value_type(constructor.fields()[field_index].type_());
            let replacement = if expected == ValueType::String {
                EvaluatedValue::Int(0.into())
            } else {
                EvaluatedValue::String("wrong".into())
            };
            let actual = replacement.value_type(&plan);
            let mut fields = value.fields().to_vec();
            fields[field_index] = replacement;
            let mutated = EvaluatedCustomValue::new(constructor_id, fields);

            assert_eq!(
                bind_custom_fields(&plan, pattern, &mutated),
                Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor_id.type_id()),
                    constructor: constructor.name().clone(),
                    field_index,
                    expected,
                    actual,
                }),
            );
        }

        let mut fields = value.fields().to_vec();
        fields[0] = EvaluatedValue::String("wrong".into());
        frame.set_custom(
            custom_local,
            EvaluatedCustomValue::new(constructor_id, fields),
        );
        assert_eq!(
            execute_steps(
                &plan,
                &mut state,
                &steps[bind_index..=bind_index],
                &mut frame,
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor_id.type_id()),
                constructor: constructor.name().clone(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    #[test]
    fn custom_total_binding_reports_structural_field_family_mismatches() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Inner { Inner(Int) }
pub type Fields { Fields(#(Int), List(Int), Inner, Int) }
fn strings() { ["wrong"] }
pub fn main() {
  let ignored = 0
  let value = Fields(#(1), [2], Inner(3), 4)
  let Fields(#(tuple) as whole_tuple, [..items], Inner(inner), _ as alias) = value
  tuple + inner + alias
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let steps = function.steps();
        let (let_index, value) = steps
            .iter()
            .enumerate()
            .find_map(|(index, step)| match step.kind() {
                StepKind::LetCustom { value, .. } => Some((index, value)),
                _ => None,
            })
            .expect("source should lower the Fields value step");
        let pattern = steps
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::BindCustomFields { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower the Fields binding step");
        let mut state = crate::runtime::RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        execute_steps(&plan, &mut state, &steps[..let_index], &mut frame)
            .expect("custom value setup should execute");
        let value =
            eval_custom_expr(&plan, &mut state, &mut frame, value).expect("Fields should evaluate");
        let constructor_id = value.constructor();
        let constructor = plan.custom_constructor(constructor_id);
        let wrong_string_list = EvaluatedValue::List(ListValueId::String(state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["wrong".into()],
        )));
        let inner = expect_custom_value(&value.fields()[2]);
        let replacements = vec![
            (
                0,
                EvaluatedValue::String("wrong".into()),
                constructor_id,
                0,
                plan.value_type(constructor.fields()[0].type_()),
                ValueType::String,
            ),
            (
                0,
                EvaluatedValue::Tuple(vec![EvaluatedValue::String("wrong".into())]),
                constructor_id,
                0,
                ValueType::Int,
                ValueType::String,
            ),
            (
                1,
                EvaluatedValue::String("wrong".into()),
                constructor_id,
                1,
                plan.value_type(constructor.fields()[1].type_()),
                ValueType::String,
            ),
            (
                1,
                wrong_string_list,
                constructor_id,
                1,
                plan.value_type(constructor.fields()[1].type_()),
                ValueType::List(Box::new(ValueType::String)),
            ),
            (
                2,
                EvaluatedValue::String("wrong".into()),
                constructor_id,
                2,
                plan.value_type(constructor.fields()[2].type_()),
                ValueType::String,
            ),
            (
                2,
                EvaluatedValue::Custom(EvaluatedCustomValue::new(
                    inner.constructor(),
                    vec![EvaluatedValue::String("wrong".into())],
                )),
                inner.constructor(),
                0,
                ValueType::Int,
                ValueType::String,
            ),
            (
                3,
                EvaluatedValue::String("wrong".into()),
                constructor_id,
                3,
                ValueType::Int,
                ValueType::String,
            ),
        ];

        for (mutation_index, replacement, error_constructor, field_index, expected, actual) in
            replacements
        {
            let mut fields = value.fields().to_vec();
            fields[mutation_index] = replacement;
            let mutated = EvaluatedCustomValue::new(constructor_id, fields);

            assert_eq!(
                bind_custom_fields(&plan, pattern, &mutated),
                Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(error_constructor.type_id()),
                    constructor: plan.custom_constructor(error_constructor).name().clone(),
                    field_index,
                    expected,
                    actual,
                }),
            );
        }
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
        let list_function = plan.int_list_function(plan.int_list_function_id(0));
        let pattern = list_function
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
        let ignored_function = ignored_plan.int_function(IntFunctionId(0));
        let ignored_tail = ignored_function
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
        let mut frame = Frame::new(list_function.frame_layout(), &mut state);

        let bindings = match_list_assert_pattern(
            &plan,
            &mut state,
            &mut frame,
            pattern,
            &value.clone().into(),
        )
        .expect("list pattern evaluation should succeed")
        .expect("list pattern should match");
        assert_eq!(bindings[0], PendingBinding::Int(IntLocalId(0), 1.into()));
        assert_eq!(int_list_binding(&bindings[0]), None);
        let (local, tail) = int_list_binding(&bindings[1]).expect("tail must bind List(Int)");
        assert_eq!(local, IntListLocalId(1));
        assert_eq!(state.int_values(tail), &[2.into(), 3.into()]);
        let mut ignored_frame = Frame::new(ignored_function.frame_layout(), &mut state);
        assert_eq!(
            match_list_assert_pattern(
                &ignored_plan,
                &mut state,
                &mut ignored_frame,
                ignored_tail,
                &value.into(),
            ),
            Ok(Some(vec![PendingBinding::Int(IntLocalId(0), 1.into())])),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &nested_plan,
                &mut state,
                &mut frame,
                tuple_pattern,
                &EvaluatedValue::Tuple(vec![
                    EvaluatedValue::Int(1.into()),
                    EvaluatedValue::String("wrong".into())
                ]),
                &mut bindings,
            ),
            Ok(None),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        bindings.clear();
        assert_eq!(
            match_assert_pattern(
                &alias_plan,
                &mut state,
                &mut frame,
                alias_pattern,
                &EvaluatedValue::Int(1.into()),
                &mut bindings,
            ),
            Ok(None),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
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
                &mut frame,
                alias_pattern,
                &EvaluatedValue::Function(wrong_kind),
                &mut bindings,
            ),
            Ok(None),
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);

        assert_eq!(
            match_list_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                patterns[0],
                &ListValueId::String(wrong_list.clone()),
            ),
            Ok(None),
        );

        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                &plan,
                &mut state,
                &mut frame,
                &patterns[1].elements()[0],
                &EvaluatedValue::List(ListValueId::String(wrong_list)),
                &mut bindings,
            ),
            Ok(None),
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
                &mut frame,
                &patterns[2].elements()[0],
                &EvaluatedValue::Function(wrong_function),
                &mut bindings,
            ),
            Ok(None),
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

    fn expect_custom_value(value: &EvaluatedValue) -> &EvaluatedCustomValue {
        match value {
            EvaluatedValue::Custom(value) => value,
            _ => panic!("expected a custom value"),
        }
    }

    fn expect_let_custom_local(step: &Step) -> CustomLocalId {
        match step.kind() {
            StepKind::LetCustom { local, .. } => *local,
            _ => panic!("expected a let-custom step"),
        }
    }

    #[test]
    #[should_panic(expected = "expected a custom value")]
    fn custom_value_shape_guard_rejects_int_values() {
        let _ = expect_custom_value(&EvaluatedValue::Int(0.into()));
    }

    #[test]
    #[should_panic(expected = "expected a let-custom step")]
    fn let_custom_step_shape_guard_rejects_int_steps() {
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let value = 1
  value
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));

        let _ = expect_let_custom_local(&function.steps()[0]);
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
