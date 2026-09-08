use crate::host::{
    AsyncHostExternalBinding, AsyncHostExternalStorage, HostCallCompletion, HostConstruction,
    HostCustom, HostCustomArgumentSlot, HostCustomConstructor, HostCustomToken, HostCustomType,
    HostExternal, HostExternalArgumentSlot, HostExternalSchema, HostExternalToken,
    HostExternalType, HostFunctionArgumentSlot, HostFunctionToken, HostList, HostListArgumentSlot,
    HostListToken, HostListType, HostProfile, HostProvider, HostScopedValue, HostTokenRuntime,
    HostTuple, HostTupleArgumentSlot, HostTupleToken, HostTupleType, HostType, HostTypeSequence,
    HostValue, HostValueArgumentSlot, HostValueToken,
};
use crate::runtime::{
    StoredRuntimeList, StoredRuntimeValue, TransferExternalPayloadLease,
    TransferExternalPayloadView, TransferValues,
};
use std::marker::PhantomData;
use std::ops::Deref;

type BoundAsyncExternalStorage<Profile, Provider, Schema> =
    <Provider as AsyncHostExternalBinding<Profile, Schema>>::Storage;
type BoundAsyncExternalPayload<Profile, Provider, Schema> =
    <BoundAsyncExternalStorage<Profile, Provider, Schema> as AsyncHostExternalStorage<
        Profile,
        Schema,
    >>::Payload;
type BoundAsyncExternalPayloadView<'call, Profile, Provider, Schema, Arguments> =
    TransferHostExternalPayloadView<
        'call,
        BoundAsyncExternalPayload<Profile, Provider, Schema>,
        Arguments,
    >;

#[derive(Clone)]
pub(crate) struct TransferHostCodecScope {
    function: std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata>,
}

impl TransferHostCodecScope {
    pub(crate) fn new(
        function: std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata>,
    ) -> Self {
        Self { function }
    }

    pub(crate) fn function(
        &self,
    ) -> &std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata> {
        &self.function
    }
}

pub(crate) trait TransferHostCallRuntime<Profile: HostProfile>: HostTokenRuntime {
    fn state(&mut self) -> &mut Profile::RunState;
    fn work(&self) -> crate::runtime::work::execution::WorkContext<Profile>;
    fn origin(&self) -> crate::runtime::HostCallOrigin;
    fn external_stores(&self) -> &Profile::ExternalStores;
    fn arguments(&self) -> &dyn crate::host::HostCallArguments;
    fn value(&self, slot: HostValueArgumentSlot) -> HostValueToken;
    fn list(&self, slot: HostListArgumentSlot) -> HostListToken;
    fn tuple(&self, slot: HostTupleArgumentSlot) -> HostTupleToken;
    fn custom(&self, slot: HostCustomArgumentSlot) -> HostCustomToken;
    fn external(&self, slot: HostExternalArgumentSlot) -> HostExternalToken;
    fn function(&self, slot: HostFunctionArgumentSlot) -> HostFunctionToken;
    fn callable(&self, function: HostFunctionToken) -> crate::runtime::TransferCallable;
    fn codec_scope(&self) -> TransferHostCodecScope;
    fn list_len(&self, value: HostListToken) -> usize;
    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken>;
    fn tuple_len(&self, value: HostTupleToken) -> usize;
    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]>;
    fn custom_constructor(&self, value: HostCustomToken) -> usize;
    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]>;
    fn take_custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]>;
    fn invoke(
        &mut self,
        function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, crate::AsyncHostCallError>;
    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool;
    fn source_hash(&self, value: HostScopedValue) -> u64;
    fn inspect(&self, value: HostScopedValue) -> ecow::EcoString;
    fn stored_equal(
        &self,
        left: &StoredRuntimeValue<TransferValues>,
        right: &StoredRuntimeValue<TransferValues>,
    ) -> bool;
    fn stored_source_hash(&self, value: &StoredRuntimeValue<TransferValues>) -> u64;
    fn stored_inspect(&self, value: &StoredRuntimeValue<TransferValues>) -> ecow::EcoString;
    fn stored_list_len(&self, value: &StoredRuntimeValue<TransferValues>) -> usize;
    fn stored_list_item(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
        index: usize,
    ) -> Option<StoredRuntimeValue<TransferValues>>;
    fn complete(&mut self, value: HostScopedValue) -> HostValueToken;
    fn build_list(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        values: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken;
    fn build_custom(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_external(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        value: TransferExternalPayloadLease,
    ) -> HostExternalToken;
    fn external_lease(&self, value: HostExternalToken) -> TransferExternalPayloadLease;
    fn resolve_host_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType>;
    fn retain_stored(&self, value: HostScopedValue) -> StoredRuntimeValue<TransferValues>;
    fn retain_list(&self, value: HostListToken) -> StoredRuntimeList<TransferValues>;
    fn restore_list(&mut self, value: &StoredRuntimeList<TransferValues>) -> HostListToken;
    fn restore_stored(&mut self, value: &StoredRuntimeValue<TransferValues>) -> HostValueToken;
    fn callback_inputs(
        &self,
        values: Box<[HostScopedValue]>,
    ) -> crate::runtime::TransferCallbackInputs;
}

/// A call-scoped adapter used by an immediate provider in transferable execution.
///
/// This type borrows the active Geam runtime and cannot cross an await point.
/// Native work retains an owned [`crate::host::HostFutureContext`] instead.
#[doc(hidden)]
pub struct TransferHostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub(in crate::host) runtime: &'call mut dyn TransferHostCallRuntime<Profile>,
    marker: PhantomData<(Provider, Return)>,
}

/// A direct, call-scoped view of one transferable external payload.
///
/// The view may be used only by an immediate provider call. It cannot be held
/// across suspension in native work.
#[doc(hidden)]
pub struct TransferHostExternalPayloadView<'call, Payload, Arguments> {
    value: TransferExternalPayloadView<Payload>,
    lifetime: PhantomData<&'call mut ()>,
    arguments: PhantomData<fn() -> Arguments>,
}

