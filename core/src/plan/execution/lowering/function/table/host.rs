use super::{FunctionTableBuilder, LoweredSpecialization, ProfiledFunctionEntries};
use crate::plan::execution::function::{
    DirectHostedExecutionProfile, FunctionTables, ValueFunctionEntry,
};
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
    pub(in crate::plan::execution::lowering) fn finish_hosted<Profile>(
        self,
        functions: ProfiledFunctionEntries<Profile>,
    ) -> SpecializationOutcome<Box<FunctionTables<Profile>>>
    where
        Profile: DirectHostedExecutionProfile,
    {
        FunctionTableBuilder::finish_profile(self.profile_hosted::<Profile>(), functions)
    }
}
