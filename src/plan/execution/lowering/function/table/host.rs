use super::{AdditionalFunctions, FunctionTableBuilder, LoweredSpecialization};
use crate::host::HostProfile;
use crate::plan::execution::function::{
    FunctionFunctionId, FunctionTables, ListFunctionFunctionBody, ListFunctionId,
    RuntimeFunctionId, ValueFunctionEntry,
};
use crate::plan::execution::host::{
    HostNeverFunctionId, HostedExecutionProfile, HostedFunctionTarget,
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

impl<Profile: HostProfile> AdditionalFunctions<HostedExecutionProfile<Profile>> {
    pub(in crate::plan::execution::lowering) fn push_never_host_function(
        &mut self,
        index: usize,
        specialization: &SpecializationKey,
        function: RuntimeFunctionId,
        target: HostNeverFunctionId,
    ) {
        match function {
            RuntimeFunctionId::Never(_) => self
                .never
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Int(_) => self
                .int
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Float(_) => self
                .float
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::String(_) => self
                .string
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::BitArray(_) => self
                .bit_array
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::UtfCodepoint(_) => self
                .utf_codepoint
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Custom(_) => self
                .custom
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Bool(_) => self
                .bool
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Nil(_) => self
                .nil
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::Tuple { .. } => self
                .tuple
                .push((index, lowered_never_host_function(specialization, target))),
            RuntimeFunctionId::List(function) => {
                self.push_never_host_list_function(specialization, function, target);
            }
            RuntimeFunctionId::Function { id, .. } => {
                self.push_never_host_function_function(index, specialization, id, target);
            }
        }
    }

    fn push_never_host_list_function(
        &mut self,
        specialization: &SpecializationKey,
        function: ListFunctionId,
        target: HostNeverFunctionId,
    ) {
        match function {
            ListFunctionId::Parameter(id) => self
                .parameter_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::ParameterList(id) => self
                .parameter_list_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Int(id) => self
                .int_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::String(id) => self
                .string_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::BitArray(id) => self
                .bit_array_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::UtfCodepoint(id) => self
                .utf_codepoint_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Custom(id) => self
                .custom_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Float(id) => self
                .float_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Bool(id) => self
                .bool_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Nil(id) => self
                .nil_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Tuple(id) => self
                .tuple_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::List(id) => self
                .list_list
                .push((id, lowered_never_host_function(specialization, target))),
            ListFunctionId::Function(id) => self
                .function_list
                .push((id, lowered_never_host_function(specialization, target))),
        }
    }

    fn push_never_host_function_function(
        &mut self,
        index: usize,
        specialization: &SpecializationKey,
        function: FunctionFunctionId,
        target: HostNeverFunctionId,
    ) {
        match function {
            FunctionFunctionId::Generic(_) => self
                .generic_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Never(_) => self
                .never_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Int(_) => self
                .int_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Float(_) => self
                .float_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::String(_) => self
                .string_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::BitArray(_) => self
                .bit_array_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::UtfCodepoint(_) => self
                .utf_codepoint_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Custom(_) => self
                .custom_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Bool(_) => self
                .bool_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Nil(_) => self
                .nil_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::Tuple(_) => self
                .tuple_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
            FunctionFunctionId::List(function) => {
                self.push_never_host_list_function_function(
                    index,
                    specialization,
                    function,
                    target,
                );
            }
            FunctionFunctionId::Function(_) => self
                .function_function_functions
                .push((index, lowered_never_host_function(specialization, target))),
        }
    }

    fn push_never_host_list_function_function(
        &mut self,
        index: usize,
        specialization: &SpecializationKey,
        function: crate::plan::execution::function::ListFunctionFunctionId,
        target: HostNeverFunctionId,
    ) {
        use crate::plan::execution::function::ListFunctionFunctionId as F;

        let lowered =
            lowered_never_host_function::<ListFunctionFunctionBody>(specialization, target);
        match function {
            F::Parameter { .. } => self
                .parameter_list_function_functions
                .push((index, lowered)),
            F::ParameterList { .. } => self
                .parameter_list_list_function_functions
                .push((index, lowered)),
            F::Int { .. } => self.int_list_function_functions.push((index, lowered)),
            F::String { .. } => self.string_list_function_functions.push((index, lowered)),
            F::BitArray { .. } => self
                .bit_array_list_function_functions
                .push((index, lowered)),
            F::UtfCodepoint { .. } => self
                .utf_codepoint_list_function_functions
                .push((index, lowered)),
            F::Custom { .. } => self.custom_list_function_functions.push((index, lowered)),
            F::Float { .. } => self.float_list_function_functions.push((index, lowered)),
            F::Bool { .. } => self.bool_list_function_functions.push((index, lowered)),
            F::Nil { .. } => self.nil_list_function_functions.push((index, lowered)),
            F::Tuple { .. } => self.tuple_list_function_functions.push((index, lowered)),
            F::List { .. } => self.list_list_function_functions.push((index, lowered)),
            F::Function { .. } => self.function_list_function_functions.push((index, lowered)),
        }
    }
}

fn lowered_never_host_function<Body>(
    specialization: &SpecializationKey,
    target: HostNeverFunctionId,
) -> LoweredSpecialization<ValueFunctionEntry<Body, HostedFunctionTarget<Body::HostValueTarget>>>
where
    Body: crate::plan::execution::function::ExecutionFunctionBody,
{
    lowered_host_function(specialization, HostedFunctionTarget::never(target))
}

impl FunctionTableBuilder {
    pub(in crate::plan::execution::lowering) fn finish_hosted<Profile: HostProfile>(
        self,
        functions: AdditionalFunctions<HostedExecutionProfile<Profile>>,
    ) -> SpecializationOutcome<Box<FunctionTables<HostedExecutionProfile<Profile>>>> {
        self.finish_profile(functions)
    }
}

#[cfg(test)]
mod tests {
    use crate::host::StatelessHostProfile;
    use crate::plan::execution::function::ValueFunctionEntry;
    use crate::{
        ExecutionError, HostFailure, HostModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use std::convert::Infallible;

    #[test]
    fn routes_non_returning_specializations_to_every_value_table() {
        let source = r#"
import host/control

pub type Boxed {
  Boxed(Int)
}

fn never_value() -> value { control.stop() }
fn int_value() -> Int { control.stop() }
fn float_value() -> Float { control.stop() }
fn string_value() -> String { control.stop() }
fn bit_array_value() -> BitArray { control.stop() }
fn utf_codepoint_value() -> UtfCodepoint { control.stop() }
fn custom_value() -> Boxed { control.stop() }
fn bool_value() -> Bool { control.stop() }
fn nil_value() -> Nil { control.stop() }
fn tuple_value() -> #(Int) { control.stop() }

pub fn main() {
  let _ = #(
    never_value,
    int_value,
    float_value,
    string_value,
    bit_array_value,
    utf_codepoint_value,
    custom_value,
    bool_value,
    nil_value,
    tuple_value,
  )
  int_value()
}
"#;
        let execution = hosted_execution(source);
        let tables = &execution.program.functions.value_returns;

        assert_eq!(
            [
                tables
                    .never_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .int_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .float_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .string_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bit_array_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .utf_codepoint_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .custom_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bool_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .nil_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .tuple_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
            ],
            [1; 10],
        );
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("stop should fail");
        assert!(matches!(
            error,
            ExecutionError::Host(error) if error.failure().message() == "table stop"
        ));
    }

    #[test]
    fn routes_non_returning_specializations_to_every_list_table() {
        let source = r#"
import host/control

pub type Boxed {
  Boxed(Int)
}

fn parameter_list() -> List(value) { control.stop() }
fn int_list() -> List(Int) { control.stop() }
fn string_list() -> List(String) { control.stop() }
fn bit_array_list() -> List(BitArray) { control.stop() }
fn utf_codepoint_list() -> List(UtfCodepoint) { control.stop() }
fn custom_list() -> List(Boxed) { control.stop() }
fn float_list() -> List(Float) { control.stop() }
fn bool_list() -> List(Bool) { control.stop() }
fn nil_list() -> List(Nil) { control.stop() }
fn tuple_list() -> List(#(Int)) { control.stop() }
fn parameter_list_list() -> List(List(value)) { control.stop() }
fn list_list() -> List(List(Int)) { control.stop() }
fn function_list() -> List(fn() -> Int) { control.stop() }

pub fn main() {
  #(
    parameter_list,
    int_list,
    string_list,
    bit_array_list,
    utf_codepoint_list,
    custom_list,
    float_list,
    bool_list,
    nil_list,
    tuple_list,
    parameter_list_list,
    list_list,
    function_list,
  )
}
"#;
        let execution = hosted_execution(source);
        let tables = &execution.program.functions.list_returns;

        assert_eq!(
            [
                tables
                    .parameter_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .int_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .string_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bit_array_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .utf_codepoint_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .custom_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .float_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bool_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .nil_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .tuple_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .parameter_list_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .list_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .function_list_functions
                    .iter()
                    .filter(|(_, function)| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
            ],
            [1; 13],
        );
    }

    #[test]
    fn routes_non_returning_specializations_to_every_function_table() {
        let source = r#"
import host/control

pub type Boxed {
  Boxed(Int)
}

fn int_function() -> fn() -> Int { control.stop() }
fn float_function() -> fn() -> Float { control.stop() }
fn string_function() -> fn() -> String { control.stop() }
fn bit_array_function() -> fn() -> BitArray { control.stop() }
fn utf_codepoint_function() -> fn() -> UtfCodepoint { control.stop() }
fn custom_function() -> fn() -> Boxed { control.stop() }
fn bool_function() -> fn() -> Bool { control.stop() }
fn nil_function() -> fn() -> Nil { control.stop() }
fn tuple_function() -> fn() -> #(Int) { control.stop() }
fn generic_function() -> fn(value) -> value { control.stop() }
fn never_function() -> fn() -> value { control.stop() }
fn parameter_list_function() -> fn() -> List(value) { control.stop() }
fn parameter_list_list_function() -> fn() -> List(List(value)) { control.stop() }
fn int_list_function() -> fn() -> List(Int) { control.stop() }
fn string_list_function() -> fn() -> List(String) { control.stop() }
fn bit_array_list_function() -> fn() -> List(BitArray) { control.stop() }
fn utf_codepoint_list_function() -> fn() -> List(UtfCodepoint) { control.stop() }
fn custom_list_function() -> fn() -> List(Boxed) { control.stop() }
fn float_list_function() -> fn() -> List(Float) { control.stop() }
fn bool_list_function() -> fn() -> List(Bool) { control.stop() }
fn nil_list_function() -> fn() -> List(Nil) { control.stop() }
fn tuple_list_function() -> fn() -> List(#(Int)) { control.stop() }
fn list_list_function() -> fn() -> List(List(Int)) { control.stop() }
fn function_list_function() -> fn() -> List(fn() -> Int) { control.stop() }
fn function_function() -> fn() -> fn() -> Int { control.stop() }

pub fn main() {
  #(
    int_function,
    float_function,
    string_function,
    bit_array_function,
    utf_codepoint_function,
    custom_function,
    bool_function,
    nil_function,
    tuple_function,
    generic_function,
    never_function,
    parameter_list_function,
    parameter_list_list_function,
    int_list_function,
    string_list_function,
    bit_array_list_function,
    utf_codepoint_list_function,
    custom_list_function,
    float_list_function,
    bool_list_function,
    nil_list_function,
    tuple_list_function,
    list_list_function,
    function_list_function,
    function_function,
  )
}
"#;
        let execution = hosted_execution(source);
        let tables = &execution.program.functions.function_returns;

        assert_eq!(
            [
                tables
                    .int_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .float_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .string_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bit_array_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .utf_codepoint_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .custom_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bool_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .nil_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .tuple_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .generic_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .never_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .parameter_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .parameter_list_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .int_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .string_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bit_array_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .utf_codepoint_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .custom_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .float_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .bool_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .nil_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .tuple_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .list_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .function_list_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
                tables
                    .function_function_functions
                    .iter()
                    .filter(|function| matches!(function, ValueFunctionEntry::Host(_)))
                    .count(),
            ],
            [1; 25],
        );
    }

    fn hosted_execution(source: &str) -> HostedExecution<StatelessHostProfile> {
        fn stop() -> Result<Infallible, HostFailure> {
            Err(HostFailure::new("table stop"))
        }

        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function("stop", stop as fn() -> Result<Infallible, HostFailure>)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
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
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");

        HostedExecution::from_module_plan(plan)
    }
}
