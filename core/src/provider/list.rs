use crate::host::{HostExternalStore, HostType};
use crate::runtime::{
    ExternalPayloadLease, ExternalPayloadView, StoredRuntimeList, StoredRuntimeListCustomFields,
    StoredRuntimeListItem, StoredRuntimeListTupleItems,
};
use std::marker::PhantomData;
use std::ops::Deref;

/// A retained, read-only view of one Gleam `List(Item)` value.
///
/// Providers cannot construct this type directly. A provider function receives
/// it through the `#[geam::function]` adapter and can inspect only its length or
/// one requested item at a time. Returning a received `List` preserves the
/// original runtime list; return a `Vec<Item>` to construct a new Gleam list.
pub struct List<Item, Context = MissingListContext> {
    context: Context,
    item: PhantomData<fn() -> Item>,
}

#[doc(hidden)]
pub struct MissingListContext;

/// Owned retained List used by a transferable provider invocation.
#[doc(hidden)]
pub struct ProviderListContext<HostItem, Decoder> {
    retained: StoredRuntimeList,
    decoder: Decoder,
    host: PhantomData<fn() -> HostItem>,
}

/// Transferable decoder for one exact List item shape.
#[doc(hidden)]
pub trait ProviderListItemDecoder<Item> {
    type View;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View;
}

/// Exact source storage selected by a typed item decoder.
#[doc(hidden)]
pub trait ProviderTypedListItemDecoder<Item>: ProviderListItemDecoder<Item> {
    type Host: HostType;
}

/// One requested transferable runtime List item.
#[doc(hidden)]
pub struct ProviderListItemValue<'value> {
    value: StoredRuntimeListItem<'value>,
}

/// Cloneable access to one transferable external payload store.
#[doc(hidden)]
pub struct ProviderExternalPayloadAccess<Payload> {
    store: HostExternalStore<Payload>,
}

/// An owned external handle that may cross suspension points.
///
/// Payload access is deliberately bounded by [`Self::with`], so no reference
/// or store guard can remain live across an `.await`.
#[doc(hidden)]
pub struct ProviderOwnedExternal<Payload> {
    access: ProviderExternalPayloadAccess<Payload>,
    lease: ExternalPayloadLease,
}

impl<Payload: Send + 'static> Clone for ProviderOwnedExternal<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
            lease: self.lease.clone(),
        }
    }
}

/// A direct-only view of one transferable external payload.
#[doc(hidden)]
pub struct ProviderExternalView<Payload> {
    value: ExternalPayloadView<Payload>,
    lease: ExternalPayloadLease,
}

/// Profile-independent decoder for one scalar List item.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ProviderScalarListDecoder<Scalar>(PhantomData<fn() -> Scalar>);

/// Transferable decoder for one external List item.
#[doc(hidden)]
pub struct ProviderOwnedExternalListDecoder<Payload> {
    access: ProviderExternalPayloadAccess<Payload>,
}

/// Direct-only transferable decoder for one external List item.
#[doc(hidden)]
pub struct ProviderExternalListDecoder<Payload> {
    access: ProviderExternalPayloadAccess<Payload>,
}

impl<Payload: Send + 'static> Clone for ProviderOwnedExternalListDecoder<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
        }
    }
}

impl<Payload: Send + 'static> Clone for ProviderExternalListDecoder<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
        }
    }
}

/// A consuming view over a tuple item in a transferable List.
#[doc(hidden)]
pub struct ProviderListTupleItems<'value> {
    values: StoredRuntimeListTupleItems<'value>,
}

/// A consuming view over a custom item in a transferable List.
#[doc(hidden)]
pub struct ProviderListCustomFields<'value> {
    fields: StoredRuntimeListCustomFields<'value>,
}

impl<Item, HostItem, Decoder> List<Item, ProviderListContext<HostItem, Decoder>>
where
    HostItem: HostType,
    Decoder: ProviderListItemDecoder<Item>,
{
    #[expect(
        clippy::len_without_is_empty,
        reason = "the provider List slice intentionally exposes only len and get"
    )]
    pub fn len(&self) -> usize {
        self.context.retained.len()
    }

    pub fn get(&self, index: usize) -> Option<Decoder::View> {
        self.context.retained.decode_item(index, |value| {
            self.context.decoder.decode(ProviderListItemValue { value })
        })
    }

    #[doc(hidden)]
    pub fn __geam_into_context(self) -> ProviderListContext<HostItem, Decoder> {
        self.context
    }
}

impl<HostItem, Decoder> ProviderListContext<HostItem, Decoder>
where
    HostItem: HostType,
{
    pub(crate) fn new<Item>(retained: StoredRuntimeList, decoder: Decoder) -> List<Item, Self> {
        List {
            context: Self {
                retained,
                decoder,
                host: PhantomData,
            },
            item: PhantomData,
        }
    }

    pub(crate) fn retained(&self) -> &StoredRuntimeList {
        &self.retained
    }
}

impl<'value> ProviderListItemValue<'value> {
    #[doc(hidden)]
    #[allow(private_bounds)]
    pub fn into_scalar<Scalar>(self) -> Scalar
    where
        Scalar: ProviderListScalar,
    {
        Scalar::decode(self.value)
    }

