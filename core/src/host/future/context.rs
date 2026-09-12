use crate::host::execution::NativeScope;
use crate::host::{
    HostExecutionContext, HostExecutionError, HostProfile, HostProvider, HostTypeSequence,
};
use crate::runtime::work::Dependencies;
use crate::runtime::work::execution::{Completion, WorkContext};

/// Access to short operations while one owned native Future is running.
///
/// The context borrows the native work's own lifetime, not its creating Gleam
/// invocation or the host state. It cannot survive the native Future:
///
/// ```compile_fail
/// use geam_core::host::{HostFutureContext, HostProfile, HostProvider, HostTypeSequence};
/// fn escape<'work, P, H, C>(context: HostFutureContext<'work, P, H, C>)
///     -> HostFutureContext<'static, P, H, C>
/// where P: HostProfile, H: HostProvider<P>, C: HostTypeSequence {
///     context
/// }
/// ```
pub struct HostFutureContext<'work, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    execution: HostExecutionContext<'work, Profile, Provider, Constructions>,
    pub(in crate::host::future) dependencies: Dependencies<Completion>,
}

impl<'work, Profile, Provider, Constructions>
    HostFutureContext<'work, Profile, Provider, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    pub(crate) fn new(
        scope: &'work mut NativeScope,
        work: WorkContext<Profile>,
        dependencies: Dependencies<Completion>,
        codec: crate::host::HostCodecScope,
        origin: crate::runtime::HostCallOrigin,
    ) -> Self {
        Self {
            execution: HostExecutionContext::new(scope, work.execution().clone(), codec, origin),
            dependencies,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        HostExecutionContext<'work, Profile, Provider, Constructions>,
        Dependencies<Completion>,
    ) {
        (self.execution, self.dependencies)
    }

    /// Access to short host-state operations and retained ordinary callbacks.
    pub fn execution(&self) -> &HostExecutionContext<'work, Profile, Provider, Constructions> {
        &self.execution
    }

    /// Runs a non-async operation against this provider's original mutable state.
    pub fn with_state<'request, Output: Send + 'static, Operation>(
        &'request self,
        operation: Operation,
    ) -> impl std::future::Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
    {
        self.execution.with_state(operation)
    }
}
