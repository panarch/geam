pub use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
    HostExternal, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostList, HostListType, HostProvider, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTuple, HostTupleType, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd,
};
pub use crate::provider::ExternalPayload;
pub use crate::provider::{
    List, ProviderExternalItem, ProviderExternalPayloadAccess, ProviderListContext,
    ProviderListItemDecoder, ProviderListItemValue,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Package identity shared by provider and module macro expansions.
pub trait ProviderPackage: HostProviderComponent {
    const PACKAGE: &'static str;
}

pub fn component_initialization_error<Component>(
    error: crate::provider::InitializationError,
) -> crate::HostProviderInitializationError
where
    Component: HostProviderComponent,
{
    crate::HostProviderInitializationError::for_component::<Component>(error.reason())
}

/// Hashes an ordinary Rust payload for generated external source semantics.
#[doc(hidden)]
pub fn external_payload_hash<Payload>(payload: &Payload) -> u64
where
    Payload: Hash + ?Sized,
{
    let mut hasher = DefaultHasher::new();
    payload.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::{
        HostProviderComponent, ProviderPackage, component_initialization_error,
        external_payload_hash,
    };
    use crate::provider::InitializationError;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct Component;

    impl HostProviderComponent for Component {
        const ID: &'static str = "macro-support";
        type Stores = ();
        type RunState = ();
    }

    impl ProviderPackage for Component {
        const PACKAGE: &'static str = "macro_support";
    }

    #[test]
    fn initialization_support_adds_the_static_component_identity() {
        let error = component_initialization_error::<Component>(InitializationError::new(
            "configuration is incomplete",
        ));

        assert_eq!(error.component_id(), "macro-support");
        assert_eq!(error.reason(), "configuration is incomplete");
        assert_eq!(Component::PACKAGE, "macro_support");
    }

    #[test]
    fn external_payload_hash_uses_ordinary_rust_hashing() {
        let value = ("tag", 7u64);
        let equal = ("tag", 7u64);
        let mut expected = DefaultHasher::new();
        value.hash(&mut expected);

        assert_eq!(external_payload_hash(&value), expected.finish());
        assert_eq!(external_payload_hash(&value), external_payload_hash(&equal));
    }
}
