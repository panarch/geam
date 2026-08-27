import gleam/order
import gleam/time/calendar
import gleam/time/duration
import gleam/time/timestamp

pub fn main() {
  assert timestamp.to_unix_seconds_and_nanoseconds(timestamp.unix_epoch)
    == #(0, 0)
  assert timestamp.to_unix_seconds_and_nanoseconds(timestamp.system_time())
    == #(1_700_000_000, 123_456_789)

  let one = timestamp.from_unix_seconds(1)
  let normalized = timestamp.from_unix_seconds_and_nanoseconds(1, -1)
  assert timestamp.to_unix_seconds_and_nanoseconds(normalized)
    == #(0, 999_999_999)
  assert timestamp.to_unix_seconds(
      timestamp.from_unix_seconds_and_nanoseconds(1, 500_000_000),
    )
    == 1.5
  assert timestamp.compare(one, timestamp.from_unix_seconds(2)) == order.Lt

  let advanced = timestamp.add(one, duration.milliseconds(1500))
  assert timestamp.to_unix_seconds_and_nanoseconds(advanced)
    == #(2, 500_000_000)
  assert timestamp.subtract(advanced, duration.nanoseconds(500_000_001))
    |> timestamp.to_unix_seconds_and_nanoseconds
    == #(1, 999_999_999)
  assert timestamp.difference(one, advanced)
    |> duration.to_seconds_and_nanoseconds
    == #(1, 500_000_000)

  let date = calendar.Date(2024, calendar.December, 25)
  let time = calendar.TimeOfDay(12, 30, 50, 123_000_000)
  let offset = duration.hours(1)
  let christmas = timestamp.from_calendar(date:, time:, offset:)
  assert timestamp.to_calendar(christmas, offset) == #(date, time)
  assert timestamp.to_rfc3339(christmas, offset)
    == "2024-12-25T12:30:50.123+01:00"

  Nil
}
// @geam:expect Nil
