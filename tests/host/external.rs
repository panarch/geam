#[path = "external/function.rs"]
mod function;
#[path = "external/linkage.rs"]
mod linkage;
#[path = "external/ownership.rs"]
mod ownership;
#[path = "external/specialization.rs"]
mod specialization;
#[path = "external/value.rs"]
mod value;

use ecow::EcoString;
use geam::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomSchema, HostCustomType, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostFunctionType, HostProfile, HostProvider, HostTypeList, HostTypeListEnd,
    HostTypeParameter,
};
use num_bigint::BigInt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(super) struct ExternalProfile;

#[derive(Default)]
pub(super) struct ExternalRunState {
    provider: (),
}

#[derive(Default)]
pub(super) struct ExternalStores {
    counters: HostExternalStore<Counter>,
    dependency_counters: HostExternalStore<Counter>,
    generic_counters: HostExternalStore<Counter>,
}

#[derive(Debug)]
pub(super) struct Counter {
    value: BigInt,
}

pub(super) struct CounterSchema;

pub(super) struct CounterProvider;

pub(super) type HostCounter = HostExternalType<CounterSchema>;

pub(super) struct WrappedCounterField;

impl HostCustomField for WrappedCounterField {
    const LABEL: Option<&'static str> = Some("value");

    type Type = HostCounter;
}

pub(super) struct WrappedCounterDefinition;

impl HostCustomConstructorDefinition for WrappedCounterDefinition {
    const NAME: &'static str = "Wrapped";

    type Fields = HostCustomFieldList<WrappedCounterField, HostCustomFieldListEnd>;
}

pub(super) struct WrappedCounterSchema;

impl HostCustomSchema for WrappedCounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Wrapped";
    const PARAMETER_COUNT: usize = 0;

    type Constructors =
        HostCustomConstructorList<WrappedCounterDefinition, HostCustomConstructorListEnd>;
}

pub(super) type HostWrappedCounter = HostCustomType<WrappedCounterSchema>;
pub(super) type HostWrappedCounterConstructor =
    HostCustomConstructorAt<HostWrappedCounter, HostCustomIndex0, WrappedCounterDefinition>;

pub(super) struct DependencyCounterSchema;

pub(super) type HostDependencyCounter = HostExternalType<DependencyCounterSchema>;

pub(super) struct GenericCounterSchema;

pub(super) type GenericCounterArguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
pub(super) type HostGenericCounter =
    HostExternalType<GenericCounterSchema, GenericCounterArguments>;
pub(super) type GenericValue = HostTypeParameter<0>;
pub(super) type NoArguments = HostTypeListEnd;
pub(super) type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
pub(super) type GenericCallback = HostFunctionType<NoArguments, GenericValue>;
pub(super) type GenericIntCallback = HostFunctionType<IntArguments, GenericValue>;

impl HostProfile for ExternalProfile {
    type RunState = ExternalRunState;
    type ExternalStores = ExternalStores;
}

impl HostExternalSchema for CounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Counter";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<CounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("Counter({})", value.value).into()
    }
}

impl HostExternalSchema for DependencyCounterSchema {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/counter";
    const NAME: &'static str = "Counter";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalStorage<DependencyCounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.dependency_counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("SupportCounter({})", value.value).into()
    }
}

impl HostExternalSchema for GenericCounterSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "GenericCounter";
    const PARAMETER_COUNT: usize = 1;
}

impl HostExternalStorage<GenericCounterSchema> for ExternalProfile {
    type Payload = Counter;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stores.generic_counters
    }

    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.value.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("GenericCounter({})", value.value).into()
    }
}

impl HostProvider<ExternalProfile> for CounterProvider {
    type State = ();

    fn project(state: &mut ExternalRunState) -> &mut Self::State {
        &mut state.provider
    }
}
