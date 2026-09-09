use crate::host::{
    HostCallArguments, HostCallCompletion, HostConstruction, HostCustom, HostCustomArgumentSlot,
    HostCustomConstructor, HostCustomType, HostExternal, HostExternalArgumentSlot,
    HostExternalBinding, HostExternalPayloadBuilder, HostExternalPayloadView, HostExternalSchema,
    HostExternalStorage, HostExternalType, HostFunctionArgumentSlot, HostList,
    HostListArgumentSlot, HostListType, HostStoredValue, HostTuple, HostTupleArgumentSlot,
    HostTupleType, HostType, HostTypeSequence, HostValue, HostValueArgumentSlot,
};
use crate::runtime::{StoredRuntimeList, StoredRuntimeValue};
use std::marker::PhantomData;

mod codec;
mod runtime;

pub(crate) use codec::HostCodecScope;

#[cfg(test)]
pub(crate) use runtime::test;
pub(crate) use runtime::{HostCallRuntime, HostTokenRuntime};

pub trait HostProfile: Send + Sync + 'static {
    type RunState: Send;
    type ExternalStores: Default + Send + 'static;
}

pub trait HostProvider<Profile: HostProfile>: Send + Sync + 'static {
    type State: Send;

    fn project(state: &mut Profile::RunState) -> &mut Self::State;
}

type BoundExternalStorage<Profile, Provider, Schema> =
    <Provider as HostExternalBinding<Profile, Schema>>::Storage;
type BoundExternalPayload<Profile, Provider, Schema> = <BoundExternalStorage<
    Profile,
    Provider,
    Schema,
> as HostExternalStorage<Profile, Schema>>::Payload;

#[derive(Debug, Clone, Copy, Default)]
pub struct StatelessHostProfile;

pub struct HostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub(in crate::host) runtime: &'call mut dyn HostCallRuntime<Profile>,
    marker: PhantomData<(Provider, Return)>,
}

impl HostProfile for StatelessHostProfile {
    type RunState = ();
    type ExternalStores = ();
}

