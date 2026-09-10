/// The provider implementation module remains private to this crate.
///
/// ```compile_fail
/// use geam_macro_cross_crate_consumer::main;
/// ```
#[geam_macros::provider(
    package = "macro_consumer",
    modules = [main],
    crate_path = geam_core,
)]
pub struct Component;

#[cfg(test)]
#[path = "../../../../../../tests/support/execution_host.rs"]
mod execution_fixture;

#[geam_macros::module(path = "macro_consumer/main", crate_path = geam_core)]
mod main {
    use ecow::EcoString;
    use geam_core::provider::{Call, Callback};
    use geam_macro_cross_crate_declarations::values;
    use num_bigint::BigInt;
    use std::future::poll_fn;
    use std::task::Poll;

    #[geam_macros::custom(input = SavedEnvelopeInput)]
    #[derive(Clone)]
    enum SavedEnvelope {
        SavedOne(values::SavedStatus),
        SavedMany(Vec<values::SavedStatus>),
    }

    #[geam_macros::function]
    fn saved(value: EcoString) -> SavedEnvelope {
        SavedEnvelope::SavedOne(values::SavedStatus::Saved(values::SavedText::new(value)))
    }

    #[geam_macros::function]
    fn saved_text(value: SavedEnvelopeInput) -> EcoString {
        match value {
            SavedEnvelopeInput::SavedOne(values::SavedStatusInput::Saved(value)) => value.text(),
            SavedEnvelopeInput::SavedOne(values::SavedStatusInput::Empty) => "empty".into(),
            SavedEnvelopeInput::SavedMany(values) => format!("many:{}", values.len()).into(),
        }
    }

    #[geam_macros::function]
    fn read_saved(value: &values::SavedText) -> EcoString {
        value.text()
    }

