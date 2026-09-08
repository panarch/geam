use crate::host::{
    AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection, HostCall,
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostListType, HostProfile,
    HostProvider, HostStoredDynamic, HostStoredType, HostStoredValue, HostType, HostTypeIndex0,
    HostTypeIndexNext, TransferHostCall,
};
use crate::provider::{
    List, ProviderConstructions, ProviderExternalDeclaration, ProviderInputValue,
    ProviderListContext, ProviderListInputCodec, ProviderListInputValue, ProviderNoConstructions,
    ProviderOutputValue, ProviderStoredOwner, ProviderTransferInputValue,
    ProviderTransferListContext, ProviderTransferListInputCodec, ProviderTransferListInputValue,
    ProviderTransferOutputValue, ProviderTransferValue, ProviderTransferValueContext,
    ProviderValue, ProviderValueContext, Value,
};
use crate::runtime::{StoredRuntimeValue, TransferValues};
use ecow::EcoString;
use std::marker::PhantomData;

/// The ordinary call-local representation for retained provider values.
pub struct LocalRetainedContext;

/// The representation used by provider values that may cross an await point.
#[doc(hidden)]
pub struct ProviderTransferRetainedContext;

mod retained_context_sealed {
    pub trait Sealed {}

    impl Sealed for super::LocalRetainedContext {}
    impl Sealed for super::ProviderTransferRetainedContext {}
}

