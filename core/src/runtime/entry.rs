use super::{EchoSink, HostCallOrigin, RetainedValues};
use crate::execution::{ExecutionHost, RunError};
use crate::host::HostWorkProfile;
use crate::plan::execution::{EntryCompletion, HostedEntry};
use crate::runtime::execution::Domain;
use std::sync::Arc;

pub(crate) async fn run_hosted_entry<Profile: HostWorkProfile>(
    entry: &mut HostedEntry<Profile>,
    host: &dyn ExecutionHost,
    state: &mut Profile::RunState,
    echo: &mut (dyn EchoSink + Send),
) -> Result<(), RunError> {
    let (plan, stores) = entry.execution.parts_mut();
    let store = crate::host::work_store::<Profile>(stores).clone_handle();
    let domain = Domain::new(
        Arc::clone(plan),
        host,
        state,
        stores,
        echo,
        Domain::<Profile>::DEFAULT_BUDGET,
    );
    let context = domain.context();
    domain
        .drive(async {
            match entry.completion {
                EntryCompletion::Immediate => {
                    context.run_main().await?;
                }
                EntryCompletion::Work(function) => {
                    let value = context
                        .call(function, HostCallOrigin::Entry, RetainedValues::empty())
                        .await
                        .map_err(|_| RunError::Cancelled)??;
                    let work = store.work(value.lease());
                    let completed = work.observe().await.map_err(|_| RunError::Cancelled)?;
                    completed.read(|result| match result {
                        Ok(_) => Ok(()),
                        Err(error) => Err(RunError::Execution(error.read(Clone::clone))),
                    })?;
                }
            }
            Ok(())
        })
        .await?
}