    #[geam_macros::function]
    async fn saved_after(value: EcoString) -> SavedEnvelope {
        pending_once().await;
        let saved = values::SavedStatus::Saved(values::SavedText::new(value));
        SavedEnvelope::SavedMany(vec![values::SavedStatus::Empty, saved.clone(), saved])
    }

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
    fn one(value: BigInt) -> Envelope {
        Envelope::One(values::Status::Count(value))
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

    #[geam_macros::function]
    async fn describe_async(value: values::StatusInput) -> EcoString {
        pending_once().await;
        match value {
            values::StatusInput::Ready => "ready".into(),
            values::StatusInput::Count(value) => format!("count:{value}").into(),
            values::StatusInput::Tagged(value) => {
                format!("tagged:{}", value.with(|token| token.0.clone())).into()
            }
        }
    }

    #[geam_macros::function]
    async fn rich(
        value: BigInt,
    ) -> (
        values::Status,
        Result<BigInt, EcoString>,
        Option<values::Token>,
        Vec<BigInt>,
    ) {
        pending_once().await;
        (
            values::Status::Count(value.clone()),
            Ok(value.clone() + 1),
            Some(values::Token(format!("token-{value}").into())),
            vec![value.clone(), value + 1],
        )
    }

    #[geam_macros::function]
    async fn invoke_twice(
        #[geam_macros::call] call: &mut Call<()>,
        callback: Callback<fn(values::Status) -> BigInt>,
        value: BigInt,
    ) -> geam_core::provider::HostResult<(BigInt, BigInt)> {
        let first = call
            .invoke(&callback, (values::Status::Count(value),))
            .await?;
        pending_once().await;
        let second = call
            .invoke(
                &callback,
                (values::Status::Tagged(values::Token("callback".into())),),
            )
            .await?;
        Ok((first, second))
    }

    async fn pending_once() {
        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use geam_builtin::FutureComponent;
    use geam_core::host::{HostFutureStore, HostWorkProfile};
    use geam_core::{
        HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
        HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, Value, compile_typed_host_program, plan_host_program,
    };
    use geam_macro_cross_crate_declarations::Component as DeclarationsComponent;

    struct Profile;

    #[derive(Default)]
    struct ProfileStores {
        future: HostFutureStore,
        declarations: <DeclarationsComponent as HostProviderComponent>::Stores,
        consumer: <Component as HostProviderComponent>::Stores,
    }

    struct RunState {
        future: (),
        declarations: <DeclarationsComponent as HostProviderComponent>::RunState,
        consumer: <Component as HostProviderComponent>::RunState,
    }

    impl HostProfile for Profile {
        type RunState = RunState;
        type ExternalStores = ProfileStores;
    }

    impl HostWorkProfile for Profile {
        type Work = FutureComponent;
    }

    impl HostComponentProfile<FutureComponent> for Profile {
        fn component_stores(stores: &ProfileStores) -> &HostFutureStore {
            &stores.future
        }

        fn component_state(state: &mut RunState) -> &mut () {
            &mut state.future
        }
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
@external(erlang, "macro_declarations", "SavedText")
pub type SavedText

pub type SavedStatus {
  Empty
  Saved(SavedText)
}

@external(erlang, "macro_declarations", "Token")
pub type Token

pub type Status {
  Ready
  Count(Int)
  Tagged(Token)
}
"#;

    const CONSUMER: &str = r#"
import geam/future.{type Future}
import gleam/option.{type Option}
import macro_declarations/values

pub type SavedEnvelope {
  SavedOne(values.SavedStatus)
  SavedMany(List(values.SavedStatus))
}

@external(erlang, "macro_consumer", "saved")
fn saved(value: String) -> SavedEnvelope
@external(erlang, "macro_consumer", "saved_text")
fn saved_text(value: SavedEnvelope) -> String
@external(erlang, "macro_consumer", "read_saved")
fn read_saved(value: values.SavedText) -> String

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
@external(erlang, "macro_consumer", "one")
fn one(value: Int) -> Envelope
@external(erlang, "macro_consumer", "wrapped_token")
fn wrapped_token(value: String) -> Envelope
@external(erlang, "macro_consumer", "envelope_text")
fn envelope_text(value: Envelope) -> String
@external(erlang, "macro_consumer", "first")
fn first(values: List(values.Status)) -> String

@external(erlang, "macro_consumer", "saved_after")
fn saved_after(value: String) -> Future(SavedEnvelope)
@external(erlang, "macro_consumer", "describe_async")
fn describe_async(value: values.Status) -> Future(String)
@external(erlang, "macro_consumer", "rich")
fn rich(value: Int) -> Future(#(values.Status, Result(Int, String), Option(values.Token), List(Int)))
@external(erlang, "macro_consumer", "invoke_twice")
fn invoke_twice(callback: fn(values.Status) -> Int, value: Int) -> Future(#(Int, Int))

pub fn main() {
  assert saved_text(saved("local")) == "local"
  assert saved_text(SavedOne(values.Empty)) == "empty"
  assert saved_text(SavedMany([])) == "many:0"
  let assert SavedOne(values.Saved(text)) = saved("payload")
  assert read_saved(text) == "payload"
  assert status_text(ready()) == "ready"
  assert status_text(count(7)) == "count:7"
  assert token_text(token("blue")) == "blue"
  assert envelope_text(many(8)) == "many:2:count:8"
  assert envelope_text(one(10)) == "one:count:10"
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
        providers.extend(FutureComponent::providers().expect("work provider"));
        let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
            .expect("cross-crate provider modules should be unique");
        let typed = compile_typed_host_program(
            "macro_consumer",
            "macro_consumer/main",
            [
                PackageSource::new(
                    "geam",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "geam/future",
                        "src/geam/future.gleam",
                        include_str!("../../../../../../builtins/geam/gleam/src/geam/future.gleam"),
                    )],
                ),
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "src/gleam/option.gleam",
                        "pub type Option(value) { Some(value) None }",
                    )],
                ),
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
                    ["macro_declarations", "geam", "gleam_stdlib"],
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
        let mut execution = HostedExecution::try_from_module_plan(plan).expect("plan should seal");

        assert_eq!(
            crate::execution_fixture::run(
                &mut execution,
                &mut RunState {
                    future: (),
                    declarations: (),
                    consumer: (),
                },
                &mut Vec::new()
            ),
            Ok(Value::Bool(true)),
        );
    }
}
