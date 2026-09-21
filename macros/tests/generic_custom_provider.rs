use geam_core::execution::TokioHost;
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(package = "generic_customs", modules = [generic_customs, relay, manual_customs], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "generic_customs", crate_path = geam_core)]
mod generic_customs {
    use super::BigInt;
    use geam_core::provider::{Call, Callback, HostResult, Stored, Value};

    #[geam_macros::custom(input = FlagsInput)]
    pub enum Flags<Item> {
        Strategy(BigInt),
        Intensity(BigInt),
    }

    #[geam_macros::custom(input = MessageInput)]
    pub enum Message<Item> {
        Single(Value<Item>),
        Batch(Vec<Value<Item>>),
    }

    #[geam_macros::custom(input = StartedInput)]
    enum Started<Data> {
        Started {
            data: Value<Data>,
            flag: Flags<Data>,
        },
    }

    #[geam_macros::custom(input = ChildInput)]
    enum ChildSpecification<Data> {
        Child {
            start: Callback<fn() -> Result<StartedInput<Data>, bool>>,
        },
    }

    #[geam_macros::function(await)]
    async fn start<Data>(
        #[geam_macros::call] call: &mut Call<()>,
        child: ChildInput<Data>,
    ) -> HostResult<(Result<Started<Data>, bool>, ChildSpecification<Data>)> {
        let ChildInput::Child { start } = child;
        let returned = call.invoke(&start, ()).await?;
        let returned = returned.map(|StartedInput::Started { data, flag }| {
            let flag = match flag {
                FlagsInput::Strategy(value) => Flags::Strategy(value),
                FlagsInput::Intensity(value) => Flags::Intensity(value),
            };
            Started::Started { data, flag }
        });
        Ok((returned, ChildSpecification::Child { start }))
    }

    #[geam_macros::external(name = "Subject", parameters = [Data], input = SubjectInput)]
    pub struct Subject<Data> {
        #[geam_macros::stored]
        value: Stored<Data>,
    }

