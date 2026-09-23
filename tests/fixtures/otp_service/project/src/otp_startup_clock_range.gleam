import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/static_supervisor as static
import gleam/otp/supervision

pub fn main() {
  let started = process.new_subject()
  let parent =
    process.spawn_unlinked(fn() {
      let first =
        supervision.worker(fn() {
          let assert Ok(actor.Started(pid, inbox)) =
            actor.new(Nil)
            |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
            |> actor.start
          process.send(started, pid)
          Ok(actor.Started(pid, inbox))
        })
      let failed =
        supervision.worker(fn() -> actor.StartResult(Nil) {
          // The host advances its clock while this startup callback is suspended.
          process.sleep(1)
          Error(actor.InitFailed("second child refused"))
        })
      let _ =
        static.new(static.OneForOne)
        |> static.add(first)
        |> static.add(failed)
        |> static.start
      Nil
    })
  let child = process.receive_forever(started)
  let monitor = process.monitor(parent)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(child)
  Nil
}
