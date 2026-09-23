use geam_core::execution::TokioHost;
use geam_core::provider::{Call, Callback, HostResult};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(package = "messages", modules = [messages, relay], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "messages", crate_path = geam_core)]
mod messages {
    use super::{BigInt, Call, Callback, HostResult};

    #[geam_macros::custom(input = FlagInput)]
    pub enum Flag {
        On,
        Off,
    }

    #[geam_macros::function]
    fn flag(value: FlagInput) -> Flag {
        match value {
            FlagInput::On => Flag::On,
            FlagInput::Off => Flag::Off,
        }
    }

    #[geam_macros::custom(input = MessageInput)]
    pub enum Message {
        Empty,
        Handler {
            argument: BigInt,
            callback: Callback<fn(BigInt) -> BigInt>,
        },
    }

    #[geam_macros::function(await)]
    async fn round_trip(
        #[geam_macros::call] call: &mut Call<()>,
        message: MessageInput,
    ) -> HostResult<(BigInt, Message)> {
        match message {
            MessageInput::Empty => Ok((0.into(), Message::Empty)),
            MessageInput::Handler { argument, callback } => {
                let result = call.invoke(&callback, (argument.clone(),)).await?;
                Ok((result, Message::Handler { argument, callback }))
            }
        }
    }

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    pub struct Token(pub BigInt);

    #[geam_macros::custom(input = RequestInput)]
    pub enum Request {
        Inspect(Callback<fn(Vec<Token>) -> BigInt>),
        Wrapped(Message),
        Handlers(Vec<Callback<fn(Vec<Token>) -> BigInt>>),
    }

    #[geam_macros::function]
    fn token_value(value: &Token) -> BigInt {
        value.0.clone()
    }

    #[geam_macros::function(await)]
    async fn inspect(
        #[geam_macros::call] call: &mut Call<()>,
        request: RequestInput,
    ) -> HostResult<(BigInt, Request)> {
        match request {
            RequestInput::Inspect(callback) => {
                let result = call
                    .invoke(&callback, (vec![Token(2.into()), Token(5.into())],))
                    .await?;
                Ok((result, Request::Inspect(callback)))
            }
            RequestInput::Wrapped(message) => {
                let (result, message) = round_trip(call, message).await?;
                Ok((result, Request::Wrapped(message)))
            }
            RequestInput::Handlers(callbacks) => {
                let first = callbacks.get(0).expect("fixture has a first handler");
                let last = callbacks
                    .get(callbacks.len() - 1)
                    .expect("fixture has a last handler");
                drop(callbacks);
                let first_result = call
                    .invoke(&first, (vec![Token(3.into()), Token(5.into())],))
                    .await?;
                let last_result = call
                    .invoke(&last, (vec![Token(13.into()), Token(21.into())],))
                    .await?;
                Ok((
                    first_result + last_result,
                    Request::Handlers(vec![last, first]),
                ))
            }
        }
    }

    #[geam_macros::function(await)]
    async fn pick(
        #[geam_macros::call] call: &mut Call<()>,
        requests: geam_core::provider::List<RequestInput>,
    ) -> HostResult<(BigInt, Request)> {
        let request = requests.get(1).expect("fixture has a second request");
        drop(requests);
        inspect(call, request).await
    }

    #[geam_macros::custom(input = BundleInput)]
    enum Bundle {
        Nested(Option<(Callback<fn(Vec<Token>) -> BigInt>, Result<Message, BigInt>)>),
        Batches(Vec<Vec<Callback<fn(Vec<Token>) -> BigInt>>>),
    }

