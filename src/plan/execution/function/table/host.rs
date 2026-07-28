use super::{FunctionTables, write_function};
use crate::host::HostProfile;
use crate::plan::execution::explain::{Explain, ExplainContext, FunctionLabel};
use crate::plan::execution::function::{
    ExecutionFunctionBody, TailCallLabelIndex, ValueFunctionEntry,
};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::host::{
    HostFunctionTables, HostValueFunctionTarget, HostedExecutionProfile, HostedFunctionMetadata,
    HostedFunctionTarget,
};

pub(in crate::plan::execution) struct HostedFunctionTablesExplanation<'a, Profile: HostProfile> {
    tables: &'a FunctionTables<HostedExecutionProfile<Profile>>,
    host_functions: &'a HostFunctionTables<Profile>,
}

impl<'a, Profile: HostProfile> HostedFunctionTablesExplanation<'a, Profile> {
    pub(in crate::plan::execution) fn new(
        tables: &'a FunctionTables<HostedExecutionProfile<Profile>>,
        host_functions: &'a HostFunctionTables<Profile>,
    ) -> Self {
        Self {
            tables,
            host_functions,
        }
    }
}

impl<Profile: HostProfile> Explain for HostedFunctionTablesExplanation<'_, Profile> {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_hosted_table(
            context,
            "never",
            &self.tables.value_returns.never_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "int",
            &self.tables.value_returns.int_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "float",
            &self.tables.value_returns.float_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "string",
            &self.tables.value_returns.string_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "bit_array",
            &self.tables.value_returns.bit_array_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "utf_codepoint",
            &self.tables.value_returns.utf_codepoint_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "custom",
            &self.tables.value_returns.custom_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "bool",
            &self.tables.value_returns.bool_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "nil",
            &self.tables.value_returns.nil_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "tuple",
            &self.tables.value_returns.tuple_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.parameter",
            self.tables
                .list_returns
                .parameter_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.int",
            self.tables
                .list_returns
                .int_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.string",
            self.tables
                .list_returns
                .string_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.bit_array",
            self.tables
                .list_returns
                .bit_array_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.utf_codepoint",
            self.tables
                .list_returns
                .utf_codepoint_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.custom",
            self.tables
                .list_returns
                .custom_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.float",
            self.tables
                .list_returns
                .float_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.bool",
            self.tables
                .list_returns
                .bool_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.nil",
            self.tables
                .list_returns
                .nil_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.tuple",
            self.tables
                .list_returns
                .tuple_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.parameter_list",
            self.tables
                .list_returns
                .parameter_list_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.list",
            self.tables
                .list_returns
                .list_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "list.function",
            self.tables
                .list_returns
                .function_list_functions
                .iter()
                .map(|(_, function)| function),
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.int",
            &self.tables.function_returns.int_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.float",
            &self.tables.function_returns.float_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.string",
            &self.tables.function_returns.string_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.bit_array",
            &self.tables.function_returns.bit_array_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.utf_codepoint",
            &self
                .tables
                .function_returns
                .utf_codepoint_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.custom",
            &self.tables.function_returns.custom_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.bool",
            &self.tables.function_returns.bool_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.nil",
            &self.tables.function_returns.nil_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.tuple",
            &self.tables.function_returns.tuple_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.generic",
            &self.tables.function_returns.generic_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.never",
            &self.tables.function_returns.never_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.parameter",
            &self
                .tables
                .function_returns
                .parameter_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.parameter_list",
            &self
                .tables
                .function_returns
                .parameter_list_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.int",
            &self.tables.function_returns.int_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.string",
            &self.tables.function_returns.string_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.bit_array",
            &self
                .tables
                .function_returns
                .bit_array_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.utf_codepoint",
            &self
                .tables
                .function_returns
                .utf_codepoint_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.custom",
            &self.tables.function_returns.custom_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.float",
            &self.tables.function_returns.float_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.bool",
            &self.tables.function_returns.bool_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.nil",
            &self.tables.function_returns.nil_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.tuple",
            &self.tables.function_returns.tuple_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.list",
            &self.tables.function_returns.list_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.list.function",
            &self
                .tables
                .function_returns
                .function_list_function_functions,
            self.host_functions,
        );
        write_hosted_table(
            context,
            "function.function",
            &self.tables.function_returns.function_function_functions,
            self.host_functions,
        );
    }
}

fn write_hosted_table<'a, Profile, Body, Functions>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    functions: Functions,
    host_functions: &HostFunctionTables<Profile>,
) where
    Profile: HostProfile,
    Body: ExecutionFunctionBody + 'a,
    Body::HostValueTarget: HostValueFunctionTarget<Profile>,
    Body::Return: LocalLabel,
    Body::TailCall: TailCallLabelIndex,
    Functions: IntoIterator<
        Item = &'a ValueFunctionEntry<Body, HostedFunctionTarget<Body::HostValueTarget>>,
    >,
{
    for (index, function) in functions.into_iter().enumerate() {
        match function {
            ValueFunctionEntry::Graph(function) => {
                write_function(context, family, index, function);
            }
            ValueFunctionEntry::Host(target) => match target {
                HostedFunctionTarget::Value(target) => {
                    write_hosted_function(context, family, index, target.metadata(host_functions))
                }
                HostedFunctionTarget::Never(target) => write_hosted_function(
                    context,
                    family,
                    index,
                    host_functions.never(*target).metadata(),
                ),
            },
        }
    }
}

