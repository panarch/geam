use super::{HostFunctionId, HostNeverFunctionId, HostedNeverFunction, HostedValueFunction};
use crate::host::HostProfile;
use crate::plan::execution::function::ExecutionFunctionBody;

pub(crate) struct HostFunctionTables<Profile: HostProfile> {
    value_functions: Box<[HostedValueFunction<Profile>]>,
    never_functions: Box<[HostedNeverFunction<Profile>]>,
}

impl<Profile: HostProfile> HostFunctionTables<Profile> {
    pub(in crate::plan::execution) fn new(
        value_functions: Box<[HostedValueFunction<Profile>]>,
        never_functions: Box<[HostedNeverFunction<Profile>]>,
    ) -> Self {
        Self {
            value_functions,
            never_functions,
        }
    }

    pub(crate) fn value<Body: ExecutionFunctionBody>(
        &self,
        id: &HostFunctionId<Body>,
    ) -> &HostedValueFunction<Profile> {
        &self.value_functions[id.index()]
    }

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &HostedNeverFunction<Profile> {
        &self.never_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn value_functions(&self) -> &[HostedValueFunction<Profile>] {
        &self.value_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn never_functions(&self) -> &[HostedNeverFunction<Profile>] {
        &self.never_functions
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    #[test]
    fn preserves_first_use_function_metadata_and_removes_unused_implementations() {
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("subtract", <BigInt as std::ops::Sub>::sub)
            .expect("host function should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("unused", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("positive", |value: BigInt| value > BigInt::from(0))
            .expect("host function should be valid")
            .with_function("unused_predicate", || false)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
        let source = r#"
import host/math

pub fn main() {
  let added = math.add(1, 2)
  #(math.subtract(added, 1), math.positive(added))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

        assert_eq!(execution.host_functions.value_functions().len(), 3);
        assert!(execution.host_functions.never_functions().is_empty());
        let add = &execution.host_functions.value_functions()[0];
        assert_eq!(add.package(), "host_support");
        assert_eq!(add.module(), "host/math");
        assert_eq!(add.name(), "add");
        let subtract = &execution.host_functions.value_functions()[1];
        assert_eq!(subtract.package(), "host_support");
        assert_eq!(subtract.module(), "host/math");
        assert_eq!(subtract.name(), "subtract");
        let positive = &execution.host_functions.value_functions()[2];
        assert_eq!(positive.package(), "host_support");
        assert_eq!(positive.module(), "host/math");
        assert_eq!(positive.name(), "positive");
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Tuple(vec![Value::Int(2.into()), Value::Bool(true)])),
        );
    }
}
