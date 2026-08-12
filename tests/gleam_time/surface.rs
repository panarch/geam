use super::{ExpectedSurface, assert_full_project_graph, assert_surface};

const DURATION_SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Day",
        "Hour",
        "Microsecond",
        "Millisecond",
        "Minute",
        "Month",
        "Nanosecond",
        "Second",
        "Week",
        "Year",
        "add",
        "approximate",
        "compare",
        "difference",
        "hours",
        "milliseconds",
        "minutes",
        "nanoseconds",
        "seconds",
        "to_iso8601_string",
        "to_milliseconds",
        "to_seconds",
        "to_seconds_and_nanoseconds",
    ],
    types: &[("Duration", 0), ("Unit", 0)],
    type_aliases: &[],
    constructors: &[
        ("Unit", "Day", 0),
        ("Unit", "Hour", 0),
        ("Unit", "Microsecond", 0),
        ("Unit", "Millisecond", 0),
        ("Unit", "Minute", 0),
        ("Unit", "Month", 0),
        ("Unit", "Nanosecond", 0),
        ("Unit", "Second", 0),
        ("Unit", "Week", 0),
        ("Unit", "Year", 0),
    ],
    functions: r#"
add: fn(Duration, Duration) -> Duration
approximate: fn(Duration) -> #(Int, Unit)
compare: fn(Duration, Duration) -> order.Order
difference: fn(Duration, Duration) -> Duration
hours: fn(Int) -> Duration
milliseconds: fn(Int) -> Duration
minutes: fn(Int) -> Duration
nanoseconds: fn(Int) -> Duration
seconds: fn(Int) -> Duration
to_iso8601_string: fn(Duration) -> String
to_milliseconds: fn(Duration) -> Int
to_seconds: fn(Duration) -> Float
to_seconds_and_nanoseconds: fn(Duration) -> #(Int, Int)
"#,
};

const CALENDAR_SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "April",
        "August",
        "Date",
        "December",
        "February",
        "January",
        "July",
        "June",
        "March",
        "May",
        "November",
        "October",
        "September",
        "TimeOfDay",
        "is_leap_year",
        "is_valid_date",
        "is_valid_time_of_day",
        "local_offset",
        "month_from_int",
        "month_to_int",
        "month_to_string",
        "naive_date_compare",
        "utc_offset",
    ],
    types: &[("Date", 0), ("Month", 0), ("TimeOfDay", 0)],
    type_aliases: &[],
    constructors: &[
        ("Date", "Date", 3),
        ("Month", "April", 0),
        ("Month", "August", 0),
        ("Month", "December", 0),
        ("Month", "February", 0),
        ("Month", "January", 0),
        ("Month", "July", 0),
        ("Month", "June", 0),
        ("Month", "March", 0),
        ("Month", "May", 0),
        ("Month", "November", 0),
        ("Month", "October", 0),
        ("Month", "September", 0),
        ("TimeOfDay", "TimeOfDay", 4),
    ],
    functions: r#"
is_leap_year: fn(Int) -> Bool
is_valid_date: fn(Date) -> Bool
is_valid_time_of_day: fn(TimeOfDay) -> Bool
local_offset: fn() -> duration.Duration
month_from_int: fn(Int) -> Result(Month, Nil)
month_to_int: fn(Month) -> Int
month_to_string: fn(Month) -> String
naive_date_compare: fn(Date, Date) -> Order
"#,
};

const TIMESTAMP_SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "add",
        "compare",
        "difference",
        "from_calendar",
        "from_unix_seconds",
        "from_unix_seconds_and_nanoseconds",
        "parse_rfc3339",
        "subtract",
        "system_time",
        "to_calendar",
        "to_rfc3339",
        "to_unix_seconds",
        "to_unix_seconds_and_nanoseconds",
        "unix_epoch",
    ],
    types: &[("Timestamp", 0)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
add: fn(Timestamp, Duration) -> Timestamp
compare: fn(Timestamp, Timestamp) -> order.Order
difference: fn(Timestamp, Timestamp) -> Duration
from_calendar: fn(date: calendar.Date, time: calendar.TimeOfDay, offset: Duration) -> Timestamp
from_unix_seconds: fn(Int) -> Timestamp
from_unix_seconds_and_nanoseconds: fn(seconds: Int, nanoseconds: Int) -> Timestamp
parse_rfc3339: fn(String) -> Result(Timestamp, Nil)
subtract: fn(Timestamp, Duration) -> Timestamp
system_time: fn() -> Timestamp
to_calendar: fn(Timestamp, Duration) -> #(calendar.Date, calendar.TimeOfDay)
to_rfc3339: fn(Timestamp, Duration) -> String
to_unix_seconds: fn(Timestamp) -> Float
to_unix_seconds_and_nanoseconds: fn(Timestamp) -> #(Int, Int)
"#,
};

#[test]
fn tracks_official_gleam_time_public_surfaces() {
    assert_surface("gleam/time/duration", &DURATION_SURFACE);
    assert_surface("gleam/time/calendar", &CALENDAR_SURFACE);
    assert_surface("gleam/time/timestamp", &TIMESTAMP_SURFACE);
}

#[test]
fn tracks_the_complete_resolved_time_project_graph() {
    assert_full_project_graph();
}
