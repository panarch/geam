import catalog.{Summary}
import counter
import gleam/io
import gleam/json
import pure/labels

pub fn main() {
  let empty = catalog.new()
  let populated = catalog.insert(empty, "one", "alpha")
  let matching = catalog.insert(catalog.new(), "one", "alpha")
  assert empty != populated
  assert populated == matching

  let summary = catalog.summarize(populated, labels.decorate)
  let Summary(count, items) = summary
  assert count == 1
  assert items == ["pure:native:alpha"]

  let first = counter.next("count")
  let second = counter.next("count")
  assert first == "count:3"
  assert second == "count:4"

  io.print_error("provider-before\n")
  echo summary as "provider-summary"
  io.print_error("provider-after\n")
  json.string(first <> "/" <> second)
  |> json.to_string
  |> io.println
}
