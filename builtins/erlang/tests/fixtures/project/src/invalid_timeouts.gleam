import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/list
import gleam/string

fn invalid(operation: fn() -> Nil) {
  let ready = process.new_subject()
  let child =
    process.spawn_unlinked(fn() {
      let start = process.new_subject()
      process.send(ready, start)
      process.receive_forever(start)
      operation()
    })
  let monitor = process.monitor(child)
  process.send(process.receive_forever(ready), Nil)
  let assert process.ProcessDown(_, pid, process.Abnormal(reason)) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(message) { message })
    |> process.selector_receive_forever
  let assert Ok(#(tag, description)) =
    decode.run(reason, {
      use tag <- decode.field(0, atom.decoder())
      use description <- decode.field(1, decode.string)
      decode.success(#(atom.to_string(tag), description))
    })
  pid == child
  && tag == "geam_execution_error"
  && string.contains(description, "timeout")
}

pub fn main() {
  list.map([-1, 4_294_967_296, 18_446_744_073_709_551_616], fn(timeout) {
    let receive =
      invalid(fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        let _ = process.receive(subject, timeout)
        Nil
      })
    let select =
      invalid(fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        let _ =
          process.new_selector()
          |> process.select(subject)
          |> process.selector_receive(timeout)
        Nil
      })
    #(receive, select)
  })
}
// @geam:expect [#(True, True), #(True, True), #(True, True)]
