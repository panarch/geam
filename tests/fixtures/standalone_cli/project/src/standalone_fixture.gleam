import catalog.{Summary}
import counter
import gleam/io
import gleam/json
import gleam/list
import gleam/time/calendar
import gleam/time/duration
import gleam/time/timestamp
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

  let shuffled = list.shuffle([1, 2, 3])
  assert list.length(shuffled) == 3
  assert list.contains(shuffled, 1)
  assert list.contains(shuffled, 2)
  assert list.contains(shuffled, 3)

  let now = timestamp.system_time()
  let one_second = duration.seconds(1)
  assert timestamp.difference(now, timestamp.add(now, one_second)) == one_second
  let offset = calendar.local_offset()
  let #(offset_seconds, offset_nanoseconds) =
    duration.to_seconds_and_nanoseconds(offset)
  assert offset_nanoseconds == 0
  assert offset == duration.seconds(offset_seconds)

  io.print_error("provider-before\n")
  echo summary as "provider-summary"
  io.print_error("provider-after\n")
  json.string(first <> "/" <> second)
  |> json.to_string
  |> io.println
}
