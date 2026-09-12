import gleam/erlang/process

pub fn main() {
  let owner = process.self()
  let messages = process.new_subject()
  process.send(messages, 20)
  process.new_selector()
  |> process.select_map(messages, fn(value) {
    process.sleep(20)
    let assert True = process.self() == owner
    value + 1
  })
  |> process.map_selector(fn(value) {
    process.sleep(20)
    let assert True = process.self() == owner
    value * 2
  })
  |> process.selector_receive(5)
}
// @geam:expect Ok(42)
