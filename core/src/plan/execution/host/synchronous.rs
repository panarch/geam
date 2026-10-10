use super::{
    HostFunctionId, HostFunctionTables, HostedExecutionProfile, HostedFunction,
    HostedFunctionTarget,
};
use crate::host::{HostProfile, SynchronousValueFunction};
use crate::plan::execution::function::{
    CustomFunctionId, ExecutionCustomFunctionBody, ExecutionFunctionBody, ExecutionFunctionEntry,
    ExecutionFunctionRef, ExecutionStringFunctionBody, ExecutionTupleFunctionBody, FunctionTables,
    StringFunctionId, TupleFunctionId,
};
use std::sync::Arc;

pub(crate) struct SynchronousBinding<Profile: HostProfile, Body: ExecutionFunctionBody> {
    pub(crate) target: HostFunctionId<Body>,
    pub(crate) function: HostedFunction<SynchronousValueFunction<Profile>>,
}

pub(crate) type SynchronousStringBinding<Profile> =
    SynchronousBinding<Profile, ExecutionStringFunctionBody<HostedExecutionProfile>>;

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
                Some(SynchronousBinding {
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

pub(crate) type SynchronousCustomBinding<Profile> =
    SynchronousBinding<Profile, ExecutionCustomFunctionBody<HostedExecutionProfile>>;
/// Implementation-only capabilities for already sealed canonical targets.
/// This sidecar never contributes to graph identity or provider registration.
pub(crate) struct SynchronousCustomFunctions<Profile: HostProfile> {
    bindings: Box<[Option<SynchronousCustomBinding<Profile>>]>,
    enabled: Box<[bool]>,
}

impl<Profile: HostProfile> SynchronousCustomFunctions<Profile> {
    pub(in crate::plan::execution) fn new(
        functions: &FunctionTables<HostedExecutionProfile>,
        hosts: &HostFunctionTables<Profile>,
    ) -> Self {
        let bindings: Box<[_]> = functions
            .value_returns
            .custom_functions
            .iter()
            .map(|entry| {
                let ExecutionFunctionRef::Host(HostedFunctionTarget::Value(target)) =
                    entry.as_ref()
                else {
                    return None;
                };
                let function = hosts.value(target);
                let implementation = function.implementation().synchronous()?.clone();
                Some(SynchronousBinding {
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
        target: CustomFunctionId,
    ) -> Option<&SynchronousCustomBinding<Profile>> {
        self.bindings[target.index].as_ref()
    }
}

pub(crate) type SynchronousTupleBinding<Profile> =
    SynchronousBinding<Profile, ExecutionTupleFunctionBody<HostedExecutionProfile>>;
/// Implementation-only capabilities for already sealed canonical targets.
/// This sidecar never contributes to graph identity or provider registration.
pub(crate) struct SynchronousTupleFunctions<Profile: HostProfile> {
    bindings: Box<[Option<SynchronousTupleBinding<Profile>>]>,
    enabled: Box<[bool]>,
}

impl<Profile: HostProfile> SynchronousTupleFunctions<Profile> {
    pub(in crate::plan::execution) fn new(
        functions: &FunctionTables<HostedExecutionProfile>,
        hosts: &HostFunctionTables<Profile>,
    ) -> Self {
        let bindings: Box<[_]> = functions
            .value_returns
            .tuple_functions
            .iter()
            .map(|entry| {
                let ExecutionFunctionRef::Host(HostedFunctionTarget::Value(target)) =
                    entry.as_ref()
                else {
                    return None;
                };
                let function = hosts.value(target);
                let implementation = function.implementation().synchronous()?.clone();
                Some(SynchronousBinding {
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

    pub(crate) fn get(&self, target: TupleFunctionId) -> Option<&SynchronousTupleBinding<Profile>> {
        self.bindings[target.0].as_ref()
    }
}
