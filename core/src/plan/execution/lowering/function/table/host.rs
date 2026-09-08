use super::{FunctionTableBuilder, LoweredSpecialization, ProfiledFunctionEntries};
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
    pub(in crate::plan::execution::lowering) fn finish_hosted(
        self,
        functions: ProfiledFunctionEntries<HostedExecutionProfile>,
    ) -> SpecializationOutcome<Box<FunctionTables<HostedExecutionProfile>>> {
        FunctionTableBuilder::finish_profile(self.profile_hosted(), functions)
    }
}
