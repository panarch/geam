use crate::schema::{Message, Unexpected};
use crate::{A, Call, Component, One, OtpProfile};
use geam::gleam_erlang::service::{charlist_string, native_rules};
use geam::gleam_erlang::{Charlist, Component as ErlangComponent};
use geam::gleam_stdlib::provider_support::{Dynamic, DynamicSchema};
use geam::host::native::{NativeCall, NativeRules};
use geam::host::{
    HostCallCompletion, HostCallError, HostExternal, HostList, HostListType, HostProviderModule,
    HostRegistrationError, HostTypeIndex0, HostTypeListEnd, HostValue,
};
use geam::provider::advanced::{NativeKind, NativeValue};

pub(super) fn provider<Profile: OtpProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_otp", "gleam/otp/actor")
        .and_then(|module| module.with_native_function::<Component<Profile>, (Dynamic,), Message<A>, One<Message<A>>, _>(
            "convert_system_message",
            native_rules(NativeRules::default()),
            convert::<Profile>,
        ))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (A,), Dynamic, _>("erase", erase::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Charlist, HostListType<Charlist>), (), _>(
            "log_warning",
            log_warning::<Profile>,
        ))
}

fn convert<'call, Profile: OtpProfile>(
    mut call: NativeCall<'call, Profile, Component<Profile>, Message<A>, One<Message<A>>>,
    input: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, Message<A>>, HostCallError> {
    let source = input;
    let input = call
        .call()
        .external_payload_with::<ErlangComponent<Profile>, DynamicSchema, HostTypeListEnd>(input)
        .native_value()
        .clone();
    if let (NativeKind::Tuple, Some(tag), Some(message), Some(3)) =
        (input.kind(), input.index(0), input.index(1), input.len())
        && tag.as_symbol().as_deref() == Some("system")
    {
        let message = NativeValue::tuple([NativeValue::symbol("system"), message]);
        if let Some(value) = call.convert::<HostTypeIndex0>(&message) {
            return Ok(call.finish(value));
        }
    }
    let (call, _) = call.into_call();
    Ok(call.return_custom::<Unexpected<A>>((source, ())))
}

fn erase<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, Dynamic>,
    value: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
    let value = call.native_value::<A>(value);
    let dynamic = call.create_external_with_binding::<ErlangComponent<Profile>>(
        geam::gleam_stdlib::Dynamic::from_native(value),
    );
    Ok(call.return_value(dynamic))
}

fn log_warning<'call, Profile: OtpProfile>(
    mut call: Call<'call, Profile, ()>,
    format: HostExternal<'call, Charlist>,
    arguments: HostList<'call, Charlist>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let mut message = charlist_string(&mut call, format)?.to_string();
    let mut index = 0;
    while let Some(argument) = call.list_item::<Charlist>(arguments, index) {
        let argument = charlist_string(&mut call, argument)?.to_string();
        message = message.replacen("~s", &argument, 1);
        index += 1;
    }
    call.state().warnings.push(message);
    Ok(call.return_value(()))
}

#[cfg(test)]
mod tests {
    use crate::test_support::{Profile, host_failure, native_boundary, run_source};
    use geam::gleam_erlang::{Charlist, CharlistSchema, Component as ErlangComponent};
    use geam::host::{
        HostCallCompletion, HostCallError, HostConstructions, HostListType, HostProviderModule,
        HostStoredValue, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    };
    use geam::{ModuleSource, PackageSource};
    use std::sync::{Arc, Mutex};

    type CharlistConstructions =
        HostTypeList<Charlist, HostTypeList<HostListType<char>, HostTypeListEnd>>;

