use super::{Processes, Receive, RecordReceive, processes};
use crate::{GleamErlangHostProfile, Pid};
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostCall, HostCallContinuation, HostCallError, HostConstruction, HostConstructions,
    HostExecutionContext, HostExecutionError, HostExternal, HostOwnedCompletion, HostProvider,
    HostType, HostTypeSequence,
};
use geam_core::provider::advanced::NativeValue;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

/// One bounded host call with a checked source-process identity.
///
/// The checked identity applies only to this call and cannot be replaced. It does not
/// prove that the process is still alive: a signal in this call may close its
/// mailbox, so preparing a receive remains fallible.
pub struct CurrentProcess<'call, Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call: HostCall<'call, Profile, Provider, Return>,
    unit: ExecutionUnit,
}

impl<'call, Profile, Provider, Return> CurrentProcess<'call, Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Enters a process-relative operation, or rejects a call without a source
    /// invocation before running the operation.
    pub fn with<Output>(
        call: HostCall<'call, Profile, Provider, Return>,
        operation: impl FnOnce(Self) -> Result<Output, HostCallError>,
    ) -> Result<Output, HostCallError> {
        call.with_execution_unit(|call, unit| operation(Self { call, unit }))
    }

    pub fn current(&self) -> &ExecutionUnit {
        &self.unit
    }

    pub fn call(&mut self) -> &mut HostCall<'call, Profile, Provider, Return> {
        &mut self.call
    }

    pub fn into_call(self) -> HostCall<'call, Profile, Provider, Return> {
        self.call
    }

    /// Borrows the same domain for lookup, naming and delivery operations.
    pub fn processes(&mut self) -> Processes<'_, 'call, Profile, Provider, Return> {
        Processes::new(&mut self.call)
    }

    pub fn link(
        &mut self,
        target: HostExternal<'call, Pid>,
        construction: HostConstruction<'call, Pid>,
    ) -> bool {
        processes::link(&mut self.call, &self.unit, target, construction)
    }

    pub fn send_exit(
        &mut self,
        target: &ExecutionUnit,
        reason: NativeValue,
        construction: HostConstruction<'call, Pid>,
    ) {
        processes::send_exit(&mut self.call, &self.unit, target, reason, construction);
    }

    pub fn trap_exits(&mut self, trapping: bool) {
        Profile::erlang_execution(self.call.execution_state()).trap_exits(self.unit.id(), trapping);
    }

    /// Prepares the queued snapshot now, before entering the async continuation.
    pub fn receive(
        &mut self,
        tag: NativeValue,
        deadline: Option<Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        Receive::tagged(&mut self.call, self.unit.id(), tag, deadline)
    }

    pub fn receive_any(
        &mut self,
        deadline: Option<Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        Receive::any(&mut self.call, self.unit.id(), deadline)
    }

    pub fn receive_record(
        &mut self,
        tag: NativeValue,
        arity: usize,
        deadline: Option<Instant>,
    ) -> Result<Receive<Profile>, HostCallError> {
        Receive::record(&mut self.call, self.unit.id(), tag, arity, deadline)
    }

    /// Selects a tagged tuple and projects its fields before consuming it.
    /// The bounded projection cannot access host effects; `None` skips it.
    pub fn receive_record_with<Output: Send + 'static>(
        &mut self,
        tag: NativeValue,
        project: fn(&NativeValue) -> Option<Output>,
        deadline: Option<Instant>,
    ) -> Result<RecordReceive<Profile, Output>, HostCallError> {
        RecordReceive::new(&mut self.call, self.unit.id(), tag, project, deadline)
    }

    /// Transfers a tagged receive and this call to an owned continuation.
    ///
    /// Mailbox admission and its queued snapshot happen synchronously. This
    /// does not add a host request, rescan messages or change timeout ordering.
    pub fn resume_receive<Constructions: HostTypeSequence>(
        mut self,
        constructions: HostConstructions<'call, Constructions>,
        tag: NativeValue,
        deadline: Option<Instant>,
        start: impl for<'run> FnOnce(
            Receive<Profile>,
            HostExecutionContext<'run, Profile, Provider, Constructions>,
        ) -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            HostOwnedCompletion<Profile, Provider, Return, Constructions>,
                            HostExecutionError,
                        >,
                    > + Send
                    + 'run,
            >,
        > + Send
        + 'static,
    ) -> Result<HostCallContinuation<'call, Return>, HostCallError> {
        let receive = self.receive(tag, deadline)?;
        Ok(self
            .call
            .resume(constructions, move |context| start(receive, context)))
    }
}

/// Performs one bounded process-relative operation through its original host.
///
/// Admission checks the source identity once. Cancellation, absent identity,
/// and errors from the operation all stay in the original execution. Typed
/// views and the checked call scope cannot escape the operation.
pub async fn with_current_process<Profile, Provider, Constructions, Output, Operation>(
    context: &HostExecutionContext<'_, Profile, Provider, Constructions>,
    operation: Operation,
) -> Result<Output, HostExecutionError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
    Output: Send + 'static,
    Operation: for<'call> FnOnce(
            CurrentProcess<'call, Profile, Provider, ()>,
            HostConstructions<'call, Constructions>,
        ) -> Result<Output, HostCallError>
        + Send
        + 'static,
{
    context
        .with_constructions(move |call, constructions| {
            CurrentProcess::with(call, |process| operation(process, constructions))
        })
        .await?
        .map_err(Into::into)
}