fn write_hosted_function(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    index: usize,
    function: &HostedFunctionMetadata,
) {
    context.push_str("\nfunction ");
    FunctionLabel::new(family, index).write(context.output());
    context.push_str("\n  host ");
    context.push_str(function.package());
    context.push_str("::");
    context.push_str(function.module());
    context.push('.');
    context.push_str(function.name());
    context.push_str(" signature=");
    context.write(function.type_());
    context.push('\n');
}

#[cfg(test)]
mod tests {
    use super::HostedFunctionTablesExplanation;
    use crate::plan::execution::explain;
    use crate::{
        BitArrayValue, ExecutionError, HostFailure, HostModule, HostProviderSet, HostedExecution,
        ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn writes_every_scalar_host_target_in_family_order() {
        let scalars = HostModule::new("host_support", "host/scalars")
            .expect("host module should be valid")
            .with_function("int", BigInt::default)
            .expect("host function should be valid")
            .with_function("float", || 2.5)
            .expect("host function should be valid")
            .with_function("string", EcoString::default)
            .expect("host function should be valid")
            .with_function("bit_array", std::convert::identity::<BitArrayValue>)
            .expect("host function should be valid")
            .with_function("utf_codepoint", || '5')
            .expect("host function should be valid")
            .with_function("bool", || true)
            .expect("host function should be valid")
            .with_function("nil", || ())
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([scalars]).expect("host modules should be unique");
        let source = r#"
import host/scalars

pub fn main() {
  #(
    scalars.int(),
    scalars.float(),
    scalars.string(),
    scalars.bit_array(<<>>),
    scalars.utf_codepoint(),
    scalars.bool(),
    scalars.nil(),
  )
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
        let expected = r#"
function int#0
  host host_support::host/scalars.int signature=fn() -> Int

function float#0
  host host_support::host/scalars.float signature=fn() -> Float

function string#0
  host host_support::host/scalars.string signature=fn() -> String

function bit_array#0
  host host_support::host/scalars.bit_array signature=fn(BitArray) -> BitArray

function utf_codepoint#0
  host host_support::host/scalars.utf_codepoint signature=fn() -> UtfCodepoint

function bool#0
  host host_support::host/scalars.bool signature=fn() -> Bool

function nil#0
  host host_support::host/scalars.nil signature=fn() -> Nil

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.call int#0 args=[]
    %float#0:shape#1(Float) = float.call float#0 args=[]
    %string#0:shape#2(String) = string.call string#0 args=[]
    %bit_array#0:shape#3(BitArray) = bit_array.value []
    %bit_array#1:shape#3(BitArray) = bit_array.call bit_array#0 args=[%bit_array#0]
    %utf_codepoint#0:shape#4(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
    %bool#0:shape#5(Bool) = bool.call bool#0 args=[]
    %nil#0:shape#6(Nil) = nil.call nil#0 args=[]
    %tuple#0:shape#7(#(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil)) = tuple.value elements=[%int#0, %float#0, %string#0, %bit_array#1, %utf_codepoint#0, %bool#0, %nil#0]
    return %tuple#0
"#;
        let mut actual = String::new();
        let mut context = explain::ExplainContext::new_hosted(&execution, &mut actual);
        context.write(&HostedFunctionTablesExplanation::new(
            &execution.program.functions,
            &execution.host_functions,
        ));

        assert_eq!(actual, expected);
    }

    #[test]
    fn writes_generic_never_and_concrete_non_returning_host_targets() {
        fn stop() -> Result<Infallible, HostFailure> {
            Err(HostFailure::new("stopped"))
        }

        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function(
                "generic_stop",
                stop as fn() -> Result<Infallible, HostFailure>,
            )
            .expect("host function should be valid")
            .with_fallible_function("int_stop", stop as fn() -> Result<Infallible, HostFailure>)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
        let source = r#"
import host/control

pub fn main() {
  let int_stop: fn() -> Int = control.int_stop
  let _ = int_stop == int_stop
  control.generic_stop()
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
        let expected = r#"
function never#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.int#0:shape#1(fn() -> Int) = function[Int] reference int#0
    %bool#0:shape#2(Bool) = bool.equal %function.int#0 %function.int#0
    tail never#1 args=[]

function never#1
  host host_support::host/control.generic_stop signature=fn() -> param#0

function int#0
  host host_support::host/control.int_stop signature=fn() -> Int
"#;
        let mut actual = String::new();
        let mut context = explain::ExplainContext::new_hosted(&execution, &mut actual);
        context.write(&HostedFunctionTablesExplanation::new(
            &execution.program.functions,
            &execution.host_functions,
        ));

        assert_eq!(actual, expected);
        assert!(matches!(
            execution.run_main(&mut (), &mut Vec::new()),
            Err(ExecutionError::Host(error)) if error.failure().message() == "stopped"
        ));
    }
}
