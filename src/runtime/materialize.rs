use super::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture, EvaluatedCaptureKind,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use super::state::{ListValueId, RuntimeState};
use super::{
    BitArrayFunctionValue, BitArrayValue, BoolFunctionValue, CaptureListValue, CaptureValue,
    CustomFieldValue, CustomFunctionValue, CustomFunctionValueTarget, CustomValue,
    FloatFunctionValue, FunctionFunctionValue, FunctionValue, FunctionValueKind, IntFunctionValue,
    ListFunctionValue, ListValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue,
    UtfCodepointFunctionValue, Value,
};
use crate::plan::execution::ExecutionPlan;
use crate::runtime::evaluated::EvaluatedCustomFunctionTarget;

pub(super) fn value(plan: &ExecutionPlan, state: &RuntimeState, value: EvaluatedValue) -> Value {
    match value {
        EvaluatedValue::Int(value) => Value::Int(value),
        EvaluatedValue::Float(value) => Value::Float(value),
        EvaluatedValue::String(value) => Value::String(value),
        EvaluatedValue::BitArray(value) => {
            Value::BitArray(BitArrayValue::from_evaluated(value.bits()))
        }
        EvaluatedValue::UtfCodepoint(value) => Value::UtfCodepoint(value),
        EvaluatedValue::Custom(value) => Value::Custom(custom(plan, state, value)),
        EvaluatedValue::Bool(value) => Value::Bool(value),
        EvaluatedValue::Nil => Value::Nil,
        EvaluatedValue::Tuple(values) => Value::Tuple(
            values
                .into_iter()
                .map(|value| self::value(plan, state, value))
                .collect(),
        ),
        EvaluatedValue::List(value) => Value::List(list(plan, state, &value)),
        EvaluatedValue::Function(value) => Value::Function(function(plan, state, value)),
    }
}

fn list(plan: &ExecutionPlan, state: &RuntimeState, value: &ListValueId) -> ListValue {
    match value {
        ListValueId::Int(value) => ListValue::int(state.int_values(value).to_vec()),
        ListValueId::String(value) => ListValue::string(state.string_values(value).to_vec()),
        ListValueId::BitArray(value) => ListValue::bit_array(
            state
                .bit_array_values(value)
                .iter()
                .map(|value| BitArrayValue::from_evaluated(value.bits()))
                .collect(),
        ),
        ListValueId::UtfCodepoint(value) => {
            ListValue::utf_codepoint(state.utf_codepoint_values(value).to_vec())
        }
        ListValueId::Custom(value) => ListValue::from_evaluated_custom(
            plan.custom_value_type(value.type_id().item_type()),
            state
                .custom_values(value)
                .iter()
                .cloned()
                .map(|value| custom(plan, state, value))
                .collect(),
        ),
        ListValueId::Float(value) => ListValue::float(state.float_values(value).to_vec()),
        ListValueId::Bool(value) => ListValue::bool(state.bool_values(value).to_vec()),
        ListValueId::Nil(value) => ListValue::nil(state.nil_len(value)),
        ListValueId::Tuple(value) => ListValue::from_evaluated_tuple(
            plan.tuple_list_item_type(value.type_id()),
            state
                .tuple_values(value)
                .iter()
                .cloned()
                .map(|values| {
                    values
                        .into_iter()
                        .map(|value| self::value(plan, state, value))
                        .collect()
                })
                .collect(),
        ),
        ListValueId::List(value) => ListValue::from_evaluated_list(
            plan.nested_list_item_type(value.type_id()),
            state
                .list_values(value)
                .iter()
                .cloned()
                .map(|core| {
                    list(
                        plan,
                        state,
                        &ListValueId::from_core(plan, value.type_id().item_type(), core),
                    )
                })
                .collect(),
        ),
        ListValueId::Function(value) => ListValue::from_evaluated_function(
            plan.function_list_item_type(value.type_id()),
            state
                .function_values(value)
                .iter()
                .cloned()
                .map(|value| function(plan, state, value))
                .collect(),
        ),
    }
}

