import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/io
import gleam/otp/actor
import gleam/otp/system
import otp_service_support

type Command {
  Stop
  Fail
}

pub fn main() {
  process.trap_exits(True)
  let entered = process.new_subject()
  let assert Error(actor.InitFailed("cannot initialise")) =
    actor.new_with_initialiser(5, fn(_: process.Subject(Nil)) -> Result(
      actor.Initialised(Nil, Nil, Nil),
      String,
    ) {
      process.send(entered, process.self())
      Error("cannot initialise")
    })
    |> actor.start
  let assert Ok(failed) = process.receive(entered, 0)
  let assert False = process.is_alive(failed)

  let assert Error(actor.InitExited(process.Killed)) =
    actor.new_with_initialiser(5, fn(_: process.Subject(Nil)) {
      process.send(entered, process.self())
      process.kill(process.self())
      Ok(actor.initialised(Nil))
    })
    |> actor.start
  let assert Ok(killed) = process.receive(entered, 0)
  let assert False = process.is_alive(killed)

  stop_actor(Stop)
  stop_actor(Fail)
  io.println(
    "original actor: failed init, exited init, normal and abnormal stop",
  )

  let assert Ok(actor.Started(pid, _)) =
    actor.new(12)
    |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
    |> actor.start
  otp_service_support.send_message(pid, "unexpected fixture message")
  otp_service_support.send_message(pid, #(
    atom.create("system"),
    "bad request",
    Nil,
  ))
  otp_service_support.send_message(pid, #(atom.create("other"), 0, Nil))
  let assert Ok(12) = decode.run(system.get_state(pid), decode.int)
  let status = otp_service_support.get_status(pid)
  let assert True = status.parent == process.self()
  let assert system.Running = status.mode
  let assert Ok(12) = decode.run(status.state, decode.int)
  let assert True = status.debug_state == system.debug_state([])
  let assert False = status.debug_state == system.debug_state([system.NoDebug])
  system.suspend(pid)
  let status = otp_service_support.get_status(pid)
  let assert system.Suspended = status.mode
  let assert Ok(12) = decode.run(status.state, decode.int)
  system.resume(pid)
  process.unlink(pid)
  process.kill(pid)
}

fn stop_actor(command: Command) {
  let assert Ok(actor.Started(pid, inbox)) =
    actor.new(Nil)
    |> actor.on_message(fn(_, command) {
      case command {
        Stop -> actor.stop()
        Fail -> actor.stop_abnormal("fixture failure")
      }
    })
    |> actor.start
  process.unlink(pid)
  let monitor = process.monitor(pid)
  process.send(inbox, command)
  let down =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert True = down.monitor == monitor
  case command {
    Stop -> {
      let assert process.Normal = down.reason
      Nil
    }
    Fail -> {
      let assert process.Abnormal(reason) = down.reason
      let assert Ok("fixture failure") = decode.run(reason, decode.string)
      Nil
    }
  }
  let assert False = process.is_alive(pid)
}
