import gleam/erlang/process

pub fn main() {
  let absent: process.Subject(Int) =
    process.named_subject(process.new_name("absent_recipient"))
  let undeliverable = process.send_after(absent, 10, 0)
  let messages = process.new_subject()
  let sender =
    process.spawn_unlinked(fn() {
      let _ = process.send_after(messages, 10, 42)
      Nil
    })
  let sender_monitor = process.monitor(sender)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(sender_monitor, fn(down) { down })
    |> process.selector_receive_forever

  let ready = process.new_subject()
  let name = process.new_name("timer_recipient")
  let target =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, process.new_subject())
      process.sleep_forever()
    })
  let target_monitor = process.monitor(target)
  let destination = process.receive_forever(ready)
  let pid_timer = process.send_after(destination, 100, 1)
  let named = process.named_subject(name)
  let name_timer = process.send_after(named, 10, 99)
  process.kill(target)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(target_monitor, fn(down) { down })
    |> process.selector_receive_forever
  let cancelled_by_exit = process.cancel_timer(pid_timer)
  let dead_timer = process.send_after(destination, 100, 2)
  let cancelled_at_creation = process.cancel_timer(dead_timer)
  let assert Ok(Nil) = process.register(process.self(), name)
  let first = process.receive_forever(messages)
  let second = process.receive_forever(named)
  #(
    first,
    second,
    cancelled_by_exit,
    cancelled_at_creation,
    process.cancel_timer(name_timer),
    process.is_alive(sender),
    process.cancel_timer(undeliverable),
  )
}
// @geam:expect #(42, 99, TimerNotFound, TimerNotFound, TimerNotFound, False, TimerNotFound)
