import gleam/erlang/process

type Request {
  Add(Int, process.Subject(Int))
  Stop
}

fn serve(requests: process.Subject(Request), total: Int) {
  case process.receive_forever(requests) {
    Add(value, reply) -> {
      let total = total + value
      process.send(reply, total)
      serve(requests, total)
    }
    Stop -> Nil
  }
}

pub fn main() {
  let ready = process.new_subject()
  let pid =
    process.spawn(fn() {
      let requests = process.new_subject()
      process.send(ready, requests)
      serve(requests, 0)
    })
  let monitor = process.monitor(pid)
  let requests = process.receive_forever(ready)
  let first = process.call_forever(requests, fn(reply) { Add(20, reply) })
  let second = process.call(requests, 1000, fn(reply) { Add(22, reply) })
  process.send(requests, Stop)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(message) { message })
    |> process.selector_receive_forever
  #(first, second, process.is_alive(pid))
}
// @geam:expect #(20, 42, False)