impl<Payload, Arguments> Deref for TransferHostExternalPayloadView<'_, Payload, Arguments> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'call, Profile, Provider, Return> TransferHostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub(crate) fn new(runtime: &'call mut dyn TransferHostCallRuntime<Profile>) -> Self {
        Self {
            runtime,
            marker: PhantomData,
        }
    }

    pub fn state(&mut self) -> &mut Provider::State {
        Provider::project(self.runtime.state())
    }

    pub(crate) fn arguments(&self) -> &dyn crate::host::HostCallArguments {
        self.runtime.arguments()
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

    pub fn source_hash<Type: HostType>(&self, value: Type::Value<'call>) -> u64 {
        self.runtime
            .source_hash(crate::host::type_::into_scoped::<Type>(value))
    }

    pub fn inspect<Type: HostType>(&self, value: Type::Value<'call>) -> ecow::EcoString {
        self.runtime
            .inspect(crate::host::type_::into_scoped::<Type>(value))
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
            .map(|token| crate::host::type_::from_runtime_token::<Item, _>(self.runtime, token))
    }

    pub(crate) fn stored_equal(
        &self,
        left: &StoredRuntimeValue<TransferValues>,
        right: &StoredRuntimeValue<TransferValues>,
    ) -> bool {
        self.runtime.stored_equal(left, right)
    }

    pub(crate) fn stored_source_hash(&self, value: &StoredRuntimeValue<TransferValues>) -> u64 {
        self.runtime.stored_source_hash(value)
    }

    pub(crate) fn stored_inspect(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
    ) -> ecow::EcoString {
        self.runtime.stored_inspect(value)
    }

    pub(crate) fn stored_list_len(&self, value: &StoredRuntimeValue<TransferValues>) -> usize {
        self.runtime.stored_list_len(value)
    }

    pub(crate) fn stored_list_item(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
        index: usize,
    ) -> Option<StoredRuntimeValue<TransferValues>> {
        self.runtime.stored_list_item(value, index)
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

    #[doc(hidden)]
    pub fn provider_list<Item, HostItem, Decoder>(
        &self,
        value: HostList<'call, HostItem>,
        decoder: Decoder,
    ) -> crate::provider::List<Item, crate::provider::ProviderTransferListContext<HostItem, Decoder>>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderTransferListItemDecoder<Item>,
    {
        crate::provider::ProviderTransferListContext::new(self.retain_list_value(value), decoder)
    }

    #[doc(hidden)]
    pub fn provider_input_list<Item, HostItem, Decoder>(
        &self,
        value: HostList<'call, HostItem>,
        decoder: Decoder,
    ) -> crate::provider::List<Item, crate::provider::ProviderTransferInputListContext<Decoder>>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderTransferListItemDecoder<Item>,
    {
        crate::provider::ProviderTransferInputListContext::new(
            self.retain_list_value(value),
            decoder,
        )
    }

    #[doc(hidden)]
    pub fn provider_list_from_input<Item, HostItem, Decoder>(
        &mut self,
        value: crate::provider::List<
            Item,
            crate::provider::ProviderTransferListContext<HostItem, Decoder>,
        >,
    ) -> HostList<'call, HostItem>
    where
        HostItem: HostType,
        Decoder: crate::provider::ProviderTransferListItemDecoder<Item>,
    {
        self.restore_list_value(value.__geam_into_transfer_context().retained())
    }

    pub fn tuple_len<Elements>(&self, value: HostTuple<'call, Elements>) -> usize {
        self.runtime.tuple_len(value.token)
    }

    pub fn tuple_values<Elements: HostTypeSequence>(
        &mut self,
        value: HostTuple<'call, Elements>,
    ) -> Elements::Values<'call> {
        let values = self.runtime.tuple_values(value.token);
        crate::host::type_::from_runtime_tokens::<Elements, _>(self.runtime, &values)
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
        Some(crate::host::type_::from_runtime_tokens::<
            Constructor::Fields,
            _,
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
        Some(crate::host::type_::from_runtime_tokens::<
            Constructor::Fields,
            _,
        >(self.runtime, &fields))
    }

    #[doc(hidden)]
    pub fn provider_remaining_custom_fields<Constructor>(
        &mut self,
        value: HostCustom<'call, Constructor::Custom>,
    ) -> <Constructor::Fields as HostTypeSequence>::Values<'call>
    where
        Constructor: HostCustomConstructor,
    {
        let fields = self.runtime.take_custom_fields(value.token);
        crate::host::type_::from_runtime_tokens::<Constructor::Fields, _>(self.runtime, &fields)
    }

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

    pub fn external_payload<Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> BoundAsyncExternalPayloadView<'call, Profile, Provider, Schema, Arguments>
    where
        Schema: HostExternalSchema,
        Provider: AsyncHostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundAsyncExternalPayload<Profile, Provider, Schema>: Send,
    {
        let lease = self.runtime.external_lease(value.token);
        let value = <Provider::Storage as AsyncHostExternalStorage<Profile, Schema>>::store(
            self.runtime.external_stores(),
        )
        .view(&lease);
        TransferHostExternalPayloadView {
            value,
            lifetime: PhantomData,
            arguments: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn provider_transfer_external_item_with<Binding, Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> crate::provider::ProviderTransferExternalItem<
        BoundAsyncExternalPayload<Profile, Binding, Schema>,
    >
    where
        Schema: HostExternalSchema,
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        let lease = self.runtime.external_lease(value.token);
        let access = self.provider_transfer_external_payload_access_with::<Binding, Schema>();
        crate::provider::ProviderTransferExternalItem::new(access, lease)
    }

    #[doc(hidden)]
    pub fn provider_transfer_external_view_with<Binding, Schema, Arguments>(
        &self,
        value: HostExternal<'call, HostExternalType<Schema, Arguments>>,
    ) -> crate::provider::ProviderTransferExternalView<
        BoundAsyncExternalPayload<Profile, Binding, Schema>,
    >
    where
        Schema: HostExternalSchema,
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        let lease = self.runtime.external_lease(value.token);
        let view = BoundAsyncExternalStorage::<Profile, Binding, Schema>::store(
            self.runtime.external_stores(),
        )
        .view(&lease);
        crate::provider::ProviderTransferExternalView::new(view, lease)
    }

    #[doc(hidden)]
    pub fn provider_transfer_external_from_item<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderTransferExternalItem<Payload>,
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
    pub fn provider_transfer_external_from_view<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderTransferExternalView<Payload>,
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
    pub fn provider_transfer_external_from_return<Schema, Arguments, Payload>(
        &mut self,
        value: crate::provider::ProviderTransferExternalReturn<Payload>,
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
    pub fn provider_transfer_external_payload_access_with<Binding, Schema>(
        &self,
    ) -> crate::provider::ProviderTransferExternalPayloadAccess<
        BoundAsyncExternalPayload<Profile, Binding, Schema>,
    >
    where
        Schema: HostExternalSchema,
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        crate::provider::ProviderTransferExternalPayloadAccess::new(BoundAsyncExternalStorage::<
            Profile,
            Binding,
            Schema,
        >::store(
            self.runtime.external_stores(),
        ))
    }

    #[doc(hidden)]
    pub fn construct_external_with_binding<Binding, Schema, Arguments>(
        &mut self,
        _construction: HostConstruction<'call, HostExternalType<Schema, Arguments>>,
        value: BoundAsyncExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        self.seal_external_payload_with::<Binding, Schema, Arguments>(value)
    }

    fn seal_external_payload_with<Binding, Schema, Arguments>(
        &mut self,
        value: BoundAsyncExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Schema: HostExternalSchema,
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        Arguments: HostTypeSequence,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        let lease = BoundAsyncExternalStorage::<Profile, Binding, Schema>::store(
            self.runtime.external_stores(),
        )
        .insert::<Profile, Schema, BoundAsyncExternalStorage<Profile, Binding, Schema>>(value);
        HostExternal::new(self.runtime.build_external(
            &crate::host::HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            lease,
        ))
    }

    pub fn invoke<Arguments, FunctionReturn>(
        &mut self,
        function: crate::host::HostCallable<'call, Arguments, FunctionReturn>,
        arguments: Arguments::Values<'call>,
    ) -> Result<FunctionReturn::Value<'call>, crate::AsyncHostCallError>
    where
        Arguments: HostTypeSequence,
        FunctionReturn: HostType,
    {
        let mut values = Vec::new();
        crate::host::type_::into_scoped_values::<Arguments>(arguments, &mut values);
        let returned = self
            .runtime
            .invoke(function.token, values.into_boxed_slice())?;
        Ok(crate::host::type_::from_runtime_token::<FunctionReturn, _>(
            self.runtime,
            returned,
        ))
    }

    pub(in crate::host) fn resolve_host_type<Type: HostType>(
        &self,
    ) -> Option<crate::plan::ValueType> {
        self.runtime
            .resolve_host_type(&crate::host::HostTypeDescriptor::of::<Type>())
    }

    pub(crate) fn retain_value<Type: HostType>(
        &self,
        value: Type::Value<'call>,
    ) -> StoredRuntimeValue<TransferValues> {
        self.runtime
            .retain_stored(crate::host::type_::into_scoped::<Type>(value))
    }

    pub(crate) fn restore_value<Type: HostType>(
        &mut self,
        value: &StoredRuntimeValue<TransferValues>,
    ) -> Type::Value<'call> {
        let token = self.runtime.restore_stored(value);
        crate::host::type_::from_runtime_token::<Type, _>(self.runtime, token)
    }

    pub(crate) fn stored_has_type<Type: HostType>(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
    ) -> bool {
        self.resolve_host_type::<Type>()
            .is_some_and(|requested| value.type_() == &requested)
    }

    pub(crate) fn retain_list_value<Item: HostType>(
        &self,
        value: HostList<'call, Item>,
    ) -> StoredRuntimeList<TransferValues> {
        self.runtime.retain_list(value.token)
    }

    pub(crate) fn restore_list_value<Item: HostType>(
        &mut self,
        value: &StoredRuntimeList<TransferValues>,
    ) -> HostList<'call, Item> {
        HostList::new(self.runtime.restore_list(value))
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    TransferHostCall<'call, Profile, Provider, HostExternalType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence,
{
    #[doc(hidden)]
    pub fn create_external_with_binding<Binding>(
        &mut self,
        value: BoundAsyncExternalPayload<Profile, Binding, Schema>,
    ) -> HostExternal<'call, HostExternalType<Schema, Arguments>>
    where
        Binding: AsyncHostExternalBinding<Profile, Schema>,
        BoundAsyncExternalPayload<Profile, Binding, Schema>: Send,
    {
        self.seal_external_payload_with::<Binding, Schema, Arguments>(value)
    }
}

impl<'call, Profile, Provider, Item> TransferHostCall<'call, Profile, Provider, HostListType<Item>>
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

impl<'call, Profile, Provider, Elements>
    TransferHostCall<'call, Profile, Provider, HostTupleType<Elements>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Elements: HostTypeSequence,
{
    pub fn return_tuple(
        self,
        values: Elements::Values<'call>,
    ) -> HostCallCompletion<'call, HostTupleType<Elements>> {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Elements>(values, &mut output);
        HostCallCompletion::new(self.runtime.build_tuple(output.into_boxed_slice()))
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    TransferHostCall<'call, Profile, Provider, HostCustomType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Schema: crate::host::HostCustomSchema,
    Arguments: HostTypeSequence,
{
    pub fn return_custom<Constructor>(
        self,
        fields: <Constructor::Fields as HostTypeSequence>::Values<'call>,
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
    use super::TransferHostCall;
    use crate::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
    use crate::frontend::compile_typed_transfer_host_program;
    use crate::host::{
        AsyncHostComponentProfile, HostFutureStore, HostProfile, HostTupleType,
        TransferHostProviderComponentRegistration, TransferHostProviderModule,
        TransferHostProviderSet,
    };
    use crate::provider::{ProviderError, ProviderOk, ProviderResult};
    use crate::work_fixture::{WorkComponent, WorkHostType, WorkSchema};
    use crate::{
        AsyncHostCallError, HostCallCompletion, HostCustom, HostExternal, HostList, HostListType,
        HostTuple, HostTypeList, HostTypeListEnd, ModuleSource, PackageSource,
    };
    use ecow::EcoString;
    use futures_util::FutureExt;
    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for Profile {
        fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    type Pair = HostTypeList<BigInt, HostTypeList<EcoString, HostTypeListEnd>>;
    type Choice = ProviderResult<BigInt, EcoString>;

    fn inspect_data<'call>(
        mut call: TransferHostCall<'call, Profile, WorkComponent, ()>,
        values: HostList<'call, BigInt>,
        pair: HostTuple<'call, Pair>,
        choice: HostCustom<'call, Choice>,
    ) -> Result<HostCallCompletion<'call, ()>, AsyncHostCallError> {
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
        call: TransferHostCall<'call, Profile, WorkComponent, bool>,
        left: HostExternal<'call, WorkHostType<BigInt>>,
        right: HostExternal<'call, WorkHostType<BigInt>>,
    ) -> Result<HostCallCompletion<'call, bool>, AsyncHostCallError> {
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
            <WorkComponent as TransferHostProviderComponentRegistration<Profile>>::providers()
                .expect("Future registration");
        providers.push(TransferHostProviderModule::new_for_profile("application", "library")
            .expect("module")
            .with_scoped_function::<WorkComponent, (HostListType<BigInt>, HostTupleType<Pair>, Choice), (), _>("inspect_data", inspect_data).expect("data observer")
            .with_scoped_function::<WorkComponent, (WorkHostType<BigInt>, WorkHostType<BigInt>), bool, _>("same_work", same_work).expect("work identity"));
        let program = compile_typed_transfer_host_program(
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
            TransferHostProviderSet::new(providers).expect("selected providers"),
        )
        .expect("ordinary source");
        let (bindings, run) = WorkModuleBuilder::new(program)
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
