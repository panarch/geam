use crate::schema::{NameSchema, PidSchema, PortSchema};
use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostComponentProfile, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalStorage, HostExternalStore,
};
use geam_core::provider::advanced::NativeValue;
use std::convert::Infallible;
use std::hash::{Hash, Hasher};

pub struct PidStorage;
pub struct NameStorage;
pub struct PortStorage;

impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, PidSchema>
    for Component<Profile>
{
    type Storage = PidStorage;
}
impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, PidSchema> for PidStorage {
    type Payload = ExecutionUnit;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<ExecutionUnit> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).pids
    }
    fn source_equal(
        _: &HostExternalEquality<'_>,
        left: &ExecutionUnit,
        right: &ExecutionUnit,
    ) -> bool {
        left.id() == right.id()
    }
    fn source_hash(_: &HostExternalHashing<'_>, value: &ExecutionUnit) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        value.id().hash(&mut hash);
        hash.finish()
    }
    fn inspect(_: &HostExternalInspection<'_>, value: &ExecutionUnit) -> EcoString {
        format!("Pid({:?})", value.id()).into()
    }
}
impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, NameSchema>
    for Component<Profile>
{
    type Storage = NameStorage;
}
impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, NameSchema> for NameStorage {
    type Payload = EcoString;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<EcoString> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).names
    }
    fn source_equal(_: &HostExternalEquality<'_>, left: &EcoString, right: &EcoString) -> bool {
        left == right
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &EcoString) -> u64 {
        NativeValue::symbol(value.clone()).source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &EcoString) -> EcoString {
        NativeValue::symbol(value.clone()).inspect(context)
    }
    fn native_view(value: &EcoString) -> Option<NativeValue> {
        Some(NativeValue::symbol(value.clone()))
    }
}

impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, PortSchema>
    for Component<Profile>
{
    type Storage = PortStorage;
}
impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, PortSchema> for PortStorage {
    type Payload = Infallible;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Infallible> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).ports
    }
    fn source_equal(_: &HostExternalEquality<'_>, left: &Infallible, _: &Infallible) -> bool {
        match *left {}
    }
    fn source_hash(_: &HostExternalHashing<'_>, value: &Infallible) -> u64 {
        match *value {}
    }
    fn inspect(_: &HostExternalInspection<'_>, value: &Infallible) -> EcoString {
        match *value {}
    }
}

#[cfg(test)]
mod tests {
    use super::{NameStorage, PidStorage, PortStorage};
    use crate::schema::{NameSchema, PidSchema, PortSchema};
    use crate::{GleamErlangProfile, GleamErlangStores};
    use geam_core::host::HostExternalStorage;
    use geam_core::provider::advanced::NativeValue;

    #[test]
    fn names_have_symbol_semantics_without_a_second_value_kind() {
        let stores = GleamErlangStores::default();
        let _ =
            <NameStorage as HostExternalStorage<GleamErlangProfile, NameSchema>>::store(&stores);
        let view =
            <NameStorage as HostExternalStorage<GleamErlangProfile, NameSchema>>::native_view(
                &"service".into(),
            )
            .unwrap();
        assert_eq!(view.as_symbol().as_deref(), Some("service"));
        crate::test_support::with_contexts(
            |context| {
                let equal = <NameStorage as HostExternalStorage<GleamErlangProfile, NameSchema>>::source_equal;
                assert!(equal(context, &"service".into(), &"service".into()));
                assert!(!equal(context, &"service".into(), &"replacement".into()));
            },
            |context| {
                let hash = <NameStorage as HostExternalStorage<GleamErlangProfile, NameSchema>>::source_hash;
                assert_eq!(
                    hash(context, &"service".into()),
                    NativeValue::symbol("service").source_hash(context)
                );
            },
            |context| {
                let inspect =
                    <NameStorage as HostExternalStorage<GleamErlangProfile, NameSchema>>::inspect;
                assert_eq!(inspect(context, &"service".into()), "Service");
            },
        );
    }

    #[test]
    fn pid_identity_hash_and_inspection_survive_logical_termination() {
        let stores = GleamErlangStores::default();
        let _ = <PidStorage as HostExternalStorage<GleamErlangProfile, PidSchema>>::store(&stores);
        let _ =
            <PortStorage as HostExternalStorage<GleamErlangProfile, PortSchema>>::store(&stores);
        crate::test_support::with_units(2, |units| {
            let equal_units = units.to_vec();
            let hash_unit = units[0].clone();
            let inspect_unit = units[0].clone();
            crate::test_support::with_contexts(
                move |context| {
                    let equal = <PidStorage as HostExternalStorage<
                        GleamErlangProfile,
                        PidSchema,
                    >>::source_equal;
                    assert!(equal(context, &equal_units[0], &equal_units[0].clone()));
                    assert!(!equal(context, &equal_units[0], &equal_units[1]));
                },
                move |context| {
                    let hash = <PidStorage as HostExternalStorage<GleamErlangProfile, PidSchema>>::source_hash;
                    let before = hash(context, &hash_unit);
                    hash_unit.cancel();
                    assert_eq!(hash(context, &hash_unit), before);
                },
                move |context| {
                    let inspect =
                        <PidStorage as HostExternalStorage<GleamErlangProfile, PidSchema>>::inspect;
                    assert_eq!(
                        inspect(context, &inspect_unit),
                        format!("Pid({:?})", inspect_unit.id())
                    );
                },
            );
        });
    }
}
