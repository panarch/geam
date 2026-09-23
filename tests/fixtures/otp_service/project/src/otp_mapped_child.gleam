import gleam/erlang/atom
import gleam/erlang/process
import gleam/int
import gleam/otp/actor
import gleam/otp/static_supervisor as static
import gleam/otp/supervision

fn start_supervisor(started) {
  let increment = 1
  let child =
    supervision.worker(fn() {
      let assert Ok(actor.Started(pid, _)) =
        actor.new(41)
        |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
        |> actor.start
      Ok(actor.Started(pid, #(pid, 41)))
    })
    |> supervision.map_data(fn(data) {
      let #(pid, value) = data
      let mapped = int.to_string(value + increment)
      process.send(started, #(process.self(), pid, mapped))
      mapped
    })
    |> supervision.restart(supervision.Permanent)
    |> supervision.timeout(20)
    |> supervision.significant(True)
  static.new(static.OneForOne)
  |> static.add(child)
  |> static.start
}

pub fn main() {
  let started = process.new_subject()
  let assert Ok(actor.Started(supervisor, _)) = start_supervisor(started)
  let assert Ok(#(invoker, first, "42")) = process.receive(started, 0)
  let assert True = invoker == supervisor
  // The builder and original callback handles are gone before this restart.
  process.kill(first)
  let #(invoker, second, mapped) = process.receive_forever(started)
  let assert True = invoker == supervisor
  let assert True = second != first
  let assert "42" = mapped
  process.unlink(supervisor)
  let monitor = process.monitor(supervisor)
  process.send_abnormal_exit(supervisor, atom.create("shutdown"))
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(first)
  let assert False = process.is_alive(second)

  let transformed = process.new_subject()
  let failed =
    supervision.worker(fn() -> actor.StartResult(Int) {
      Error(actor.InitFailed("map_data failure"))
    })
    |> supervision.map_data(fn(value) {
      process.send(transformed, value)
      int.to_string(value)
    })
  let assert Error(actor.InitFailed("map_data failure")) =
    static.new(static.OneForOne) |> static.add(failed) |> static.start
  let assert Error(Nil) = process.receive(transformed, 0)
  Nil
}
