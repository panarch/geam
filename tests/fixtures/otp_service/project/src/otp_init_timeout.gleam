import gleam/erlang/process
import gleam/io
import gleam/otp/actor

pub fn main() {
  let entered = process.new_subject()
  let finished = process.new_subject()
  let _ =
    process.spawn(fn() {
      let result =
        actor.new_with_initialiser(5, fn(_: process.Subject(Nil)) {
          process.send(entered, process.self())
          process.sleep_forever()
          Ok(actor.initialised(Nil))
        })
        |> actor.start
      process.send(finished, result)
    })
  let child = process.receive_forever(entered)
  let assert True = process.is_alive(child)
  let assert Error(actor.InitTimeout) = process.receive_forever(finished)
  let assert False = process.is_alive(child)
  io.println("original actor: entered initialiser times out and is killed")
}
