import gleam/erlang/process

pub fn main() {
  let messages = process.new_subject()
  let cancelled = process.send_after(messages, 100, -1)
  let before = process.cancel_timer(cancelled)
  let repeated = process.cancel_timer(cancelled)
  let timer = process.send_after(messages, 10, 42)
  let value = process.receive_forever(messages)
  let after = process.cancel_timer(timer)
  #(before, repeated, value, after, process.receive(messages, 0))
}
// @geam:expect #(Cancelled(time_remaining: 100), TimerNotFound, 42, TimerNotFound, Error(Nil))
