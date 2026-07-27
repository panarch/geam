use super::{FunctionTableBuilder, LoweredSpecialization};
use crate::plan::execution::function::{
    BoolFunctionBody, FunctionTables, IntFunctionBody, ValueFunctionEntry,
};
use crate::plan::execution::lowering::SpecializationOutcome;
use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};

type HostedFunctionTables<IntHost, BoolHost> = FunctionTables<
    ValueFunctionEntry<IntFunctionBody, IntHost>,
    ValueFunctionEntry<BoolFunctionBody, BoolHost>,
>;

pub(in crate::plan::execution::lowering) fn lowered_host_function<Body, Host>(
    specialization: &SpecializationKey,
    target: Host,
) -> LoweredSpecialization<ValueFunctionEntry<Body, Host>> {
    LoweredSpecialization {
        specialization: specialization.clone(),
        value: Representability::Inhabited(ValueFunctionEntry::host(target)),
    }
}

impl FunctionTableBuilder {
    pub(in crate::plan::execution::lowering) fn finish_hosted<IntHost, BoolHost>(
        mut self,
        host_int_functions: Vec<(
            usize,
            LoweredSpecialization<ValueFunctionEntry<IntFunctionBody, IntHost>>,
        )>,
        host_bool_functions: Vec<(
            usize,
            LoweredSpecialization<ValueFunctionEntry<BoolFunctionBody, BoolHost>>,
        )>,
    ) -> SpecializationOutcome<Box<HostedFunctionTables<IntHost, BoolHost>>> {
        let int_functions = std::mem::take(&mut self.int_functions)
            .into_iter()
            .map(|(index, function)| {
                (
                    index,
                    LoweredSpecialization {
                        specialization: function.specialization,
                        value: function.value.map(ValueFunctionEntry::graph),
                    },
                )
            })
            .chain(host_int_functions)
            .collect();
        let bool_functions = std::mem::take(&mut self.bool_functions)
            .into_iter()
            .map(|(index, function)| {
                (
                    index,
                    LoweredSpecialization {
                        specialization: function.specialization,
                        value: function.value.map(ValueFunctionEntry::graph),
                    },
                )
            })
            .chain(host_bool_functions)
            .collect();
        self.finish_with_value_functions(int_functions, bool_functions)
    }
}
