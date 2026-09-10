use crate::schema::{Charlist, CharlistSchema};
use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
    HostExternal, HostExternalBinding, HostExternalEquality, HostExternalHashing,
    HostExternalInspection, HostExternalStorage, HostExternalStore, HostListType,
    HostProviderModule, HostRegistrationError, HostStoredValue, HostTypeIndex0, HostTypeList,
    HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;

pub struct Storage;

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/charlist")
        .and_then(|module| module.with_external_type::<Component<Profile>, CharlistSchema>())
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (EcoString,), Charlist, HostTypeList<HostListType<char>, HostTypeListEnd>, _>("from_string", from_string::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Charlist,), EcoString, _>("to_string", to_string::<Profile>))
}

fn from_string<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Charlist>,
    constructions: HostConstructions<'call, HostTypeList<HostListType<char>, HostTypeListEnd>>,
    string: EcoString,
) -> Result<HostCallCompletion<'call, Charlist>, HostCallError> {
    let characters = call.construct_list(constructions.at::<HostTypeIndex0>(), string.chars());
    let value =
        call.create_external_with(|builder| builder.store::<HostListType<char>>(characters));
    Ok(call.return_value(value))
}

fn to_string<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, EcoString>,
    characters: HostExternal<'call, Charlist>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError> {
    let characters = call
        .external_payload(characters)
        .restore(&mut call, |characters| characters);
    let mut output = EcoString::new();
    let mut index = 0;
    while let Some(character) = call.list_item::<char>(characters, index) {
        output.push(character);
        index += 1;
    }
    Ok(call.return_value(output))
}

impl<Profile: GleamErlangHostProfile> HostExternalBinding<Profile, CharlistSchema>
    for Component<Profile>
{
    type Storage = Storage;
}

impl<Profile: GleamErlangHostProfile> HostExternalStorage<Profile, CharlistSchema> for Storage {
    type Payload = HostStoredValue<HostListType<char>>;
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &<Profile as HostComponentProfile<Component<Profile>>>::component_stores(stores).charlists
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.native_view()
            .source_equal(context, &right.native_view())
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.native_view().source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        value.native_view().inspect(context)
    }
    fn native_view(value: &Self::Payload) -> Option<NativeValue> {
        Some(value.native_view())
    }
}