    #[geam_macros::function]
    fn subject<Data>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Data>,
    ) -> Subject<Data> {
        Subject {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    fn received<Data>(
        #[geam_macros::call] call: &mut Call<()>,
        subject: SubjectInput<Data>,
    ) -> Value<Data> {
        call.restore(subject.value())
    }

    #[geam_macros::function]
    fn opaque_subject<Item>(value: Value<Subject<Item>>) -> Value<Subject<Item>> {
        value
    }

    #[geam_macros::function]
    fn opaque_subject_result<Item>(
        value: Value<Result<Subject<Item>, bool>>,
    ) -> Value<Result<Subject<Item>, bool>> {
        value
    }

    #[geam_macros::custom(input = ProcessStartedInput)]
    enum ProcessStarted<Data> {
        Running(Subject<Data>),
        Waiting(Vec<Subject<Data>>),
    }

    #[geam_macros::custom(input = ProcessChildInput)]
    enum ProcessChild<Data> {
        ProcessChild(Callback<fn() -> Result<ProcessStartedInput<Data>, bool>>),
    }

    #[geam_macros::function(await)]
    async fn start_process<Data>(
        #[geam_macros::call] call: &mut Call<()>,
        child: ProcessChildInput<Data>,
    ) -> HostResult<(Result<ProcessStarted<Data>, bool>, ProcessChild<Data>)> {
        let ProcessChildInput::ProcessChild(start) = child;
        let returned = call.invoke(&start, ()).await?;
        let returned = returned.map(|value| {
            let value = match value {
                ProcessStartedInput::Running(subject) => call.restore(subject.value()),
                ProcessStartedInput::Waiting(subjects) => {
                    let subject = subjects.get(1).expect("fixture supplies two subjects");
                    drop(subjects);
                    call.restore(subject.value())
                }
            };
            ProcessStarted::Running(Subject {
                value: call.store(value),
            })
        });
        Ok((returned, ProcessChild::ProcessChild(start)))
    }

    #[geam_macros::function]
    fn selected_subject<Data>(
        #[geam_macros::call] call: &mut Call<()>,
        started: ProcessStartedInput<Data>,
    ) -> ProcessStarted<Data> {
        let value = match started {
            ProcessStartedInput::Running(subject) => call.restore(subject.value()),
            ProcessStartedInput::Waiting(subjects) => {
                let subject = subjects.get(0).expect("fixture supplies a subject");
                drop(subjects);
                call.restore(subject.value())
            }
        };
        ProcessStarted::Waiting(vec![Subject {
            value: call.store(value),
        }])
    }

    #[geam_macros::custom(input = HandlerInput)]
    enum Handler<Argument, Data> {
        Opaque(Value<fn(Argument) -> Data>),
    }

    #[geam_macros::function]
    fn handler<Argument, Data>(callback: Value<fn(Argument) -> Data>) -> Handler<Argument, Data> {
        Handler::Opaque(callback)
    }

    #[geam_macros::function]
    fn take_handler<Argument, Data>(
        handler: HandlerInput<Argument, Data>,
    ) -> Value<fn(Argument) -> Data> {
        let HandlerInput::Opaque(callback) = handler;
        callback
    }

    #[geam_macros::function]
    fn fixed() -> Flags<(geam_core::StringValue, BigInt)> {
        Flags::Intensity(27.into())
    }

    #[geam_macros::custom(input = PropertyInput)]
    pub enum Property<Argument, Data> {
        Map(Callback<fn(Value<Argument>) -> Value<Data>>),
        Starts {
            flag: Flags<Data>,
            callbacks: Vec<(Value<Argument>, Callback<fn() -> Value<Data>>)>,
        },
    }

    #[geam_macros::function(await)]
    async fn apply<Argument, Data>(
        #[geam_macros::call] call: &mut Call<()>,
        property: PropertyInput<Argument, Data>,
        argument: Value<Argument>,
    ) -> HostResult<(Value<Data>, Property<Argument, Data>)> {
        match property {
            PropertyInput::Map(callback) => {
                let result = call.invoke(&callback, (argument,)).await?;
                Ok((result, Property::Map(callback)))
            }
            PropertyInput::Starts { flag, callbacks } => {
                let (value, callback) = callbacks
                    .get(1)
                    .expect("fixture supplies two start functions");
                drop(callbacks);
                let result = call.invoke(&callback, ()).await?;
                let flag = match flag {
                    FlagsInput::Strategy(value) => Flags::Strategy(value),
                    FlagsInput::Intensity(value) => Flags::Intensity(value),
                };
                Ok((
                    result,
                    Property::Starts {
                        flag,
                        callbacks: vec![(value, callback)],
                    },
                ))
            }
        }
    }

    #[geam_macros::function]
    fn message<Host>(value: Value<Host>) -> Message<Host> {
        Message::Single(value)
    }

    #[geam_macros::function]
    fn opaque_message<Item>(value: Value<Message<Item>>) -> Value<Message<Item>> {
        value
    }

    #[geam_macros::function]
    fn nested_message<Item>(
        values: geam_core::List<geam_core::List<MessageInput<Item>>>,
    ) -> Value<Item> {
        let selected = values.get(1).expect("fixture supplies two rows");
        drop(values);
        let selected = selected.get(0).expect("selected row is nonempty");
        unpack(selected)
    }

    #[geam_macros::function]
    fn unpack<Item>(value: MessageInput<Item>) -> Value<Item> {
        match value {
            MessageInput::Single(value) => value,
            MessageInput::Batch(values) => values.get(1).expect("fixture sends two items"),
        }
    }

    #[geam_macros::function]
    fn batch<Item>(first: Value<Item>, second: Value<Item>) -> Message<Item> {
        Message::Batch(vec![first, second])
    }

    #[geam_macros::function]
    fn intensity<Item>(value: BigInt) -> Flags<Item> {
        if value < 0.into() {
            Flags::Strategy(-value)
        } else {
            Flags::Intensity(value)
        }
    }

    #[geam_macros::function]
    fn read<Item>(value: FlagsInput<Item>) -> BigInt {
        match value {
            FlagsInput::Strategy(value) => -value,
            FlagsInput::Intensity(value) => value,
        }
    }

    #[geam_macros::function]
    fn first<Item>(values: geam_core::List<FlagsInput<Item>>) -> BigInt {
        values.get(0).map_or_else(|| 0.into(), read::<Item>)
    }
}

#[geam_macros::module(path = "relay", crate_path = geam_core)]
mod relay {
    use super::{generic_customs, manual_customs};
    use geam_core::provider::{Call, HostResult, Value};

    #[geam_macros::custom(input = WrapperInput)]
    enum Wrapper<Argument, Data> {
        Wrapped(generic_customs::Property<Argument, Data>),
    }

    #[geam_macros::function(await)]
    async fn invoke<Argument, Data>(
        #[geam_macros::call] call: &mut Call<()>,
        value: WrapperInput<Argument, Data>,
        argument: Value<Argument>,
    ) -> HostResult<(Value<Data>, Wrapper<Argument, Data>)> {
        let WrapperInput::Wrapped(value) = value;
        let (result, value) = match value {
            generic_customs::PropertyInput::Map(callback) => {
                let result = call.invoke(&callback, (argument,)).await?;
                (result, generic_customs::Property::Map(callback))
            }
            generic_customs::PropertyInput::Starts { flag, callbacks } => {
                let (argument, callback) =
                    callbacks.get(0).expect("fixture supplies a start function");
                drop(callbacks);
                let result = call.invoke(&callback, ()).await?;
                let flag = match flag {
                    generic_customs::FlagsInput::Strategy(value) => {
                        generic_customs::Flags::Strategy(value)
                    }
                    generic_customs::FlagsInput::Intensity(value) => {
                        generic_customs::Flags::Intensity(value)
                    }
                };
                (
                    result,
                    generic_customs::Property::Starts {
                        flag,
                        callbacks: vec![(argument, callback)],
                    },
                )
            }
        };
        Ok((result, Wrapper::Wrapped(value)))
    }

    #[geam_macros::custom(input = ReceiptInput)]
    enum Receipt<Item> {
        Receipt(Vec<manual_customs::Packet<Item>>),
    }

    #[geam_macros::function]
    fn relay_packet<Item>(receipt: ReceiptInput<Item>) -> Receipt<Item> {
        let ReceiptInput::Receipt(packets) = receipt;
        let packet = packets.get(0).expect("fixture supplies a packet");
        drop(packets);
        Receipt::Receipt(vec![packet.into_value()])
    }

    #[geam_macros::function]
    fn pick<Item>(values: geam_core::List<generic_customs::MessageInput<Item>>) -> Value<Item> {
        let value = values.get(1).expect("fixture supplies two messages");
        drop(values);
        match value {
            generic_customs::MessageInput::Single(value) => value,
            generic_customs::MessageInput::Batch(values) => {
                values.get(0).expect("fixture supplies a message")
            }
        }
    }
}

#[geam_macros::module(path = "manual_customs", crate_path = geam_core)]
mod manual_customs {
    use geam_core::provider::advanced::{
        Equality, Hashing, Index0, Inspection, Retained, RetainedExternalPayload,
    };
    use geam_core::provider::{Call, Callback, HostResult, Value};
    use std::cell::Cell;

    pub struct Payload {
        value: Retained<Self, Index0>,
        reads: Cell<usize>,
    }

    impl RetainedExternalPayload for Payload {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }
        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }
        fn inspect(&self, context: &Inspection<'_>) -> ecow::EcoString {
            self.value.inspect(context)
        }
    }

    #[geam_macros::external(name = "Packet", parameters = [Item], input = PacketInput, payload = Payload, manual)]
    pub struct Packet<Item>;

    #[geam_macros::function]
    fn packet<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> Packet<Item> {
        Packet::from_payload(Payload {
            value: call.store(value).into_retained(),
            reads: Cell::new(0),
        })
    }

    #[geam_macros::function]
    fn reads<Item>(value: PacketInput<Item>) -> num_bigint::BigInt {
        value.payload().reads.get().into()
    }

    #[geam_macros::custom(input = EnvelopeInput)]
    pub enum Envelope<Item> {
        One(Packet<Item>),
        Many(Vec<Packet<Item>>),
        Start(Callback<fn() -> PacketInput<Item>>),
    }

    #[geam_macros::function(await)]
    async fn open<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        envelope: EnvelopeInput<Item>,
    ) -> HostResult<(Value<Item>, Envelope<Item>)> {
        let packet = match envelope {
            EnvelopeInput::One(packet) => packet,
            EnvelopeInput::Many(packets) => {
                let packet = packets.get(1).expect("fixture supplies two packets");
                drop(packets);
                packet
            }
            EnvelopeInput::Start(start) => call.invoke(&start, ()).await?,
        };
        packet.with_payload(|payload| payload.reads.set(payload.reads.get() + 1));
        let value = call.restore(packet.stored_item(|payload| &payload.value));
        Ok((value, Envelope::One(packet.into_value())))
    }

    #[geam_macros::function]
    fn forward<Item>(envelope: EnvelopeInput<Item>) -> Envelope<Item> {
        match envelope {
            EnvelopeInput::One(packet) => Envelope::Many(vec![packet.into_value()]),
            EnvelopeInput::Many(packets) => {
                let packet = packets.get(0).expect("fixture supplies a packet");
                drop(packets);
                Envelope::One(packet.into_value())
            }
            EnvelopeInput::Start(start) => Envelope::Start(start),
        }
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
fn phantom_custom_parameters_keep_their_nominal_identity_through_direct_and_list_views() {
    let value = run(r#"
fn int_flag(value: Flags(Int)) -> Int { read(value) }
fn string_flags(values: List(Flags(String))) -> Int { first(values) }
pub fn main() {
  #(int_flag(intensity(17)), string_flags([intensity(23)]),
    int_flag(Strategy(31)), string_flags([Strategy(41)]), string_flags([]))
}
"#);
    assert_eq!(value.inspect().to_string(), "#(17, 23, -31, -41, 0)");
}

#[test]
fn generic_custom_fields_retain_opaque_values_without_erasing_source_types() {
    let value = run(r#"
pub fn main() {
  let callback = fn(x) { x + 7 }
  let wrapped = opaque_message(Batch([fn(x) { x + 1 }, callback]))
  let retained = unpack(wrapped)
  let selected = nested_message([[message(fn(x) { x })], [wrapped]])
  #(unpack(opaque_message(message(17))), unpack(batch("first", "second")),
    retained(10), selected == callback, selected(20))
}
"#);
    assert_eq!(
        value.inspect().to_string(),
        "#(17, \"second\", 17, True, 27)"
    );
}

#[test]
fn independent_custom_parameters_preserve_callable_fields_and_nested_lists() {
    let value = run(r#"
pub fn main() {
  let mapper = fn(x) { #(x + 3, "mapped") }
  let #(mapped, returned) = apply(Map(mapper), 7)
  let assert Map(alias) = returned
  let start = fn() { "started" }
  let property = Starts(Intensity(5), [#(0, fn() { "unused" }), #(1, start)])
  let #(started, returned) = apply(property, 9)
  let assert Starts(flag, [#(argument, alias_start)]) = returned
  #(mapped, alias == mapper, alias(17), started, read(flag), argument, alias_start == start, alias_start())
}
"#);
    assert_eq!(
        value.inspect().to_string(),
        r#"#(#(10, "mapped"), True, #(20, "mapped"), "started", 5, 1, True, "started")"#
    );
}

fn run(body: &str) -> Value {
    run_module("generic_customs", body)
}

fn run_module(entry: &str, body: &str) -> Value {
    let source = if entry == "generic_customs" {
        format!("{DECLARATIONS}\n{body}")
    } else {
        DECLARATIONS.to_owned()
    };
    let relay = if entry == "relay" {
        format!("{RELAY_DECLARATIONS}\n{body}")
    } else {
        RELAY_DECLARATIONS.to_owned()
    };
    let manual = if entry == "manual_customs" {
        format!("{MANUAL_DECLARATIONS}\n{body}")
    } else {
        MANUAL_DECLARATIONS.to_owned()
    };
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers).unwrap();
    let typed = compile_typed_host_program(
        "generic_customs",
        entry,
        [PackageSource::new(
            "generic_customs",
            Vec::<&str>::new(),
            [
                ModuleSource::new("generic_customs", "src/generic_customs.gleam", &source),
                ModuleSource::new("relay", "src/relay.gleam", &relay),
                ModuleSource::new("manual_customs", "src/manual_customs.gleam", &manual),
            ],
        )],
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
pub type Flags(item) { Strategy(Int) Intensity(Int) }
@external(erlang, "generic_customs", "intensity")
pub fn intensity(value: Int) -> Flags(item)
@external(erlang, "generic_customs", "read")
pub fn read(value: Flags(item)) -> Int
@external(erlang, "generic_customs", "first")
pub fn first(values: List(Flags(item))) -> Int
pub type Message(item) { Single(item) Batch(List(item)) }
@external(erlang, "generic_customs", "message")
pub fn message(value: item) -> Message(item)
@external(erlang, "generic_customs", "opaque_message")
pub fn opaque_message(value: Message(item)) -> Message(item)
@external(erlang, "generic_customs", "nested_message")
pub fn nested_message(values: List(List(Message(item)))) -> item
@external(erlang, "generic_customs", "unpack")
pub fn unpack(value: Message(item)) -> item
@external(erlang, "generic_customs", "batch")
pub fn batch(first: item, second: item) -> Message(item)
pub type Property(argument, data) {
  Map(fn(argument) -> data)
  Starts(flag: Flags(data), callbacks: List(#(argument, fn() -> data)))
}
@external(erlang, "generic_customs", "apply")
pub fn apply(property: Property(argument, data), argument: argument) -> #(data, Property(argument, data))
pub type Started(data) { Started(data: data, flag: Flags(data)) }
pub type ChildSpecification(data) { Child(start: fn() -> Result(Started(data), Bool)) }
@external(erlang, "generic_customs", "start")
pub fn start(child: ChildSpecification(data)) -> #(Result(Started(data), Bool), ChildSpecification(data))
pub type Subject(data)
@external(erlang, "generic_customs", "subject")
pub fn subject(value: data) -> Subject(data)
@external(erlang, "generic_customs", "received")
pub fn received(subject: Subject(data)) -> data
@external(erlang, "generic_customs", "opaque_subject")
pub fn opaque_subject(value: Subject(item)) -> Subject(item)
@external(erlang, "generic_customs", "opaque_subject_result")
pub fn opaque_subject_result(value: Result(Subject(item), Bool)) -> Result(Subject(item), Bool)
pub type ProcessStarted(data) { Running(Subject(data)) Waiting(List(Subject(data))) }
pub type ProcessChild(data) { ProcessChild(fn() -> Result(ProcessStarted(data), Bool)) }
@external(erlang, "generic_customs", "start_process")
pub fn start_process(child: ProcessChild(data)) -> #(Result(ProcessStarted(data), Bool), ProcessChild(data))
@external(erlang, "generic_customs", "selected_subject")
pub fn selected_subject(started: ProcessStarted(data)) -> ProcessStarted(data)
pub type Handler(argument, data) { Opaque(fn(argument) -> data) }
@external(erlang, "generic_customs", "handler")
pub fn handler(callback: fn(argument) -> data) -> Handler(argument, data)
@external(erlang, "generic_customs", "take_handler")
pub fn take_handler(handler: Handler(argument, data)) -> fn(argument) -> data
@external(erlang, "generic_customs", "fixed")
pub fn fixed() -> Flags(#(String, Int))
"#;

const RELAY_DECLARATIONS: &str = r#"
import generic_customs.{type Message, type Property, Map, Starts, Single, Batch, Intensity}
import manual_customs.{type Packet}
pub type Receipt(item) { Receipt(List(Packet(item))) }
@external(erlang, "relay", "relay_packet")
pub fn relay_packet(receipt: Receipt(item)) -> Receipt(item)
pub type Wrapper(argument, data) { Wrapped(Property(argument, data)) }
@external(erlang, "relay", "invoke")
pub fn invoke(value: Wrapper(argument, data), argument: argument) -> #(data, Wrapper(argument, data))
@external(erlang, "relay", "pick")
pub fn pick(values: List(Message(item))) -> item
"#;

#[test]
fn qualified_generic_declarations_keep_original_callback_codecs_and_nominal_schemas() {
    let value = run_module(
        "relay",
        r#"
pub fn main() {
  let mapper = fn(x) { #(x + 11, "cross-module") }
  let #(result, Wrapped(returned)) = invoke(Wrapped(Map(mapper)), 7)
  let assert Map(alias) = returned
  let start = fn() { "started" }
  let #(started, Wrapped(returned_start)) = invoke(Wrapped(Starts(Intensity(5), [#(19, start)])), 1)
  let assert Starts(flag, [#(argument, alias_start)]) = returned_start
  #(result, alias == mapper, alias(10), started, generic_customs.read(flag), argument,
    alias_start == start, pick([Single("unused"), Batch(["retained"])]))
}
"#,
    );
    assert_eq!(
        value.inspect().to_string(),
        r#"#(#(18, "cross-module"), True, #(21, "cross-module"), "started", 5, 19, True, "retained")"#
    );
}

#[test]
fn generic_start_callbacks_preserve_nominal_result_types_and_source_failures() {
    let value = run(r#"
pub fn main() {
  let boot = fn() { Ok(Started("ready", Intensity(7))) }
  let #(success, Child(alias)) = start(Child(boot))
  let assert Ok(Started(data, flag)) = success
  let fail: fn() -> Result(Started(Int), Bool) = fn() { Error(True) }
  let #(failure, Child(failure_alias)) = start(Child(fail))
  let callback = fn(x) { #(x + 3, "opaque") }
  let opaque_alias = take_handler(handler(callback))
  #(data, read(flag), alias == boot, failure, failure_alias == fail,
    opaque_alias == callback, opaque_alias(9), read(fixed()))
}
"#);
    assert_eq!(
        value.inspect().to_string(),
        r#"#("ready", 7, True, Error(True), True, True, #(12, "opaque"), 27)"#
    );
}

#[test]
fn generic_external_fields_preserve_nominal_types_through_custom_callbacks_and_lists() {
    let value = run(r#"
pub fn main() {
  let callback = fn(value) { value + 13 }
  let boot = fn() { Ok(Running(subject(callback))) }
  let #(running, ProcessChild(alias)) = start_process(ProcessChild(boot))
  let assert Ok(Running(retained)) = running
  let assert Ok(retained) = opaque_subject_result(Ok(opaque_subject(retained)))
  let restored = received(retained)
  let #(waiting, _) = start_process(ProcessChild(fn() { Ok(Waiting([subject("unused"), subject("ready")])) }))
  let assert Ok(Running(ready)) = waiting
  let assert Waiting([selected]) = selected_subject(Running(subject(23)))
  let assert Waiting([selected_again]) = selected_subject(Waiting([subject(29)]))
  let fail: fn() -> Result(ProcessStarted(Int), Bool) = fn() { Error(True) }
  let #(failure, _) = start_process(ProcessChild(fail))
  #(alias == boot, restored == callback, restored(7), received(ready), received(selected), received(selected_again), failure)
}
"#);
    assert_eq!(
        value.inspect().to_string(),
        r#"#(True, True, 20, "ready", 23, 29, Error(True))"#
    );
}

const MANUAL_DECLARATIONS: &str = r#"
pub type Packet(item)
@external(erlang, "manual_customs", "packet")
pub fn packet(value: item) -> Packet(item)
@external(erlang, "manual_customs", "reads")
pub fn reads(value: Packet(item)) -> Int
pub type Envelope(item) { One(Packet(item)) Many(List(Packet(item))) Start(fn() -> Packet(item)) }
@external(erlang, "manual_customs", "open")
pub fn open(envelope: Envelope(item)) -> #(item, Envelope(item))
@external(erlang, "manual_customs", "forward")
pub fn forward(envelope: Envelope(item)) -> Envelope(item)
"#;

#[test]
fn manual_generic_payloads_keep_one_owner_through_custom_lists_and_qualified_forms() {
    let value = run_module(
        "relay",
        r#"
pub fn main() {
  let original = manual_customs.packet("retained")
  let assert Receipt([relayed]) = relay_packet(Receipt([original]))
  let assert #(value, manual_customs.One(alias)) = manual_customs.open(manual_customs.One(relayed))
  let #(selected, _) = manual_customs.open(manual_customs.Many([manual_customs.packet("unused"), original]))
  let boot = fn() { original }
  let #(started, _) = manual_customs.open(manual_customs.forward(manual_customs.Start(boot)))
  let assert manual_customs.Many([forwarded]) = manual_customs.forward(manual_customs.One(alias))
  let assert manual_customs.One(forwarded_again) = manual_customs.forward(manual_customs.Many([forwarded]))
  #(value, selected, started, manual_customs.reads(original), manual_customs.reads(forwarded_again), original == alias)
}
"#,
    );
    assert_eq!(
        value.inspect().to_string(),
        r#"#("retained", "retained", "retained", 3, 3, True)"#
    );
}
