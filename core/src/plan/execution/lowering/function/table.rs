mod builder;
mod family;
mod host;
mod profile;

use crate::plan::execution::function::ExecutableFunction;
use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};

pub(in crate::plan::execution::lowering) use builder::FunctionTableBuilder;
pub(super) use builder::{push_core_list_function_function, push_external_list_function_function};
pub(in crate::plan::execution::lowering) use family::{
    CoreListFunctionFunctionSignature, ExternalListFunctionFunctionSignature, FunctionTableFamily,
    ListFunctionFunctionSignature, function_function_id, function_function_table_family,
    function_id, list_function_function_signature, list_function_id, list_function_table_family,
    stored_function_table_family,
};
pub(in crate::plan::execution::lowering) use host::lowered_host_function;
pub(in crate::plan::execution::lowering) use profile::ProfiledFunctionEntries;

pub(in crate::plan::execution::lowering) struct LoweredSpecialization<Value> {
    specialization: SpecializationKey,
    value: Representability<Value>,
}

pub(super) type LoweredFunction<Return> = LoweredSpecialization<ExecutableFunction<Return>>;

pub(super) fn lowered_function<Return>(
    specialization: &SpecializationKey,
    graph: Representability<super::super::graph::LoweredFunctionGraph<Return>>,
) -> LoweredFunction<Return> {
    LoweredSpecialization {
        specialization: specialization.clone(),
        value: graph.map(|graph| ExecutableFunction::new(graph.parameter_count, graph.body)),
    }
}
