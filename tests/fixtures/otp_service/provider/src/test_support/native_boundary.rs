//! Passive primitive declarations and producer bindings for owner native tests.
//! Each owner supplies its complete local OTP declarations and source behavior.
use super::Profile;
use geam::gleam_erlang::{Component, PidSchema, ReferenceSchema};
use geam::gleam_stdlib::provider_support::DynamicSchema;
use geam::{HostProviderModule, ModuleSource, PackageSource};

pub(crate) fn packages(source: &str, otp_modules: &[(&str, &str)]) -> Vec<PackageSource> {
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
            [
                ModuleSource::new(
                    "gleam/erlang/process",
                    "process.gleam",
                    "import gleam/dynamic.{type Dynamic}\npub type Pid\npub type Name(message)\npub type Monitor\npub type ExitReason { Normal Killed Abnormal(reason: Dynamic) }",
                ),
                ModuleSource::new(
                    "gleam/erlang/reference",
                    "reference.gleam",
                    "pub type Reference",
                ),
                ModuleSource::new(
                    "gleam/erlang/atom",
                    "atom.gleam",
                    r#"
import gleam/dynamic.{type Dynamic}
pub type Atom
@external(erlang, "fixture", "create") pub fn create(name: String) -> Atom
@external(erlang, "fixture", "get") pub fn get(name: String) -> Result(Atom, Nil)
@external(erlang, "fixture", "to_string") pub fn to_string(atom: Atom) -> String
@external(erlang, "fixture", "to_dynamic") pub fn to_dynamic(atom: Atom) -> Dynamic
@external(erlang, "fixture", "cast_from_dynamic") pub fn cast_from_dynamic(value: Dynamic) -> Atom
@external(erlang, "fixture", "is_atom") pub fn is_atom(value: Dynamic) -> Bool
"#,
                ),
            ],
        ),
        PackageSource::new(
            "gleam_otp",
            ["gleam_stdlib", "gleam_erlang"],
            otp_modules
                .iter()
                .map(|(name, source)| ModuleSource::new(*name, format!("{name}.gleam"), *source)),
        ),
        PackageSource::new(
            "application",
            ["gleam_stdlib", "gleam_erlang", "gleam_otp"],
            [ModuleSource::new("main", "main.gleam", source)],
        ),
    ]
}

pub(crate) fn providers() -> Vec<HostProviderModule<Profile>> {
    let mut providers = vec![
        HostProviderModule::new("gleam_stdlib", "gleam/dynamic").unwrap()
            .with_external_type::<Component<Profile>, DynamicSchema>().unwrap(),
        HostProviderModule::new("gleam_erlang", "gleam/erlang/process").unwrap()
            .with_external_type::<Component<Profile>, PidSchema>().unwrap()
            .with_external_type::<Component<Profile>, geam::gleam_erlang::NameSchema>().unwrap()
            .with_external_type::<Component<Profile>, geam::gleam_erlang::service::types::MonitorSchema>().unwrap(),
        HostProviderModule::new("gleam_erlang", "gleam/erlang/reference").unwrap()
            .with_external_type::<Component<Profile>, ReferenceSchema>().unwrap(),
    ];
    providers.extend(
        geam::gleam_erlang::host_providers::<Profile>()
            .unwrap()
            .into_iter()
            .filter(|module| module.module() == "gleam/erlang/atom"),
    );
    providers
}
