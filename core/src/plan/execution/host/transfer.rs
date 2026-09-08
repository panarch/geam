use super::{HostFunctionId, HostNeverFunctionId, HostedFunction, HostedFunctionMetadata};
use crate::host::{
    AsyncHostCallError, AsyncHostCallErrorKind, HostProfile, TransferHostNeverFunction,
    TransferHostValueFunction,
};
use std::sync::Arc;

pub(crate) struct TransferHostFunctionTables<Profile: HostProfile> {
    values: Box<[TransferHostedValueFunction<Profile>]>,
    nevers: Box<[TransferHostedNeverFunction<Profile>]>,
}

pub(crate) struct TransferHostFunctionTableBuilder<Profile: HostProfile> {
    values: Vec<TransferHostedValueFunction<Profile>>,
    nevers: Vec<TransferHostedNeverFunction<Profile>>,
}

pub(crate) type TransferHostedValueFunction<Profile> =
    HostedFunction<TransferHostValueFunction<Profile>>;
pub(crate) type TransferHostedNeverFunction<Profile> =
    HostedFunction<TransferHostNeverFunction<Profile>>;

impl<Profile: HostProfile> TransferHostFunctionTableBuilder<Profile> {
    pub(in crate::plan::execution) fn new() -> Self {
        Self {
            values: Vec::new(),
            nevers: Vec::new(),
        }
    }

    pub(in crate::plan::execution) fn push_scoped_value(
        &mut self,
        metadata: Arc<HostedFunctionMetadata>,
        function: &TransferHostValueFunction<Profile>,
    ) -> usize {
        let index = self.values.len();
        self.values
            .push(HostedFunction::new(metadata, function.clone()));
        index
    }

    pub(in crate::plan::execution) fn push_scoped_never(
        &mut self,
        metadata: Arc<HostedFunctionMetadata>,
        function: &TransferHostNeverFunction<Profile>,
    ) -> usize {
        let index = self.nevers.len();
        self.nevers
            .push(HostedFunction::new(metadata, function.clone()));
        index
    }

    pub(in crate::plan::execution) fn finish(self) -> TransferHostFunctionTables<Profile> {
        TransferHostFunctionTables {
            values: self.values.into_boxed_slice(),
            nevers: self.nevers.into_boxed_slice(),
        }
    }
}

impl<Profile: HostProfile> TransferHostFunctionTables<Profile> {
    pub(crate) fn value<Body: crate::plan::execution::function::ExecutionFunctionBody>(
        &self,
        id: &HostFunctionId<Body>,
    ) -> &TransferHostedValueFunction<Profile> {
        &self.values[id.index()]
    }

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &TransferHostedNeverFunction<Profile> {
        &self.nevers[id.index()]
    }
}

pub(crate) fn call_transfer_value<Profile: HostProfile>(
    function: &TransferHostedValueFunction<Profile>,
    runtime: &mut dyn crate::host::TransferHostCallRuntime<Profile>,
) -> Result<crate::host::HostValueToken, AsyncHostCallError> {
    function.implementation().call(runtime)
}

pub(crate) fn call_transfer_never<Profile: HostProfile>(
    function: &TransferHostedNeverFunction<Profile>,
    runtime: &mut dyn crate::host::TransferHostCallRuntime<Profile>,
) -> Result<std::convert::Infallible, AsyncHostCallError> {
    function.implementation().call(runtime)
}

pub(crate) fn async_host_failure(
    error: AsyncHostCallError,
) -> Result<crate::HostFailure, crate::runtime::TransferExecutionError> {
    match error.into_kind() {
        AsyncHostCallErrorKind::Failure(failure) => Ok(failure),
        AsyncHostCallErrorKind::Nested(error) => Err(error),
    }
}
