import gleam/order
import gleam/time/calendar
import gleam/time/duration

pub fn main() {
  assert calendar.month_to_string(calendar.January) == "January"
  assert calendar.month_to_string(calendar.February) == "February"
  assert calendar.month_to_string(calendar.March) == "March"
  assert calendar.month_to_string(calendar.April) == "April"
  assert calendar.month_to_string(calendar.May) == "May"
  assert calendar.month_to_string(calendar.June) == "June"
  assert calendar.month_to_string(calendar.July) == "July"
  assert calendar.month_to_string(calendar.August) == "August"
  assert calendar.month_to_string(calendar.September) == "September"
  assert calendar.month_to_string(calendar.October) == "October"
  assert calendar.month_to_string(calendar.November) == "November"
  assert calendar.month_to_string(calendar.December) == "December"

  assert calendar.month_to_int(calendar.January) == 1
  assert calendar.month_to_int(calendar.February) == 2
  assert calendar.month_to_int(calendar.March) == 3
  assert calendar.month_to_int(calendar.April) == 4
  assert calendar.month_to_int(calendar.May) == 5
  assert calendar.month_to_int(calendar.June) == 6
  assert calendar.month_to_int(calendar.July) == 7
  assert calendar.month_to_int(calendar.August) == 8
  assert calendar.month_to_int(calendar.September) == 9
  assert calendar.month_to_int(calendar.October) == 10
  assert calendar.month_to_int(calendar.November) == 11
  assert calendar.month_to_int(calendar.December) == 12

  assert calendar.month_from_int(1) == Ok(calendar.January)
  assert calendar.month_from_int(6) == Ok(calendar.June)
  assert calendar.month_from_int(12) == Ok(calendar.December)
  assert calendar.month_from_int(0) == Error(Nil)
  assert calendar.month_from_int(13) == Error(Nil)

  assert calendar.is_leap_year(2000)
  assert calendar.is_leap_year(2024)
  assert !calendar.is_leap_year(1900)
  assert !calendar.is_leap_year(2023)
  assert calendar.is_valid_date(calendar.Date(2024, calendar.February, 29))
  assert !calendar.is_valid_date(calendar.Date(2023, calendar.February, 29))
  assert calendar.is_valid_date(calendar.Date(2023, calendar.December, 31))
  assert !calendar.is_valid_date(calendar.Date(2023, calendar.April, 31))
  assert !calendar.is_valid_date(calendar.Date(2023, calendar.January, 0))

  assert calendar.is_valid_time_of_day(calendar.TimeOfDay(0, 0, 0, 0))
  assert calendar.is_valid_time_of_day(
    calendar.TimeOfDay(23, 59, 59, 999_999_999),
  )
  assert !calendar.is_valid_time_of_day(calendar.TimeOfDay(-1, 0, 0, 0))
  assert !calendar.is_valid_time_of_day(calendar.TimeOfDay(24, 0, 0, 0))
  assert !calendar.is_valid_time_of_day(calendar.TimeOfDay(0, 60, 0, 0))
  assert !calendar.is_valid_time_of_day(calendar.TimeOfDay(0, 0, 60, 0))
  assert !calendar.is_valid_time_of_day(
    calendar.TimeOfDay(0, 0, 0, 1_000_000_000),
  )

  let first = calendar.Date(2024, calendar.March, 1)
  assert calendar.naive_date_compare(first, calendar.Date(2025, calendar.January, 1))
    == order.Lt
  assert calendar.naive_date_compare(first, calendar.Date(2024, calendar.April, 1))
    == order.Lt
  assert calendar.naive_date_compare(first, calendar.Date(2024, calendar.March, 2))
    == order.Lt
  assert calendar.naive_date_compare(first, first) == order.Eq

  assert duration.to_seconds_and_nanoseconds(calendar.utc_offset) == #(0, 0)
  assert duration.to_seconds_and_nanoseconds(calendar.local_offset())
    == #(3600, 0)

  Nil
}
// @geam:expect Nil
