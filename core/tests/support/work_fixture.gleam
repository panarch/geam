pub type Work(value)

@external(erlang, "fixture", "ready")
pub fn ready(value: value) -> Work(value)

@external(erlang, "fixture", "map")
pub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)

@external(erlang, "fixture", "flatten")
pub fn flatten(value: Work(Work(a))) -> Work(a)

@external(erlang, "fixture", "all")
pub fn all(values: List(Work(a))) -> Work(List(a))

pub fn then(value: Work(a), callback: fn(a) -> Work(b)) -> Work(b) {
  flatten(map(value, callback))
}
