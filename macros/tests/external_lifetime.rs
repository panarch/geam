#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::host::{HostCall, HostCallCompletion, HostCallError, HostType};
use geam_core::provider::{
    Call, ProviderConstructions, ProviderExternalCodec, ProviderExternalView,
    ProviderOwnedExternal, ProviderRootOutputValue, ProviderValue, ProviderValueContext, Value,
};
use geam_core::{
    HostComponentProfile, HostProfile, HostProvider, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource,
};
use std::cell::RefCell;

#[derive(Default)]
pub struct State {
    owned: Option<ProviderOwnedExternal<declarations::Token>>,
    value: Option<RetainedToken>,
}

#[geam_macros::provider(package = "external_lifetime", modules = [declarations], state = State, crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "tokens", crate_path = geam_core)]
mod declarations {
    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    pub struct Token(pub geam_core::StringValue);

    #[geam_macros::function]
    fn token(value: geam_core::StringValue) -> Token {
        Token(value)
    }
}

type TokenHost = <declarations::Token as ProviderValue>::Host;
type RetainedToken = Value<declarations::Token, ProviderValueContext<TokenHost>>;

thread_local! {
    // A direct payload view is not Send, but safe Rust can retain it on its thread.
    static BORROWED: RefCell<Option<ProviderExternalView<declarations::Token>>> = const { RefCell::new(None) };
}

struct Profile;
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &Stores {
        stores
    }
    fn component_state(state: &mut State) -> &mut State {
        state
    }
}
impl HostProvider<Profile> for Component {
    type State = State;
    fn project(state: &mut State) -> &mut State {
        Profile::component_state(state)
    }
}

fn borrowed<'call>(
    call: HostCall<'call, Profile, Component, TokenHost>,
    value: <TokenHost as HostType>::Value<'call>,
) -> Result<HostCallCompletion<'call, TokenHost>, HostCallError> {
    let current = declarations::Token::immediate_input(&call, value);
    let previous = BORROWED.replace(Some(current));
    let returned = previous.unwrap_or_else(|| declarations::Token::immediate_input(&call, value));
    ProviderRootOutputValue::complete(returned, call, &ProviderConstructions::none())
}

fn owned<'call>(
    mut call: HostCall<'call, Profile, Component, TokenHost>,
    value: <TokenHost as HostType>::Value<'call>,
) -> Result<HostCallCompletion<'call, TokenHost>, HostCallError> {
    let current = declarations::Token::owned_input(&call, value);
    let returned = call
        .state()
        .owned
        .replace(current.clone())
        .unwrap_or(current);
    ProviderRootOutputValue::complete(returned, call, &ProviderConstructions::none())
}

fn payload<'call>(
    mut call: HostCall<'call, Profile, Component, TokenHost>,
    value: <TokenHost as HostType>::Value<'call>,
) -> Result<HostCallCompletion<'call, TokenHost>, HostCallError> {
    let current = RetainedToken::from_host(&call, value);
    let returned = call
        .state()
        .value
        .replace(current.clone())
        .unwrap_or(current);
    let mut call = Call::from_host_call(call);
    let payload = call.external_payload(returned)?;
    assert_eq!(payload.0, "original");
    drop(payload);
    Ok(call.into_host_call().return_value(value))
}

fn execution(function: &str) -> HostedExecution<Profile> {
    let mut providers = Component::providers().unwrap();
    let module = providers
        .pop()
        .unwrap()
        .with_scoped_function::<Component, (TokenHost,), TokenHost, _>("borrowed", borrowed)
        .unwrap()
        .with_scoped_function::<Component, (TokenHost,), TokenHost, _>("owned", owned)
        .unwrap()
        .with_scoped_function::<Component, (TokenHost,), TokenHost, _>("payload", payload)
        .unwrap();
    providers.push(module);
    let source = format!(
        r#"
pub type Token
@external(erlang, "native", "token") fn token(value: String) -> Token
@external(erlang, "native", "borrowed") fn borrowed(value: Token) -> Token
@external(erlang, "native", "owned") fn owned(value: Token) -> Token
@external(erlang, "native", "payload") fn payload(value: Token) -> Token
pub fn main() {{ let _ = {function}(token("original")) 42 }}
"#
    );
    let typed = geam_core::compile_typed_host_program(
        "external_lifetime",
        "tokens",
        [PackageSource::new(
            "external_lifetime",
            Vec::<&str>::new(),
            [ModuleSource::new("tokens", "tokens.gleam", source)],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    HostedExecution::try_from_module_plan(geam_core::plan_host_program(typed).unwrap()).unwrap()
}

#[test]
fn payload_views_and_root_outputs_preserve_the_original_owner_and_execution() {
    for function in ["borrowed", "owned", "payload"] {
        BORROWED.take();
        let mut state = State::default();
        let mut original = execution(function);
        assert_eq!(
            execution_fixture::run(&mut original, &mut state, &mut Vec::new()).unwrap(),
            geam_core::Value::Int(42.into()),
        );
        for (execution, message) in [
            (&mut original, "retained value belongs to another execution"),
            (
                &mut execution(function),
                "retained value belongs to another owner or source type",
            ),
        ] {
            let error = execution_fixture::run(execution, &mut state, &mut Vec::new()).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!("host function external_lifetime::tokens.{function} failed: {message}")
            );
        }
        BORROWED.take();
    }
}
