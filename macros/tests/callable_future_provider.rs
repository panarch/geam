#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_builtin::{FutureComponent, embedding::FutureType};
use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam_core::host::{
    HostComponentProfile, HostFutureStore, HostProfile, HostProviderComponentRegistration,
    HostProviderSet, HostWorkProfile,
};
use geam_core::provider::BigInt;
use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
pub struct State {
    body_entries: Arc<AtomicUsize>,
    factory_entries: Arc<AtomicUsize>,
    arguments: Vec<BigInt>,
}

#[geam_macros::provider(package = "native_work", state = State, modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "native_work", crate_path = geam_core)]
mod native {
    use super::{Ordering, State};
    use geam_core::provider::{BigInt, Call, Callback, Factory, Future, HostResult, Value};

    #[geam_macros::custom(input = WorkHolderInput)]
    enum WorkHolder {
        WorkHolder(Callback<fn(BigInt) -> Future<BigInt>>),
    }

    #[geam_macros::custom(input = PendingInput)]
    enum Pending {
        Pending(Future<BigInt>, Vec<(bool, Result<Future<BigInt>, BigInt>)>),
    }

    #[geam_macros::function]
    fn pending(work: Future<BigInt>) -> Pending {
        Pending::Pending(work.clone(), vec![(false, Err(7.into())), (true, Ok(work))])
    }

    #[geam_macros::callable(factory = KeepPending)]
    fn keep_pending(
        #[geam_macros::capture] pending: geam_core::provider::List<PendingInput>,
    ) -> Future<BigInt> {
        let value = pending.get(0).unwrap();
        drop(pending);
        let pending = value;
        let PendingInput::Pending(original, items) = pending;
        assert_eq!(items.len(), 2);
        assert!(!items.get(0).unwrap().0);
        let (selected, work) = items.get(1).unwrap();
        assert!(selected);
        drop(items);
        drop(original);
        work.unwrap()
    }

    #[geam_macros::function]
    fn capture_pending(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<KeepPending>,
        pending: geam_core::provider::List<PendingInput>,
    ) -> HostResult<Callback<fn() -> Future<BigInt>>> {
        call.create(&factory, (pending,))
    }

    #[geam_macros::function]
    fn select_work<Item>(
        works: geam_core::provider::List<Future<Value<Item>>>,
    ) -> Future<Value<Item>> {
        works.get(1).unwrap()
    }

    #[geam_macros::function]
    fn retained_work(
        works: geam_core::provider::List<Future<BigInt>>,
    ) -> geam_core::provider::List<Future<BigInt>> {
        works
    }

    #[geam_macros::function]
    fn nested_work(
        works: geam_core::provider::List<geam_core::provider::List<Future<BigInt>>>,
    ) -> geam_core::provider::List<geam_core::provider::List<Future<BigInt>>> {
        works
    }

    #[geam_macros::function]
    fn fresh_nested_work(
        works: geam_core::provider::List<geam_core::provider::List<Future<BigInt>>>,
    ) -> Vec<Vec<Future<BigInt>>> {
        let original = works.get(1).unwrap().get(0).unwrap();
        drop(works);
        vec![vec![], vec![original.clone(), original]]
    }

    #[geam_macros::function]
    fn work_result(
        work: Result<Future<BigInt>, BigInt>,
    ) -> (Result<Future<BigInt>, BigInt>, Vec<Future<BigInt>>) {
        match work {
            Ok(work) => (Ok(work.clone()), vec![work]),
            Err(error) => (Err(error), vec![]),
        }
    }

    #[geam_macros::function]
    async fn invoke_work_item(
        #[geam_macros::call] call: &mut Call<State>,
        works: geam_core::provider::List<Future<Callback<fn(Vec<BigInt>) -> BigInt>>>,
    ) -> HostResult<BigInt> {
        let work = works.get(1).unwrap();
        drop(works);
        let callback = call.observe(&work).await?;
        call.invoke(&callback, (vec![4.into(), 5.into()],)).await
    }

