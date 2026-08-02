mod invoke;
mod scoped;

pub(super) use self::invoke::{invoke_never, invoke_value};

use self::scoped::ScopedValues;
pub(crate) use self::scoped::StoredRuntimeValue;
use crate::host::{
    ExternalPayloadLease, HostCallArguments, HostCallRuntime, HostCustomArgumentSlot,
    HostCustomToken, HostExternalArgumentSlot, HostExternalToken, HostFunctionArgumentSlot,
    HostFunctionToken, HostListArgumentSlot, HostListToken, HostProfile, HostScopedValue,
    HostTupleArgumentSlot, HostTupleToken, HostValueArgumentSlot, HostValueToken,
};
use crate::plan::execution::host::{HostCallParameter, HostedFunction};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::plan::execution::type_::{ListTypeId, ValueType};
use crate::runtime::evaluated::EvaluatedCustomValue;
use crate::runtime::graph::{BlockEnvironment, RetainedValues};
use crate::runtime::state::RuntimeStateFor;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) struct RuntimeHostCall<'call, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    plan: &'call crate::plan::execution::HostedExecution<Profile>,
    state: &'call mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    external_arguments: Vec<HostExternalToken>,
    function_arguments: Vec<HostFunctionToken>,
    scoped: ScopedValues,
    type_arguments: &'call [crate::plan::ValueType],
    return_lists: Box<[ListTypeId]>,
    return_customs: Box<[crate::plan::execution::type_::CustomTypeId]>,
    return_externals: Box<[crate::plan::execution::type_::ExternalTypeId]>,
    origin: crate::runtime::error::HostCallOrigin,
    profile: std::marker::PhantomData<Profile>,
}

struct PreparedHostCall {
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    external_arguments: Vec<HostExternalToken>,
    function_arguments: Vec<HostFunctionToken>,
    scoped: ScopedValues,
    return_lists: Box<[ListTypeId]>,
    return_customs: Box<[crate::plan::execution::type_::CustomTypeId]>,
    return_externals: Box<[crate::plan::execution::type_::ExternalTypeId]>,
}

impl<'call, 'run, Profile> RuntimeHostCall<'call, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    pub(super) fn new(
        plan: &'call crate::plan::execution::HostedExecution<Profile>,
        state: &'call mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
        function: &'call HostedFunction<impl Sized>,
        inputs: RetainedValues,
    ) -> Self {
        let PreparedHostCall {
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            external_arguments,
            function_arguments,
            scoped,
            return_lists,
            return_customs,
            return_externals,
        } = PreparedHostCall::new(
            function.call_parameters(),
            function.type_().return_(),
            inputs,
        );
        state.lists_mut().drain_releases();

        Self {
            plan,
            state,
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            external_arguments,
            function_arguments,
            scoped,
            type_arguments: function.type_arguments(),
            return_lists,
            return_customs,
            return_externals,
            origin: crate::runtime::error::HostCallOrigin::host(function.metadata()),
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
}

impl PreparedHostCall {
    fn new(
        parameters: &[HostCallParameter],
        return_type: &ValueType,
        inputs: RetainedValues,
    ) -> Self {
        let environment = BlockEnvironment::from_retained(inputs);
        let mut arguments = RetainedValues::empty();
        let mut scoped = ScopedValues::default();
        let mut value_arguments = Vec::new();
        let mut list_arguments = Vec::new();
        let mut tuple_arguments = Vec::new();
        let mut custom_arguments = Vec::new();
        let mut external_arguments = Vec::new();
        let mut function_arguments = Vec::new();

        for parameter in parameters {
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
                HostCallParameter::List(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    list_arguments.push(scoped.list_token(token));
                }
                HostCallParameter::Tuple(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    tuple_arguments.push(scoped.tuple_token(token));
                }
                HostCallParameter::Custom(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    custom_arguments.push(scoped.custom_token(token));
                }
                HostCallParameter::External(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    external_arguments.push(scoped.external_token(token));
                }
                HostCallParameter::Function(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    function_arguments.push(scoped.function_token(token));
                }
            }
        }

        let mut return_lists = Vec::new();
        let mut return_customs = Vec::new();
        let mut return_externals = Vec::new();
        match return_type {
            ValueType::List(type_id) => return_lists.push(type_id.to_owned()),
            ValueType::Custom(type_id) => return_customs.push(type_id.to_owned()),
            ValueType::External(type_id) => return_externals.push(type_id.to_owned()),
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
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            external_arguments,
            function_arguments,
            scoped,
            return_lists: return_lists.into_boxed_slice(),
            return_customs: return_customs.into_boxed_slice(),
            return_externals: return_externals.into_boxed_slice(),
        }
    }
}

