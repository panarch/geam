use super::HostType;
use crate::runtime::StoredRuntimeValue;
use std::marker::PhantomData;

/// Owned input layout used by generated immediate provider adapters.
///
/// The sealed host type still owns the source schema and scoped layout. This
/// adds a retained representation for functions which require no active call.
#[doc(hidden)]
#[allow(private_bounds)]
pub trait HostRetainedType: HostType + RetainedAbi<Self::Retained> {
    type Retained: Send + 'static;
}

pub(crate) trait RetainedAbi<Owned> {
    fn read(value: &StoredRuntimeValue) -> Owned;
    fn store(value: Owned, input: &StoredRuntimeValue) -> StoredRuntimeValue;
}

/// A typed handle; it preserves the original value, metadata and lease.
#[doc(hidden)]
pub struct HostRetainedValue<Type: HostType> {
    pub(crate) value: StoredRuntimeValue,
    pub(crate) type_: PhantomData<fn() -> Type>,
}
