import gleam/erlang/process

pub fn main() {
  let ready = process.new_subject()
  let sleeper =
    process.spawn_unlinked(fn() {
      process.send(ready, Nil)
      process.sleep(18_446_744_073_709_551_616)
    })
  let monitor = process.monitor(sleeper)
  process.receive_forever(ready)
  process.sleep(0)
  process.sleep(4_294_967_302)
  let alive = process.is_alive(sleeper)
  process.kill(sleeper)
  let assert process.ProcessDown(ref, pid, process.Killed) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(message) { message })
    |> process.selector_receive_forever
  #(alive, ref == monitor, pid == sleeper, process.is_alive(sleeper))
}
// @geam:expect #(True, True, True, False)
