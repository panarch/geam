import example_process_service as service
import gleam/erlang/process

type Request {
  Add(Int, process.Subject(Int))
  Stop
}

pub fn main() {
  let name = process.new_name("deadline_calculator")
  let ready = process.new_subject()
  let pid =
    process.spawn_unlinked(fn() {
      let assert Ok(Nil) = process.register(process.self(), name)
      process.send(ready, Nil)
      worker(process.named_subject(name))
    })
  process.receive_forever(ready)
  let assert Ok(42) = service.request(name, Add(35, _), 5)

  let silent_name = process.new_name("deadline_silent")
  let silent = process.spawn_unlinked(fn() { process.sleep_forever() })
  let assert Ok(Nil) = process.register(silent, silent_name)
  let assert Error(service.InvalidTimeout) =
    service.request(silent_name, Add(1, _), 18_446_744_073_709_551_616)
  let assert Error(service.TimedOut) =
    service.request(silent_name, Add(1, _), 5)
  let assert True = process.is_alive(silent)
  process.kill(silent)

  let assert Error(service.TargetExited) =
    service.request(name, fn(_) { Stop }, 5)
  let assert False = process.is_alive(pid)
  Nil
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
