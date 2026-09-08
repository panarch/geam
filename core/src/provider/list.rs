use crate::host::{
    AsyncHostExternalStore, ExternalPayloadLease, ExternalPayloadView, HostExternalStore, HostList,
    HostType,
};
use crate::runtime::{
    StoredRuntimeList, StoredRuntimeListCustomFields, StoredRuntimeListItem,
    StoredRuntimeListTupleItems, TransferExternalPayloadLease, TransferExternalPayloadView,
    TransferValues,
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

/// The concrete retained-list owner inserted by provider macro expansion.
#[doc(hidden)]
pub struct ProviderListContext<'call, HostItem, Decoder> {
    host: HostList<'call, HostItem>,
    retained: StoredRuntimeList,
    decoder: Decoder,
}

/// An input-only retained List nested inside another source value.
#[doc(hidden)]
pub struct ProviderInputListContext<Decoder> {
    retained: StoredRuntimeList,
    decoder: Decoder,
}

/// Owned retained List used by a transferable provider invocation.
#[doc(hidden)]
pub struct ProviderTransferListContext<HostItem, Decoder> {
    retained: StoredRuntimeList<TransferValues>,
    decoder: Decoder,
    host: PhantomData<fn() -> HostItem>,
}

/// An input-only transferable List nested inside another source value.
#[doc(hidden)]
pub struct ProviderTransferInputListContext<Decoder> {
    retained: StoredRuntimeList<TransferValues>,
    decoder: Decoder,
}

/// The statically generated decoder for one exact List item shape.
#[doc(hidden)]
pub trait ProviderListItemDecoder<Item> {
    type View;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View;
}

/// Transferable decoder for one exact List item shape.
#[doc(hidden)]
pub trait ProviderTransferListItemDecoder<Item> {
    type View;

    fn decode(&self, value: ProviderTransferListItemValue<'_>) -> Self::View;
}

/// One requested runtime List item passed to a generated typed decoder.
#[doc(hidden)]
pub struct ProviderListItemValue<'value> {
    value: StoredRuntimeListItem<'value>,
}

/// One requested transferable runtime List item.
#[doc(hidden)]
pub struct ProviderTransferListItemValue<'value> {
    value: StoredRuntimeListItem<'value, TransferValues>,
}

/// Typed access to one provider-owned external payload store.
#[doc(hidden)]
pub struct ProviderExternalPayloadAccess<Payload> {
    store: HostExternalStore<Payload>,
}

/// An external List item that retains its store entry without cloning the payload.
#[doc(hidden)]
pub struct ProviderExternalItem<Payload> {
    value: ExternalPayloadView<Payload>,
    lease: ExternalPayloadLease,
}

/// Cloneable access to one transferable external payload store.
#[doc(hidden)]
pub struct ProviderTransferExternalPayloadAccess<Payload> {
    store: AsyncHostExternalStore<Payload>,
}

/// An owned external handle that may cross suspension points.
///
/// Payload access is deliberately bounded by [`Self::with`], so no reference
/// or store guard can remain live across an `.await`.
#[doc(hidden)]
pub struct ProviderTransferExternalItem<Payload> {
    access: ProviderTransferExternalPayloadAccess<Payload>,
    lease: TransferExternalPayloadLease,
}

/// A direct-only view of one transferable external payload.
#[doc(hidden)]
pub struct ProviderTransferExternalView<Payload> {
    value: TransferExternalPayloadView<Payload>,
    lease: TransferExternalPayloadLease,
}

/// Profile-independent decoder for one scalar List item.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ProviderScalarListDecoder<Scalar>(PhantomData<fn() -> Scalar>);

/// Profile-independent decoder for one external List item.
#[doc(hidden)]
pub struct ProviderExternalListDecoder<Payload> {
    access: ProviderExternalPayloadAccess<Payload>,
}

/// Transferable decoder for one external List item.
#[doc(hidden)]
pub struct ProviderTransferExternalListDecoder<Payload> {
    access: ProviderTransferExternalPayloadAccess<Payload>,
}

/// Direct-only transferable decoder for one external List item.
#[doc(hidden)]
pub struct ProviderTransferExternalViewListDecoder<Payload> {
    access: ProviderTransferExternalPayloadAccess<Payload>,
}

impl<Payload: 'static> Clone for ProviderExternalListDecoder<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
        }
    }
}

impl<Payload: Send + 'static> Clone for ProviderTransferExternalListDecoder<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
        }
    }
}

impl<Payload: Send + 'static> Clone for ProviderTransferExternalViewListDecoder<Payload> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
        }
    }
}

