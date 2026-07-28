use crate::host::{
    HostCallArguments, HostCallRuntime, HostCustomArgumentSlot, HostCustomToken,
    HostListArgumentSlot, HostListToken, HostProfile, HostScopedValue, HostTupleArgumentSlot,
    HostTupleToken, HostValueArgumentSlot, HostValueFamily, HostValueToken,
};
use crate::plan::execution::host::{HostCallParameter, HostedFunction};
use crate::plan::execution::type_::{ListStorageTypeId, ListTypeId, ValueType};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue,
};
use crate::runtime::graph::{BlockEnvironment, RetainedValues};
use crate::runtime::state::{
    CustomListAllocation, ListValueId, ParameterListValueId, RuntimeStateFor, StoredListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

pub(super) struct RuntimeHostCall<'call, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    plan: &'call Plan,
    state: &'call mut RuntimeStateFor<'run, Plan>,
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    scoped: ScopedValues,
    return_lists: Box<[ListTypeId]>,
    return_customs: Box<[crate::plan::execution::type_::CustomTypeId]>,
    profile: std::marker::PhantomData<Profile>,
}

impl<'call, 'run, Plan, Profile> RuntimeHostCall<'call, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    pub(super) fn new(
        plan: &'call Plan,
        state: &'call mut RuntimeStateFor<'run, Plan>,
        function: &HostedFunction<impl Sized>,
        inputs: RetainedValues,
    ) -> Self {
        let environment = BlockEnvironment::from_retained(inputs);
        let mut arguments = RetainedValues::empty();
        let mut scoped = ScopedValues::default();
        let mut value_arguments = Vec::new();
        let mut list_arguments = Vec::new();
        let mut tuple_arguments = Vec::new();
        let mut custom_arguments = Vec::new();

        for parameter in function.call_parameters() {
            match parameter {
                HostCallParameter::Int(_)
                | HostCallParameter::Float(_)
                | HostCallParameter::String(_)
                | HostCallParameter::BitArray(_)
                | HostCallParameter::UtfCodepoint(_)
                | HostCallParameter::Bool(_)
                | HostCallParameter::Nil(_) => {
                    arguments.push_evaluated(environment.value(&parameter.local()));
                }
                HostCallParameter::Value(_) => {
                    value_arguments.push(scoped.push(environment.value(&parameter.local())));
                }
                HostCallParameter::List(local) => {
                    list_arguments.push(scoped.push_list_value(environment.list(local)));
                }
                HostCallParameter::Tuple(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    tuple_arguments.push(HostTupleToken(token.index));
                }
                HostCallParameter::Custom(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    custom_arguments.push(HostCustomToken(token.index));
                }
            }
        }

        drop(environment);
        state.values_mut().drain_releases();

        let mut return_lists = Vec::new();
        let mut return_customs = Vec::new();
        match function.type_().return_() {
            ValueType::List(type_id) => return_lists.push(type_id.to_owned()),
            ValueType::Custom(type_id) => return_customs.push(type_id.to_owned()),
            ValueType::Parameter(_)
            | ValueType::Int
            | ValueType::Float
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Bool
            | ValueType::Nil
            | ValueType::Tuple(_)
            | ValueType::Function(_) => {}
        }

        Self {
            plan,
            state,
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            scoped,
            return_lists: return_lists.into_boxed_slice(),
            return_customs: return_customs.into_boxed_slice(),
            profile: std::marker::PhantomData,
        }
    }

    pub(super) fn finish<Value: crate::runtime::graph::GraphValue>(
        &self,
        returned: HostValueToken,
        local: &Value,
    ) -> Value::Evaluated {
        let mut retained = RetainedValues::empty();
        self.scoped.retain(returned, &mut retained);
        local.read(&BlockEnvironment::from_retained(retained))
    }

    fn push_scoped(&mut self, value: HostScopedValue) -> HostValueToken {
        self.scoped.push_scoped(value)
    }

    fn value(&self, token: HostValueToken) -> EvaluatedValue {
        self.scoped.value(token)
    }

    fn allocate_list(&mut self, type_id: ListTypeId, values: &[HostValueToken]) -> ListValueId {
        match self.plan.list_storage_type(type_id) {
            ListStorageTypeId::Parameter(type_id) => ParameterListValueId::new(type_id).into(),
            ListStorageTypeId::Int(type_id) => self
                .state
                .values_mut()
                .int(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.ints[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::String(type_id) => self
                .state
                .values_mut()
                .string(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.strings[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::BitArray(type_id) => self
                .state
                .values_mut()
                .bit_array(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.bit_arrays[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::UtfCodepoint(type_id) => self
                .state
                .values_mut()
                .utf_codepoint(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.utf_codepoints[token.index])
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Custom(type_id) => self
                .state
                .values_mut()
                .custom(CustomListAllocation::new(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.customs[token.index].clone())
                        .collect(),
                ))
                .into(),
            ListStorageTypeId::Float(type_id) => self
                .state
                .values_mut()
                .float(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.floats[token.index])
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Bool(type_id) => self
                .state
                .values_mut()
                .bool(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.bools[token.index])
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Nil(type_id) => {
                self.state.values_mut().nil(type_id, values.len()).into()
            }
            ListStorageTypeId::Tuple(type_id) => self
                .state
                .values_mut()
                .tuple(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.tuples[token.index].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::ParameterList(type_id) => self
                .state
                .values_mut()
                .parameter_list_list(type_id, values.len())
                .into(),
            ListStorageTypeId::List(type_id) => self
                .state
                .values_mut()
                .list(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.stored_list_values[token].clone())
                        .collect(),
                )
                .into(),
            ListStorageTypeId::Function(type_id) => self
                .state
                .values_mut()
                .function(
                    type_id,
                    values
                        .iter()
                        .map(|token| self.scoped.functions[token.index].clone())
                        .collect(),
                )
                .into(),
        }
    }
}

impl<'run, Plan, Profile> HostCallRuntime<Profile> for RuntimeHostCall<'_, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    fn state(&mut self) -> &mut Profile::RunState {
        self.state.host_state()
    }

    fn arguments(&self) -> &dyn HostCallArguments {
        &self.arguments
    }

    fn scalar_context(&mut self) -> (&mut Profile::RunState, &dyn HostCallArguments) {
        (self.state.host_state(), &self.arguments)
    }

    fn value(&self, slot: HostValueArgumentSlot) -> HostValueToken {
        self.value_arguments[slot.index()]
    }

    fn list(&self, slot: HostListArgumentSlot) -> HostListToken {
        self.list_arguments[slot.index()]
    }

    fn tuple(&self, slot: HostTupleArgumentSlot) -> HostTupleToken {
        self.tuple_arguments[slot.index()]
    }

    fn custom(&self, slot: HostCustomArgumentSlot) -> HostCustomToken {
        self.custom_arguments[slot.index()]
    }

    fn int(&self, value: HostValueToken) -> BigInt {
        self.scoped.ints[value.index].clone()
    }

    fn float(&self, value: HostValueToken) -> f64 {
        self.scoped.floats[value.index]
    }

    fn string(&self, value: HostValueToken) -> EcoString {
        self.scoped.strings[value.index].clone()
    }

    fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue {
        self.scoped.bit_arrays[value.index].value()
    }

    fn utf_codepoint(&self, value: HostValueToken) -> char {
        self.scoped.utf_codepoints[value.index]
    }

    fn bool(&self, value: HostValueToken) -> bool {
        self.scoped.bools[value.index]
    }

    fn nil(&self, _value: HostValueToken) {}

    fn list_token(&self, value: HostValueToken) -> HostListToken {
        self.scoped.list_tokens[value.index]
    }

    fn tuple_token(&self, value: HostValueToken) -> HostTupleToken {
        HostTupleToken(value.index)
    }

    fn custom_token(&self, value: HostValueToken) -> HostCustomToken {
        HostCustomToken(value.index)
    }

    fn list_len(&self, value: HostListToken) -> usize {
        self.state.values().list_len(&self.scoped.list_value(value))
    }

    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken> {
        let value = self.scoped.list_value(value);
        self.state
            .values()
            .evaluated_value_at(&value, index)
            .map(|value| self.scoped.push(value))
    }

    fn tuple_len(&self, value: HostTupleToken) -> usize {
        self.scoped.tuples[value.0].len()
    }

    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]> {
        self.scoped.tuples[value.0]
            .clone()
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn custom_constructor(&self, value: HostCustomToken) -> usize {
        self.scoped.customs[value.0].constructor().index()
    }

    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]> {
        self.scoped.customs[value.0]
            .fields()
            .to_vec()
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool {
        crate::runtime::evaluated::values_equal(
            self.plan,
            self.state,
            &self.value_from_scoped(left),
            &self.value_from_scoped(right),
        )
    }

    fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
        self.push_scoped(value)
    }

    fn build_list(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.push_scoped(value))
            .collect::<Vec<_>>();
        let list = self.allocate_list(self.return_lists[0], &values);
        self.scoped.push_list(list)
    }

    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.value_from_scoped(value))
            .collect();
        let index = self.scoped.tuples.len();
        self.scoped.tuples.push(values);
        HostValueToken {
            family: HostValueFamily::Tuple,
            index,
        }
    }

    fn build_custom(
        &mut self,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let fields = fields
            .into_vec()
            .into_iter()
            .map(|value| self.value_from_scoped(value))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let constructor = self
            .plan
            .custom_constructor_id(self.return_customs[0], constructor);
        let index = self.scoped.customs.len();
        self.scoped
            .customs
            .push(EvaluatedCustomValue::from_fields(constructor, fields));
        HostValueToken {
            family: HostValueFamily::Custom,
            index,
        }
    }
}

