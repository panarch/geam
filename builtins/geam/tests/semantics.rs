use ecow::EcoString;
#[path = "../../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_builtin::embedding::FutureType;
use geam_builtin::{FutureComponent, HostFutureSchema, HostFutureStorage, HostFutureType};
use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam_core::frontend::compile_typed_host_program;
use geam_core::host::{
    HostCall, HostComponentProfile, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
};
use geam_core::host::{HostFuturePayload, HostFutureStore, HostWorkProfile};
use geam_core::provider::ProviderOwnedExternal;
use geam_core::{HostCallCompletion, HostCallError, HostExternal, ModuleSource, PackageSource};
use num_bigint::BigInt;

struct Profile;
struct Observer;

#[derive(Default)]
struct State {
    work: (),
    observations: Vec<(u64, EcoString)>,
    retained: Option<ProviderOwnedExternal<HostFuturePayload>>,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = HostFutureStore;
}

impl HostWorkProfile for Profile {
    type Work = FutureComponent;
}

impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
        stores
    }

    fn component_state(state: &mut State) -> &mut () {
        &mut state.work
    }
}

impl HostProvider<Profile> for Observer {
    type State = State;

    fn project(state: &mut State) -> &mut State {
        state
    }
}

impl geam_core::host::HostExternalBinding<Profile, HostFutureSchema> for Observer {
    type Storage = HostFutureStorage;
}

fn snapshot<'call>(
    mut call: HostCall<'call, Profile, Observer, ()>,
    value: HostExternal<'call, HostFutureType<BigInt>>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    let work = call.provider_external_item_with::<Observer, HostFutureSchema, _>(value);
    let hash = call.source_hash::<HostFutureType<BigInt>>(value);
    let inspection = call.inspect::<HostFutureType<BigInt>>(value);
    call.state().retained = Some(work);
    call.state().observations.push((hash, inspection));
    Ok(call.return_value(()))
}

fn recall(
    mut call: HostCall<'_, Profile, Observer, HostFutureType<BigInt>>,
) -> Result<HostCallCompletion<'_, HostFutureType<BigInt>>, HostCallError> {
    let work = call.state().retained.take().expect("observed work");
    let value = call.provider_external_from_item::<HostFutureSchema, geam_core::HostTypeList<BigInt, geam_core::HostTypeListEnd>, _>(work);
    Ok(call.return_value(value))
}

#[test]
fn source_hash_and_inspection_survive_completion_and_scope_cancellation() {
    let execution_host = crate::execution_fixture::TestHost::default();

    let mut providers = FutureComponent::providers::<Profile>().expect("Future registration");
    providers.push(
        HostProviderModule::new("application", "library")
            .expect("observer module")
            .with_scoped_function::<Observer, (HostFutureType<BigInt>,), (), _>(
                "snapshot", snapshot,
            )
            .expect("snapshot registration")
            .with_scoped_function::<Observer, (), HostFutureType<BigInt>, _>("recall", recall)
            .expect("retained work registration"),
    );
    let program = compile_typed_host_program(
        "application",
        "library",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "application",
                ["geam"],
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
import geam/future
@external(erlang, "observer", "snapshot")
fn snapshot(work: future.Future(Int)) -> Nil
@external(erlang, "observer", "recall")
fn recall() -> future.Future(Int)
pub fn work() {
  let work = future.map(future.ready(21), fn(value) { echo value value * 2 })
  snapshot(work)
  work
}
pub fn check() { let work = recall() snapshot(work) work }
"#,
                )],
            ),
        ],
        HostProviderSet::from_providers(providers).expect("providers"),
    )
    .expect("ordinary source");
    let (mut bindings, work) = HostedModuleBuilder::new(program)
        .expect("plan")
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new("work"))
        .expect("work entry");
    let check = bindings
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new("check"))
        .expect("check entry");
    let mut module = bindings.seal().expect("sealed module");
    for complete in [true, false] {
        let mut state = State::default();
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<FutureComponent>>::component_state(&mut state),
            &state.work,
        ));
        let mut outputs = Vec::new();
        let mut echo = |output: geam_core::EchoOutput| outputs.push(output.to_string());
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    let work = scope.call(&work, ()).await.expect("created work");
                    if complete {
                        scope
                            .observe(&work)
                            .await
                            .expect("completion")
                            .read(|value| assert_eq!(value, &BigInt::from(42)));
                    }
                },
            ))
            .expect("caller drives ready dependencies");
        assert_eq!(state.observations.len(), 1);
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    let original = scope
                        .call(&check, ())
                        .await
                        .expect("source semantics after scope exit");
                    match scope.observe(&original).await {
                        Ok(result) => {
                            assert!(complete);
                            result.read(|value| assert_eq!(value, &BigInt::from(42)));
                        }
                        Err(geam_core::embedding::ObservationError::Cancelled) => {
                            assert!(!complete)
                        }
                        Err(error) => panic!("unexpected observation: {error}"),
                    }
                },
            ))
            .expect("no work is restarted");
        assert_eq!(state.observations.len(), 2);
        let (before_hash, before_inspection) = &state.observations[0];
        let (after_hash, after_inspection) = &state.observations[1];
        assert_eq!(before_hash, after_hash);
        assert_eq!(before_inspection, "Future(...)");
        assert_eq!(after_inspection, before_inspection);
        assert_eq!(
            outputs,
            if complete {
                vec!["src/library.gleam:8\n21"]
            } else {
                vec![]
            }
        );
    }
}
