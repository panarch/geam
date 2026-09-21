use geam_core::execution::TokioHost;
use geam_core::provider::{BigInt, Call, Callback, Factory, HostResult, Value};
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
    plan_host_program,
};

#[derive(Default)]
pub struct State {
    effects: Vec<BigInt>,
}

#[geam_macros::provider(package = "callables", state = State, modules = [callables], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "callables", crate_path = geam_core)]
mod callables {
    use super::{BigInt, Call, Callback, Factory, HostResult, State, Value};

    #[geam_macros::callable(factory = Add)]
    fn add(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] offset: BigInt,
        value: BigInt,
    ) -> BigInt {
        call.state_mut().effects.push(value.clone());
        offset + value
    }

    #[geam_macros::function]
    fn make(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Add>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
        call.create(&factory, (offset,))
    }

    #[geam_macros::function(await)]
    async fn make_owned(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Add>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
        let callback = call.create(&factory, (offset,)).await?;
        let result = call.invoke(&callback, (2.into(),)).await?;
        assert_eq!(result, BigInt::from(22));
        Ok(callback)
    }

    #[geam_macros::callable(factory = Rows)]
    fn rows(#[geam_macros::capture] offset: BigInt) -> Vec<BigInt> {
        vec![offset.clone(), offset + 1]
    }

    #[geam_macros::function]
    fn make_rows(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Rows>,
        offset: BigInt,
    ) -> HostResult<Callback<fn() -> geam_core::provider::List<BigInt>>> {
        call.create(&factory, (offset,))
    }

    #[geam_macros::callable(factory = Constant)]
    fn constant<Provider>(#[geam_macros::capture] value: Value<Provider>) -> Value<Provider> {
        value
    }

    #[geam_macros::function]
    fn constant_factory<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Constant<Item>>,
        value: Value<Item>,
    ) -> HostResult<Callback<fn() -> Value<Item>>> {
        call.create(&factory, (value,))
    }

    #[geam_macros::callable(factory = Forward, await)]
    async fn forward<Profile, Return>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] callback: Callback<fn(Value<Profile>) -> Value<Return>>,
        argument: Value<Profile>,
    ) -> HostResult<Value<Return>> {
        call.with_state(|state| state.effects.push(100.into()))
            .await?;
        let value = call.invoke(&callback, (argument,)).await?;
        call.with_state(|state| state.effects.push(101.into()))
            .await?;
        Ok(value)
    }

    #[geam_macros::function]
    fn wrap<Argument, Output>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Forward<Argument, Output>>,
        callback: Callback<fn(Value<Argument>) -> Value<Output>>,
    ) -> HostResult<Callback<fn(Value<Argument>) -> Value<Output>>> {
        call.create(&factory, (callback,))
    }

    #[geam_macros::callable(factory = Subtract)]
    fn subtract(#[geam_macros::capture] offset: BigInt, value: BigInt) -> BigInt {
        value - offset
    }

    #[geam_macros::function]
    fn selected(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] first: Factory<Add>,
        #[geam_macros::factory] second: Factory<Subtract>,
    ) -> HostResult<(
        Callback<fn(BigInt) -> BigInt>,
        Callback<fn(BigInt) -> BigInt>,
    )> {
        Ok((
            call.create(&first, (3.into(),))?,
            call.create(&second, (4.into(),))?,
        ))
    }

    #[geam_macros::callable(factory = ManyCaptures)]
    fn many_captures(
        #[geam_macros::capture] a: BigInt,
        #[geam_macros::capture] b: BigInt,
        #[geam_macros::capture] c: BigInt,
        #[geam_macros::capture] d: BigInt,
        #[geam_macros::capture] e: BigInt,
        #[geam_macros::capture] f: BigInt,
        #[geam_macros::capture] g: BigInt,
        #[geam_macros::capture] h: BigInt,
    ) -> BigInt {
        a + b * 10 + c * 100 + d * 1_000 + e * 10_000 + f * 100_000 + g * 1_000_000 + h * 10_000_000
    }

    #[geam_macros::function]
    fn many(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<ManyCaptures>,
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
    fn aliases(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Add>,
    ) -> HostResult<(
        Callback<fn(BigInt) -> BigInt>,
        Callback<fn(BigInt) -> BigInt>,
    )> {
        let callback = call.create(&factory, (30.into(),))?;
        Ok((callback.clone(), callback))
    }

    #[geam_macros::custom(input = ReplyInput)]
    enum Reply<Item> {
        Saved(Value<Item>),
        Send(Callback<fn(Value<Item>) -> BigInt>),
    }

    #[geam_macros::callable(factory = Deliver, await)]
    async fn deliver<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] reply: ReplyInput<Item>,
        argument: Value<Item>,
    ) -> HostResult<BigInt> {
        match reply {
            ReplyInput::Saved(saved) => {
                let equal = call
                    .with_call(move |call| call.equal(&saved, &argument))
                    .await?;
                Ok(if equal { 1.into() } else { 0.into() })
            }
            ReplyInput::Send(callback) => call.invoke(&callback, (argument,)).await,
        }
    }

    #[geam_macros::function]
    fn reply_factory<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Deliver<Item>>,
        reply: ReplyInput<Item>,
    ) -> HostResult<Callback<fn(Value<Item>) -> BigInt>> {
        let reply = match reply {
            ReplyInput::Saved(value) => Reply::Saved(value),
            ReplyInput::Send(callback) => Reply::Send(callback),
        };
        call.create(&factory, (reply,))
    }

    #[geam_macros::custom(input = StartedInput)]
    enum Started<Decoder> {
        Running(Value<Decoder>),
        Waiting,
    }

    #[geam_macros::callable(factory = Start, await)]
    async fn start<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::capture] callback: Callback<fn() -> Result<StartedInput<Item>, bool>>,
    ) -> HostResult<Result<Started<Item>, bool>> {
        Ok(match call.invoke(&callback, ()).await? {
            Ok(StartedInput::Running(value)) => Ok(Started::Running(value)),
            Ok(StartedInput::Waiting) => Ok(Started::Waiting),
            Err(error) => Err(error),
        })
    }

    #[geam_macros::function]
    fn start_factory<Item>(
        #[geam_macros::call] call: &mut Call<State>,
        #[geam_macros::factory] factory: Factory<Start<Item>>,
        callback: Callback<fn() -> Result<StartedInput<Item>, bool>>,
    ) -> HostResult<Callback<fn() -> Result<StartedInput<Item>, bool>>> {
        call.create(&factory, (callback,))
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

fn run(source: &str) -> (String, State) {
    let source = format!("{DECLARATIONS}\n{source}");
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "callables",
        "callables",
        [PackageSource::new(
            "callables",
            Vec::<String>::new(),
            [ModuleSource::new(
                "callables",
                "callables.gleam",
                source.as_str(),
            )],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let plan = plan_host_program(typed).unwrap();
    let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut state = State::default();
    let value = runtime
        .block_on(execution.run_main(&host, &mut state, &mut Vec::new()))
        .unwrap();
    (value.inspect().to_string(), state)
}

const DECLARATIONS: &str = r#"
@external(erlang, "native", "make_rows")
fn make_rows(offset: Int) -> fn() -> List(Int)
@external(erlang, "native", "make")
fn make(offset: Int) -> fn(Int) -> Int
@external(erlang, "native", "make_owned")
fn make_owned(offset: Int) -> fn(Int) -> Int
@external(erlang, "native", "constant_factory")
fn constant_factory(value: a) -> fn() -> a
@external(erlang, "native", "wrap")
fn wrap(callback: fn(a) -> b) -> fn(a) -> b
@external(erlang, "native", "selected")
fn selected() -> #(fn(Int) -> Int, fn(Int) -> Int)
@external(erlang, "native", "many")
fn many() -> fn() -> Int
@external(erlang, "native", "aliases")
fn aliases() -> #(fn(Int) -> Int, fn(Int) -> Int)
pub type Reply(item) { Saved(item) Send(fn(item) -> Int) }
@external(erlang, "native", "reply_factory")
fn reply_factory(reply: Reply(a)) -> fn(a) -> Int
pub type Started(item) { Running(item) Waiting }
@external(erlang, "native", "start_factory")
fn start_factory(callback: fn() -> Result(Started(a), Bool)) -> fn() -> Result(Started(a), Bool)
"#;

#[test]
fn fresh_native_list_results_preserve_each_capture_instance_across_repeated_calls() {
    let (result, state) = run(r#"
pub fn main() {
  let first = make_rows(6)
  let second = make_rows(20)
  #(first(), second(), first())
}
"#);
    assert_eq!(result, "#([6, 7], [20, 21], [6, 7])");
    assert!(state.effects.is_empty());
}

#[test]
fn private_factories_construct_immediately_and_preserve_native_identity_and_captures() {
    let (result, state) = run(r#"
pub fn main() {
  let first = make(7)
  let alias = first
  let second = make(7)
  let owned = make_owned(20)
  #(first(3), second(5), alias == first, second == first, owned(9))
}
"#);
    assert_eq!(result, "#(10, 12, True, False, 29)");
    assert_eq!(state.effects, [2.into(), 3.into(), 5.into(), 9.into()]);
}

#[test]
fn generic_factory_captures_keep_their_source_type_and_function_identity() {
    let (result, state) = run(r#"
pub fn main() {
  let integer = constant_factory(41)
  let text = constant_factory("kept")
  let original = fn(value) { value + 3 }
  let callback = constant_factory(original)
  let restored = callback()
  #(integer(), text(), restored == original, restored(7), callback() == original)
}
"#);
    assert_eq!(result, "#(41, \"kept\", True, 10, True)");
    assert!(state.effects.is_empty());
}

#[test]
fn generic_native_wrappers_resume_through_source_and_native_callbacks() {
    let (result, state) = run(r#"
pub fn main() {
  let native = make(5)
  let first = wrap(native)
  let nested = wrap(first)
  let original = fn(pair: #(Int, String)) { #(pair.1, pair.0) }
  let swapped = wrap(original)
  let #(add, subtract) = selected()
  let captured = many()
  #(nested(10), swapped(#(7, "seven")), first == nested, add(20), subtract(20), captured())
}
"#);
    assert_eq!(result, "#(15, #(\"seven\", 7), False, 23, 16, 87654321)");
    assert_eq!(
        state.effects,
        [
            100.into(),
            100.into(),
            10.into(),
            101.into(),
            101.into(),
            100.into(),
            101.into(),
            20.into()
        ]
    );
}

#[test]
fn custom_captures_and_generic_custom_results_keep_callbacks_and_source_errors() {
    let (result, state) = run(r#"
pub fn main() {
  let original = fn(value) { value + 3 }
  let saved = reply_factory(Saved(original))
  let send = reply_factory(Send(fn(value) { value(4) }))
  let started = start_factory(fn() { Ok(Running(original)) })
  let failed = start_factory(fn() { Error(True) })
  let waiting = start_factory(fn() { Ok(Waiting) })
  let assert Ok(Running(restored)) = started()
  let #(first, alias) = aliases()
  #(saved(original), send(original), restored == original, restored(5), failed(), waiting(), first == alias, alias(9))
}
"#);
    assert_eq!(
        result,
        "#(1, 7, True, 8, Error(True), Ok(Waiting), True, 39)"
    );
    assert_eq!(state.effects, [9.into()]);
}
