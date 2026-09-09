use super::{PreparedHostCall, ScopedValues, StoredRuntimeList, StoredRuntimeValue};
use crate::host::{
    HostCallArguments, HostCallRuntime, HostCodecScope, HostCustomArgumentSlot, HostCustomToken,
    HostExternalArgumentSlot, HostExternalToken, HostFunctionArgumentSlot, HostFunctionToken,
    HostListArgumentSlot, HostListToken, HostProfile, HostScopedValue, HostTokenRuntime,
    HostTupleArgumentSlot, HostTupleToken, HostValueArgumentSlot, HostValueToken,
};
use crate::plan::execution::host::{HostedFunction, HostedFunctionMetadata};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedExternalValue};
use crate::runtime::graph::{BlockEnvironment, RetainedValues};
use crate::runtime::state::RuntimeStateFor;
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) struct RuntimeHostCall<'call, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedProgram<Profile>: 'run,
{
    plan: &'call crate::plan::execution::HostedProgram<Profile>,
    state: &'call mut RuntimeStateFor<'run, crate::plan::execution::HostedProgram<Profile>>,
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    external_arguments: Vec<HostExternalToken>,
    function_arguments: Vec<HostFunctionToken>,
    scoped: ScopedValues,
    function: &'call std::sync::Arc<HostedFunctionMetadata>,
    origin: crate::runtime::error::HostCallOrigin,
    profile: std::marker::PhantomData<Profile>,
}

impl<'call, 'run, Profile> RuntimeHostCall<'call, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedProgram<Profile>: 'run,
{
    pub(in crate::runtime) fn new(
        plan: &'call crate::plan::execution::HostedProgram<Profile>,
        state: &'call mut RuntimeStateFor<'run, crate::plan::execution::HostedProgram<Profile>>,
        function: &'call HostedFunction<impl Sized>,
        inputs: RetainedValues,
        origin: crate::runtime::error::HostCallOrigin,
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
        } = PreparedHostCall::new(function.call_parameters(), inputs);

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
            function: function.metadata_handle(),
            origin,
            profile: std::marker::PhantomData,
        }
    }

    pub(in crate::runtime) fn new_codec(
        plan: &'call crate::plan::execution::HostedProgram<Profile>,
        state: &'call mut RuntimeStateFor<'run, crate::plan::execution::HostedProgram<Profile>>,
        scope: &'call HostCodecScope,
        origin: crate::runtime::error::HostCallOrigin,
    ) -> Self {
        Self {
            plan,
            state,
            arguments: RetainedValues::empty(),
            value_arguments: Vec::new(),
            list_arguments: Vec::new(),
            tuple_arguments: Vec::new(),
            custom_arguments: Vec::new(),
            external_arguments: Vec::new(),
            function_arguments: Vec::new(),
            scoped: ScopedValues::default(),
            function: scope.function(),
            origin,
            profile: std::marker::PhantomData,
        }
    }

    pub(in crate::runtime) fn finish<Value>(
        &self,
        returned: HostValueToken,
        local: &Value,
    ) -> Value::Evaluated
    where
        Value: crate::runtime::graph::GraphValue,
    {
        let mut retained = RetainedValues::empty();
        self.scoped.retain(returned, &mut retained);
        local.read(&BlockEnvironment::from_retained(retained))
    }
}

impl<Profile> HostTokenRuntime for RuntimeHostCall<'_, '_, Profile>
where
    Profile: HostProfile,
{
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
}

