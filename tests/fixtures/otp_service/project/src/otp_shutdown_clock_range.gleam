import gleam/erlang/atom
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
  let assert Ok(actor.Started(static, _)) =
    static.new(static.OneForOne)
    |> static.add(supervision.worker(start_child))
    |> static.start
  let assert Ok(actor.Started(factory_pid, factory)) =
    factory.worker_child(fn(_: Nil) { start_child() }) |> factory.start
  let assert Ok(actor.Started(child, _)) = factory.start_child(factory, Nil)
  process.unlink(static)
  process.unlink(factory_pid)
  // The host advances its monotonic clock to the last representable second.
  process.sleep(1)
  process.send_abnormal_exit(static, atom.create("shutdown"))
  process.send_abnormal_exit(factory_pid, atom.create("shutdown"))
  await_exit(static)
  await_exit(factory_pid)
  let assert False = process.is_alive(child)
  Nil
}
