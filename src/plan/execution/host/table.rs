use super::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNeverFunctionId, HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId,
    HostedBitArrayFunction, HostedBoolFunction, HostedFloatFunction, HostedFunctionMetadata,
    HostedIntFunction, HostedNeverFunction, HostedNilFunction, HostedStringFunction,
    HostedUtfCodepointFunction,
};
use crate::host::HostProfile;
use std::convert::Infallible;

pub(crate) trait HostValueFunctionTarget<Profile: HostProfile> {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata;
}

pub(crate) struct HostFunctionTables<Profile: HostProfile> {
    values: HostValueFunctionTables<Profile>,
    never_functions: Box<[HostedNeverFunction<Profile>]>,
}

pub(in crate::plan::execution) struct HostValueFunctionTables<Profile: HostProfile> {
    int_functions: Box<[HostedIntFunction<Profile>]>,
    float_functions: Box<[HostedFloatFunction<Profile>]>,
    string_functions: Box<[HostedStringFunction<Profile>]>,
    bit_array_functions: Box<[HostedBitArrayFunction<Profile>]>,
    utf_codepoint_functions: Box<[HostedUtfCodepointFunction<Profile>]>,
    bool_functions: Box<[HostedBoolFunction<Profile>]>,
    nil_functions: Box<[HostedNilFunction<Profile>]>,
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostIntFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.int(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostFloatFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.float(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostStringFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.string(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostBitArrayFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.bit_array(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostUtfCodepointFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.utf_codepoint(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostBoolFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.bool(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for HostNilFunctionId {
    fn metadata<'a>(&self, tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        tables.nil(*self).metadata()
    }
}

impl<Profile: HostProfile> HostValueFunctionTarget<Profile> for Infallible {
    fn metadata<'a>(&self, _tables: &'a HostFunctionTables<Profile>) -> &'a HostedFunctionMetadata {
        match *self {}
    }
}

impl<Profile: HostProfile> HostFunctionTables<Profile> {
    pub(in crate::plan::execution) fn new(
        values: HostValueFunctionTables<Profile>,
        never_functions: Box<[HostedNeverFunction<Profile>]>,
    ) -> Self {
        Self {
            values,
            never_functions,
        }
    }

    pub(crate) fn int(&self, id: HostIntFunctionId) -> &HostedIntFunction<Profile> {
        self.values.int(id)
    }

    pub(crate) fn float(&self, id: HostFloatFunctionId) -> &HostedFloatFunction<Profile> {
        self.values.float(id)
    }

    pub(crate) fn string(&self, id: HostStringFunctionId) -> &HostedStringFunction<Profile> {
        self.values.string(id)
    }

    pub(crate) fn bit_array(&self, id: HostBitArrayFunctionId) -> &HostedBitArrayFunction<Profile> {
        self.values.bit_array(id)
    }

    pub(crate) fn utf_codepoint(
        &self,
        id: HostUtfCodepointFunctionId,
    ) -> &HostedUtfCodepointFunction<Profile> {
        self.values.utf_codepoint(id)
    }

    pub(crate) fn bool(&self, id: HostBoolFunctionId) -> &HostedBoolFunction<Profile> {
        self.values.bool(id)
    }

    pub(crate) fn nil(&self, id: HostNilFunctionId) -> &HostedNilFunction<Profile> {
        self.values.nil(id)
    }

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &HostedNeverFunction<Profile> {
        &self.never_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn int_functions(&self) -> &[HostedIntFunction<Profile>] {
        &self.values.int_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bool_functions(&self) -> &[HostedBoolFunction<Profile>] {
        &self.values.bool_functions
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn never_functions(&self) -> &[HostedNeverFunction<Profile>] {
        &self.never_functions
    }
}

impl<Profile: HostProfile> HostValueFunctionTables<Profile> {
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
}

#[cfg(test)]
mod tests {
    use crate::{
        ExecutionError, HostFailure, HostModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, Value, compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

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
        assert_eq!(execution.host_functions.bool_functions().len(), 1);
        assert!(execution.host_functions.never_functions().is_empty());
        let add = &execution.host_functions.int_functions()[0];
        assert_eq!(add.metadata().package(), "host_support");
        assert_eq!(add.metadata().module(), "host/math");
        assert_eq!(add.metadata().name(), "add");
        let subtract = &execution.host_functions.int_functions()[1];
        assert_eq!(subtract.metadata().package(), "host_support");
        assert_eq!(subtract.metadata().module(), "host/math");
        assert_eq!(subtract.metadata().name(), "subtract");
        let positive = &execution.host_functions.bool_functions()[0];
        assert_eq!(positive.metadata().package(), "host_support");
        assert_eq!(positive.metadata().module(), "host/math");
        assert_eq!(positive.metadata().name(), "positive");
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Tuple(vec![Value::Int(2.into()), Value::Bool(true)])),
        );
    }

    #[test]
    fn preserves_first_use_never_metadata_and_removes_unused_implementations() {
        fn stop(value: BigInt) -> Result<Infallible, HostFailure> {
            Err(HostFailure::new(format!("stop {value}")))
        }

        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function(
                "first",
                stop as fn(BigInt) -> Result<Infallible, HostFailure>,
            )
            .expect("host function should be valid")
            .with_fallible_function(
                "second",
                stop as fn(BigInt) -> Result<Infallible, HostFailure>,
            )
            .expect("host function should be valid")
            .with_fallible_function(
                "unused",
                stop as fn(BigInt) -> Result<Infallible, HostFailure>,
            )
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
        let source = r#"
import host/control

pub fn main() {
  let first: fn(Int) -> Int = control.first
  let second: fn(Int) -> Int = control.second

  case first == first {
    True -> second(7)
    False -> first(7)
  }
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

        assert!(execution.host_functions.int_functions().is_empty());
        assert!(execution.host_functions.bool_functions().is_empty());
        assert_eq!(execution.host_functions.never_functions().len(), 2);
        assert_eq!(
            execution.host_functions.never_functions()[0]
                .metadata()
                .name(),
            "first"
        );
        assert_eq!(
            execution.host_functions.never_functions()[1]
                .metadata()
                .name(),
            "second"
        );
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("second should fail");
        assert!(matches!(
            error,
            ExecutionError::Host(error)
                if error.function() == "second" && error.failure().message() == "stop 7"
        ));
    }
}
