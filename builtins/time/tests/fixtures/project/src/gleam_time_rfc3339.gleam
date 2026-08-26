import gleam/result
import gleam/time/duration
import gleam/time/timestamp

fn parts(source: String) -> Result(#(Int, Int), Nil) {
  timestamp.parse_rfc3339(source)
  |> result.map(timestamp.to_unix_seconds_and_nanoseconds)
}

pub fn main() {
  assert parts("1970-01-01T00:00:00Z") == Ok(#(0, 0))
  assert parts("1970-01-01t00:00:00z") == Ok(#(0, 0))
  assert parts("1970-01-01 01:00:00+01:00") == Ok(#(0, 0))
  assert parts("1969-12-31T23:00:00-01:00") == Ok(#(0, 0))
  assert parts("1970-01-01T00:00:00.1234567899Z")
    == Ok(#(0, 123_456_789))
  assert parts("2000-02-29T23:59:60Z") == Ok(#(951_868_800, 0))

  assert parts("") == Error(Nil)
  assert parts("1995-10-31") == Error(Nil)
  assert parts("1900-02-29T00:00:00Z") == Error(Nil)
  assert parts("2024-13-01T00:00:00Z") == Error(Nil)
  assert parts("2024-01-01T24:00:00Z") == Error(Nil)
  assert parts("2024-01-01T00:60:00Z") == Error(Nil)
  assert parts("2024-01-01T00:00:61Z") == Error(Nil)
  assert parts("2024-01-01T00:00:00.Z") == Error(Nil)
  assert parts("2024-01-01T00:00:00Z trailing") == Error(Nil)

  assert timestamp.to_rfc3339(timestamp.unix_epoch, duration.seconds(0))
    == "1970-01-01T00:00:00Z"
  assert timestamp.to_rfc3339(
      timestamp.from_unix_seconds_and_nanoseconds(0, 120_000_000),
      duration.seconds(0),
    )
    == "1970-01-01T00:00:00.12Z"
  assert timestamp.to_rfc3339(timestamp.from_unix_seconds(-1), duration.seconds(0))
    == "1969-12-31T23:59:59Z"
  assert timestamp.to_rfc3339(timestamp.unix_epoch, duration.seconds(3661))
    == "1970-01-01T01:01:00+01:01"
  assert timestamp.to_rfc3339(timestamp.unix_epoch, duration.hours(-1))
    == "1969-12-31T23:00:00-01:00"

  Nil
}
// @geam:expect Nil
