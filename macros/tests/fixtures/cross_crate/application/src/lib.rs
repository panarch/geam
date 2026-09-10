use geam_builtin::FutureComponent;
#[cfg(test)]
#[path = "../../../../../../tests/support/execution_host.rs"]
mod execution_fixture;
use geam_core::frontend::{HostedTypedProgram, compile_typed_host_program};
use geam_core::host::{HostFutureStore, HostProviderComponentRegistration, HostProviderSet};
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, ModuleSource, PackageSource,
};
use geam_macro_cross_crate_consumer::Component as ConsumerComponent;
use geam_macro_cross_crate_declarations::Component as DeclarationsComponent;

pub struct Profile;

#[derive(Default)]
pub struct Stores {
    declarations: <DeclarationsComponent as HostProviderComponent>::Stores,
    consumer: <ConsumerComponent as HostProviderComponent>::Stores,
    future: HostFutureStore,
}

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = Stores;
}

impl HostComponentProfile<DeclarationsComponent> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<DeclarationsComponent as HostProviderComponent>::Stores {
        &stores.declarations
    }

    fn component_state(state: &mut Self::RunState) -> &mut () {
        state
    }
}

impl HostComponentProfile<ConsumerComponent> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<ConsumerComponent as HostProviderComponent>::Stores {
        &stores.consumer
    }

    fn component_state(state: &mut Self::RunState) -> &mut () {
        state
    }
}

impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &Stores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
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
pub fn saved(value: String) -> SavedEnvelope
@external(erlang, "macro_consumer", "saved_text")
pub fn saved_text(value: SavedEnvelope) -> String
@external(erlang, "macro_consumer", "read_saved")
pub fn read_saved(value: values.SavedText) -> String
@external(erlang, "macro_consumer", "saved_after")
pub fn saved_after(value: String) -> Future(SavedEnvelope)

pub type Envelope {
  One(values.Status)
  Many(List(values.Status))
  Token(values.Token)
}

@external(erlang, "macro_consumer", "ready")
pub fn ready() -> values.Status

@external(erlang, "macro_consumer", "count")
pub fn count(value: Int) -> values.Status

@external(erlang, "macro_consumer", "token")
pub fn token(value: String) -> values.Token

@external(erlang, "macro_consumer", "status_text")
pub fn status_text(value: values.Status) -> String

@external(erlang, "macro_consumer", "token_text")
pub fn token_text(value: values.Token) -> String

@external(erlang, "macro_consumer", "many")
pub fn many(value: Int) -> Envelope

@external(erlang, "macro_consumer", "one")
pub fn one(value: Int) -> Envelope

@external(erlang, "macro_consumer", "wrapped_token")
pub fn wrapped_token(value: String) -> Envelope

@external(erlang, "macro_consumer", "envelope_text")
pub fn envelope_text(value: Envelope) -> String

@external(erlang, "macro_consumer", "first")
pub fn first(values: List(values.Status)) -> String

@external(erlang, "macro_consumer", "describe_async")
pub fn describe_async(value: values.Status) -> Future(String)

@external(erlang, "macro_consumer", "rich")
pub fn rich(
  value: Int,
) -> Future(#(values.Status, Result(Int, String), Option(values.Token), List(Int)))

@external(erlang, "macro_consumer", "invoke_twice")
pub fn invoke_twice(
  callback: fn(values.Status) -> Int,
  value: Int,
) -> Future(#(Int, Int))
"#;

const APPLICATION: &str = r#"
import geam/future
import gleam/option.{None, Some}
import macro_consumer/main
import macro_declarations/values

fn score(status: values.Status) -> Int {
  case status {
    values.Ready -> 0
    values.Count(value) -> value + 1
    values.Tagged(_) -> 99
  }
}

pub fn direct(value: Int) -> String {
  main.envelope_text(main.one(value))
}

pub fn saved(value: String) {
  let direct = main.saved_text(main.saved(value))
  assert main.saved_text(main.SavedOne(values.Empty)) == "empty"
  assert main.saved_text(main.SavedMany([])) == "many:0"
  use envelope <- future.map(main.saved_after(value))
  let assert main.SavedMany([values.Empty, values.Saved(first), values.Saved(second)]) = envelope
  assert first == second
  #(direct, main.read_saved(first), main.read_saved(second))
}

pub fn run(value: Int) {
  use #(status, result, token, numbers) <- future.then(main.rich(value))
  use description <- future.then(main.describe_async(status))
  let token_text = case token {
    Some(token) -> main.token_text(token)
    None -> "none"
  }
  use #(first, second) <- future.map(main.invoke_twice(score, value))
  #(description, result, token_text, numbers, first, second)
}
"#;

const OPTION: &str = "pub type Option(value) { Some(value) None }";

pub fn program() -> HostedTypedProgram<Profile> {
    let mut providers = FutureComponent::providers().expect("Future component");
    providers.extend(
        <DeclarationsComponent as HostProviderComponentRegistration<Profile>>::providers()
            .expect("cross-crate declaration provider should register"),
    );
    providers.extend(
        <ConsumerComponent as HostProviderComponentRegistration<Profile>>::providers()
            .expect("cross-crate async provider should register"),
    );
    let providers = HostProviderSet::from_providers(providers)
        .expect("cross-crate async provider modules should be unique");
    compile_typed_host_program(
        "application",
        "application",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../../../../../../builtins/geam/gleam/src/geam/future.gleam"),
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
                ["gleam_stdlib", "macro_declarations", "geam"],
                [ModuleSource::new(
                    "macro_consumer/main",
                    "src/main.gleam",
                    CONSUMER,
                )],
            ),
            PackageSource::new(
                "application",
                [
                    "gleam_stdlib",
                    "macro_consumer",
                    "macro_declarations",
                    "geam",
                ],
                [ModuleSource::new(
                    "application",
                    "src/application.gleam",
                    APPLICATION,
                )],
            ),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    OPTION,
                )],
            ),
        ],
        providers,
    )
    .expect("cross-crate async provider source should compile")
}

