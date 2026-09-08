import geam/future.{type Future}

@external(erlang, "native", "apply")
pub fn apply(callback: fn(Int) -> Int) -> Future(Int)
