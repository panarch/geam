use super::{ProviderConstructionRequirements, ProviderNoConstructions, ProviderOwnedCaptures};
use crate::HostTypeListEnd;

/// The exact permission tree owned by one generated provider function.
#[doc(hidden)]
pub trait ProviderCallBindings {
    type Requirements: ProviderConstructionRequirements;
    type CaptureMode;
}

#[doc(hidden)]
pub struct ProviderNoCallBindings;

impl ProviderCallBindings for ProviderNoCallBindings {
    type Requirements = ProviderNoConstructions;
    type CaptureMode = ProviderOwnedCaptures;
}

pub(super) type CallConstructions<Bindings> =
    <<Bindings as ProviderCallBindings>::Requirements as ProviderConstructionRequirements>::Types<
        HostTypeListEnd,
    >;