    fn retained_characters(
        previous: Arc<Mutex<Option<HostStoredValue<HostListType<char>>>>>,
    ) -> impl for<'call> Fn(
        crate::Call<'call, Profile, Charlist>,
        HostConstructions<'call, CharlistConstructions>,
    ) -> Result<HostCallCompletion<'call, Charlist>, HostCallError> {
        move |mut call, constructions| {
            let retained = previous.lock().unwrap().take();
            let construction = constructions.at::<HostTypeIndex0>();
            let value = match retained {
                Some(characters) => call.construct_external_with_binding::<
                    ErlangComponent<Profile>,
                    CharlistSchema,
                    HostTypeListEnd,
                >(construction, characters),
                None => {
                    let characters = call.construct_list(
                        constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
                        "retained".chars(),
                    );
                    call.construct_retained_external_with_binding::<
                        ErlangComponent<Profile>,
                        CharlistSchema,
                        HostTypeListEnd,
                    >(construction, |builder| {
                        *previous.lock().unwrap() =
                            Some(builder.store::<HostListType<char>>(characters));
                        builder.store::<HostListType<char>>(characters)
                    })
                }
            };
            Ok(call.return_value(value))
        }
    }

    #[test]
    fn warning_format_and_arguments_reject_characters_from_another_loaded_program() {
        for warning in [
            "actor.log_warning(retained(), [])",
            "actor.log_warning(charlist.from_string(\"~s\"), [retained()])",
        ] {
            let previous = Arc::new(Mutex::new(None));
            for body in ["let _ = retained() Nil", warning] {
                let providers = geam::gleam_erlang::host_providers::<Profile>()
                    .unwrap()
                    .into_iter()
                    .filter(|module| module.module() == "gleam/erlang/charlist")
                    .chain([
                        HostProviderModule::new("application", "main")
                            .unwrap()
                            .with_scoped_function_and_constructions::<crate::Component<Profile>, (), Charlist, CharlistConstructions, _>(
                                "retained", retained_characters(previous.clone()),
                            ).unwrap(),
                        HostProviderModule::new("gleam_otp", "gleam/otp/actor")
                            .unwrap()
                            .with_scoped_function::<crate::Component<Profile>, (Charlist, HostListType<Charlist>), (), _>(
                                "log_warning", super::log_warning::<Profile>,
                            ).unwrap(),
                    ]);
                let result = run_source(
                    [
                        PackageSource::new(
                            "gleam_erlang",
                            Vec::<String>::new(),
                            [ModuleSource::new(
                                "gleam/erlang/charlist",
                                "charlist.gleam",
                                r#"
pub type Charlist
@external(erlang, "unicode", "characters_to_list") pub fn from_string(value: String) -> Charlist
@external(erlang, "unicode", "characters_to_binary") pub fn to_string(value: Charlist) -> String
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
import gleam/erlang/charlist.{type Charlist}
@external(erlang, "host", "log_warning") pub fn log_warning(format: Charlist, arguments: List(Charlist)) -> Nil
"#,
                            )],
                        ),
                        PackageSource::new(
                            "application",
                            ["gleam_erlang", "gleam_otp"],
                            [ModuleSource::new(
                                "main",
                                "main.gleam",
                                format!(
                                    r#"
import gleam/erlang/charlist
import gleam/otp/actor
@external(erlang, "host", "retained") fn retained() -> charlist.Charlist
pub fn main() {{ {body} }}
"#
                                ),
                            )],
                        ),
                    ],
                    providers,
                );
                if body == warning {
                    assert_eq!(
                        host_failure(result).as_deref(),
                        Some("retained value belongs to another owner or source type"),
                    );
                } else {
                    assert_eq!(result.unwrap(), geam::Value::Nil);
                }
            }
        }
    }

    #[test]
    fn native_converter_restores_system_callbacks_and_preserves_unexpected_input() {
        let actor = r#"
import gleam/dynamic.{type Dynamic}
import gleam/otp/system.{type SystemMessage}
pub type Message(msg) { Message(msg) System(SystemMessage) Unexpected(Dynamic) }
@external(erlang, "fixture", "convert_system_message") pub fn convert_system_message(value: Dynamic) -> Message(msg)
@external(erlang, "fixture", "erase") pub fn erase(value: a) -> Dynamic
"#;
        let system = r#"
import gleam/dynamic.{type Dynamic}
import gleam/erlang/atom.{type Atom}
import gleam/erlang/process.{type Pid}
pub type DebugState
pub type Mode { Running Suspended }
pub type StatusInfo { StatusInfo(module: Atom, parent: Pid, mode: Mode, debug_state: DebugState, state: Dynamic) }
pub type SystemMessage { Resume(fn() -> Nil) Suspend(fn() -> Nil) GetState(fn(Dynamic) -> Nil) GetStatus(fn(StatusInfo) -> Nil) }
"#;
        let source = r#"
import gleam/erlang/atom
import gleam/otp/actor
import gleam/otp/system
fn check(value: a) {
  let original = actor.erase(value)
  let message: actor.Message(Int) = actor.convert_system_message(original)
  let assert actor.Unexpected(actual) = message
  let assert True = actual == original
}
pub fn main() {
  let original = actor.erase(#(atom.create("system"), system.Resume(fn() { Nil }), Nil))
  let assert actor.System(system.Resume(reply)) = actor.convert_system_message(original)
  reply()
  check("ordinary value")
  check(#())
  check(#(atom.create("other"), 0, Nil))
  check(#(atom.create("system"), 0))
  check(#(atom.create("system"), 0, Nil))
  Nil
}
"#;
        let mut providers = native_boundary::providers();
        providers.push(geam::HostProviderModule::new("gleam_otp", "gleam/otp/actor").unwrap()
            .with_native_function::<crate::Component<Profile>, (super::Dynamic,), super::Message<crate::A>, crate::One<super::Message<crate::A>>, _>("convert_system_message", super::native_rules(geam::host::native::NativeRules::default()), super::convert::<Profile>).unwrap()
            .with_scoped_function::<crate::Component<Profile>, (crate::A,), super::Dynamic, _>("erase", super::erase::<Profile>).unwrap());
        providers.push(
            geam::HostProviderModule::new("gleam_otp", "gleam/otp/system")
                .unwrap()
                .with_external_type::<crate::Component<Profile>, crate::schema::DebugStateSchema>()
                .unwrap(),
        );
        assert_eq!(
            run_source(
                native_boundary::packages(
                    source,
                    &[("gleam/otp/actor", actor), ("gleam/otp/system", system),]
                ),
                providers
            )
            .unwrap(),
            geam::Value::Nil
        );
    }
}