#[cfg(test)]
mod tests {
    use super::program;
    use ecow::EcoString;
    use geam_builtin::embedding::FutureType;
    use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder, List};
    use geam_core::{EchoOutput, EchoSink};
    use num_bigint::BigInt;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    #[derive(Default)]
    struct Echo(usize);

    impl EchoSink for Echo {
        fn emit(&mut self, _output: EchoOutput) {
            self.0 += 1;
        }
    }

    #[test]
    fn public_typed_embedding_runs_a_separately_compiled_async_provider() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let (mut bindings, direct) = HostedModuleBuilder::new(program())
            .expect("cross-crate provider plan")
            .function(FunctionDeclaration::<(BigInt,), EcoString>::new("direct"))
            .expect("direct cross-crate binding");
        let run = bindings
            .function(FunctionDeclaration::<
                (BigInt,),
                FutureType<(
                    EcoString,
                    Result<BigInt, EcoString>,
                    EcoString,
                    List<BigInt>,
                    BigInt,
                    BigInt,
                )>,
            >::new("run"))
            .expect("async cross-crate binding");
        let mut module = bindings.seal().expect("cross-crate bindings should seal");
        let mut state = ();
        let mut echo = Echo::default();

        let result = {
            let mut future = pin!(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(
                        scope
                            .call(&direct, (BigInt::from(5),))
                            .await
                            .expect("direct entry"),
                        EcoString::from("one:count:5")
                    );
                    let work = scope
                        .call(&run, (BigInt::from(7),))
                        .await
                        .expect("construct source work");
                    scope.observe(&work).await
                }
            ));
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
            execution_host
                .block_on(future)
                .expect("controlled execution")
                .expect("cross-crate async call should complete")
        };

        result.read(|(description, result, token, numbers, first, second)| {
            assert_eq!(description, "count:7");
            assert_eq!(result, Ok(&BigInt::from(8)));
            assert_eq!(token, "token-7");
            assert_eq!(numbers.len(), 2);
            assert_eq!(numbers.read_item(0, Clone::clone), Some(BigInt::from(7)));
            assert_eq!(numbers.read_item(1, Clone::clone), Some(BigInt::from(8)));
            assert_eq!(first, &BigInt::from(8));
            assert_eq!(second, &BigInt::from(99));
        });
        assert_eq!(echo.0, 0);
    }

    #[test]
    fn foreign_custom_fields_select_transferable_payloads_before_construction() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let (bindings, saved) = HostedModuleBuilder::new(program())
            .expect("cross-crate provider plan")
            .function(FunctionDeclaration::<
                (EcoString,),
                FutureType<(EcoString, EcoString, EcoString)>,
            >::new("saved"))
            .expect("custom field binding");
        let mut module = bindings.seal().expect("custom field module");
        let mut state = ();
        let mut echo = Echo::default();
        let result = {
            let mut future = pin!(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    let work = scope
                        .call(&saved, (EcoString::from("shared"),))
                        .await
                        .expect("construct custom work");
                    scope.observe(&work).await
                }
            ));
            let mut context = Context::from_waker(Waker::noop());
            assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
            execution_host
                .block_on(future)
                .expect("controlled execution")
                .expect("custom completion")
        };
        std::thread::spawn(move || {
            result.read(|(direct, first, second)| {
                assert_eq!(direct, "shared");
                assert_eq!(first, "shared");
                assert_eq!(second, "shared");
            })
        })
        .join()
        .expect("transfer completed result");
        assert_eq!(echo.0, 0);
    }
}
