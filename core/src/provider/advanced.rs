use crate::host::{
    HostCall, HostExternalEquality, HostExternalHashing, HostExternalInspection, HostListType,
    HostProfile, HostProvider, HostStoredDynamic, HostStoredType, HostStoredValue, HostType,
    HostTypeIndex0, HostTypeIndexNext,
};
use crate::provider::{
    List, ProviderConstructions, ProviderExternalDeclaration, ProviderInputValue,
    ProviderListContext, ProviderListInputCodec, ProviderListInputValue, ProviderNoConstructions,
    ProviderOutputValue, ProviderStoredOwner, ProviderValue, ProviderValueContext,
    ProviderValueForms, Value,
};
use crate::runtime::StoredRuntimeValue;
use ecow::EcoString;
use std::marker::PhantomData;

/// Source-equality access for retained values in one immutable payload.
pub type Equality<'value> = HostExternalEquality<'value>;

/// Source-hash access for retained values in one immutable payload.
pub type Hashing<'value> = HostExternalHashing<'value>;

/// Source-inspection access for retained values in one immutable payload.
pub type Inspection<'value> = HostExternalInspection<'value>;

/// One retained source value owned by an advanced external payload.
///
/// The payload type is the owner brand. The argument index identifies the
/// corresponding source type parameter. Values are created only by
/// [`super::Call::store`] followed by the generated external boundary.
pub struct Retained<Owner, Index> {
    value: HostStoredValue<HostStoredType<Index>>,
    owner: PhantomData<fn() -> Owner>,
}

/// One existential source value owned by an advanced external payload.
///
/// The exact specialized source type stays sealed inside Geam. Providers can
/// inspect its broad family, confirm a generated external declaration, or
/// request an exact typed restore through an active [`crate::provider::Call`].
pub struct StoredDynamic<Owner> {
    value: HostStoredDynamic,
    owner: PhantomData<fn() -> Owner>,
}

/// An existing external source value with its original runtime identity.
///
/// This advanced input is useful when a provider must pass an external value
/// to a callback or return it unchanged. Dereferencing it borrows the ordinary
/// Rust payload; consuming it preserves the original source handle.
pub type External<Payload> = crate::provider::ProviderExternalView<Payload>;

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

/// Static pass-through into transferable existential retention.
#[doc(hidden)]
pub trait ProviderDynamicValue<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;

    fn into_stored<Owner>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> StoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner;
}

impl<'call, Profile, Provider, Return, Type, Host>
    ProviderDynamicValue<'call, Profile, Provider, Return>
    for Value<Type, ProviderValueContext<Host>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Host = Host;

    fn into_stored<Owner>(
        self,
        _call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> StoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        StoredDynamic::from_runtime_value(self.into_stored())
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

    fn into_stored<Owner>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> StoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        let value = self.into_host(call, &ProviderConstructions::none());
        StoredDynamic::from_runtime_value(call.retain_value::<Self::Host>(value))
    }
}

impl<'call, Profile, Provider, Return, Item, HostItem, Decoder>
    ProviderDynamicValue<'call, Profile, Provider, Return>
    for List<Item, ProviderListContext<HostItem, Decoder>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    HostItem: HostType,
    Decoder: crate::provider::ProviderListItemDecoder<Item>,
{
    type Host = HostListType<HostItem>;

    fn into_stored<Owner>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> StoredDynamic<Owner>
    where
        Owner: ProviderStoredOwner,
    {
        let value = call.provider_list_from_input(self);
        StoredDynamic::from_runtime_value(call.retain_value::<Self::Host>(value))
    }
}

/// Static input conversion used by transferable existential restores.
#[doc(hidden)]
pub trait ProviderDynamicInput<Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;
    type View;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View;
}

impl<Profile, Provider, Return, Type> ProviderDynamicInput<Profile, Provider, Return> for Type
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderValueForms,
    Type::ImmediateInput: ProviderInputValue<Profile, Provider, Return, Host = Type::Host>,
{
    type Host = Type::Host;
    type View = Type::ImmediateInput;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View {
        Type::ImmediateInput::from_host(call, value)
    }
}

impl<Profile, Provider, Return, Item> ProviderDynamicInput<Profile, Provider, Return> for List<Item>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Item: ProviderValueForms,
    Item::ImmediateListInput:
        ProviderListInputCodec<Profile, Provider> + ProviderListInputValue<Host = Item::Host>,
{
    type Host = HostListType<Item::Host>;
    type View = List<
        Item::ImmediateListInput,
        ProviderListContext<
            Item::Host,
            <Item::ImmediateListInput as ProviderListInputValue>::Decoder,
        >,
    >;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self::View {
        let decoder =
            <Item::ImmediateListInput as ProviderListInputCodec<Profile, Provider>>::decoder(call);
        call.provider_retained_list(value, decoder)
    }
}