    #[doc(hidden)]
    pub fn into_external<Payload>(
        self,
        access: &ProviderExternalPayloadAccess<Payload>,
    ) -> ProviderOwnedExternal<Payload>
    where
        Payload: Send + 'static,
    {
        ProviderOwnedExternal::new(access.clone(), self.value.into_external_lease())
    }

    #[doc(hidden)]
    pub fn into_external_view<Payload>(
        self,
        access: &ProviderExternalPayloadAccess<Payload>,
    ) -> ProviderExternalView<Payload>
    where
        Payload: Send + 'static,
    {
        let lease = self.value.into_external_lease();
        let value = access.store.view(&lease);
        ProviderExternalView::new(value, lease)
    }

    #[doc(hidden)]
    pub fn into_tuple(self) -> ProviderListTupleItems<'value> {
        ProviderListTupleItems {
            values: self.value.into_tuple_items(),
        }
    }

    #[doc(hidden)]
    pub fn into_custom(self) -> ProviderListCustomFields<'value> {
        ProviderListCustomFields {
            fields: self.value.into_custom_fields(),
        }
    }

    #[doc(hidden)]
    pub fn into_typed_list<Item, Decoder>(
        self,
        decoder: Decoder,
    ) -> List<Item, ProviderListContext<Decoder::Host, Decoder>>
    where
        Decoder: ProviderTypedListItemDecoder<Item>,
    {
        ProviderListContext::new(self.value.into_list(), decoder)
    }
}

impl ProviderListTupleItems<'_> {
    #[doc(hidden)]
    pub fn take_item(&mut self, index: usize) -> ProviderListItemValue<'_> {
        ProviderListItemValue {
            value: self.values.take_item(index),
        }
    }
}

impl ProviderListCustomFields<'_> {
    #[doc(hidden)]
    pub fn constructor(&self) -> usize {
        self.fields.constructor()
    }

    #[doc(hidden)]
    pub fn take_field(&mut self, index: usize) -> ProviderListItemValue<'_> {
        ProviderListItemValue {
            value: self.fields.take_field(index),
        }
    }
}

impl ProviderListItemValue<'_> {
    pub(crate) fn into_stored_external(
        self,
        retention: &crate::runtime::ValueRetention,
    ) -> (crate::runtime::StoredRuntimeValue, ExternalPayloadLease) {
        self.value.into_stored_external(retention)
    }

    pub(crate) fn into_stored(
        self,
        retention: &crate::runtime::ValueRetention,
    ) -> crate::runtime::StoredRuntimeValue {
        self.value.into_stored(retention)
    }

    pub(crate) fn into_callable(self) -> crate::runtime::RetainedCallable {
        self.value.into_callable()
    }
}

impl<Payload: Send + 'static> ProviderExternalPayloadAccess<Payload> {
    pub(crate) fn new(store: &HostExternalStore<Payload>) -> Self {
        Self {
            store: store.clone_handle(),
        }
    }
}

impl<Payload: Send + 'static> Clone for ProviderExternalPayloadAccess<Payload> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone_handle(),
        }
    }
}

impl<Payload: Send + 'static> ProviderOwnedExternal<Payload> {
    pub(crate) fn new(
        access: ProviderExternalPayloadAccess<Payload>,
        lease: ExternalPayloadLease,
    ) -> Self {
        Self { access, lease }
    }

    pub fn with<Output>(&self, read: impl FnOnce(&Payload) -> Output) -> Output {
        self.access.store.with_view(&self.lease, read)
    }

    pub(crate) fn into_lease(self) -> ExternalPayloadLease {
        self.lease
    }
}

impl<Payload> Deref for ProviderExternalView<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Payload> ProviderExternalView<Payload> {
    pub(crate) fn new(value: ExternalPayloadView<Payload>, lease: ExternalPayloadLease) -> Self {
        Self { value, lease }
    }

    pub(crate) fn into_lease(self) -> ExternalPayloadLease {
        self.lease
    }
}

impl<Scalar> ProviderScalarListDecoder<Scalar> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Scalar> ProviderTypedListItemDecoder<Scalar> for ProviderScalarListDecoder<Scalar>
where
    Scalar: ProviderListScalar + HostType,
{
    type Host = Scalar;
}

impl<Scalar> ProviderListItemDecoder<Scalar> for ProviderScalarListDecoder<Scalar>
where
    Scalar: ProviderListScalar,
{
    type View = Scalar;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        value.into_scalar()
    }
}

impl<Payload: Send + 'static> ProviderOwnedExternalListDecoder<Payload> {
    pub fn new(access: ProviderExternalPayloadAccess<Payload>) -> Self {
        Self { access }
    }
}

impl<Payload: Send + 'static> ProviderExternalListDecoder<Payload> {
    pub fn new(access: ProviderExternalPayloadAccess<Payload>) -> Self {
        Self { access }
    }
}

impl<Payload> ProviderTypedListItemDecoder<ProviderOwnedExternal<Payload>>
    for ProviderOwnedExternalListDecoder<Payload>
