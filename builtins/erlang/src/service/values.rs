use crate::execution::{Destination, ReferenceId};
use crate::schema::{AtomSchema, CharlistSchema, MonitorSchema, ReferenceSchema};
use crate::{Charlist, Component, GleamErlangHostProfile, Pid, PidSchema, Reference, reference};
use geam_core::execution::ExecutionUnit;
use geam_core::host::native::NativeRules;
use geam_core::host::{
    HostCall, HostConstruction, HostCustom, HostExternal, HostListType, HostProvider, HostType,
    HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::provider_support::DynamicSchema;

/// Constructs a Charlist through the producer's original retained character list.
///
/// Register both [`Charlist`] and `HostListType<char>` constructions on the
/// calling function and pass their exact tokens. This also permits construction
/// inside a tuple or list return without a consumer-owned Charlist binding.
///
/// The value preserves Unicode scalar values, including NUL and combining
/// characters, without normalization. It retains its own list and does not
/// borrow `text`; the returned handle remains scoped to the active host call.
pub fn charlist_from_string<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    charlist: HostConstruction<'call, Charlist>,
    characters: HostConstruction<'call, HostListType<char>>,
    text: &str,
) -> HostExternal<'call, Charlist>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let characters = call.construct_list(characters, text.chars());
    call.construct_retained_external_with_binding::<Component<Profile>, CharlistSchema, HostTypeListEnd>(
        charlist,
        |builder| builder.store::<HostListType<char>>(characters),
    )
}

/// Decodes a source Charlist through its original retained list representation.
pub fn charlist_string<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    value: HostExternal<'call, Charlist>,
) -> geam_core::StringValue
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let characters = call
        .external_payload_with::<Component<Profile>, CharlistSchema, HostTypeListEnd>(value)
        .restore(call, |characters| characters);
    let mut output = ecow::EcoString::new();
    let mut index = 0;
    while let Some(character) = call.list_item::<char>(characters, index) {
        output.push(character);
        index += 1;
    }
    output.into()
}

/// Adds the process producer's native conversions to a consumer registration.
/// Exact typed values keep their original source representation and binding.
pub fn native_rules<Profile, Provider, Return>(
    rules: NativeRules<Profile, Provider, Return>,
) -> NativeRules<Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    rules
        .external::<AtomSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, AtomSchema, HostTypeListEnd>(construction, value))
        })
        .external::<ReferenceSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(construction, reference::Payload::View(value)))
        })
        .external::<DynamicSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, DynamicSchema, HostTypeListEnd>(construction, geam_stdlib::Dynamic::from_native(value)))
        })
        .external::<MonitorSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, MonitorSchema, HostTypeListEnd>(construction, value))
        })
}

/// Constructs a fresh source Reference using the producer's original identity store.
pub fn new_reference<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Reference>,
) -> HostExternal<'call, Reference>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call.construct_external_with_binding::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(
        construction,
        reference::Payload::Identity(ReferenceId::new()),
    )
}

/// Constructs a source Pid for an existing logical unit without extending its lifetime.
pub fn pid_value<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Pid>,
    unit: ExecutionUnit,
) -> HostExternal<'call, Pid>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call.construct_external_with_binding::<Component<Profile>, PidSchema, HostTypeListEnd>(
        construction,
        unit,
    )
}

pub(crate) fn fresh_name<Profile, Provider, Return>(
    call: &mut HostCall<'_, Profile, Provider, Return>,
    prefix: &str,
) -> Result<ecow::EcoString, geam_core::HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let name = format!("{prefix}${}", ReferenceId::new().number()).into();
    Ok(Profile::erlang_execution(call.execution_state()).intern(name)?)
}

