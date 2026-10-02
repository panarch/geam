import gleam/dict
import gleam/erlang/process
import gleam/erlang/reference
import gleam/io
import selective_receive_service_fixture/native

pub fn main() {
  let owner = process.self()
  let a = reference.new()
  let b = reference.new()
  let key_a = native.key(a)
  let key_alias = native.key(a)
  let key_b = native.key(b)
  let assert True = key_a == key_alias
  let assert False = key_a == key_b
  let keys = dict.from_list([#(key_a, 11), #(key_b, 22)])
  let assert Ok(11) = dict.get(keys, key_alias)
  native.send(owner, #("tcp", b, 20))
  native.send(owner, #("user", a, 99))
  native.send(owner, #("tcp", a, 10))
  let assert Ok(#("tcp", selected_a, 10)) = native.receive(a, False)
  let assert True = selected_a == a
  // The B message stays before the rejected user message.
  let assert Ok(#("tcp", selected_b, 20)) = native.next()
  let assert True = selected_b == b
  let assert Ok(#("user", selected_user, 99)) = native.next()
  let assert True = selected_user == a
  native.send(owner, #("passive", a, 30))
  native.send(owner, #("tcp", b, 40))
  native.send(owner, #("closed", a, 50))
  native.send(owner, #("error", a, 60))
  let assert Ok(#("passive", _, 30)) = native.receive(a, False)
  let assert Ok(#("closed", _, 50)) = native.receive(a, True)
  let assert Ok(#("error", _, 60)) = native.receive(a, False)
  let assert Ok(#("tcp", _, 40)) = native.receive(b, False)
  let assert Error(Nil) = native.receive(a, False)
  let assert Error(Nil) = native.next()
  native.send(owner, 123)
  native.send(owner, #("tcp", a))
  native.send(owner, #(123, a, 0))
  native.send(owner, #("tcp", 123, 0))
  // Match before typed restoration: an invalid payload is ordinary absence.
  native.send(owner, #("tcp", a, False))
  let assert Error(Nil) = native.receive(a, False)
  let assert Error(Nil) = native.next()
  let assert Error(Nil) = native.next()
  let assert Error(Nil) = native.next()
  let assert Error(Nil) = native.next()
  // Matching an integer identity cannot restore it as the producer's Reference.
  native.send(owner, #("tcp", 123, 0))
  let assert Error(Nil) = native.receive(123, False)
  native.send(owner, #("user", a, False))
  let assert Error(Nil) = native.next()
  io.println("caller identity and mixed tags preserve mailbox order")
  Nil
}

pub fn inspect_key() {
  let identity = reference.new()
  echo native.key(identity)
  echo identity
  Nil
}
