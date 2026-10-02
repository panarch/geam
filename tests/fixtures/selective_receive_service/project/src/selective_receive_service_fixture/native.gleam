import gleam/erlang/process.{type Pid}
import gleam/erlang/reference.{type Reference}

pub type Key

@external(erlang, "fixture", "key")
pub fn key(identity: a) -> Key

@external(erlang, "fixture", "send")
pub fn send(target: Pid, message: a) -> Nil

@external(erlang, "fixture", "receive")
pub fn receive(
  identity: a,
  forever: Bool,
) -> Result(#(String, Reference, Int), Nil)

@external(erlang, "fixture", "next")
pub fn next() -> Result(#(String, Reference, Int), Nil)
