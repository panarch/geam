use geam_core::StringValue;
use geam_core::execution::{RunError, TokioHost};
use geam_core::provider::{Call, Callback, HostFailure, HostResult, List, Value};
use geam_core::{
    ExecutionError, HostComponentProfile, HostLocation, HostModule, HostProfile,
    HostProviderComponent, HostProviderComponentRegistration, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, PanicKind, PanicMessage, Value as RuntimeValue,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[derive(Default)]
pub struct RunState {
    entries: Vec<StringValue>,
}

#[geam_macros::provider(
    package = "callback_provider",
    state = RunState,
    modules = [callback_provider],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "callback_provider", crate_path = geam_core)]
mod callback_provider {
    use super::{
        BigInt, Call, Callback, HostFailure, HostResult, List, RunState, StringValue, Value,
    };

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    struct Token(StringValue);

    #[geam_macros::custom(input = DecisionInput)]
    enum Decision {
        Accepted(StringValue),
        Rejected,
    }

    #[geam_macros::function]
    fn record(#[geam_macros::call] call: &mut Call<RunState>, entry: StringValue) -> () {
        call.state_mut().entries.push(entry);
    }

    #[geam_macros::function]
    fn entries(#[geam_macros::call] call: &Call<RunState>) -> StringValue {
        call.state().entries.join("/").into()
    }

