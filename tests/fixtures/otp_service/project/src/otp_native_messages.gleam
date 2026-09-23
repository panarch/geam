import gleam/erlang/atom
import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory
import gleam/otp/static_supervisor as static
import otp_service_support as support

fn await_exit(pid: process.Pid) {
  let monitor = process.monitor(pid)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(pid)
}

fn reject_factory_message(message: message) {
  let assert Ok(actor.Started(pid, _)) =
    factory.worker_child(fn(_: Int) -> actor.StartResult(Nil) {
      panic as "malformed messages must not invoke this callback"
    })
    |> factory.start
  process.unlink(pid)
  support.send_message(pid, message)
  await_exit(pid)
}

pub fn main() {
  reject_factory_message(#(
    atom.create("factory"),
    atom.create("start"),
    "wrong argument type",
    process.self(),
    Nil,
  ))
  reject_factory_message(#(
    atom.create("factory"),
    atom.create("start"),
    1,
    "not a pid",
    Nil,
  ))
  reject_factory_message(#(
    atom.create("EXIT"),
    "not a pid",
    atom.create("normal"),
  ))
  reject_factory_message(#(
    atom.create("factory"),
    atom.create("unknown"),
    Nil,
    process.self(),
    Nil,
  ))
  reject_factory_message(#(atom.create("factory")))
  let assert Ok(actor.Started(pid, _)) =
    static.new(static.OneForOne) |> static.start
  process.unlink(pid)
  support.send_message(pid, #(
    atom.create("EXIT"),
    "not a pid",
    atom.create("normal"),
  ))
  await_exit(pid)
  Nil
}
