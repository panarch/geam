mod store;

use crate::host::{HostProfile, HostTypeListEnd};
use ecow::EcoString;
use std::marker::PhantomData;

pub(crate) use store::ExternalPayloadLease;
pub use store::HostExternalStore;

/// A source-declared external Gleam type linked to Rust storage.
pub trait HostExternalSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;
    const PARAMETER_COUNT: usize;
}

/// The profile-owned storage and value semantics for one external schema.
pub trait HostExternalStorage<Schema>: HostProfile
where
    Schema: HostExternalSchema,
{
    type Payload: 'static;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload>;
    fn equal(left: &Self::Payload, right: &Self::Payload) -> bool;
    fn inspect(value: &Self::Payload) -> EcoString;
}

/// A source-declared external type and its concrete type arguments.
pub struct HostExternalType<Schema, Arguments = HostTypeListEnd>(PhantomData<(Schema, Arguments)>);

/// The source-facing identity of one registered external type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostExternalTypeSchema {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameter_count: usize,
}

impl HostExternalTypeSchema {
    pub fn of<Schema: HostExternalSchema>() -> Self {
        Self::new(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            Schema::PARAMETER_COUNT,
        )
    }

    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
        name: impl Into<EcoString>,
        parameter_count: usize,
    ) -> Self {
        Self {
            package: package.into(),
            module: module.into(),
            name: name.into(),
            parameter_count,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn parameter_count(&self) -> usize {
        self.parameter_count
    }
}

#[cfg(test)]
pub(crate) struct ExternalTestProfile;

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestRunState {
    pub(crate) provider: (),
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestStores {
    pub(crate) units: HostExternalStore<()>,
    pub(crate) integers: HostExternalStore<num_bigint::BigInt>,
    pub(crate) indices: HostExternalStore<usize>,
}

#[cfg(test)]
impl HostProfile for ExternalTestProfile {
    type RunState = ExternalTestRunState;
    type ExternalStores = ExternalTestStores;
}
