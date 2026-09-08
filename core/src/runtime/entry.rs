use super::shared::Shared;
use super::work::driver::Driver;
use super::{EchoSink, HostCallOrigin, ObservationError, RetainedValues, SharedExecutionError};
use crate::host::HostWorkProfile;
use crate::plan::execution::{EntryCompletion, HostedEntry};

pub(crate) async fn run_hosted_entry<Profile: HostWorkProfile>(
    entry: &mut HostedEntry<Profile>,
    state: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<(), ObservationError> {
    let function = match entry.completion {
        EntryCompletion::Immediate => {
            return entry
                .execution
                .run_main(state, echo)
                .map(|_| ())
                .map_err(|error| {
                    ObservationError::Execution(SharedExecutionError(Shared::new(error)))
                });
        }
        EntryCompletion::Work(function) => function,
    };
    let (plan, stores) = entry.execution.parts_mut();
    let store = crate::host::work_store::<Profile>(stores).clone_handle();
    let mut driver = Driver::new(plan, state, stores, echo);
    let work = driver.call(|plan, runtime| {
        super::function::run_external(
            plan,
            runtime,
            function,
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        )
        .map(|value| store.work(value.lease()))
        .map_err(|error| ObservationError::Execution(SharedExecutionError(Shared::new(error))))
    })?;
    let completed = driver
        .observe(&work)
        .await
        .map_err(|_| ObservationError::Cancelled)?;
    completed.read(|result| match result {
        Ok(_) => Ok(()),
        Err(error) => Err(ObservationError::Execution(SharedExecutionError(
            error.clone(),
        ))),
    })
}