/// Representation-specific storage and source semantics for retained values.
///
/// The trait is sealed. It exists so an advanced payload can explicitly share
/// one Rust shape between local and transferable provider composition.
#[doc(hidden)]
pub trait RetainedContext: retained_context_sealed::Sealed + 'static {
    type Retained<Index>;
    type Dynamic;
    type Equality<'value>;
    type Hashing<'value>;
    type Inspection<'value>;

    fn retained_equal<Index>(
        context: &Self::Equality<'_>,
        left: &Self::Retained<Index>,
        right: &Self::Retained<Index>,
    ) -> bool;

    fn retained_hash<Index>(context: &Self::Hashing<'_>, value: &Self::Retained<Index>) -> u64;

    fn retained_inspect<Index>(
        context: &Self::Inspection<'_>,
        value: &Self::Retained<Index>,
    ) -> EcoString;

    fn dynamic_kind(value: &Self::Dynamic) -> DynamicKind;

    fn dynamic_is_external<Declaration>(value: &Self::Dynamic) -> bool
    where
        Declaration: ProviderExternalDeclaration;

    fn dynamic_tuple_items(value: Self::Dynamic) -> Result<Box<[Self::Dynamic]>, Self::Dynamic>;

    fn dynamic_equal(
        context: &Self::Equality<'_>,
        left: &Self::Dynamic,
        right: &Self::Dynamic,
    ) -> bool;

    fn dynamic_hash(context: &Self::Hashing<'_>, value: &Self::Dynamic) -> u64;

    fn dynamic_inspect(context: &Self::Inspection<'_>, value: &Self::Dynamic) -> EcoString;
}

/// Source-equality access for retained values in one immutable payload.
pub type Equality<'value, Context = LocalRetainedContext> =
    <Context as RetainedContext>::Equality<'value>;

/// Source-hash access for retained values in one immutable payload.
pub type Hashing<'value, Context = LocalRetainedContext> =
    <Context as RetainedContext>::Hashing<'value>;

/// Source-inspection access for retained values in one immutable payload.
pub type Inspection<'value, Context = LocalRetainedContext> =
    <Context as RetainedContext>::Inspection<'value>;

/// Source-equality access for values retained by a transferable payload.
#[doc(hidden)]
pub type ProviderTransferEquality<'value> = Equality<'value, ProviderTransferRetainedContext>;

/// Source-hash access for values retained by a transferable payload.
#[doc(hidden)]
pub type ProviderTransferHashing<'value> = Hashing<'value, ProviderTransferRetainedContext>;

/// Source-inspection access for values retained by a transferable payload.
#[doc(hidden)]
pub type ProviderTransferInspection<'value> = Inspection<'value, ProviderTransferRetainedContext>;

/// One retained source value owned by an advanced external payload.
///
/// The payload type is the owner brand. The argument index identifies the
/// corresponding source type parameter. Values are created only by
/// [`super::Call::store`] followed by the generated external boundary.
pub struct Retained<Owner, Index, Context = LocalRetainedContext>
where
    Context: RetainedContext,
{
    value: Context::Retained<Index>,
    owner: PhantomData<fn() -> (Owner, Context)>,
}

/// One existential source value owned by an advanced external payload.
///
/// The exact specialized source type stays sealed inside Geam. Providers can
/// inspect its broad family, confirm a generated external declaration, or
/// request an exact typed restore through an active [`crate::provider::Call`].
pub struct StoredDynamic<Owner, Context = LocalRetainedContext>
where
    Context: RetainedContext,
{
    value: Context::Dynamic,
    owner: PhantomData<fn() -> (Owner, Context)>,
}

#[doc(hidden)]
pub struct ProviderTransferRetainedValue<Index> {
    value: StoredRuntimeValue<TransferValues>,
    index: PhantomData<fn() -> Index>,
}

#[doc(hidden)]
pub struct ProviderTransferDynamicStorage {
    value: StoredRuntimeValue<TransferValues>,
}

/// One typed source value retained by a transferable external payload.
#[doc(hidden)]
pub type ProviderTransferRetained<Owner, Index> =
    Retained<Owner, Index, ProviderTransferRetainedContext>;

/// One existential source value retained by a transferable external payload.
#[doc(hidden)]
pub type ProviderTransferStoredDynamic<Owner> =
    StoredDynamic<Owner, ProviderTransferRetainedContext>;

/// An existing external source value with its original runtime identity.
///
/// This advanced input is useful when a provider must pass an external value
/// to a callback or return it unchanged. Dereferencing it borrows the ordinary
/// Rust payload; consuming it preserves the original source handle.
pub type External<Payload> = crate::provider::ProviderExternalItem<Payload>;

/// Static pass-through into existential retention without materialization.
#[doc(hidden)]
pub trait ProviderDynamicValue<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;

    fn into_host(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> <Self::Host as HostType>::Value<'call>;
}

impl<'call, Profile, Provider, Return, Type, Host>
    ProviderDynamicValue<'call, Profile, Provider, Return>
    for Value<Type, ProviderValueContext<'call, Host>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Host = Host;

    fn into_host(
        self,
        _call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> <Self::Host as HostType>::Value<'call> {
        self.into_host()
    }
}

impl<'call, Profile, Provider, Return, Type> ProviderDynamicValue<'call, Profile, Provider, Return>
    for Type
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderValue<OutputRequirements = ProviderNoConstructions>
        + ProviderOutputValue<Profile, Provider, Return>,
{
    type Host = Type::Host;

    fn into_host(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> <Self::Host as HostType>::Value<'call> {
        self.into_host(call, &ProviderConstructions::none())
    }
}

impl<'call, Profile, Provider, Return, Item, HostItem, Decoder>
    ProviderDynamicValue<'call, Profile, Provider, Return>
    for List<Item, ProviderListContext<'call, HostItem, Decoder>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    HostItem: HostType,
    Decoder: crate::provider::ProviderListItemDecoder<Item>,
{
    type Host = HostListType<HostItem>;

    fn into_host(
        self,
        _call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> <Self::Host as HostType>::Value<'call> {
        self.__geam_into_context().into_host()
    }
}

/// Broad runtime family of one existentially retained source value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicKind {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List,
    Tuple,
    Custom,
    External,
    Function,
}

