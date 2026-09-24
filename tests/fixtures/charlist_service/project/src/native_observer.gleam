// Implemented by the Rust integration target using only public native views.
@external(erlang, "native_observer", "assert_equal")
pub fn assert_equal(actual: a, expected: b) -> Nil
