use super::TimeSource;
use crate::HostFailure;
use jiff::{Timestamp, tz::TimeZone};
use num_bigint::BigInt;
use std::time::{SystemTime, UNIX_EPOCH};

/// A wall-clock source backed by the operating system clock and time zone.
#[derive(Debug, Clone, Copy)]
pub struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn system_time(&mut self) -> Result<SystemTime, HostFailure> {
        Ok(SystemTime::now())
    }

    fn local_offset_seconds(&mut self) -> Result<i32, HostFailure> {
        current_offset_seconds(
            map_system_time_zone(TimeZone::try_system()),
            Timestamp::now(),
        )
    }
}

pub(super) fn split_system_time(time: SystemTime) -> (BigInt, BigInt) {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => (
            BigInt::from(duration.as_secs()),
            BigInt::from(duration.subsec_nanos()),
        ),
        Err(error) => {
            let duration = error.duration();
            let seconds = BigInt::from(duration.as_secs());
            let nanoseconds = duration.subsec_nanos();
            if nanoseconds == 0 {
                (-seconds, BigInt::from(0))
            } else {
                (-seconds - 1, BigInt::from(1_000_000_000u32 - nanoseconds))
            }
        }
    }
}

fn map_system_time_zone<Error>(result: Result<TimeZone, Error>) -> Result<TimeZone, HostFailure> {
    result.map_err(|_| HostFailure::new("could not determine the current local UTC offset"))
}

fn current_offset_seconds(
    time_zone: Result<TimeZone, HostFailure>,
    timestamp: Timestamp,
) -> Result<i32, HostFailure> {
    Ok(time_zone?.to_offset(timestamp).seconds())
}

#[cfg(test)]
mod tests {
    use super::{
        SystemTimeSource, current_offset_seconds, map_system_time_zone, split_system_time,
    };
    use crate::gleam_time::TimeSource;
    use jiff::{
        Timestamp,
        tz::{TimeZone, offset},
    };
    use num_bigint::BigInt;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn splits_system_time_into_canonical_seconds_and_nanoseconds() {
        for (time, expected) in [
            (UNIX_EPOCH, (BigInt::from(0), BigInt::from(0))),
            (
                UNIX_EPOCH + Duration::from_nanos(1),
                (BigInt::from(0), BigInt::from(1)),
            ),
            (
                UNIX_EPOCH + Duration::new(100_000_000_000, 999_999_999),
                (
                    BigInt::from(100_000_000_000u64),
                    BigInt::from(999_999_999u32),
                ),
            ),
            (
                UNIX_EPOCH - Duration::from_nanos(1),
                (BigInt::from(-1), BigInt::from(999_999_999u32)),
            ),
            (
                UNIX_EPOCH - Duration::from_secs(1),
                (BigInt::from(-1), BigInt::from(0)),
            ),
            (
                UNIX_EPOCH - Duration::new(2, 3),
                (BigInt::from(-3), BigInt::from(999_999_997u32)),
            ),
        ] {
            assert_eq!(split_system_time(time), expected);
        }
    }

    #[test]
    fn maps_system_time_zone_discovery_without_a_silent_fallback() {
        let tokyo = TimeZone::fixed(offset(9));
        let failure = map_system_time_zone::<()>(Err(()))
            .expect_err("failed discovery should remain a host failure");

        assert_eq!(
            map_system_time_zone::<()>(Ok(tokyo.clone()))
                .expect("known time zone should remain available"),
            tokyo,
        );
        assert_eq!(
            current_offset_seconds(Ok(tokyo), Timestamp::UNIX_EPOCH)
                .expect("known time zone should have an offset"),
            32_400,
        );
        assert_eq!(
            current_offset_seconds(Err(failure), Timestamp::UNIX_EPOCH)
                .expect_err("failed discovery should remain a host failure")
                .message(),
            "could not determine the current local UTC offset",
        );
    }

    #[test]
    fn system_source_reads_the_current_wall_clock_and_offset_fallibly() {
        let before = SystemTime::now();
        let mut source = SystemTimeSource;
        let current = source
            .system_time()
            .expect("the operating system wall clock should be available");
        let after = SystemTime::now();

        assert!(current >= before);
        assert!(current <= after);
        assert!(
            source
                .local_offset_seconds()
                .map_or(true, |offset| (-86_400..=86_400).contains(&offset)),
        );
    }
}
