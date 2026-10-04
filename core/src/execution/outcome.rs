use num_bigint::BigInt;
use std::fmt;

/// Completion of one execution domain, after its workers have been released.
#[derive(Debug, PartialEq)]
pub enum ExecutionOutcome<Value> {
    Returned(Value),
    Exited(ExitStatus),
}

/// A checked application status, portable across supported operating systems.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExitStatus(u8);

/// A requested application status is outside the portable range 0..=255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("application exit status must be between 0 and 255")]
pub struct InvalidExitStatus;

impl ExitStatus {
    pub const fn new(status: u8) -> Self {
        Self(status)
    }

    pub const fn code(self) -> u8 {
        self.0
    }
}

impl TryFrom<i64> for ExitStatus {
    type Error = InvalidExitStatus;

    fn try_from(status: i64) -> Result<Self, Self::Error> {
        u8::try_from(status)
            .map(Self)
            .map_err(|_| InvalidExitStatus)
    }
}

impl TryFrom<&BigInt> for ExitStatus {
    type Error = InvalidExitStatus;

    fn try_from(status: &BigInt) -> Result<Self, Self::Error> {
        u8::try_from(status)
            .map(Self)
            .map_err(|_| InvalidExitStatus)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<Value> ExecutionOutcome<Value> {
    /// Returns the completed value, or the application's intentional status.
    pub fn try_into_value(self) -> Result<Value, ExitStatus> {
        match self {
            Self::Returned(value) => Ok(value),
            Self::Exited(status) => Err(status),
        }
    }
}

impl<Value, Error> ExecutionOutcome<Result<Value, Error>> {
    pub(crate) fn transpose(self) -> Result<ExecutionOutcome<Value>, Error> {
        match self {
            Self::Returned(value) => value.map(ExecutionOutcome::Returned),
            Self::Exited(status) => Ok(ExecutionOutcome::Exited(status)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionOutcome, ExitStatus, InvalidExitStatus};
    use num_bigint::BigInt;

    #[test]
    fn status_conversion_checks_the_full_portable_range() {
        for code in [0, 7, 255] {
            let status = ExitStatus::new(code);
            assert_eq!(status.code(), code);
            assert_eq!(ExitStatus::try_from(i64::from(code)), Ok(status));
            assert_eq!(ExitStatus::try_from(&BigInt::from(code)), Ok(status));
            assert_eq!(status.to_string(), code.to_string());
        }
        for code in [i64::MIN, -1, 256, i64::MAX] {
            assert_eq!(ExitStatus::try_from(code), Err(InvalidExitStatus));
            assert_eq!(
                ExitStatus::try_from(&BigInt::from(code)),
                Err(InvalidExitStatus)
            );
        }
        assert_eq!(
            ExitStatus::try_from(&(BigInt::from(1) << 128)),
            Err(InvalidExitStatus)
        );
        assert_eq!(
            InvalidExitStatus.to_string(),
            "application exit status must be between 0 and 255"
        );
    }

    #[test]
    fn returned_values_and_intentional_statuses_remain_distinct() {
        assert_eq!(ExecutionOutcome::Returned(42).try_into_value(), Ok(42));
        assert_eq!(
            ExecutionOutcome::<i32>::Exited(ExitStatus::new(7)).try_into_value(),
            Err(ExitStatus::new(7))
        );
        assert_eq!(
            ExecutionOutcome::Returned(Ok::<_, &str>(42)).transpose(),
            Ok(ExecutionOutcome::Returned(42))
        );
        assert_eq!(
            ExecutionOutcome::Returned(Err::<i32, _>("source failed")).transpose(),
            Err("source failed")
        );
        assert_eq!(
            ExecutionOutcome::<Result<i32, &str>>::Exited(ExitStatus::new(0)).transpose(),
            Ok(ExecutionOutcome::Exited(ExitStatus::new(0)))
        );
    }
}
