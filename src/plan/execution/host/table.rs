use super::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId, HostedBitArrayFunction,
    HostedBoolFunction, HostedFloatFunction, HostedIntFunction, HostedNilFunction,
    HostedStringFunction, HostedUtfCodepointFunction,
};
use crate::host::HostProfile;

pub(crate) struct HostFunctionTables<Profile: HostProfile> {
    int_functions: Box<[HostedIntFunction<Profile>]>,
    float_functions: Box<[HostedFloatFunction<Profile>]>,
    string_functions: Box<[HostedStringFunction<Profile>]>,
    bit_array_functions: Box<[HostedBitArrayFunction<Profile>]>,
    utf_codepoint_functions: Box<[HostedUtfCodepointFunction<Profile>]>,
    bool_functions: Box<[HostedBoolFunction<Profile>]>,
    nil_functions: Box<[HostedNilFunction<Profile>]>,
}

impl<Profile: HostProfile> HostFunctionTables<Profile> {
    pub(in crate::plan::execution) fn new(
        int_functions: Box<[HostedIntFunction<Profile>]>,
        float_functions: Box<[HostedFloatFunction<Profile>]>,
        string_functions: Box<[HostedStringFunction<Profile>]>,
        bit_array_functions: Box<[HostedBitArrayFunction<Profile>]>,
        utf_codepoint_functions: Box<[HostedUtfCodepointFunction<Profile>]>,
        bool_functions: Box<[HostedBoolFunction<Profile>]>,
        nil_functions: Box<[HostedNilFunction<Profile>]>,
    ) -> Self {
        Self {
            int_functions,
            float_functions,
            string_functions,
            bit_array_functions,
            utf_codepoint_functions,
            bool_functions,
            nil_functions,
        }
    }

    pub(crate) fn int(&self, id: HostIntFunctionId) -> &HostedIntFunction<Profile> {
        &self.int_functions[id.index()]
    }

    pub(crate) fn float(&self, id: HostFloatFunctionId) -> &HostedFloatFunction<Profile> {
        &self.float_functions[id.index()]
    }

    pub(crate) fn string(&self, id: HostStringFunctionId) -> &HostedStringFunction<Profile> {
        &self.string_functions[id.index()]
    }

    pub(crate) fn bit_array(&self, id: HostBitArrayFunctionId) -> &HostedBitArrayFunction<Profile> {
        &self.bit_array_functions[id.index()]
    }

    pub(crate) fn utf_codepoint(
        &self,
        id: HostUtfCodepointFunctionId,
    ) -> &HostedUtfCodepointFunction<Profile> {
        &self.utf_codepoint_functions[id.index()]
    }

    pub(crate) fn bool(&self, id: HostBoolFunctionId) -> &HostedBoolFunction<Profile> {
        &self.bool_functions[id.index()]
    }

    pub(crate) fn nil(&self, id: HostNilFunctionId) -> &HostedNilFunction<Profile> {
        &self.nil_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn int_functions(&self) -> &[HostedIntFunction<Profile>] {
        &self.int_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bool_functions(&self) -> &[HostedBoolFunction<Profile>] {
        &self.bool_functions
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
        let execution = HostedExecution::from_module_plan(plan);

        assert_eq!(execution.host_functions.int_functions().len(), 2);
        let add = &execution.host_functions.int_functions()[0];
        assert_eq!(add.package(), "host_support");
        assert_eq!(add.module(), "host/math");
        assert_eq!(add.name(), "add");
        let subtract = &execution.host_functions.int_functions()[1];
        assert_eq!(subtract.package(), "host_support");
        assert_eq!(subtract.module(), "host/math");
        assert_eq!(subtract.name(), "subtract");
        assert_eq!(execution.host_functions.bool_functions().len(), 1);
        let positive = &execution.host_functions.bool_functions()[0];
        assert_eq!(positive.package(), "host_support");
        assert_eq!(positive.module(), "host/math");
        assert_eq!(positive.name(), "positive");
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Tuple(vec![Value::Int(2.into()), Value::Bool(true)])),
        );
    }
}
