import geam/future.{type Future}

@external(erlang, "standalone_future", "timer")
pub fn timer() -> Future(Int)

@external(erlang, "standalone_future", "current")
pub fn current() -> Int

@external(erlang, "standalone_future", "request")
pub fn request(address: String) -> Future(String)

@external(erlang, "standalone_future", "fail")
pub fn fail() -> Future(Nil)

@external(erlang, "standalone_future", "pending")
pub fn pending() -> Future(Nil)

@external(erlang, "standalone_future", "spawn_worker")
pub fn spawn_worker() -> Future(Nil)
