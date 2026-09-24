# Execution services

An execution domain can own several statically selected services. Providers
that require the same producer use one instance of its service. Starting a new
execution domain creates fresh service state; retaining a source value does not
keep the domain alive.

`HostProviderComponent` continues to own external stores and caller-supplied
run state. A component that also provides a domain service implements
`HostExecutionService`:

```rust
pub trait HostExecutionService: HostProviderComponent {
    type State: HostExecutionState;
    fn initialize_service(state: &mut Self::RunState) -> Self::State;
}
```

Component initialization parses explicit configuration before execution.
`initialize_service` creates each domain's state from that initialized run
state. Preparing source or a prepared artifact does neither. The execution
owner subsequently delivers `initialize`, `started`, `finished`, `poll`, and
`close` to the service.

`HostServiceProfile<Producer>` projects the producer's state from the concrete
profile. An immediate provider `Call` accesses it with
`call.service::<Producer>()`; a resumable call uses
`call.with_call(|call| { ... }).await`. The bounded operation must release its
service borrow before invoking source callbacks, awaiting, or waking work.

Manual hosts compose states with
`ExecutionServices<First, ExecutionServices<Second, ()>>`. Both owners receive
each event, and both are polled even when the first reports progress. Each
service must bound its own poll work and register its required wake. Projection
is a static field access; execution does not search a service registry.

## Declaring composition

Existing schema 1 metadata remains valid for components without service
dependencies. Schema 2 declares the component's Rust shape and service needs:

```toml
[package.metadata.geam.provider]
schema = 2
gleam-package = "example_process_service"
gleam-version = ">= 0.1.0 and < 0.2.0"
component = "plain"
execution-service = false
requires-services = ["gleam_erlang"]
```

All six fields are required. `component = "plain"` selects `Component`;
`"profile"` selects `Component<Profile>`, including stores that retain typed
callbacks for the final profile. `execution-service = true` means that component
implements `HostExecutionService`. `requires-services` names producer Gleam
packages. It is independent of whether the selected source directly calls a
producer's native functions.

The standalone runner uses explicitly selected provider dependencies. Generated
embedding selects enabled direct Cargo providers. Both validate producer
ownership and generate concrete state, initialization, and projections. Shared
dependencies are initialized once. A missing provider or a dependency that does
not provide a service fails before application execution. Ordinary Cargo aliases,
features, configuration, and lockfile ownership still apply.

## Sharing the Erlang process service

Enable `geam`'s `gleam-erlang` feature in the consumer. The producer owns its
schemas, bindings, process identities, names, mailbox, and lifecycle. Consumer
providers do not define replacement `Pid`, `Name`, or reference stores.

Macro-authored functions use the producer's narrow types and `ProcessCall`:

```rust
use geam::gleam_erlang::service::{self, ProcessCall};
use geam::provider::Call;

#[geam::function(profile = Profile)]
fn lookup<Message>(
    #[geam::call] call: &mut Call<()>,
    name: service::Name<Message>,
) -> Option<service::Pid> {
    call.named(&name)
}
```

The surrounding module declares
`profile = geam::gleam_erlang::GleamErlangHostProfile`, its component, and
`crate_path = geam`. As with other cross-module provider declarations, use a
qualified path such as `service::Name<Message>` so the macro recognizes its
nominal type arguments. The specialization is preserved even when the message
argument is phantom.

`service::Pid`, `service::Name<Message>`, and `service::Reference` retain original
producer payloads on input and preserve them on return. Cloning shares the
retained handle. Lists decode only requested items. A newly returned process
identity or reference is constructed through the producer binding.

`service::Subject<Message>` also preserves the original source value and exact
message specialization. Its representation stays private to the producer adapter.
The process provider explicitly shares the low-level `Subject` schema with
native consumers; ordinary Gleam opacity remains unchanged.

Low-level typed registrations use `service::Processes` with their `HostCall`,
the producer-owned schemas, and exact registered construction tokens. Creating
the facade does not require a current process. Lookup, registration, delivery,
and timers also work in an owned native Future after its source invocation ends.
Current-process operations, including links, monitors, exit signals and receives,
fail if the call has no source invocation. Both authoring and typed calls use the
same producer implementation and domain state. A `Receive`
belongs to its original unit; waiting from another unit fails before consuming
a message. Waiting uses the existing bounded scanner and original execution
context. Unmatched messages remain queued, and the queued snapshot is checked
before a supplied timeout. Dropping an unstarted receive consumes nothing.

For several process-relative operations in one typed call, enter
`CurrentProcess::with(call, |process| ...)`. The producer checks the source
identity once and keeps it with that exact call. `with_current_process` performs
the same bounded operation through a `HostExecutionContext`, preserving its
registered construction permissions and cancellation. This is an identity
proof, not a mailbox-liveness guarantee: sending an exit signal can close the
current mailbox in the same operation, and receive preparation still reports
that error. `resume_receive` prepares a tagged receive before transferring the
call to its continuation, preserving the queued snapshot without another host
request.

`CurrentProcess::receive_record_with` combines tagged record matching and field
projection. Its bounded Rust function returns `Some(fields)` to select the
message or `None` to leave it queued. The function has no host-call access and
cannot retain call-scoped views. The returned `RecordReceive` yields those exact
fields; consumers do not need to parse a record they have already matched. This
uses the same mailbox scanner, receiver check, timeout ordering, and cancellation
as other receives. Source callbacks continue to use the producer's selector
and its registered callable permissions.