    #[geam_macros::function]
    fn holder(callback: Callback<fn(BigInt) -> Future<BigInt>>) -> WorkHolder {
        WorkHolder::WorkHolder(callback)
    }

    #[geam_macros::function(await)]
    async fn invoke_holder(
        #[geam_macros::call] call: &mut Call<State>,
        holder: WorkHolderInput,
    ) -> HostResult<Future<BigInt>> {
        let WorkHolderInput::WorkHolder(callback) = holder;
        call.invoke(&callback, (3.into(),)).await
    }

    #[geam_macros::function(await)]
    async fn second_holder(
        #[geam_macros::call] call: &mut Call<State>,
        holders: geam_core::provider::List<(BigInt, WorkHolderInput)>,
    ) -> HostResult<Future<BigInt>> {
        let (label, holder) = holders.get(1).expect("source supplies two holders");
        assert_eq!(label, BigInt::from(7));
        drop(holders);
        invoke_holder(call, holder).await
    }

    #[geam_macros::callable(factory = Ready)]
    async fn ready(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] offset: BigInt,
        argument: BigInt,
    ) -> HostResult<BigInt> {
        let returned = &offset + &argument;
        call.with_state(move |state| {
            state.body_entries.fetch_add(1, Ordering::SeqCst);
            state.arguments.push(argument);
        })
        .await?;
        Ok(returned)
    }

    #[geam_macros::callable(factory = RetainWork)]
    fn retain_work<Item>(#[geam_macros::capture] work: Future<Value<Item>>) -> Future<Value<Item>> {
        work
    }

    #[geam_macros::function]
    fn capture_work<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<RetainWork<Item>>,
        work: Future<Value<Item>>,
    ) -> HostResult<Callback<fn() -> Future<Value<Item>>>> {
        call.create(&factory, (work,))
    }

    #[geam_macros::callable(factory = ForwardWork, await)]
    async fn forward_work(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] callback: Callback<fn(Future<BigInt>) -> Future<BigInt>>,
        work: Future<BigInt>,
    ) -> HostResult<Future<BigInt>> {
        call.invoke(&callback, (work,)).await
    }

    #[geam_macros::function]
    fn wrap_work(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ForwardWork>,
        callback: Callback<fn(Future<BigInt>) -> Future<BigInt>>,
    ) -> HostResult<Callback<fn(Future<BigInt>) -> Future<BigInt>>> {
        call.create(&factory, (callback,))
    }

    #[geam_macros::function]
    fn make(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Ready>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> Future<BigInt>>> {
        call.create(&factory, (offset,))
    }

    #[geam_macros::function]
    async fn make_in_work(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Ready>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> Future<BigInt>>> {
        call.with_state(|state| {
            state.factory_entries.fetch_add(1, Ordering::SeqCst);
        })
        .await?;
        call.create(&factory, (offset,)).await
    }

    #[geam_macros::function]
    async fn invoke_work(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Ready>,
        offset: BigInt,
    ) -> HostResult<BigInt> {
        let callback = call
            .with_call(move |call| call.create(&factory, (offset,)))
            .await??;
        let before = call
            .with_state(|state| state.body_entries.load(Ordering::SeqCst))
            .await?;
        let work = call.invoke(&callback, (6.into(),)).await?;
        assert_eq!(
            call.with_state(|state| state.body_entries.load(Ordering::SeqCst))
                .await?,
            before
        );
        let alias = work.clone();
        let first = call.observe(&work).await?;
        let second = call.observe(&alias).await?;
        Ok(first + second)
    }
}

struct Profile;
#[derive(Default)]
struct HostState {
    native: State,
    future: (),
}
#[derive(Default)]
struct HostStores {
    native: Stores,
    future: HostFutureStore,
}
impl HostProfile for Profile {
    type RunState = HostState;
    type ExternalStores = HostStores;
    type ExecutionState = ();
}
impl HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &HostStores) -> &Stores {
        &stores.native
    }
    fn component_state(state: &mut HostState) -> &mut State {
        &mut state.native
    }
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &HostStores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut HostState) -> &mut () {
        &mut state.future
    }
}

