use super::{AdditionalFunctions, FunctionTableBuilder, LoweredSpecialization};
use crate::host::HostProfile;
use crate::plan::execution::function::{FunctionTables, ValueFunctionEntry};
use crate::plan::execution::host::HostedExecutionProfile;
use crate::plan::execution::lowering::SpecializationOutcome;
use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};

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
    pub(in crate::plan::execution::lowering) fn finish_hosted<Profile: HostProfile>(
        self,
        functions: AdditionalFunctions<HostedExecutionProfile<Profile>>,
    ) -> SpecializationOutcome<Box<FunctionTables<HostedExecutionProfile<Profile>>>> {
        self.finish_profile(functions)
    }
}
