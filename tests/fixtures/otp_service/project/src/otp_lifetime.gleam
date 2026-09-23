import gleam/erlang/process
import gleam/otp/actor
import gleam/otp/static_supervisor as static
import gleam/otp/supervision
import gleam/otp/system

pub type Token

@external(erlang, "fixture", "token")
fn token() -> Token

@external(erlang, "fixture", "touch")
fn touch(token: Token) -> Nil

// The constructor frame and its original token handle have returned before
// the host cancels the still-running supervisor and its retained callback.
fn start_supervisor() {
  let retained = token()
  static.new(static.OneForOne)
  |> static.add(
    supervision.worker(fn() {
      touch(retained)
      actor.new(Nil)
      |> actor.on_message(fn(state, _: Nil) { actor.continue(state) })
      |> actor.start
    }),
  )
  |> static.start
}

pub fn main() {
  let assert Ok(_) = start_supervisor()
  let entering = process.new_subject()
  let _ =
    process.spawn_unlinked(fn() {
      process.send(entering, Nil)
      system.get_state(process.self())
    })
  process.receive_forever(entering)
  process.sleep_forever()
}
