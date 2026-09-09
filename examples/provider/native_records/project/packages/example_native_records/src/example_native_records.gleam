import gleam/dynamic.{type Dynamic}

pub type Key

pub type Record

@external(erlang, "geam_example_native_records", "key")
pub fn key(name: String) -> Key

@external(erlang, "geam_example_native_records", "record")
pub fn record(label: String, count: Int) -> Record

@external(erlang, "geam_example_native_records", "erase")
pub fn erase(value: a) -> Dynamic

@external(erlang, "geam_example_native_records", "map")
pub fn map(value: Dynamic, transform: fn(String, Int) -> a) -> Result(a, Nil)
