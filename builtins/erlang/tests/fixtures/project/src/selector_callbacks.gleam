import gleam/dynamic/decode
import gleam/erlang/process

fn wait(owner: process.Pid, value: Int) -> Int {
  let assert True = process.self() == owner
  let reply = process.new_subject()
  let _ = process.spawn_unlinked(fn() { process.send(reply, value + 1) })
  let result = process.receive_forever(reply)
  let assert True = process.self() == owner
  result
}

pub fn main() {
  let owner = process.self()
  let messages = process.new_subject()
  process.send(messages, 20)
  let ordinary =
    process.new_selector()
    |> process.select_map(messages, fn(value) { wait(owner, value) })
    |> process.selector_receive_forever

  let name = process.new_name("callbacks")
  let assert Ok(Nil) = process.register(owner, name)
  let named = process.named_subject(name)
  process.send(named, 30)
  let by_name =
    process.new_selector()
    |> process.select_map(named, fn(value) { wait(owner, value) })
    |> process.selector_receive_forever

  process.send(named, 40)
  let record =
    process.new_selector()
    |> process.select_record(name, 1, fn(message) {
      let assert Ok(value) = decode.run(message, decode.at([1], decode.int))
      wait(owner, value)
    })
    |> process.selector_receive_forever

  process.send(messages, 50)
  let other =
    process.new_selector()
    |> process.select_other(fn(message) {
      let assert Ok(value) = decode.run(message, decode.at([1], decode.int))
      wait(owner, value)
    })
    |> process.selector_receive_forever

  let child = process.spawn_unlinked(fn() { Nil })
  let monitor = process.monitor(child)
  let specific =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) {
      let assert process.ProcessDown(actual, pid, _) = down
      let assert True = actual == monitor && pid == child
      wait(owner, 60)
    })
    |> process.selector_receive_forever

  let another = process.spawn_unlinked(fn() { Nil })
  let another_monitor = process.monitor(another)
  let general =
    process.new_selector()
    |> process.select_monitors(fn(down) {
      let assert process.ProcessDown(actual, pid, _) = down
      let assert True = actual == another_monitor && pid == another
      wait(owner, 70)
    })
    |> process.selector_receive_forever

  process.trap_exits(True)
  let linked = process.spawn(fn() { Nil })
  let trapped =
    process.new_selector()
    |> process.select_trapped_exits(fn(exit) {
      let assert process.ExitMessage(pid, process.Normal) = exit
      let assert True = pid == linked
      wait(owner, 79)
    })
    |> process.map_selector(fn(value) { wait(owner, value) })
    |> process.selector_receive_forever
  #(ordinary, by_name, record, other, specific, general, trapped)
}
// @geam:expect #(21, 31, 41, 51, 61, 71, 81)