    #[geam_macros::function(await)]
    async fn bundle(
        #[geam_macros::call] call: &mut Call<()>,
        value: BundleInput,
    ) -> HostResult<(BigInt, Bundle)> {
        match value {
            BundleInput::Nested(None) => Ok((0.into(), Bundle::Nested(None))),
            BundleInput::Nested(Some((callback, message))) => {
                let (value, message) = match message {
                    Ok(message) => {
                        let (value, returned) = round_trip(call, message).await?;
                        (value, Ok(returned))
                    }
                    Err(code) => (code.clone(), Err(code)),
                };
                let extra = call
                    .invoke(&callback, (vec![Token(2.into()), Token(5.into())],))
                    .await?;
                Ok((value + extra, Bundle::Nested(Some((callback, message)))))
            }
            BundleInput::Batches(batches) => {
                let batch = batches.get(0).expect("fixture has one batch");
                let callback = batch.get(1).expect("fixture has a second callback");
                drop(batch);
                drop(batches);
                let value = call
                    .invoke(&callback, (vec![Token(2.into()), Token(5.into())],))
                    .await?;
                Ok((value, Bundle::Batches(vec![vec![callback]])))
            }
        }
    }

    #[geam_macros::custom(input = GridInput)]
    enum Grid {
        Rows(Vec<Vec<BigInt>>),
        Pair((Vec<BigInt>, Option<Vec<BigInt>>)),
    }

    #[geam_macros::function]
    fn grid() -> Grid {
        Grid::Rows(vec![vec![1.into(), 2.into()], vec![3.into()]])
    }

    #[geam_macros::function]
    fn grid_pair() -> Grid {
        Grid::Pair((vec![4.into()], Some(vec![5.into(), 6.into()])))
    }

    #[geam_macros::function]
    fn grid_total(grid: GridInput) -> BigInt {
        let mut total = BigInt::from(0);
        match grid {
            GridInput::Rows(rows) => {
                for row in 0..rows.len() {
                    let values = rows.get(row).unwrap();
                    for index in 0..values.len() {
                        total += values.get(index).unwrap();
                    }
                }
            }
            GridInput::Pair((values, optional)) => {
                for index in 0..values.len() {
                    total += values.get(index).unwrap();
                }
                if let Some(values) = optional {
                    for index in 0..values.len() {
                        total += values.get(index).unwrap();
                    }
                }
            }
        }
        total
    }

    #[geam_macros::function(await)]
    async fn produced(
        #[geam_macros::call] call: &mut Call<()>,
        factory: Callback<fn() -> RequestInput>,
    ) -> HostResult<(BigInt, Request)> {
        let request = call.invoke(&factory, ()).await?;
        inspect(call, request).await
    }

    #[geam_macros::function(await)]
    async fn submit(
        #[geam_macros::call] call: &mut Call<()>,
        consumer: Callback<fn(Request) -> BigInt>,
        request: RequestInput,
    ) -> HostResult<BigInt> {
        let (_, request) = inspect(call, request).await?;
        call.invoke(&consumer, (request,)).await
    }

    #[geam_macros::custom(input = ProcessorInput)]
    enum Processor {
        Apply(Callback<fn(Request) -> RequestInput>),
    }

    #[geam_macros::function(await)]
    async fn process(
        #[geam_macros::call] call: &mut Call<()>,
        processor: ProcessorInput,
        request: RequestInput,
    ) -> HostResult<(BigInt, Processor, Request)> {
        let ProcessorInput::Apply(callback) = processor;
        let (before, request) = inspect(call, request).await?;
        let request = call.invoke(&callback, (request,)).await?;
        let (after, request) = inspect(call, request).await?;
        Ok((before + after, Processor::Apply(callback), request))
    }
}

struct Profile;

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = <Component as HostProviderComponent>::Stores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        stores
    }
    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        state
    }
}

#[test]
fn custom_callbacks_reenter_source_and_preserve_identity_after_round_trip() {
    let source = r#"
pub fn main() {
  let offset = 7
  let callback = fn(value) { value + offset }
  let #(result, returned) = round_trip(Handler(5, callback))
  let assert Handler(argument, restored) = returned
  let #(empty_result, empty) = round_trip(Empty)
  #(result, argument, callback == restored, restored(8), empty_result, empty == Empty)
}
"#;
    let result = run(source);
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(12.into()),
            Value::Int(5.into()),
            Value::Bool(true),
            Value::Int(15.into()),
            Value::Int(0.into()),
            Value::Bool(true)
        ])
    );
}

#[geam_macros::module(path = "relay", crate_path = geam_core)]
mod relay {
    use super::{BigInt, Call, HostResult, messages};

