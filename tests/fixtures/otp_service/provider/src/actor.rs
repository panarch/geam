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
    let mut message = charlist_string(&mut call, format).to_string();
    let mut index = 0;
    while let Some(argument) = call.list_item::<Charlist>(arguments, index) {
        let argument = charlist_string(&mut call, argument);
        message = message.replacen("~s", &argument, 1);
        index += 1;
    }
    call.state().warnings.push(message);
    Ok(call.return_value(()))
}

#[cfg(test)]
mod tests {
    use crate::test_support::{Profile, native_boundary, run_source};

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
