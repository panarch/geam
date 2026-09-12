use crate::{Component, GleamErlangHostProfile, reference, selector};
use ecow::EcoString;
use geam_core::host::{
    HostComponentProfile, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use std::marker::PhantomData;

pub struct AtomSchema;
pub struct ReferenceSchema;
pub struct PidSchema;
pub(crate) struct NameSchema;
pub(crate) struct MonitorSchema;
pub(crate) struct TimerSchema;
pub(crate) struct DoNotLeakSchema;
pub(crate) struct SelectorSchema;
pub(crate) struct NodeSchema;
pub(crate) struct CharlistSchema;
pub(crate) struct PortSchema;

pub type Atom = HostExternalType<AtomSchema>;
pub type Reference = HostExternalType<ReferenceSchema>;
pub type Pid = HostExternalType<PidSchema>;
pub(crate) type Name<A> = HostExternalType<NameSchema, HostTypeList<A, HostTypeListEnd>>;
pub(crate) type Monitor = HostExternalType<MonitorSchema>;
pub(crate) type Timer = HostExternalType<TimerSchema>;
pub(crate) type DoNotLeak = HostExternalType<DoNotLeakSchema>;
pub(crate) type Selector<A> = HostExternalType<SelectorSchema, HostTypeList<A, HostTypeListEnd>>;
pub(crate) type Node = HostExternalType<NodeSchema>;
pub(crate) type Charlist = HostExternalType<CharlistSchema>;
pub(crate) type Port = HostExternalType<PortSchema>;

impl HostExternalSchema for AtomSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/atom";
    const NAME: &'static str = "Atom";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for ReferenceSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/reference";
    const NAME: &'static str = "Reference";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for PidSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Pid";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for NameSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Name";
    const PARAMETER_COUNT: usize = 1;
}
impl HostExternalSchema for MonitorSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Monitor";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for TimerSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Timer";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for DoNotLeakSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "DoNotLeak";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for SelectorSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Selector";
    const PARAMETER_COUNT: usize = 1;
}
impl HostExternalSchema for NodeSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/node";
    const NAME: &'static str = "Node";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for CharlistSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/charlist";
    const NAME: &'static str = "Charlist";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalSchema for PortSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/port";
    const NAME: &'static str = "Port";
    const PARAMETER_COUNT: usize = 0;
}

pub(crate) trait NativeSchema: HostExternalSchema {}
impl NativeSchema for AtomSchema {}
impl NativeSchema for MonitorSchema {}
impl NativeSchema for TimerSchema {}
impl NativeSchema for DoNotLeakSchema {}
impl NativeSchema for NodeSchema {}

pub struct NativeStorage<Schema>(PhantomData<Schema>);

impl<Profile: GleamErlangHostProfile, Schema: NativeSchema> HostExternalBinding<Profile, Schema>
    for Component<Profile>
{
    type Storage = NativeStorage<Schema>;
}

impl<Profile: GleamErlangHostProfile, Schema: NativeSchema> HostExternalStorage<Profile, Schema>
    for NativeStorage<Schema>
{
    type Payload = NativeValue;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<NativeValue> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).native
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &NativeValue,
        right: &NativeValue,
    ) -> bool {
        left.source_equal(context, right)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &NativeValue) -> u64 {
        value.source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &NativeValue) -> EcoString {
        value.inspect(context)
    }
    fn native_view(value: &NativeValue) -> Option<NativeValue> {
        Some(value.clone())
    }
}

impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, ReferenceSchema>
    for Component<Profile>
{
    type Storage = reference::Storage;
}

impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, SelectorSchema>
    for Component<Profile>
{
    type Storage = selector::Storage;
}

impl<Profile: GleamErlangHostProfile>
    HostExternalBinding<Profile, geam_stdlib::provider_support::DynamicSchema>
    for Component<Profile>
{
    type Storage = geam_stdlib::provider_support::DynamicExternalStorage;
}

#[cfg(test)]
mod tests {
    use super::{
        AtomSchema, DoNotLeakSchema, MonitorSchema, NativeSchema, NativeStorage, NodeSchema,
        TimerSchema,
    };
    use crate::{GleamErlangProfile, GleamErlangStores};
    use geam_core::host::HostExternalStorage;
    use geam_core::provider::advanced::NativeValue;

    fn preserves_native_protocol<Schema: NativeSchema>() {
        let stores = GleamErlangStores::default();
        let _ = <NativeStorage<Schema> as HostExternalStorage<GleamErlangProfile, Schema>>::store(
            &stores,
        );
        crate::test_support::with_contexts(
            |context| {
                let left = NativeValue::symbol("native_payload");
                let right = NativeValue::symbol("other_payload");
                assert!(<NativeStorage<Schema> as HostExternalStorage<
                    GleamErlangProfile,
                    Schema,
                >>::source_equal(context, &left, &left));
                assert!(!<NativeStorage<Schema> as HostExternalStorage<
                    GleamErlangProfile,
                    Schema,
                >>::source_equal(context, &left, &right));
            },
            |context| {
                let value = NativeValue::symbol("native_payload");
                assert_eq!(<NativeStorage<Schema> as HostExternalStorage<GleamErlangProfile, Schema>>::source_hash(context, &value), value.source_hash(context));
            },
            |context| {
                let value = NativeValue::symbol("native_payload");
                assert_eq!(<NativeStorage<Schema> as HostExternalStorage<GleamErlangProfile, Schema>>::inspect(context, &value), "NativePayload");
            },
        );
    }

    #[test]
    fn every_native_backed_schema_delegates_the_same_value_protocol() {
        preserves_native_protocol::<AtomSchema>();
        preserves_native_protocol::<MonitorSchema>();
        preserves_native_protocol::<TimerSchema>();
        preserves_native_protocol::<DoNotLeakSchema>();
        preserves_native_protocol::<NodeSchema>();
    }
}