/// A consuming view over the elements of one runtime tuple List item.
#[doc(hidden)]
pub struct ProviderListTupleItems<'value> {
    values: StoredRuntimeListTupleItems<'value>,
}

/// A consuming view over one runtime custom value stored in a List.
#[doc(hidden)]
pub struct ProviderListCustomFields<'value> {
    fields: StoredRuntimeListCustomFields<'value>,
}

/// A consuming view over a tuple item in a transferable List.
#[doc(hidden)]
pub struct ProviderTransferListTupleItems<'value> {
    values: StoredRuntimeListTupleItems<'value, TransferValues>,
}

/// A consuming view over a custom item in a transferable List.
#[doc(hidden)]
pub struct ProviderTransferListCustomFields<'value> {
    fields: StoredRuntimeListCustomFields<'value, TransferValues>,
}

impl<'call, Item, HostItem, Decoder> List<Item, ProviderListContext<'call, HostItem, Decoder>>
where
    HostItem: HostType,
    Decoder: ProviderListItemDecoder<Item>,
{
    /// Returns the List length without decoding an item.
    #[expect(
        clippy::len_without_is_empty,
        reason = "the first provider List slice intentionally exposes only len and get"
    )]
    pub fn len(&self) -> usize {
        self.context.retained.len()
    }

    /// Decodes only the item at `index` through the statically generated codec.
    pub fn get(&self, index: usize) -> Option<Decoder::View> {
        self.context.retained.decode_item(index, |value| {
            self.context.decoder.decode(ProviderListItemValue { value })
        })
    }

    #[doc(hidden)]
    pub fn __geam_into_context(self) -> ProviderListContext<'call, HostItem, Decoder> {
        self.context
    }
}

impl<Item, Decoder> List<Item, ProviderInputListContext<Decoder>>
where
    Decoder: ProviderListItemDecoder<Item>,
{
    /// Returns the nested List length without decoding an item.
    #[expect(
        clippy::len_without_is_empty,
        reason = "the provider List slice intentionally exposes only len and get"
    )]
    pub fn len(&self) -> usize {
        self.context.retained.len()
    }

    /// Decodes only the nested List item at `index`.
    pub fn get(&self, index: usize) -> Option<Decoder::View> {
        self.context.retained.decode_item(index, |value| {
            self.context.decoder.decode(ProviderListItemValue { value })
        })
    }
}

impl<Item, HostItem, Decoder> List<Item, ProviderTransferListContext<HostItem, Decoder>>
where
    HostItem: HostType,
    Decoder: ProviderTransferListItemDecoder<Item>,
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
            self.context
                .decoder
                .decode(ProviderTransferListItemValue { value })
        })
    }

    #[doc(hidden)]
    pub fn __geam_into_transfer_context(self) -> ProviderTransferListContext<HostItem, Decoder> {
        self.context
    }
}

impl<Item, Decoder> List<Item, ProviderTransferInputListContext<Decoder>>
where
    Decoder: ProviderTransferListItemDecoder<Item>,
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
            self.context
                .decoder
                .decode(ProviderTransferListItemValue { value })
        })
    }
}

