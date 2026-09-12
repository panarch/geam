import gleam/dynamic/decode
import gleam/erlang/process

pub fn main() {
  process.trap_exits(True)
  let ready = process.new_subject()
  let child =
    process.spawn(fn() {
      let finish = process.new_subject()
      process.send(ready, finish)
      process.receive_forever(finish)
    })
  let finish = process.receive_forever(ready)
  let linked = process.link(child)
  process.send(finish, Nil)
  let exits =
    process.new_selector()
    |> process.select_trapped_exits(fn(message) { message })
  let assert process.ExitMessage(normal_pid, process.Normal) =
    process.selector_receive_forever(exits)
  let abnormal_child = process.spawn_unlinked(process.sleep_forever)
  let assert True = process.link(abnormal_child)
  process.send_abnormal_exit(abnormal_child, "broken")
  let assert process.ExitMessage(abnormal_pid, process.Abnormal(reason)) =
    process.selector_receive_forever(exits)
  let assert Ok(reason) = decode.run(reason, decode.string)
  let unlinked = process.spawn_unlinked(process.sleep_forever)
  let assert True = process.link(unlinked)
  process.unlink(unlinked)
  process.kill(unlinked)
  let empty = process.selector_receive(exits, 0)
  process.trap_exits(False)
  #(linked, normal_pid == child, abnormal_pid == abnormal_child, reason, empty)
}
// @geam:expect #(True, True, True, "broken", Error(Nil))
