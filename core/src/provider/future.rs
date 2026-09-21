mod list;

pub use list::ProviderFutureListDecoder;

use super::{ProviderConstructionRequirements, ProviderConstructions};
use crate::host::{
    HostCall, HostExecutionError, HostExternal, HostFutureType, HostFutureValue, HostProfile,
    HostProvider, HostType, HostWorkProfile,
};
use std::marker::PhantomData;

/// A Gleam Future received by native provider code.
///
/// Receiving or cloning it does not start the operation. An async provider uses
/// its [`super::Call::observe`] capability to explicitly observe the shared work.
pub struct Future<Value, Context = MissingFutureContext> {
    context: Context,
    value: PhantomData<fn() -> Value>,
}

#[doc(hidden)]
pub struct MissingFutureContext;

// The decoder keeps its original provider projection; transporting a Future
// between providers must not change its Rust type or its execution endpoint.
struct FutureProvider;

impl<Profile: HostProfile> HostProvider<Profile> for FutureProvider {
    type State = Profile::RunState;
    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

type Decode<Profile, Host, Output> = for<'call> fn(
    HostCall<'call, Profile, FutureProvider, ()>,
    <Host as HostType>::Value<'call>,
    usize,
) -> Output;

/// Statically selected conversion for the result of one source Future.
#[doc(hidden)]
pub trait ProviderFutureCodec<Profile, Provider>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    type Host: HostType;
    type Output;
    type Requirements: ProviderConstructionRequirements;

    fn decode<'call>(
        call: &mut HostCall<'call, Profile, Provider, ()>,
        value: <Self::Host as HostType>::Value<'call>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Output;
}

/// The exact native decoder and the work's original execution endpoint.
#[doc(hidden)]
pub struct ProviderFutureValueContext<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    work: HostFutureValue<Profile, FutureProvider, Host>,
    decode: Decode<Profile, Host, Output>,
    callable_base: usize,
}

impl<Value, Profile, Host, Output> Future<Value, ProviderFutureValueContext<Profile, Host, Output>>
where
    Profile: HostProfile,
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_host<'call, Codec, Provider, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: HostExternal<'call, HostFutureType<Host, crate::host::HostWorkSchema<Profile>>>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Profile: HostWorkProfile,
        Provider: HostProvider<Profile>,
        Codec: ProviderFutureCodec<Profile, Provider, Host = Host, Output = Output>,
    {
        Self::from_host_with::<Codec, Provider, Provider, Return>(call, value, constructions)
    }

    #[doc(hidden)]
    pub fn from_host_with<'call, Codec, Provider, CallerProvider, Return: HostType>(
        call: &HostCall<'call, Profile, CallerProvider, Return>,
        value: HostExternal<'call, HostFutureType<Host, crate::host::HostWorkSchema<Profile>>>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Profile: HostWorkProfile,
        Provider: HostProvider<Profile>,
        CallerProvider: HostProvider<Profile>,
        Codec: ProviderFutureCodec<Profile, Provider, Host = Host, Output = Output>,
    {
        Self {
            context: ProviderFutureValueContext {
                work: call.future_value(value).into_provider(),
                decode: decode::<Profile, Provider, Codec>,
                callable_base: constructions.host().callable_base(),
            },
            value: PhantomData,
        }
    }

    pub(crate) async fn observe(
        &self,
        dependencies: &crate::runtime::work::Dependencies<
            crate::runtime::work::execution::Completion,
        >,
    ) -> Result<Output, HostExecutionError>
    where
        Output: Send + 'static,
    {
        let decode = self.context.decode;
        let callable_base = self.context.callable_base;
        self.context
            .work
            .observe_dependencies(dependencies, move |call, value| {
                Ok(decode(call, value, callable_base))
            })
            .await
    }

    #[doc(hidden)]
    pub fn into_host<'call, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Result<
        HostExternal<'call, HostFutureType<Host, crate::host::HostWorkSchema<Profile>>>,
        crate::HostCallError,
    >
    where
        Profile: HostWorkProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        self.context.work.restore(call)
    }
}

impl<Profile, Host, Output> Clone for ProviderFutureValueContext<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            decode: self.decode,
            callable_base: self.callable_base,
        }
    }
}

