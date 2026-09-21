use super::{HostFunctionId, HostNeverFunctionId, HostedFunction};
use crate::plan::execution::function::ExecutionFunctionBody;

pub(crate) struct HostBindingTables<Value, Never> {
    value_functions: Box<[HostedFunction<Value>]>,
    never_functions: Box<[HostedFunction<Never>]>,
}

pub(crate) type HostFunctionTables<Profile> = HostBindingTables<
    crate::host::HostValueFunction<Profile>,
    crate::host::HostNeverFunction<Profile>,
>;

impl<Value, Never> HostBindingTables<Value, Never> {
    pub(in crate::plan::execution) fn new(
        value_functions: Box<[HostedFunction<Value>]>,
        never_functions: Box<[HostedFunction<Never>]>,
    ) -> Self {
        Self {
            value_functions,
            never_functions,
        }
    }

    pub(crate) fn value<Body: ExecutionFunctionBody>(
        &self,
        id: &HostFunctionId<Body>,
    ) -> &HostedFunction<Value> {
        &self.value_functions[id.index()]
    }

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &HostedFunction<Never> {
        &self.never_functions[id.index()]
    }

    pub(in crate::plan::execution) fn into_metadata(
        self,
    ) -> (
        super::super::storage::Table<std::sync::Arc<super::HostedFunctionMetadata>>,
        super::super::storage::Table<std::sync::Arc<super::HostedFunctionMetadata>>,
    ) {
        (
            self.value_functions
                .into_vec()
                .into_iter()
                .map(super::HostedFunction::into_metadata)
                .collect(),
            self.never_functions
                .into_vec()
                .into_iter()
                .map(super::HostedFunction::into_metadata)
                .collect(),
        )
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn value_functions(&self) -> &[HostedFunction<Value>] {
        &self.value_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn never_functions(&self) -> &[HostedFunction<Never>] {
        &self.never_functions
    }
}