pub(crate) fn subject_parts<'call, Profile, Provider, Return, Message>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    subject: HostCustom<'call, super::types::Subject<Message>>,
) -> (Destination, NativeValue)
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: HostType,
{
    use super::types::{NamedSubject, OrdinarySubject};
    if let Some((owner, (tag, ()))) = call.custom_fields::<OrdinarySubject<Message>>(subject) {
        let owner = call
            .external_payload_with::<Component<Profile>, PidSchema, HostTypeListEnd>(owner)
            .id();
        let tag = call
            .external_payload_with::<Component<Profile>, DynamicSchema, HostTypeListEnd>(tag)
            .native_value()
            .clone();
        (Destination::Pid(owner), tag)
    } else {
        let (name, ()) =
            call.provider_borrow_remaining_custom_fields::<NamedSubject<Message>>(subject);
        let destination = call.external_payload_with::<Component<Profile>, crate::NameSchema, HostTypeList<Message, HostTypeListEnd>>(name).clone();
        (
            Destination::Name(destination),
            call.native_value::<crate::Name<Message>>(name),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{charlist_from_string, charlist_string, new_reference};
    use crate::{
        Charlist, Component, GleamErlangProfile, GleamErlangRunState, Reference, ReferenceSchema,
    };
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostList,
        HostListType, HostProviderModule, HostProviderSet, HostTupleType, HostTypeIndex0,
        HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, StringValue, compile_typed_host_program,
        plan_host_program,
    };
    use num_bigint::BigInt;

    type CharlistConstructions =
        HostTypeList<Charlist, HostTypeList<HostListType<char>, HostTypeListEnd>>;
    type CharlistPair =
        HostTupleType<HostTypeList<Charlist, HostTypeList<Charlist, HostTypeListEnd>>>;

    #[test]
    fn constructed_charlists_keep_native_semantics_and_own_their_retained_text() {
        let consumer = HostProviderModule::<GleamErlangProfile>::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<
                Component<GleamErlangProfile>,
                (StringValue, Charlist, HostListType<BigInt>, StringValue),
                CharlistPair,
                CharlistConstructions,
                _,
            >("construct", construct_and_check)
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
@external(erlang, "host", "construct")
fn construct(text: String, same: Charlist, integers: List(Int), inspection: String)
  -> #(Charlist, Charlist)
pub fn main() {
  let empty = construct("", charlist.from_string(""), [], "[]")
  assert charlist.to_string(empty.0) == ""
  let ascii = construct("AZ", charlist.from_string("AZ"), [65, 90],
    "charlist.from_string(\"AZ\")")
  assert charlist.to_string(ascii.0) == "AZ"
  let combining = construct("e\u{301}", charlist.from_string("e\u{301}"),
    [101, 769], "[101, 769]")
  assert charlist.to_string(combining.0) == "e\u{301}"
  let unicode = construct("\u{0}Aé🙂", charlist.from_string("\u{0}Aé🙂"),
    [0, 65, 233, 128578], "[0, 65, 233, 128578]")
  assert charlist.to_string(unicode.0) == "\u{0}Aé🙂"
  assert unicode.0 == unicode.1
  unicode
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([crate::charlist::host_provider().unwrap(), consumer])
                .unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        let first = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        let second = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        drop(execution);
        drop(state);
        for value in [first, second] {
            assert_eq!(
                value.inspect().to_string(),
                "#([0, 65, 233, 128578], [0, 65, 233, 128578])"
            );
        }
        assert!(echo.is_empty());
    }

    fn construct_and_check<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, CharlistPair>,
        constructions: HostConstructions<'call, CharlistConstructions>,
        string: StringValue,
        same: HostExternal<'call, Charlist>,
        integers: HostList<'call, BigInt>,
        inspection: StringValue,
    ) -> Result<HostCallCompletion<'call, CharlistPair>, HostCallError> {
        let mut text = string.to_string();
        let value = charlist_from_string(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
            &text,
        );
        text.clear();
        text.push_str("changed input");
        drop(text);
        assert_eq!(charlist_string(&mut call, value), string);
        assert!(call.equal::<Charlist>(value, same));
        assert_eq!(
            call.source_hash::<Charlist>(value),
            call.source_hash::<Charlist>(same)
        );
        assert_eq!(call.inspect::<Charlist>(value).as_str(), &*inspection);
        let different = charlist_from_string(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
            "different text",
        );
        assert!(!call.equal::<Charlist>(value, different));
        let native = call.native_value::<Charlist>(value);
        let integers = call.native_value::<HostListType<BigInt>>(integers);
        assert!(call.native_equal(&native, &integers));
        assert!(call.native_equal(&integers, &native));
        assert_eq!(call.native_hash(&native), call.native_hash(&integers));
        Ok(call.return_tuple((value, (same, ()))))
    }

    #[test]
    fn fresh_references_are_distinct_producer_values_with_stable_identity() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, ReferenceSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), bool, HostTypeList<Reference, HostTypeListEnd>, _>("check", check)
            .unwrap();
        let result = crate::test_support::run_main(
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/reference",
                        "reference.gleam",
                        "pub type Reference",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "@external(erlang, \"host\", \"check\") fn check() -> Bool\npub fn main() { check() }",
                    )],
                ),
            ],
            [producer, consumer],
        );
        assert_eq!(result, geam_core::Value::Bool(true));
    }

    fn check<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, bool>,
        constructions: HostConstructions<'call, HostTypeList<Reference, HostTypeListEnd>>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let first = new_reference(&mut call, constructions.at::<HostTypeIndex0>());
        let second = new_reference(&mut call, constructions.at::<HostTypeIndex0>());
        let alias = first;
        assert!(call.equal::<Reference>(first, alias));
        assert_eq!(
            call.source_hash::<Reference>(first),
            call.source_hash::<Reference>(alias)
        );
        let result = !call.equal::<Reference>(first, second);
        Ok(call.return_value(result))
    }
}
