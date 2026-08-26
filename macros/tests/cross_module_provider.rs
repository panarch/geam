use ecow::EcoString;
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(
    package = "cross_module",
    modules = [declarations, consumer],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "declarations", crate_path = geam_core)]
mod declarations {
    use super::{BigInt, EcoString};

    #[geam_macros::external(name = "Token")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Token(pub(super) EcoString);

    #[geam_macros::custom(input = StatusInput)]
    pub enum Status {
        Ready,
        Count(BigInt),
        Tagged(Token),
    }
}

#[geam_macros::module(path = "consumer", crate_path = geam_core)]
mod consumer {
    use super::{BigInt, EcoString, declarations};

    #[geam_macros::custom(input = EnvelopeInput)]
    enum Envelope {
        One(declarations::Status),
        Many(Vec<declarations::Status>),
        Token(declarations::Token),
        Flags(Vec<std::primitive::bool>),
        Tokens(Vec<declarations::Token>),
    }

    #[geam_macros::function]
    fn ready() -> declarations::Status {
        declarations::Status::Ready
    }

    #[geam_macros::function]
    fn count(value: BigInt) -> declarations::Status {
        declarations::Status::Count(value)
    }

    #[geam_macros::function]
    fn token(value: EcoString) -> declarations::Token {
        declarations::Token(value)
    }

    #[geam_macros::function]
    fn status_text(value: declarations::StatusInput) -> EcoString {
        match value {
            declarations::StatusInput::Ready => "ready".into(),
            declarations::StatusInput::Count(value) => format!("count:{value}").into(),
            declarations::StatusInput::Tagged(value) => format!("tagged:{}", value.0).into(),
        }
    }

    #[geam_macros::function]
    fn token_text(value: &declarations::Token) -> EcoString {
        value.0.clone()
    }

    #[geam_macros::function]
    fn wrap(value: declarations::StatusInput) -> Envelope {
        let value = match value {
            declarations::StatusInput::Ready => declarations::Status::Ready,
            declarations::StatusInput::Count(value) => declarations::Status::Count(value),
            declarations::StatusInput::Tagged(value) => {
                declarations::Status::Tagged(declarations::Token(value.0.clone()))
            }
        };
        Envelope::One(value)
    }

    #[geam_macros::function]
    fn many(value: BigInt) -> Envelope {
        Envelope::Many(vec![
            declarations::Status::Ready,
            declarations::Status::Count(value),
        ])
    }

    #[geam_macros::function]
    fn wrapped_token(value: EcoString) -> Envelope {
        Envelope::Token(declarations::Token(value))
    }

    #[geam_macros::function]
    fn flags() -> Envelope {
        Envelope::Flags(vec![true, false])
    }

    #[geam_macros::function]
    fn tokens(value: EcoString) -> Envelope {
        Envelope::Tokens(vec![declarations::Token(value)])
    }

    #[geam_macros::function]
    fn envelope_text(value: EnvelopeInput) -> EcoString {
        match value {
            EnvelopeInput::One(value) => format!("one:{}", status_text(value)).into(),
            EnvelopeInput::Many(values) => {
                let second = values.get(1).map_or_else(|| "missing".into(), status_text);
                format!("many:{}:{second}", values.len()).into()
            }
            EnvelopeInput::Token(value) => format!("token:{}", value.0).into(),
            EnvelopeInput::Flags(values) => {
                let first = values.get(0).unwrap_or(false);
                format!("flags:{}:{first}", values.len()).into()
            }
            EnvelopeInput::Tokens(values) => {
                let first = values
                    .get(0)
                    .map_or_else(|| "missing".into(), |value| value.0.clone());
                format!("tokens:{}:{first}", values.len()).into()
            }
        }
    }

    #[geam_macros::function]
    fn first(values: geam_core::List<declarations::StatusInput>) -> EcoString {
        values.get(0).map_or_else(|| "missing".into(), status_text)
    }

