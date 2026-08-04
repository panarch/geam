import gleam/order
import gleam/time/duration

pub fn main() {
  assert duration.to_seconds_and_nanoseconds(duration.seconds(5)) == #(5, 0)
  assert duration.to_seconds_and_nanoseconds(duration.minutes(2)) == #(120, 0)
  assert duration.to_seconds_and_nanoseconds(duration.hours(2)) == #(7200, 0)
  assert duration.to_seconds_and_nanoseconds(duration.milliseconds(-1501))
    == #(-2, 499_000_000)
  assert duration.to_seconds_and_nanoseconds(duration.nanoseconds(-1))
    == #(-1, 999_999_999)

  assert duration.add(duration.seconds(2), duration.milliseconds(1500))
    |> duration.to_seconds_and_nanoseconds
    == #(3, 500_000_000)
  assert duration.difference(duration.seconds(2), duration.seconds(5))
    |> duration.to_seconds_and_nanoseconds
    == #(3, 0)
  assert duration.compare(duration.seconds(-2), duration.seconds(1))
    == order.Gt

  assert duration.approximate(duration.nanoseconds(999))
    == #(999, duration.Nanosecond)
  assert duration.approximate(duration.nanoseconds(2000))
    == #(2, duration.Microsecond)
  assert duration.approximate(duration.milliseconds(2))
    == #(2, duration.Millisecond)
  assert duration.approximate(duration.seconds(2)) == #(2, duration.Second)
  assert duration.approximate(duration.minutes(2)) == #(2, duration.Minute)
  assert duration.approximate(duration.hours(2)) == #(2, duration.Hour)
  assert duration.approximate(duration.hours(48)) == #(2, duration.Day)
  assert duration.approximate(duration.hours(24 * 14)) == #(2, duration.Week)
  assert duration.approximate(duration.seconds(2_629_800))
    == #(1, duration.Month)
  assert duration.approximate(duration.seconds(31_557_600))
    == #(1, duration.Year)

  assert duration.to_iso8601_string(duration.seconds(0)) == "PT0S"
  assert duration.to_iso8601_string(
      duration.add(duration.hours(26), duration.milliseconds(1250)),
    )
    == "P1DT2H1.25S"
  assert duration.to_seconds(duration.milliseconds(1500)) == 1.5
  assert duration.to_milliseconds(
      duration.add(duration.seconds(-2), duration.nanoseconds(500_000_000)),
    )
    == -1500

  Nil
}
// @geam:expect Nil
