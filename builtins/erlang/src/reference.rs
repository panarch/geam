use crate::execution::ReferenceId;
use crate::schema::{Reference, ReferenceSchema};
use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalStorage, HostExternalStore,
    HostProviderModule, HostRegistrationError,
};
use geam_core::provider::advanced::NativeValue;
use std::hash::{Hash, Hasher};

pub enum Payload {
    Identity(ReferenceId),
    View(NativeValue),
}

pub struct Storage;

impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, ReferenceSchema> for Storage {
    type Payload = Payload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Payload> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).references
    }
    fn source_equal(context: &HostExternalEquality<'_>, left: &Payload, right: &Payload) -> bool {
        match (left, right) {
            (Payload::Identity(left), Payload::Identity(right)) => left == right,
            (Payload::View(left), Payload::View(right)) => left.source_equal(context, right),
            _ => false,
        }
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &Payload) -> u64 {
        match value {
            Payload::Identity(id) => {
                let mut hash = std::collections::hash_map::DefaultHasher::new();
                id.hash(&mut hash);
                hash.finish()
            }
            Payload::View(value) => value.source_hash(context),
        }
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &Payload) -> EcoString {
        match value {
            Payload::Identity(id) => format!("#Ref<{}>", id.number()).into(),
            Payload::View(value) => value.inspect(context),
        }
    }
    fn native_view(value: &Payload) -> Option<NativeValue> {
        match value {
            Payload::Identity(_) => None,
            Payload::View(value) => Some(value.clone()),
        }
    }
}

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
        .and_then(|module| module.with_external_type::<Component<Profile>, ReferenceSchema>())
        .and_then(|module| {
            module
                .with_scoped_function::<Component<Profile>, (), Reference, _>("new", new::<Profile>)
        })
}

fn new<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Reference>,
) -> Result<HostCallCompletion<'call, Reference>, HostCallError> {
    let value = call.create_external(Payload::Identity(ReferenceId::new()));
    Ok(call.return_value(value))
}

#[cfg(test)]
mod tests {
    use super::{Payload, Storage};
    use crate::execution::ReferenceId;
    use crate::schema::ReferenceSchema;
    use crate::{GleamErlangProfile, GleamErlangStores};
    use geam_core::host::HostExternalStorage;
    use geam_core::provider::advanced::NativeValue;

    #[test]
    fn identity_and_foreign_view_remain_distinct_in_every_value_operation() {
        let first = ReferenceId::new();
        let second = ReferenceId::new();
        let stores = GleamErlangStores::default();
        let _ =
            <Storage as HostExternalStorage<GleamErlangProfile, ReferenceSchema>>::store(&stores);
        let view =
            <Storage as HostExternalStorage<GleamErlangProfile, ReferenceSchema>>::native_view;
        assert!(view(&Payload::Identity(first)).is_none());
        assert_eq!(
            view(&Payload::View(NativeValue::symbol("tag")))
                .unwrap()
                .as_symbol()
                .as_deref(),
            Some("tag")
        );
        crate::test_support::with_contexts(
            move |context| {
                let equal = <Storage as HostExternalStorage<GleamErlangProfile, ReferenceSchema>>::source_equal;
                assert!(equal(
                    context,
                    &Payload::Identity(first),
                    &Payload::Identity(first)
                ));
                assert!(!equal(
                    context,
                    &Payload::Identity(first),
                    &Payload::Identity(second)
                ));
                assert!(!equal(
                    context,
                    &Payload::Identity(first),
                    &Payload::View(NativeValue::symbol("tag"))
                ));
                assert!(!equal(
                    context,
                    &Payload::View(NativeValue::symbol("tag")),
                    &Payload::Identity(first)
                ));
                assert!(equal(
                    context,
                    &Payload::View(NativeValue::symbol("tag")),
                    &Payload::View(NativeValue::symbol("tag"))
                ));
                assert!(!equal(
                    context,
                    &Payload::View(NativeValue::symbol("tag")),
                    &Payload::View(NativeValue::symbol("other"))
                ));
            },
            move |context| {
                let hash = <Storage as HostExternalStorage<GleamErlangProfile, ReferenceSchema>>::source_hash;
                assert_eq!(
                    hash(context, &Payload::Identity(first)),
                    hash(context, &Payload::Identity(first))
                );
                assert_ne!(
                    hash(context, &Payload::Identity(first)),
                    hash(context, &Payload::Identity(second))
                );
                let tag = NativeValue::symbol("tag");
                assert_eq!(
                    hash(context, &Payload::View(tag.clone())),
                    tag.source_hash(context)
                );
            },
            move |context| {
                let inspect =
                    <Storage as HostExternalStorage<GleamErlangProfile, ReferenceSchema>>::inspect;
                assert_eq!(
                    inspect(context, &Payload::Identity(first)),
                    format!("#Ref<{}>", first.number())
                );
                assert_eq!(
                    inspect(context, &Payload::View(NativeValue::symbol("tag"))),
                    "Tag"
                );
            },
        );
    }
}
