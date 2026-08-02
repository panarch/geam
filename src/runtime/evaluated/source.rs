use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::EvaluatedValue;
use super::function::{
    EvaluatedCustomFunction, EvaluatedFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionIdentity, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
};
use crate::runtime::state::list::{RuntimeListStorage, StoredListValueId};

pub(in crate::runtime) fn values_equal(
    storage: &RuntimeListStorage,
    left: &EvaluatedValue,
    right: &EvaluatedValue,
) -> bool {
    match (left, right) {
        (EvaluatedValue::Int(left), EvaluatedValue::Int(right)) => left == right,
        (EvaluatedValue::Float(left), EvaluatedValue::Float(right)) => left == right,
        (EvaluatedValue::String(left), EvaluatedValue::String(right)) => left == right,
        (EvaluatedValue::BitArray(left), EvaluatedValue::BitArray(right)) => left == right,
        (EvaluatedValue::UtfCodepoint(left), EvaluatedValue::UtfCodepoint(right)) => left == right,
        (EvaluatedValue::Custom(left), EvaluatedValue::Custom(right)) => {
            left.constructor == right.constructor
                && left.fields.len() == right.fields.len()
                && left
                    .fields
                    .iter()
                    .zip(&right.fields)
                    .all(|(left, right)| values_equal(storage, left, right))
        }
        (EvaluatedValue::External(left), EvaluatedValue::External(right)) => {
            let equal = |left: &crate::runtime::StoredRuntimeValue,
                         right: &crate::runtime::StoredRuntimeValue| {
                values_equal(storage, left.value(), right.value())
            };
            left.source_equal(&crate::host::HostExternalEquality::new(&equal), right)
        }
        (EvaluatedValue::Bool(left), EvaluatedValue::Bool(right)) => left == right,
        (EvaluatedValue::Nil, EvaluatedValue::Nil) => true,
        (EvaluatedValue::Tuple(left), EvaluatedValue::Tuple(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| values_equal(storage, left, right))
        }
        (EvaluatedValue::ParameterList(left), EvaluatedValue::ParameterList(right)) => {
            left.type_id() == right.type_id()
        }
        (EvaluatedValue::List(left), EvaluatedValue::List(right)) => {
            lists_equal(storage, left, right)
        }
        (EvaluatedValue::Function(left), EvaluatedValue::Function(right)) => {
            functions_equal(left, right)
        }
        _ => false,
    }
}

pub(in crate::runtime) fn value_source_hash(
    storage: &RuntimeListStorage,
    value: &EvaluatedValue,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    hash_value(storage, value, &mut hasher);
    hasher.finish()
}

