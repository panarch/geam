import exit_signals
import gleam/erlang/application
import gleam/erlang/node
import gleam/erlang/process
import gleam/string
import lifecycle
import links
import native_interop
import native_values
import selector_callbacks
import selector_wait
import selectors
import service
import subjects

fn check(scenario: fn() -> Nil) {
  let ready = process.new_subject()
  let child =
    process.spawn_unlinked(fn() {
      let start = process.new_subject()
      process.send(ready, start)
      process.receive_forever(start)
      scenario()
    })
  let monitor = process.monitor(child)
  process.send(process.receive_forever(ready), Nil)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  Nil
}

pub fn main() {
  check(fn() {
    assert subjects.main() == #(22, 42, "untouched", Error(Nil), True, True)
  })
  check(fn() {
    assert selector_wait.main() == #(42, False, True)
  })
  check(fn() {
    assert service.main() == #(20, 42, False)
  })
  check(fn() {
    assert lifecycle.main()
      == #(
        True,
        True,
        True,
        Error(Nil),
        True,
        "noproc",
        Error(Nil),
        False,
        Error(Nil),
        Error(Nil),
        Error(Nil),
      )
  })
  check(fn() {
    assert links.main() == #(True, True, True, "broken", Error(Nil))
  })
  check(fn() {
    assert exit_signals.main() == #(True, True, True, True, True)
  })
  check(fn() {
    assert selectors.main()
      == #(42, -1, True, True, Ok(42), "selector", ["Function"], True)
  })
  check(fn() {
    assert selector_callbacks.main() == #(21, 31, 41, 51, 61, 71, 81)
  })
  check(fn() {
    assert native_values.main()
      == #(
        Ok(42),
        True,
        "packet",
        "hello",
        [],
        "nonode@nohost",
        Error(node.LocalNodeIsNotAlive),
        Error(Nil),
        True,
        False,
      )
  })
  check(fn() {
    assert native_interop.main()
      == #(
        native_interop.RuntimeMarker,
        True,
        True,
        True,
        True,
        True,
        True,
        True,
        True,
        True,
        True,
      )
  })
  check(fn() {
    let assert Ok(root) = application.priv_directory("geam_erlang_test")
    let assert Ok(erlang) = application.priv_directory("gleam_erlang")
    let assert Ok(stdlib) = application.priv_directory("gleam_stdlib")
    assert string.ends_with(string.replace(root, "\\", "/"), "/priv")
    assert string.ends_with(string.replace(erlang, "\\", "/"), "/priv")
    assert string.ends_with(string.replace(stdlib, "\\", "/"), "/priv")
    assert root != erlang && root != stdlib && erlang != stdlib
    assert application.priv_directory("../gleam_stdlib") == Error(Nil)
    assert application.priv_directory("unknown_package") == Error(Nil)
  })
}
// @geam:expect Nil
