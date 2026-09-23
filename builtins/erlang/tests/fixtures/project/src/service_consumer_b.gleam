import gleam/erlang/process.{type Name, type Pid, type Subject}
import gleam/erlang/reference.{type Reference}
import gleam/option.{type Option}

@external(erlang, "consumer_b", "ticket")
pub fn ticket() -> Int

@external(erlang, "consumer_b", "pid")
pub fn pid(pid: Pid) -> Pid

@external(erlang, "consumer_b", "reference")
pub fn reference(reference: Reference) -> Reference

@external(erlang, "consumer_b", "name")
pub fn name(name: Name(message)) -> Name(message)

@external(erlang, "consumer_b", "named")
pub fn named(name: Name(message)) -> Option(Pid)

@external(erlang, "consumer_b", "subject")
pub fn subject(subject: Subject(message)) -> Subject(message)
