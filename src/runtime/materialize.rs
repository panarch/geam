use super::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture, EvaluatedCaptureKind,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedExternalFunction,
    EvaluatedExternalValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedGenericFunction,
    EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use super::state::{ParameterListValueId, RuntimeValueStorage, StoredListValueId};
use super::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, CustomFieldValue,
    CustomFunctionValue, CustomFunctionValueTarget, CustomValue, ExternalFunctionValue,
    ExternalValue, FloatFunctionValue, FunctionFunctionValue, FunctionValue, FunctionValueKind,
    GenericFunctionValue, IntFunctionValue, ListFunctionValue, ListValue, NeverFunctionValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue, Value,
};
use crate::plan::execution::runtime::RuntimeValueMetadata;

pub(super) fn value(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: EvaluatedValue,
) -> Value {
    match value {
        EvaluatedValue::Int(value) => Value::Int(value),
        EvaluatedValue::Float(value) => Value::Float(value),
        EvaluatedValue::String(value) => Value::String(value),
        EvaluatedValue::BitArray(value) => Value::BitArray(value.value()),
        EvaluatedValue::UtfCodepoint(value) => Value::UtfCodepoint(value),
        EvaluatedValue::Custom(value) => Value::Custom(custom(plan, state, value)),
        EvaluatedValue::External(value) => Value::External(external(plan, value)),
        EvaluatedValue::Bool(value) => Value::Bool(value),
        EvaluatedValue::Nil => Value::Nil,
        EvaluatedValue::Tuple(values) => Value::Tuple(
            values
                .into_iter()
                .map(|value| self::value(plan, state, value))
                .collect(),
        ),
        EvaluatedValue::ParameterList(value) => Value::List(parameter_list(value)),
        EvaluatedValue::List(value) => Value::List(list(plan, state, &value)),
        EvaluatedValue::Function(value) => Value::Function(function(plan, state, value)),
    }
}

fn external(plan: RuntimeValueMetadata<'_>, value: EvaluatedExternalValue) -> ExternalValue {
    let (type_id, lease) = value.into_parts();
    ExternalValue::from_evaluated(plan.external_value_type(type_id), lease)
}