    #[geam_macros::custom(input = EnvelopeInput)]
    enum Envelope {
        One(messages::Request),
        Many(Vec<messages::Request>),
        Maybe(Option<(messages::Request, Result<messages::Flag, BigInt>)>),
    }

    #[geam_macros::function(await)]
    async fn envelope(
        #[geam_macros::call] call: &mut Call<()>,
        value: EnvelopeInput,
    ) -> HostResult<(BigInt, Envelope)> {
        match value {
            EnvelopeInput::One(request) => {
                let (value, request) = inspect(call, request).await?;
                Ok((value, Envelope::One(request)))
            }
            EnvelopeInput::Many(requests) => {
                let (value, request) = pick(call, requests).await?;
                Ok((value, Envelope::Many(vec![request])))
            }
            EnvelopeInput::Maybe(None) => Ok((0.into(), Envelope::Maybe(None))),
            EnvelopeInput::Maybe(Some((request, result))) => {
                let (value, request) = inspect(call, request).await?;
                let result = result.map(|flag| match flag {
                    messages::FlagInput::On => messages::Flag::On,
                    messages::FlagInput::Off => messages::Flag::Off,
                });
                Ok((value, Envelope::Maybe(Some((request, result)))))
            }
        }
    }

    #[geam_macros::function(await)]
    async fn pick_envelope(
        #[geam_macros::call] call: &mut Call<()>,
        envelopes: geam_core::provider::List<EnvelopeInput>,
    ) -> HostResult<(BigInt, Envelope)> {
        let selected = envelopes.get(1).expect("two envelopes");
        drop(envelopes);
        envelope(call, selected).await
    }

    #[geam_macros::function(await)]
    async fn flag(
        value: messages::FlagInput,
        values: geam_core::provider::List<messages::FlagInput>,
    ) -> messages::Flag {
        let selected = values.get(0).unwrap_or(value);
        match selected {
            messages::FlagInput::On => messages::Flag::On,
            messages::FlagInput::Off => messages::Flag::Off,
        }
    }

    #[geam_macros::function(await)]
    async fn pick(
        #[geam_macros::call] call: &mut Call<()>,
        requests: geam_core::provider::List<messages::RequestInput>,
    ) -> HostResult<(BigInt, messages::Request)> {
        let request = requests.get(1).expect("two requests");
        drop(requests);
        inspect(call, request).await
    }

    #[geam_macros::function(await)]
    async fn pick_nested(
        #[geam_macros::call] call: &mut Call<()>,
        batches: geam_core::provider::List<geam_core::provider::List<messages::RequestInput>>,
    ) -> HostResult<(BigInt, messages::Request)> {
        let requests = batches.get(1).expect("two batches");
        drop(batches);
        pick(call, requests).await
    }

    #[geam_macros::function(await)]
    async fn inspect(
        #[geam_macros::call] call: &mut Call<()>,
        request: messages::RequestInput,
    ) -> HostResult<(BigInt, messages::Request)> {
        match request {
            messages::RequestInput::Inspect(callback) => {
                let value = call
                    .invoke(
                        &callback,
                        (vec![messages::Token(13.into()), messages::Token(21.into())],),
                    )
                    .await?;
                Ok((value, messages::Request::Inspect(callback)))
            }
            messages::RequestInput::Wrapped(message) => match message {
                messages::MessageInput::Empty => Ok((
                    0.into(),
                    messages::Request::Wrapped(messages::Message::Empty),
                )),
                messages::MessageInput::Handler { argument, callback } => {
                    let result = call.invoke(&callback, (argument.clone(),)).await?;
                    Ok((
                        result,
                        messages::Request::Wrapped(messages::Message::Handler {
                            argument,
                            callback,
                        }),
                    ))
                }
            },
            messages::RequestInput::Handlers(callbacks) => {
                let callback = callbacks.get(1).expect("two callbacks");
                drop(callbacks);
                let value = call
                    .invoke(
                        &callback,
                        (vec![messages::Token(2.into()), messages::Token(3.into())],),
                    )
                    .await?;
                Ok((value, messages::Request::Handlers(vec![callback])))
            }
        }
    }
}

