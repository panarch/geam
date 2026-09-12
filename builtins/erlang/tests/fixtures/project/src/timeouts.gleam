import gleam/erlang/process

pub fn main() {
  let messages: process.Subject(Int) = process.new_subject()
  let first = process.receive(messages, 10)
  let second =
    process.new_selector()
    |> process.select(messages)
    |> process.selector_receive(10)
  process.sleep(10)
  #(first, second, process.is_alive(process.self()))
}
// @geam:expect #(Error(Nil), Error(Nil), True)