where
    Payload: super::ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
}

impl<Payload: Send + 'static> ProviderListItemDecoder<ProviderOwnedExternal<Payload>>
    for ProviderOwnedExternalListDecoder<Payload>
{
    type View = ProviderOwnedExternal<Payload>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        value.into_external(&self.access)
    }
}

impl<Payload> ProviderTypedListItemDecoder<ProviderExternalView<Payload>>
    for ProviderExternalListDecoder<Payload>
where
    Payload: super::ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
}

impl<Payload: Send + 'static> ProviderListItemDecoder<ProviderExternalView<Payload>>
    for ProviderExternalListDecoder<Payload>
{
    type View = ProviderExternalView<Payload>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        value.into_external_view(&self.access)
    }
}

trait ProviderListScalar: Sized {
    fn decode(value: StoredRuntimeListItem<'_>) -> Self;
}

macro_rules! provider_list_scalar {
    ($type:ty, $method:ident) => {
        impl ProviderListScalar for $type {
            fn decode(value: StoredRuntimeListItem<'_>) -> Self {
                value.$method()
            }
        }
    };
}

provider_list_scalar!(num_bigint::BigInt, into_int);
provider_list_scalar!(f64, into_float);
provider_list_scalar!(crate::StringValue, into_string);
provider_list_scalar!(crate::BitArrayValue, into_bit_array);
provider_list_scalar!(char, into_utf_codepoint);
provider_list_scalar!(bool, into_bool);
provider_list_scalar!((), into_nil);

#[cfg(test)]
mod tests {
    use super::{
        ProviderExternalListDecoder, ProviderExternalPayloadAccess, ProviderListContext,
        ProviderListItemDecoder, ProviderListItemValue, ProviderOwnedExternal,
        ProviderOwnedExternalListDecoder,
    };
    use crate::host::HostExternalStore;
    use crate::runtime::StoredRuntimeList;
    use num_bigint::BigInt;
    use std::sync::mpsc::Receiver;

    struct IntDecoder;

    impl ProviderListItemDecoder<BigInt> for IntDecoder {
        type View = BigInt;

        fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
            value.into_scalar()
        }
    }

    #[test]
    fn retained_list_length_and_indexing_decode_only_requested_items() {
        let retained = StoredRuntimeList::test_ints(vec![1.into(), 2.into()]);
        let list = ProviderListContext::<BigInt, _>::new::<BigInt>(retained, IntDecoder);

        assert_eq!(list.context.retained.item_reads(), 0);
        assert_eq!(list.len(), 2);
        assert_eq!(list.context.retained.item_reads(), 0);
        assert_eq!(list.get(1), Some(2.into()));
        assert_eq!(list.context.retained.item_reads(), 1);
        assert_eq!(list.get(2), None);
        assert_eq!(list.context.retained.item_reads(), 2);
    }

    #[test]
    fn transferable_external_decoders_clone_store_access_without_payload_bounds() {
        let store = HostExternalStore::<Receiver<()>>::default();
        let access = ProviderExternalPayloadAccess::new(&store);
        let owned = ProviderOwnedExternalListDecoder::new(access.clone());
        let direct = ProviderExternalListDecoder::new(access);

        let _owned_clone = owned.clone();
        let _direct_clone = direct.clone();
    }

    #[test]
    fn owned_external_aliases_share_a_non_clone_payload_until_the_last_drop() {
        let (sender, receiver) = std::sync::mpsc::channel::<usize>();
        let store = HostExternalStore::default();
        let label = crate::host::HostStoredValue::<BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(7.into()),
        );
        let lease = store.insert(
            (receiver, label),
            |context, left, right| context.stored_values_equal(&left.1, &right.1),
            |context, value| context.stored_value_hash(&value.1),
            |context, value| format!("Receiver({})", context.inspect_stored_value(&value.1)).into(),
            |_| None,
        );
        let original =
            ProviderOwnedExternal::new(ProviderExternalPayloadAccess::new(&store), lease);
        let alias = original.clone();
        assert_eq!(original.lease.identity(), alias.lease.identity());
        let equal = crate::host::RetainedValueEquality::new(&|_, _| true);
        let hash = crate::host::RetainedValueHashing::new(&|_| 7);
        let inspect = crate::host::RetainedValueInspection::new(&|_| "7".into());
        assert!(original.lease.source_equal(&equal, &alias.lease));
        assert_eq!(
            original.lease.source_hash(&hash),
            alias.lease.source_hash(&hash)
        );
        assert_eq!(original.lease.inspection(&inspect), "Receiver(7)");
        sender.send(7).unwrap();
        assert_eq!(original.with(|payload| payload.0.try_recv().unwrap()), 7);
        drop(original);
        drop(store);
        sender.send(9).unwrap();
        assert_eq!(alias.lease.inspection(&inspect), "Receiver(7)");
        assert_eq!(alias.with(|payload| payload.0.try_recv().unwrap()), 9);
        drop(alias);
        assert_eq!(sender.send(11), Err(std::sync::mpsc::SendError(11)));
    }
}
