//! Typed calls and scoped observation over the generic work runtime.

mod input;
pub(in crate::embedding) mod return_;
mod value;

pub use crate::runtime::{ObservationError, SharedExecutionError};
pub use value::{Completed, Future, FutureType, ReadValue, SharedList, SourceType};
pub(crate) use value::{ScopedOutput, SharedValue};

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub(crate) struct ScopeBrand<'scope>(PhantomData<fn(&'scope ()) -> &'scope ()>);

impl ScopeBrand<'_> {
    pub(in crate::embedding) fn new() -> Self {
        Self(PhantomData)
    }
}
