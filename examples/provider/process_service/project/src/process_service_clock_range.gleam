import example_process_service as service
import gleam/erlang/process

pub fn main() {
  let name = process.new_name("clock_range")
  let assert Ok(Nil) = process.register(process.self(), name)
  let _ = service.request(name, fn(_: process.Subject(Int)) { Nil }, 1000)
  Nil
}
