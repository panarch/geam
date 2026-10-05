import example_process_service as service
import gleam/erlang/process

pub fn main() {
  let name = process.new_name("delayed_calculator")
  let ready = process.new_subject()
  let pid =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, Nil)
      let reply = process.named_subject(name) |> process.receive_forever
      process.sleep(10_000)
      process.send(reply, 42)
    })
  process.receive_forever(ready)
  let monitor = process.monitor(pid)
  let assert Ok(42) = service.request_forever(name, fn(reply) { reply })
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert Error(service.Unavailable) =
    service.request_forever(name, fn(reply) { reply })
  Nil
}
