@external(erlang, "example_callables", "make_adder")
pub fn make_adder(offset: Int) -> fn(Int) -> Int

@external(erlang, "example_callables", "make_constant")
pub fn make_constant(value: item) -> fn() -> item

@external(erlang, "example_callables", "wrap")
pub fn wrap(callback: fn(argument) -> output) -> fn(argument) -> output

pub type Reply(item) {
  Reply(fn(item) -> Int)
}

@external(erlang, "example_callables", "reply")
pub fn reply(callback: fn(item) -> Int) -> Reply(item)

@external(erlang, "example_callables", "deliver")
pub fn deliver(reply: Reply(item), value: item) -> Int

@external(erlang, "example_callables", "calls")
pub fn calls() -> Int
