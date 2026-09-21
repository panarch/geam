#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::provider::BigInt;
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};

#[derive(Default)]
pub struct State {
    events: Vec<BigInt>,
}

#[geam_macros::provider(package = "system_provider", state = State, modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "system_provider", crate_path = geam_core)]
mod native {
    use super::{BigInt, State};
    use geam_core::provider::{Call, Callback, Factory, HostResult, Value};

    #[geam_macros::custom(input = SystemInput)]
    enum SystemMessage {
        GetState(Callback<fn(BigInt) -> ()>),
        Suspend(Callback<fn() -> ()>),
        Resume(Callback<fn() -> ()>),
    }

    #[geam_macros::custom(input = MessageInput)]
    enum Message<Item> {
        User(Value<Item>),
        System(SystemMessage),
    }

    #[geam_macros::callable(factory = Reply, await)]
    async fn reply(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] route: BigInt,
        #[geam_macros::capture] deliver: Callback<fn(BigInt) -> ()>,
        value: BigInt,
    ) -> HostResult<()> {
        call.invoke(&deliver, (route + value,)).await
    }

    #[geam_macros::callable(factory = Signal)]
    fn signal(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] route: BigInt,
        #[geam_macros::capture] tag: BigInt,
    ) -> () {
        call.state_mut().events.push(route + tag);
    }

    #[geam_macros::function]
    fn convert<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] reply: Factory<Reply>,
        #[geam_macros::factory] signal: Factory<Signal>,
        kind: BigInt,
        route: BigInt,
        deliver: Callback<fn(BigInt) -> ()>,
        message: Value<Item>,
    ) -> HostResult<Message<Item>> {
        Ok(if kind == BigInt::from(0) {
            Message::System(SystemMessage::GetState(
                call.create(&reply, (route, deliver))?,
            ))
        } else if kind == BigInt::from(1) {
            Message::System(SystemMessage::Suspend(
                call.create(&signal, (route, 100.into()))?,
            ))
        } else if kind == BigInt::from(2) {
            Message::System(SystemMessage::Resume(
                call.create(&signal, (route, 200.into()))?,
            ))
        } else {
            Message::User(message)
        })
    }

    #[geam_macros::function]
    fn record(#[geam_macros::call] call: &mut Call<State>, value: BigInt) -> () {
        call.state_mut().events.push(value);
    }
}

struct Profile;
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = <Component as HostProviderComponent>::Stores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
        stores
    }
    fn component_state(state: &mut State) -> &mut State {
        state
    }
}

#[test]
fn nested_system_replies_preserve_phantom_messages_captures_and_reentry() {
    let source = r#"
pub type SystemMessage { GetState(fn(Int) -> Nil) Suspend(fn() -> Nil) Resume(fn() -> Nil) }
pub type Message(item) { User(item) System(SystemMessage) }
@external(erlang, "native", "convert")
fn convert(kind: Int, route: Int, deliver: fn(Int) -> Nil, message: item) -> Message(item)
@external(erlang, "native", "record")
fn record(value: Int) -> Nil
pub fn main() {
  let assert System(GetState(reply)) = convert(0, 40, record, 7)
  let assert System(Suspend(suspend)) = convert(1, 10, record, "message")
  let assert System(Resume(resume)) = convert(2, 20, record, #(True, 9))
  let assert User(ordinary) = convert(3, 0, record, fn() { 17 })
  let alias = reply
  assert alias == reply
  assert suspend != resume
  reply(2)
  suspend()
  resume()
  alias(3)
  ordinary()
}
"#;
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "system_provider",
        "system_provider",
        [PackageSource::new(
            "system_provider",
            Vec::<String>::new(),
            [ModuleSource::new("system_provider", "system.gleam", source)],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let mut state = State::default();
    assert_eq!(
        execution_fixture::run(&mut execution, &mut state, &mut Vec::new()),
        Ok(Value::Int(17.into()))
    );
    assert_eq!(state.events, [42.into(), 110.into(), 220.into(), 43.into()]);
}