fn run(body: &str) -> Value {
    let source = format!("{}\n{}", DECLARATIONS, body);
    run_modules("messages", &source, None)
}

fn run_modules(entry: &str, source: &str, relay: Option<&str>) -> Value {
    let relay = format!("{}\n{}", RELAY_DECLARATIONS, relay.unwrap_or_default());
    let modules = [
        ModuleSource::new("messages", "src/messages.gleam", source),
        ModuleSource::new("relay", "src/relay.gleam", &relay),
    ];
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers).unwrap();
    let typed = compile_typed_host_program(
        "messages",
        entry,
        [
            PackageSource::new("messages", ["gleam_stdlib"], modules),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    "pub type Option(a) { Some(a) None }",
                )],
            ),
        ],
        hosts,
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    runtime
        .block_on(execution.run_main(&host, &mut (), &mut Vec::new()))
        .unwrap()
}

const DECLARATIONS: &str = r#"
import gleam/option.{type Option, Some, None}
pub type Token
pub type Flag { On Off }
@external(erlang, "messages", "flag")
pub fn flag(value: Flag) -> Flag
pub type Message {
  Empty
  Handler(argument: Int, callback: fn(Int) -> Int)
}
pub type Request {
  Inspect(fn(List(Token)) -> Int)
  Wrapped(Message)
  Handlers(List(fn(List(Token)) -> Int))
}
@external(erlang, "messages", "round_trip")
fn round_trip(message: Message) -> #(Int, Message)
@external(erlang, "messages", "token_value")
pub fn token_value(token: Token) -> Int
@external(erlang, "messages", "inspect")
fn inspect(request: Request) -> #(Int, Request)
@external(erlang, "messages", "pick")
fn pick(requests: List(Request)) -> #(Int, Request)
pub type Bundle {
  Nested(Option(#(fn(List(Token)) -> Int, Result(Message, Int))))
  Batches(List(List(fn(List(Token)) -> Int)))
}
pub type Grid {
  Rows(List(List(Int)))
  Pair(#(List(Int), Option(List(Int))))
}
@external(erlang, "messages", "bundle")
fn bundle(value: Bundle) -> #(Int, Bundle)
@external(erlang, "messages", "grid")
fn grid() -> Grid
@external(erlang, "messages", "grid_pair")
fn grid_pair() -> Grid
@external(erlang, "messages", "grid_total")
fn grid_total(value: Grid) -> Int

pub type Processor { Apply(fn(Request) -> Request) }
@external(erlang, "messages", "produced")
fn produced(factory: fn() -> Request) -> #(Int, Request)
@external(erlang, "messages", "submit")
fn submit(consumer: fn(Request) -> Int, request: Request) -> Int
@external(erlang, "messages", "process")
fn process(processor: Processor, request: Request) -> #(Int, Processor, Request)

"#;

#[test]
fn nested_custom_and_list_callbacks_keep_external_argument_constructions() {
    let result = run(r#"
fn total(values: List(Token)) -> Int {
  case values {
    [] -> 0
    [one, two] -> token_value(one) + token_value(two)
    _ -> panic as "fixture requires zero or two tokens"
  }
}
pub fn main() {
  let offset = 10
  let first = fn(values) { total(values) + offset }
  let last = fn(values) { total(values) + 100 }
  let #(inspected, request) = inspect(Inspect(first))
  let assert Inspect(restored) = request
  let #(handled, batch) = inspect(Handlers([first, last]))
  let assert Handlers([left, right]) = batch
  let callback = fn(value) { value + 7 }
  let #(wrapped, message) = inspect(Wrapped(Handler(5, callback)))
  let assert Wrapped(Handler(_, wrapped_callback)) = message
  let #(picked, selection) = pick([Handlers([last]), Inspect(first)])
  let assert Inspect(selected) = selection
  #(inspected, restored == first, restored([]), handled,
    left == last, right == first, wrapped, wrapped_callback == callback,
    picked, selected == first)
}
"#);
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(17.into()),
            Value::Bool(true),
            Value::Int(10.into()),
            Value::Int(152.into()),
            Value::Bool(true),
            Value::Bool(true),
            Value::Int(12.into()),
            Value::Bool(true),
            Value::Int(17.into()),
            Value::Bool(true),
        ])
    );
}

