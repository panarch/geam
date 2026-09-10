import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/string

fn observe_failure(operation: fn() -> Nil, message: String) {
  let ready = process.new_subject()
  let child =
    process.spawn(fn() {
      let start = process.new_subject()
      process.send(ready, start)
      process.receive_forever(start)
      operation()
    })
  let monitor = process.monitor(child)
  process.send(process.receive_forever(ready), Nil)
  let assert process.ProcessDown(_, down_pid, process.Abnormal(down)) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(message) { message })
    |> process.selector_receive_forever
  let assert process.ExitMessage(exit_pid, process.Abnormal(exit)) =
    process.new_selector()
    |> process.select_trapped_exits(fn(message) { message })
    |> process.selector_receive_forever
  let assert True = down == exit
  let assert Ok(#(tag, description)) =
    decode.run(down, {
      use tag <- decode.field(0, atom.decoder())
      use description <- decode.field(1, decode.string)
      decode.success(#(atom.to_string(tag), description))
    })
  down_pid == child
  && exit_pid == child
  && tag == "geam_execution_error"
  && string.contains(description, message)
}

pub fn main() {
  process.trap_exits(True)
  let source =
    observe_failure(
      fn() { panic as "intentional source failure" },
      "intentional source failure",
    )
  let provider =
    observe_failure(
      fn() {
        let _ = atom.create(string.repeat("x", 256))
        Nil
      },
      "atom name exceeds 255 Unicode codepoints",
    )
  let sleep = observe_failure(fn() { process.sleep(-1) }, "timeout")
  let invalid_name =
    observe_failure(
      fn() {
        let _ = process.new_name(string.repeat("x", 256))
        Nil
      },
      "atom name exceeds 255 Unicode codepoints",
    )
  let selected_callback =
    observe_failure(
      fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        let _ =
          process.new_selector()
          |> process.select_map(subject, fn(_) { panic as "selected callback" })
          |> process.selector_receive(0)
        Nil
      },
      "selected callback",
    )
  let mapped_callback =
    observe_failure(
      fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        process.new_selector()
        |> process.select_map(subject, fn(_) { Nil })
        |> process.map_selector(fn(_) { panic as "mapped callback" })
        |> process.selector_receive_forever
      },
      "mapped callback",
    )
  let selected_forever =
    observe_failure(
      fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        process.new_selector()
        |> process.select_map(subject, fn(_) { panic as "selected forever" })
        |> process.selector_receive_forever
      },
      "selected forever",
    )
  let mapped_timed =
    observe_failure(
      fn() {
        let subject = process.new_subject()
        process.send(subject, Nil)
        let _ =
          process.new_selector()
          |> process.select_map(subject, fn(_) { Nil })
          |> process.map_selector(fn(_) { panic as "mapped timed" })
          |> process.selector_receive(0)
        Nil
      },
      "mapped timed",
    )
  let invalid_atom =
    observe_failure(
      fn() {
        let _ = atom.to_string(atom.cast_from_dynamic(dynamic.int(42)))
        Nil
      },
      "atom_to_binary requires an atom",
    )
  let negative_timer =
    observe_failure(
      fn() {
        let _ = process.send_after(process.new_subject(), -1, Nil)
        Nil
      },
      "timer timeout",
    )
  let overflowing_timer =
    observe_failure(
      fn() {
        let name = process.new_name("invalid_timer")
        let subject = process.named_subject(name)
        let _ = process.send_after(subject, 18_446_744_073_709_551_616, Nil)
        Nil
      },
      "timer timeout",
    )
  let empty =
    process.new_selector()
    |> process.select_monitors(fn(_) { Nil })
    |> process.select_trapped_exits(fn(_) { Nil })
    |> process.selector_receive(0)
  process.trap_exits(False)
  #(
    source,
    provider,
    sleep,
    invalid_name,
    selected_callback,
    mapped_callback,
    selected_forever,
    mapped_timed,
    invalid_atom,
    negative_timer,
    overflowing_timer,
    empty,
  )
}
// @geam:expect #(True, True, True, True, True, True, True, True, True, True, True, Error(Nil))