impl DynamicKind {
    fn from_family(family: crate::provider_support::HostStoredValueFamily) -> Self {
        match family {
            crate::provider_support::HostStoredValueFamily::Int => Self::Int,
            crate::provider_support::HostStoredValueFamily::Float => Self::Float,
            crate::provider_support::HostStoredValueFamily::String => Self::String,
            crate::provider_support::HostStoredValueFamily::BitArray => Self::BitArray,
            crate::provider_support::HostStoredValueFamily::UtfCodepoint => Self::UtfCodepoint,
            crate::provider_support::HostStoredValueFamily::Bool => Self::Bool,
            crate::provider_support::HostStoredValueFamily::Nil => Self::Nil,
            crate::provider_support::HostStoredValueFamily::List => Self::List,
            crate::provider_support::HostStoredValueFamily::Tuple => Self::Tuple,
            crate::provider_support::HostStoredValueFamily::Custom => Self::Custom,
            crate::provider_support::HostStoredValueFamily::External => Self::External,
            crate::provider_support::HostStoredValueFamily::Function => Self::Function,
        }
    }
}

/// Static input conversion used by exact existential restores.
#[doc(hidden)]
pub trait ProviderDynamicInput<Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;
    type View<'call>;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View<'call>;
}

/// Static pass-through into transferable existential retention.
#[doc(hidden)]
pub trait ProviderTransferDynamicValue<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;

    fn into_stored<Owner>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
    ) -> ProviderTransferStoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner;
}

impl<'call, Profile, Provider, Return, Type, Host>
    ProviderTransferDynamicValue<'call, Profile, Provider, Return>
    for Value<Type, ProviderTransferValueContext<Host>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Host = Host;

    fn into_stored<Owner>(
        self,
        _call: &mut TransferHostCall<'call, Profile, Provider, Return>,
    ) -> ProviderTransferStoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        ProviderTransferStoredDynamic::new_transfer(self.into_stored())
    }
}

impl<'call, Profile, Provider, Return, Type>
    ProviderTransferDynamicValue<'call, Profile, Provider, Return> for Type
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderValue<OutputRequirements = ProviderNoConstructions>
        + ProviderTransferOutputValue<Profile, Provider, Return>,
{
    type Host = Type::Host;

    fn into_stored<Owner>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
    ) -> ProviderTransferStoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        let value = self.into_host(call, &ProviderConstructions::none());
        ProviderTransferStoredDynamic::new_transfer(call.retain_value::<Self::Host>(value))
    }
}

impl<'call, Profile, Provider, Return, Item, HostItem, Decoder>
    ProviderTransferDynamicValue<'call, Profile, Provider, Return>
    for List<Item, ProviderTransferListContext<HostItem, Decoder>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    HostItem: HostType,
    Decoder: crate::provider::ProviderTransferListItemDecoder<Item>,
{
    type Host = HostListType<HostItem>;

    fn into_stored<Owner>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
    ) -> ProviderTransferStoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        let value = call.provider_list_from_input(self);
        ProviderTransferStoredDynamic::new_transfer(call.retain_value::<Self::Host>(value))
    }
}

/// Static input conversion used by transferable existential restores.
#[doc(hidden)]
pub trait ProviderTransferDynamicInput<Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;
    type View;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View;
}

impl<Profile, Provider, Return, Type> ProviderTransferDynamicInput<Profile, Provider, Return>
    for Type
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderTransferValue,
    Type::ImmediateInput: ProviderTransferInputValue<Profile, Provider, Return, Host = Type::Host>,
{
    type Host = Type::Host;
    type View = Type::ImmediateInput;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View {
        Type::ImmediateInput::from_host(call, value)
    }
}

impl<Profile, Provider, Return, Item> ProviderTransferDynamicInput<Profile, Provider, Return>
    for List<Item>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Item: ProviderTransferValue,
    Item::ImmediateListInput: ProviderTransferListInputCodec<Profile, Provider>
        + ProviderTransferListInputValue<Host = Item::Host>,
{
    type Host = HostListType<Item::Host>;
    type View = List<
        Item::ImmediateListInput,
        ProviderTransferListContext<
            Item::Host,
            <Item::ImmediateListInput as ProviderTransferListInputValue>::Decoder,
        >,
    >;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View {
        let decoder = <Item::ImmediateListInput as ProviderTransferListInputCodec<
            Profile,
            Provider,
        >>::decoder(call);
        call.provider_list(value, decoder)
    }
}

