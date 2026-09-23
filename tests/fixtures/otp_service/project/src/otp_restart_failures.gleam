import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory
import gleam/otp/static_supervisor as static
import gleam/otp/supervision

fn start_child() {
  actor.new(Nil)
  |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
  |> actor.start
}

fn await_exit(pid: process.Pid) {
  let monitor = process.monitor(pid)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(pid)
}

fn static_stops_after_restart_failure() {
  let attempt = process.new_name("static_restart_attempt")
  let children = process.new_subject()
  let first =
    supervision.worker(fn() {
      case process.register(process.self(), attempt) {
        Error(_) -> Error(actor.InitFailed("restart refused"))
        Ok(_) -> {
          let assert Ok(actor.Started(pid, inbox)) = start_child()
          process.send(children, pid)
          Ok(actor.Started(pid, inbox))
        }
      }
    })
    |> supervision.restart(supervision.Permanent)
  let sibling =
    supervision.worker(fn() {
      let assert Ok(actor.Started(pid, inbox)) = start_child()
      process.send(children, pid)
      Ok(actor.Started(pid, inbox))
    })
  let assert Ok(actor.Started(supervisor, _)) =
    static.new(static.OneForOne)
    |> static.add(first)
    |> static.add(sibling)
    |> static.start
  let assert Ok(first) = process.receive(children, 0)
  let assert Ok(sibling) = process.receive(children, 0)
  process.unlink(supervisor)
  process.kill(first)
  await_exit(supervisor)
  let assert False = process.is_alive(sibling)
  let assert Error(Nil) = process.receive(children, 0)
}

fn static_stops_when_restart_budget_is_empty() {
  let children = process.new_subject()
  let child =
    supervision.worker(fn() {
      let assert Ok(actor.Started(pid, inbox)) = start_child()
      process.send(children, pid)
      Ok(actor.Started(pid, inbox))
    })
    |> supervision.restart(supervision.Permanent)
  let assert Ok(actor.Started(supervisor, _)) =
    static.new(static.OneForOne)
    |> static.restart_tolerance(0, 5)
    |> static.add(child)
    |> static.add(child)
    |> static.start
  let assert Ok(first) = process.receive(children, 0)
  let assert Ok(sibling) = process.receive(children, 0)
  process.unlink(supervisor)
  process.kill(first)
  await_exit(supervisor)
  let assert False = process.is_alive(sibling)
  let assert Error(Nil) = process.receive(children, 0)
}

fn factory_stops_after_restart_failure() {
  let attempt = process.new_name("factory_restart_attempt")
  let calls = process.new_subject()
  let assert Ok(actor.Started(supervisor, factory)) =
    factory.worker_child(fn(argument: Int) {
      case argument, process.register(process.self(), attempt) {
        1, Error(_) -> Error(actor.InitFailed("restart refused"))
        _, _ -> {
          let assert Ok(actor.Started(pid, _)) = start_child()
          process.send(calls, argument)
          Ok(actor.Started(pid, argument))
        }
      }
    })
    |> factory.restart_strategy(supervision.Permanent)
    |> factory.start
  let assert Ok(actor.Started(first, 1)) = factory.start_child(factory, 1)
  let assert Ok(actor.Started(sibling, 2)) = factory.start_child(factory, 2)
  let assert Ok(1) = process.receive(calls, 0)
  let assert Ok(2) = process.receive(calls, 0)
  process.unlink(supervisor)
  process.kill(first)
  await_exit(supervisor)
  let assert False = process.is_alive(sibling)
  let assert Error(Nil) = process.receive(calls, 0)
}

pub fn main() {
  static_stops_after_restart_failure()
  static_stops_when_restart_budget_is_empty()
  factory_stops_after_restart_failure()
  Nil
}