    #[geam_macros::function(await)]
    async fn around<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> Value<Item>>,
    ) -> HostResult<Value<Item>> {
        call.with_state(|state| state.entries.push("before".into()))
            .await?;
        let returned = call.invoke(&callback, ()).await?;
        call.with_state(|state| state.entries.push("after".into()))
            .await?;
        Ok(returned)
    }

    #[geam_macros::function(await)]
    async fn defer<Output, Cleanup>(
        #[geam_macros::call] call: &mut Call<RunState>,
        cleanup: Callback<fn() -> Value<Cleanup>>,
        body: Callback<fn() -> Value<Output>>,
    ) -> HostResult<Value<Output>> {
        let result = call.invoke(&body, ()).await;
        call.invoke(&cleanup, ()).await?;
        result
    }

    #[geam_macros::function(await)]
    async fn on_crash<Output, Cleanup>(
        #[geam_macros::call] call: &mut Call<RunState>,
        cleanup: Callback<fn() -> Value<Cleanup>>,
        body: Callback<fn() -> Value<Output>>,
    ) -> HostResult<Value<Output>> {
        let result = call.invoke(&body, ()).await;
        if result.is_err() {
            call.invoke(&cleanup, ()).await?;
        }
        result
    }

    #[geam_macros::function(await)]
    async fn apply<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(Value<Item>) -> Value<Item>>,
        value: Value<Item>,
    ) -> HostResult<Value<Item>> {
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function(await)]
    async fn rotate(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(StringValue, BigInt) -> (BigInt, StringValue)>,
        label: StringValue,
        number: BigInt,
    ) -> HostResult<(BigInt, StringValue)> {
        call.invoke(&callback, (label, number)).await
    }

    #[geam_macros::function(await)]
    async fn decide(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(Token, Decision) -> DecisionInput>,
        label: StringValue,
    ) -> HostResult<Decision> {
        let returned = call
            .invoke(&callback, (Token(label.clone()), Decision::Accepted(label)))
            .await?;
        Ok(match returned {
            DecisionInput::Accepted(label) => Decision::Accepted(label),
            DecisionInput::Rejected => Decision::Rejected,
        })
    }

    #[geam_macros::function(await)]
    async fn list_total(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<
            fn(Vec<((BigInt, StringValue), self::Token)>) -> geam_core::List<BigInt>,
        >,
    ) -> HostResult<BigInt> {
        let values = call
            .invoke(
                &callback,
                (vec![
                    ((1.into(), "one".into()), Token("first".into())),
                    ((2.into(), "two".into()), Token("second".into())),
                    ((3.into(), "three".into()), Token("third".into())),
                ],),
            )
            .await?;
        let mut total = BigInt::from(0);
        for index in 0..values.len() {
            total += values.get(index).expect("callback List index must exist");
        }
        Ok(total)
    }

    #[geam_macros::function(await)]
    async fn classify(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<
            fn(
                (StringValue, BigInt),
                Result<StringValue, Decision>,
                Option<BigInt>,
            ) -> Option<BigInt>,
        >,
    ) -> HostResult<Option<BigInt>> {
        call.invoke(
            &callback,
            (
                ("pair".into(), 2.into()),
                Ok("result".into()),
                Some(7.into()),
            ),
        )
        .await
    }

    #[geam_macros::function(await)]
    async fn inspect_callback<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> (Value<Item>, List<StringValue>)>,
    ) -> HostResult<(Value<Item>, BigInt)> {
        let (value, messages) = call.invoke(&callback, ()).await?;
        Ok((value, messages.len().into()))
    }

    #[geam_macros::function]
    fn fail() -> HostResult<()> {
        Err(HostFailure::new("callback provider failed").into())
    }

    #[geam_macros::function]
    fn keep_callback<Input, Output>(
        callback: Callback<fn(Value<Input>) -> Value<Output>>,
    ) -> Callback<fn(Value<Input>) -> Value<Output>> {
        callback
    }

    #[geam_macros::function(await)]
    async fn higher_order<Input, Output>(
        #[geam_macros::call] call: &mut Call<RunState>,
        factory: Callback<fn() -> Callback<fn(Value<Input>) -> Value<Output>>>,
        consumer: Callback<
            fn(Callback<fn(Value<Input>) -> Value<Output>>, Value<Input>) -> Value<Output>,
        >,
        value: Value<Input>,
    ) -> HostResult<Value<Output>> {
        let callback = call.invoke(&factory, ()).await?;
        call.invoke(&consumer, (callback, value)).await
    }

    #[geam_macros::function(await)]
    async fn constructed_callback_arguments(
        #[geam_macros::call] call: &mut Call<RunState>,
        factory: Callback<fn() -> Callback<fn(Vec<Token>) -> BigInt>>,
    ) -> HostResult<BigInt> {
        let callback = call.invoke(&factory, ()).await?;
        call.invoke(&callback, (vec![Token("one".into()), Token("two".into())],))
            .await
    }

    #[geam_macros::function]
    fn callback_bundle(
        callbacks: (Callback<fn(BigInt) -> BigInt>,),
    ) -> (
        Option<Callback<fn(BigInt) -> BigInt>>,
        Result<Callback<fn(BigInt) -> BigInt>, StringValue>,
        Vec<Callback<fn(BigInt) -> BigInt>>,
    ) {
        (
            Some(callbacks.0.clone()),
            Ok(callbacks.0.clone()),
            vec![callbacks.0],
        )
    }

    #[geam_macros::function(await)]
    async fn callback_compounds(
        #[geam_macros::call] call: &mut Call<RunState>,
        factory: Callback<
            fn() -> (
                Result<Callback<fn(Vec<Token>) -> BigInt>, StringValue>,
                Option<Callback<fn(BigInt) -> BigInt>>,
            ),
        >,
        consume: Callback<
            fn(
                (
                    Option<Callback<fn(BigInt) -> BigInt>>,
                    Result<Callback<fn(BigInt) -> BigInt>, StringValue>,
                ),
            ) -> BigInt,
        >,
    ) -> HostResult<BigInt> {
        let (tokens, number) = call.invoke(&factory, ()).await?;
        let total = match tokens {
            Ok(callback) => {
                call.invoke(&callback, (vec![Token("nested".into())],))
                    .await?
            }
            Err(_) => BigInt::from(0),
        };
        let callbacks = match number {
            Some(callback) => (Some(callback.clone()), Ok(callback)),
            None => (None, Err("missing".into())),
        };
        Ok(total + call.invoke(&consume, (callbacks,)).await?)
    }
    #[geam_macros::function]
    fn keep_callbacks<Input, Output>(
        callbacks: List<Callback<fn(Value<Input>) -> Value<Output>>>,
    ) -> List<Callback<fn(Value<Input>) -> Value<Output>>> {
        callbacks
    }

    #[geam_macros::function(await)]
    async fn pick_callback<Input, Output>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callbacks: List<Callback<fn(Value<Input>) -> Value<Output>>>,
        value: Value<Input>,
    ) -> HostResult<Value<Output>> {
        let callback = callbacks.get(1).expect("selected callback");
        drop(callbacks);
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function(await)]
    async fn callback_list_factory(
        #[geam_macros::call] call: &mut Call<RunState>,
        factory: Callback<fn() -> List<Callback<fn(Vec<Token>) -> BigInt>>>,
    ) -> HostResult<BigInt> {
        let callbacks = call.invoke(&factory, ()).await?;
        let callback = callbacks.get(1).expect("selected constructed callback");
        drop(callbacks);
        let first = call
            .invoke(&callback, (vec![Token("a".into()), Token("b".into())],))
            .await?;
        Ok(first + call.invoke(&callback, (vec![Token("c".into())],)).await?)
    }

    #[geam_macros::function(await)]
    async fn callback_list_compounds(
        #[geam_macros::call] call: &mut Call<RunState>,
        callbacks: List<(
            Option<Callback<fn(BigInt) -> BigInt>>,
            Result<Callback<fn(BigInt) -> BigInt>, StringValue>,
        )>,
    ) -> HostResult<BigInt> {
        let (first, second) = callbacks.get(1).expect("selected compound");
        drop(callbacks);
        let first = first.expect("present callback");
        let second = second.unwrap_or_else(|_| panic!("successful callback"));
        Ok(call.invoke(&first, (1.into(),)).await? + call.invoke(&second, (2.into(),)).await?)
    }
    #[geam_macros::function(await)]
    async fn produce_callbacks<Input, Output>(
        #[geam_macros::call] call: &mut Call<RunState>,
        producer: Callback<fn() -> List<Callback<fn(Value<Input>) -> Value<Output>>>>,
    ) -> HostResult<List<Callback<fn(Value<Input>) -> Value<Output>>>> {
        call.invoke(&producer, ()).await
    }

    #[geam_macros::function(await)]
    async fn pass_callbacks<Input, Output>(
        #[geam_macros::call] call: &mut Call<RunState>,
        consumer: Callback<
            fn(List<Callback<fn(Value<Input>) -> Value<Output>>>, Value<Input>) -> Value<Output>,
        >,
        callbacks: List<Callback<fn(Value<Input>) -> Value<Output>>>,
        value: Value<Input>,
    ) -> HostResult<Value<Output>> {
        call.invoke(&consumer, (callbacks, value)).await
    }

    #[geam_macros::function]
    fn inner_callbacks<Input, Output>(
        callbacks: List<List<Callback<fn(Value<Input>) -> Value<Output>>>>,
    ) -> List<Callback<fn(Value<Input>) -> Value<Output>>> {
        callbacks.get(1).expect("selected inner list")
    }

    #[geam_macros::function(await)]
    async fn nested_callback_list(
        #[geam_macros::call] call: &mut Call<RunState>,
        callbacks: List<Option<List<Callback<fn(Vec<Token>) -> BigInt>>>>,
    ) -> HostResult<BigInt> {
        let Some(inner) = callbacks.get(1).expect("selected option") else {
            return Ok(0.into());
        };
        drop(callbacks);
        let callback = inner.get(1).expect("selected inner callback");
        drop(inner);
        let first = call
            .invoke(&callback, (vec![Token("a".into()), Token("b".into())],))
            .await?;
        Ok(first + call.invoke(&callback, (vec![Token("c".into())],)).await?)
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

#[derive(Default)]
struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const SOURCE: &str = r#"
import gleam/option.{type Option}

@external(erlang, "callback_provider", "record")
fn record(entry: String) -> Nil

@external(erlang, "callback_provider", "entries")
fn entries() -> String

@external(erlang, "callback_provider", "around")
fn around(callback: fn() -> item) -> item

@external(erlang, "callback_provider", "apply")
fn apply(callback: fn(item) -> item, value: item) -> item

@external(erlang, "callback_provider", "rotate")
fn rotate(callback: fn(String, Int) -> #(Int, String), label: String, number: Int) -> #(Int, String)

@external(erlang, "callback_provider", "Token")
pub type Token

pub type Decision {
  Accepted(String)
  Rejected
}

@external(erlang, "callback_provider", "decide")
fn decide(callback: fn(Token, Decision) -> Decision, label: String) -> Decision

@external(erlang, "callback_provider", "list_total")
fn list_total(callback: fn(List(#(#(Int, String), Token))) -> List(Int)) -> Int

@external(erlang, "callback_provider", "classify")
fn classify(callback: fn(#(String, Int), Result(String, Decision), Option(Int)) -> Option(Int)) -> Option(Int)

@external(erlang, "callback_provider", "inspect_callback")
fn inspect_callback(callback: fn() -> #(item, List(String))) -> #(item, Int)

@external(erlang, "callback_provider", "fail")
fn fail() -> Nil

fn body() {
  record("inside")
  41
}

fn increment(value: Int) -> Int {
  value + 1
}

fn rotate_value(label: String, number: Int) -> #(Int, String) {
  #(number + 1, label <> "!")
}

fn keep_decision(_token: Token, decision: Decision) -> Decision {
  decision
}

fn keep_list(values: List(#(#(Int, String), Token))) -> List(Int) {
  case values {
    [] -> []
    [#(#(value, _), _), ..rest] -> [value, ..keep_list(rest)]
  }
}

fn classify_values(pair, result, optional) {
  assert pair == #("pair", 2)
  assert result == Ok("result")
  optional
}

fn callback_pair() {
  #(42, ["first", "second"])
}

fn fail_callback() {
  fail()
}

fn panic_callback() -> Int {
  panic as "callback panic"
}

pub fn main() {
  #(
    around(body),
    apply(increment, 4),
    rotate(rotate_value, "tag", 8),
    decide(keep_decision, "accepted"),
    list_total(keep_list),
    classify(classify_values),
    inspect_callback(callback_pair),
    entries(),
  )
}
@external(erlang, "callback_provider", "keep_callback")
fn keep_callback(callback: fn(a) -> b) -> fn(a) -> b
@external(erlang, "callback_provider", "higher_order")
fn higher_order(factory: fn() -> fn(a) -> b, consumer: fn(fn(a) -> b, a) -> b, value: a) -> b
@external(erlang, "callback_provider", "constructed_callback_arguments")
fn constructed_callback_arguments(factory: fn() -> fn(List(Token)) -> Int) -> Int
@external(erlang, "callback_provider", "callback_bundle")
fn callback_bundle(callbacks: #(fn(Int) -> Int)) -> #(Option(fn(Int) -> Int), Result(fn(Int) -> Int, String), List(fn(Int) -> Int))
@external(erlang, "callback_provider", "callback_compounds")
fn callback_compounds(
  factory: fn() -> #(Result(fn(List(Token)) -> Int, String), Option(fn(Int) -> Int)),
  consume: fn(#(Option(fn(Int) -> Int), Result(fn(Int) -> Int, String))) -> Int,
) -> Int

@external(erlang, "callback_provider", "keep_callbacks")
fn keep_callbacks(callbacks: List(fn(a) -> b)) -> List(fn(a) -> b)
@external(erlang, "callback_provider", "pick_callback")
fn pick_callback(callbacks: List(fn(a) -> b), value: a) -> b
@external(erlang, "callback_provider", "callback_list_factory")
fn callback_list_factory(factory: fn() -> List(fn(List(Token)) -> Int)) -> Int
@external(erlang, "callback_provider", "callback_list_compounds")
fn callback_list_compounds(callbacks: List(#(Option(fn(Int) -> Int), Result(fn(Int) -> Int, String)))) -> Int

@external(erlang, "callback_provider", "inner_callbacks")
fn inner_callbacks(callbacks: List(List(fn(a) -> b))) -> List(fn(a) -> b)
@external(erlang, "callback_provider", "nested_callback_list")
fn nested_callback_list(callbacks: List(Option(List(fn(List(Token)) -> Int)))) -> Int

@external(erlang, "callback_provider", "produce_callbacks")
fn produce_callbacks(producer: fn() -> List(fn(a) -> b)) -> List(fn(a) -> b)
@external(erlang, "callback_provider", "pass_callbacks")
fn pass_callbacks(consumer: fn(List(fn(a) -> b), a) -> b, callbacks: List(fn(a) -> b), value: a) -> b

@external(erlang, "callback_provider", "defer")
fn defer(cleanup: fn() -> b, body: fn() -> a) -> a
@external(erlang, "callback_provider", "on_crash")
fn on_crash(cleanup: fn() -> b, body: fn() -> a) -> a
"#;

const OPTION_SOURCE: &str = r#"
pub type Option(value) {
  Some(value)
  None
}
"#;

fn execution(source: &str) -> HostedExecution<Profile> {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored callback component should register");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("macro-authored callback module should be unique");
    let typed = compile_typed_host_program(
        "callback_provider",
        "callback_provider",
        [
            PackageSource::new(
                "callback_provider",
                ["gleam_stdlib"],
                [ModuleSource::new(
                    "callback_provider",
                    "src/callback_provider.gleam",
                    source,
                )],
            ),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    OPTION_SOURCE,
                )],
            ),
        ],
        hosts,
    )
    .expect("complete callback provider source should compile");
    let plan = plan_host_program(typed).expect("matching callback provider should plan");
    HostedExecution::try_from_module_plan(plan).expect("matching callback provider should seal")
}

#[test]
fn callbacks_reenter_the_component_and_preserve_typed_results() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(SOURCE).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .expect("typed callbacks should execute");

    let RuntimeValue::Tuple(values) = returned else {
        panic!("callback result should preserve the complete tuple");
    };
    assert_eq!(values[0], RuntimeValue::Int(41.into()));
    assert_eq!(values[1], RuntimeValue::Int(5.into()));
    assert_eq!(
        values[2],
        RuntimeValue::Tuple(vec![
            RuntimeValue::Int(9.into()),
            RuntimeValue::String("tag!".into()),
        ]),
    );
    let RuntimeValue::Custom(decision) = &values[3] else {
        panic!("callback custom return should preserve its constructor");
    };
    assert_eq!(decision.constructor_name().as_str(), "Accepted");
    assert_eq!(decision.constructor_index(), 0);
    assert_eq!(decision.fields().len(), 1);
    assert_eq!(
        decision.fields()[0].value(),
        &RuntimeValue::String("accepted".into()),
    );
    assert_eq!(values[4], RuntimeValue::Int(6.into()));
    let RuntimeValue::Custom(optional) = &values[5] else {
        panic!("callback Option return should preserve its constructor");
    };
    assert_eq!(optional.constructor_name().as_str(), "Some");
    assert_eq!(optional.constructor_index(), 0);
    assert_eq!(optional.fields().len(), 1);
    assert_eq!(optional.fields()[0].value(), &RuntimeValue::Int(7.into()));
    assert_eq!(
        values[6],
        RuntimeValue::Tuple(vec![
            RuntimeValue::Int(42.into()),
            RuntimeValue::Int(2.into())
        ]),
    );
    assert_eq!(
        values[7],
        RuntimeValue::String("before/inside/after".into()),
    );
}

#[test]
fn callbacks_return_and_accept_callbacks_with_their_original_construction_proofs() {
    let source = SOURCE.replace("pub fn main()", "fn original_main()")
        + r#"
pub fn main() {
  let callback = fn(value) { record("callback") value + 2 }
  let kept = keep_callback(callback)
  #(
    callback == kept,
    kept(3),
    higher_order(fn() { kept }, fn(next, value) { record("consumer") next(value) }, 5),
    higher_order(fn() { fn(value) { #(value, "typed") } }, fn(next, value) { next(value) }, True),
    constructed_callback_arguments(fn() { fn(tokens) { case tokens { [_, _] -> 2 _ -> 0 } } }),
    entries(),
  )
}

"#;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .unwrap();
    assert_eq!(
        returned.inspect().to_string(),
        "#(True, 5, 7, #(True, \"typed\"), 2, \"callback/consumer/callback\")"
    );
}

#[test]
fn compounds_keep_callable_identity_and_recursive_invocation_permissions() {
    let source = SOURCE.replace("pub fn main()", "fn original_main()")
        + r#"
pub fn main() {
  let callback = fn(value) { record("called") value + 10 }
  let #(maybe, result, functions) = callback_bundle(#(callback))
  let aliases = case #(maybe, result, functions) {
    #(option.Some(a), Ok(b), [c]) -> #(a == callback, b == callback, c == callback, c(1))
    _ -> #(False, False, False, 0)
  }
  let consume = fn(pair) {
    case pair {
      #(option.Some(a), Ok(b)) -> a(1) + b(2)
      #(option.None, Error("missing")) -> 100
      _ -> 0
    }
  }
  #(
    aliases,
    callback_compounds(fn() { #(Ok(fn(tokens) { case tokens { [_] -> 1 _ -> 0 } }), option.Some(callback)) }, consume),
    callback_compounds(fn() { #(Error("none"), option.None) }, consume),
    entries(),
  )
}
"#;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .unwrap();
    assert_eq!(
        returned.inspect().to_string(),
        "#(#(True, True, True, 11), 24, 100, \"called/called/called\")"
    );
}

#[test]
fn retained_callback_lists_preserve_generic_views_identity_and_demanded_permissions() {
    let source = SOURCE.replace("pub fn main()", "fn original_main()")
        + r#"
pub fn main() {
  let selected = fn(value) { record("generic") #(value, "selected") }
  let callbacks = keep_callbacks([fn(value) { record("unused") #(value, "unused") }, selected])
  let identity = case callbacks { [_, callback] -> callback == selected _ -> False }
  let scalar = fn(value) { record("scalar") value + 10 }
  #(
    identity,
    pick_callback(callbacks, True),
    callback_list_factory(fn() {
      [fn(_) { record("unused") 0 }, fn(tokens) { record("tokens") case tokens { [_, _] -> 2 [_] -> 1 _ -> 0 } }]
    }),
    callback_list_compounds([#(option.None, Error("unused")), #(option.Some(scalar), Ok(scalar))]),
    entries(),
  )
}
"#;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .unwrap();
    assert_eq!(
        returned.inspect().to_string(),
        "#(True, #(True, \"selected\"), 3, 23, \"generic/tokens/tokens/scalar/scalar\")"
    );
}

#[test]
fn nested_retained_lists_keep_function_identity_and_constructor_proofs_after_outer_drop() {
    let source = SOURCE.replace("pub fn main()", "fn original_main()")
        + r#"
pub fn main() {
  let selected = fn(value) { record("kept") #(value, "kept") }
  let inner = inner_callbacks([[], [fn(value) { #(value, "unused") }, selected]])
  let identity = case inner { [_, callback] -> callback == selected _ -> False }
  let callback = fn(tokens) { record("nested") case tokens { [_, _] -> 2 [_] -> 1 _ -> 0 } }
  #(
    identity,
    pick_callback(inner, True),
    nested_callback_list([option.None, option.Some([fn(_) { record("unused") 100 }, callback])]),
    nested_callback_list([option.None, option.None]),
    entries(),
  )
}
"#;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .unwrap();
    assert_eq!(
        returned.inspect().to_string(),
        "#(True, #(True, \"kept\"), 3, 0, \"kept/nested/nested\")"
    );
}

#[test]
fn nested_provider_failure_remains_the_original_execution_error() {
    let source = SOURCE.replace(
        "#(\n    around(body),\n    apply(increment, 4),\n    rotate(rotate_value, \"tag\", 8),\n    decide(keep_decision, \"accepted\"),\n    list_total(keep_list),\n    classify(classify_values),\n    inspect_callback(callback_pair),\n    entries(),\n  )",
        "around(fail_callback)",
    );
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let error = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .expect_err("nested provider failure should stop the outer callback");

    let RunError::Execution(ExecutionError::Host(error)) = error else {
        panic!("nested provider failure should remain a host error");
    };
    assert_eq!(error.package().as_str(), "callback_provider");
    assert_eq!(error.module().as_str(), "callback_provider");
    assert_eq!(error.function().as_str(), "fail");
    assert_eq!(
        error.failure().message().as_str(),
        "callback provider failed"
    );
    let HostLocation::Resolved { site, path, line } = error.location() else {
        panic!("source callback host failure should preserve its source call site");
    };
    assert_eq!(site.module(), "callback_provider");
    assert_eq!(site.function(), "fail_callback");
    assert_eq!(path.as_str(), "src/callback_provider.gleam");
    assert_eq!(*line, 77);
}

#[test]
fn nested_source_panic_is_not_rewrapped_as_a_host_failure() {
    let source = SOURCE.replace(
        "#(\n    around(body),\n    apply(increment, 4),\n    rotate(rotate_value, \"tag\", 8),\n    decide(keep_decision, \"accepted\"),\n    list_total(keep_list),\n    classify(classify_values),\n    inspect_callback(callback_pair),\n    entries(),\n  )",
        "around(panic_callback)",
    );
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let error = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .expect_err("nested source panic should stop the outer callback");

    let RunError::Execution(ExecutionError::Panic(panic)) = error else {
        panic!("nested source panic should preserve its source error");
    };
    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(
        panic.message(),
        &PanicMessage::Explicit("callback panic".into()),
    );
    assert_eq!(panic.site().module(), "callback_provider");
    assert_eq!(panic.site().function(), "panic_callback");
}

#[test]
fn callback_produced_lists_return_to_source_and_pass_to_other_callbacks() {
    let source = SOURCE.replace("pub fn main()", "fn original_main()")
        + r#"
pub fn main() {
  let selected = fn(value) { record("called") #(value, "kept") }
  let callbacks = produce_callbacks(fn() { record("produced") [selected] })
  let assert [kept] = callbacks
  let consumer = fn(callbacks, value) {
    let assert [next] = callbacks
    next(value)
  }
  #(kept == selected, kept(True), pass_callbacks(consumer, callbacks, False), entries())
}
"#;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let returned = runtime
        .block_on(execution(&source).run_main(&host, &mut ProfileState::default(), &mut Vec::new()))
        .unwrap();
    assert_eq!(
        returned.inspect().to_string(),
        "#(True, #(True, \"kept\"), #(False, \"kept\"), \"produced/called/called\")"
    );
}

#[test]
fn generic_cleanup_wrappers_preserve_success_and_unresolved_failures() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for (wrapper, body, cleanup, expected, entries) in [
        ("defer", "42", "Nil", Ok("42"), vec!["body", "cleanup"]),
        ("on_crash", "42", "Nil", Ok("42"), vec!["body"]),
        (
            "defer",
            "panic as \"body stopped\"",
            "Nil",
            Err("panic: body stopped"),
            vec!["body", "cleanup"],
        ),
        (
            "on_crash",
            "panic as \"body stopped\"",
            "Nil",
            Err("panic: body stopped"),
            vec!["body", "cleanup"],
        ),
        (
            "defer",
            "42",
            "panic as \"cleanup stopped\"",
            Err("panic: cleanup stopped"),
            vec!["body", "cleanup"],
        ),
        (
            "defer",
            "panic as \"body stopped\"",
            "panic as \"cleanup stopped\"",
            Err("panic: cleanup stopped"),
            vec!["body", "cleanup"],
        ),
        (
            "on_crash",
            "panic as \"body stopped\"",
            "panic as \"cleanup stopped\"",
            Err("panic: cleanup stopped"),
            vec!["body", "cleanup"],
        ),
        (
            "defer",
            "fail() panic as \"unreachable\"",
            "Nil",
            Err(
                "host function callback_provider::callback_provider.fail failed: callback provider failed",
            ),
            vec!["body", "cleanup"],
        ),
    ] {
        let source = SOURCE.replace("pub fn main()", "fn original_main()")
            + &format!(
                r#"
pub fn main() {{
  {wrapper}(fn() {{ record("cleanup") {cleanup} }}, fn() {{ record("body") {body} }})
}}
"#
            );
        let mut state = ProfileState::default();
        let result =
            runtime.block_on(execution(&source).run_main(&host, &mut state, &mut Vec::new()));
        assert_eq!(
            result
                .map(|value| value.inspect().to_string())
                .map_err(|error| error.to_string()),
            expected.map(str::to_owned).map_err(str::to_owned),
            "{wrapper}: {body}; {cleanup}"
        );
        assert_eq!(state.component.entries, entries);
    }
}