impl<Plan, Profile> RuntimeHostCall<'_, '_, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState>,
    Profile: HostProfile,
{
    fn value_from_scoped(&self, value: HostScopedValue) -> EvaluatedValue {
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
                EvaluatedValue::ParameterList(self.scoped.parameter_lists[index])
            }
            HostScopedValue::List(HostListToken::Stored(index)) => {
                EvaluatedValue::List(self.scoped.lists[index].clone())
            }
            HostScopedValue::Tuple(HostTupleToken(index)) => {
                EvaluatedValue::Tuple(self.scoped.tuples[index].clone())
            }
            HostScopedValue::Custom(HostCustomToken(index)) => {
                EvaluatedValue::Custom(self.scoped.customs[index].clone())
            }
        }
    }
}

#[derive(Default)]
struct ScopedValues {
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
    functions: Vec<EvaluatedFunctionValue>,
}

impl ScopedValues {
    fn retain(&self, token: HostValueToken, retained: &mut RetainedValues) {
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
            HostValueFamily::Function => {
                retained.push_function(self.functions[token.index].clone())
            }
        }
    }

    fn push_list_value(&mut self, value: ListValueId) -> HostListToken {
        match value {
            ListValueId::Parameter(value) => {
                let index = self.parameter_lists.len();
                self.parameter_lists.push(value);
                HostListToken::Parameter(index)
            }
            value => {
                let token = self.push_list(value);
                HostListToken::Stored(token.index)
            }
        }
    }

    fn push(&mut self, value: EvaluatedValue) -> HostValueToken {
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
            EvaluatedValue::Function(value) => {
                let index = self.functions.len();
                self.functions.push(value);
                HostValueToken {
                    family: HostValueFamily::Function,
                    index,
                }
            }
        }
    }

    fn push_list(&mut self, value: ListValueId) -> HostValueToken {
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

    fn push_scoped(&mut self, value: HostScopedValue) -> HostValueToken {
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
        }
    }

    fn value(&self, token: HostValueToken) -> EvaluatedValue {
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
            HostValueFamily::Function => {
                EvaluatedValue::Function(self.functions[token.index].clone())
            }
        }
    }

    fn list_value(&self, token: HostListToken) -> ListValueId {
        match token {
            HostListToken::Parameter(index) => ListValueId::Parameter(self.parameter_lists[index]),
            HostListToken::Stored(index) => self.lists[index].clone().into_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::host::test::{StatelessTestProvider, TestTypeParameter, stateless_identity};
    use crate::{
        BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostList, HostListType,
        HostModule, HostProviderModule, HostProviderSet, HostTupleType, HostTypeList,
        HostTypeListEnd, HostedExecution, ModuleSource, PackageSource, StatelessHostProfile, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

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