fn list(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &StoredListValueId,
) -> ListValue {
    match value {
        StoredListValueId::Int(value) => ListValue::int(state.int_values(value).to_vec()),
        StoredListValueId::String(value) => ListValue::string(state.string_values(value).to_vec()),
        StoredListValueId::BitArray(value) => ListValue::bit_array(
            state
                .bit_array_values(value)
                .iter()
                .map(|value| value.value())
                .collect(),
        ),
        StoredListValueId::UtfCodepoint(value) => {
            ListValue::utf_codepoint(state.utf_codepoint_values(value).to_vec())
        }
        StoredListValueId::Custom(value) => ListValue::from_evaluated_custom(
            plan.custom_value_type(value.type_id().item_type()),
            state
                .custom_values(value)
                .iter()
                .cloned()
                .map(|value| custom(plan, state, value))
                .collect(),
        ),
        StoredListValueId::External(value) => ListValue::from_evaluated_external(
            plan.external_value_type(value.type_id().item_type()),
            state
                .external_values(value)
                .iter()
                .cloned()
                .map(|value| external(plan, value))
                .collect(),
        ),
        StoredListValueId::Float(value) => ListValue::float(state.float_values(value).to_vec()),
        StoredListValueId::Bool(value) => ListValue::bool(state.bool_values(value).to_vec()),
        StoredListValueId::Nil(value) => ListValue::nil(state.nil_len(value)),
        StoredListValueId::Tuple(value) => ListValue::from_evaluated_tuple(
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
        StoredListValueId::ParameterList(value) => {
            let item_type = crate::plan::ValueType::Parameter(value.type_id().item_type().item());
            ListValue::from_evaluated_list(
                item_type.clone(),
                vec![ListValue::empty(item_type); state.parameter_list_list_len(value)],
            )
        }
        StoredListValueId::List(value) => ListValue::from_evaluated_list(
            plan.nested_list_item_type(value.type_id()),
            state
                .list_values(value)
                .iter()
                .map(|value| list(plan, state, value))
                .collect(),
        ),
        StoredListValueId::Function(value) => ListValue::from_evaluated_function(
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

fn parameter_list(value: ParameterListValueId) -> ListValue {
    ListValue::empty(crate::plan::ValueType::Parameter(value.type_id().item()))
}

fn function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: EvaluatedFunctionValue,
) -> FunctionValue {
    let kind = match value.kind() {
        EvaluatedFunctionValueKind::Generic(value) => {
            FunctionValueKind::Generic(generic_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Never(value) => {
            FunctionValueKind::Never(never_function(plan, state, value))
        }
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
        EvaluatedFunctionValueKind::External(value) => {
            FunctionValueKind::External(external_function(plan, state, value))
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

fn custom(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: EvaluatedCustomValue,
) -> CustomValue {
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedIntFunction,
) -> IntFunctionValue {
    IntFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn generic_function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedGenericFunction,
) -> GenericFunctionValue {
    GenericFunctionValue::from_evaluated(
        value.runtime_id().clone(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn never_function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedNeverFunction,
) -> NeverFunctionValue {
    NeverFunctionValue::from_evaluated(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn float_function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedCustomFunction,
) -> CustomFunctionValue {
    let target = match value {
        EvaluatedCustomFunction::Function(value) => {
            CustomFunctionValueTarget::Function(value.runtime_id())
        }
        EvaluatedCustomFunction::Constructor(value) => {
            CustomFunctionValueTarget::Constructor(value.runtime_id())
        }
    };
    CustomFunctionValue::new_with_captures(
        target,
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn external_function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedExternalFunction,
) -> ExternalFunctionValue {
    ExternalFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn bool_function(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
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
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedFunctionFunction,
) -> FunctionFunctionValue {
    let runtime_id = match value {
        EvaluatedFunctionFunction::Core(value) => {
            <std::convert::Infallible as crate::plan::execution::function::ExecutionGraphProfile>::function_function(
                &value.runtime_id(),
            )
        }
        EvaluatedFunctionFunction::External(value) => value.runtime_id().runtime_id(),
    };
    FunctionFunctionValue::from_evaluated(
        runtime_id,
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn nested_list_values(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &super::state::ListListValueId,
) -> Vec<ListValue> {
    state
        .list_values(value)
        .iter()
        .map(|value| list(plan, state, value))
        .collect()
}

fn captures(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    values: &[EvaluatedCapture],
) -> Vec<CaptureValue> {
    values
        .iter()
        .map(|value| capture(plan, state, value))
        .collect()
}

fn capture(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedCapture,
) -> CaptureValue {
    match value.kind() {
        EvaluatedCaptureKind::Int { local, value } => CaptureValue::int(*local, value.clone()),
        EvaluatedCaptureKind::Float { local, value } => CaptureValue::float(*local, *value),
        EvaluatedCaptureKind::String { local, value } => {
            CaptureValue::string(*local, value.clone())
        }
        EvaluatedCaptureKind::BitArray { local, value } => {
            CaptureValue::bit_array(*local, value.value())
        }
        EvaluatedCaptureKind::UtfCodepoint { local, value } => {
            CaptureValue::utf_codepoint(*local, *value)
        }
        EvaluatedCaptureKind::Custom { local, value } => {
            CaptureValue::custom(*local, custom(plan, state, value.clone()))
        }
        EvaluatedCaptureKind::External { local, value } => {
            CaptureValue::external(*local, external(plan, value.clone()))
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
            CaptureValue::custom_function(local.id(), custom_function(plan, state, value))
        }
        EvaluatedCaptureKind::ExternalFunction { local, value } => {
            CaptureValue::external_function(local.id(), external_function(plan, state, value))
        }
        EvaluatedCaptureKind::GenericFunction { local, value } => {
            CaptureValue::generic_function(local.id(), generic_function(plan, state, value))
        }
        EvaluatedCaptureKind::NeverFunction { local, value } => {
            CaptureValue::never_function(local.id(), never_function(plan, state, value))
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
            CaptureValue::function_function(local.clone(), function_function(plan, state, value))
        }
    }
}

fn list_capture(
    plan: RuntimeValueMetadata<'_>,
    state: &RuntimeValueStorage,
    value: &EvaluatedListCapture,
) -> CaptureListValue {
    match value {
        EvaluatedListCapture::Parameter { local, value } => CaptureListValue::Parameter {
            local: *local,
            item_type: value.type_id().item(),
        },
        EvaluatedListCapture::ParameterList { local, value } => CaptureListValue::ParameterList {
            local: *local,
            item_type: value.type_id().item_type().item(),
            len: state.parameter_list_list_len(value),
        },
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
                .map(|value| value.value())
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
        EvaluatedListCapture::External { local, value } => CaptureListValue::External {
            local: *local,
            item_type: plan.external_value_type(value.type_id().item_type()),
            value: state
                .external_values(value)
                .iter()
                .cloned()
                .map(|value| external(plan, value))
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
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, FunctionFunctionId,
        IntFunctionFunctionId, IntFunctionId, ListFunctionId, NilFunctionId,
        ProfiledFunctionFunctionId, RuntimeListFunctionId, StringFunctionId, TupleFunctionId,
        UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomFunctionLocalId,
        CustomListLocalId, CustomLocal, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        FunctionFunctionLocal, FunctionListLocalId, IntFunctionLocalId, IntListFunctionLocalId,
        IntListLocalId, IntLocalId, ListFunctionLocal, ListListLocalId, NilFunctionLocalId,
        NilListLocalId, NilLocalId, ParamLocal, ParamSlot, StringFunctionLocalId,
        StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::{FunctionType, TypeParameterId, ValueType};
    use crate::runtime::evaluated::{
        EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
        EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction, EvaluatedFunction,
        EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedIntFunction,
        EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction, EvaluatedStringFunction,
        EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
    };
    use crate::runtime::state::{CustomListAllocation, ListValueId, RuntimeState};
    use crate::runtime::{
        BitArrayValue, CaptureListValue, CaptureValue, CustomFieldValue, CustomFunctionValue,
        CustomFunctionValueTarget, CustomValue, FunctionValue, ListValue, Value,
    };
    use bitvec::vec::BitVec;

    fn only_param(params: &[ParamSlot]) -> &ParamSlot {
        match params {
            [param] => param,
            _ => panic!("expected exactly one parameter"),
        }
    }

    fn custom_function_local(local: &ParamLocal) -> CustomFunctionLocal {
        match local {
            ParamLocal::CustomFunction(local) => local.clone(),
            _ => panic!("expected a custom-function local"),
        }
    }

    fn function_function_local(local: &ParamLocal) -> FunctionFunctionLocal {
        match local {
            ParamLocal::FunctionFunction(local) => local.clone(),
            _ => panic!("expected a function-function local"),
        }
    }

    fn custom_local(local: &ParamLocal) -> CustomLocal {
        match local {
            ParamLocal::Custom(local) => *local,
            _ => panic!("expected a custom local"),
        }
    }

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
    fn utf_codepoints() -> List(UtfCodepoint) { [] }
    pub type Boxed { Boxed(Int) }
    fn custom() -> Boxed { Boxed(1) }
    fn take_custom(value: Boxed) -> Boxed { value }
    fn customs() -> List(Boxed) { [Boxed(1)] }
    fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn nested_customs() -> List(List(Boxed)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn take_custom_function(value: fn() -> Boxed) { 0 }
fn take_function_function(value: fn() -> fn() -> Int) { 0 }
pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    custom,
    take_custom,
    customs,
    floats,
    bools,
    nils,
    tuples,
    lists,
    nested_customs,
    functions,
    take_custom_function,
    take_function_function,
  )
  0
}
"#;

    #[test]
    fn materializes_every_runtime_value_and_list_storage_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let custom_value = EvaluatedCustomValue::from_fields(
            plan.custom_constructor_id(0, 0),
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let custom_type = plan.custom_value_type(custom_value.type_id());
        let expected_custom_value = CustomValue::from_evaluated(
            custom_type.clone(),
            "Boxed".into(),
            0,
            vec![CustomFieldValue::from_evaluated(None, Value::Int(1.into()))],
        );
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let int_list = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.values_mut().string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = EvaluatedBitArray::new(BitVec::from_vec(vec![1]));
        let bit_array_list = state.values_mut().bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![bit_array.clone()],
        );
        let utf_codepoint_list = state.values_mut().utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_list = state.values_mut().custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        ));
        let float_list = state
            .values_mut()
            .float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state
            .values_mut()
            .bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state
            .values_mut()
            .nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.values_mut().tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.values_mut().list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into()],
        );
        let function_list = state.values_mut().function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function)],
        );

        let actual = value(
            plan.value_metadata(),
            state.values(),
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
                EvaluatedValue::from(ListValueId::Int(int_list)),
                EvaluatedValue::from(ListValueId::String(string_list)),
                EvaluatedValue::from(ListValueId::BitArray(bit_array_list)),
                EvaluatedValue::from(ListValueId::UtfCodepoint(utf_codepoint_list)),
                EvaluatedValue::from(ListValueId::Custom(custom_list)),
                EvaluatedValue::from(ListValueId::Float(float_list)),
                EvaluatedValue::from(ListValueId::Bool(bool_list)),
                EvaluatedValue::from(ListValueId::Nil(nil_list)),
                EvaluatedValue::from(ListValueId::Tuple(tuple_list)),
                EvaluatedValue::from(ListValueId::List(nested_list)),
                EvaluatedValue::from(ListValueId::Function(function_list)),
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
                        crate::plan::execution::function::RuntimeFunctionId::Core(
                            crate::plan::execution::function::CoreRuntimeFunctionId::Int(
                                IntFunctionId(0),
                            ),
                        ),
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
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let custom_value = EvaluatedCustomValue::from_fields(
            plan.custom_constructor_id(0, 0),
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let custom_type_id = custom_value.type_id();
        let custom_type = plan.custom_value_type(custom_type_id);
        let expected_custom_value = CustomValue::from_evaluated(
            custom_type.clone(),
            "Boxed".into(),
            0,
            vec![CustomFieldValue::from_evaluated(None, Value::Int(1.into()))],
        );
        let constructor_function = EvaluatedCustomFunction::constructor(
            custom_value.constructor(),
            crate::plan::execution::type_::FunctionType::new(
                vec![crate::plan::execution::type_::ValueType::Int],
                crate::plan::execution::type_::ValueType::Custom(custom_value.type_id()),
            ),
        );
        assert_eq!(
            value(
                plan.value_metadata(),
                state.values(),
                EvaluatedValue::Function(EvaluatedFunctionValue::from(constructor_function)),
            ),
            Value::Function(FunctionValue::from(CustomFunctionValue::new_with_captures(
                CustomFunctionValueTarget::Constructor(custom_value.constructor()),
                Vec::new(),
                Vec::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Custom(custom_type.clone())),
            ),)),
        );
        let execution_int_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Int,
        );
        let module_int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let float_function = EvaluatedFloatFunction::reference(
            FloatFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Float,
            ),
        );
        let string_function = EvaluatedStringFunction::reference(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::String,
            ),
        );
        let bit_array_function = EvaluatedBitArrayFunction::reference(
            BitArrayFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::BitArray,
            ),
        );
        let utf_codepoint_function = EvaluatedUtfCodepointFunction::reference(
            UtfCodepointFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::UtfCodepoint,
            ),
        );
        let custom_function = EvaluatedCustomFunction::reference(
            plan.custom_function_id(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Custom(custom_type_id),
            ),
        );
        let bool_function = EvaluatedBoolFunction::reference(
            BoolFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Bool,
            ),
        );
        let nil_function = EvaluatedNilFunction::reference(
            NilFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Nil,
            ),
        );
        let tuple_function = EvaluatedTupleFunction::reference(
            TupleFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Tuple(vec![
                    crate::plan::execution::type_::ValueType::Int,
                ]),
            ),
        );
        let list_function_id =
            RuntimeListFunctionId::Core(ListFunctionId::Int(plan.int_list_function_id(0)));
        let list_function = EvaluatedListFunction::reference(
            list_function_id.clone(),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::List(
                    plan.int_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let function_function = EvaluatedFunctionFunction::Core(EvaluatedFunction::reference(
            ProfiledFunctionFunctionId::<std::convert::Infallible>::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Function(Box::new(
                    execution_int_type.clone(),
                )),
            ),
        ));
        let custom_function_owner = plan.int_function(IntFunctionId(1));
        let custom_function_param = only_param(
            custom_function_owner
                .entry()
                .params(custom_function_owner.body()),
        );
        let custom_function_local = custom_function_local(custom_function_param.local());
        let function_function_owner = plan.int_function(IntFunctionId(2));
        let function_function_param = only_param(
            function_function_owner
                .entry()
                .params(function_function_owner.body()),
        );
        let function_function_local = function_function_local(function_function_param.local());
        let int_list = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.values_mut().string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let bit_array = EvaluatedBitArray::new(BitVec::from_vec(vec![1]));
        let bit_array_list = state.values_mut().bit_array(
            plan.bit_array_list_function_id(0).type_id(),
            vec![bit_array.clone()],
        );
        let utf_codepoint_list = state.values_mut().utf_codepoint(
            plan.utf_codepoint_list_function_id(0).type_id(),
            vec!['\u{10ffff}'],
        );
        let custom_list = state.values_mut().custom(CustomListAllocation::new(
            plan.custom_list_function_id(0).type_id(),
            vec![custom_value.clone()],
        ));
        let float_list = state
            .values_mut()
            .float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state
            .values_mut()
            .bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state
            .values_mut()
            .nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.values_mut().tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state
            .values_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.values_mut().list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into()],
        );
        let function_list = state.values_mut().function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function.clone())],
        );
        let list_function_local = ListFunctionLocal::Int {
            local: IntListFunctionLocalId(0),
            type_: execution_int_type.clone(),
            list_type: plan.int_list_function_id(0).type_id(),
        };
        let custom_owner = plan.custom_function(plan.custom_function_id(1));
        let custom_param = only_param(
            custom_owner
                .entry()
                .params(custom_owner.body().function_body()),
        );
        let custom_local = custom_local(custom_param.local());

        let captures = [
            EvaluatedCapture::int(IntLocalId(0), 1.into()),
            EvaluatedCapture::float(FloatLocalId(0), 1.5),
            EvaluatedCapture::string(StringLocalId(0), "one".into()),
            EvaluatedCapture::bit_array(BitArrayLocalId(0), bit_array),
            EvaluatedCapture::utf_codepoint(UtfCodepointLocalId(0), '\u{10ffff}'),
            EvaluatedCapture::custom(custom_local, custom_value),
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
            EvaluatedCapture::custom_function(custom_function_local, custom_function.clone()),
            EvaluatedCapture::bool_function(BoolFunctionLocalId(0), bool_function.clone()),
            EvaluatedCapture::nil_function(NilFunctionLocalId(0), nil_function.clone()),
            EvaluatedCapture::tuple_function(TupleFunctionLocalId(0), tuple_function.clone()),
            EvaluatedCapture::list_function(list_function_local.clone(), list_function.clone()),
            EvaluatedCapture::function_function(
                function_function_local.clone(),
                function_function.clone(),
            ),
        ];
        let expected = [
            CaptureValue::int(IntLocalId(0), 1.into()),
            CaptureValue::float(FloatLocalId(0), 1.5),
            CaptureValue::string(StringLocalId(0), "one".into()),
            CaptureValue::bit_array(BitArrayLocalId(0), BitArrayValue::from_bytes(vec![1])),
            CaptureValue::utf_codepoint(UtfCodepointLocalId(0), '\u{10ffff}'),
            CaptureValue::custom(custom_local, expected_custom_value.clone()),
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
                    crate::plan::execution::function::RuntimeFunctionId::Core(
                        crate::plan::execution::function::CoreRuntimeFunctionId::Int(
                            IntFunctionId(0),
                        ),
                    ),
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
                    CustomFunctionValueTarget::Function(plan.custom_function_id(0)),
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
                function_function_local,
                crate::runtime::FunctionFunctionValue::from_evaluated(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Function(Box::new(module_int_type))),
                ),
            ),
        ];

        for (capture, expected) in captures.iter().zip(expected) {
            assert_eq!(
                super::capture(plan.value_metadata(), state.values(), capture),
                expected,
            );
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
            let materialized = value(
                plan.value_metadata(),
                state.values(),
                EvaluatedValue::Function(function),
            );
            assert_eq!(materialized.value_type(), expected_type);
        }
    }

    #[test]
    fn materializes_function_captures_through_the_function_owner() {
        let plan =
            crate::runtime::plan_src("fn identity(value: Int) { value } pub fn main() { 0 }");
        let mut echo = Vec::new();
        let state = RuntimeState::new(&mut echo);
        let function_type = crate::plan::execution::type_::FunctionType::new(
            vec![crate::plan::execution::type_::ValueType::Int],
            crate::plan::execution::type_::ValueType::Int,
        );
        let function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            vec![crate::plan::execution::graph::ParamLocal::Int(IntLocalId(
                0,
            ))],
            vec![EvaluatedCapture::int(IntLocalId(1), 42.into())],
            function_type,
        );

        assert_eq!(
            value(
                plan.value_metadata(),
                state.values(),
                EvaluatedValue::Function(EvaluatedFunctionValue::from(function)),
            ),
            Value::Function(crate::runtime::FunctionValue::from(
                crate::runtime::IntFunctionValue::new_with_captures(
                    IntFunctionId(0),
                    vec![crate::plan::execution::graph::ParamLocal::Int(IntLocalId(
                        0
                    ))],
                    vec![CaptureValue::int(IntLocalId(1), 42.into())],
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )),
        );
    }

    #[test]
    fn source_materialization_preserves_generic_and_never_function_types() {
        let captured = crate::runtime::run_src(include_str!(
            "../../tests/fixtures/execution/functions/generic_materialized_capture_families.gleam"
        ));
        assert_eq!(
            captured.value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![
                    ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
                    ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                        TypeParameterId(1),
                    ))))),
                    ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Parameter(TypeParameterId(2))],
                        ValueType::Parameter(TypeParameterId(2)),
                    ))),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Parameter(TypeParameterId(3)),
                    ))),
                ]),
            ))),
        );

        let generic = crate::runtime::run_src(include_str!(
            "../../tests/fixtures/execution/functions/generic_function_main.gleam"
        ));
        assert_eq!(
            generic.value_type(),
            ValueType::Tuple(vec![
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(0))],
                    ValueType::Parameter(TypeParameterId(0)),
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(1))],
                    ValueType::Parameter(TypeParameterId(1)),
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(2))],
                    ValueType::Parameter(TypeParameterId(2)),
                ))),
            ]),
        );

        let never = crate::runtime::run_src(include_str!(
            "../../tests/fixtures/execution/functions/generic_never_function_materialization.gleam"
        ));
        assert_eq!(
            never.value_type(),
            ValueType::Tuple(vec![
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Parameter(TypeParameterId(0)),
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Parameter(TypeParameterId(1)),
                    ))),
                ))),
            ]),
        );
    }

    #[test]
    #[should_panic(expected = "expected exactly one parameter")]
    fn single_parameter_guard_rejects_empty_entries() {
        only_param(&[]);
    }

    #[test]
    #[should_panic(expected = "expected a custom-function local")]
    fn custom_function_local_guard_rejects_other_families() {
        custom_function_local(&ParamLocal::Int(IntLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "expected a function-function local")]
    fn function_function_local_guard_rejects_other_families() {
        function_function_local(&ParamLocal::Int(IntLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "expected a custom local")]
    fn custom_local_guard_rejects_other_families() {
        custom_local(&ParamLocal::Int(IntLocalId(0)));
    }
}
