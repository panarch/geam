import gleam/erlang/process

@external(erlang, "host", "both")
fn both(callback: fn(Int) -> Int) -> #(Int, Int)

pub fn main() {
  let parent = process.self()
  both(fn(value) {
    let assert True = process.self() == parent
    let reply = process.new_subject()
    process.spawn_unlinked(fn() {
      process.sleep(10)
      process.send(reply, value)
    })
    process.receive_forever(reply)
  })
}
