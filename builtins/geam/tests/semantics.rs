use ecow::EcoString;
use futures_util::FutureExt;
use geam_core::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
use geam_core::frontend::compile_typed_transfer_host_program;
use geam_core::host::{
    AsyncHostComponentProfile, HostProfile, HostProvider, TransferHostCall,
    TransferHostProviderModule, TransferHostProviderSet,
};
use geam_core::host::{HostFuturePayload, HostFutureStore, HostWorkProfile};
use geam_core::provider::ProviderTransferExternalItem;
use geam_core::{
    AsyncHostCallError, HostCallCompletion, HostExternal, ModuleSource, PackageSource,
};
use geam_runtime_api::embedding::FutureType;
use geam_runtime_api::{FutureComponent, HostFutureSchema, HostFutureStorage, HostFutureType};
use num_bigint::BigInt;

struct Profile;
struct Observer;

#[derive(Default)]
struct State {
    work: (),
    observations: Vec<(u64, EcoString)>,
    retained: Option<ProviderTransferExternalItem<HostFuturePayload>>,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = HostFutureStore;
}

impl HostWorkProfile for Profile {
    type Work = FutureComponent;
}

impl AsyncHostComponentProfile<FutureComponent> for Profile {
    fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
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

impl geam_core::host::AsyncHostExternalBinding<Profile, HostFutureSchema> for Observer {
    type Storage = HostFutureStorage;
}

fn snapshot<'call>(
    mut call: TransferHostCall<'call, Profile, Observer, ()>,
    value: HostExternal<'call, HostFutureType<BigInt>>,
) -> Result<HostCallCompletion<'call, ()>, AsyncHostCallError> {
    let work = call.provider_transfer_external_item_with::<Observer, HostFutureSchema, _>(value);
    let hash = call.source_hash::<HostFutureType<BigInt>>(value);
    let inspection = call.inspect::<HostFutureType<BigInt>>(value);
    call.state().retained = Some(work);
    call.state().observations.push((hash, inspection));
    Ok(call.return_value(()))
}

fn recall(
    mut call: TransferHostCall<'_, Profile, Observer, HostFutureType<BigInt>>,
) -> Result<HostCallCompletion<'_, HostFutureType<BigInt>>, AsyncHostCallError> {
    let work = call.state().retained.take().expect("observed work");
    let value = call.provider_transfer_external_from_item::<HostFutureSchema, geam_core::HostTypeList<BigInt, geam_core::HostTypeListEnd>, _>(work);
    Ok(call.return_value(value))
}

#[test]
fn source_hash_and_inspection_survive_completion_and_scope_cancellation() {
    let mut providers = FutureComponent::providers::<Profile>().expect("Future registration");
    providers.push(
        TransferHostProviderModule::new_for_profile("application", "library")
            .expect("observer module")
            .with_scoped_function::<Observer, (HostFutureType<BigInt>,), (), _>(
                "snapshot", snapshot,
            )
            .expect("snapshot registration")
            .with_scoped_function::<Observer, (), HostFutureType<BigInt>, _>("recall", recall)
            .expect("retained work registration"),
    );
    let program = compile_typed_transfer_host_program(
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
        TransferHostProviderSet::new(providers).expect("providers"),
    )
    .expect("ordinary source");
    let (mut bindings, work) = WorkModuleBuilder::new(program)
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
            <Profile as AsyncHostComponentProfile<FutureComponent>>::component_state(&mut state),
            &state.work,
        ));
        let mut outputs = Vec::new();
        let mut echo = |output: geam_core::EchoOutput| outputs.push(output.to_string());
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let work = scope.call(&work, ()).expect("created work");
            if complete {
                scope
                    .observe(&work)
                    .await
                    .expect("completion")
                    .read(|value| assert_eq!(value, &BigInt::from(42)));
            }
        })
        .now_or_never()
        .expect("caller drives ready dependencies");
        assert_eq!(state.observations.len(), 1);
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let original = scope
                .call(&check, ())
                .expect("source semantics after scope exit");
            match scope.observe(&original).await {
                Ok(result) => {
                    assert!(complete);
                    result.read(|value| assert_eq!(value, &BigInt::from(42)));
                }
                Err(geam_core::embedding::ObservationError::Cancelled) => assert!(!complete),
                Err(error) => panic!("unexpected observation: {error}"),
            }
        })
        .now_or_never()
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
