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

pub fn main() {
  let attempt = process.new_name("static_restart_panic")
  let children = process.new_subject()
  let first =
    supervision.worker(fn() {
      let assert Ok(Nil) = case process.register(process.self(), attempt) {
        Error(_) -> panic as "static restart callback"
        Ok(_) -> Ok(Nil)
      }
      let assert Ok(actor.Started(pid, inbox)) = start_child()
      process.send(children, pid)
      Ok(actor.Started(pid, inbox))
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

  let attempt = process.new_name("factory_restart_panic")
  let assert Ok(actor.Started(supervisor, factory)) =
    factory.worker_child(fn(argument: Int) {
      case argument, process.register(process.self(), attempt) {
        1, Error(_) -> panic as "factory restart callback"
        _, _ -> {
          let assert Ok(actor.Started(pid, _)) = start_child()
          Ok(actor.Started(pid, argument))
        }
      }
    })
    |> factory.restart_strategy(supervision.Permanent)
    |> factory.start
  let assert Ok(actor.Started(first, 1)) = factory.start_child(factory, 1)
  let assert Ok(actor.Started(sibling, 2)) = factory.start_child(factory, 2)
  process.unlink(supervisor)
  process.kill(first)
  await_exit(supervisor)
  let assert False = process.is_alive(sibling)
  Nil
}
