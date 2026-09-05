use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::HostCallOrigin;
use crate::{AsyncExecutionError, HostFailure};

pub(in crate::runtime) type TransferExecutionResult<Value> = Result<Value, AsyncExecutionError>;

impl AsyncExecutionError {
    pub(in crate::runtime) fn host_failure(
        plan: &impl RuntimeExecutionPlan,
        origin: HostCallOrigin,
        function: &crate::plan::execution::host::HostedFunctionMetadata,
        failure: HostFailure,
    ) -> Self {
        match origin.into_source_site(function.site()) {
            Ok(site) => Self::from_host_call(
                function,
                site.clone(),
                plan.source_context_for(site.module()),
                failure,
            ),
            Err(caller) => Self::from_host_origin(function, caller, failure),
        }
    }
}
