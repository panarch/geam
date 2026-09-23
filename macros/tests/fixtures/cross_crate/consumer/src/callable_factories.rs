use geam_builtin::FutureComponent;
use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam_core::host::{HostFutureStore, HostWorkProfile};
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
};
use geam_macro_cross_crate_declarations::Component as Declarations;
use geam_macro_cross_crate_declarations::work_values::Component as WorkDeclarations;
use num_bigint::BigInt;

trait FactoryProfile:
    HostWorkProfile + HostComponentProfile<Declarations> + HostComponentProfile<WorkDeclarations>
{
}
impl<Profile> FactoryProfile for Profile where
    Profile: HostWorkProfile
        + HostComponentProfile<Declarations>
        + HostComponentProfile<WorkDeclarations>
{
}

struct Component;
impl HostProviderComponent for Component {
    const ID: &'static str = "factory-consumer";
    type Stores = ();
    type RunState = ();
}
impl geam_core::__macro_support::ProviderPackage for Component {
    const PACKAGE: &'static str = "factory_consumer";
}

#[geam_macros::module(
    path = "factory_consumer",
    crate_path = geam_core,
    profile = super::FactoryProfile,
    component = super::Component,
)]
mod consumer {
    use geam_core::StringValue;
    use geam_core::provider::{BigInt, Call, Callback, Factory, Future, HostResult};
    use geam_macro_cross_crate_declarations::values;
    use geam_macro_cross_crate_declarations::work_values::values as work;

    #[geam_macros::custom(input = BatchInput)]
    enum Batch {
        Batch(Vec<(BigInt, work::Holder)>),
    }

    #[geam_macros::function]
    fn pack(callback: Callback<fn(BigInt) -> Future<BigInt>>) -> Batch {
        Batch::Batch(vec![(3.into(), work::Holder::Run(callback))])
    }

    #[geam_macros::function]
    fn pack_work(work: Future<BigInt>) -> work::Pending {
        work::Pending::Pending(work.clone(), vec![work.clone(), work])
    }

    #[geam_macros::function]
    fn unpack_work(pending: geam_core::provider::List<work::Pending>) -> Future<BigInt> {
        let work::PendingInput::Pending(original, items) = pending.get(0).unwrap();
        drop(pending);
        drop(original);
        let selected = items.get(1).unwrap();
        drop(items);
        selected
    }

    #[geam_macros::function]
    fn batch_len(batch: BatchInput) -> BigInt {
        let BatchInput::Batch(holders) = batch;
        holders.len().into()
    }

    #[geam_macros::function(await)]
    async fn run_batch(
        #[geam_macros::call] call: &mut Call<()>,
        batch: BatchInput,
    ) -> HostResult<Future<BigInt>> {
        let BatchInput::Batch(holders) = batch;
        let (argument, work::HolderInput::Run(callback)) =
            holders.get(0).expect("pack supplies one holder");
        drop(holders);
        call.invoke(&callback, (argument,)).await
    }

    #[geam_macros::function]
    fn add(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<values::AddOffset>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
        call.create(&factory, (offset,))
    }