impl<'call, HostItem, Decoder> ProviderListContext<'call, HostItem, Decoder>
where
    HostItem: HostType,
{
    pub(crate) fn new(
        host: HostList<'call, HostItem>,
        retained: StoredRuntimeList,
        decoder: Decoder,
    ) -> Self {
        Self {
            host,
            retained,
            decoder,
        }
    }

    pub(crate) fn into_list<Item>(self) -> List<Item, Self> {
        List {
            context: self,
            item: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host(self) -> HostList<'call, HostItem> {
        self.host
    }
}

impl<Decoder> ProviderInputListContext<Decoder> {
    pub(crate) fn new<Item>(retained: StoredRuntimeList, decoder: Decoder) -> List<Item, Self> {
        List {
            context: Self { retained, decoder },
            item: PhantomData,
        }
    }
}

impl<HostItem, Decoder> ProviderTransferListContext<HostItem, Decoder>
where
    HostItem: HostType,
{
    pub(crate) fn new<Item>(
        retained: StoredRuntimeList<TransferValues>,
        decoder: Decoder,
    ) -> List<Item, Self> {
        List {
            context: Self {
                retained,
                decoder,
                host: PhantomData,
            },
            item: PhantomData,
        }
    }

    pub(crate) fn retained(&self) -> &StoredRuntimeList<TransferValues> {
        &self.retained
    }
}

impl<Decoder> ProviderTransferInputListContext<Decoder> {
    pub(crate) fn new<Item>(
        retained: StoredRuntimeList<TransferValues>,
        decoder: Decoder,
    ) -> List<Item, Self> {
        List {
            context: Self { retained, decoder },
            item: PhantomData,
        }
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
    ) -> ProviderExternalItem<Payload>
    where
        Payload: 'static,
    {
        let lease = self.value.into_external_lease();
        let value = access.store.view(&lease);
        ProviderExternalItem { value, lease }
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
    pub fn into_list<Item, Decoder>(
        self,
        decoder: Decoder,
    ) -> List<Item, ProviderInputListContext<Decoder>>
    where
        Decoder: ProviderListItemDecoder<Item>,
    {
        ProviderInputListContext::new(self.value.into_list(), decoder)
    }
}

impl<'value> ProviderTransferListItemValue<'value> {
    #[doc(hidden)]
    #[allow(private_bounds)]
    pub fn into_scalar<Scalar>(self) -> Scalar
    where
        Scalar: ProviderTransferListScalar,
    {
        Scalar::decode(self.value)
    }

    #[doc(hidden)]
    pub fn into_external<Payload>(
        self,
        access: &ProviderTransferExternalPayloadAccess<Payload>,
    ) -> ProviderTransferExternalItem<Payload>
    where
        Payload: Send + 'static,
    {
        ProviderTransferExternalItem::new(access.clone(), self.value.into_external_lease())
    }

    #[doc(hidden)]
    pub fn into_external_view<Payload>(
        self,
        access: &ProviderTransferExternalPayloadAccess<Payload>,
    ) -> ProviderTransferExternalView<Payload>
    where
        Payload: Send + 'static,
    {
        let lease = self.value.into_external_lease();
        let value = access.store.view(&lease);
        ProviderTransferExternalView::new(value, lease)
    }

    #[doc(hidden)]
    pub fn into_tuple(self) -> ProviderTransferListTupleItems<'value> {
        ProviderTransferListTupleItems {
            values: self.value.into_tuple_items(),
        }
    }

    #[doc(hidden)]
    pub fn into_custom(self) -> ProviderTransferListCustomFields<'value> {
        ProviderTransferListCustomFields {
            fields: self.value.into_custom_fields(),
        }
    }

    #[doc(hidden)]
    pub fn into_list<Item, Decoder>(
        self,
        decoder: Decoder,
    ) -> List<Item, ProviderTransferInputListContext<Decoder>>
    where
        Decoder: ProviderTransferListItemDecoder<Item>,
    {
        ProviderTransferInputListContext::new(self.value.into_list(), decoder)
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

impl ProviderTransferListTupleItems<'_> {
    #[doc(hidden)]
    pub fn take_item(&mut self, index: usize) -> ProviderTransferListItemValue<'_> {
        ProviderTransferListItemValue {
            value: self.values.take_item(index),
        }
    }
}

impl ProviderTransferListCustomFields<'_> {
    #[doc(hidden)]
    pub fn constructor(&self) -> usize {
        self.fields.constructor()
    }

    #[doc(hidden)]
    pub fn take_field(&mut self, index: usize) -> ProviderTransferListItemValue<'_> {
        ProviderTransferListItemValue {
            value: self.fields.take_field(index),
        }
    }
}

impl<Payload: 'static> ProviderExternalPayloadAccess<Payload> {
    pub(crate) fn new(store: &HostExternalStore<Payload>) -> Self {
        Self {
            store: store.clone_handle(),
        }
    }
}

impl<Payload: 'static> Clone for ProviderExternalPayloadAccess<Payload> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone_handle(),
        }
    }
}

impl<Payload> Deref for ProviderExternalItem<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Payload> ProviderExternalItem<Payload> {
    pub(crate) fn new(value: ExternalPayloadView<Payload>, lease: ExternalPayloadLease) -> Self {
        Self { value, lease }
    }

    pub(crate) fn into_lease(self) -> ExternalPayloadLease {
        self.lease
    }
}

impl<Payload: Send + 'static> ProviderTransferExternalPayloadAccess<Payload> {
    pub(crate) fn new(store: &AsyncHostExternalStore<Payload>) -> Self {
        Self {
            store: store.clone_handle(),
        }
    }
}

impl<Payload: Send + 'static> Clone for ProviderTransferExternalPayloadAccess<Payload> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone_handle(),
        }
    }
}

impl<Payload: Send + 'static> ProviderTransferExternalItem<Payload> {
    pub(crate) fn new(
        access: ProviderTransferExternalPayloadAccess<Payload>,
        lease: TransferExternalPayloadLease,
    ) -> Self {
        Self { access, lease }
    }

    pub fn with<Output>(&self, read: impl FnOnce(&Payload) -> Output) -> Output {
        self.access.store.with_view(&self.lease, read)
    }

    pub(crate) fn into_lease(self) -> TransferExternalPayloadLease {
        self.lease
    }
}

