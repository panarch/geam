import gleam/erlang/process
import gleam/int
import gleam/io

pub type Request {
  Add(Int, process.Subject(Int))
  Stop(process.Subject(Nil))
}

pub fn start() -> #(process.Pid, process.Subject(Request)) {
  let ready = process.new_subject()
  let pid =
    process.spawn_unlinked(fn() {
      let requests = process.new_subject()
      process.send(ready, requests)
      serve(0, requests)
    })
  #(pid, process.receive_forever(ready))
}

fn serve(total: Int, requests: process.Subject(Request)) -> Nil {
  case process.receive_forever(requests) {
    Add(amount, reply) -> {
      let total = total + amount
      process.send(reply, total)
      serve(total, requests)
    }
    Stop(reply) -> process.send(reply, Nil)
  }
}

pub fn add(requests: process.Subject(Request), amount: Int) -> Int {
  process.call_forever(requests, fn(reply) { Add(amount, reply) })
}

pub fn is_alive(pid: process.Pid) -> Bool {
  process.is_alive(pid)
}

pub fn stop(pid: process.Pid, requests: process.Subject(Request)) -> Bool {
  let monitor = process.monitor(pid)
  process.call_forever(requests, Stop)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  !process.is_alive(pid)
}

pub fn main() {
  let #(pid, requests) = start()
  add(requests, 20) |> int.to_string |> io.println
  add(requests, 22) |> int.to_string |> io.println
  let assert True = stop(pid, requests)
}