Async macro providers create a receive inside `call.with_call` using
`ProcessCall::receive_tagged` or `receive_any`, then use
`receive.wait_in(call).await`. The timeout is measured by the host clock.
For a service loop without a timeout result, use
`receive.wait_forever_in(call).await` (`wait_forever(&context)` in a typed
registration). This returns a message or an execution error and discards any
deadline supplied when preparing that receive.
These operations retain the original receiver and release the active call
borrow before waiting. The selected `NativeValue` can be returned as a producer
`Dynamic` or restored at its exact source type with `Call::restore_native`.

The [process provider example](../../examples/provider/process_service) and the
[OTP integration fixture](../../tests/fixtures/otp_service) exercise external
consumers. The latter checks pinned upstream source and retained callbacks; it
is not a distributable OTP provider or a claim that every OTP policy is
implemented.

## Constructing Erlang Charlists

Manual providers can construct the original `gleam/erlang/charlist.Charlist`
with `geam::gleam_erlang::service::charlist_from_string`. Enable the
`gleam-erlang` feature and compose the original Erlang component in the host
profile. The function accepts a mutable `HostCall`, exact construction tokens
for `Charlist` and `HostListType<char>`, and borrowed Rust text (`&str`).

Declare both constructions on the calling function, including when Charlist is
its exact return type:

```rust
use geam::gleam_erlang::{Charlist, service};
use geam::host::{HostListType, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd};

type Constructions = HostTypeList<
    Charlist,
    HostTypeList<HostListType<char>, HostTypeListEnd>,
>;

// Inside a callback registered with with_scoped_function_and_constructions:
let value = service::charlist_from_string(
    &mut call,
    constructions.at::<HostTypeIndex0>(),
    constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
    &text,
);
Ok(call.return_value(value))
```

The producer retains the character list through its original binding and store.
Consumers do not declare a replacement Charlist schema, implement its storage
adapter, or call hidden binding-aware construction methods. Text is traversed
as Unicode scalar values without normalization; the value does not borrow the
input text. The host handle remains call-scoped, while a returned source value
retains its payload through the ordinary external-value lifetime.

The same function can construct a Charlist inside a tuple or list return. Declare
any additional intermediate containers in that registration's construction
sequence and build them with the existing typed host methods. Construction
does not require a new execution service, mailbox, or asynchronous wait.
Original `charlist.to_string`, source equality and hashing, and native
integer-list views continue to use the producer's representation.

The independent [Charlist service fixture](../../tests/fixtures/charlist_service)
contains a complete manual component, exact registration sequences, and the host
profile. It executes direct, header-pair, and nested status/header returns against
original `gleam_erlang` source. This boundary supports external provider authors;
the fixture does not implement HTTP requests or extend macro return mappings.

## Constructing standard-library Dicts

Manual providers can construct the original `gleam/dict.Dict(key, item)` through
`geam::gleam_stdlib::service::dict_from_entries`. Enable `provider` and
`gleam-stdlib`, and compose the standard-library component in the host profile.
The profile implements `GleamStdlibHostProfile` and
`HostComponentProfile<geam::gleam_stdlib::Component<Profile::Io>>`.

`DictOf<Key, Item>` accepts existing `HostType` arguments such as `StringValue`,
`BigInt` and `HostListType<StringValue>`. Register an exact Dict construction
on the native function, then pass its token and an iterator of typed pairs:

```rust
use geam::gleam_stdlib::{DictOf, service};
use geam::host::{HostTypeIndex0, HostTypeList, HostTypeListEnd};
use geam::provider::StringValue;

type TextDict = DictOf<StringValue, StringValue>;
type Constructions = HostTypeList<TextDict, HostTypeListEnd>;

// Inside a callback registered with with_scoped_function_and_constructions:
let entries = [("LANG", "한국어"), ("EMPTY", "")]
    .map(|(key, item)| (StringValue::from(key), StringValue::from(item)));
let dict = service::dict_from_entries(
    &mut call,
    constructions.at::<HostTypeIndex0>(),
    entries,
);
Ok(call.return_value(dict))
```

The exact Dict token authorizes retention of its key and item types. Generic
parameters must already be bound by the registered function's signature.
Additional intermediate containers need their own construction tokens; existing
typed values do not. The return type can contain Dicts inside tuples or Lists.
The producer owns binding and storage, so consumers need no storage adapter,
replacement schema or hidden API.

The iterator is consumed once. Equal keys use Gleam source semantics, and the
last pair wins, matching `dict.from_list`. Hash collisions preserve distinct
keys. Dict iteration order is unspecified. This new service does not change the
existing first-wins behavior of JSON object decoding.

The immutable payload retains its entries independently of the input container,
and `dict.insert`/`delete` preserve earlier aliases. The returned host handle is
call-scoped; ordinary source data can outlive execution through its retained
value. Values with an execution lifetime keep that restriction when nested in a
Dict. Construction requires no new execution service or asynchronous operation.

The independent [Dict fixture](../../tests/fixtures/dict_service) provides the
complete manual registration and host profile. It exercises String pairs,
integer keys with List items, and Dicts nested in a tuple and List through
unchanged stdlib source. It establishes the construction boundary needed by
external providers, without implementing environment-variable policy or
extending provider macro return mappings.