    #[geam_macros::function]
    fn delay(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<values::DelayedOffset>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> Future<BigInt>>> {
        call.create(&factory, (offset,))
    }

    #[geam_macros::function(await)]
    async fn delay_owned(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<values::DelayedOffset>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> Future<BigInt>>> {
        call.create(&factory, (offset,)).await
    }

    #[geam_macros::function]
    fn status(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<values::StatusText>,
        value: BigInt,
    ) -> HostResult<Callback<fn() -> StringValue>> {
        call.create(&factory, (values::Status::Count(value),))
    }
}

struct Profile;
#[derive(Default)]
struct Stores {
    declarations: <Declarations as HostProviderComponent>::Stores,
    work: <WorkDeclarations as HostProviderComponent>::Stores,
    future: HostFutureStore,
}
impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = Stores;
    type ExecutionState = ();
}
impl HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl HostComponentProfile<Declarations> for Profile {
    fn component_stores(stores: &Stores) -> &<Declarations as HostProviderComponent>::Stores {
        &stores.declarations
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(_: &Stores) -> &() {
        &()
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}
impl HostComponentProfile<WorkDeclarations> for Profile {
    fn component_stores(stores: &Stores) -> &<WorkDeclarations as HostProviderComponent>::Stores {
        &stores.work
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}
impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &Stores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn qualified_factory_declarations_keep_their_original_codecs_across_crates() {
    let source = r#"
import geam/future
import macro_work/values.{type Holder, type Pending}
pub type Batch { Batch(List(#(Int, Holder))) }
@external(erlang, "native", "pack")
fn pack(callback: fn(Int) -> future.Future(Int)) -> Batch
@external(erlang, "native", "pack_work")
fn pack_work(work: future.Future(Int)) -> Pending
@external(erlang, "native", "unpack_work")
fn unpack_work(pending: List(Pending)) -> future.Future(Int)
@external(erlang, "native", "batch_len")
fn batch_len(batch: Batch) -> Int
@external(erlang, "native", "run_batch")
fn run_batch(batch: Batch) -> future.Future(Int)
@external(erlang, "native", "add")
fn add(offset: Int) -> fn(Int) -> Int
@external(erlang, "native", "delay")
fn delay(offset: Int) -> fn(Int) -> future.Future(Int)
@external(erlang, "native", "delay_owned")
fn delay_owned(offset: Int) -> fn(Int) -> future.Future(Int)
@external(erlang, "native", "status")
fn status(value: Int) -> fn() -> String
pub fn main() {
  let increment = add(3)
  let read_status = status(27)
  assert increment(4) == 7
  assert read_status() == "count:27"
  let delayed = delay(10)
  let delayed_owned = delay_owned(20)
  let work = delayed_owned(7)
  assert unpack_work([pack_work(work)]) == work
  use first <- future.then(delayed(5))
  use second <- future.then(work)
  use third <- future.then(work)
  let batch = pack(delayed)
  assert batch_len(batch) == 1
  use fourth <- future.map(run_batch(batch))
  first + second + third + fourth
}
"#;
    let mut providers =
        <Declarations as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    providers.push(consumer::__geam_module::<Profile>().unwrap());
    providers.extend(
        <WorkDeclarations as HostProviderComponentRegistration<Profile>>::providers().unwrap(),
    );
    providers.extend(FutureComponent::providers::<Profile>().unwrap());
    let typed = compile_typed_host_program(
        "factory_consumer",
        "factory_consumer",
        [
            PackageSource::new(
                "geam",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "future.gleam",
                    include_str!("../../../../../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "macro_declarations",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "macro_declarations/values",
                    "values.gleam",
                    r#"
pub type SavedText
pub type SavedStatus { Empty Saved(SavedText) }
pub type Token
pub type Status { Ready Count(Int) Tagged(Token) }
"#,
                )],
            ),
            PackageSource::new(
                "macro_work",
                ["geam"],
                [ModuleSource::new(
                    "macro_work/values",
                    "work_values.gleam",
                    "import geam/future\npub type Holder { Run(fn(Int) -> future.Future(Int)) }\npub type Pending { Pending(future.Future(Int), List(future.Future(Int))) }",
                )],
            ),
            PackageSource::new(
                "factory_consumer",
                ["macro_declarations", "macro_work", "geam"],
                [ModuleSource::new(
                    "factory_consumer",
                    "factory_consumer.gleam",
                    source,
                )],
            ),
        ],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (builder, entry) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<
            (),
            geam_builtin::embedding::FutureType<BigInt>,
        >::new("main"))
        .unwrap();
    let mut module = builder.seal().unwrap();
    let host = crate::execution_fixture::TestHost::default();
    let mut state = ();
    host.block_on(
        module.with_execution(&host, &mut state, &mut |_| {}, async |scope| {
            let work = scope.call(&entry, ()).await.unwrap();
            assert_eq!(
                scope.observe(&work).await.unwrap().read(Clone::clone),
                BigInt::from(82)
            );
        }),
    )
    .unwrap();
}
