import gleam/time/calendar
import gleam/time/duration
import gleam/time/timestamp

pub fn main() {
  let first_time = timestamp.system_time()
  let first_offset = calendar.local_offset()
  let second_time = timestamp.system_time()
  let second_offset = calendar.local_offset()

  assert timestamp.to_unix_seconds_and_nanoseconds(first_time) == #(5, 0)
  assert duration.to_seconds_and_nanoseconds(first_offset) == #(3600, 0)
  assert timestamp.to_unix_seconds_and_nanoseconds(second_time)
    == #(-1, 999_999_999)
  assert duration.to_seconds_and_nanoseconds(second_offset) == #(-18_000, 0)

  Nil
}
// @geam:expect Nil
