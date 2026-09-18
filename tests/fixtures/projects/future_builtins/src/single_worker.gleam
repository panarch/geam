import gleam/erlang/process

fn spin(count: Int) -> Nil {
  spin(count + 1)
}

pub fn main() {
  let ready = process.new_subject()
  let _ =
    process.spawn_unlinked(fn() {
      process.send(ready, Nil)
      spin(0)
    })
  process.receive_forever(ready)
  let _ =
    process.spawn_unlinked(fn() {
      process.sleep(1)
      process.send(ready, Nil)
    })
  process.receive_forever(ready)
  echo 42
  Nil
}
