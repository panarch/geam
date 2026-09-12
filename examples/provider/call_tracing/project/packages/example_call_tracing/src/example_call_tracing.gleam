@external(erlang, "geam_example_call_tracing", "record")
pub fn record(entry: String) -> Nil

@external(erlang, "geam_example_call_tracing", "record_later")
pub fn record_later(entry: String) -> Nil

@external(erlang, "geam_example_call_tracing", "around")
pub fn around(callback: fn() -> item) -> item

@external(erlang, "geam_example_call_tracing", "entries")
pub fn entries() -> List(String)
