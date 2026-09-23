import gleam/erlang/atom
import gleam/erlang/process
import gleam/io
import gleam/otp/actor
import gleam/otp/static_supervisor as static
import gleam/otp/supervision

pub fn main() {
  let children = process.new_subject()
  let child =
    supervision.worker(fn() {
      let ready = process.new_subject()
      let pid =
        process.spawn(fn() {
          process.trap_exits(True)
          process.send(ready, Nil)
          process.sleep_forever()
        })
      process.receive_forever(ready)
      process.send(children, pid)
      Ok(actor.Started(pid, Nil))
    })
    |> supervision.timeout(5)
  let assert Ok(actor.Started(supervisor, _)) =
    static.new(static.OneForOne)
    |> static.add(child)
    |> static.start
  let assert Ok(child) = process.receive(children, 0)
  let monitor = process.monitor(supervisor)
  process.unlink(supervisor)
  process.send_abnormal_exit(supervisor, atom.create("shutdown"))
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(child)
  let assert False = process.is_alive(supervisor)
  io.println("original supervisor: finite shutdown expires and kills child")
}
