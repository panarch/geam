import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/erlang/reference
import gleam/option.{None, Some}
import service_consumer_a as a
import service_consumer_b as b

pub fn main() {
  let first_ticket = a.ticket()
  let second_ticket = b.ticket()
  let third_ticket = a.ticket()
  let pid = process.self()
  let assert True = a.current() == pid
  let assert True = a.alive(pid)
  let assert True = b.pid(pid) == pid
  let assert True = a.pids([pid, pid]) == [pid, pid]
  let child = process.spawn_unlinked(fn() { process.sleep_forever() })
  let child_name: process.Name(Int) = process.new_name("child")
  let assert True = a.alive(child)
  let assert True = a.pids([pid, child]) == [child, pid]
  let assert True = a.register(child, child_name)
  let assert True = b.named(child_name) == Some(child)
  process.kill(child)
  let assert False = a.alive(child)
  let assert None = a.named(child_name)
  let name = process.new_name("shared")
  let other_name = process.new_name("shared")
  let assert True = b.name(name) == name
  let assert True = a.names([name, other_name]) == [other_name, name]
  let assert None = b.named(name)
  let assert True = a.register(pid, name)
  let assert False = a.register(pid, name)
  let assert True = a.named(name) == Some(pid)
  let assert True = b.named(name) == Some(pid)
  let assert True = a.unregister(name)
  let assert False = a.unregister(name)
  let assert None = a.named(name)
  let first = reference.new()
  let second = a.reference()
  let assert False = first == second
  let assert True = b.reference(first) == first
  let assert True = b.reference(second) == second
  let assert True = a.references([first, second]) == [second, first]
  let reply = process.unsafely_create_subject(pid, dynamic.string("reply"))
  process.send(reply, 42)
  let assert Error(Nil) = a.receive(second)
  let assert Ok(value) = a.receive("reply")
  let assert Ok(42) = decode.run(value, decode.int)
  let reply = process.unsafely_create_subject(pid, dynamic.string("reply"))
  process.send(reply, "preserved")
  let assert Ok(value) = a.receive_any()
  let assert Ok("preserved") = decode.run(value, decode.at([1], decode.string))
  let assert Error(Nil) = a.receive_any()
  a.send(pid, "direct reply")
  let value = a.receive_forever()
  let assert Ok("direct reply") = decode.run(value, decode.string)
  let fresh = a.new_name("provider")
  let assert None = a.named(fresh)
  let assert True = b.name(fresh) == fresh
  let assert True = a.names([fresh, name]) == [name, fresh]
  let subject = process.new_subject()
  let other = process.new_subject()
  let assert True = a.subject(subject) == subject
  let assert True = b.subject(subject) == subject
  let assert True = a.subjects([subject, other]) == [other, subject]
  let assert True = a.send_subject(b.subject(subject), #(pid, fn(x) { x + 7 }))
  let assert Ok(#(owner, callback)) = a.receive_subject(subject)
  let assert True = owner == pid
  let assert 42 = callback(35)
  let assert Error(Nil) = a.receive_subject(subject)
  let named = process.named_subject(fresh)
  let assert True = a.subject(named) == named
  let assert False = a.send_subject(named, "missing")
  let assert True = a.register(pid, fresh)
  let assert True = a.send_subject(b.subject(named), "named reply")
  let assert Ok("named reply") = a.receive_subject(named)
  let assert True = a.unregister(fresh)
  #(first_ticket, second_ticket, third_ticket)
}
