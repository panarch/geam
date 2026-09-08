//! Typed calls and scoped observation over the generic work runtime.

mod execution;
mod input;
mod return_;
mod value;

pub use execution::{
    ExecutionScope, ObservationError, SharedExecutionError, WorkModule, WorkModuleBindings,
    WorkModuleBuilder,
};
pub use value::{Completed, Future, FutureType, ReadValue, SharedList, SourceType};
pub(crate) use value::{ScopedOutput, SharedValue};

use std::marker::PhantomData;

/// A fresh, single-use lifetime for one attached execution.
///
/// Reusing a guard to attach a second owner is a moved-value error:
///
/// ```compile_fail
/// use geam_core::embedding::{ExecutionGuard, WorkModule};
/// use geam_core::host::HostWorkProfile;
/// fn twice<P>(first: &mut WorkModule<P>, second: &mut WorkModule<P>,
///     guard: ExecutionGuard<'_>, state: &mut P::RunState,
///     echo: &mut (dyn geam_core::EchoSink + Send))
/// where P: HostWorkProfile,
///       P::RunState: Send, P::ExternalStores: Send,
/// {
///     drop(first.attach(guard, state, echo));
///     drop(second.attach(guard, state, echo));
/// }
/// ```
pub struct ExecutionGuard<'scope> {
    brand: ScopeBrand<'scope>,
}

#[derive(Clone, Copy)]
pub(crate) struct ScopeBrand<'scope>(PhantomData<fn(&'scope ()) -> &'scope ()>);

/// Runs the caller's async closure with a fresh execution lifetime.
///
/// This does not create an executor, spawn work, or drive returned work.
pub async fn with_execution_scope<Output>(
    run: impl for<'scope> AsyncFnOnce(ExecutionGuard<'scope>) -> Output,
) -> Output {
    run(ExecutionGuard {
        brand: ScopeBrand(PhantomData),
    })
    .await
}

impl<'scope> ExecutionGuard<'scope> {
    pub(crate) fn into_brand(self) -> ScopeBrand<'scope> {
        self.brand
    }
}