impl<Profile, Provider, Return, Type> ProviderDynamicInput<Profile, Provider, Return> for Type
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderValue,
    Type::Input: ProviderInputValue<Profile, Provider, Return> + ProviderValue<Host = Type::Host>,
{
    type Host = Type::Host;
    type View<'call> = Type::Input;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View<'call> {
        Type::Input::from_host(call, value)
    }
}

impl<Profile, Provider, Return, Item> ProviderDynamicInput<Profile, Provider, Return> for List<Item>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Item: ProviderValue<ListInput = Item>,
    Item::ListInput: ProviderListInputCodec<Profile>,
{
    type Host = HostListType<Item::Host>;
    type View<'call> = List<
        Item,
        ProviderListContext<
            'call,
            Item::Host,
            <Item::ListInput as ProviderListInputValue>::Decoder,
        >,
    >;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View<'call> {
        let decoder = <Item::ListInput as ProviderListInputCodec<Profile>>::decoder(call);
        call.provider_list(value, decoder)
    }
}

/// The first source type-argument position of an advanced external payload.
pub type Index0 = HostTypeIndex0;

/// The source type-argument position after `Index`.
pub type Next<Index> = HostTypeIndexNext<Index>;

/// Context-aware source semantics for an advanced retained payload declared
/// with `#[geam::external(name = "...", retained)]`.
pub trait RetainedExternalPayload<Context = LocalRetainedContext>: 'static
where
    Context: RetainedContext,
{
    fn source_equal(&self, context: &Equality<'_, Context>, other: &Self) -> bool;

    fn source_hash(&self, context: &Hashing<'_, Context>) -> u64;

    fn inspect(&self, context: &Inspection<'_, Context>) -> EcoString;
}

impl RetainedContext for LocalRetainedContext {
    type Retained<Index> = HostStoredValue<HostStoredType<Index>>;
    type Dynamic = HostStoredDynamic;
    type Equality<'value> = HostExternalEquality<'value>;
    type Hashing<'value> = HostExternalHashing<'value>;
    type Inspection<'value> = HostExternalInspection<'value>;

    fn retained_equal<Index>(
        context: &Self::Equality<'_>,
        left: &Self::Retained<Index>,
        right: &Self::Retained<Index>,
    ) -> bool {
        context.stored_values_equal(left, right)
    }

    fn retained_hash<Index>(context: &Self::Hashing<'_>, value: &Self::Retained<Index>) -> u64 {
        context.stored_value_hash(value)
    }

    fn retained_inspect<Index>(
        context: &Self::Inspection<'_>,
        value: &Self::Retained<Index>,
    ) -> EcoString {
        context.inspect_stored_value(value)
    }

    fn dynamic_kind(value: &Self::Dynamic) -> DynamicKind {
        DynamicKind::from_family(value.value_family())
    }

    fn dynamic_is_external<Declaration>(value: &Self::Dynamic) -> bool
    where
        Declaration: ProviderExternalDeclaration,
    {
        value.has_external_schema::<Declaration::Schema>()
    }

    fn dynamic_tuple_items(value: Self::Dynamic) -> Result<Box<[Self::Dynamic]>, Self::Dynamic> {
        value.map_tuple_items(|value| value)
    }

    fn dynamic_equal(
        context: &Self::Equality<'_>,
        left: &Self::Dynamic,
        right: &Self::Dynamic,
    ) -> bool {
        context.dynamic_values_equal(left, right)
    }

    fn dynamic_hash(context: &Self::Hashing<'_>, value: &Self::Dynamic) -> u64 {
        context.dynamic_value_hash(value)
    }

    fn dynamic_inspect(context: &Self::Inspection<'_>, value: &Self::Dynamic) -> EcoString {
        context.inspect_dynamic_value(value)
    }
}

impl RetainedContext for ProviderTransferRetainedContext {
    type Retained<Index> = ProviderTransferRetainedValue<Index>;
    type Dynamic = ProviderTransferDynamicStorage;
    type Equality<'value> = AsyncHostExternalEquality<'value>;
    type Hashing<'value> = AsyncHostExternalHashing<'value>;
    type Inspection<'value> = AsyncHostExternalInspection<'value>;

    fn retained_equal<Index>(
        context: &Self::Equality<'_>,
        left: &Self::Retained<Index>,
        right: &Self::Retained<Index>,
    ) -> bool {
        context.provider_stored_values_equal(&left.value, &right.value)
    }

    fn retained_hash<Index>(context: &Self::Hashing<'_>, value: &Self::Retained<Index>) -> u64 {
        context.provider_stored_value_hash(&value.value)
    }

    fn retained_inspect<Index>(
        context: &Self::Inspection<'_>,
        value: &Self::Retained<Index>,
    ) -> EcoString {
        context.provider_inspect_stored_value(&value.value)
    }

    fn dynamic_kind(value: &Self::Dynamic) -> DynamicKind {
        DynamicKind::from_family(value.value.family())
    }

    fn dynamic_is_external<Declaration>(value: &Self::Dynamic) -> bool
    where
        Declaration: ProviderExternalDeclaration,
    {
        value.value.has_external_schema::<Declaration::Schema>()
    }

    fn dynamic_tuple_items(value: Self::Dynamic) -> Result<Box<[Self::Dynamic]>, Self::Dynamic> {
        value
            .value
            .map_tuple_items(|value| ProviderTransferDynamicStorage { value })
            .map_err(|value| ProviderTransferDynamicStorage { value })
    }

    fn dynamic_equal(
        context: &Self::Equality<'_>,
        left: &Self::Dynamic,
        right: &Self::Dynamic,
    ) -> bool {
        context.provider_stored_values_equal(&left.value, &right.value)
    }

    fn dynamic_hash(context: &Self::Hashing<'_>, value: &Self::Dynamic) -> u64 {
        context.provider_stored_value_hash(&value.value)
    }

    fn dynamic_inspect(context: &Self::Inspection<'_>, value: &Self::Dynamic) -> EcoString {
        context.provider_inspect_stored_value(&value.value)
    }
}

impl<Owner, Index, Context> Retained<Owner, Index, Context>
where
    Owner: ProviderStoredOwner,
    Context: RetainedContext,
{
    /// Compares two retained values with Gleam source equality.
    pub fn source_equal(&self, context: &Equality<'_, Context>, other: &Self) -> bool {
        Context::retained_equal(context, &self.value, &other.value)
    }

    /// Hashes this retained value consistently with Gleam source equality.
    pub fn source_hash(&self, context: &Hashing<'_, Context>) -> u64 {
        Context::retained_hash(context, &self.value)
    }

    /// Inspects this retained value with Gleam source formatting.
    pub fn inspect(&self, context: &Inspection<'_, Context>) -> EcoString {
        Context::retained_inspect(context, &self.value)
    }
}

impl<Owner, Context> StoredDynamic<Owner, Context>
where
    Owner: ProviderStoredOwner,
    Context: RetainedContext,
{
    /// Returns the broad source family without exposing its runtime type.
    pub fn kind(&self) -> DynamicKind {
        Context::dynamic_kind(&self.value)
    }

    /// Confirms one generated external declaration without exposing names.
    pub fn is_external<Declaration>(&self) -> bool
    where
        Declaration: ProviderExternalDeclaration,
    {
        Context::dynamic_is_external::<Declaration>(&self.value)
    }

    /// Consumes a retained tuple and retains each element under the same owner.
    ///
    /// Non-tuples are returned unchanged.
    pub fn into_tuple_items(self) -> Result<Box<[Self]>, Self> {
        Context::dynamic_tuple_items(self.value)
            .map(|items| {
                items
                    .into_vec()
                    .into_iter()
                    .map(|value| Self {
                        value,
                        owner: PhantomData,
                    })
                    .collect()
            })
            .map_err(|value| Self {
                value,
                owner: PhantomData,
            })
    }

    /// Compares two existential values with Gleam source equality.
    pub fn source_equal(&self, context: &Equality<'_, Context>, other: &Self) -> bool {
        Context::dynamic_equal(context, &self.value, &other.value)
    }

    /// Hashes this existential value consistently with Gleam source equality.
    pub fn source_hash(&self, context: &Hashing<'_, Context>) -> u64 {
        Context::dynamic_hash(context, &self.value)
    }

    /// Inspects this existential value with Gleam source formatting.
    pub fn inspect(&self, context: &Inspection<'_, Context>) -> EcoString {
        Context::dynamic_inspect(context, &self.value)
    }
}

impl<Owner, Index> Retained<Owner, Index, LocalRetainedContext>
where
    Owner: ProviderStoredOwner,
{
    pub(crate) fn new_local(value: HostStoredValue<HostStoredType<Index>>) -> Self {
        Self {
            value,
            owner: PhantomData,
        }
    }

    pub(crate) fn host(&self) -> &HostStoredValue<HostStoredType<Index>> {
        &self.value
    }
}

impl<Owner> StoredDynamic<Owner, LocalRetainedContext>
where
    Owner: ProviderStoredOwner,
{
    pub(crate) fn new_local(value: HostStoredDynamic) -> Self {
        Self {
            value,
            owner: PhantomData,
        }
    }

    pub(crate) fn host(&self) -> &HostStoredDynamic {
        &self.value
    }
}

impl<Owner, Index> Retained<Owner, Index, ProviderTransferRetainedContext>
where
    Owner: ProviderStoredOwner,
{
    pub(crate) fn new_transfer(value: StoredRuntimeValue<TransferValues>) -> Self {
        Self {
            value: ProviderTransferRetainedValue {
                value,
                index: PhantomData,
            },
            owner: PhantomData,
        }
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue<TransferValues> {
        &self.value.value
    }

    pub(crate) fn clone_transfer(&self) -> Self {
        Self::new_transfer(self.value.value.clone_transfer())
    }
}

impl<Owner> StoredDynamic<Owner, ProviderTransferRetainedContext>
where
    Owner: ProviderStoredOwner,
{
    pub(crate) fn new_transfer(value: StoredRuntimeValue<TransferValues>) -> Self {
        Self {
            value: ProviderTransferDynamicStorage { value },
            owner: PhantomData,
        }
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue<TransferValues> {
        &self.value.value
    }
}

#[cfg(test)]
mod tests {
    use super::{DynamicKind, Index0, Retained};
    use crate::host::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostStoredType,
        HostStoredValue,
    };
    use crate::runtime::StoredRuntimeValue;

    struct Payload;

    impl crate::provider::ProviderStoredOwner for Payload {}

    fn retained(value: i64) -> Retained<Payload, Index0> {
        Retained::new_local(HostStoredValue::<HostStoredType<Index0>>::new(
            StoredRuntimeValue::test_int(value.into()),
        ))
    }

    #[test]
    fn dynamic_kind_covers_every_retained_runtime_family() {
        use crate::provider_support::HostStoredValueFamily;

        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Int),
            DynamicKind::Int
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Float),
            DynamicKind::Float
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::String),
            DynamicKind::String
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::BitArray),
            DynamicKind::BitArray,
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::UtfCodepoint),
            DynamicKind::UtfCodepoint,
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Bool),
            DynamicKind::Bool
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Nil),
            DynamicKind::Nil
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::List),
            DynamicKind::List
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Tuple),
            DynamicKind::Tuple
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Custom),
            DynamicKind::Custom
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::External),
            DynamicKind::External,
        );
        assert_eq!(
            DynamicKind::from_family(HostStoredValueFamily::Function),
            DynamicKind::Function,
        );
    }

    #[test]
    fn retained_values_delegate_each_source_operation_to_its_narrow_context() {
        let first = retained(7);
        let different = retained(8);
        let stored_equal =
            |left: &StoredRuntimeValue, right: &StoredRuntimeValue| std::ptr::eq(left, right);
        let stored_hash = |_: &StoredRuntimeValue| 17;
        let stored_inspect = |_: &StoredRuntimeValue| "Int(7)".into();
        let equality = HostExternalEquality::new(&stored_equal);
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&stored_inspect);

        assert!(first.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &different));
        assert_eq!(first.source_hash(&hashing), 17);
        assert_eq!(first.inspect(&inspection), "Int(7)");
    }
}