#[test]
fn custom_function_compounds_preserve_results_options_and_nested_lists() {
    let result = run(r#"
fn total(values: List(Token)) -> Int {
  case values {
    [] -> 0
    [one, two] -> token_value(one) + token_value(two)
    _ -> panic as "fixture requires zero or two tokens"
  }
}
pub fn main() {
  let offset = 10
  let callback = fn(values) { total(values) + offset }
  let inner = fn(value) { value + 7 }
  let #(success, wrapped) = bundle(Nested(Some(#(callback, Ok(Handler(5, inner))))))
  let assert Nested(Some(#(success_callback, Ok(Handler(_, restored))))) = wrapped
  let #(failure, error) = bundle(Nested(Some(#(callback, Error(99)))))
  let assert Nested(Some(#(failure_callback, Error(99)))) = error
  let #(empty, option) = bundle(Nested(None))
  let last = fn(values) { total(values) + 100 }
  let #(nested, batches) = bundle(Batches([[callback, last]]))
  let assert Batches([[batch_callback]]) = batches
  #(success, success_callback == callback, restored == inner,
    failure, failure_callback == callback, empty, option == Nested(None),
    nested, batch_callback == last,
    grid_total(grid()), grid_total(grid_pair()), grid_total(Pair(#([2], None))), grid_total(Rows([])))
}
"#);
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(29.into()),
            Value::Bool(true),
            Value::Bool(true),
            Value::Int(116.into()),
            Value::Bool(true),
            Value::Int(0.into()),
            Value::Bool(true),
            Value::Int(107.into()),
            Value::Bool(true),
            Value::Int(6.into()),
            Value::Int(15.into()),
            Value::Int(2.into()),
            Value::Int(0.into()),
        ])
    );
}

#[test]
fn callbacks_exchange_customs_with_their_nested_invocation_permissions() {
    let result = run(r#"
fn total(values: List(Token)) -> Int {
  case values {
    [] -> 0
    [one, two] -> token_value(one) + token_value(two)
    _ -> panic as "fixture requires zero or two tokens"
  }
}
pub fn main() {
  let offset = 10
  let callback = fn(values) { total(values) + offset }
  let factory = fn() { Inspect(callback) }
  let #(value, created) = produced(factory)
  let assert Inspect(created_callback) = created
  let consumer = fn(request) {
    let assert Inspect(received) = request
    received([])
  }
  let consumed = submit(consumer, Inspect(callback))
  let processor = fn(request) { request }
  let #(processed, owner, returned) = process(Apply(processor), Inspect(callback))
  let assert Apply(restored_processor) = owner
  let assert Inspect(restored_callback) = returned
  #(value, created_callback == callback, consumed, processed,
    restored_processor == processor, restored_callback == callback)
}
"#);
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(17.into()),
            Value::Bool(true),
            Value::Int(10.into()),
            Value::Int(34.into()),
            Value::Bool(true),
            Value::Bool(true),
        ])
    );
}

#[test]
fn qualified_custom_callbacks_keep_the_declaring_codec_across_provider_modules() {
    let result = run_modules(
        "relay",
        DECLARATIONS,
        Some(
            r#"
fn total(values: List(Token)) -> Int {
  case values { [] -> 0 [a, b] -> messages.token_value(a) + messages.token_value(b) _ -> panic }
}
pub fn main() {
  let offset = 10
  let callback = fn(values) { total(values) + offset }
  let #(first, request) = inspect(Inspect(callback))
  let assert Inspect(restored) = request
  let inner = fn(value) { value + 7 }
  let #(second, wrapper) = inspect(Wrapped(Handler(5, inner)))
  let assert Wrapped(Handler(_, nested)) = wrapper
  let #(third, batch) = inspect(Handlers([fn(_) { 99 }, callback]))
  let assert Handlers([selected]) = batch
  let #(empty, _) = inspect(Wrapped(Empty))
  #(first, restored == callback, restored([]), second, nested == inner, third, selected == callback, empty)
}
"#,
        ),
    );
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(10.into()),
            Value::Int(12.into()),
            Value::Bool(true),
            Value::Int(15.into()),
            Value::Bool(true),
            Value::Int(0.into())
        ])
    );
}

