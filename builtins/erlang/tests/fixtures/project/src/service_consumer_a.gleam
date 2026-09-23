import gleam/dynamic.{type Dynamic}
import gleam/erlang/process.{type Name, type Pid, type Subject}
import gleam/erlang/reference.{type Reference}
import gleam/option.{type Option}

@external(erlang, "consumer_a", "ticket")
pub fn ticket() -> Int

@external(erlang, "consumer_a", "current")
pub fn current() -> Pid

@external(erlang, "consumer_a", "alive")
pub fn alive(pid: Pid) -> Bool

@external(erlang, "consumer_a", "send")
pub fn send(pid: Pid, message: message) -> Nil

@external(erlang, "consumer_a", "reference")
pub fn reference() -> Reference

@external(erlang, "consumer_a", "receive")
pub fn receive(tag: tag) -> Result(Dynamic, Nil)

@external(erlang, "consumer_a", "receive_any")
pub fn receive_any() -> Result(Dynamic, Nil)

@external(erlang, "consumer_a", "receive_forever")
pub fn receive_forever() -> Dynamic

@external(erlang, "consumer_a", "named")
pub fn named(name: Name(message)) -> Option(Pid)

@external(erlang, "consumer_a", "register")
pub fn register(pid: Pid, name: Name(message)) -> Bool

@external(erlang, "consumer_a", "unregister")
pub fn unregister(name: Name(message)) -> Bool

@external(erlang, "consumer_a", "pids")
pub fn pids(values: List(Pid)) -> List(Pid)

@external(erlang, "consumer_a", "names")
pub fn names(values: List(Name(message))) -> List(Name(message))

@external(erlang, "consumer_a", "references")
pub fn references(values: List(Reference)) -> List(Reference)

@external(erlang, "consumer_a", "new_name")
pub fn new_name(prefix: String) -> Name(message)

@external(erlang, "consumer_a", "subject")
pub fn subject(subject: Subject(message)) -> Subject(message)

@external(erlang, "consumer_a", "subjects")
pub fn subjects(subjects: List(Subject(message))) -> List(Subject(message))

@external(erlang, "consumer_a", "send_subject")
pub fn send_subject(subject: Subject(message), message: message) -> Bool

@external(erlang, "consumer_a", "receive_subject")
pub fn receive_subject(subject: Subject(message)) -> Result(message, Nil)
