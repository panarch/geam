import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory

fn await_exit(pid: process.Pid) {
  let monitor = process.monitor(pid)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  Nil
}

pub fn main() {
  let missing: factory.Supervisor(Int, Nil) =
    factory.get_by_name(process.new_name("missing_factory"))
  process.spawn_unlinked(fn() {
    let _ = factory.count_children(missing)
    Nil
  })
  |> await_exit
  process.spawn_unlinked(fn() {
    let _ = factory.start_child(missing, 1)
    Nil
  })
  |> await_exit
  let assert Ok(actor.Started(pid, exited)) =
    factory.worker_child(fn(_: Int) -> actor.StartResult(Nil) {
      Error(actor.InitFailed("unused"))
    })
    |> factory.start
  process.unlink(pid)
  process.kill(pid)
  await_exit(pid)
  process.spawn_unlinked(fn() {
    let _ = factory.count_children(exited)
    Nil
  })
  |> await_exit
  process.spawn_unlinked(fn() {
    let _ = factory.start_child(exited, 1)
    Nil
  })
  |> await_exit
}
