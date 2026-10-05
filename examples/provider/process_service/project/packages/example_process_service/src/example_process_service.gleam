import gleam/erlang/process.{type Name, type Subject}
import gleam/option.{type Option, None, Some}

pub type RequestError {
  Unavailable
  TimedOut
  TargetExited
  InvalidTimeout
  InvalidReply
}

/// Send a request to the process registered under `name` and await its reply.
/// A fresh reply subject keeps concurrent requests separate. The deadline uses
/// the application's host clock. If no reply arrives, a process that has exited
/// is distinguished from a live process that did not respond by the deadline.
/// A zero timeout only accepts a reply already queued when receiving starts.
pub fn request(
  name: Name(message),
  make_message: fn(Subject(reply)) -> message,
  timeout_ms: Int,
) -> Result(reply, RequestError) {
  let reply = process.new_subject()
  exchange(
    name,
    process.named_subject(name),
    make_message(reply),
    reply,
    Some(timeout_ms),
  )
}

/// Send a request without a deadline and wait for its reply.
/// A missing name returns Unavailable. After sending, the wait continues until
/// a reply arrives or the enclosing execution is cancelled.
pub fn request_forever(
  name: Name(message),
  make_message: fn(Subject(reply)) -> message,
) -> Result(reply, RequestError) {
  let reply = process.new_subject()
  exchange(name, process.named_subject(name), make_message(reply), reply, None)
}

@external(erlang, "example_process_service_native", "exchange")
fn exchange(
  name: Name(message),
  destination: Subject(message),
  message: message,
  reply: Subject(reply),
  timeout_ms: Option(Int),
) -> Result(reply, RequestError)
