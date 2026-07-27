use super::{FunctionTables, write_function, write_table};
use crate::plan::execution::explain::{Explain, ExplainContext, FunctionLabel};
use crate::plan::execution::function::{BoolFunctionBody, IntFunctionBody, ValueFunctionEntry};
use crate::plan::execution::host::{
    HostBoolFunctionId, HostFunctionTables, HostIntFunctionId, HostedFunction,
};

pub(in crate::plan::execution) struct HostedFunctionTablesExplanation<'a> {
    tables: &'a FunctionTables<
        ValueFunctionEntry<IntFunctionBody, HostIntFunctionId>,
        ValueFunctionEntry<BoolFunctionBody, HostBoolFunctionId>,
    >,
    host_functions: &'a HostFunctionTables,
}

impl<'a> HostedFunctionTablesExplanation<'a> {
    pub(in crate::plan::execution) fn new(
        tables: &'a FunctionTables<
            ValueFunctionEntry<IntFunctionBody, HostIntFunctionId>,
            ValueFunctionEntry<BoolFunctionBody, HostBoolFunctionId>,
        >,
        host_functions: &'a HostFunctionTables,
    ) -> Self {
        Self {
            tables,
            host_functions,
        }
    }
}

impl Explain for HostedFunctionTablesExplanation<'_> {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table(context, "never", &self.tables.value_returns.never_functions);
        for (index, function) in self.tables.value_returns.int_functions.iter().enumerate() {
            match function {
                ValueFunctionEntry::Graph(function) => {
                    write_function(context, "int", index, function);
                }
                ValueFunctionEntry::Host(target) => {
                    write_hosted_function(context, "int", index, self.host_functions.int(*target));
                }
            }
        }
        write_table(context, "float", &self.tables.value_returns.float_functions);
        write_table(
            context,
            "string",
            &self.tables.value_returns.string_functions,
        );
        write_table(
            context,
            "bit_array",
            &self.tables.value_returns.bit_array_functions,
        );
        write_table(
            context,
            "utf_codepoint",
            &self.tables.value_returns.utf_codepoint_functions,
        );
        write_table(
            context,
            "custom",
            &self.tables.value_returns.custom_functions,
        );
        for (index, function) in self.tables.value_returns.bool_functions.iter().enumerate() {
            match function {
                ValueFunctionEntry::Graph(function) => {
                    write_function(context, "bool", index, function);
                }
                ValueFunctionEntry::Host(target) => {
                    write_hosted_function(
                        context,
                        "bool",
                        index,
                        self.host_functions.bool(*target),
                    );
                }
            }
        }
        write_table(context, "nil", &self.tables.value_returns.nil_functions);
        write_table(context, "tuple", &self.tables.value_returns.tuple_functions);
        context.write(&self.tables.list_returns);
        context.write(&self.tables.function_returns);
    }
}

fn write_hosted_function<Implementation>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    index: usize,
    function: &HostedFunction<Implementation>,
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
        HostModule, HostModules, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    #[test]
    fn writes_hosted_int_and_bool_targets_in_family_order() {
        let choose = |condition: bool, left: BigInt, right: BigInt| {
            if condition { left } else { right }
        };
        assert_eq!(
            choose(false, BigInt::from(10), BigInt::from(20)),
            BigInt::from(20),
        );
        assert_eq!(
            choose(true, BigInt::from(10), BigInt::from(20)),
            BigInt::from(10),
        );
        let all = |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool| {
            a && b && c && d && e && f && g
        };
        assert!(all(true, true, true, true, true, true, true));

        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("choose", choose)
            .expect("host function should be valid")
            .with_function("ready", <bool as Default>::default)
            .expect("host function should be valid")
            .with_function("all", all)
            .expect("host function should be valid");
        let hosts = HostModules::new([math]).expect("host modules should be unique");
        let source = r#"
import host/math

fn identity(value: Bool) {
  value
}

pub fn main() {
  let flag = True
  #(
    math.choose(flag, 1, 2),
    math.ready(),
    math.all(flag, flag, flag, flag, flag, flag, flag),
    identity(flag),
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
  host host_support::host/math.choose signature=fn(Bool, Int, Int) -> Int

function bool#0
  host host_support::host/math.ready signature=fn() -> Bool

function bool#1
  host host_support::host/math.all signature=fn(Bool, Bool, Bool, Bool, Bool, Bool, Bool) -> Bool

function bool#2
  entry b0 params=[%bool#0:shape#0(Bool)] captures=[]
  block b0 params=[%bool#0:shape#0(Bool)]
    return %bool#0

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %bool#0:shape#0(Bool) = bool.value True
    %int#0:shape#1(Int) = int.value 1
    %int#1:shape#1(Int) = int.value 2
    %int#2:shape#1(Int) = int.call int#0 args=[%bool#0, %int#0, %int#1]
    %bool#1:shape#0(Bool) = bool.call bool#0 args=[]
    %bool#2:shape#0(Bool) = bool.call bool#1 args=[%bool#0, %bool#0, %bool#0, %bool#0, %bool#0, %bool#0, %bool#0]
    %bool#3:shape#0(Bool) = bool.call bool#2 args=[%bool#0]
    %tuple#0:shape#2(#(Int, Bool, Bool, Bool)) = tuple.value elements=[%int#2, %bool#1, %bool#2, %bool#3]
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
}
