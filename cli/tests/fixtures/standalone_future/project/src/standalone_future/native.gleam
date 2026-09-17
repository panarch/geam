import geam/future.{type Future}

@external(erlang, "standalone_future", "timer")
pub fn timer() -> Future(Int)

@external(erlang, "standalone_future", "current")
pub fn current() -> Int

@external(erlang, "standalone_future", "apply")
pub fn apply(callback: fn(Int) -> Int) -> Int

@external(erlang, "standalone_future", "panic_driver")
pub fn panic_driver() -> Nil

@external(erlang, "standalone_future", "panic_worker")
pub fn panic_worker() -> Nil

@external(erlang, "standalone_future", "request")
pub fn request(address: String) -> Future(String)

@external(erlang, "standalone_future", "fail")
pub fn fail() -> Future(Nil)

@external(erlang, "standalone_future", "pending")
pub fn pending() -> Future(Nil)

@external(erlang, "standalone_future", "spawn_worker")
pub fn spawn_worker() -> Future(Nil)
