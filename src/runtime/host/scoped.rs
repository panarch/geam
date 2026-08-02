use crate::host::{
    HostCustomToken, HostExternalToken, HostFunctionToken, HostListToken, HostScopedValue,
    HostTupleToken, HostValueFamily, HostValueToken,
};
use crate::plan::execution::type_::ListStorageTypeId;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedFunctionValueKind, EvaluatedGenericFunction, EvaluatedValue,
};
use crate::runtime::function::InvocableFunctionValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::list::{
    CustomListAllocation, ExternalListAllocation, ListValueId, ParameterListValueId,
    RuntimeListStorage, StoredListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct ScopedValues {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<EvaluatedBitArray>,
    utf_codepoints: Vec<char>,
    bools: Vec<bool>,
    parameter_lists: Vec<ParameterListValueId>,
    lists: Vec<StoredListValueId>,
    list_tokens: Vec<HostListToken>,
    stored_list_values: HashMap<HostValueToken, StoredListValueId>,
    tuples: Vec<Vec<EvaluatedValue>>,
    customs: Vec<EvaluatedCustomValue>,
    externals: Vec<EvaluatedExternalValue>,
    functions: Vec<InvocableFunctionValue>,
    symbolic_functions: Vec<EvaluatedGenericFunction>,
    function_values: HashMap<HostValueToken, EvaluatedFunctionValue>,
}

pub(crate) struct StoredRuntimeValue {
    value: EvaluatedValue,
    type_: crate::plan::ValueType,
}

impl StoredRuntimeValue {
    pub(in crate::runtime) fn new(value: EvaluatedValue, type_: crate::plan::ValueType) -> Self {
        Self { value, type_ }
    }

    #[cfg(test)]
    pub(crate) fn test_int(value: BigInt) -> Self {
        Self::new(EvaluatedValue::Int(value), crate::plan::ValueType::Int)
    }

    pub(in crate::runtime) fn value(&self) -> &EvaluatedValue {
        &self.value
    }

    pub(crate) fn type_(&self) -> &crate::plan::ValueType {
        &self.type_
    }
}

impl ScopedValues {
    pub(super) fn retain(&self, token: HostValueToken, retained: &mut RetainedValues) {
        match token.family {
            HostValueFamily::Int => retained.push_int(self.ints[token.index].clone()),
            HostValueFamily::Float => retained.push_float(self.floats[token.index]),
            HostValueFamily::String => retained.push_string(self.strings[token.index].clone()),
            HostValueFamily::BitArray => {
                retained.push_bit_array(self.bit_arrays[token.index].clone())
            }
            HostValueFamily::UtfCodepoint => {
                retained.push_utf_codepoint(self.utf_codepoints[token.index])
            }
            HostValueFamily::Bool => retained.push_bool(self.bools[token.index]),
            HostValueFamily::Nil => retained.push_nil(),
            HostValueFamily::List => {
                retained.push_list(self.list_value(self.list_tokens[token.index]))
            }
            HostValueFamily::Tuple => retained.push_tuple(self.tuples[token.index].clone()),
            HostValueFamily::Custom => retained.push_custom(self.customs[token.index].clone()),
            HostValueFamily::External => {
                retained.push_external(self.externals[token.index].clone())
            }
            HostValueFamily::Function => {
                retained.push_function(self.functions[token.index].clone().into_evaluated())
            }
            HostValueFamily::SymbolicFunction => {
                retained.push_function(self.symbolic_functions[token.index].clone().into())
            }
        }
    }

    pub(super) fn push(&mut self, value: EvaluatedValue) -> HostValueToken {
        match value {
            EvaluatedValue::Int(value) => {
                let index = self.ints.len();
                self.ints.push(value);
                HostValueToken {
                    family: HostValueFamily::Int,
                    index,
                }
            }
            EvaluatedValue::Float(value) => {
                let index = self.floats.len();
                self.floats.push(value);
                HostValueToken {
                    family: HostValueFamily::Float,
                    index,
                }
            }
            EvaluatedValue::String(value) => {
                let index = self.strings.len();
                self.strings.push(value);
                HostValueToken {
                    family: HostValueFamily::String,
                    index,
                }
            }
            EvaluatedValue::BitArray(value) => {
                let index = self.bit_arrays.len();
                self.bit_arrays.push(value);
                HostValueToken {
                    family: HostValueFamily::BitArray,
                    index,
                }
            }
            EvaluatedValue::UtfCodepoint(value) => {
                let index = self.utf_codepoints.len();
                self.utf_codepoints.push(value);
                HostValueToken {
                    family: HostValueFamily::UtfCodepoint,
                    index,
                }
            }
            EvaluatedValue::Custom(value) => {
                let index = self.customs.len();
                self.customs.push(value);
                HostValueToken {
                    family: HostValueFamily::Custom,
                    index,
                }
            }
            EvaluatedValue::External(value) => {
                let index = self.externals.len();
                self.externals.push(value);
                HostValueToken {
                    family: HostValueFamily::External,
                    index,
                }
            }
            EvaluatedValue::Bool(value) => {
                let index = self.bools.len();
                self.bools.push(value);
                HostValueToken {
                    family: HostValueFamily::Bool,
                    index,
                }
            }
            EvaluatedValue::Nil => HostValueToken {
                family: HostValueFamily::Nil,
                index: 0,
            },
            EvaluatedValue::Tuple(value) => {
                let index = self.tuples.len();
                self.tuples.push(value);
                HostValueToken {
                    family: HostValueFamily::Tuple,
                    index,
                }
            }
            EvaluatedValue::ParameterList(value) => self.push_list(ListValueId::Parameter(value)),
            EvaluatedValue::List(value) => {
                let token = self.push_stored(value);
                self.value_for_list(token)
            }
            EvaluatedValue::Function(value) => self.push_function(value),
        }
    }

    pub(super) fn push_list(&mut self, value: ListValueId) -> HostValueToken {
        let token = match value {
            ListValueId::Parameter(value) => {
                let index = self.parameter_lists.len();
                self.parameter_lists.push(value);
                HostListToken::Parameter(index)
            }
            ListValueId::Int(value) => self.push_stored(value.into()),
            ListValueId::String(value) => self.push_stored(value.into()),
            ListValueId::BitArray(value) => self.push_stored(value.into()),
            ListValueId::UtfCodepoint(value) => self.push_stored(value.into()),
            ListValueId::Custom(value) => self.push_stored(value.into()),
            ListValueId::External(value) => self.push_stored(value.into()),
            ListValueId::Float(value) => self.push_stored(value.into()),
            ListValueId::Bool(value) => self.push_stored(value.into()),
            ListValueId::Nil(value) => self.push_stored(value.into()),
            ListValueId::Tuple(value) => self.push_stored(value.into()),
            ListValueId::ParameterList(value) => self.push_stored(value.into()),
            ListValueId::List(value) => self.push_stored(value.into()),
            ListValueId::Function(value) => self.push_stored(value.into()),
        };
        self.value_for_list(token)
    }

    fn push_stored(&mut self, value: StoredListValueId) -> HostListToken {
        let index = self.lists.len();
        self.lists.push(value);
        HostListToken::Stored(index)
    }

    fn value_for_list(&mut self, value: HostListToken) -> HostValueToken {
        let index = self.list_tokens.len();
        self.list_tokens.push(value);
        let token = HostValueToken {
            family: HostValueFamily::List,
            index,
        };
        if let HostListToken::Stored(index) = value {
            self.stored_list_values
                .insert(token, self.lists[index].clone());
        }
        token
    }

    pub(super) fn push_scoped(&mut self, value: HostScopedValue) -> HostValueToken {
        match value {
            HostScopedValue::Int(value) => self.push(EvaluatedValue::Int(value)),
            HostScopedValue::Float(value) => self.push(EvaluatedValue::Float(value)),
            HostScopedValue::String(value) => self.push(EvaluatedValue::String(value)),
            HostScopedValue::BitArray(value) => self.push(EvaluatedValue::BitArray(
                EvaluatedBitArray::from_value(value),
            )),
            HostScopedValue::UtfCodepoint(value) => self.push(EvaluatedValue::UtfCodepoint(value)),
            HostScopedValue::Bool(value) => self.push(EvaluatedValue::Bool(value)),
            HostScopedValue::Nil => self.push(EvaluatedValue::Nil),
            HostScopedValue::Value(token) => token,
            HostScopedValue::List(value) => self.value_for_list(value),
            HostScopedValue::Tuple(HostTupleToken(index)) => HostValueToken {
                family: HostValueFamily::Tuple,
                index,
            },
            HostScopedValue::Custom(HostCustomToken(index)) => HostValueToken {
                family: HostValueFamily::Custom,
                index,
            },
            HostScopedValue::External(HostExternalToken(index)) => HostValueToken {
                family: HostValueFamily::External,
                index,
            },
            HostScopedValue::Function(HostFunctionToken(index)) => HostValueToken {
                family: HostValueFamily::Function,
                index,
            },
        }
    }

    pub(super) fn value(&self, token: HostValueToken) -> EvaluatedValue {
        match token.family {
            HostValueFamily::Int => EvaluatedValue::Int(self.ints[token.index].clone()),
            HostValueFamily::Float => EvaluatedValue::Float(self.floats[token.index]),
            HostValueFamily::String => EvaluatedValue::String(self.strings[token.index].clone()),
            HostValueFamily::BitArray => {
                EvaluatedValue::BitArray(self.bit_arrays[token.index].clone())
            }
            HostValueFamily::UtfCodepoint => {
                EvaluatedValue::UtfCodepoint(self.utf_codepoints[token.index])
            }
            HostValueFamily::Bool => EvaluatedValue::Bool(self.bools[token.index]),
            HostValueFamily::Nil => EvaluatedValue::Nil,
            HostValueFamily::List => {
                EvaluatedValue::from(self.list_value(self.list_tokens[token.index]))
            }
            HostValueFamily::Tuple => EvaluatedValue::Tuple(self.tuples[token.index].clone()),
            HostValueFamily::Custom => EvaluatedValue::Custom(self.customs[token.index].clone()),
            HostValueFamily::External => {
                EvaluatedValue::External(self.externals[token.index].clone())
            }
            HostValueFamily::Function => {
                EvaluatedValue::Function(self.functions[token.index].clone().into_evaluated())
            }
            HostValueFamily::SymbolicFunction => {
                EvaluatedValue::Function(self.symbolic_functions[token.index].clone().into())
            }
        }
    }

    pub(super) fn list_value(&self, token: HostListToken) -> ListValueId {
        match token {
            HostListToken::Parameter(index) => ListValueId::Parameter(self.parameter_lists[index]),
            HostListToken::Stored(index) => self.lists[index].clone().into_value(),
        }
    }

    fn push_function(&mut self, value: EvaluatedFunctionValue) -> HostValueToken {
        let (family, index) = match value.kind() {
            EvaluatedFunctionValueKind::Generic(function) => {
                let index = self.symbolic_functions.len();
                self.symbolic_functions.push(function.clone());
                (HostValueFamily::SymbolicFunction, index)
            }
            EvaluatedFunctionValueKind::Never(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Never(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Int(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Int(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Float(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Float(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::String(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::String(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::BitArray(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::BitArray(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::UtfCodepoint(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::UtfCodepoint(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Custom(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Custom(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::External(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::External(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Bool(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Bool(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Nil(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Nil(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Tuple(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Tuple(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::List(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::List(function.clone()));
                (HostValueFamily::Function, index)
            }
            EvaluatedFunctionValueKind::Function(function) => {
                let index = self.functions.len();
                self.functions
                    .push(InvocableFunctionValue::Function(function.clone()));
                (HostValueFamily::Function, index)
            }
        };
        let token = HostValueToken { family, index };
        self.function_values.insert(token, value);
        token
    }
}

impl ScopedValues {
    pub(super) fn allocate_list(
        &self,
        storage_type: ListStorageTypeId,
        storage: &mut RuntimeListStorage,
        values: &[HostValueToken],
    ) -> ListValueId {
        match storage_type {
            ListStorageTypeId::Parameter(type_id) => ParameterListValueId::new(type_id).into(),
            ListStorageTypeId::Int(type_id) => storage
                .int(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.ints[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::String(type_id) => storage
                .string(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.strings[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::BitArray(type_id) => storage
                .bit_array(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.bit_arrays[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::UtfCodepoint(type_id) => storage
                .utf_codepoint(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.utf_codepoints[token.index])
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Custom(type_id) => storage
                .custom(CustomListAllocation::new(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.customs[token.index].clone())
                        .collect(),
                ))
                .into(),
            ListStorageTypeId::External(type_id) => storage
                .external(ExternalListAllocation::new(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.externals[token.index].clone())
                        .collect(),
                ))
                .into(),
            ListStorageTypeId::Float(type_id) => storage
                .float(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.floats[token.index])
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Bool(type_id) => storage
                .bool(
                    type_id,
                    values.iter().map(|token| self.bools[token.index]).collect(),
                )
                .into(),
            ListStorageTypeId::Nil(type_id) => storage.nil(type_id, values.len()).into(),
            ListStorageTypeId::Tuple(type_id) => storage
                .tuple(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.tuples[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::ParameterList(type_id) => {
                storage.parameter_list_list(type_id, values.len()).into()
            }
            ListStorageTypeId::List(type_id) => storage
                .list(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.stored_list_values[token].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Function(type_id) => storage
                .function(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.function_values[token].clone())
                        .collect(),
                )
                .into(),
        }
    }

    pub(super) fn value_from_scoped(&self, value: HostScopedValue) -> EvaluatedValue {
        match value {
            HostScopedValue::Int(value) => EvaluatedValue::Int(value),
            HostScopedValue::Float(value) => EvaluatedValue::Float(value),
            HostScopedValue::String(value) => EvaluatedValue::String(value),
            HostScopedValue::BitArray(value) => {
                EvaluatedValue::BitArray(EvaluatedBitArray::from_value(value))
            }
            HostScopedValue::UtfCodepoint(value) => EvaluatedValue::UtfCodepoint(value),
            HostScopedValue::Bool(value) => EvaluatedValue::Bool(value),
            HostScopedValue::Nil => EvaluatedValue::Nil,
            HostScopedValue::Value(token) => self.value(token),
            HostScopedValue::List(HostListToken::Parameter(index)) => {
                EvaluatedValue::ParameterList(self.parameter_lists[index])
            }
            HostScopedValue::List(HostListToken::Stored(index)) => {
                EvaluatedValue::List(self.lists[index].clone())
            }
            HostScopedValue::Tuple(HostTupleToken(index)) => {
                EvaluatedValue::Tuple(self.tuples[index].clone())
            }
            HostScopedValue::Custom(HostCustomToken(index)) => {
                EvaluatedValue::Custom(self.customs[index].clone())
            }
            HostScopedValue::External(HostExternalToken(index)) => {
                EvaluatedValue::External(self.externals[index].clone())
            }
            HostScopedValue::Function(HostFunctionToken(index)) => {
                EvaluatedValue::Function(self.functions[index].clone().into_evaluated())
            }
        }
    }

    pub(super) fn list_token(&self, value: HostValueToken) -> HostListToken {
        self.list_tokens[value.index]
    }

    pub(super) fn tuple_token(&self, value: HostValueToken) -> HostTupleToken {
        HostTupleToken(value.index)
    }

    pub(super) fn custom_token(&self, value: HostValueToken) -> HostCustomToken {
        HostCustomToken(value.index)
    }

    pub(super) fn external_token(&self, value: HostValueToken) -> HostExternalToken {
        HostExternalToken(value.index)
    }

    pub(super) fn function_token(&self, value: HostValueToken) -> HostFunctionToken {
        HostFunctionToken(value.index)
    }

    pub(super) fn int(&self, value: HostValueToken) -> BigInt {
        self.ints[value.index].clone()
    }

    pub(super) fn float(&self, value: HostValueToken) -> f64 {
        self.floats[value.index]
    }

    pub(super) fn string(&self, value: HostValueToken) -> EcoString {
        self.strings[value.index].clone()
    }

    pub(super) fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue {
        self.bit_arrays[value.index].value()
    }

    pub(super) fn utf_codepoint(&self, value: HostValueToken) -> char {
        self.utf_codepoints[value.index]
    }

    pub(super) fn bool(&self, value: HostValueToken) -> bool {
        self.bools[value.index]
    }

    pub(super) fn tuple_len(&self, value: HostTupleToken) -> usize {
        self.tuples[value.0].len()
    }

    pub(super) fn tuple_values(&self, value: HostTupleToken) -> Vec<EvaluatedValue> {
        self.tuples[value.0].clone()
    }

    pub(super) fn custom_constructor(&self, value: HostCustomToken) -> usize {
        self.customs[value.0].constructor().index()
    }

    pub(super) fn custom_fields(&self, value: HostCustomToken) -> Vec<EvaluatedValue> {
        self.customs[value.0].fields().to_vec()
    }

    pub(super) fn function(&self, value: HostFunctionToken) -> InvocableFunctionValue {
        self.functions[value.0].clone()
    }

    pub(super) fn push_tuple(&mut self, values: Vec<EvaluatedValue>) -> HostValueToken {
        let index = self.tuples.len();
        self.tuples.push(values);
        HostValueToken {
            family: HostValueFamily::Tuple,
            index,
        }
    }

    pub(super) fn push_custom(&mut self, value: EvaluatedCustomValue) -> HostValueToken {
        let index = self.customs.len();
        self.customs.push(value);
        HostValueToken {
            family: HostValueFamily::Custom,
            index,
        }
    }

    pub(super) fn push_external(&mut self, value: EvaluatedExternalValue) -> HostExternalToken {
        let index = self.externals.len();
        self.externals.push(value);
        HostExternalToken(index)
    }

    pub(super) fn external(&self, value: HostExternalToken) -> &EvaluatedExternalValue {
        &self.externals[value.0]
    }
}

#[cfg(test)]
mod tests {
    use super::{ScopedValues, StoredRuntimeValue};
    use crate::host::test::{StatelessTestProvider, TestTypeParameter, stateless_identity};
    use crate::runtime::EvaluatedValue;
    use crate::{
        BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostList, HostListType,
        HostModule, HostProviderModule, HostProviderSet, HostTupleType, HostTypeList,
        HostTypeListEnd, HostedExecution, ModuleSource, PackageSource, StatelessHostProfile, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn stored_runtime_value_preserves_its_exact_type_and_restores_its_value() {
        let stored =
            StoredRuntimeValue::new(EvaluatedValue::Int(7.into()), crate::plan::ValueType::Int);
        let mut scoped = ScopedValues::default();

        let restored = scoped.push(stored.value().clone());

        assert_eq!(stored.type_(), &crate::plan::ValueType::Int);
        assert_eq!(scoped.int(restored), BigInt::from(7));
    }

    #[test]
    fn runtime_host_call_reads_scoped_compound_values() {
        type Seventh = HostTypeList<(), HostTypeListEnd>;
        type Sixth = HostTypeList<bool, Seventh>;
        type Fifth = HostTypeList<char, Sixth>;
        type Fourth = HostTypeList<BitArrayValue, Fifth>;
        type Third = HostTypeList<EcoString, Fourth>;
        type Second = HostTypeList<f64, Third>;
        type Elements = HostTypeList<BigInt, Second>;
        type Tuple = HostTupleType<Elements>;
        type Values = HostListType<Tuple>;

        fn inspect<'call>(
            mut call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, bool>,
            values: HostList<'call, Tuple>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            let value = call
                .list_item(values, 0)
                .expect("tuple list should contain one value");
            let (int, (float, (string, (bits, (codepoint, (bool_, (nil, ()))))))) =
                call.tuple_values(value);
            let matches = call.tuple_len(value) == 7
                && call.equal::<Tuple>(value, value)
                && call.equal::<BigInt>(int, 1.into())
                && call.equal::<f64>(float, 1.5)
                && call.equal::<EcoString>(string, "text".into())
                && call.equal::<BitArrayValue>(bits, BitArrayValue::from_bytes(vec![1]))
                && call.equal::<char>(codepoint, 'A')
                && call.equal::<bool>(bool_, true)
                && call.equal::<()>(nil, ());
            Ok(call.return_value(matches))
        }

        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<StatelessTestProvider, (Values,), bool, _>("inspect", inspect)
            .expect("tuple-list provider should be valid")
            .with_scoped_function::<
                StatelessTestProvider,
                (TestTypeParameter,),
                TestTypeParameter,
                _,
            >("identity", stateless_identity)
            .expect("tuple-list provider should be valid");
        let source = r#"
@external(erlang, "host", "inspect")
fn inspect(
  values: List(#(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil)),
) -> Bool

@external(erlang, "host", "identity")
fn identity(value: value) -> value

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  inspect(identity([#(1, 1.5, "text", <<1>>, codepoint, True, Nil)]))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let expected = Value::Bool(true);

        assert_eq!(execution.run_main(&mut (), &mut Vec::new()), Ok(expected),);
    }
}