const RELAY_DECLARATIONS: &str = r#"
import messages.{type Token, type Request, Inspect, Wrapped, Handlers, Handler, Empty}
import gleam/option.{type Option, Some, None}
pub type Envelope {
  One(Request)
  Many(List(Request))
  Maybe(Option(#(Request, Result(messages.Flag, Int))))
}
@external(erlang, "relay", "envelope")
fn envelope(value: Envelope) -> #(Int, Envelope)
@external(erlang, "relay", "pick_envelope")
fn pick_envelope(values: List(Envelope)) -> #(Int, Envelope)
@external(erlang, "relay", "inspect")
fn inspect(request: Request) -> #(Int, Request)
@external(erlang, "relay", "pick")
fn pick(requests: List(Request)) -> #(Int, Request)
@external(erlang, "relay", "pick_nested")
fn pick_nested(batches: List(List(Request))) -> #(Int, Request)
@external(erlang, "relay", "flag")
fn flag(value: messages.Flag, values: List(messages.Flag)) -> messages.Flag
"#;

#[test]
fn qualified_custom_lists_keep_callbacks_after_all_parent_lists_are_dropped() {
    let result = run_modules(
        "relay",
        DECLARATIONS,
        Some(
            r#"
fn total(values: List(Token)) -> Int {
  case values { [] -> 0 [a, b] -> messages.token_value(a) + messages.token_value(b) _ -> panic }
}
pub fn main() {
  let offset = 10
  let callback = fn(values) { total(values) + offset }
  let #(first, request) = pick([Wrapped(Empty), Inspect(callback)])
  let assert Inspect(restored) = request
  let #(second, nested) = pick_nested([[], [Wrapped(Empty), Inspect(callback)]])
  let assert Inspect(nested_callback) = nested
  #(first, restored == callback, restored([]), second, nested_callback == callback, nested_callback([]))
}
"#,
        ),
    );
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(10.into()),
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(10.into())
        ])
    );
}

#[test]
fn unit_only_customs_keep_immediate_owned_and_qualified_list_forms() {
    let result = run_modules(
        "relay",
        DECLARATIONS,
        Some(
            r#"
pub fn main() {
  #(messages.flag(messages.On) == messages.On,
    messages.flag(messages.Off) == messages.Off,
    flag(messages.On, [messages.Off]) == messages.Off,
    flag(messages.On, []) == messages.On)
}
"#,
        ),
    );
    assert_eq!(result, Value::Tuple(vec![Value::Bool(true); 4]));
}

#[test]
fn qualified_custom_fields_keep_typed_callbacks_through_local_wrappers() {
    let result = run_modules(
        "relay",
        DECLARATIONS,
        Some(
            r#"
fn total(values: List(Token)) -> Int {
  case values { [] -> 0 [a, b] -> messages.token_value(a) + messages.token_value(b) _ -> panic }
}
pub fn main() {
  let offset = 10
  let callback = fn(values) { total(values) + offset }
  let #(one, first) = envelope(One(Inspect(callback)))
  let assert One(Inspect(restored)) = first
  let #(many, second) = pick_envelope([Maybe(None), Many([Wrapped(Empty), Inspect(callback)])])
  let assert Many([Inspect(selected)]) = second
  let #(success, third) = envelope(Maybe(Some(#(Inspect(callback), Ok(messages.On)))))
  let assert Maybe(Some(#(Inspect(success_callback), Ok(messages.On)))) = third
  let #(failure, fourth) = envelope(Maybe(Some(#(Inspect(callback), Error(8)))))
  let assert Maybe(Some(#(Inspect(failure_callback), Error(8)))) = fourth
  let #(empty, fifth) = envelope(Maybe(None))
  #(one, restored == callback, restored([]), many, selected == callback,
    success, success_callback == callback, failure, failure_callback == callback,
    empty, fifth == Maybe(None))
}
"#,
        ),
    );
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(10.into()),
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(44.into()),
            Value::Bool(true),
            Value::Int(0.into()),
            Value::Bool(true),
        ])
    );
}