fn decode<'call, Profile, Provider, Codec>(
    call: HostCall<'call, Profile, FutureProvider, ()>,
    value: <Codec::Host as HostType>::Value<'call>,
    callable_base: usize,
) -> Codec::Output
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderFutureCodec<Profile, Provider>,
{
    let mut call = call.into_provider::<Provider>();
    let constructions = crate::HostConstructions::with_base(callable_base);
    Codec::decode(
        &mut call,
        value,
        &ProviderConstructions::new(&constructions),
    )
}

impl<Value, Context: Clone> Clone for Future<Value, Context> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            value: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Future, ProviderFutureCodec, ProviderFutureValueContext};
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::host::HostFutureStore;
    use crate::provider::{
        Call, Callback, ProviderCallbackCodec, ProviderConstruction, ProviderConstructionIndex0,
        ProviderConstructionIndexNext, ProviderConstructionList, ProviderConstructionRequirements,
        ProviderConstructions, ProviderNoConstructions, ProviderOwnedCallbackContext,
    };
    use crate::work_fixture::{WorkComponent, WorkHostType, WorkType};
    use crate::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostCallableSchema, HostCaptures, HostComponentProfile, HostConstructions,
        HostCreatedFunction, HostExecutionContext, HostExecutionError, HostExternal,
        HostFunctionType, HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule,
        HostProviderSet, HostReturns, HostTypeList, HostTypeListEnd, ModuleSource, PackageSource,
    };
    use num_bigint::BigInt;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    type End = HostTypeListEnd;
    type One<Type> = HostTypeList<Type, End>;
    type Thunk = HostFunctionType<End, BigInt>;
    type Consumer = HostFunctionType<One<Thunk>, BigInt>;
    type Needed = ProviderConstruction<HostCreatedFunction<Add<100>>>;
    type Requirements = ProviderConstructionList<
        ProviderConstruction<HostCreatedFunction<Add<10>>>,
        ProviderConstructionList<Needed, ProviderNoConstructions>,
    >;
    type Constructions = <Requirements as ProviderConstructionRequirements>::Types<End>;
    type Selected = <Needed as ProviderConstructionRequirements>::Types<End>;
    type Returned =
        Callback<fn(Thunk) -> BigInt, ProviderOwnedCallbackContext<Profile, Native, Consume>>;

    struct Profile;
    #[derive(Default)]
    struct State {
        unit: (),
        native_effects: Vec<usize>,
        factory_entries: Arc<AtomicUsize>,
        decoded_values: usize,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut State) -> &mut () {
            &mut state.unit
        }
    }
    struct Native;
    impl HostProvider<Profile> for Native {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }
    struct Observer;
    impl HostProvider<Profile> for Observer {
        type State = ();
        fn project(state: &mut State) -> &mut () {
            &mut state.unit
        }
    }
    struct Add<const OFFSET: usize>;
    impl<const OFFSET: usize> HostCallableSchema for Add<OFFSET> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = if OFFSET == 10 { "first" } else { "second" };
        type Arguments = End;
        type Return = BigInt;
        type Captures = One<BigInt>;
        type Constructions = End;
        type Completion = HostReturns;
    }
    fn add<'call, const OFFSET: usize>(
        mut call: HostCall<'call, Profile, Native, BigInt>,
        captures: HostCaptures<'call, One<BigInt>>,
        _: HostConstructions<'call, End>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let (value, ()) = call.captures(captures);
        call.state().native_effects.push(OFFSET);
        Ok(call.return_value(value + OFFSET))
    }
    struct Consume;
    impl ProviderCallbackCodec<Profile, Native, ()> for Consume {
        type HostArguments = One<Thunk>;
        type HostReturn = BigInt;
        type Arguments = (BigInt,);
        type Returned = BigInt;
        type Requirements = Needed;
        fn into_host_arguments<'call>(
            arguments: (BigInt,),
            call: &mut HostCall<'call, Profile, Native, ()>,
            constructions: &ProviderConstructions<'call, Needed>,
        ) -> Result<(HostCallable<'call, End, BigInt>, ()), HostCallError> {
            Ok((
                call.construct_function(constructions.token(), (arguments.0, ())),
                (),
            ))
        }
        fn from_host_return<'call>(
            value: BigInt,
            _: &mut HostCall<'call, Profile, Native, ()>,
            _: &ProviderConstructions<'call, Needed>,
        ) -> BigInt {
            value
        }
    }
    struct CompletedConsumer;
    impl ProviderFutureCodec<Profile, Native> for CompletedConsumer {
        type Host = Consumer;
        type Output = Returned;
        type Requirements = Needed;
        fn decode<'call>(
            call: &mut HostCall<'call, Profile, Native, ()>,
            value: HostCallable<'call, One<Thunk>, BigInt>,
            constructions: &ProviderConstructions<'call, Needed>,
        ) -> Returned {
            call.state().decoded_values += 1;
            Callback::from_owned_host::<Consume, _, _>(
                call,
                value,
                ProviderConstructions::new(&constructions.host()),
            )
        }
    }
    fn observe<'call>(
        call: HostCall<'call, Profile, Native, WorkHostType<BigInt>>,
        constructions: HostConstructions<'call, Constructions>,
        work: HostExternal<'call, WorkHostType<Consumer>>,
    ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
        let proof = ProviderConstructions::<Requirements>::new(&constructions)
            .select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>();
        let work =
            Future::<Consumer, ProviderFutureValueContext<Profile, Consumer, Returned>>::from_host::<
                CompletedConsumer,
                _,
                _,
            >(&call, work, proof);
        let alias = work.clone();
        let call = call.into_provider::<Observer>();
        Ok(call.return_future(constructions, move |context| {
            Box::pin(async move {
                let mut call = Call::from_future_context(context);
                let first = call.observe(&work).await.unwrap();
                let second = call.observe(&alias).await.unwrap();
                drop(work);
                drop(alias);
                let first = call.invoke(&first, (7.into(),)).await.unwrap();
                let second = call.invoke(&second, (8.into(),)).await.unwrap();
                Ok(HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(first + second))
                }))
            })
        }))
    }

    async fn construct_in_body(
        context: &HostExecutionContext<'_, Profile, Native, Selected>,
    ) -> Result<HostOwnedCompletion<Profile, Native, Thunk, Selected>, HostExecutionError> {
        let callback = context
            .with_constructions(|mut call, proof| {
                call.state().factory_entries.fetch_add(1, Ordering::SeqCst);
                let effects_before = call.state().native_effects.len();
                let callback =
                    call.construct_function(proof.at::<crate::HostTypeIndex0>(), (7.into(), ()));
                assert_eq!(call.state().native_effects.len(), effects_before);
                call.owned_callable(callback, &HostConstructions::<End>::new())
            })
            .await
            .unwrap();
        let first = callback
            .invoke(context, |_, _| (), |_, _, value| Ok(value))
            .await
            .unwrap();
        assert_eq!(first, BigInt::from(107));
        Ok(HostOwnedCompletion::new(move |mut call, _| {
            let callback = callback.restore(&mut call).unwrap();
            Ok(call.return_value(callback))
        }))
    }

    fn resumed_factory<'call>(
        call: HostCall<'call, Profile, Native, Thunk>,
        constructions: HostConstructions<'call, Constructions>,
    ) -> Result<HostCallContinuation<'call, Thunk>, HostCallError> {
        let proof = ProviderConstructions::<Requirements>::new(&constructions)
            .select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>()
            .host();
        Ok(call.resume(proof, move |context| {
            Box::pin(async move { construct_in_body(&context).await })
        }))
    }

    fn future_factory<'call>(
        call: HostCall<'call, Profile, Native, WorkHostType<Thunk>>,
        constructions: HostConstructions<'call, Constructions>,
    ) -> Result<HostCallCompletion<'call, WorkHostType<Thunk>>, HostCallError> {
        let proof = ProviderConstructions::<Requirements>::new(&constructions)
            .select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>()
            .host();
        Ok(call.return_future(proof, move |context| {
            Box::pin(async move { construct_in_body(context.execution()).await })
        }))
    }

    #[test]
    fn neutral_projection_preserves_the_borrowed_application_state() {
        struct Profile;
        impl crate::HostProfile for Profile {
            type RunState = Vec<usize>;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        let mut state = vec![3];
        let original = std::ptr::from_mut(&mut state);
        let projected =
            <super::FutureProvider as crate::HostProvider<Profile>>::project(&mut state);
        assert_eq!(std::ptr::from_mut(projected), original);
        projected.push(5);
        assert_eq!(state, [3, 5]);
    }

    #[test]
    fn resumed_and_future_bodies_create_with_their_selected_native_permission() {
        let mut providers = WorkComponent::providers::<Profile>().unwrap();
        providers.push(HostProviderModule::new("application", "library").unwrap()
            .with_resumable_function::<Native, (), Thunk, Constructions, _>("resumed_factory", resumed_factory).unwrap()
            .with_scoped_function_and_constructions::<Native, (), WorkHostType<Thunk>, Constructions, _>("future_factory", future_factory).unwrap()
            .with_callable::<Native, Add<10>, (), _>(add::<10>).unwrap()
            .with_callable::<Native, Add<100>, (), _>(add::<100>).unwrap());
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
import fixture/work
@external(erlang, "native", "resumed_factory")
fn resumed_factory() -> fn() -> Int
@external(erlang, "native", "future_factory")
fn future_factory() -> work.Work(fn() -> Int)
pub fn run() { let callback = resumed_factory() callback() }
pub fn run_work() { work.map(future_factory(), fn(callback) { callback() }) }
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap();
        let (mut builder, run) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let run_work = builder
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("run_work"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = State::default();
        assert_eq!(
            std::ptr::from_mut(Profile::component_state(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        assert_eq!(
            std::ptr::from_mut(Observer::project(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        let entries = Arc::clone(&state.factory_entries);
        host.block_on(
            module.with_execution(&host, &mut state, &mut drop, async |scope| {
                assert_eq!(scope.call(&run, ()).await.unwrap(), BigInt::from(107));
                assert_eq!(entries.load(Ordering::SeqCst), 1);
                let work = scope.call(&run_work, ()).await.unwrap();
                assert_eq!(entries.load(Ordering::SeqCst), 1);
                let alias = work.clone();
                assert_eq!(
                    scope.observe(&work).await.unwrap().read(Clone::clone),
                    BigInt::from(107)
                );
                assert_eq!(
                    scope.observe(&alias).await.unwrap().read(Clone::clone),
                    BigInt::from(107)
                );
                assert_eq!(entries.load(Ordering::SeqCst), 2);
            }),
        )
        .unwrap();
        assert_eq!(state.native_effects, [100, 100, 100, 100]);
    }

    #[test]
    fn observed_callbacks_preserve_the_selected_native_body_proof_and_shared_work() {
        let mut providers = WorkComponent::providers::<Profile>().unwrap();
        providers.push(HostProviderModule::new("application", "library").unwrap()
            .with_scoped_function_and_constructions::<Native, (WorkHostType<Consumer>,), WorkHostType<BigInt>, Constructions, _>("observe", observe).unwrap()
            .with_callable::<Native, Add<10>, (), _>(add::<10>).unwrap()
            .with_callable::<Native, Add<100>, (), _>(add::<100>).unwrap());
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
import fixture/work
@external(erlang, "native", "observe")
fn observe(value: work.Work(fn(fn() -> Int) -> Int)) -> work.Work(Int)
pub fn run() {
  observe(work.map(work.ready(Nil), fn(_) { echo "built" fn(next) { next() + 1 } }))
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap();
        let (builder, run) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("run"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = State::default();
        assert_eq!(
            std::ptr::from_mut(Profile::component_state(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        assert_eq!(
            std::ptr::from_mut(Observer::project(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        let mut echoes = Vec::new();
        let mut echo = |value: crate::EchoOutput| echoes.push(value.value().inspect().to_string());
        let result = host
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    let work = scope.call(&run, ()).await.unwrap();
                    scope.observe(&work).await.unwrap().read(Clone::clone)
                }),
            )
            .unwrap();
        assert_eq!(result, BigInt::from(217));
        assert_eq!(state.native_effects, [100, 100]);
        assert_eq!(state.decoded_values, 2);
        assert_eq!(echoes, ["\"built\""]);
    }
}
