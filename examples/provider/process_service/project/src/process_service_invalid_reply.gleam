import example_process_service as service
import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/process

@external(erlang, "fixture", "erase")
fn erase(value: anything) -> dynamic.Dynamic

pub fn main() {
  let name = process.new_name("malformed_reply")
  let ready = process.new_subject()
  let worker =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, Nil)
      let reply: process.Subject(Int) =
        process.named_subject(name) |> process.receive_forever
      let assert Ok(owner) = process.subject_owner(reply)
      // Model a native sender that violates the reply contract. The same tag
      // reaches the waiting request, but its payload is a String instead of Int.
      let assert Ok(tag) =
        decode.run(erase(reply), decode.at([2], decode.dynamic))
      let untyped = process.unsafely_create_subject(owner, tag)
      process.send(untyped, "wrong reply type")
    })
  process.receive_forever(ready)
  let assert Error(service.InvalidReply) =
    service.request(name, fn(reply) { reply }, 1000)
  let assert False = process.is_alive(worker)
  Nil
}