/// The first source type-argument position of an advanced external payload.
pub type Index0 = HostTypeIndex0;

/// The source type-argument position after `Index`.
pub type Next<Index> = HostTypeIndexNext<Index>;

/// Context-aware source semantics for an advanced retained payload declared
/// with `#[geam::external(name = "...", retained)]`.
pub trait RetainedExternalPayload: 'static {
    fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool;

    fn source_hash(&self, context: &Hashing<'_>) -> u64;

    fn inspect(&self, context: &Inspection<'_>) -> EcoString;
}

impl<Owner, Index> Retained<Owner, Index>
where
    Owner: ProviderStoredOwner,
{
    /// Compares two retained values with Gleam source equality.
    pub fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
        context.stored_values_equal(&self.value, &other.value)
    }

    /// Hashes this retained value consistently with Gleam source equality.
    pub fn source_hash(&self, context: &Hashing<'_>) -> u64 {
        context.stored_value_hash(&self.value)
    }

    /// Inspects this retained value with Gleam source formatting.
    pub fn inspect(&self, context: &Inspection<'_>) -> EcoString {
        context.inspect_stored_value(&self.value)
    }

    pub(crate) fn from_host_value(value: HostStoredValue<HostStoredType<Index>>) -> Self {
        Self {
            value,
            owner: PhantomData,
        }
    }

    pub(crate) fn from_runtime_value(value: StoredRuntimeValue) -> Self {
        Self {
            value: HostStoredValue::new(value),
            owner: PhantomData,
        }
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue {
        &self.value.value
    }

    pub(crate) fn clone_retained(&self) -> Self {
        Self::from_runtime_value(self.value.value.clone_retained())
    }
}

impl<Owner> StoredDynamic<Owner>
where
    Owner: ProviderStoredOwner,
{
    /// Returns the broad source family without exposing its runtime type.
    pub fn kind(&self) -> DynamicKind {
        DynamicKind::from_family(self.value.value_family())
    }

    /// Confirms one generated external declaration without exposing names.
    pub fn is_external<Declaration>(&self) -> bool
    where
        Declaration: ProviderExternalDeclaration,
    {
        self.value.has_external_schema::<Declaration::Schema>()
    }

    /// Consumes a retained tuple and retains each element under the same owner.
    ///
    /// Non-tuples are returned unchanged.
    #[expect(
        clippy::result_large_err,
        reason = "non-tuples retain the original value without another heap allocation"
    )]
    pub fn into_tuple_items(self) -> Result<Box<[Self]>, Self> {
        self.value
            .map_tuple_items(|value| value)
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
    pub fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
        context.dynamic_values_equal(&self.value, &other.value)
    }

    /// Hashes this existential value consistently with Gleam source equality.
    pub fn source_hash(&self, context: &Hashing<'_>) -> u64 {
        context.dynamic_value_hash(&self.value)
    }

    /// Inspects this existential value with Gleam source formatting.
    pub fn inspect(&self, context: &Inspection<'_>) -> EcoString {
        context.inspect_dynamic_value(&self.value)
    }

    pub(crate) fn from_host_value(value: HostStoredDynamic) -> Self {
        Self {
            value,
            owner: PhantomData,
        }
    }

    pub(crate) fn from_runtime_value(value: StoredRuntimeValue) -> Self {
        Self {
            value: HostStoredDynamic::new(value),
            owner: PhantomData,
        }
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue {
        self.value.runtime_value()
    }
}

#[cfg(test)]
mod tests {
    use super::{DynamicKind, Index0, Retained};
    use crate::host::{HostStoredType, HostStoredValue};
    use crate::runtime::StoredRuntimeValue;

    struct Payload;

    impl crate::provider::ProviderStoredOwner for Payload {}

    fn retained(value: i64) -> Retained<Payload, Index0> {
        Retained::from_host_value(HostStoredValue::<HostStoredType<Index0>>::new(
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
        let comparisons = std::cell::Cell::new(0);
        let stored_equal = |_: &crate::runtime::RetainedValueRef,
                            _: &crate::runtime::RetainedValueRef| {
            let comparison = comparisons.get();
            comparisons.set(comparison + 1);
            comparison == 0
        };
        let stored_hash = |_: &crate::runtime::RetainedValueRef| 17;
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| "Int(7)".into();
        let raw_equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let equality = crate::host::HostExternalEquality(&raw_equality);
        let raw_hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        let hashing = crate::host::HostExternalHashing(&raw_hashing);
        let raw_inspection = crate::host::RetainedValueInspection::new(&stored_inspect);
        let inspection = crate::host::HostExternalInspection(&raw_inspection);

        assert!(first.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &different));
        assert_eq!(comparisons.get(), 2);
        assert_eq!(first.source_hash(&hashing), 17);
        assert_eq!(first.inspect(&inspection), "Int(7)");
    }
}