#[cfg(test)]
mod tests {
    use super::{Storage, host_provider};
    use crate::schema::{Charlist, CharlistSchema};
    use crate::{Component, GleamErlangProfile, GleamErlangRunState};
    use ecow::EcoString;
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostExternal, HostExternalBinding,
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
        HostExternalStorage, HostExternalStore, HostExternalType, HostList, HostListType,
        HostProvider, HostProviderModule, HostProviderSet, HostStoredValue,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    // This opaque probe exposes the storage protocol even when the ordinary
    // Charlist path follows its native view before reaching these callbacks.
    struct Probe;
    type ProbeValue = HostExternalType<Probe>;
    impl HostProvider<GleamErlangProfile> for Probe {
        type State = crate::Configuration;
        fn project(state: &mut GleamErlangRunState) -> &mut Self::State {
            &mut state.erlang
        }
    }
    impl HostExternalSchema for Probe {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Probe";
        const PARAMETER_COUNT: usize = 0;
    }
    impl HostExternalBinding<GleamErlangProfile, Probe> for Probe {
        type Storage = Probe;
    }
    impl HostExternalStorage<GleamErlangProfile, Probe> for Probe {
        type Payload = HostStoredValue<HostListType<char>>;
        fn store(stores: &crate::GleamErlangStores) -> &HostExternalStore<Self::Payload> {
            <Storage as HostExternalStorage<GleamErlangProfile, CharlistSchema>>::store(stores)
        }
        fn source_equal(
            context: &HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            <Storage as HostExternalStorage<GleamErlangProfile, CharlistSchema>>::source_equal(
                context, left, right,
            )
        }
        fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            <Storage as HostExternalStorage<GleamErlangProfile, CharlistSchema>>::source_hash(
                context, value,
            )
        }
        fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
            <Storage as HostExternalStorage<GleamErlangProfile, CharlistSchema>>::inspect(
                context, value,
            )
        }
    }

    fn retain<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Probe, ProbeValue>,
        characters: HostList<'call, char>,
    ) -> Result<HostCallCompletion<'call, ProbeValue>, HostCallError> {
        let value =
            call.create_external_with(|builder| builder.store::<HostListType<char>>(characters));
        Ok(call.return_value(value))
    }

    fn check_protocol<'call>(
        call: HostCall<'call, GleamErlangProfile, Probe, ()>,
        first: HostExternal<'call, ProbeValue>,
        same: HostExternal<'call, ProbeValue>,
        empty: HostExternal<'call, ProbeValue>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert!(call.equal::<ProbeValue>(first, same));
        assert!(!call.equal::<ProbeValue>(first, empty));
        assert_eq!(
            call.source_hash::<ProbeValue>(first),
            call.source_hash::<ProbeValue>(same)
        );
        assert_eq!(
            call.inspect::<ProbeValue>(first),
            "charlist.from_string(\"A\")"
        );
        assert_eq!(call.inspect::<ProbeValue>(empty), "[]");
        Ok(call.return_value(()))
    }

    #[test]
    fn charlist_storage_protocol_preserves_its_typed_payload_semantics() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Probe, Probe>()
            .unwrap()
            .with_scoped_function::<Probe, (HostListType<char>,), ProbeValue, _>("retain", retain)
            .unwrap()
            .with_scoped_function::<Probe, (ProbeValue, ProbeValue, ProbeValue), (), _>(
                "check",
                check_protocol,
            )
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
pub type Probe
@external(erlang, "host", "retain") fn retain(chars: List(UtfCodepoint)) -> Probe
@external(erlang, "host", "check") fn check(first: Probe, same: Probe, empty: Probe) -> Nil
pub fn main() {
  let assert <<a:utf8_codepoint>> = <<"A">>
  check(retain([a]), retain([a]), retain([]))
}
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let plan = plan_host_program(typed).unwrap();
        let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let expected = &mut state.erlang as *mut crate::Configuration;
        assert_eq!(Probe::project(&mut state) as *mut _, expected);
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut state, &mut echo))
                .unwrap(),
            geam_core::Value::Nil
        );
        assert!(echo.is_empty());
    }

    fn check<'call>(
        call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, ()>,
        ascii: HostExternal<'call, Charlist>,
        same: HostExternal<'call, Charlist>,
        different: HostExternal<'call, Charlist>,
        unicode: HostExternal<'call, Charlist>,
        integers: HostList<'call, BigInt>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert!(call.equal::<Charlist>(ascii, same));
        assert!(!call.equal::<Charlist>(ascii, different));
        assert_eq!(
            call.source_hash::<Charlist>(ascii),
            call.source_hash::<Charlist>(same)
        );
        assert_eq!(
            call.inspect::<Charlist>(ascii),
            "charlist.from_string(\"AZ\")"
        );
        assert_eq!(call.inspect::<Charlist>(unicode), "[0, 65, 233, 128578]");
        let unicode = call.native_value::<Charlist>(unicode);
        let integers = call.native_value::<HostListType<BigInt>>(integers);
        assert!(call.native_equal(&unicode, &integers));
        assert!(call.native_equal(&integers, &unicode));
        assert_eq!(call.native_hash(&unicode), call.native_hash(&integers));
        Ok(call.return_value(()))
    }

    #[test]
    fn typed_charlists_round_trip_and_keep_integer_list_native_semantics() {
        let check = HostProviderModule::<GleamErlangProfile>::new("application", "main")
            .unwrap()
            .with_scoped_function::<
                Component<GleamErlangProfile>,
                (Charlist, Charlist, Charlist, Charlist, HostListType<BigInt>),
                (),
                _,
            >("check", check)
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/charlist",
                        "charlist.gleam",
                        r#"
pub type Charlist
@external(erlang, "unicode", "characters_to_list")
pub fn from_string(value: String) -> Charlist
@external(erlang, "unicode", "characters_to_binary")
pub fn to_string(value: Charlist) -> String
"#,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/erlang/charlist.{type Charlist}
@external(erlang, "host", "check")
fn check(a: Charlist, b: Charlist, c: Charlist, d: Charlist, ints: List(Int)) -> Nil
pub fn main() {
  let text = "\u{0}A\u{e9}\u{1f642}"
  check(charlist.from_string("AZ"), charlist.from_string("AZ"),
    charlist.from_string("B"), charlist.from_string(text), [0, 65, 233, 128578])
  #(charlist.to_string(charlist.from_string("")),
    charlist.to_string(charlist.from_string(text)) == text)
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([host_provider().unwrap(), check]).unwrap(),
        )
        .unwrap();
        let plan = plan_host_program(typed).unwrap();
        let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        let value = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        assert_eq!(value.inspect().to_string(), "#(\"\", True)");
        assert!(echo.is_empty());
    }
}