impl<Payload> Deref for ProviderTransferExternalView<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Payload> ProviderTransferExternalView<Payload> {
    pub(crate) fn new(
        value: TransferExternalPayloadView<Payload>,
        lease: TransferExternalPayloadLease,
    ) -> Self {
        Self { value, lease }
    }

    pub(crate) fn into_lease(self) -> TransferExternalPayloadLease {
        self.lease
    }
}

impl<Scalar> ProviderScalarListDecoder<Scalar> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
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

impl<Scalar> ProviderTransferListItemDecoder<Scalar> for ProviderScalarListDecoder<Scalar>
where
    Scalar: ProviderTransferListScalar,
{
    type View = Scalar;

    fn decode(&self, value: ProviderTransferListItemValue<'_>) -> Self::View {
        value.into_scalar()
    }
}

impl<Payload: 'static> ProviderExternalListDecoder<Payload> {
    pub fn new(access: ProviderExternalPayloadAccess<Payload>) -> Self {
        Self { access }
    }
}

impl<Payload: Send + 'static> ProviderTransferExternalListDecoder<Payload> {
    pub fn new(access: ProviderTransferExternalPayloadAccess<Payload>) -> Self {
        Self { access }
    }
}

impl<Payload: Send + 'static> ProviderTransferExternalViewListDecoder<Payload> {
    pub fn new(access: ProviderTransferExternalPayloadAccess<Payload>) -> Self {
        Self { access }
    }
}

impl<Payload: 'static> ProviderListItemDecoder<Payload> for ProviderExternalListDecoder<Payload> {
    type View = ProviderExternalItem<Payload>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        value.into_external(&self.access)
    }
}

impl<Payload: Send + 'static> ProviderTransferListItemDecoder<ProviderTransferExternalItem<Payload>>
    for ProviderTransferExternalListDecoder<Payload>
{
    type View = ProviderTransferExternalItem<Payload>;

    fn decode(&self, value: ProviderTransferListItemValue<'_>) -> Self::View {
        value.into_external(&self.access)
    }
}

impl<Payload: Send + 'static> ProviderTransferListItemDecoder<ProviderTransferExternalView<Payload>>
    for ProviderTransferExternalViewListDecoder<Payload>
{
    type View = ProviderTransferExternalView<Payload>;

    fn decode(&self, value: ProviderTransferListItemValue<'_>) -> Self::View {
        value.into_external_view(&self.access)
    }
}

trait ProviderListScalar: Sized {
    fn decode(value: StoredRuntimeListItem) -> Self;
}

trait ProviderTransferListScalar: Sized {
    fn decode(value: StoredRuntimeListItem<'_, TransferValues>) -> Self;
}

macro_rules! provider_list_scalar {
    ($type:ty, $method:ident) => {
        impl ProviderListScalar for $type {
            fn decode(value: StoredRuntimeListItem) -> Self {
                value.$method()
            }
        }

        impl ProviderTransferListScalar for $type {
            fn decode(value: StoredRuntimeListItem<'_, TransferValues>) -> Self {
                value.$method()
            }
        }
    };
}

provider_list_scalar!(num_bigint::BigInt, into_int);
provider_list_scalar!(f64, into_float);
provider_list_scalar!(ecow::EcoString, into_string);
provider_list_scalar!(crate::BitArrayValue, into_bit_array);
provider_list_scalar!(char, into_utf_codepoint);
provider_list_scalar!(bool, into_bool);
provider_list_scalar!((), into_nil);

#[cfg(test)]
mod tests {
    use super::{
        ProviderListContext, ProviderListItemDecoder, ProviderListItemValue,
        ProviderTransferExternalListDecoder, ProviderTransferExternalPayloadAccess,
        ProviderTransferExternalViewListDecoder,
    };
    use crate::host::{AsyncHostExternalStore, HostList, HostListToken};
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
        let host = HostList::<BigInt>::new(HostListToken::Stored(0));
        let list = ProviderListContext::new(host, retained, IntDecoder).into_list::<BigInt>();

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
        let store = AsyncHostExternalStore::<Receiver<()>>::default();
        let access = ProviderTransferExternalPayloadAccess::new(&store);
        let owned = ProviderTransferExternalListDecoder::new(access.clone());
        let direct = ProviderTransferExternalViewListDecoder::new(access);

        let _owned_clone = owned.clone();
        let _direct_clone = direct.clone();
    }
}
