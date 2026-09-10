import gleam/erlang/atom
import gleam/erlang/process

fn down(monitor) {
  process.new_selector()
  |> process.select_specific_monitor(monitor, fn(message) { message })
  |> process.selector_receive_forever
}

pub fn main() {
  let ready = process.new_subject()
  let ordinary =
    process.spawn_unlinked(fn() {
      let finish = process.new_subject()
      process.send(ready, finish)
      process.receive_forever(finish)
    })
  let monitor = process.monitor(ordinary)
  let finish = process.receive_forever(ready)
  process.send_abnormal_exit(ordinary, atom.create("normal"))
  let ignored = process.is_alive(ordinary)
  process.send(finish, Nil)
  let assert process.ProcessDown(_, _, process.Normal) = down(monitor)

  let observed = process.new_subject()
  let trapping =
    process.spawn_unlinked(fn() {
      process.trap_exits(True)
      process.send(observed, Nil)
      let assert process.ExitMessage(_, process.Normal) =
        process.new_selector()
        |> process.select_trapped_exits(fn(message) { message })
        |> process.selector_receive_forever
      process.send(observed, Nil)
      process.sleep_forever()
    })
  let monitor = process.monitor(trapping)
  process.receive_forever(observed)
  process.send_abnormal_exit(trapping, atom.create("normal"))
  process.receive_forever(observed)
  process.send_abnormal_exit(trapping, atom.create("kill"))
  let assert process.ProcessDown(_, killed_pid, process.Killed) = down(monitor)

  let self_exiting =
    process.spawn_unlinked(fn() {
      let start = process.new_subject()
      process.send(ready, start)
      process.receive_forever(start)
      process.send_abnormal_exit(process.self(), atom.create("normal"))
      panic as "self normal exit must stop execution"
    })
  let monitor = process.monitor(self_exiting)
  process.send(process.receive_forever(ready), Nil)
  let assert process.ProcessDown(_, normal_pid, process.Normal) = down(monitor)

  process.trap_exits(True)
  process.send_abnormal_exit(process.self(), atom.create("normal"))
  let assert process.ExitMessage(self_pid, process.Normal) =
    process.new_selector()
    |> process.select_trapped_exits(fn(message) { message })
    |> process.selector_receive_forever

  let linked = process.spawn(fn() { process.sleep_forever() })
  process.kill(linked)
  let assert process.ExitMessage(linked_pid, process.Killed) =
    process.new_selector()
    |> process.select_trapped_exits(fn(message) { message })
    |> process.selector_receive_forever
  process.trap_exits(False)
  #(
    ignored,
    killed_pid == trapping,
    normal_pid == self_exiting,
    self_pid == process.self(),
    linked_pid == linked,
  )
}
// @geam:expect #(True, True, True, True, True)
