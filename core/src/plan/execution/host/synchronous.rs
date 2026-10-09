use super::{
    HostFunctionId, HostFunctionTables, HostedExecutionProfile, HostedFunction,
    HostedFunctionTarget,
};
use crate::host::{HostProfile, SynchronousValueFunction};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionStringFunctionBody, FunctionTables,
    StringFunctionId,
};
use std::sync::Arc;

pub(crate) struct SynchronousStringBinding<Profile: HostProfile> {
    pub(crate) target: HostFunctionId<ExecutionStringFunctionBody<HostedExecutionProfile>>,
    pub(crate) function: HostedFunction<SynchronousValueFunction<Profile>>,
}

/// Implementation-only capabilities for already sealed canonical targets.
/// This sidecar never contributes to graph identity or provider registration.
pub(crate) struct SynchronousStringFunctions<Profile: HostProfile> {
    bindings: Box<[Option<SynchronousStringBinding<Profile>>]>,
    enabled: Box<[bool]>,
}

impl<Profile: HostProfile> SynchronousStringFunctions<Profile> {
    pub(in crate::plan::execution) fn new(
        functions: &FunctionTables<HostedExecutionProfile>,
        hosts: &HostFunctionTables<Profile>,
    ) -> Self {
        let bindings: Box<[_]> = functions
            .value_returns
            .string_functions
            .iter()
            .map(|entry| {
                let ExecutionFunctionRef::Host(HostedFunctionTarget::Value(target)) =
                    entry.as_ref()
                else {
                    return None;
                };
                let function = hosts.value(target);
                let implementation = function.implementation().synchronous()?.clone();
                Some(SynchronousStringBinding {
                    target: *target,
                    function: HostedFunction::new(
                        Arc::clone(function.metadata_handle()),
                        implementation,
                    ),
                })
            })
            .collect();
        let enabled = bindings.iter().map(Option::is_some).collect();
        Self { bindings, enabled }
    }

    pub(crate) fn enabled(&self) -> &[bool] {
        &self.enabled
    }

    pub(crate) fn get(
        &self,
        target: StringFunctionId,
    ) -> Option<&SynchronousStringBinding<Profile>> {
        self.bindings[target.0].as_ref()
    }
}
