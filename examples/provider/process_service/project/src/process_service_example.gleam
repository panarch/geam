import example_process_service as service
import gleam/erlang/process
import gleam/io

type Request {
  Add(Int, process.Subject(Int))
  Stop
}

pub fn main() {
  let name = process.new_name("calculator")
  let ready = process.new_subject()
  let pid =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, Nil)
      worker(process.named_subject(name))
    })
  process.receive_forever(ready)
  let assert True = process.is_alive(pid)
  let assert Ok(42) = service.request_forever(name, Add(35, _))
  let assert Ok(17) = service.request_forever(name, Add(10, _))
  io.println("named service replied: 42, 17")

  let silent_name = process.new_name("silent")
  let silent = process.spawn_unlinked(fn() { process.sleep_forever() })
  let assert Ok(Nil) = process.register(silent, silent_name)
  let assert Error(service.TimedOut) =
    service.request(silent_name, Add(1, _), 0)
  let assert Error(service.InvalidTimeout) =
    service.request(name, Add(1, _), -1)
  process.kill(silent)
  let assert Error(service.Unavailable) =
    service.request(silent_name, Add(1, _), 0)
  io.println("request timeout and unavailable name handled")

  let monitor = process.monitor(pid)
  process.send(process.named_subject(name), Stop)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(pid)
  let assert Error(service.Unavailable) = service.request(name, Add(1, _), 0)
  io.println("worker stopped and name released")
}

fn worker(inbox: process.Subject(Request)) {
  case process.receive_forever(inbox) {
    Add(value, reply) -> {
      process.send(reply, value + 7)
      worker(inbox)
    }
    Stop -> Nil
  }
}