    #[geam_macros::function]
    fn first_envelope(values: geam_core::List<EnvelopeInput>) -> EcoString {
        values
            .get(0)
            .map_or_else(|| "missing".into(), envelope_text)
    }

    #[geam_macros::function]
    fn invert(value: std::primitive::bool) -> std::primitive::bool {
        !value
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct RunState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const DECLARATIONS: &str = r#"
@external(erlang, "cross_module", "Token")
pub type Token

pub type Status {
  Ready
  Count(Int)
  Tagged(Token)
}
"#;

const CONSUMER: &str = r#"
import declarations

pub type Envelope {
  One(declarations.Status)
  Many(List(declarations.Status))
  Token(declarations.Token)
  Flags(List(Bool))
  Tokens(List(declarations.Token))
}

@external(erlang, "cross_module", "ready")
fn ready() -> declarations.Status
@external(erlang, "cross_module", "count")
fn count(value: Int) -> declarations.Status
@external(erlang, "cross_module", "token")
fn token(value: String) -> declarations.Token
@external(erlang, "cross_module", "status_text")
fn status_text(value: declarations.Status) -> String
@external(erlang, "cross_module", "token_text")
fn token_text(value: declarations.Token) -> String
@external(erlang, "cross_module", "wrap")
fn wrap(value: declarations.Status) -> Envelope
@external(erlang, "cross_module", "many")
fn many(value: Int) -> Envelope
@external(erlang, "cross_module", "wrapped_token")
fn wrapped_token(value: String) -> Envelope
@external(erlang, "cross_module", "flags")
fn flags() -> Envelope
@external(erlang, "cross_module", "tokens")
fn tokens(value: String) -> Envelope
@external(erlang, "cross_module", "envelope_text")
fn envelope_text(value: Envelope) -> String
@external(erlang, "cross_module", "first")
fn first(values: List(declarations.Status)) -> String
@external(erlang, "cross_module", "first_envelope")
fn first_envelope(values: List(Envelope)) -> String
@external(erlang, "cross_module", "invert")
fn invert(value: Bool) -> Bool

pub fn main() {
  assert status_text(ready()) == "ready"
  assert status_text(count(7)) == "count:7"
  assert token_text(token("blue")) == "blue"
  assert envelope_text(wrap(count(8))) == "one:count:8"
  assert envelope_text(many(9)) == "many:2:count:9"
  assert envelope_text(wrapped_token("green")) == "token:green"
  assert envelope_text(flags()) == "flags:2:true"
  assert envelope_text(tokens("violet")) == "tokens:1:violet"
  assert first([]) == "missing"
  assert first([count(10)]) == "count:10"
  assert first_envelope([flags()]) == "flags:2:true"
  assert first_envelope([tokens("indigo")]) == "tokens:1:indigo"
  assert invert(True) == False
  True
}
"#;

#[test]
fn sibling_modules_share_static_custom_and_external_codecs() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("cross-module provider should register");
    assert_eq!(
        providers
            .iter()
            .map(|provider| provider.module().as_str())
            .collect::<Vec<_>>(),
        ["declarations", "consumer"],
    );
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("provider modules should be unique");
    let typed = compile_typed_host_program(
        "cross_module",
        "consumer",
        [PackageSource::new(
            "cross_module",
            Vec::<&str>::new(),
            [
                ModuleSource::new("declarations", "src/declarations.gleam", DECLARATIONS),
                ModuleSource::new("consumer", "src/consumer.gleam", CONSUMER),
            ],
        )],
        hosts,
    )
    .expect("cross-module source should compile");
    let plan = plan_host_program(typed).expect("cross-module codecs should link");
    let execution = HostedExecution::try_from_module_plan(plan).expect("plan should seal");

    assert_eq!(
        execution.run_main(&mut RunState { component: () }, &mut Vec::new()),
        Ok(Value::Bool(true)),
    );
}