impl<'call, Profile, Provider, Return> HostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub(crate) fn new(runtime: &'call mut dyn HostCallRuntime<Profile>) -> Self {
        Self {
            runtime,
            marker: PhantomData,
        }
    }

    pub fn state(&mut self) -> &mut Provider::State {
        Provider::project(self.runtime.state())
    }

    pub fn return_value(self, value: Return::Value<'call>) -> HostCallCompletion<'call, Return> {
        HostCallCompletion::new(
            self.runtime
                .complete(crate::host::type_::into_scoped::<Return>(value)),
        )
    }

    pub fn equal<Type: HostType>(
        &self,
        left: Type::Value<'call>,
        right: Type::Value<'call>,
    ) -> bool {
        self.runtime.equal(
            crate::host::type_::into_scoped::<Type>(left),
            crate::host::type_::into_scoped::<Type>(right),
        )
    }

    /// Hashes a call-scoped value consistently with Gleam source equality.
    ///
    /// The result is intended for runtime lookup within this execution. It is
    /// not a stable serialization or a process-independent value.
    pub fn source_hash<Type: HostType>(&self, value: Type::Value<'call>) -> u64 {
        self.runtime
            .source_hash(crate::host::type_::into_scoped::<Type>(value))
    }

    /// Returns the canonical Gleam-facing inspection of a call-scoped value.
    pub fn inspect<Type: HostType>(&self, value: Type::Value<'call>) -> ecow::EcoString {
        self.runtime
            .inspect(crate::host::type_::into_scoped::<Type>(value))
    }

    pub(crate) fn arguments(&self) -> &dyn HostCallArguments {
        self.runtime.arguments()
    }

    pub(crate) fn value<Type>(&self, slot: HostValueArgumentSlot) -> HostValue<'call, Type> {
        HostValue::new(self.runtime.value(slot))
    }

    pub(crate) fn list<Item>(&self, slot: HostListArgumentSlot) -> HostList<'call, Item> {
        HostList::new(self.runtime.list(slot))
    }

    pub(crate) fn tuple<Elements>(
        &self,
        slot: HostTupleArgumentSlot,
    ) -> HostTuple<'call, Elements> {
        HostTuple::new(self.runtime.tuple(slot))
    }

    pub(crate) fn custom<Custom>(&self, slot: HostCustomArgumentSlot) -> HostCustom<'call, Custom> {
        HostCustom::new(self.runtime.custom(slot))
    }

    pub(crate) fn external<Type>(
        &self,
        slot: HostExternalArgumentSlot,
    ) -> HostExternal<'call, Type> {
        HostExternal::new(self.runtime.external(slot))
    }

    pub(crate) fn function<Arguments, FunctionReturn>(
        &self,
        slot: HostFunctionArgumentSlot,
    ) -> crate::host::HostCallable<'call, Arguments, FunctionReturn> {
        crate::host::HostCallable::new(self.runtime.function(slot))
    }

    pub fn list_len<Item>(&self, value: HostList<'call, Item>) -> usize {
        self.runtime.list_len(value.token)
    }

    pub fn list_item<Item: HostType>(
        &mut self,
        value: HostList<'call, Item>,
        index: usize,
    ) -> Option<Item::Value<'call>> {
        self.runtime
            .list_item(value.token, index)
            .map(|token| crate::host::type_::from_token::<Item, Profile>(self.runtime, token))
    }

    /// Constructs a list authorized by one registered construction token.
    pub fn construct_list<Item: HostType>(
        &mut self,
        _construction: HostConstruction<'call, HostListType<Item>>,
        values: impl IntoIterator<Item = Item::Value<'call>>,
    ) -> HostList<'call, Item> {
        let values = values
            .into_iter()
            .map(crate::host::type_::into_scoped::<Item>)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let value = self.runtime.build_list(
            &crate::host::HostTypeDescriptor::of::<HostListType<Item>>(),
            values,
        );
        HostList::new(self.runtime.list_token(value))
    }

    pub fn tuple_len<Elements>(&self, value: HostTuple<'call, Elements>) -> usize {
        self.runtime.tuple_len(value.token)
    }

    pub fn tuple_values<Elements: HostTypeSequence>(
        &mut self,
        value: HostTuple<'call, Elements>,
    ) -> Elements::Values<'call> {
        let values = self.runtime.tuple_values(value.token);
        crate::host::type_::from_tokens::<Elements, Profile>(self.runtime, &values)
    }

    /// Constructs a tuple authorized by one registered construction token.
    pub fn construct_tuple<Elements: HostTypeSequence>(
        &mut self,
        _construction: HostConstruction<'call, HostTupleType<Elements>>,
        values: Elements::Values<'call>,
    ) -> HostTuple<'call, Elements> {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Elements>(values, &mut output);
        let value = self.runtime.build_tuple(output.into_boxed_slice());
        HostTuple::new(self.runtime.tuple_token(value))
    }

    pub fn custom_constructor<Custom>(&self, value: HostCustom<'call, Custom>) -> usize {
        self.runtime.custom_constructor(value.token)
    }

    pub fn custom_fields<Constructor>(
        &mut self,
        value: HostCustom<'call, Constructor::Custom>,
    ) -> Option<<Constructor::Fields as HostTypeSequence>::Values<'call>>
    where
        Constructor: HostCustomConstructor,
    {
        if self.runtime.custom_constructor(value.token)
            != crate::host::type_::custom_constructor_index::<Constructor>()
        {
            return None;
        }
        let fields = self.runtime.custom_fields(value.token);
        Some(crate::host::type_::from_tokens::<
            Constructor::Fields,
            Profile,
        >(self.runtime, &fields))
    }

    #[doc(hidden)]
    pub fn provider_custom_fields<Constructor>(
        &mut self,
        value: HostCustom<'call, Constructor::Custom>,
    ) -> Option<<Constructor::Fields as HostTypeSequence>::Values<'call>>
    where
        Constructor: HostCustomConstructor,
    {
        if self.runtime.custom_constructor(value.token)
            != crate::host::type_::custom_constructor_index::<Constructor>()
        {
            return None;
        }
        let fields = self.runtime.take_custom_fields(value.token);
        Some(crate::host::type_::from_tokens::<
            Constructor::Fields,
            Profile,
        >(self.runtime, &fields))
    }

    /// Takes the fields of the constructor left after generated code has
    /// excluded every preceding constructor in the linked schema.
    #[doc(hidden)]
    pub fn provider_remaining_custom_fields<Constructor>(
        &mut self,
        value: HostCustom<'call, Constructor::Custom>,
    ) -> <Constructor::Fields as HostTypeSequence>::Values<'call>
    where
        Constructor: HostCustomConstructor,
    {
        let fields = self.runtime.take_custom_fields(value.token);
        crate::host::type_::from_tokens::<Constructor::Fields, Profile>(self.runtime, &fields)
    }

    /// Constructs an ordinary custom value authorized by one registered type token.
    pub fn construct_custom<Constructor>(
        &mut self,
        _construction: HostConstruction<'call, Constructor::Custom>,
        fields: <Constructor::Fields as HostTypeSequence>::Values<'call>,
    ) -> HostCustom<'call, Constructor::Custom>
    where
        Constructor: HostCustomConstructor,
        Constructor::Fields: HostTypeSequence,
    {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Constructor::Fields>(fields, &mut output);
        let value = self.runtime.build_custom(
            &crate::host::HostTypeDescriptor::of::<Constructor::Custom>(),
            crate::host::type_::custom_constructor_index::<Constructor>(),
            output.into_boxed_slice(),
        );
        HostCustom::new(self.runtime.custom_token(value))
    }

    /// Borrows the Rust payload behind one typed external value.
    pub fn external_payload<Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> HostExternalPayloadView<'call, BoundExternalPayload<Profile, Provider, Schema>, Arguments>
    where
        Schema: HostExternalSchema,
        Provider: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
    {
        self.external_payload_with::<Provider, Schema, Arguments>(value)
    }

    #[doc(hidden)]
    pub fn external_payload_with<Binding, Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> HostExternalPayloadView<'call, BoundExternalPayload<Profile, Binding, Schema>, Arguments>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
    {
        let lease = self.runtime.external_lease(value.token);
        HostExternalPayloadView::new(
            BoundExternalStorage::<Profile, Binding, Schema>::store(self.runtime.external_stores())
                .view(&lease),
        )
    }

    pub(crate) fn restore_stored<Type, Stored>(
        &mut self,
        value: &HostStoredValue<Stored>,
    ) -> Type::Value<'call>
    where
        Type: HostType,
    {
        self.restore_runtime_value::<Type>(&value.value)
    }

    pub(in crate::host) fn restore_runtime_value<Type>(
        &mut self,
        value: &crate::runtime::StoredRuntimeValue,
    ) -> Type::Value<'call>
    where
        Type: HostType,
    {
        let token = self.runtime.restore_stored(value);
        crate::host::type_::from_token::<Type, Profile>(self.runtime, token)
    }

    pub(in crate::host) fn resolve_host_type<Type: HostType>(
        &self,
    ) -> Option<crate::plan::ValueType> {
        self.runtime
            .resolve_host_type(&crate::host::HostTypeDescriptor::of::<Type>())
    }

    /// Invokes a Gleam callable while this host call owns the active runtime.
    ///
    /// A provider-state borrow must end before re-entry.
    ///
    /// ```compile_fail
    /// use geam_core::{
    ///     HostCall, HostCallCompletion, HostCallError, HostCallable, HostProfile, HostProvider,
    ///     HostTypeList, HostTypeListEnd,
    /// };
    /// use num_bigint::BigInt;
    ///
    /// struct Profile;
    /// struct Provider;
    ///
    /// impl HostProfile for Profile {
    ///     type RunState = usize;
    ///     type ExternalStores = ();
    /// }
    ///
    /// impl HostProvider<Profile> for Provider {
    ///     type State = usize;
    ///
    ///     fn project(state: &mut usize) -> &mut Self::State {
    ///         state
    ///     }
    /// }
    ///
    /// type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
    ///
    /// fn reenter_with_live_state<'call>(
    ///     mut call: HostCall<'call, Profile, Provider, BigInt>,
    ///     callable: HostCallable<'call, Arguments, BigInt>,
    /// ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    ///     let state = call.state();
    ///     let returned = call.invoke(callable, (BigInt::from(1), ()))?;
    ///     *state += 1;
    ///     Ok(call.return_value(returned))
    /// }
    /// ```
    pub fn invoke<Arguments, FunctionReturn>(
        &mut self,
        function: crate::host::HostCallable<'call, Arguments, FunctionReturn>,
        arguments: Arguments::Values<'call>,
    ) -> Result<FunctionReturn::Value<'call>, crate::HostCallError>
    where
        Arguments: HostTypeSequence,
        FunctionReturn: HostType,
    {
        let mut values = Vec::new();
        crate::host::type_::into_scoped_values::<Arguments>(arguments, &mut values);
        let returned = self
            .runtime
            .invoke(function.token, values.into_boxed_slice())?;
        Ok(crate::host::type_::from_token::<FunctionReturn, Profile>(
            self.runtime,
            returned,
        ))
    }

    /// Constructs an intermediate external payload authorized by one registered type token.
    pub fn construct_external<Schema, Arguments>(
        &mut self,
        _construction: HostConstruction<'call, HostExternalType<Schema, Arguments>>,
        value: BoundExternalPayload<Profile, Provider, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Provider: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        self.construct_external_with_binding::<Provider, Schema, Arguments>(_construction, value)
    }

    #[doc(hidden)]
    pub fn construct_external_with_binding<Binding, Schema, Arguments>(
        &mut self,
        _construction: HostConstruction<'call, HostExternalType<Schema, Arguments>>,
        value: BoundExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        self.seal_constructed_external_with::<Binding, Schema, Arguments>(value)
    }

    /// Constructs an intermediate external payload that retains typed Gleam values.
    pub fn construct_external_with<Schema, Arguments>(
        &mut self,
        _construction: HostConstruction<'call, HostExternalType<Schema, Arguments>>,
        build: impl FnOnce(
            &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
        ) -> BoundExternalPayload<Profile, Provider, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Provider: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        let value = {
            let mut builder = HostExternalPayloadBuilder::new(self.runtime);
            build(&mut builder)
        };
        let lease = self.insert_external_payload_with::<Provider, Schema, Arguments>(value);
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            lease,
        ))
    }

    /// Constructs a retained external payload through its declaration-owned
    /// binding rather than the provider handling the active function.
    #[doc(hidden)]
    pub fn construct_retained_external_with_binding<Binding, Schema, Arguments>(
        &mut self,
        _construction: HostConstruction<'call, HostExternalType<Schema, Arguments>>,
        build: impl FnOnce(
            &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
        ) -> BoundExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        let value = {
            let mut builder = HostExternalPayloadBuilder::new(self.runtime);
            build(&mut builder)
        };
        self.seal_constructed_external_with::<Binding, Schema, Arguments>(value)
    }

    fn seal_constructed_external_with<Binding, Schema, Arguments>(
        &mut self,
        value: BoundExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        let lease = self.insert_external_payload_with::<Binding, Schema, Arguments>(value);
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            lease,
        ))
    }

    fn insert_external_payload_with<Binding, Schema, Arguments>(
        &self,
        value: BoundExternalPayload<Profile, Binding, Schema>,
    ) -> crate::host::ExternalPayloadLease
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        HostExternalType<Schema, Arguments>: HostType,
    {
        BoundExternalStorage::<Profile, Binding, Schema>::store(self.runtime.external_stores())
            .insert(
                value,
                BoundExternalStorage::<Profile, Binding, Schema>::source_equal,
                BoundExternalStorage::<Profile, Binding, Schema>::source_hash,
                BoundExternalStorage::<Profile, Binding, Schema>::inspect,
            )
    }

    pub(crate) fn stored_equal(
        &self,
        left: &StoredRuntimeValue,
        right: &StoredRuntimeValue,
    ) -> bool {
        self.runtime.stored_equal(left, right)
    }

    pub(crate) fn stored_source_hash(&self, value: &StoredRuntimeValue) -> u64 {
        self.runtime.stored_source_hash(value)
    }

    pub(crate) fn stored_inspect(&self, value: &StoredRuntimeValue) -> ecow::EcoString {
        self.runtime.stored_inspect(value)
    }

    pub(crate) fn stored_list_len(&self, value: &StoredRuntimeValue) -> usize {
        self.runtime.stored_list_len(value)
    }

    pub(crate) fn stored_list_item(
        &self,
        value: &StoredRuntimeValue,
        index: usize,
    ) -> Option<StoredRuntimeValue> {
        self.runtime.stored_list_item(value, index)
    }

    #[doc(hidden)]
    pub fn provider_retained_list<Item, HostItem, Decoder>(
        &self,
        value: HostList<'call, HostItem>,
        decoder: Decoder,
    ) -> crate::provider::List<Item, crate::provider::ProviderListContext<HostItem, Decoder>>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderListItemDecoder<Item>,
    {
        crate::provider::ProviderListContext::new(self.retain_list_value(value), decoder)
    }

    #[doc(hidden)]
    pub fn provider_retained_input_list<Item, HostItem, Decoder>(
        &self,
        value: HostList<'call, HostItem>,
        decoder: Decoder,
    ) -> crate::provider::List<Item, crate::provider::ProviderInputListContext<Decoder>>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderListItemDecoder<Item>,
    {
        crate::provider::ProviderInputListContext::new(self.retain_list_value(value), decoder)
    }

    #[doc(hidden)]
    pub fn provider_list_from_input<Item, HostItem, Decoder>(
        &mut self,
        value: crate::provider::List<Item, crate::provider::ProviderListContext<HostItem, Decoder>>,
    ) -> HostList<'call, HostItem>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderListItemDecoder<Item>,
    {
        self.restore_list_value(value.__geam_into_context().retained())
    }

    #[doc(hidden)]
    pub fn provider_external_item_with<Binding, Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> crate::provider::ProviderOwnedExternal<BoundExternalPayload<Profile, Binding, Schema>>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundExternalPayload<Profile, Binding, Schema>: Send,
    {
        let lease = self.runtime.external_lease(value.token);
        let access = self.provider_external_payload_access_with::<Binding, Schema>();
        crate::provider::ProviderOwnedExternal::new(access, lease)
    }

    #[doc(hidden)]
    pub fn provider_external_view_with<Binding, Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> crate::provider::ProviderExternalView<BoundExternalPayload<Profile, Binding, Schema>>
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundExternalPayload<Profile, Binding, Schema>: Send,
    {
        let lease = self.runtime.external_lease(value.token);
        let view =
            BoundExternalStorage::<Profile, Binding, Schema>::store(self.runtime.external_stores())
                .view(&lease);
        crate::provider::ProviderExternalView::new(view, lease)
    }

    #[doc(hidden)]
    pub fn provider_external_from_item<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderOwnedExternal<Payload>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Arguments: HostTypeSequence,
        Payload: Send + 'static,
    {
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            value.into_lease(),
        ))
    }

    #[doc(hidden)]
    pub fn provider_external_from_view<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderExternalView<Payload>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Arguments: HostTypeSequence,
    {
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            value.into_lease(),
        ))
    }

    #[doc(hidden)]
    pub fn provider_external_from_return<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderExternalReturn<Payload>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Arguments: HostTypeSequence,
    {
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            value.into_lease(),
        ))
    }

    #[doc(hidden)]
    pub fn provider_external_payload_access_with<Binding, Schema>(
        &self,
    ) -> crate::provider::ProviderExternalPayloadAccess<
        BoundExternalPayload<Profile, Binding, Schema>,
    >
    where
        Schema: HostExternalSchema,
        Binding: HostExternalBinding<Profile, Schema>,
        BoundExternalPayload<Profile, Binding, Schema>: Send,
    {
        crate::provider::ProviderExternalPayloadAccess::new(BoundExternalStorage::<
            Profile,
            Binding,
            Schema,
        >::store(
            self.runtime.external_stores()
        ))
    }

    pub(crate) fn retain_value<Type: HostType>(
        &self,
        value: Type::Value<'call>,
    ) -> StoredRuntimeValue {
        self.runtime
            .retain_stored(crate::host::type_::into_scoped::<Type>(value))
    }

    pub(crate) fn restore_value<Type: HostType>(
        &mut self,
        value: &StoredRuntimeValue,
    ) -> Type::Value<'call> {
        let token = self.runtime.restore_stored(value);
        crate::host::type_::from_runtime_token::<Type, _>(self.runtime, token)
    }

    pub(crate) fn stored_has_type<Type: HostType>(&self, value: &StoredRuntimeValue) -> bool {
        self.resolve_host_type::<Type>()
            .is_some_and(|requested| value.type_() == &requested)
    }

    pub(crate) fn retain_list_value<Item: HostType>(
        &self,
        value: HostList<'call, Item>,
    ) -> StoredRuntimeList {
        self.runtime.retain_list(value.token)
    }

    pub(crate) fn restore_list_value<Item: HostType>(
        &mut self,
        value: &StoredRuntimeList,
    ) -> HostList<'call, Item> {
        HostList::new(self.runtime.restore_list(value))
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    HostCall<'call, Profile, Provider, HostExternalType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence,
    HostExternalType<Schema, Arguments>: HostType,
{
    pub fn create_external(
        &mut self,
        value: BoundExternalPayload<Profile, Provider, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Provider: HostExternalBinding<Profile, Schema>,
    {
        self.create_external_with_binding::<Provider>(value)
    }

    #[doc(hidden)]
    pub fn create_external_with_binding<Binding>(
        &mut self,
        value: BoundExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Binding: HostExternalBinding<Profile, Schema>,
    {
        self.seal_external_payload_with::<Binding>(value)
    }

    /// Creates an external payload that may retain typed Gleam values.
    pub fn create_external_with(
        &mut self,
        build: impl FnOnce(
            &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
        ) -> BoundExternalPayload<Profile, Provider, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Provider: HostExternalBinding<Profile, Schema>,
    {
        let value = {
            let mut builder = HostExternalPayloadBuilder::new(self.runtime);
            build(&mut builder)
        };
        self.seal_external_payload_with::<Provider>(value)
    }

    fn seal_external_payload_with<Binding>(
        &mut self,
        value: BoundExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Binding: HostExternalBinding<Profile, Schema>,
    {
        let lease = self.insert_external_payload_with::<Binding, Schema, Arguments>(value);
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            lease,
        ))
    }
}

impl<'call, Profile, Provider, Item> HostCall<'call, Profile, Provider, HostListType<Item>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Item: HostType,
{
    pub fn return_list(
        self,
        values: impl IntoIterator<Item = Item::Value<'call>>,
    ) -> HostCallCompletion<'call, HostListType<Item>> {
        let values = values
            .into_iter()
            .map(crate::host::type_::into_scoped::<Item>)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        HostCallCompletion::new(self.runtime.build_list(
            &crate::host::HostTypeDescriptor::of::<HostListType<Item>>(),
            values,
        ))
    }
}

impl<'call, Profile, Provider, Elements> HostCall<'call, Profile, Provider, HostTupleType<Elements>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Elements: HostTypeSequence,
{
    pub fn return_tuple(
        self,
        values: <Elements as crate::host::HostTypeSequence>::Values<'call>,
    ) -> HostCallCompletion<'call, HostTupleType<Elements>> {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Elements>(values, &mut output);
        HostCallCompletion::new(self.runtime.build_tuple(output.into_boxed_slice()))
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    HostCall<'call, Profile, Provider, HostCustomType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Schema: crate::host::HostCustomSchema,
    Arguments: HostTypeSequence,
{
    pub fn return_custom<Constructor>(
        self,
        fields: <Constructor::Fields as crate::host::HostTypeSequence>::Values<'call>,
    ) -> HostCallCompletion<'call, HostCustomType<Schema, Arguments>>
    where
        Constructor: HostCustomConstructor<Custom = HostCustomType<Schema, Arguments>>,
        Constructor::Fields: HostTypeSequence,
    {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Constructor::Fields>(fields, &mut output);
        HostCallCompletion::new(self.runtime.build_custom(
            &crate::host::HostTypeDescriptor::of::<HostCustomType<Schema, Arguments>>(),
            crate::host::type_::custom_constructor_index::<Constructor>(),
            output.into_boxed_slice(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCall, HostProvider};
    use crate::BitArrayValue;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder, with_execution_scope};
    use crate::frontend::compile_typed_host_program;
    use crate::host::function::CallArguments;
    use crate::host::test::{
        StatelessTestProvider, TestHostCallRuntime, TestHostProfile, TestRunState,
    };
    use crate::host::{
        HostCallable, HostConstructions, HostCustom, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomConstructorSchema, HostCustomFieldListEnd, HostCustomFieldSchema,
        HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomToken, HostCustomType,
        HostCustomTypeSchema, HostFunctionToken, HostFunctionType, HostList, HostListToken,
        HostListType, HostScopedValue, HostTuple, HostTupleToken, HostTupleType, HostTypeIndex0,
        HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        HostValueFamily, HostValueToken, StatelessHostProfile,
    };
    use crate::host::{
        HostComponentProfile, HostFutureStore, HostProfile, HostProviderComponentRegistration,
        HostProviderModule, HostProviderSet,
    };
    use crate::provider::{ProviderError, ProviderOk, ProviderResult};
    use crate::provider::{ProviderListItemDecoder, ProviderListItemValue};
    use crate::work_fixture::{WorkComponent, WorkHostType, WorkSchema};
    use crate::{HostCallCompletion, HostCallError, HostExternal, ModuleSource, PackageSource};
    use ecow::EcoString;
    use futures_util::FutureExt;
    use num_bigint::BigInt;

    struct Counter;

    struct IntListDecoder;

    impl ProviderListItemDecoder<BigInt> for IntListDecoder {
        type View = BigInt;

        fn decode(&self, value: ProviderListItemValue) -> Self::View {
            value.into_scalar()
        }
    }

    impl HostProvider<TestHostProfile> for Counter {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    struct MarkerSchema;

    struct MarkerConstructor;

    impl HostCustomConstructorDefinition for MarkerConstructor {
        const NAME: &'static str = "Marker";

        type Fields = HostCustomFieldListEnd;
    }

    struct OtherConstructor;

    impl HostCustomConstructorDefinition for OtherConstructor {
        const NAME: &'static str = "Other";

        type Fields = HostCustomFieldListEnd;
    }

    impl HostCustomSchema for MarkerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/marker";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;

        type Constructors = HostCustomConstructorList<
            MarkerConstructor,
            HostCustomConstructorList<OtherConstructor, HostCustomConstructorListEnd>,
        >;
    }

    type MarkerType = HostCustomType<MarkerSchema>;
    type Marker = HostCustomConstructorAt<MarkerType, HostCustomIndex0, MarkerConstructor>;
    type Other = HostCustomConstructorAt<
        MarkerType,
        HostCustomIndexNext<HostCustomIndex0>,
        OtherConstructor,
    >;

    #[test]
    fn host_call_exposes_only_the_selected_provider_state() {
        let mut state = TestRunState {
            counter: 1,
            unrelated: true,
        };
        let arguments = crate::host::function::CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        *HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime).state() += 1;

        assert_eq!(state.counter, 2);
        assert!(state.unrelated);
    }

    #[test]
    fn stateless_provider_projects_the_complete_run_state() {
        let mut state = ();

        let projected =
            <StatelessTestProvider as HostProvider<StatelessHostProfile>>::project(&mut state);

        assert_eq!(*projected, ());
    }

    #[test]
    fn host_call_reads_and_compares_call_scoped_values() {
        type EmptyTuple = HostTupleType<HostTypeListEnd>;

        assert_eq!(
            HostCustomTypeSchema::of::<MarkerSchema>(),
            HostCustomTypeSchema::new(
                "domain",
                "domain/marker",
                "Marker",
                0,
                [
                    HostCustomConstructorSchema::new("Marker", Vec::<HostCustomFieldSchema>::new(),),
                    HostCustomConstructorSchema::new("Other", Vec::<HostCustomFieldSchema>::new(),),
                ],
            ),
        );
        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let mut call = HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime);
        let list = HostList::<BigInt>::new(HostListToken::Stored(0));
        let tuple = HostTuple::<HostTypeListEnd>::new(HostTupleToken(0));
        let custom = HostCustom::<MarkerType>::new(HostCustomToken(0));

        assert_eq!(call.list_len(list), 0);
        assert_eq!(call.list_item(list, 0), None);
        assert_eq!(call.tuple_len(tuple), 0);
        assert_eq!(call.tuple_values::<HostTypeListEnd>(tuple), ());
        assert_eq!(call.custom_constructor(custom), 0);
        assert_eq!(call.custom_fields::<Marker>(custom), Some(()),);
        assert_eq!(call.custom_fields::<Other>(custom), None,);
        assert_eq!(call.provider_custom_fields::<Marker>(custom), Some(()),);
        assert_eq!(call.provider_custom_fields::<Other>(custom), None,);
        assert_eq!(call.provider_remaining_custom_fields::<Marker>(custom), ());
        assert!(!call.equal::<BigInt>(1.into(), 1.into()));
        assert!(!call.equal::<HostListType<BigInt>>(list, list));
        assert!(!call.equal::<EmptyTuple>(tuple, tuple));
        assert!(!call.equal::<MarkerType>(custom, custom));
        assert_eq!(call.source_hash::<BigInt>(1.into()), 17);
        assert_eq!(call.inspect::<BigInt>(1.into()), "inspected");
    }

    #[test]
    fn host_call_completes_every_scalar_and_scoped_handle_family() {
        type Parameter = HostTypeParameter<0>;
        type List = HostListType<BigInt>;
        type Tuple = HostTupleType<HostTypeListEnd>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let parameter = HostValue::<Parameter>::new(HostValueToken {
            family: HostValueFamily::Bool,
            index: 4,
        });
        let list = HostList::<BigInt>::new(HostListToken::Stored(0));
        let tuple = HostTuple::<HostTypeListEnd>::new(HostTupleToken(0));
        let custom = HostCustom::<MarkerType>::new(HostCustomToken(0));

        let tokens = [
            HostCall::<TestHostProfile, Counter, BigInt>::new(&mut runtime)
                .return_value(1.into())
                .token,
            HostCall::<TestHostProfile, Counter, f64>::new(&mut runtime)
                .return_value(1.5)
                .token,
            HostCall::<TestHostProfile, Counter, EcoString>::new(&mut runtime)
                .return_value("text".into())
                .token,
            HostCall::<TestHostProfile, Counter, BitArrayValue>::new(&mut runtime)
                .return_value(BitArrayValue::from_bytes(vec![1]))
                .token,
            HostCall::<TestHostProfile, Counter, char>::new(&mut runtime)
                .return_value('A')
                .token,
            HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime)
                .return_value(true)
                .token,
            HostCall::<TestHostProfile, Counter, ()>::new(&mut runtime)
                .return_value(())
                .token,
            HostCall::<TestHostProfile, Counter, Parameter>::new(&mut runtime)
                .return_value(parameter)
                .token,
            HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
                .return_value(list)
                .token,
            HostCall::<TestHostProfile, Counter, Tuple>::new(&mut runtime)
                .return_value(tuple)
                .token,
            HostCall::<TestHostProfile, Counter, MarkerType>::new(&mut runtime)
                .return_value(custom)
                .token,
        ];

        assert_eq!(
            tokens.map(|token| token.family),
            [
                HostValueFamily::Int,
                HostValueFamily::Float,
                HostValueFamily::String,
                HostValueFamily::BitArray,
                HostValueFamily::UtfCodepoint,
                HostValueFamily::Bool,
                HostValueFamily::Nil,
                HostValueFamily::Bool,
                HostValueFamily::List,
                HostValueFamily::Tuple,
                HostValueFamily::Custom,
            ],
        );
        assert_eq!(tokens[7].index, 4);
    }

    #[test]
    fn host_call_builds_typed_compound_returns() {
        type List = HostListType<BigInt>;
        type Tuple = HostTupleType<HostTypeListEnd>;
        type Constructions =
            HostTypeList<List, HostTypeList<Tuple, HostTypeList<MarkerType, HostTypeListEnd>>>;
        type TupleIndex = HostTypeIndexNext<HostTypeIndex0>;
        type CustomIndex = HostTypeIndexNext<TupleIndex>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        let list = HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
            .return_list([BigInt::from(1), BigInt::from(2)])
            .token;
        let mut call = HostCall::<TestHostProfile, Counter, Tuple>::new(&mut runtime);
        let constructions = HostConstructions::<Constructions>::new();
        let nested_list =
            call.construct_list(constructions.at::<HostTypeIndex0>(), [BigInt::from(3)]);
        let nested_tuple = call.construct_tuple(constructions.at::<TupleIndex>(), ());
        let nested_custom = call.construct_custom::<Marker>(constructions.at::<CustomIndex>(), ());
        assert_eq!(nested_list.token, HostListToken::Stored(0));
        assert_eq!(nested_tuple.token, HostTupleToken(0));
        assert_eq!(nested_custom.token, HostCustomToken(0));
        let tuple = call.return_tuple(()).token;
        let custom = HostCall::<TestHostProfile, Counter, MarkerType>::new(&mut runtime)
            .return_custom::<Marker>(())
            .token;

        assert_eq!(list.family, HostValueFamily::List);
        assert_eq!(tuple.family, HostValueFamily::Tuple);
        assert_eq!(custom.family, HostValueFamily::Custom);
        assert_eq!(runtime.list_builds(), 2);
    }

    #[test]
    fn returning_an_existing_list_does_not_build_a_new_list() {
        type List = HostListType<BigInt>;

        let mut state = TestRunState::default();
        {
            let arguments = CallArguments::new(Vec::new(), Vec::new());
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            let existing = HostList::<BigInt>::new(HostListToken::Stored(0));
            let retained = HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
                .provider_retained_list(existing, IntListDecoder);

            assert_eq!(retained.len(), 1);
            assert_eq!(retained.get(0), Some(BigInt::from(1)));
        }

        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let existing = HostList::<BigInt>::new(HostListToken::Stored(0));

        let returned = HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
            .return_value(existing)
            .token;

        assert_eq!(returned.family, HostValueFamily::List);
        assert_eq!(runtime.list_builds(), 0);

        HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
            .return_list([BigInt::from(1), BigInt::from(2)]);
        assert_eq!(runtime.list_builds(), 1);
    }

    #[test]
    fn host_call_invokes_and_completes_typed_function_handles() {
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Function = HostFunctionType<Arguments, BigInt>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let callable = HostCallable::<Arguments, BigInt>::new(HostFunctionToken(0));
        let returned = HostCall::<TestHostProfile, Counter, BigInt>::new(&mut runtime)
            .invoke(callable, (BigInt::from(7), ()))
            .expect("test runtime should return the first callback argument");

        assert_eq!(returned, BigInt::from(0));
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(7))),
        );

        let completion = HostCall::<TestHostProfile, Counter, Function>::new(&mut runtime)
            .return_value(callable)
            .token;
        assert_eq!(completion.family, HostValueFamily::Function);
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Function(HostFunctionToken(0))),
        );

        let empty = HostCallable::<HostTypeListEnd, ()>::new(HostFunctionToken(1));
        HostCall::<TestHostProfile, Counter, ()>::new(&mut runtime)
            .invoke(empty, ())
            .expect("zero-argument test callback should return Nil");
    }

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    type Pair = HostTypeList<BigInt, HostTypeList<EcoString, HostTypeListEnd>>;
    type Choice = ProviderResult<BigInt, EcoString>;

    fn inspect_data<'call>(
        mut call: HostCall<'call, Profile, WorkComponent, ()>,
        values: HostList<'call, BigInt>,
        pair: HostTuple<'call, Pair>,
        choice: HostCustom<'call, Choice>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert_eq!(call.state(), &());
        assert_eq!(call.list_len(values), 2);
        assert_eq!(call.list_item(values, 1), Some(BigInt::from(42)));
        assert_eq!(call.list_item(values, 2), None);
        assert_eq!(call.tuple_len(pair), 2);
        assert_eq!(
            call.tuple_values(pair),
            (BigInt::from(42), (EcoString::from("answer"), ()))
        );
        let ok = call.custom_fields::<ProviderOk<BigInt, EcoString>>(choice);
        let error = call.custom_fields::<ProviderError<BigInt, EcoString>>(choice);
        assert_eq!(ok.is_some(), call.custom_constructor(choice) == 0);
        assert_eq!(error.is_some(), call.custom_constructor(choice) == 1);
        if let Some((value, ())) = ok {
            assert_eq!(value, BigInt::from(42));
        }
        if let Some((message, ())) = error {
            assert_eq!(message, "failed");
        }
        assert_eq!(
            call.inspect::<HostListType<BigInt>>(values),
            "charlist.from_string(\")*\")"
        );
        Ok(call.return_value(()))
    }

    fn same_work<'call>(
        call: HostCall<'call, Profile, WorkComponent, bool>,
        left: HostExternal<'call, WorkHostType<BigInt>>,
        right: HostExternal<'call, WorkHostType<BigInt>>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let equal = call.equal::<WorkHostType<BigInt>>(left, right);
        if equal {
            assert_eq!(
                call.source_hash::<WorkHostType<BigInt>>(left),
                call.source_hash::<WorkHostType<BigInt>>(right)
            );
        }
        assert_eq!(call.inspect::<WorkHostType<BigInt>>(left), "Work(...)");
        let payload =
            call.external_payload::<WorkSchema, HostTypeList<BigInt, HostTypeListEnd>>(left);
        drop(payload);
        Ok(call.return_value(equal))
    }

    #[test]
    fn scoped_views_preserve_source_shapes_and_operation_identity() {
        let mut providers =
            <WorkComponent as HostProviderComponentRegistration<Profile>>::providers()
                .expect("Future registration");
        providers.push(HostProviderModule::new("application", "library")
            .expect("module")
            .with_scoped_function::<WorkComponent, (HostListType<BigInt>, HostTupleType<Pair>, Choice), (), _>("inspect_data", inspect_data).expect("data observer")
            .with_scoped_function::<WorkComponent, (WorkHostType<BigInt>, WorkHostType<BigInt>), bool, _>("same_work", same_work).expect("work identity"));
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
@external(erlang, "native", "inspect_data")
fn inspect_data(values: List(Int), pair: #(Int, String), choice: Result(Int, String)) -> Nil
@external(erlang, "native", "same_work")
fn same_work(left: future.Work(Int), right: future.Work(Int)) -> Bool
pub fn run() {
  echo 42
  inspect_data([41, 42], #(42, "answer"), Ok(42))
  inspect_data([41, 42], #(42, "answer"), Error("failed"))
  let work = future.ready(42)
  same_work(work, work) && !same_work(work, future.ready(42))
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("selected providers"),
        )
        .expect("ordinary source");
        let (bindings, run) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(), bool>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("sealed execution");
        let mut state = ();
        let mut outputs = Vec::new();
        let mut echo = |output: crate::EchoOutput| outputs.push(output.to_string());
        with_execution_scope(async |guard| {
            assert_eq!(
                module.attach(guard, &mut state, &mut echo).call(&run, ()),
                Ok(true)
            );
        })
        .now_or_never()
        .expect("direct native calls do not need an executor");
        assert_eq!(outputs, ["src/library.gleam:8\n42"]);
    }
}
