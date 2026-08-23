#[geam_macros::provider(
    package = "macro_consumer",
    modules = [main],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "macro_consumer/main", crate_path = geam_core)]
mod main {
    use ecow::EcoString;
    use geam_macro_cross_crate_declarations::values;
    use num_bigint::BigInt;

    #[geam_macros::custom(input = EnvelopeInput)]
    enum Envelope {
        One(values::Status),
        Many(Vec<values::Status>),
        Token(values::Token),
    }

    #[geam_macros::function]
    fn ready() -> values::Status {
        values::Status::Ready
    }

    #[geam_macros::function]
    fn count(value: BigInt) -> values::Status {
        values::Status::Count(value)
    }

    #[geam_macros::function]
    fn token(value: EcoString) -> values::Token {
        values::Token(value)
    }

    #[geam_macros::function]
    fn status_text(value: values::StatusInput) -> EcoString {
        match value {
            values::StatusInput::Ready => "ready".into(),
            values::StatusInput::Count(value) => format!("count:{value}").into(),
            values::StatusInput::Tagged(value) => format!("tagged:{}", value.0).into(),
        }
    }

    #[geam_macros::function]
    fn token_text(value: &values::Token) -> EcoString {
        value.0.clone()
    }

    #[geam_macros::function]
    fn many(value: BigInt) -> Envelope {
        Envelope::Many(vec![values::Status::Ready, values::Status::Count(value)])
    }

    #[geam_macros::function]
    fn wrapped_token(value: EcoString) -> Envelope {
        Envelope::Token(values::Token(value))
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
        }
    }

    #[geam_macros::function]
    fn first(values: geam_core::List<values::StatusInput>) -> EcoString {
        values.get(0).map_or_else(|| "missing".into(), status_text)
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use geam_core::{
        HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
        HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, Value, compile_typed_host_program, plan_host_program,
    };
    use geam_macro_cross_crate_declarations::Component as DeclarationsComponent;
    use geam_macro_cross_crate_declarations::values;

    struct Profile;

    #[derive(Default)]
    struct ProfileStores {
        declarations: <DeclarationsComponent as HostProviderComponent>::Stores,
        consumer: <Component as HostProviderComponent>::Stores,
    }

    struct RunState {
        declarations: <DeclarationsComponent as HostProviderComponent>::RunState,
        consumer: <Component as HostProviderComponent>::RunState,
    }

    impl HostProfile for Profile {
        type RunState = RunState;
        type ExternalStores = ProfileStores;
    }

    impl HostComponentProfile<DeclarationsComponent> for Profile {
        fn component_stores(
            stores: &Self::ExternalStores,
        ) -> &<DeclarationsComponent as HostProviderComponent>::Stores {
            &stores.declarations
        }

        fn component_state(
            state: &mut Self::RunState,
        ) -> &mut <DeclarationsComponent as HostProviderComponent>::RunState {
            &mut state.declarations
        }
    }

    impl HostComponentProfile<Component> for Profile {
        fn component_stores(
            stores: &Self::ExternalStores,
        ) -> &<Component as HostProviderComponent>::Stores {
            &stores.consumer
        }

        fn component_state(
            state: &mut Self::RunState,
        ) -> &mut <Component as HostProviderComponent>::RunState {
            &mut state.consumer
        }
    }

    const DECLARATIONS: &str = r#"
@external(erlang, "macro_declarations", "Token")
pub type Token

pub type Status {
  Ready
  Count(Int)
  Tagged(Token)
}
"#;

    const CONSUMER: &str = r#"
import macro_declarations/values

pub type Envelope {
  One(values.Status)
  Many(List(values.Status))
  Token(values.Token)
}

@external(erlang, "macro_consumer", "ready")
fn ready() -> values.Status
@external(erlang, "macro_consumer", "count")
fn count(value: Int) -> values.Status
@external(erlang, "macro_consumer", "token")
fn token(value: String) -> values.Token
@external(erlang, "macro_consumer", "status_text")
fn status_text(value: values.Status) -> String
@external(erlang, "macro_consumer", "token_text")
fn token_text(value: values.Token) -> String
@external(erlang, "macro_consumer", "many")
fn many(value: Int) -> Envelope
@external(erlang, "macro_consumer", "wrapped_token")
fn wrapped_token(value: String) -> Envelope
@external(erlang, "macro_consumer", "envelope_text")
fn envelope_text(value: Envelope) -> String
@external(erlang, "macro_consumer", "first")
fn first(values: List(values.Status)) -> String

pub fn main() {
  assert status_text(ready()) == "ready"
  assert status_text(count(7)) == "count:7"
  assert token_text(token("blue")) == "blue"
  assert envelope_text(many(8)) == "many:2:count:8"
  assert envelope_text(wrapped_token("green")) == "token:green"
  assert first([]) == "missing"
  assert first([count(9)]) == "count:9"
  True
}
"#;

    #[test]
    fn sibling_crates_share_static_custom_and_external_codecs() {
        let mut providers =
            <DeclarationsComponent as HostProviderComponentRegistration<Profile>>::providers()
                .expect("declaration provider should register");
        providers.extend(
            <Component as HostProviderComponentRegistration<Profile>>::providers()
                .expect("consumer provider should register"),
        );
        let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
            .expect("cross-crate provider modules should be unique");
        let typed = compile_typed_host_program(
            "macro_consumer",
            "macro_consumer/main",
            [
                PackageSource::new(
                    "macro_declarations",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "macro_declarations/values",
                        "src/values.gleam",
                        DECLARATIONS,
                    )],
                ),
                PackageSource::new(
                    "macro_consumer",
                    ["macro_declarations"],
                    [ModuleSource::new(
                        "macro_consumer/main",
                        "src/main.gleam",
                        CONSUMER,
                    )],
                ),
            ],
            hosts,
        )
        .expect("cross-crate source should compile");
        let plan = plan_host_program(typed).expect("cross-crate codecs should link");
        let execution = HostedExecution::try_from_module_plan(plan).expect("plan should seal");

        assert_eq!(
            execution.run_main(
                &mut RunState {
                    declarations: (),
                    consumer: (),
                },
                &mut Vec::new(),
            ),
            Ok(Value::Bool(true)),
        );
    }
}
