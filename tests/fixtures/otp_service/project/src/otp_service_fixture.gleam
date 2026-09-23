import gleam/dynamic/decode
import gleam/erlang/process
import gleam/int
import gleam/io
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory
import gleam/otp/static_supervisor
import gleam/otp/supervision
import gleam/otp/system

pub fn main() {
  let assert Ok(actor.Started(pid, inbox)) =
    actor.new(10)
    |> actor.on_message(fn(state, delta) { actor.continue(state + delta) })
    |> actor.start
  let assert Ok(10) = decode.run(system.get_state(pid), decode.int)
  system.suspend(pid)
  process.send(inbox, 7)
  let assert Ok(10) = decode.run(system.get_state(pid), decode.int)
  system.resume(pid)
  let assert Ok(17) = decode.run(system.get_state(pid), decode.int)
  process.unlink(pid)
  process.kill(pid)
  io.println("original actor: state, suspend, resume")
  static_child()
  factory_children()
}

fn factory_children() {
  let calls = process.new_subject()
  let name = process.new_name("integer_factory")
  let assert Ok(actor.Started(supervisor, by_pid)) =
    start_integer_factory(calls, name)
  let by_name = factory.get_by_name(name)
  let assert 0 = factory.count_children(by_pid)
  let assert Ok(actor.Started(first, "42")) = factory.start_child(by_name, 42)
  let assert Ok(invoker) = process.receive(calls, 0)
  let assert True = invoker == supervisor
  let assert Ok(actor.Started(second, "17")) = factory.start_child(by_pid, 17)
  let assert Ok(invoker) = process.receive(calls, 0)
  let assert True = invoker == supervisor
  let assert True = first != second
  let assert 2 = factory.count_children(by_name)
  let assert Ok(42) = decode.run(system.get_state(first), decode.int)
  let assert Ok(17) = decode.run(system.get_state(second), decode.int)

  let assert Ok(actor.Started(other, string_factory)) =
    start_string_factory(calls)
  let assert Ok(actor.Started(third, 81)) =
    factory.start_child(string_factory, "80")
  let assert Ok(invoker) = process.receive(calls, 0)
  let assert True = invoker == other
  let assert Ok(80) = decode.run(system.get_state(third), decode.int)
  let assert 1 = factory.count_children(string_factory)
  process.unlink(supervisor)
  process.kill(supervisor)
  process.unlink(other)
  process.kill(other)
  io.println("original factory: two typed callbacks, named and pid handles")
}

fn static_child() {
  let calls = process.new_subject()
  let assert Ok(actor.Started(supervisor, _)) = start_static_supervisor(calls)
  let assert Ok(#(invoker, first, _)) = process.receive(calls, 0)
  let assert True = invoker == supervisor
  process.kill(first)
  let #(invoker, second, inbox) = process.receive_forever(calls)
  let assert True = invoker == supervisor
  let assert True = first != second
  process.send(inbox, 19)
  let assert Ok(42) = decode.run(system.get_state(second), decode.int)
  process.unlink(supervisor)
  process.kill(supervisor)
  let assert False = process.is_alive(first)
  let assert False = process.is_alive(second)
  io.println("original static child: retained callback in supervisor")
}

fn start_integer_factory(calls, name) {
  factory.worker_child(fn(argument: Int) {
    process.send(calls, process.self())
    let assert Ok(actor.Started(pid, _)) =
      actor.new(argument)
      |> actor.on_message(fn(state, delta) { actor.continue(state + delta) })
      |> actor.start
    Ok(actor.Started(pid, int.to_string(argument)))
  })
  |> factory.named(name)
  |> factory.start
}

fn start_string_factory(calls) {
  factory.worker_child(fn(argument: String) {
    process.send(calls, process.self())
    let assert Ok(value) = int.parse(argument)
    let assert Ok(actor.Started(pid, _)) =
      actor.new(value)
      |> actor.on_message(fn(state, delta) { actor.continue(state + delta) })
      |> actor.start
    Ok(actor.Started(pid, value + 1))
  })
  |> factory.start
}

fn start_static_supervisor(calls) {
  let child =
    supervision.worker(fn() {
      let invoker = process.self()
      let assert Ok(actor.Started(pid, inbox)) =
        actor.new(23)
        |> actor.on_message(fn(state, delta) { actor.continue(state + delta) })
        |> actor.start
      process.send(calls, #(invoker, pid, inbox))
      Ok(actor.Started(pid, inbox))
    })
    |> supervision.restart(supervision.Permanent)
  static_supervisor.new(static_supervisor.OneForOne)
  |> static_supervisor.restart_tolerance(2, 5)
  |> static_supervisor.add(child)
  |> static_supervisor.start
}
