//// Compose work from Rust async functions. The Rust application hosting Geam
//// drives that work and reads its shared result.
////
//// These operations are currently implemented by Geam. Erlang and JavaScript
//// implementations are not currently available.

/// One operation and its shared result. Copying a Future shares that operation.
@external(erlang, "geam_future", "Future")
pub type Future(value)

/// Creates a Future whose result is already available.
@external(erlang, "geam_future", "ready")
pub fn ready(value: value) -> Future(value)

/// Transforms the result when the Rust host drives the returned Future.
@external(erlang, "geam_future", "map")
pub fn map(value: Future(a), callback: fn(a) -> b) -> Future(b)

/// Continues with the Future constructed by the callback.
pub fn then(value: Future(a), callback: fn(a) -> Future(b)) -> Future(b) {
  flatten(map(value, callback))
}

@external(erlang, "geam_future", "flatten")
fn flatten(value: Future(Future(a))) -> Future(a)

/// Drives the supplied work together and keeps results in input order.
@external(erlang, "geam_future", "all")
pub fn all(values: List(Future(a))) -> Future(List(a))
