use super::AsyncExecutionError;
use crate::runtime::shared::Shared;
use std::fmt;

/// An original execution failure shared by observations of one operation.
#[derive(Clone)]
pub struct SharedExecutionError(pub(crate) Shared<AsyncExecutionError>);

impl SharedExecutionError {
    /// Borrows the actual failure without copying retained source values.
    pub fn read<Output>(&self, read: impl FnOnce(&AsyncExecutionError) -> Output) -> Output {
        self.0.read(read)
    }
}

impl fmt::Debug for SharedExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.read(|error| fmt::Debug::fmt(error, formatter))
    }
}

impl fmt::Display for SharedExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.read(|error| fmt::Display::fmt(error, formatter))
    }
}

impl std::error::Error for SharedExecutionError {}

#[cfg(test)]
mod tests {
    use super::{AsyncExecutionError, Shared, SharedExecutionError};
    use crate::{InvariantError, ValueType};
    use std::error::Error;

    #[test]
    fn shared_diagnostics_borrow_the_original_failure_and_keep_its_rendering() {
        let error = SharedExecutionError(Shared::new(AsyncExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        )));
        let alias = error.clone();
        error.read(|first| alias.read(|second| assert!(std::ptr::eq(first, second))));
        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
        assert_eq!(
            format!("{error:?}"),
            "Invariant(ListIndexOutOfBounds { item_type: Int, index: 1, length: 0 })"
        );
        assert!(error.source().is_none());
    }
}