impl<'run, Profile> HostCallRuntime<Profile> for RuntimeHostCall<'_, 'run, Profile>
where
    Profile: HostProfile,
    crate::plan::execution::HostedProgram<Profile>: 'run,
{
    fn state(&mut self) -> &mut Profile::RunState {
        self.state.host_state()
    }

    fn work(&self) -> crate::runtime::work::execution::WorkContext<Profile> {
        self.state.host().work()
    }

    fn origin(&self) -> crate::runtime::HostCallOrigin {
        self.origin.clone()
    }

    fn external_stores(&self) -> &Profile::ExternalStores {
        self.state.host().stores()
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

    fn callable(&self, function: HostFunctionToken) -> crate::runtime::RetainedCallable {
        self.scoped.function(function)
    }

    fn codec_scope(&self) -> HostCodecScope {
        HostCodecScope::new(std::sync::Arc::clone(self.function))
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

    fn take_custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]> {
        self.scoped
            .take_custom_fields(value)
            .into_vec()
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
        function
            .with_value(|function| {
                crate::runtime::function::invoke_callable(
                    self.plan,
                    self.state,
                    function,
                    crate::runtime::error::HostCallOrigin::host(self.function),
                    arguments.into_boxed_slice(),
                )
            })
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

    fn inspect(&self, value: HostScopedValue) -> EcoString {
        crate::runtime::materialize::value(
            self.plan.value_metadata(),
            self.state.lists(),
            self.scoped.value_from_scoped(value),
        )
        .inspect()
        .to_string()
        .into()
    }

    fn stored_equal(&self, left: &StoredRuntimeValue, right: &StoredRuntimeValue) -> bool {
        crate::runtime::evaluated::values_equal(self.state.lists(), left.value(), right.value())
    }

    fn native_equal(
        &self,
        left: &crate::runtime::NativeValue,
        right: &crate::runtime::NativeValue,
    ) -> bool {
        crate::runtime::native::values_equal(self.state.lists(), left, right)
    }

    fn native_hash(&self, value: &crate::runtime::NativeValue) -> u64 {
        crate::runtime::native::value_hash(self.state.lists(), value)
    }

    fn native_tuple(&self, value: HostListToken) -> crate::runtime::NativeValue {
        crate::runtime::NativeValue::tuple_from_list(
            self.scoped.list_value(value),
            self.plan.value_metadata(),
        )
    }

    fn stored_source_hash(&self, value: &StoredRuntimeValue) -> u64 {
        crate::runtime::evaluated::value_source_hash(self.state.lists(), value.value())
    }

    fn stored_inspect(&self, value: &StoredRuntimeValue) -> EcoString {
        crate::runtime::materialize::value(
            self.plan.value_metadata(),
            self.state.lists(),
            value.value().clone(),
        )
        .inspect()
        .to_string()
        .into()
    }

    fn stored_list_len(&self, value: &StoredRuntimeValue) -> usize {
        self.state
            .lists()
            .list_len(&crate::runtime::BorrowedValue::from_stored(value).list())
    }

    fn stored_list_item(
        &self,
        value: &StoredRuntimeValue,
        index: usize,
    ) -> Option<StoredRuntimeValue> {
        self.state
            .lists()
            .evaluated_value_at(
                &crate::runtime::BorrowedValue::from_stored(value).list(),
                index,
            )
            .map(|value| StoredRuntimeValue::new(value, self.plan.value_metadata()))
    }

    fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
        self.scoped.push_scoped(value)
    }

    fn build_list(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        values: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let type_ = type_.resolve_sealed(self.function.type_arguments());
        self.build_native_list(self.function.constructions().list(&type_), values)
    }

    fn build_native_list(
        &mut self,
        type_: crate::plan::execution::type_::ListTypeId,
        values: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.push_scoped(value))
            .collect::<Vec<_>>();
        let storage_type = self.plan.list_storage_type(type_);
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
        type_: &crate::host::HostTypeDescriptor,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let type_ = type_.resolve_sealed(self.function.type_arguments());
        let constructor = self
            .plan
            .custom_constructor_id(self.function.constructions().custom(&type_), constructor);
        self.build_native_custom(constructor, fields)
    }

    fn build_native_custom(
        &mut self,
        constructor: crate::plan::execution::type_::CustomConstructorId,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let fields = fields
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.scoped
            .push_custom(EvaluatedCustomValue::from_fields(constructor, fields))
    }

    fn build_external(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        value: crate::runtime::ExternalPayloadLease,
    ) -> HostExternalToken {
        let type_ = type_.resolve_sealed(self.function.type_arguments());
        self.scoped.push_external(EvaluatedExternalValue::new(
            self.function.constructions().external(&type_),
            value,
        ))
    }

    fn external_lease(&self, value: HostExternalToken) -> crate::runtime::ExternalPayloadLease {
        self.scoped.external(value).lease().clone()
    }

    fn resolve_host_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType> {
        descriptor.resolve(self.function.type_arguments())
    }

    fn retain_stored(&self, value: HostScopedValue) -> StoredRuntimeValue {
        let value = self.scoped.value_from_scoped(value);
        StoredRuntimeValue::new(value, self.plan.value_metadata())
    }

    fn owns_stored(&self, value: &StoredRuntimeValue) -> bool {
        self.plan.value_metadata().shares_owner(value.metadata())
    }

    fn retain_list(&self, value: HostListToken) -> StoredRuntimeList {
        StoredRuntimeList::new(self.scoped.list_value(value))
    }

    fn restore_list(&mut self, value: &StoredRuntimeList) -> HostListToken {
        let value = self.scoped.push_list(value.handle());
        self.scoped.list_token(value)
    }

    fn restore_stored(&mut self, value: &StoredRuntimeValue) -> HostValueToken {
        self.scoped.push(value.value().clone())
    }

    fn callback_inputs(&self, values: Box<[HostScopedValue]>) -> crate::runtime::CallbackInputs {
        let mut inputs = crate::runtime::CallbackInputs::new();
        for value in values {
            inputs.push_value(self.scoped.value_from_scoped(value));
        }
        inputs
    }
}
