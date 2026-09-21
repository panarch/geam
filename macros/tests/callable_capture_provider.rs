use geam_core::execution::TokioHost;
use geam_core::provider::{BigInt, Call, Callback, Factory, HostResult};
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
    plan_host_program,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
pub struct State {
    drops: Arc<AtomicUsize>,
    reads: Vec<BigInt>,
}

#[geam_macros::provider(package = "capture_provider", state = State, modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "capture_provider", crate_path = geam_core)]
mod native {
    use super::{Arc, AtomicUsize, BigInt, Call, Callback, Factory, HostResult, Ordering, State};
    use geam_core::provider::advanced::External;
    use geam_core::provider::{EcoString, ExternalPayload};
    use std::cell::Cell;

    #[geam_macros::external(name = "Token", manual)]
    struct Token {
        value: BigInt,
        drops: Arc<AtomicUsize>,
        reads: Cell<usize>,
    }
    impl ExternalPayload for Token {
        fn source_equal(&self, other: &Self) -> bool {
            self.value == other.value
        }
        fn source_hash(&self) -> u64 {
            0
        }
        fn inspect(&self) -> EcoString {
            "Token(<opaque>)".into()
        }
    }
    impl Drop for Token {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[geam_macros::callable(factory = SumCaptures)]
    fn sum_captures(
        #[geam_macros::capture] a: BigInt,
        #[geam_macros::capture] b: BigInt,
        #[geam_macros::capture] c: BigInt,
        #[geam_macros::capture] d: BigInt,
        #[geam_macros::capture] e: BigInt,
        #[geam_macros::capture] f: BigInt,
        #[geam_macros::capture] g: BigInt,
        #[geam_macros::capture] h: BigInt,
    ) -> BigInt {
        a + b + c + d + e + f + g + h
    }

    #[geam_macros::function]
    fn capture_eight(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<SumCaptures>,
    ) -> HostResult<Callback<fn() -> BigInt>> {
        call.create(
            &factory,
            (
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
                6.into(),
                7.into(),
                8.into(),
            ),
        )
    }

    #[geam_macros::function]
    fn token(#[geam_macros::call] call: &mut Call<State>, value: BigInt) -> Token {
        Token {
            value,
            drops: Arc::clone(&call.state().drops),
            reads: Cell::new(0),
        }
    }

    #[geam_macros::callable(factory = ReadList, await)]
    async fn read_list(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] values: List<Token>,
    ) -> HostResult<BigInt> {
        let selected = values.get(1).expect("second captured item");
        drop(values);
        let value = selected.with(|token| {
            token.reads.set(token.reads.get() + 1);
            token.value.clone()
        });
        let result = value.clone();
        call.with_state(move |state| state.reads.push(value))
            .await?;
        Ok(result)
    }

    #[geam_macros::callable(factory = ReadOne, await)]
    async fn read_one(#[geam_macros::capture] token: External<Token>) -> BigInt {
        token.with(|token| token.value.clone())
    }

    #[geam_macros::function]
    fn capture_list(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ReadList>,
        values: List<Token>,
    ) -> HostResult<Callback<fn() -> BigInt>> {
        call.create(&factory, (values,))
    }

    #[geam_macros::function]
    fn capture_one(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ReadOne>,
        value: External<Token>,
    ) -> HostResult<Callback<fn() -> BigInt>> {
        call.create(&factory, (value,))
    }

    #[geam_macros::function(await)]
    async fn capture_list_owned(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ReadList>,
        values: List<Token>,
    ) -> HostResult<Callback<fn() -> BigInt>> {
        call.create(&factory, (values,)).await
    }

    #[geam_macros::function(await)]
    async fn capture_one_bounded(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ReadOne>,
        value: External<Token>,
    ) -> HostResult<Callback<fn() -> BigInt>> {
        Ok(call
            .with_call(move |call| call.create(&factory, (value,)))
            .await??)
    }
}

struct Profile;
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = <Component as HostProviderComponent>::Stores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
        stores
    }
    fn component_state(state: &mut State) -> &mut State {
        state
    }
}

#[test]
fn received_external_and_lazy_list_captures_keep_non_clone_send_only_payloads() {
    let source = r#"
pub type Token
@external(erlang, "native", "token")
fn token(value: Int) -> Token
@external(erlang, "native", "capture_list")
fn capture_list(values: List(Token)) -> fn() -> Int
@external(erlang, "native", "capture_one")
fn capture_one(value: Token) -> fn() -> Int
@external(erlang, "native", "capture_list_owned")
fn capture_list_owned(values: List(Token)) -> fn() -> Int
@external(erlang, "native", "capture_one_bounded")
fn capture_one_bounded(value: Token) -> fn() -> Int
@external(erlang, "native", "capture_eight")
fn capture_eight() -> fn() -> Int
pub fn main() {
  let sum = capture_eight()
  let assert 36 = sum()
  let assert 36 = sum()
  let list_callback = capture_list([token(10), token(20), token(30)])
  let single = capture_one(token(40))
  let owned_list = capture_list_owned([token(50), token(60), token(70)])
  let bounded_single = capture_one_bounded(token(80))
  #(list_callback(), list_callback(), single(), owned_list(), bounded_single())
}
"#;
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "capture_provider",
        "capture_provider",
        [PackageSource::new(
            "capture_provider",
            Vec::<String>::new(),
            [ModuleSource::new(
                "capture_provider",
                "capture_provider.gleam",
                source,
            )],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut state = State::default();
    let value = runtime
        .block_on(execution.run_main(&host, &mut state, &mut Vec::new()))
        .unwrap();
    assert_eq!(value.inspect().to_string(), "#(20, 20, 40, 60, 80)");
    assert_eq!(state.reads, [20.into(), 20.into(), 60.into()]);
    assert_eq!(state.drops.load(Ordering::SeqCst), 8);
}
