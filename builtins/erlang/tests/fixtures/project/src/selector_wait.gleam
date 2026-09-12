import gleam/erlang/process

pub fn main() {
  let acknowledgements = process.new_subject()
  let ready = process.new_subject()
  let worker =
    process.spawn_unlinked(fn() {
      let requests = process.new_subject()
      process.send(ready, requests)
      let value = process.receive_forever(requests)
      process.send(acknowledgements, value + 1)
    })
  let monitor = process.monitor(worker)
  let requests = process.receive_forever(ready)
  let owner = process.self()
  let subject = process.new_subject()
  let name = process.new_name("waiting")
  let assert Ok(Nil) = process.register(owner, name)
  let named = process.named_subject(name)
  process.send(named, 20)
  let selector =
    process.new_selector()
    |> process.select_map(subject, fn(value) { value })
    |> process.select_map(named, fn(value) {
      let assert True = process.self() == owner
      process.send(requests, value)
      let response = process.receive_forever(acknowledgements)
      let assert True = process.self() == owner
      response
    })
    |> process.map_selector(fn(value) { value * 2 })
  let result = process.selector_receive_forever(selector)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(message) { message })
    |> process.selector_receive_forever
  #(result, process.is_alive(worker), process.self() == owner)
}
// @geam:expect #(42, False, True)
