import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process

pub fn main() {
  let ready = process.new_subject()
  let name = process.new_name("lifecycle")
  let worker =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, Nil)
      process.sleep_forever()
    })
  let first_monitor = process.monitor(worker)
  let second_monitor = process.monitor(worker)
  let _ = process.receive_forever(ready)
  let before = process.named(name) == Ok(worker)
  let duplicate = process.register(process.self(), name)
  let second_name = process.new_name("second")
  let already_named = process.register(worker, second_name)
  let unregistered = process.unregister(second_name)
  let specific =
    process.new_selector()
    |> process.select_specific_monitor(first_monitor, fn(message) { message })
  process.kill(worker)
  let assert process.ProcessDown(first, first_pid, process.Killed) =
    process.selector_receive_forever(specific)
  let assert process.ProcessDown(second, second_pid, process.Killed) =
    process.new_selector()
    |> process.select_monitors(fn(message) { message })
    |> process.selector_receive_forever
  let after = process.named(name)
  let dead_monitor = process.monitor(worker)
  let assert process.ProcessDown(dead, dead_pid, process.Abnormal(reason)) =
    process.new_selector()
    |> process.select_specific_monitor(dead_monitor, fn(message) { message })
    |> process.selector_receive_forever
  let assert Ok(reason_atom) = decode.run(reason, atom.decoder())
  let removed_monitor = process.monitor(worker)
  process.demonitor_process(removed_monitor)
  let empty =
    process.new_selector()
    |> process.select_monitors(fn(_) { Nil })
    |> process.selector_receive(0)
  #(
    before,
    first == first_monitor && first_pid == worker,
    second == second_monitor && second_pid == worker,
    after,
    dead == dead_monitor && dead_pid == worker,
    atom.to_string(reason_atom),
    empty,
    process.is_alive(worker),
    duplicate,
    already_named,
    unregistered,
  )
}
// @geam:expect #(True, True, True, Error(Nil), True, "noproc", Error(Nil), False, Error(Nil), Error(Nil), Error(Nil))
