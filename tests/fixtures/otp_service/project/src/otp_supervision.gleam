import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/int
import gleam/io
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory
import gleam/otp/static_supervisor as static
import gleam/otp/supervision
import gleam/otp/system

pub fn main() {
  failed_static_start()
  graceful_static_shutdown()
  factory_restarts_and_exhaustion()
  factory_duplicate_and_child_error()
  io.println(
    "original supervisors: failure, graceful shutdown, restart budget, cleanup",
  )
}

fn failed_static_start() {
  let children = process.new_subject()
  let good =
    supervision.worker(fn() {
      let assert Ok(actor.Started(pid, inbox)) =
        actor.new(12)
        |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
        |> actor.start
      process.send(children, pid)
      Ok(actor.Started(pid, inbox))
    })
  let bad =
    supervision.worker(fn() -> actor.StartResult(Nil) {
      Error(actor.InitFailed("child refused"))
    })
  let assert Error(actor.InitFailed("child refused")) =
    static.new(static.OneForOne)
    |> static.add(good)
    |> static.add(bad)
    |> static.start
  let assert Ok(child) = process.receive(children, 0)
  let assert False = process.is_alive(child)
}

fn graceful_static_shutdown() {
  let children = process.new_subject()
  let stopped = process.new_subject()
  let child =
    supervision.supervisor(fn() {
      let ready = process.new_subject()
      let pid =
        process.spawn(fn() {
          process.trap_exits(True)
          process.send(ready, Nil)
          let exit =
            process.new_selector()
            |> process.select_trapped_exits(fn(exit) { exit })
            |> process.selector_receive_forever
          process.send(stopped, exit.reason)
        })
      process.receive_forever(ready)
      process.send(children, pid)
      Ok(actor.Started(pid, Nil))
    })
  let assert Ok(actor.Started(supervisor, _)) =
    static.new(static.OneForOne)
    |> static.add(child)
    |> static.start
  let assert Ok(child) = process.receive(children, 0)
  process.unlink(supervisor)
  let monitor = process.monitor(supervisor)
  process.send_abnormal_exit(supervisor, atom.create("shutdown"))
  let assert process.Abnormal(reason) = process.receive_forever(stopped)
  let assert Ok(shutdown) = decode.run(reason, atom.decoder())
  let assert "shutdown" = atom.to_string(shutdown)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(child)
  let assert False = process.is_alive(supervisor)
}

fn factory_restarts_and_exhaustion() {
  let calls = process.new_subject()
  let assert Ok(actor.Started(supervisor, factory)) =
    factory.worker_child(fn(argument: Int) {
      let assert Ok(actor.Started(pid, _)) =
        actor.new(argument)
        |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
        |> actor.start
      process.send(calls, #(process.self(), pid, argument))
      Ok(actor.Started(pid, int.to_string(argument)))
    })
    |> factory.restart_strategy(supervision.Permanent)
    |> factory.restart_tolerance(1, 5)
    |> factory.start
  process.unlink(supervisor)
  let monitor = process.monitor(supervisor)
  let assert Ok(actor.Started(first, "42")) = factory.start_child(factory, 42)
  let assert Ok(#(owner, first_call, 42)) = process.receive(calls, 0)
  let assert True = owner == supervisor && first_call == first
  let assert Ok(actor.Started(sibling, "9")) = factory.start_child(factory, 9)
  let assert Ok(#(_, sibling_call, 9)) = process.receive(calls, 0)
  let assert True = sibling_call == sibling
  process.kill(first)
  let #(owner, second, argument) = process.receive_forever(calls)
  let assert True = owner == supervisor && second != first
  let assert 42 = argument
  let assert Ok(42) = decode.run(system.get_state(second), decode.int)
  let assert 2 = factory.count_children(factory)
  process.kill(second)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(supervisor)
  let assert False = process.is_alive(sibling)
  let assert Error(Nil) = process.receive(calls, 0)
}

fn factory_duplicate_and_child_error() {
  let name = process.new_name("failing_factory")
  let builder =
    factory.worker_child(fn(_: Int) -> actor.StartResult(String) {
      Error(actor.InitFailed("bad argument"))
    })
    |> factory.named(name)
  let assert Ok(actor.Started(supervisor, factory)) = factory.start(builder)
  let assert Error(actor.InitFailed("supervisor name is already registered")) =
    factory.start(builder)
  let assert Error(actor.InitFailed("bad argument")) =
    factory.start_child(factory, 42)
  let assert 0 = factory.count_children(factory)
  process.unlink(supervisor)
  process.kill(supervisor)
}
