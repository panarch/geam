use super::super::GraphValue;
use super::super::environment::BlockEnvironment;
use crate::plan::ValueType;
use crate::plan::execution::constant::{ConstantId, ConstantValue};
use crate::plan::execution::graph::{
    BitArrayInstruction, BoolInstruction, CustomInstruction, FloatInstruction, IntInstruction,
    NilInstruction, ParamLocal, StringInstruction, TupleInstruction, UtfCodepointInstruction,
};
use crate::runtime::constant::evaluate as evaluate_constant;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedValue, values_equal,
};
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutableRuntimePlan, ExecutionError, InvariantError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn int(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &IntInstruction,
    expected: &ValueType,
) -> ExecutionResult<BigInt> {
    use IntInstruction as I;

    match instruction {
        I::Value(value) => Ok(value.clone()),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_int(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.int_function(*function);
            crate::runtime::function::run_int(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Int(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Int(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.int_values(&environment.int_list(*list)),
        ),
        I::Add { left, right } => Ok(environment.int(*left) + environment.int(*right)),
        I::Sub { left, right } => Ok(environment.int(*left) - environment.int(*right)),
        I::Mult { left, right } => Ok(environment.int(*left) * environment.int(*right)),
        I::Div { left, right } => {
            let right = environment.int(*right);
            if right == BigInt::from(0) {
                Ok(BigInt::from(0))
            } else {
                Ok(environment.int(*left) / right)
            }
        }
        I::Remainder { left, right } => {
            let right = environment.int(*right);
            if right == BigInt::from(0) {
                Ok(BigInt::from(0))
            } else {
                Ok(environment.int(*left) % right)
            }
        }
        I::Negate(value) => Ok(-environment.int(*value)),
    }
}

pub(super) fn float(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &FloatInstruction,
    expected: &ValueType,
) -> ExecutionResult<f64> {
    use FloatInstruction as I;

    match instruction {
        I::Value(value) => Ok(*value),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_float(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.float_function(*function);
            crate::runtime::function::run_float(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Float(value) => Some(*value),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Float(value) => Some(*value),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.float_values(&environment.float_list(*list)),
        ),
        I::Add { left, right } => Ok(environment.float(*left) + environment.float(*right)),
        I::Sub { left, right } => Ok(environment.float(*left) - environment.float(*right)),
        I::Mult { left, right } => Ok(environment.float(*left) * environment.float(*right)),
        I::Div { left, right } => {
            let right = environment.float(*right);
            if right == 0.0 {
                Ok(0.0)
            } else {
                Ok(environment.float(*left) / right)
            }
        }
    }
}

pub(super) fn string(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &StringInstruction,
    expected: &ValueType,
) -> ExecutionResult<EcoString> {
    use StringInstruction as I;

    match instruction {
        I::Value(value) => Ok(value.clone()),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_string(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.string_function(*function);
            crate::runtime::function::run_string(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::String(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::String(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.string_values(&environment.string_list(*list)),
        ),
        I::Concatenate { left, right } => Ok(format!(
            "{}{}",
            environment.string(*left),
            environment.string(*right),
        )
        .into()),
        I::DropPrefix { value, prefix } => {
            let value = environment.string(*value);
            Ok(value[prefix.len()..].into())
        }
    }
}

pub(super) fn bit_array(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &BitArrayInstruction,
    expected: &ValueType,
) -> ExecutionResult<EvaluatedBitArray> {
    use BitArrayInstruction as I;

    match instruction {
        I::Value(segments) => super::super::bit_array::evaluate(plan, environment, segments),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => crate::runtime::function::run_bit_array(
            plan,
            state,
            *function,
            environment.retain(args),
        ),
        I::FunctionCall { function, args } => {
            let function = environment.bit_array_function(*function);
            crate::runtime::function::run_bit_array(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::BitArray(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::BitArray(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.bit_array_values(&environment.bit_array_list(*list)),
        ),
    }
}

pub(super) fn utf_codepoint(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &UtfCodepointInstruction,
    expected: &ValueType,
) -> ExecutionResult<char> {
    use UtfCodepointInstruction as I;

    match instruction {
        I::Call { function, args } => crate::runtime::function::run_utf_codepoint(
            plan,
            state,
            *function,
            environment.retain(args),
        ),
        I::FunctionCall { function, args } => {
            let function = environment.utf_codepoint_function(*function);
            crate::runtime::function::run_utf_codepoint(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::UtfCodepoint(value) => Some(*value),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::UtfCodepoint(value) => Some(*value),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.utf_codepoint_values(&environment.utf_codepoint_list(*list)),
        ),
    }
}

pub(super) fn custom(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &CustomInstruction,
    expected: &ValueType,
) -> ExecutionResult<EvaluatedCustomValue> {
    use CustomInstruction as I;

    match instruction {
        I::Construct {
            constructor,
            fields,
        } => Ok(EvaluatedCustomValue::from_fields(
            *constructor,
            environment.values(fields),
        )),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_custom(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.custom_function(function);
            match function {
                EvaluatedCustomFunction::Function(function) => {
                    crate::runtime::function::run_custom(
                        plan,
                        state,
                        function.runtime_id(),
                        inputs_with_captures(environment, args, function.captures()),
                    )
                }
                EvaluatedCustomFunction::Constructor(function) => {
                    Ok(EvaluatedCustomValue::from_fields(
                        function.runtime_id(),
                        environment.values(args),
                    ))
                }
            }
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Custom(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Custom(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.custom_values(&environment.custom_list(*list)),
        ),
    }
}

pub(super) fn bool(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &BoolInstruction,
    expected: &ValueType,
) -> ExecutionResult<bool> {
    use BoolInstruction as I;

    match instruction {
        I::Value(value) => Ok(*value),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_bool(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.bool_function(*function);
            crate::runtime::function::run_bool(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Bool(value) => Some(*value),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Bool(value) => Some(*value),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.bool_values(&environment.bool_list(*list)),
        ),
        I::Not(value) => Ok(!environment.bool(*value)),
        I::LtInt { left, right } => Ok(environment.int(*left) < environment.int(*right)),
        I::LtEqInt { left, right } => Ok(environment.int(*left) <= environment.int(*right)),
        I::GtInt { left, right } => Ok(environment.int(*left) > environment.int(*right)),
        I::GtEqInt { left, right } => Ok(environment.int(*left) >= environment.int(*right)),
        I::LtFloat { left, right } => Ok(environment.float(*left) < environment.float(*right)),
        I::LtEqFloat { left, right } => Ok(environment.float(*left) <= environment.float(*right)),
        I::GtFloat { left, right } => Ok(environment.float(*left) > environment.float(*right)),
        I::GtEqFloat { left, right } => Ok(environment.float(*left) >= environment.float(*right)),
        I::Equal { left, right } => Ok(values_equal(
            plan,
            state,
            &environment.value(left),
            &environment.value(right),
        )),
        I::NotEqual { left, right } => Ok(!values_equal(
            plan,
            state,
            &environment.value(left),
            &environment.value(right),
        )),
        I::StringStartsWith { value, prefix } => {
            Ok(environment.string(*value).starts_with(prefix.as_str()))
        }
        I::ListLengthEquals { value, length } => {
            Ok(state.list_len(&environment.list(value)) == *length)
        }
        I::ListLengthAtLeast { value, length } => {
            Ok(state.list_len(&environment.list(value)) >= *length)
        }
    }
}

pub(super) fn nil(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &NilInstruction,
    expected: &ValueType,
) -> ExecutionResult<()> {
    use NilInstruction as I;

    match instruction {
        I::Value => Ok(()),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_nil(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.nil_function(*function);
            crate::runtime::function::run_nil(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => {
            tuple_projection(plan, environment, *tuple, *index, expected, |value| {
                matches!(value, EvaluatedValue::Nil).then_some(())
            })
        }
        I::CustomField { source, index } => {
            custom_projection(plan, environment, source, *index, expected, |value| {
                matches!(value, EvaluatedValue::Nil).then_some(())
            })
        }
        I::ListIndex { list, index } => {
            let length = state.nil_len(&environment.nil_list(*list));
            ensure_list_index(expected, *index, length)
        }
    }
}

pub(super) fn tuple(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    instruction: &TupleInstruction,
    expected: &ValueType,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    use TupleInstruction as I;

    match instruction {
        I::Value(values) => Ok(environment.values(values).into_vec()),
        I::Constant(id) => constant(plan, state, *id),
        I::Call { function, args } => {
            crate::runtime::function::run_tuple(plan, state, *function, environment.retain(args))
        }
        I::FunctionCall { function, args } => {
            let function = environment.tuple_function(*function);
            crate::runtime::function::run_tuple(
                plan,
                state,
                function.runtime_id(),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan,
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Tuple(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Tuple(value) => Some(value.clone()),
                _ => None,
            },
        ),
        I::ListIndex { list, index } => list_element(
            plan,
            expected,
            *index,
            state.tuple_values(&environment.tuple_list(*list)),
        ),
    }
}

pub(super) fn constant<Value>(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    id: ConstantId<Value>,
) -> ExecutionResult<Value::Evaluated>
where
    Value: ConstantValue + GraphValue,
{
    evaluate_constant(plan, state, plan.constant(id))
}

pub(super) fn tuple_projection<Value>(
    plan: &impl ExecutableRuntimePlan,
    environment: &BlockEnvironment,
    tuple: crate::plan::execution::graph::TupleLocalId,
    index: usize,
    expected: &ValueType,
    project: impl FnOnce(&EvaluatedValue) -> Option<Value>,
) -> ExecutionResult<Value> {
    let values = environment.tuple(tuple);
    let Some(value) = values.get(index) else {
        return Err(ExecutionError::Invariant(
            InvariantError::TupleIndexFamilyMismatch {
                expected: expected.clone(),
                actual: ValueType::Tuple(
                    values.iter().map(|value| value.value_type(plan)).collect(),
                ),
            },
        ));
    };
    let actual = value.value_type(plan);
    if let Some(value) = project(value)
        && actual == *expected
    {
        return Ok(value);
    }

    Err(ExecutionError::Invariant(
        InvariantError::TupleIndexFamilyMismatch {
            expected: expected.clone(),
            actual,
        },
    ))
}

pub(super) fn custom_projection<Value>(
    plan: &impl ExecutableRuntimePlan,
    environment: &BlockEnvironment,
    source: &crate::plan::execution::graph::CustomLocal,
    index: usize,
    expected: &ValueType,
    project: impl FnOnce(&EvaluatedValue) -> Option<Value>,
) -> ExecutionResult<Value> {
    let source = environment.custom(*source);
    let constructor = source.constructor();
    let value = &source.fields()[index];
    let actual = value.value_type(plan);
    if let Some(value) = project(value)
        && actual == *expected
    {
        return Ok(value);
    }
    let descriptor = plan.custom_constructor(constructor);

    Err(ExecutionError::Invariant(
        InvariantError::CustomFieldFamilyMismatch {
            custom_type: plan.custom_value_type(constructor.type_id()),
            constructor: descriptor.name().clone(),
            field_index: index,
            expected: expected.clone(),
            actual,
        },
    ))
}

pub(super) fn list_element<Value: Clone>(
    _plan: &impl ExecutableRuntimePlan,
    item_type: &ValueType,
    index: usize,
    values: &[Value],
) -> ExecutionResult<Value> {
    match values.get(index) {
        Some(value) => Ok(value.clone()),
        None => Err(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: item_type.clone(),
                index,
                length: values.len(),
            },
        )),
    }
}

pub(super) fn ensure_list_index(
    item_type: &ValueType,
    index: usize,
    length: usize,
) -> ExecutionResult<()> {
    if index < length {
        Ok(())
    } else {
        Err(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: item_type.clone(),
                index,
                length,
            },
        ))
    }
}

fn inputs_with_captures(
    environment: &BlockEnvironment,
    args: &[ParamLocal],
    captures: &[crate::runtime::EvaluatedCapture],
) -> super::super::environment::RetainedValues {
    let mut inputs = environment.retain(args);
    inputs.append_captures(captures);
    inputs
}

#[cfg(test)]
mod tests {
    use super::super::super::environment::{BlockEnvironment, RetainedValues};
    use super::{ensure_list_index, list_element, tuple_projection};
    use crate::plan::execution::graph::TupleLocalId;
    use crate::plan::{
        CustomConstructor, CustomConstructorDefinition, CustomConstructorField, CustomExpr,
        CustomFieldAccess, CustomFieldDefinition, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, CustomTypeTemplate, Expr, FunctionExpr, FunctionReference,
        FunctionShape, FunctionTemplate, FunctionTemplateId, FunctionType, IntExpr, ListExpr,
        ModulePlan, ReturnBody, ReturnExpr, StringExpr, TupleExpr, ValueType,
        monomorphic_function_instantiation,
    };
    use crate::runtime::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue,
        ExecutionError, InvariantError, Value,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    fn string_value(value: &EvaluatedValue) -> Option<EcoString> {
        match value {
            EvaluatedValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    #[test]
    fn missing_tuple_field_reports_the_actual_tuple_type() {
        let plan = crate::runtime::plan_src("pub fn main() { 0 }");
        let mut values = RetainedValues::empty();
        values.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
        let environment = BlockEnvironment::from_retained(values);

        assert_eq!(
            tuple_projection(
                &plan,
                &environment,
                TupleLocalId(0),
                1,
                &ValueType::String,
                string_value,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected: ValueType::String,
                    actual: ValueType::Tuple(vec![ValueType::Int]),
                },
            )),
        );
    }

    #[test]
    fn tuple_projection_distinguishes_valid_values_from_wrong_field_families() {
        let plan = crate::runtime::plan_src("pub fn main() { 0 }");
        let expected = ValueType::String;

        let mut valid = RetainedValues::empty();
        valid.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::String(
            "value".into(),
        )]));
        let valid = BlockEnvironment::from_retained(valid);
        assert_eq!(
            tuple_projection(&plan, &valid, TupleLocalId(0), 0, &expected, string_value,),
            Ok("value".into()),
        );

        let mut wrong = RetainedValues::empty();
        wrong.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
        let wrong = BlockEnvironment::from_retained(wrong);
        assert_eq!(
            tuple_projection(&plan, &wrong, TupleLocalId(0), 0, &expected, string_value,),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected,
                    actual: ValueType::Int,
                },
            )),
        );
    }

    #[test]
    fn every_leaf_list_storage_reports_the_exact_missing_index() {
        let plan = crate::runtime::plan_src("pub fn main() { 0 }");
        assert_missing_list_element::<BigInt>(&plan, ValueType::Int);
        assert_missing_list_element::<f64>(&plan, ValueType::Float);
        assert_missing_list_element::<EcoString>(&plan, ValueType::String);
        assert_missing_list_element::<EvaluatedBitArray>(&plan, ValueType::BitArray);
        assert_missing_list_element::<char>(&plan, ValueType::UtfCodepoint);
        assert_missing_list_element::<EvaluatedCustomValue>(&plan, ValueType::Custom(boxed_type()));
        assert_missing_list_element::<bool>(&plan, ValueType::Bool);
        assert_missing_list_element::<Vec<EvaluatedValue>>(
            &plan,
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_missing_list_element::<EvaluatedFunctionValue>(
            &plan,
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );

        assert_eq!(
            ensure_list_index(&ValueType::Nil, 2, 0),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: ValueType::Nil,
                    index: 2,
                    length: 0,
                },
            )),
        );
    }

    fn assert_missing_list_element<Value: Clone>(plan: &crate::ExecutionPlan, type_: ValueType) {
        let values: &[Value] = &[];
        assert_eq!(
            list_element(plan, &type_, 2, values).map(|_| ()),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: type_,
                    index: 2,
                    length: 0,
                },
            )),
        );
    }

    #[test]
    fn source_int_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }

pub fn main() {
  let local = 1
  let function = add_one
  #(
    local,
    add_one(1),
    function(1),
    #(3).0,
    case [4] { [value] -> value _ -> 0 },
    1 + 2,
    5 - 2,
    2 * 3,
    7 / 2,
    7 / 0,
    7 % 3,
    7 % 0,
    -local,
    case True { True -> 1 False -> 0 },
    case False { True -> 1 False -> 0 },
    case 1 { 1 -> 2 _ -> 0 },
    case 2 { 1 -> 2 _ -> 3 },
    case "one" { "one" -> 1 _ -> 0 },
    case "two" { "one" -> 1 _ -> 2 },
    case 1.0 { 1.0 -> 1 _ -> 0 },
    case 2.0 { 1.0 -> 1 _ -> 2 },
    { let _ = 0 4 },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                vec![
                    1_i64, 2, 2, 3, 4, 3, 3, 6, 3, 0, 1, 0, -1, 1, 0, 2, 3, 1, 2, 1, 2, 4,
                ]
                .into_iter()
                .map(|value| Value::Int(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_float_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn add_half(value: Float) -> Float { value +. 0.5 }

pub fn main() {
  let local = 1.0
  let function = add_half
  let true_selector = True
  let false_selector = False
  #(
    local,
    add_half(1.0),
    function(1.0),
    #(2.0).0,
    case [3.0] { [value] -> value _ -> 0.0 },
    1.0 +. 2.0,
    5.0 -. 2.0,
    2.0 *. 3.0,
    7.0 /. 2.0,
    7.0 /. 0.0,
    case true_selector { True -> 1.0 False -> 0.0 },
    case false_selector { True -> 1.0 False -> 0.0 },
    case 1 { 1 -> 2.0 _ -> 0.0 },
    case 2 { 1 -> 2.0 _ -> 3.0 },
    case "one" { "one" -> 1.0 _ -> 0.0 },
    case "two" { "one" -> 1.0 _ -> 2.0 },
    case 1.0 { 1.0 -> 1.0 _ -> 0.0 },
    case 2.0 { 1.0 -> 1.0 _ -> 2.0 },
    { let _ = 0 4.0 },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                vec![
                    1.0, 1.5, 1.5, 2.0, 3.0, 3.0, 3.0, 6.0, 3.5, 0.0, 1.0, 0.0, 2.0, 3.0, 1.0, 2.0,
                    1.0, 2.0, 4.0,
                ]
                .into_iter()
                .map(Value::Float)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_string_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn suffix(value: String) -> String { value <> "!" }

pub fn main() {
  let local = "local"
  let function = suffix
  let true_selector = True
  let false_selector = False
  #(
    local,
    suffix("call"),
    function("function"),
    #("tuple").0,
    case ["list"] { [value] -> value _ -> "missing" },
    "left" <> "right",
    case "prefix-rest" { "prefix-" <> rest -> rest _ -> "missing" },
    case true_selector { True -> "true" False -> "false" },
    case false_selector { True -> "true" False -> "false" },
    case 1 { 1 -> "one" _ -> "other" },
    case 2 { 1 -> "one" _ -> "other" },
    case "one" { "one" -> "match" _ -> "other" },
    case "two" { "one" -> "match" _ -> "other" },
    case 1.0 { 1.0 -> "match" _ -> "other" },
    case 2.0 { 1.0 -> "match" _ -> "other" },
    { let _ = 0 "block" },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                [
                    "local",
                    "call!",
                    "function!",
                    "tuple",
                    "list",
                    "leftright",
                    "rest",
                    "true",
                    "false",
                    "one",
                    "other",
                    "match",
                    "other",
                    "match",
                    "other",
                    "block",
                ]
                .into_iter()
                .map(|value| Value::String(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_bool_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn invert(value: Bool) -> Bool { !value }

pub fn main() {
  let local = True
  let function = invert
  #(
    local,
    invert(True),
    function(False),
    #(True).0,
    case [True] { [value] -> value _ -> False },
    !False,
    1 < 2,
    1 <= 1,
    2 > 1,
    2 >= 2,
    1.0 <. 2.0,
    1.0 <=. 1.0,
    2.0 >. 1.0,
    2.0 >=. 2.0,
    #(1, "one") == #(1, "one"),
    [1] != [2],
    case "prefix-rest" { "prefix-" <> _ -> True _ -> False },
    case [1, 2] { [_, _] -> True _ -> False },
    case [1, 2] { [_, ..] -> True _ -> False },
    True && True,
    False && True,
    True || False,
    False || True,
    case True { True -> True False -> False },
    case False { True -> True False -> False },
    case 1 { 1 -> True _ -> False },
    case 2 { 1 -> True _ -> False },
    case "one" { "one" -> True _ -> False },
    case "two" { "one" -> True _ -> False },
    case 1.0 { 1.0 -> True _ -> False },
    case 2.0 { 1.0 -> True _ -> False },
    { let _ = 0 True },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                vec![
                    true, false, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, false, true, true, true, false, true,
                    false, true, false, true, false, true,
                ]
                .into_iter()
                .map(Value::Bool)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_nil_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn nil_value() -> Nil { Nil }

pub fn main() {
  let local = Nil
  let function = nil_value
  let true_selector = True
  let false_selector = False
  #(
    local,
    nil_value(),
    function(),
    #(Nil).0,
    case [Nil] { [value] -> value _ -> Nil },
    case true_selector { True -> Nil False -> Nil },
    case false_selector { True -> Nil False -> Nil },
    case 1 { 1 -> Nil _ -> Nil },
    case 2 { 1 -> Nil _ -> Nil },
    case "one" { "one" -> Nil _ -> Nil },
    case "two" { "one" -> Nil _ -> Nil },
    case 1.0 { 1.0 -> Nil _ -> Nil },
    case 2.0 { 1.0 -> Nil _ -> Nil },
    { let _ = 0 Nil },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![Value::Nil; 14]),
        );
    }

    #[test]
    fn source_tuple_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn pair(value: Int) { #(value) }

pub fn main() {
  let local = #(1)
  let function = pair
  let true_selector = True
  let false_selector = False
  #(
    #(0),
    local,
    pair(2),
    function(3),
    #(#(4)).0,
    case [#(5)] { [value] -> value _ -> #(0) },
    case true_selector { True -> #(6) False -> #(0) },
    case false_selector { True -> #(0) False -> #(7) },
    case 1 { 1 -> #(8) _ -> #(0) },
    case 2 { 1 -> #(0) _ -> #(9) },
    case "one" { "one" -> #(10) _ -> #(0) },
    case "two" { "one" -> #(0) _ -> #(11) },
    case 1.0 { 1.0 -> #(12) _ -> #(0) },
    case 2.0 { 1.0 -> #(0) _ -> #(13) },
    { let _ = 0 #(14) },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                (0_i64..=14)
                    .map(|value| Value::Tuple(vec![Value::Int(value.into())]))
                    .collect(),
            ),
        );
    }

    #[test]
    fn source_custom_instruction_variants_evaluate_exact_values() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn boxed(value: Int) -> Boxed { Boxed(value) }

fn unbox(value: Boxed) -> Int {
  case value { Boxed(inner) -> inner }
}

pub fn main() {
  let local = Boxed(1)
  let function = boxed
  let true_selector = True
  let false_selector = False
  #(
    unbox(local),
    unbox(boxed(2)),
    unbox(function(3)),
    unbox(#(Boxed(4)).0),
    case [Boxed(5)] { [Boxed(value)] -> value _ -> 0 },
    unbox(case true_selector { True -> Boxed(6) False -> Boxed(0) }),
    unbox(case false_selector { True -> Boxed(0) False -> Boxed(7) }),
    unbox(case 1 { 1 -> Boxed(8) _ -> Boxed(0) }),
    unbox(case 0 { 1 -> Boxed(0) _ -> Boxed(9) }),
    unbox(case "hit" { "hit" -> Boxed(10) _ -> Boxed(0) }),
    unbox(case "miss" { "hit" -> Boxed(0) _ -> Boxed(11) }),
    unbox(case 1.0 { 1.0 -> Boxed(12) _ -> Boxed(0) }),
    unbox(case 0.0 { 1.0 -> Boxed(0) _ -> Boxed(13) }),
    unbox({ let _ = 0 Boxed(14) }),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple((1_i64..=14).map(|value| Value::Int(value.into())).collect(),),
        );
    }

    #[test]
    fn source_utf_codepoint_instruction_paths_evaluate_exact_values() {
        let bytes = [
            1, 2, 3, 11, 12, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16, 17, 18, 19, 20, 21, 22, 13,
        ];

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../../tests/fixtures/execution/values/utf_codepoint_expression_paths.gleam"
            )),
            Value::Tuple(
                bytes
                    .into_iter()
                    .map(|byte| {
                        Value::BitArray(crate::runtime::BitArrayValue::from_bytes(vec![byte]))
                    })
                    .collect(),
            ),
        );
    }

    #[test]
    fn source_value_instruction_operands_preserve_failure_order() {
        let cases = [
            ("Int", "fail_int() + 1"),
            ("Int", "1 + fail_int()"),
            ("Int", "fail_int() - 1"),
            ("Int", "1 - fail_int()"),
            ("Int", "fail_int() * 1"),
            ("Int", "1 * fail_int()"),
            ("Int", "fail_int() / 1"),
            ("Int", "1 / fail_int()"),
            ("Int", "fail_int() % 1"),
            ("Int", "1 % fail_int()"),
            ("Int", "-fail_int()"),
            ("Float", "fail_float() +. 1.0"),
            ("Float", "1.0 +. fail_float()"),
            ("Float", "fail_float() -. 1.0"),
            ("Float", "1.0 -. fail_float()"),
            ("Float", "fail_float() *. 1.0"),
            ("Float", "1.0 *. fail_float()"),
            ("Float", "fail_float() /. 1.0"),
            ("Float", "1.0 /. fail_float()"),
            ("String", "fail_string() <> \"suffix\""),
            ("String", "\"prefix\" <> fail_string()"),
            ("Bool", "!fail_bool()"),
            ("Bool", "fail_int() < 1"),
            ("Bool", "1 < fail_int()"),
            ("Bool", "fail_float() <. 1.0"),
            ("Bool", "1.0 <. fail_float()"),
            ("Bool", "fail_int() == 1"),
            ("Bool", "1 != fail_int()"),
            ("Bool", "fail_bool() && True"),
            ("Bool", "True && fail_bool()"),
            ("Bool", "fail_bool() || False"),
            ("Bool", "False || fail_bool()"),
            ("#(Int)", "#(fail_int())"),
        ];

        for (return_type, expression) in cases {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
pub fn main() -> {return_type} {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn custom_field_and_tuple_projections_report_every_exact_family_mismatch() {
        let inner_type = CustomType::new(inner_name(), Vec::new());
        let int_function = FunctionType::new(Vec::new(), ValueType::Int);
        let expected_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(inner_type.clone()),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ValueType::List(Box::new(ValueType::Custom(inner_type.clone()))),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::String)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(
                int_function.clone(),
            )))),
            ValueType::Function(Box::new(int_function.clone())),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::BitArray))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::UtfCodepoint,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Custom(inner_type),
            ))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(int_function)),
            ))),
        ];

        for expected in expected_types {
            let (actual, actual_value, functions) = mismatched_field_value(&expected);
            let access = field_access(expected.clone(), actual_value);
            assert_eq!(
                run_projection(
                    Expr::custom_field_shape(
                        access,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    functions,
                ),
                Err(ExecutionError::Invariant(
                    InvariantError::CustomFieldFamilyMismatch {
                        custom_type: boxed_type(),
                        constructor: "Boxed".into(),
                        field_index: 0,
                        expected: expected.clone(),
                        actual: actual.clone(),
                    },
                )),
            );

            let (tuple_actual, tuple_value, tuple_functions) = mismatched_field_value(&expected);
            assert_eq!(tuple_actual, actual);
            let tuple = TupleExpr::value(vec![tuple_value], vec![expected.clone()]);
            assert_eq!(
                run_projection(
                    Expr::tuple_index_shape(
                        tuple,
                        0,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    tuple_functions,
                ),
                Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: expected.clone(),
                        actual: actual.clone(),
                    },
                )),
            );

            let access = CustomFieldAccess::new(
                CustomExpr::tuple_index_shape(
                    TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Custom(boxed_type())],
                    ),
                    0,
                    crate::plan::CustomValueShape::any(boxed_type()),
                ),
                0,
                Some("value".into()),
            );
            assert_eq!(
                run_projection(
                    Expr::custom_field_shape(
                        access,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    Vec::new(),
                ),
                Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: ValueType::Custom(boxed_type()),
                        actual: ValueType::Int,
                    },
                )),
            );
        }
    }

    fn mismatched_field_value(expected: &ValueType) -> (ValueType, Expr, Vec<FunctionTemplate>) {
        match expected {
            ValueType::Int => (
                ValueType::String,
                Expr::string(crate::plan::StringExpr::value("wrong".into())),
                Vec::new(),
            ),
            ValueType::List(item) if item.as_ref() == &ValueType::Int => (
                ValueType::List(Box::new(ValueType::String)),
                Expr::list(ListExpr::value(Vec::new(), ValueType::String)),
                Vec::new(),
            ),
            ValueType::List(_) => (
                ValueType::List(Box::new(ValueType::Int)),
                Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                Vec::new(),
            ),
            ValueType::Function(function) if function.return_() == &ValueType::Int => {
                let actual = FunctionType::new(Vec::new(), ValueType::String);
                (
                    ValueType::Function(Box::new(actual.clone())),
                    Expr::function(FunctionExpr::reference(FunctionReference::new(
                        monomorphic_function_instantiation(
                            1,
                            FunctionShape::from_function_type(actual),
                        ),
                    ))),
                    vec![FunctionTemplate::new(
                        FunctionTemplateId::new(1),
                        "string_target".into(),
                        Vec::new(),
                        Vec::new(),
                        ReturnExpr::string_body(ReturnBody::expr(StringExpr::value(
                            "wrong".into(),
                        ))),
                    )],
                )
            }
            ValueType::Function(_) => {
                let actual = FunctionType::new(Vec::new(), ValueType::Int);
                (
                    ValueType::Function(Box::new(actual.clone())),
                    Expr::function(FunctionExpr::reference(FunctionReference::new(
                        monomorphic_function_instantiation(
                            1,
                            FunctionShape::from_function_type(actual),
                        ),
                    ))),
                    vec![FunctionTemplate::new(
                        FunctionTemplateId::new(1),
                        "int_target".into(),
                        Vec::new(),
                        Vec::new(),
                        ReturnExpr::int_body(ReturnBody::expr(IntExpr::value(1.into()))),
                    )],
                )
            }
            _ => (
                ValueType::Int,
                Expr::int(IntExpr::value(1.into())),
                Vec::new(),
            ),
        }
    }

    fn field_access(expected: ValueType, value: Expr) -> CustomFieldAccess {
        let constructor = CustomConstructor::new(
            boxed_type(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(Some("value".into()), expected)],
        );
        CustomFieldAccess::new(
            CustomExpr::try_constructor(constructor, vec![value])
                .expect("test custom construction should be valid"),
            0,
            Some("value".into()),
        )
    }

    fn run_projection(
        expression: Expr,
        functions: Vec<FunctionTemplate>,
    ) -> Result<Value, ExecutionError> {
        let return_type = expression.value_type();
        let field_type = custom_type_template(&return_type);
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::tuple_body(
                vec![return_type.clone()],
                ReturnBody::expr(TupleExpr::value(vec![expression], vec![return_type])),
            ),
        );
        let module = ModulePlan::new("main".into(), main, functions).with_custom_types(vec![
            CustomTypeDefinition::new(
                inner_name(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Inner".into(),
                    0,
                    Vec::new(),
                )],
            ),
            CustomTypeDefinition::new(
                boxed_name(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Boxed".into(),
                    0,
                    vec![CustomFieldDefinition::new(Some("value".into()), field_type)],
                )],
            ),
        ]);
        let plan = crate::ExecutionPlan::from_module_plan(module);
        crate::run_main(&plan, &mut Vec::new())
    }

    fn custom_type_template(type_: &ValueType) -> CustomTypeTemplate {
        match type_ {
            ValueType::Parameter(parameter) => {
                CustomTypeTemplate::Parameter(crate::plan::CustomTypeParameterId(parameter.0))
            }
            ValueType::Int => CustomTypeTemplate::Int,
            ValueType::Float => CustomTypeTemplate::Float,
            ValueType::String => CustomTypeTemplate::String,
            ValueType::BitArray => CustomTypeTemplate::BitArray,
            ValueType::UtfCodepoint => CustomTypeTemplate::UtfCodepoint,
            ValueType::Bool => CustomTypeTemplate::Bool,
            ValueType::Nil => CustomTypeTemplate::Nil,
            ValueType::Tuple(elements) => {
                CustomTypeTemplate::Tuple(elements.iter().map(custom_type_template).collect())
            }
            ValueType::List(item) => CustomTypeTemplate::List(Box::new(custom_type_template(item))),
            ValueType::Function(function) => CustomTypeTemplate::Function {
                arguments: function
                    .argument_types()
                    .iter()
                    .map(custom_type_template)
                    .collect(),
                return_: Box::new(custom_type_template(function.return_())),
            },
            ValueType::Custom(custom) => CustomTypeTemplate::Custom {
                name: custom.type_name().clone(),
                arguments: custom
                    .arguments()
                    .iter()
                    .map(custom_type_template)
                    .collect(),
            },
        }
    }

    #[test]
    fn custom_type_template_preserves_parameter_identity() {
        assert_eq!(
            custom_type_template(&ValueType::Parameter(crate::plan::TypeParameterId(3))),
            CustomTypeTemplate::Parameter(crate::plan::CustomTypeParameterId(3)),
        );
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }

    fn boxed_type() -> CustomType {
        CustomType::new(boxed_name(), Vec::new())
    }

    fn inner_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Inner".into())
    }
}