fn function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: EvaluatedFunctionValue,
) -> FunctionValue {
    let kind = match value.kind() {
        EvaluatedFunctionValueKind::Int(value) => {
            FunctionValueKind::Int(int_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Float(value) => {
            FunctionValueKind::Float(float_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::String(value) => {
            FunctionValueKind::String(string_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::BitArray(value) => {
            FunctionValueKind::BitArray(bit_array_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::UtfCodepoint(value) => {
            FunctionValueKind::UtfCodepoint(utf_codepoint_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Custom(value) => {
            FunctionValueKind::Custom(custom_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Bool(value) => {
            FunctionValueKind::Bool(bool_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Nil(value) => {
            FunctionValueKind::Nil(nil_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Tuple(value) => {
            FunctionValueKind::Tuple(tuple_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::List(value) => {
            FunctionValueKind::List(list_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Function(value) => {
            FunctionValueKind::Function(function_function(plan, state, value))
        }
    };
    FunctionValue::from_kind(kind)
}

fn custom(plan: &ExecutionPlan, state: &RuntimeState, value: EvaluatedCustomValue) -> CustomValue {
    let constructor = plan.custom_constructor(value.constructor());
    let fields = value
        .fields()
        .iter()
        .enumerate()
        .map(|(index, value)| {
            CustomFieldValue::from_evaluated(
                constructor.fields()[index].label().cloned(),
                self::value(plan, state, value.clone()),
            )
        })
        .collect();
    CustomValue::from_evaluated(
        plan.custom_value_type(value.type_id()),
        constructor.name().clone(),
        constructor.id().index(),
        fields,
    )
}

fn int_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedIntFunction,
) -> IntFunctionValue {
    IntFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn float_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedFloatFunction,
) -> FloatFunctionValue {
    FloatFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn string_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedStringFunction,
) -> StringFunctionValue {
    StringFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn bit_array_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedBitArrayFunction,
) -> BitArrayFunctionValue {
    BitArrayFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn utf_codepoint_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedUtfCodepointFunction,
) -> UtfCodepointFunctionValue {
    UtfCodepointFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn custom_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedCustomFunction,
) -> CustomFunctionValue {
    let target = match value.runtime_id() {
        EvaluatedCustomFunctionTarget::Function(id) => CustomFunctionValueTarget::Function(id),
        EvaluatedCustomFunctionTarget::Constructor(id) => {
            CustomFunctionValueTarget::Constructor(id)
        }
    };
    CustomFunctionValue::new_with_captures(
        target,
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn bool_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedBoolFunction,
) -> BoolFunctionValue {
    BoolFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn nil_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedNilFunction,
) -> NilFunctionValue {
    NilFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn tuple_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedTupleFunction,
) -> TupleFunctionValue {
    TupleFunctionValue::from_evaluated(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn list_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedListFunction,
) -> ListFunctionValue {
    ListFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn function_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedFunctionFunction,
) -> FunctionFunctionValue {
    FunctionFunctionValue::from_evaluated(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn nested_list_values(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &super::state::ListListValueId,
) -> Vec<ListValue> {
    state
        .list_values(value)
        .iter()
        .cloned()
        .map(|core| {
            list(
                plan,
                state,
                &ListValueId::from_core(plan, value.type_id().item_type(), core),
            )
        })
        .collect()
}

fn captures(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    values: &[EvaluatedCapture],
) -> Vec<CaptureValue> {
    values
        .iter()
        .map(|value| capture(plan, state, value))
        .collect()
}

fn capture(plan: &ExecutionPlan, state: &RuntimeState, value: &EvaluatedCapture) -> CaptureValue {
    match value.kind() {
        EvaluatedCaptureKind::Int { local, value } => CaptureValue::int(*local, value.clone()),
        EvaluatedCaptureKind::Float { local, value } => CaptureValue::float(*local, *value),
        EvaluatedCaptureKind::String { local, value } => {
            CaptureValue::string(*local, value.clone())
        }
        EvaluatedCaptureKind::BitArray { local, value } => {
            CaptureValue::bit_array(*local, BitArrayValue::from_evaluated(value.bits()))
        }
        EvaluatedCaptureKind::UtfCodepoint { local, value } => {
            CaptureValue::utf_codepoint(*local, *value)
        }
        EvaluatedCaptureKind::Custom { local, value } => {
            CaptureValue::custom(*local, custom(plan, state, value.clone()))
        }
        EvaluatedCaptureKind::Bool { local, value } => CaptureValue::bool(*local, *value),
        EvaluatedCaptureKind::Nil { local } => CaptureValue::nil(*local),
        EvaluatedCaptureKind::Tuple { local, value } => CaptureValue::tuple(
            *local,
            value
                .iter()
                .cloned()
                .map(|value| self::value(plan, state, value))
                .collect(),
        ),
        EvaluatedCaptureKind::List(value) => CaptureValue::list(list_capture(plan, state, value)),
        EvaluatedCaptureKind::IntFunction { local, value } => {
            CaptureValue::int_function(*local, int_function(plan, state, value))
        }
        EvaluatedCaptureKind::FloatFunction { local, value } => {
            CaptureValue::float_function(*local, float_function(plan, state, value))
        }
        EvaluatedCaptureKind::StringFunction { local, value } => {
            CaptureValue::string_function(*local, string_function(plan, state, value))
        }
        EvaluatedCaptureKind::BitArrayFunction { local, value } => {
            CaptureValue::bit_array_function(*local, bit_array_function(plan, state, value))
        }
        EvaluatedCaptureKind::UtfCodepointFunction { local, value } => {
            CaptureValue::utf_codepoint_function(*local, utf_codepoint_function(plan, state, value))
        }
        EvaluatedCaptureKind::CustomFunction { local, value } => {
            CaptureValue::custom_function(*local, custom_function(plan, state, value))
        }
        EvaluatedCaptureKind::BoolFunction { local, value } => {
            CaptureValue::bool_function(*local, bool_function(plan, state, value))
        }
        EvaluatedCaptureKind::NilFunction { local, value } => {
            CaptureValue::nil_function(*local, nil_function(plan, state, value))
        }
        EvaluatedCaptureKind::TupleFunction { local, value } => {
            CaptureValue::tuple_function(*local, tuple_function(plan, state, value))
        }
        EvaluatedCaptureKind::ListFunction { local, value } => {
            CaptureValue::list_function(local.clone(), list_function(plan, state, value))
        }
        EvaluatedCaptureKind::FunctionFunction { local, value } => {
            CaptureValue::function_function(*local, function_function(plan, state, value))
        }
    }
}

fn list_capture(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedListCapture,
) -> CaptureListValue {
    match value {
        EvaluatedListCapture::Int { local, value } => CaptureListValue::Int {
            local: *local,
            value: state.int_values(value).to_vec(),
        },
        EvaluatedListCapture::String { local, value } => CaptureListValue::String {
            local: *local,
            value: state.string_values(value).to_vec(),
        },
        EvaluatedListCapture::BitArray { local, value } => CaptureListValue::BitArray {
            local: *local,
            value: state
                .bit_array_values(value)
                .iter()
                .map(|value| BitArrayValue::from_evaluated(value.bits()))
                .collect(),
        },
        EvaluatedListCapture::UtfCodepoint { local, value } => CaptureListValue::UtfCodepoint {
            local: *local,
            value: state.utf_codepoint_values(value).to_vec(),
        },
        EvaluatedListCapture::Custom { local, value } => CaptureListValue::Custom {
            local: *local,
            item_type: plan.custom_value_type(value.type_id().item_type()),
            value: state
                .custom_values(value)
                .iter()
                .cloned()
                .map(|value| custom(plan, state, value))
                .collect(),
        },
        EvaluatedListCapture::Float { local, value } => CaptureListValue::Float {
            local: *local,
            value: state.float_values(value).to_vec(),
        },
        EvaluatedListCapture::Bool { local, value } => CaptureListValue::Bool {
            local: *local,
            value: state.bool_values(value).to_vec(),
        },
        EvaluatedListCapture::Nil { local, value } => CaptureListValue::Nil {
            local: *local,
            len: state.nil_len(value),
        },
        EvaluatedListCapture::Tuple { local, value } => CaptureListValue::Tuple {
            local: *local,
            item_type: plan.tuple_list_item_type(value.type_id()),
            value: state
                .tuple_values(value)
                .iter()
                .cloned()
                .map(|values| {
                    values
                        .into_iter()
                        .map(|value| self::value(plan, state, value))
                        .collect()
                })
                .collect(),
        },
        EvaluatedListCapture::List { local, value } => CaptureListValue::List {
            local: *local,
            item_type: Box::new(plan.nested_list_item_type(value.type_id())),
            value: nested_list_values(plan, state, value),
        },
        EvaluatedListCapture::Function { local, value } => CaptureListValue::Function {
            local: *local,
            item_type: plan.function_list_item_type(value.type_id()),
            value: state
                .function_values(value)
                .iter()
                .cloned()
                .map(|value| function(plan, state, value))
                .collect(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::value;
    use crate::plan::execution::{
        BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
        BoolFunctionId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId, CustomFunctionId,
        CustomFunctionLocalId, CustomListLocalId, CustomLocalId, FloatFunctionId,
        FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionListLocalId, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionId,
        ListFunctionLocal, ListListLocalId, NilFunctionId, NilFunctionLocalId, NilListLocalId,
        NilLocalId, StringFunctionId, StringFunctionLocalId, StringListLocalId, StringLocalId,
        TupleFunctionId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        UtfCodepointFunctionId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
        UtfCodepointLocalId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
        EvaluatedCustomFunction, EvaluatedCustomFunctionTarget, EvaluatedFloatFunction,
        EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedIntFunction,
        EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction, EvaluatedStringFunction,
        EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
    };
    use crate::runtime::state::{ListValueId, RuntimeState};
    use crate::runtime::{
        BitArrayValue, CaptureListValue, CaptureValue, CustomFieldValue, CustomFunctionValue,
        CustomFunctionValueTarget, CustomValue, FunctionValue, ListValue, Value,
    };
    use bitvec::vec::BitVec;

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
    fn utf_codepoints() -> List(UtfCodepoint) { [] }
    pub type Boxed { Boxed(Int) }
    fn custom() -> Boxed { Boxed(1) }
    fn customs() -> List(Boxed) { [] }
    fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn nested_customs() -> List(List(Boxed)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() { 0 }
"#;

    #[test]
    fn materializes_every_runtime_value_and_list_storage_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let mut caller_frame = crate::runtime::frame::Frame::new(
            plan.int_function(IntFunctionId(0)).frame_layout(),
            &mut state,
        );
        let custom_value = crate::runtime::function::run_custom_call(
            &plan,
            &mut state,
            CustomFunctionId(0),
            &[],
            &mut caller_frame,
        )
        .expect("custom constructor function should evaluate");
        let custom_type = plan.custom_value_type(custom_value.type_id());
        let expected_custom_value = CustomValue::from_evaluated(
            custom_type.clone(),
            "Boxed".into(),
            0,
            vec![CustomFieldValue::from_evaluated(None, Value::Int(1.into()))],
        );
        let int_function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );
        let int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = EvaluatedBitArray::new(BitVec::from_vec(vec![1]));
        let bit_array_list = state.bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![bit_array.clone()],
        );
        let utf_codepoint_list = state.utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_list = state.custom(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        );
        let float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into_core()],
        );
        let function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function)],
        );

        let actual = value(
            &plan,
            &state,
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int(1.into()),
                EvaluatedValue::Float(1.5),
                EvaluatedValue::String("one".into()),
                EvaluatedValue::BitArray(bit_array),
                EvaluatedValue::UtfCodepoint('\u{10ffff}'),
                EvaluatedValue::Custom(custom_value),
                EvaluatedValue::Bool(true),
                EvaluatedValue::Nil,
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                EvaluatedValue::List(ListValueId::Int(int_list)),
                EvaluatedValue::List(ListValueId::String(string_list)),
                EvaluatedValue::List(ListValueId::BitArray(bit_array_list)),
                EvaluatedValue::List(ListValueId::UtfCodepoint(utf_codepoint_list)),
                EvaluatedValue::List(ListValueId::Custom(custom_list)),
                EvaluatedValue::List(ListValueId::Float(float_list)),
                EvaluatedValue::List(ListValueId::Bool(bool_list)),
                EvaluatedValue::List(ListValueId::Nil(nil_list)),
                EvaluatedValue::List(ListValueId::Tuple(tuple_list)),
                EvaluatedValue::List(ListValueId::List(nested_list)),
                EvaluatedValue::List(ListValueId::Function(function_list)),
            ]),
        );

        assert_eq!(
            actual,
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Float(1.5),
                Value::String("one".into()),
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::UtfCodepoint('\u{10ffff}'),
                Value::Custom(expected_custom_value.clone()),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(1.into())]),
                Value::List(ListValue::int(vec![1.into()])),
                Value::List(ListValue::string(vec!["one".into()])),
                Value::List(ListValue::bit_array(vec![BitArrayValue::from_bytes(vec![
                    1
                ])])),
                Value::List(ListValue::utf_codepoint(vec!['\u{10ffff}'])),
                Value::List(ListValue::from_evaluated_custom(
                    custom_type,
                    vec![expected_custom_value],
                )),
                Value::List(ListValue::float(vec![1.5])),
                Value::List(ListValue::bool(vec![true])),
                Value::List(ListValue::nil(1)),
                Value::List(ListValue::from_evaluated_tuple(
                    vec![ValueType::Int],
                    vec![vec![Value::Int(1.into())]],
                )),
                Value::List(ListValue::from_evaluated_list(
                    ValueType::Int,
                    vec![ListValue::int(vec![1.into()])],
                )),
                Value::List(ListValue::from_evaluated_function(
                    FunctionType::new(Vec::new(), ValueType::Int),
                    vec![crate::runtime::FunctionValue::new(
                        crate::plan::execution::RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                        FunctionType::new(Vec::new(), ValueType::Int),
                    )],
                )),
            ]),
        );
    }

    #[test]
    fn materializes_every_function_and_capture_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let mut caller_frame = crate::runtime::frame::Frame::new(
            plan.int_function(IntFunctionId(0)).frame_layout(),
            &mut state,
        );
        let custom_value = crate::runtime::function::run_custom_call(
            &plan,
            &mut state,
            CustomFunctionId(0),
            &[],
            &mut caller_frame,
        )
        .expect("custom constructor function should evaluate");
        let custom_type_id = custom_value.type_id();
        let custom_type = plan.custom_value_type(custom_type_id);
        let expected_custom_value = CustomValue::from_evaluated(
            custom_type.clone(),
            "Boxed".into(),
            0,
            vec![CustomFieldValue::from_evaluated(None, Value::Int(1.into()))],
        );
        let constructor_function = EvaluatedCustomFunction::new(
            EvaluatedCustomFunctionTarget::Constructor(custom_value.constructor()),
            vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                vec![crate::plan::execution::ValueType::Int],
                crate::plan::execution::ValueType::Custom(custom_value.type_id()),
            ),
        );
        assert_eq!(
            value(
                &plan,
                &state,
                EvaluatedValue::Function(EvaluatedFunctionValue::from(constructor_function)),
            ),
            Value::Function(FunctionValue::from(CustomFunctionValue::new_with_captures(
                CustomFunctionValueTarget::Constructor(custom_value.constructor()),
                vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
                Vec::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Custom(custom_type.clone())),
            ),)),
        );
        let execution_int_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let module_int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let int_function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let float_function = EvaluatedFloatFunction::new(
            FloatFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Float,
            ),
        );
        let string_function = EvaluatedStringFunction::new(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::String,
            ),
        );
        let bit_array_function = EvaluatedBitArrayFunction::new(
            BitArrayFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::BitArray,
            ),
        );
        let utf_codepoint_function = EvaluatedUtfCodepointFunction::new(
            UtfCodepointFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::UtfCodepoint,
            ),
        );
        let custom_function = EvaluatedCustomFunction::new(
            EvaluatedCustomFunctionTarget::Function(CustomFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Custom(custom_type_id),
            ),
        );
        let bool_function = EvaluatedBoolFunction::new(
            BoolFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Bool,
            ),
        );
        let nil_function = EvaluatedNilFunction::new(
            NilFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Nil,
            ),
        );
        let tuple_function = EvaluatedTupleFunction::new(
            TupleFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Tuple(vec![
                    crate::plan::execution::ValueType::Int,
                ]),
            ),
        );
        let list_function_id = ListFunctionId::Int(plan.int_list_function_id(0));
        let list_function = EvaluatedListFunction::new(
            list_function_id.clone(),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::List(
                    plan.int_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let function_function = EvaluatedFunctionFunction::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Function(Box::new(execution_int_type.clone())),
            ),
        );
        let int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = EvaluatedBitArray::new(BitVec::from_vec(vec![1]));
        let bit_array_list = state.bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![bit_array.clone()],
        );
        let utf_codepoint_list = state.utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_list = state.custom(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        );
        let float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into_core()],
        );
        let function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function.clone())],
        );
        let list_function_local = ListFunctionLocal::Int {
            local: IntListFunctionLocalId(0),
            type_: execution_int_type.clone(),
            list_type: plan.int_list_function_id(0).type_id(),
        };

        let captures = [
            EvaluatedCapture::int(IntLocalId(0), 1.into()),
            EvaluatedCapture::float(FloatLocalId(0), 1.5),
            EvaluatedCapture::string(StringLocalId(0), "one".into()),
            EvaluatedCapture::bit_array(BitArrayLocalId(0), bit_array),
            EvaluatedCapture::utf_codepoint(UtfCodepointLocalId(0), '\u{10ffff}'),
            EvaluatedCapture::custom(CustomLocalId(0), custom_value),
            EvaluatedCapture::bool(BoolLocalId(0), true),
            EvaluatedCapture::nil(NilLocalId(0)),
            EvaluatedCapture::tuple(TupleLocalId(0), vec![EvaluatedValue::Int(1.into())]),
            EvaluatedCapture::list(EvaluatedListCapture::Int {
                local: IntListLocalId(0),
                value: int_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::String {
                local: StringListLocalId(0),
                value: string_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::BitArray {
                local: BitArrayListLocalId(0),
                value: bit_array_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: utf_codepoint_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Custom {
                local: CustomListLocalId(0),
                value: custom_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Float {
                local: FloatListLocalId(0),
                value: float_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Bool {
                local: BoolListLocalId(0),
                value: bool_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Nil {
                local: NilListLocalId(0),
                value: nil_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                local: TupleListLocalId(0),
                value: tuple_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::List {
                local: ListListLocalId(0),
                value: nested_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Function {
                local: FunctionListLocalId(0),
                value: function_list,
            }),
            EvaluatedCapture::int_function(IntFunctionLocalId(0), int_function.clone()),
            EvaluatedCapture::float_function(FloatFunctionLocalId(0), float_function.clone()),
            EvaluatedCapture::string_function(StringFunctionLocalId(0), string_function.clone()),
            EvaluatedCapture::bit_array_function(
                BitArrayFunctionLocalId(0),
                bit_array_function.clone(),
            ),
            EvaluatedCapture::utf_codepoint_function(
                UtfCodepointFunctionLocalId(0),
                utf_codepoint_function.clone(),
            ),
            EvaluatedCapture::custom_function(CustomFunctionLocalId(0), custom_function.clone()),
            EvaluatedCapture::bool_function(BoolFunctionLocalId(0), bool_function.clone()),
            EvaluatedCapture::nil_function(NilFunctionLocalId(0), nil_function.clone()),
            EvaluatedCapture::tuple_function(TupleFunctionLocalId(0), tuple_function.clone()),
            EvaluatedCapture::list_function(list_function_local.clone(), list_function.clone()),
            EvaluatedCapture::function_function(
                FunctionFunctionLocalId(0),
                function_function.clone(),
            ),
        ];
        let expected = [
            CaptureValue::int(IntLocalId(0), 1.into()),
            CaptureValue::float(FloatLocalId(0), 1.5),
            CaptureValue::string(StringLocalId(0), "one".into()),
            CaptureValue::bit_array(BitArrayLocalId(0), BitArrayValue::from_bytes(vec![1])),
            CaptureValue::utf_codepoint(UtfCodepointLocalId(0), '\u{10ffff}'),
            CaptureValue::custom(CustomLocalId(0), expected_custom_value.clone()),
            CaptureValue::bool(BoolLocalId(0), true),
            CaptureValue::nil(NilLocalId(0)),
            CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]),
            CaptureValue::list(CaptureListValue::Int {
                local: IntListLocalId(0),
                value: vec![1.into()],
            }),
            CaptureValue::list(CaptureListValue::String {
                local: StringListLocalId(0),
                value: vec!["one".into()],
            }),
            CaptureValue::list(CaptureListValue::BitArray {
                local: BitArrayListLocalId(0),
                value: vec![BitArrayValue::from_bytes(vec![1])],
            }),
            CaptureValue::list(CaptureListValue::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: vec!['\u{10ffff}'],
            }),
            CaptureValue::list(CaptureListValue::Custom {
                local: CustomListLocalId(0),
                item_type: custom_type.clone(),
                value: vec![expected_custom_value],
            }),
            CaptureValue::list(CaptureListValue::Float {
                local: FloatListLocalId(0),
                value: vec![1.5],
            }),
            CaptureValue::list(CaptureListValue::Bool {
                local: BoolListLocalId(0),
                value: vec![true],
            }),
            CaptureValue::list(CaptureListValue::Nil {
                local: NilListLocalId(0),
                len: 1,
            }),
            CaptureValue::list(CaptureListValue::Tuple {
                local: TupleListLocalId(0),
                item_type: vec![ValueType::Int],
                value: vec![vec![Value::Int(1.into())]],
            }),
            CaptureValue::list(CaptureListValue::List {
                local: ListListLocalId(0),
                item_type: Box::new(ValueType::Int),
                value: vec![ListValue::int(vec![1.into()])],
            }),
            CaptureValue::list(CaptureListValue::Function {
                local: FunctionListLocalId(0),
                item_type: module_int_type.clone(),
                value: vec![crate::runtime::FunctionValue::new(
                    crate::plan::execution::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                    module_int_type.clone(),
                )],
            }),
            CaptureValue::int_function(
                IntFunctionLocalId(0),
                crate::runtime::IntFunctionValue::new(
                    IntFunctionId(0),
                    Vec::new(),
                    module_int_type.clone(),
                ),
            ),
            CaptureValue::float_function(
                FloatFunctionLocalId(0),
                crate::runtime::FloatFunctionValue::new_with_captures(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Float),
                ),
            ),
            CaptureValue::string_function(
                StringFunctionLocalId(0),
                crate::runtime::StringFunctionValue::new_with_captures(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::String),
                ),
            ),
            CaptureValue::bit_array_function(
                BitArrayFunctionLocalId(0),
                crate::runtime::BitArrayFunctionValue::new_with_captures(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::BitArray),
                ),
            ),
            CaptureValue::utf_codepoint_function(
                UtfCodepointFunctionLocalId(0),
                crate::runtime::UtfCodepointFunctionValue::new_with_captures(
                    UtfCodepointFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                ),
            ),
            CaptureValue::custom_function(
                CustomFunctionLocalId(0),
                CustomFunctionValue::new_with_captures(
                    CustomFunctionValueTarget::Function(CustomFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Custom(custom_type)),
                ),
            ),
            CaptureValue::bool_function(
                BoolFunctionLocalId(0),
                crate::runtime::BoolFunctionValue::new_with_captures(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Bool),
                ),
            ),
            CaptureValue::nil_function(
                NilFunctionLocalId(0),
                crate::runtime::NilFunctionValue::new_with_captures(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Nil),
                ),
            ),
            CaptureValue::tuple_function(
                TupleFunctionLocalId(0),
                crate::runtime::TupleFunctionValue::from_evaluated(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                ),
            ),
            CaptureValue::list_function(
                list_function_local,
                crate::runtime::ListFunctionValue::new_with_captures(
                    list_function_id,
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                ),
            ),
            CaptureValue::function_function(
                FunctionFunctionLocalId(0),
                crate::runtime::FunctionFunctionValue::from_evaluated(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Function(Box::new(module_int_type))),
                ),
            ),
        ];

        for (capture, expected) in captures.iter().zip(expected) {
            assert_eq!(super::capture(&plan, &state, capture), expected);
        }

        let functions = [
            EvaluatedFunctionValue::from(int_function),
            EvaluatedFunctionValue::from(float_function),
            EvaluatedFunctionValue::from(string_function),
            EvaluatedFunctionValue::from(bit_array_function),
            EvaluatedFunctionValue::from(utf_codepoint_function),
            EvaluatedFunctionValue::from(custom_function),
            EvaluatedFunctionValue::from(bool_function),
            EvaluatedFunctionValue::from(nil_function),
            EvaluatedFunctionValue::from(tuple_function),
            EvaluatedFunctionValue::from(list_function),
            EvaluatedFunctionValue::from(function_function),
        ];
        for function in functions {
            let expected_type = ValueType::Function(Box::new(plan.function_type(function.type_())));
            let materialized = value(&plan, &state, EvaluatedValue::Function(function));
            assert_eq!(materialized.value_type(), expected_type);
        }
    }

    #[test]
    fn materializes_function_captures_through_the_function_owner() {
        let plan =
            crate::runtime::plan_src("fn identity(value: Int) { value } pub fn main() { 0 }");
        let state = RuntimeState::new();
        let function_type = crate::plan::execution::FunctionType::new(
            vec![crate::plan::execution::ValueType::Int],
            crate::plan::execution::ValueType::Int,
        );
        let function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
            vec![EvaluatedCapture::int(IntLocalId(1), 42.into())],
            function_type,
        );

        assert_eq!(
            value(
                &plan,
                &state,
                EvaluatedValue::Function(EvaluatedFunctionValue::from(function)),
            ),
            Value::Function(crate::runtime::FunctionValue::from(
                crate::runtime::IntFunctionValue::new_with_captures(
                    IntFunctionId(0),
                    vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
                    vec![CaptureValue::int(IntLocalId(1), 42.into())],
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )),
        );
    }
}
