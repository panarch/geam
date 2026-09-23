import gleam/erlang/atom
import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/factory_supervisor as factory
import gleam/otp/static_supervisor as static
import gleam/otp/supervision
import otp_service_support as support

fn await_exit(pid: process.Pid) {
  let monitor = process.monitor(pid)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  Nil
}

fn rejected_flags() {
  process.spawn_unlinked(fn() {
    let _ = static.new(static.OneForAll) |> static.start
    Nil
  })
  |> await_exit
  process.spawn_unlinked(fn() {
    let _ =
      static.new(static.OneForOne)
      |> static.restart_tolerance(-1, 5)
      |> static.start
    Nil
  })
  |> await_exit
  process.spawn_unlinked(fn() {
    let _ =
      factory.worker_child(fn(_: Int) -> actor.StartResult(Nil) {
        Error(actor.InitTimeout)
      })
      |> factory.restart_tolerance(1, 0)
      |> factory.start
    Nil
  })
  |> await_exit
}

fn callback_failures() {
  process.spawn_unlinked(fn() {
    let child = supervision.worker(fn() { panic as "static start callback" })
    let _ = static.new(static.OneForOne) |> static.add(child) |> static.start
    Nil
  })
  |> await_exit
  process.spawn_unlinked(fn() {
    let assert Ok(actor.Started(_, factory)) =
      factory.worker_child(fn(_: Int) { panic as "factory start callback" })
      |> factory.start
    let _ = factory.start_child(factory, 1)
    Nil
  })
  |> await_exit
}

fn parent_termination() {
  let started = process.new_subject()
  let parent =
    process.spawn_unlinked(fn() {
      let assert Ok(actor.Started(pid, _)) =
        factory.supervisor_child(fn(_: Int) -> actor.StartResult(Nil) {
          Error(actor.InitTimeout)
        })
        |> factory.start
      process.send(started, pid)
      process.sleep_forever()
    })
  let supervisor = process.receive_forever(started)
  process.kill(parent)
  await_exit(supervisor)
  let assert False = process.is_alive(supervisor)
}

fn ignores_unowned_exits_and_does_not_restart_temporary_children() {
  let assert Ok(actor.Started(supervisor, factory)) =
    factory.worker_child(fn(_: Nil) {
      actor.new(Nil)
      |> actor.on_message(fn(_, _: Nil) { actor.stop() })
      |> actor.start
    })
    |> factory.restart_strategy(supervision.Temporary)
    |> factory.start
  let assert Ok(actor.Started(child, inbox)) = factory.start_child(factory, Nil)
  process.send(inbox, Nil)
  await_exit(child)
  support.send_message(supervisor, #(
    atom.create("EXIT"),
    child,
    atom.create("normal"),
  ))
  let assert 0 = factory.count_children(factory)
  process.unlink(supervisor)
  process.kill(supervisor)
}

fn static_ignores_repeated_exits_without_restarting_temporary_children() {
  let started = process.new_subject()
  let child =
    supervision.worker(fn() {
      let assert Ok(actor.Started(pid, inbox)) =
        actor.new(Nil)
        |> actor.on_message(fn(_, _: Nil) { actor.stop() })
        |> actor.start
      process.send(started, #(pid, inbox))
      Ok(actor.Started(pid, Nil))
    })
    |> supervision.restart(supervision.Temporary)
  let assert Ok(actor.Started(supervisor, _)) =
    static.new(static.OneForOne) |> static.add(child) |> static.start
  let assert Ok(#(child, inbox)) = process.receive(started, 0)
  // Nonmatching EXIT records remain unconsumed; they must not stop the supervisor.
  support.send_message(supervisor, #(atom.create("EXIT")))
  support.send_message(supervisor, #(atom.create("EXIT"), child))
  support.send_message(supervisor, #(
    atom.create("EXIT"),
    child,
    atom.create("normal"),
    Nil,
  ))
  process.send(inbox, Nil)
  await_exit(child)
  support.send_message(supervisor, #(
    atom.create("EXIT"),
    child,
    atom.create("normal"),
  ))
  process.unlink(supervisor)
  process.send_abnormal_exit(supervisor, atom.create("shutdown"))
  await_exit(supervisor)
  let assert Error(Nil) = process.receive(started, 0)
}

pub fn main() {
  rejected_flags()
  callback_failures()
  parent_termination()
  ignores_unowned_exits_and_does_not_restart_temporary_children()
  static_ignores_repeated_exits_without_restarting_temporary_children()
  Nil
}