fn hash_value(storage: &RuntimeListStorage, value: &EvaluatedValue, hasher: &mut DefaultHasher) {
    match value {
        EvaluatedValue::Int(value) => {
            0u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Float(value) => {
            1u8.hash(hasher);
            if *value == 0.0 {
                0u64.hash(hasher);
            } else {
                value.to_bits().hash(hasher);
            }
        }
        EvaluatedValue::String(value) => {
            2u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::BitArray(value) => {
            3u8.hash(hasher);
            value.bits().len().hash(hasher);
            value.value.bytes().hash(hasher);
        }
        EvaluatedValue::UtfCodepoint(value) => {
            4u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Custom(value) => {
            5u8.hash(hasher);
            value.constructor().hash(hasher);
            value.fields().len().hash(hasher);
            for field in value.fields() {
                hash_value(storage, field, hasher);
            }
        }
        EvaluatedValue::External(value) => {
            6u8.hash(hasher);
            value.type_id().hash(hasher);
            value.source_hash().hash(hasher);
        }
        EvaluatedValue::Bool(value) => {
            7u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedValue::Nil => {
            8u8.hash(hasher);
        }
        EvaluatedValue::Tuple(values) => {
            9u8.hash(hasher);
            values.len().hash(hasher);
            for value in values {
                hash_value(storage, value, hasher);
            }
        }
        EvaluatedValue::ParameterList(value) => {
            10u8.hash(hasher);
            value.type_id().hash(hasher);
        }
        EvaluatedValue::List(value) => {
            11u8.hash(hasher);
            value.list_type().hash(hasher);
            let values = storage.evaluated_values(value);
            values.len().hash(hasher);
            for value in &values {
                hash_value(storage, value, hasher);
            }
        }
        EvaluatedValue::Function(value) => {
            12u8.hash(hasher);
            hash_function(value, hasher);
        }
    }
}

fn hash_function(value: &EvaluatedFunctionValue, hasher: &mut DefaultHasher) {
    match value.kind() {
        EvaluatedFunctionValueKind::Generic(value) => {
            0u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Never(value) => {
            1u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Int(value) => {
            2u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Float(value) => {
            3u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::String(value) => {
            4u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::BitArray(value) => {
            5u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::UtfCodepoint(value) => {
            6u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Custom(value) => {
            7u8.hash(hasher);
            match value {
                EvaluatedCustomFunction::Function(value) => {
                    0u8.hash(hasher);
                    hash_function_identity(&value.identity, hasher);
                }
                EvaluatedCustomFunction::Constructor(value) => {
                    1u8.hash(hasher);
                    hash_function_identity(&value.identity, hasher);
                }
            }
        }
        EvaluatedFunctionValueKind::External(value) => {
            8u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Bool(value) => {
            9u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Nil(value) => {
            10u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Tuple(value) => {
            11u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::List(value) => {
            12u8.hash(hasher);
            hash_function_identity(&value.identity, hasher);
        }
        EvaluatedFunctionValueKind::Function(value) => {
            13u8.hash(hasher);
            hash_function_identity(value.identity(), hasher);
        }
    }
}

fn hash_function_identity(value: &EvaluatedFunctionIdentity, hasher: &mut DefaultHasher) {
    match value {
        EvaluatedFunctionIdentity::Reference(value) => {
            0u8.hash(hasher);
            value.hash(hasher);
        }
        EvaluatedFunctionIdentity::Instance(value) => {
            1u8.hash(hasher);
            value.0.hash(hasher);
        }
    }
}

fn lists_equal(
    storage: &RuntimeListStorage,
    left: &StoredListValueId,
    right: &StoredListValueId,
) -> bool {
    if left.list_type() != right.list_type() {
        return false;
    }

    let left = storage.evaluated_values(left);
    let right = storage.evaluated_values(right);
    left.len() == right.len()
        && left
            .iter()
            .zip(&right)
            .all(|(left, right)| values_equal(storage, left, right))
}

fn functions_equal(left: &EvaluatedFunctionValue, right: &EvaluatedFunctionValue) -> bool {
    match (left.kind(), right.kind()) {
        (EvaluatedFunctionValueKind::Generic(left), EvaluatedFunctionValueKind::Generic(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Never(left), EvaluatedFunctionValueKind::Never(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Int(left), EvaluatedFunctionValueKind::Int(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Float(left), EvaluatedFunctionValueKind::Float(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::String(left), EvaluatedFunctionValueKind::String(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedFunctionValueKind::BitArray(left),
            EvaluatedFunctionValueKind::BitArray(right),
        ) => function_values_equal(left, right),
        (
            EvaluatedFunctionValueKind::UtfCodepoint(left),
            EvaluatedFunctionValueKind::UtfCodepoint(right),
        ) => function_values_equal(left, right),
        (EvaluatedFunctionValueKind::Custom(left), EvaluatedFunctionValueKind::Custom(right)) => {
            custom_function_values_equal(left, right)
        }
        (
            EvaluatedFunctionValueKind::External(left),
            EvaluatedFunctionValueKind::External(right),
        ) => function_values_equal(left, right),
        (EvaluatedFunctionValueKind::Bool(left), EvaluatedFunctionValueKind::Bool(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Nil(left), EvaluatedFunctionValueKind::Nil(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Tuple(left), EvaluatedFunctionValueKind::Tuple(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::List(left), EvaluatedFunctionValueKind::List(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedFunctionValueKind::Function(left),
            EvaluatedFunctionValueKind::Function(right),
        ) => function_function_values_equal(left, right),
        _ => false,
    }
}

fn function_values_equal<Id>(left: &EvaluatedFunction<Id>, right: &EvaluatedFunction<Id>) -> bool {
    left.identity == right.identity
}

fn custom_function_values_equal(
    left: &EvaluatedCustomFunction,
    right: &EvaluatedCustomFunction,
) -> bool {
    match (left, right) {
        (EvaluatedCustomFunction::Function(left), EvaluatedCustomFunction::Function(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedCustomFunction::Constructor(left),
            EvaluatedCustomFunction::Constructor(right),
        ) => function_values_equal(left, right),
        _ => false,
    }
}

fn function_function_values_equal(
    left: &EvaluatedFunctionFunction,
    right: &EvaluatedFunctionFunction,
) -> bool {
    left.identity() == right.identity()
}

#[cfg(test)]
mod tests {
    use super::super::function::{
        EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
        EvaluatedFloatFunction, EvaluatedFunction, EvaluatedFunctionFunction,
        EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction,
        EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction,
        EvaluatedTupleFunction, EvaluatedUtfCodepointFunction,
    };
    use super::super::{
        EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedValue,
    };
    use super::{value_source_hash, values_equal};
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionFunctionId, IntFunctionId,
        ListFunctionId, NeverFunctionId, NilFunctionId, ProfiledFunctionFunctionId,
        RuntimeListFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
    };
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::{ListValueId, ParameterListValueId};
    use bitvec::order::Msb0;

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { [] }
fn custom() -> Boxed { Boxed(1) }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn parameters(values: List(value)) { values }
fn parameter_lists(values: List(List(value))) { values }
fn take_function_function(value: fn() -> fn() -> Int) { 0 }
pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    customs,
    custom,
    floats,
    bools,
    nils,
    tuples,
    lists,
    functions,
    take_function_function,
  )
  let _ = parameters([])
  let _ = parameter_lists([[]])
  0
}
"#;
    #[test]
    fn semantic_value_equality_covers_every_list_and_function_family() {
        fn external_equal(
            context: &crate::host::HostExternalEquality<'_>,
            left: &crate::host::HostStoredValue<num_bigint::BigInt>,
            right: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> bool {
            context.stored_values_equal(left, right)
        }

        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let execution_int_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Int,
        );
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let custom_type = plan.custom_list_function_id(0).type_id().item_type();
        let custom_function = EvaluatedCustomFunction::reference(
            plan.custom_function_id(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Custom(custom_type),
            ),
        );
        let constructor_id = plan.custom_constructor_id(0, 0);
        let constructor = plan.custom_constructor(constructor_id);
        let constructor_function = EvaluatedCustomFunction::constructor(
            constructor_id,
            crate::plan::execution::type_::FunctionType::new(
                constructor
                    .fields()
                    .iter()
                    .map(|field| field.type_().clone())
                    .collect(),
                crate::plan::execution::type_::ValueType::Custom(constructor_id.type_id()),
            ),
        );
        let never_function = EvaluatedNeverFunction::reference(
            NeverFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Parameter(crate::plan::TypeParameterId(
                    0,
                )),
            ),
        );
        let function_pairs = [
            (
                EvaluatedFunctionValue::from(never_function.clone()),
                EvaluatedFunctionValue::from(never_function),
            ),
            (
                EvaluatedFunctionValue::from(int_function.clone()),
                EvaluatedFunctionValue::from(int_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::String,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::String,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::BitArray,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::BitArray,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::UtfCodepoint,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::UtfCodepoint,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(custom_function.clone()),
                EvaluatedFunctionValue::from(custom_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(constructor_function.clone()),
                EvaluatedFunctionValue::from(constructor_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Bool,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Bool,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Nil,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Nil,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Tuple(vec![
                            crate::plan::execution::type_::ValueType::Int,
                        ]),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Tuple(vec![
                            crate::plan::execution::type_::ValueType::Int,
                        ]),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    RuntimeListFunctionId::Core(ListFunctionId::Int(plan.int_list_function_id(0))),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    RuntimeListFunctionId::Core(ListFunctionId::Int(plan.int_list_function_id(0))),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::Core(
                    EvaluatedFunction::reference(
                        ProfiledFunctionFunctionId::<std::convert::Infallible>::Int(
                            IntFunctionFunctionId(0),
                        ),
                        Vec::new(),
                        Vec::new(),
                        crate::plan::execution::type_::FunctionType::new(
                            Vec::new(),
                            crate::plan::execution::type_::ValueType::Function(Box::new(
                                execution_int_type.clone(),
                            )),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::Core(
                    EvaluatedFunction::reference(
                        ProfiledFunctionFunctionId::<std::convert::Infallible>::Int(
                            IntFunctionFunctionId(0),
                        ),
                        Vec::new(),
                        Vec::new(),
                        crate::plan::execution::type_::FunctionType::new(
                            Vec::new(),
                            crate::plan::execution::type_::ValueType::Function(Box::new(
                                execution_int_type.clone(),
                            )),
                        ),
                    ),
                )),
            ),
        ];

        for (left, right) in function_pairs {
            let family = left.kind().family();
            assert_eq!(family, right.kind().family());
            let left = EvaluatedValue::Function(left);
            let right = EvaluatedValue::Function(right);
            assert!(values_equal(state.lists(), &left, &right,));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(custom_function)),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(constructor_function)),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(int_function.clone())),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::type_::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::type_::ValueType::Float,
                    ),
                ),
            )),
        ));

        let int_lists = (
            state
                .lists_mut()
                .int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
            state
                .lists_mut()
                .int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
        );
        let string_lists = (
            state.lists_mut().string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
            state.lists_mut().string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
        );
        let float_lists = (
            state
                .lists_mut()
                .float(plan.float_list_function_id(0).type_id(), vec![1.5]),
            state
                .lists_mut()
                .float(plan.float_list_function_id(0).type_id(), vec![1.5]),
        );
        let utf_codepoint_lists = (
            state
                .lists_mut()
                .utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
            state
                .lists_mut()
                .utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
        );
        let bool_lists = (
            state
                .lists_mut()
                .bool(plan.bool_list_function_id(0).type_id(), vec![true]),
            state
                .lists_mut()
                .bool(plan.bool_list_function_id(0).type_id(), vec![true]),
        );
        let nil_lists = (
            state
                .lists_mut()
                .nil(plan.nil_list_function_id(0).type_id(), 1),
            state
                .lists_mut()
                .nil(plan.nil_list_function_id(0).type_id(), 1),
        );
        let tuple_lists = (
            state.lists_mut().tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
            state.lists_mut().tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
        );
        let left_child = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let right_child = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_lists = (
            state.lists_mut().list(
                plan.list_list_function_id(0).type_id(),
                vec![left_child.into()],
            ),
            state.lists_mut().list(
                plan.list_list_function_id(0).type_id(),
                vec![right_child.into()],
            ),
        );
        let function_lists = (
            state.lists_mut().function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
            state.lists_mut().function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
        );
        let list_pairs = [
            (
                ListValueId::Int(int_lists.0.clone()),
                ListValueId::Int(int_lists.1.clone()),
            ),
            (
                ListValueId::String(string_lists.0.clone()),
                ListValueId::String(string_lists.1.clone()),
            ),
            (
                ListValueId::UtfCodepoint(utf_codepoint_lists.0.clone()),
                ListValueId::UtfCodepoint(utf_codepoint_lists.1.clone()),
            ),
            (
                ListValueId::Float(float_lists.0.clone()),
                ListValueId::Float(float_lists.1.clone()),
            ),
            (
                ListValueId::Bool(bool_lists.0.clone()),
                ListValueId::Bool(bool_lists.1.clone()),
            ),
            (
                ListValueId::Nil(nil_lists.0.clone()),
                ListValueId::Nil(nil_lists.1.clone()),
            ),
            (
                ListValueId::Tuple(tuple_lists.0.clone()),
                ListValueId::Tuple(tuple_lists.1.clone()),
            ),
            (
                ListValueId::List(nested_lists.0.clone()),
                ListValueId::List(nested_lists.1.clone()),
            ),
            (
                ListValueId::Function(function_lists.0.clone()),
                ListValueId::Function(function_lists.1.clone()),
            ),
        ];

        for (left, right) in list_pairs {
            let left = EvaluatedValue::from(left);
            let right = EvaluatedValue::from(right);
            assert!(values_equal(state.lists(), &left, &right,));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }

        let bit_array = EvaluatedBitArray::new(bitvec::bitvec![u8, Msb0; 1, 0, 1]);
        let scalar_and_compound_pairs = vec![
            (EvaluatedValue::Int(1.into()), EvaluatedValue::Int(1.into())),
            (EvaluatedValue::Float(0.0), EvaluatedValue::Float(-0.0)),
            (EvaluatedValue::Float(1.5), EvaluatedValue::Float(1.5)),
            (
                EvaluatedValue::String("one".into()),
                EvaluatedValue::String("one".into()),
            ),
            (
                EvaluatedValue::BitArray(bit_array.clone()),
                EvaluatedValue::BitArray(bit_array),
            ),
            (
                EvaluatedValue::UtfCodepoint('A'),
                EvaluatedValue::UtfCodepoint('A'),
            ),
            (EvaluatedValue::Bool(true), EvaluatedValue::Bool(true)),
            (EvaluatedValue::Nil, EvaluatedValue::Nil),
            (
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            ),
            (
                EvaluatedValue::ParameterList(ParameterListValueId::new(
                    plan.parameter_list_function_id(0).type_id(),
                )),
                EvaluatedValue::ParameterList(ParameterListValueId::new(
                    plan.parameter_list_function_id(0).type_id(),
                )),
            ),
            (
                EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                    constructor_id,
                    vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
                )),
                EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                    constructor_id,
                    vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
                )),
            ),
        ];
        for (left, right) in scalar_and_compound_pairs {
            assert!(values_equal(state.lists(), &left, &right));
            assert_eq!(
                value_source_hash(state.lists(), &left),
                value_source_hash(state.lists(), &right),
            );
        }

        let external_store = crate::host::HostExternalStore::default();
        let first = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            external_equal,
            41,
            "External(7)".into(),
        );
        let equal = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            external_equal,
            41,
            "External(7)".into(),
        );
        let collision = external_store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(8.into()),
            ),
            external_equal,
            41,
            "External(8)".into(),
        );
        let external_type = crate::plan::execution::type_::ExternalTypeId::new(0);
        let first = EvaluatedValue::External(EvaluatedExternalValue::new(external_type, first));
        let equal = EvaluatedValue::External(EvaluatedExternalValue::new(external_type, equal));
        let collision =
            EvaluatedValue::External(EvaluatedExternalValue::new(external_type, collision));
        assert!(values_equal(state.lists(), &first, &equal));
        assert_eq!(
            value_source_hash(state.lists(), &first),
            value_source_hash(state.lists(), &equal),
        );
        assert!(!values_equal(state.lists(), &first, &collision));
        assert_eq!(
            value_source_hash(state.lists(), &first),
            value_source_hash(state.lists(), &collision),
        );
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::from(ListValueId::Int(int_lists.0)),
            &EvaluatedValue::from(ListValueId::String(string_lists.0)),
        ));
        assert!(values_equal(
            state.lists(),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            &EvaluatedValue::Tuple(Vec::new()),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Int(1.into()),
            &EvaluatedValue::String("one".into()),
        ));
    }
}
