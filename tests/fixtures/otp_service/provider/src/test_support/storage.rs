//! Exercise the storage callbacks even when an ordinary source operation uses
//! the payload's native view first. The payload and all semantics stay with the
//! production storage owner.
use super::{Profile, Stores};
use crate::{Call, Component};
use geam::host::{
    HostCallCompletion, HostCallError, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
    HostExternalStore, HostExternalType,
};
use std::marker::PhantomData;

pub(crate) struct Probe<Schema>(PhantomData<fn() -> Schema>);
pub(crate) type ProbeValue<Schema> = HostExternalType<Probe<Schema>>;
type Storage<Schema> = <Component<Profile> as HostExternalBinding<Profile, Schema>>::Storage;
type Payload<Schema> = <Storage<Schema> as HostExternalStorage<Profile, Schema>>::Payload;

impl<Schema: HostExternalSchema> HostExternalSchema for Probe<Schema> {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Probe";
    const PARAMETER_COUNT: usize = 0;
}

impl<Schema: HostExternalSchema> HostExternalBinding<Profile, Probe<Schema>> for Component<Profile>
where
    Component<Profile>: HostExternalBinding<Profile, Schema>,
{
    type Storage = Probe<Schema>;
}

impl<Schema: HostExternalSchema> HostExternalStorage<Profile, Probe<Schema>> for Probe<Schema>
where
    Component<Profile>: HostExternalBinding<Profile, Schema>,
{
    type Payload = Payload<Schema>;
    fn store(stores: &Stores) -> &HostExternalStore<Self::Payload> {
        Storage::<Schema>::store(stores)
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        Storage::<Schema>::source_equal(context, left, right)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        Storage::<Schema>::source_hash(context, value)
    }
    fn inspect(
        context: &HostExternalInspection<'_>,
        value: &Self::Payload,
    ) -> geam::provider::EcoString {
        Storage::<Schema>::inspect(context, value)
    }
}

pub(crate) fn check<'call, Schema: HostExternalSchema>(
    call: Call<'call, Profile, ()>,
    first: HostExternal<'call, ProbeValue<Schema>>,
    same: HostExternal<'call, ProbeValue<Schema>>,
    different: HostExternal<'call, ProbeValue<Schema>>,
    expected: geam::provider::StringValue,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Component<Profile>: HostExternalBinding<Profile, Schema>,
{
    assert!(call.equal::<ProbeValue<Schema>>(first, same));
    assert!(!call.equal::<ProbeValue<Schema>>(first, different));
    assert_eq!(
        call.source_hash::<ProbeValue<Schema>>(first),
        call.source_hash::<ProbeValue<Schema>>(same)
    );
    assert_eq!(
        call.inspect::<ProbeValue<Schema>>(first).as_str(),
        expected.as_str()
    );
    let first = Storage::<Schema>::native_view(
        &call.external_payload::<Probe<Schema>, geam::host::HostTypeListEnd>(first),
    )
    .unwrap();
    let same = Storage::<Schema>::native_view(
        &call.external_payload::<Probe<Schema>, geam::host::HostTypeListEnd>(same),
    )
    .unwrap();
    let different = Storage::<Schema>::native_view(
        &call.external_payload::<Probe<Schema>, geam::host::HostTypeListEnd>(different),
    )
    .unwrap();
    assert!(call.native_equal(&first, &same));
    assert!(!call.native_equal(&first, &different));
    assert_eq!(call.native_hash(&first), call.native_hash(&same));
    Ok(call.return_value(()))
}

pub(crate) fn native_protocol<Schema: crate::schema::NativeSchema>(source: &str) {
    use geam::host::{HostProviderModule, HostTypeParameter};
    use geam::{ModuleSource, PackageSource};
    let provider = HostProviderModule::new("application", "main")
        .unwrap()
        .with_external_type::<Component<Profile>, Probe<Schema>>()
        .unwrap()
        .with_scoped_function::<Component<Profile>, (HostTypeParameter<0>,), ProbeValue<Schema>, _>(
            "make",
            native_probe::<Schema>,
        )
        .unwrap()
        .with_scoped_function::<Component<Profile>, (
            ProbeValue<Schema>,
            ProbeValue<Schema>,
            ProbeValue<Schema>,
            geam::provider::StringValue,
        ), (), _>("check", check::<Schema>)
        .unwrap();
    assert_eq!(
        super::run_source(
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("main", "main.gleam", source)]
            )],
            [provider]
        )
        .unwrap(),
        geam::Value::Nil
    );
}

fn native_probe<'call, Schema: crate::schema::NativeSchema>(
    mut call: Call<'call, Profile, ProbeValue<Schema>>,
    value: geam::host::HostValue<'call, crate::A>,
) -> Result<HostCallCompletion<'call, ProbeValue<Schema>>, HostCallError> {
    let value = call.native_value::<crate::A>(value);
    let value = call.create_external(value);
    Ok(call.return_value(value))
}

pub(crate) fn callback_packages(source: &str) -> Vec<geam::PackageSource> {
    use geam::{ModuleSource, PackageSource};
    vec![
        PackageSource::new(
            "gleam_stdlib",
            Vec::<String>::new(),
            [ModuleSource::new(
                "gleam/dynamic",
                "dynamic.gleam",
                "pub type Dynamic",
            )],
        ),
        PackageSource::new(
            "gleam_erlang",
            ["gleam_stdlib"],
            [ModuleSource::new(
                "gleam/erlang/process",
                "process.gleam",
                r#"
import gleam/dynamic.{type Dynamic}
pub type Pid
pub type ExitReason { Normal Killed Abnormal(reason: Dynamic) }
"#,
            )],
        ),
        PackageSource::new(
            "gleam_otp",
            ["gleam_erlang"],
            [ModuleSource::new(
                "gleam/otp/actor",
                "actor.gleam",
                r#"
import gleam/erlang/process.{type Pid, type ExitReason}
pub type Started(a) { Started(pid: Pid, data: a) }
pub type StartError { InitTimeout InitFailed(String) InitExited(ExitReason) }
pub type StartResult(a) = Result(Started(a), StartError)
"#,
            )],
        ),
        PackageSource::new(
            "application",
            ["gleam_otp"],
            [ModuleSource::new("main", "main.gleam", source)],
        ),
    ]
}

pub(crate) fn callback_providers(
    provider: geam::HostProviderModule<Profile>,
) -> Vec<geam::HostProviderModule<Profile>> {
    use geam::gleam_erlang::{Component as Erlang, PidSchema};
    use geam::gleam_stdlib::provider_support::DynamicSchema;
    use geam::host::HostProviderModule;
    vec![
        provider,
        HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_external_type::<Erlang<Profile>, PidSchema>()
            .unwrap(),
        HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
            .unwrap()
            .with_external_type::<Erlang<Profile>, DynamicSchema>()
            .unwrap(),
    ]
}