impl<'run, Profile> HostCallRuntime<Profile> for RuntimeHostCall<'_, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    fn state(&mut self) -> &mut Profile::RunState {
        self.state.host_state()
    }

    fn external_stores(&self) -> &Profile::ExternalStores {
        self.plan.external_stores()
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

    fn external(&self, slot: HostExternalArgumentSlot) -> HostExternalToken {
        self.external_arguments[slot.index()]
    }

    fn function(&self, slot: HostFunctionArgumentSlot) -> HostFunctionToken {
        self.function_arguments[slot.index()]
    }

    fn int(&self, value: HostValueToken) -> BigInt {
        self.scoped.int(value)
    }

    fn float(&self, value: HostValueToken) -> f64 {
        self.scoped.float(value)
    }

    fn string(&self, value: HostValueToken) -> EcoString {
        self.scoped.string(value)
    }

    fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue {
        self.scoped.bit_array(value)
    }

    fn utf_codepoint(&self, value: HostValueToken) -> char {
        self.scoped.utf_codepoint(value)
    }

    fn bool(&self, value: HostValueToken) -> bool {
        self.scoped.bool(value)
    }

    fn nil(&self, _value: HostValueToken) {}

    fn list_token(&self, value: HostValueToken) -> HostListToken {
        self.scoped.list_token(value)
    }

    fn tuple_token(&self, value: HostValueToken) -> HostTupleToken {
        self.scoped.tuple_token(value)
    }

    fn custom_token(&self, value: HostValueToken) -> HostCustomToken {
        self.scoped.custom_token(value)
    }

    fn external_token(&self, value: HostValueToken) -> HostExternalToken {
        self.scoped.external_token(value)
    }

    fn function_token(&self, value: HostValueToken) -> HostFunctionToken {
        self.scoped.function_token(value)
    }

    fn list_len(&self, value: HostListToken) -> usize {
        self.state.lists().list_len(&self.scoped.list_value(value))
    }

    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken> {
        let value = self.scoped.list_value(value);
        self.state
            .lists()
            .evaluated_value_at(&value, index)
            .map(|value| self.scoped.push(value))
    }

    fn tuple_len(&self, value: HostTupleToken) -> usize {
        self.scoped.tuple_len(value)
    }

    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]> {
        self.scoped
            .tuple_values(value)
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn custom_constructor(&self, value: HostCustomToken) -> usize {
        self.scoped.custom_constructor(value)
    }

    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]> {
        self.scoped
            .custom_fields(value)
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn invoke(
        &mut self,
        function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, crate::HostCallError> {
        let function = self.scoped.function(function);
        let arguments = arguments
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect::<Vec<_>>();
        let mut inputs = RetainedValues::empty();
        for value in &arguments {
            inputs.push_evaluated(value.clone());
        }
        crate::runtime::function::invoke_callable(
            self.plan,
            self.state,
            function,
            self.origin.clone(),
            inputs,
            arguments.into_boxed_slice(),
        )
        .map(|value| self.scoped.push(value))
        .map_err(crate::HostCallError::nested)
    }

    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool {
        crate::runtime::evaluated::values_equal(
            self.state.lists(),
            &self.scoped.value_from_scoped(left),
            &self.scoped.value_from_scoped(right),
        )
    }

    fn source_hash(&self, value: HostScopedValue) -> u64 {
        crate::runtime::evaluated::value_source_hash(
            self.state.lists(),
            &self.scoped.value_from_scoped(value),
        )
    }

    fn stored_value_hash(&self, value: &StoredRuntimeValue) -> u64 {
        crate::runtime::evaluated::value_source_hash(self.state.lists(), value.value())
    }

    fn stored_value_inspection(&self, value: &StoredRuntimeValue) -> EcoString {
        crate::runtime::materialize::value(
            self.plan.value_metadata(),
            self.state.lists(),
            value.value().clone(),
        )
        .inspect()
        .to_string()
        .into()
    }

    fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
        self.scoped.push_scoped(value)
    }

    fn build_list(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.push_scoped(value))
            .collect::<Vec<_>>();
        let storage_type = self.plan.list_storage_type(self.return_lists[0]);
        let list = self
            .scoped
            .allocate_list(storage_type, self.state.lists_mut(), &values);
        self.scoped.push_list(list)
    }

    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect();
        self.scoped.push_tuple(values)
    }

    fn build_custom(
        &mut self,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let fields = fields
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let constructor = self
            .plan
            .custom_constructor_id(self.return_customs[0], constructor);
        self.scoped
            .push_custom(EvaluatedCustomValue::from_fields(constructor, fields))
    }

    fn build_external(&mut self, value: ExternalPayloadLease) -> HostExternalToken {
        self.scoped
            .push_external(crate::runtime::evaluated::EvaluatedExternalValue::new(
                self.return_externals[0],
                value,
            ))
    }

    fn external_lease(&self, value: HostExternalToken) -> ExternalPayloadLease {
        self.scoped.external(value).lease().clone()
    }

    fn resolve_host_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType> {
        descriptor.resolve(self.type_arguments)
    }

    fn retain_stored(&self, value: HostScopedValue) -> StoredRuntimeValue {
        let value = self.scoped.value_from_scoped(value);
        let type_ = value.value_type(self.plan.value_metadata());
        StoredRuntimeValue::new(value, type_)
    }

    fn restore_stored(&mut self, value: &StoredRuntimeValue) -> HostValueToken {
        self.scoped.push(value.value().clone())
    }
}