#[test]
fn native_work_results_and_creation_inside_work_keep_explicit_observation() {
    let source = r#"
import geam/future
pub type WorkHolder { WorkHolder(fn(Int) -> future.Future(Int)) }
pub type Pending { Pending(future.Future(Int), List(#(Bool, Result(future.Future(Int), Int)))) }
@external(erlang, "native", "pending")
fn pending(work: future.Future(Int)) -> Pending
@external(erlang, "native", "capture_pending")
fn capture_pending(pending: List(Pending)) -> fn() -> future.Future(Int)
@external(erlang, "native", "select_work")
fn select_work(works: List(future.Future(item))) -> future.Future(item)
@external(erlang, "native", "retained_work")
fn retained_work(works: List(future.Future(Int))) -> List(future.Future(Int))
@external(erlang, "native", "nested_work")
fn nested_work(works: List(List(future.Future(Int)))) -> List(List(future.Future(Int)))
@external(erlang, "native", "fresh_nested_work")
fn fresh_nested_work(works: List(List(future.Future(Int)))) -> List(List(future.Future(Int)))
@external(erlang, "native", "work_result")
fn work_result(work: Result(future.Future(Int), Int)) -> #(Result(future.Future(Int), Int), List(future.Future(Int)))
@external(erlang, "native", "invoke_work_item")
fn invoke_work_item(works: List(future.Future(fn(List(Int)) -> Int))) -> future.Future(Int)
@external(erlang, "native", "holder")
fn holder(callback: fn(Int) -> future.Future(Int)) -> WorkHolder
@external(erlang, "native", "invoke_holder")
fn invoke_holder(holder: WorkHolder) -> future.Future(Int)
@external(erlang, "native", "second_holder")
fn second_holder(holders: List(#(Int, WorkHolder))) -> future.Future(Int)
@external(erlang, "native", "make")
fn make(offset: Int) -> fn(Int) -> future.Future(Int)
@external(erlang, "native", "make_in_work")
fn make_in_work(offset: Int) -> future.Future(fn(Int) -> future.Future(Int))
@external(erlang, "native", "invoke_work")
pub fn invoke_work(offset: Int) -> future.Future(Int)
@external(erlang, "native", "capture_work")
fn capture_work(work: future.Future(item)) -> fn() -> future.Future(item)
@external(erlang, "native", "wrap_work")
fn wrap_work(callback: fn(future.Future(Int)) -> future.Future(Int)) -> fn(future.Future(Int)) -> future.Future(Int)
pub fn immediate() {
  second_holder([#(100, holder(make(100))), #(7, holder(make(7)))])
}
pub fn later() { use callback <- future.then(make_in_work(20)) callback(4) }
pub fn captured() {
  let callback = make(40)
  let original = callback(2)
  let assert [kept] = retained_work([original])
  let assert [[], [nested]] = nested_work([[], [original]])
  assert kept == original && nested == original
  let assert [[], [first, second]] = fresh_nested_work([[], [original]])
  assert first == original && second == original
  let pending = pending(original)
  let pending_keeper = capture_pending([pending])
  assert pending_keeper() == original
  assert pending_keeper() == original
  let assert #(Ok(restored), [listed]) = work_result(Ok(original))
  assert restored == original && listed == original
  assert work_result(Error(7)) == #(Error(7), [])
  let retained = capture_work(original)
  let alias = retained
  assert retained == alias
  assert retained() == original
  assert alias() == original
  let wrapper = wrap_work(fn(work) { work })
  let forwarded = wrapper(retained())
  assert forwarded == original
  forwarded
}
pub fn generic() {
  let text = future.ready("ready")
  let text_keeper = capture_work(text)
  let higher = future.ready(#("function", fn(value: Int) { value + 3 }))
  let higher_keeper = capture_work(higher)
  assert select_work([future.ready("unused"), text_keeper()]) == text
  assert higher_keeper() == higher
  use text <- future.then(text_keeper())
  future.map(higher_keeper(), fn(pair) {
    let #(tag, callback) = pair
    text == "ready" && tag == "function" && callback(4) == 7
  })
}
pub fn rich_work() {
  invoke_work_item([
    future.map(future.ready(Nil), fn(_) { panic as "unused work" }),
    future.ready(fn(values) {
      let assert [left, right] = values
      left + right
    }),
  ])
}
"#;
    let mut providers = FutureComponent::providers::<Profile>().unwrap();
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = compile_typed_host_program(
        "native_work",
        "native_work",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "future.gleam",
                    include_str!("../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "native_work",
                ["geam"],
                [ModuleSource::new(
                    "native_work",
                    "native_work.gleam",
                    source,
                )],
            ),
        ],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (mut builder, immediate) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new(
            "immediate",
        ))
        .unwrap();
    let later = builder
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new("later"))
        .unwrap();
    let invoke = builder
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "invoke_work",
        ))
        .unwrap();
    let captured = builder
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new(
            "captured",
        ))
        .unwrap();
    let generic = builder
        .function(FunctionDeclaration::<(), FutureType<bool>>::new("generic"))
        .unwrap();
    let rich_work = builder
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new(
            "rich_work",
        ))
        .unwrap();
    let mut module = builder.seal().unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = HostState::default();
    let bodies = Arc::clone(&state.native.body_entries);
    let factories = Arc::clone(&state.native.factory_entries);
    host.block_on(
        module.with_execution(&host, &mut state, &mut |_| {}, async |scope| {
            let first = scope.call(&immediate, ()).await.unwrap();
            assert_eq!(bodies.load(Ordering::SeqCst), 0);
            let alias = first.clone();
            assert_eq!(
                scope.observe(&first).await.unwrap().read(Clone::clone),
                BigInt::from(10)
            );
            assert_eq!(
                scope.observe(&alias).await.unwrap().read(Clone::clone),
                BigInt::from(10)
            );
            assert_eq!(bodies.load(Ordering::SeqCst), 1);
            let second = scope.call(&later, ()).await.unwrap();
            assert_eq!(factories.load(Ordering::SeqCst), 0);
            assert_eq!(
                scope.observe(&second).await.unwrap().read(Clone::clone),
                BigInt::from(24)
            );
            assert_eq!(factories.load(Ordering::SeqCst), 1);
            let invoked = scope.call(&invoke, (5.into(),)).await.unwrap();
            assert_eq!(bodies.load(Ordering::SeqCst), 2);
            assert_eq!(
                scope.observe(&invoked).await.unwrap().read(Clone::clone),
                BigInt::from(22)
            );
            assert_eq!(bodies.load(Ordering::SeqCst), 3);
            let kept = scope.call(&captured, ()).await.unwrap();
            assert_eq!(bodies.load(Ordering::SeqCst), 3);
            let kept_alias = kept.clone();
            assert_eq!(
                scope.observe(&kept).await.unwrap().read(Clone::clone),
                BigInt::from(42)
            );
            assert_eq!(
                scope.observe(&kept_alias).await.unwrap().read(Clone::clone),
                BigInt::from(42)
            );
            assert_eq!(bodies.load(Ordering::SeqCst), 4);
            let rich = scope.call(&rich_work, ()).await.unwrap();
            assert_eq!(
                scope.observe(&rich).await.unwrap().read(Clone::clone),
                BigInt::from(9)
            );
            let generic = scope.call(&generic, ()).await.unwrap();
            assert!(
                scope
                    .observe(&generic)
                    .await
                    .unwrap()
                    .read(std::convert::identity)
            );
            assert_eq!(bodies.load(Ordering::SeqCst), 4);
        }),
    )
    .unwrap();
    assert_eq!(
        state.native.arguments,
        [3.into(), 4.into(), 6.into(), 2.into()]
    );
}
