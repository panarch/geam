use super::{FunctionTables, write_function};
use crate::host::HostProfile;
use crate::plan::execution::explain::{Explain, ExplainContext, FunctionLabel};
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionNeverFunctionBody, FunctionBodyOwner, HostedExecutionGraph,
    TailCallLabelIndex, ValueFunctionEntry,
};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::host::{
    HostFunctionTables, HostNeverFunctionId, HostedExecutionProfile, HostedFunction,
    HostedFunctionTarget,
};

pub(in crate::plan::execution) struct HostedFunctionTablesExplanation<'a, Profile: HostProfile> {
    tables: &'a FunctionTables<HostedExecutionProfile>,
    host_functions: &'a HostFunctionTables<Profile>,
}

impl<'a, Profile: HostProfile> HostedFunctionTablesExplanation<'a, Profile> {
    pub(in crate::plan::execution) fn new(
        tables: &'a FunctionTables<HostedExecutionProfile>,
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
        write_hosted_never_table(
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
            "external",
            &self.tables.value_returns.external_functions,
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
            "list.external",
            self.tables
                .list_returns
                .external_list_functions
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
            "function.external",
            &self.tables.function_returns.external_function_functions,
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
            "function.list.external",
            &self
                .tables
                .function_returns
                .external_list_function_functions,
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

fn write_hosted_never_table<'a, Profile, Functions>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    functions: Functions,
    host_functions: &HostFunctionTables<Profile>,
) where
    Profile: HostProfile,
    Functions: IntoIterator<
        Item = &'a ValueFunctionEntry<
            ExecutionNeverFunctionBody<HostedExecutionProfile>,
            HostNeverFunctionId,
        >,
    >,
{
    for (index, function) in functions.into_iter().enumerate() {
        match function {
            ValueFunctionEntry::Graph(function) => {
                write_function(context, family, index, function);
            }
            ValueFunctionEntry::Host(target) => {
                write_hosted_function(context, family, index, host_functions.never(*target));
            }
        }
    }
}

fn write_hosted_table<'a, Profile, Body, Functions>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    functions: Functions,
    host_functions: &HostFunctionTables<Profile>,
) where
    Profile: HostProfile,
    Body: ExecutionFunctionBody + FunctionBodyOwner<Graph = HostedExecutionGraph> + 'a,
    Body::Return: LocalLabel,
    Body::TailCall: TailCallLabelIndex,
    Functions: IntoIterator<Item = &'a ValueFunctionEntry<Body, HostedFunctionTarget<Body>>>,
{
    for (index, function) in functions.into_iter().enumerate() {
        match function {
            ValueFunctionEntry::Graph(function) => {
                write_function(context, family, index, function);
            }
            ValueFunctionEntry::Host(target) => match target {
                HostedFunctionTarget::Value(target) => {
                    write_hosted_function(context, family, index, host_functions.value(target))
                }
                HostedFunctionTarget::Never(target) => {
                    write_hosted_function(context, family, index, host_functions.never(*target))
                }
            },
        }
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
    use crate::host::test::{StatelessTestProvider, TestTypeParameter, stateless_identity};
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, HostCall, HostCallCompletion, HostCallError,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostProvider,
        HostProviderModule,
    };
    use crate::plan::execution::explain;
    use crate::{
        BitArrayValue, HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct CounterSchema;

    struct CounterProvider;

    type HostCounter = HostExternalType<CounterSchema>;

    impl HostExternalSchema for CounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<CounterSchema> for ExternalTestProfile {
        type Payload = BigInt;

        fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
            &stores.integers
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(value, &mut hasher);
            std::hash::Hasher::finish(&hasher)
        }

        fn inspect(
            _: &crate::host::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> EcoString {
            value.to_string().into()
        }
    }

    impl HostProvider<ExternalTestProfile> for CounterProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalTestProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let _ = call.state();
        let counter = call.create_external(value);
        Ok(call.return_value(counter))
    }

    #[test]
    fn counter_fixture_source_semantics_are_exact() {
        let retained_hash = |_: &crate::runtime::StoredRuntimeValue| 0;
        let hashing = crate::host::HostExternalHashing::new(&retained_hash);
        let value = BigInt::from(7);
        let mut expected = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&value, &mut expected);

        assert_eq!(
            <ExternalTestProfile as HostExternalStorage<CounterSchema>>::source_hash(
                &hashing, &value,
            ),
            std::hash::Hasher::finish(&expected),
        );
    }

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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
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
    fn writes_compound_host_targets_in_their_exact_family_tables() {
        let generic = HostModule::new("host_support", "host/generic")
            .expect("host module should be valid")
            .with_scoped_function::<
                StatelessTestProvider,
                (TestTypeParameter,),
                TestTypeParameter,
                _,
            >("identity", stateless_identity)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([generic]).expect("host modules should be unique");
        let source = r#"
import host/generic

pub type Marker {
  Marker(Int)
}

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  #(
    generic.identity(Marker(1)),
    generic.identity(#(2, True)),
    generic.identity([3]),
    generic.identity(increment),
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let expected = r#"
function int#0
  entry b0 params=[%int#0:shape#0(Int)] captures=[]
  block b0 params=[%int#0:shape#0(Int)]
    %int#1:shape#0(Int) = int.value 1
    %int#2:shape#0(Int) = int.add %int#0 %int#1
    return %int#2

function custom#0
  host host_support::host/generic.identity signature=fn(custom_type#0) -> custom_type#0

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 1
    %custom#0:shape#1(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0]
    %custom#1:shape#2(custom_type#0) = custom.call custom#0 args=[%custom#0]
    %int#1:shape#0(Int) = int.value 2
    %bool#0:shape#3(Bool) = bool.value True
    %tuple#0:shape#4(#(Int, Bool)) = tuple.value elements=[%int#1, %bool#0]
    %tuple#1:shape#4(#(Int, Bool)) = tuple.call tuple#1 args=[%tuple#0]
    %int#2:shape#0(Int) = int.value 3
    %list.int#0:shape#5(list_type#0) = list.int[type#0] value elements=[%int#2]
    %list.int#1:shape#5(list_type#0) = list.int[type#0] call list.int#0 args=[%list.int#0]
    %function.int#0:shape#6(fn(Int) -> Int) = function[Int] reference int#0
    %function.int#1:shape#6(fn(Int) -> Int) = function[Int] call function.int#0 args=[%function.int#0]
    %tuple#2:shape#7(#(custom_type#0, #(Int, Bool), list_type#0, fn(Int) -> Int)) = tuple.value elements=[%custom#1, %tuple#1, %list.int#1, %function.int#1]
    return %tuple#2

function tuple#1
  host host_support::host/generic.identity signature=fn(#(Int, Bool)) -> #(Int, Bool)

function list.int#0
  host host_support::host/generic.identity signature=fn(list_type#0) -> list_type#0

function function.int#0
  host host_support::host/generic.identity signature=fn(fn(Int) -> Int) -> fn(Int) -> Int
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
    fn writes_external_value_list_and_function_tables_in_family_order() {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<CounterSchema>()
            .expect("external type should be valid")
            .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
                "new_counter",
                new_counter,
            )
            .expect("external function should be valid");
        let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn pass(counter: Counter) -> Counter {
  counter
}

fn values(counter: Counter) -> List(Counter) {
  [counter]
}

fn maker() -> fn(Int) -> Counter {
  new_counter
}

fn list_maker() -> fn(Counter) -> List(Counter) {
  fn(counter) { [counter] }
}

pub fn main() {
  let counter = pass(new_counter(1))
  #(
    counter,
    values(counter),
    maker()(2),
    list_maker()(counter),
    counter == new_counter(1),
  )
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("external source should compile");
        let plan = plan_host_program(typed).expect("external source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
        let expected = r#"
function external#0
  host application::main.new_counter signature=fn(Int) -> external_type#0

function external#1
  entry b0 params=[%external#0:shape#1(external_type#0)] captures=[]
  block b0 params=[%external#0:shape#1(external_type#0)]
    return %external#0

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 1
    %external#0:shape#1(external_type#0) = external.call external#0 args=[%int#0]
    %external#1:shape#1(external_type#0) = external.call external#1 args=[%external#0]
    %list.external#0:shape#2(list_type#0) = list.external[type#0] call list.external#0 args=[%external#1]
    %function.external#0:shape#3(fn(Int) -> external_type#0) = function[External] call function.external#0 args=[]
    %int#1:shape#0(Int) = int.value 2
    %external#2:shape#1(external_type#0) = external.function_call %function.external#0 args=[%int#1]
    %function.list.external#0:shape#4(fn(external_type#0) -> list_type#0) = function[List] call function.list.external#0 args=[]
    %list.external#1:shape#2(list_type#0) = list.external[type#0] function_call %function.list.external#0 args=[%external#1]
    %int#2:shape#0(Int) = int.value 1
    %external#3:shape#1(external_type#0) = external.call external#0 args=[%int#2]
    %bool#0:shape#5(Bool) = bool.equal %external#1 %external#3
    %tuple#0:shape#6(#(external_type#0, list_type#0, external_type#0, list_type#0, Bool)) = tuple.value elements=[%external#1, %list.external#0, %external#2, %list.external#1, %bool#0]
    return %tuple#0

function list.external#0
  entry b0 params=[%external#0:shape#1(external_type#0)] captures=[]
  block b0 params=[%external#0:shape#1(external_type#0)]
    %list.external#0:shape#2(list_type#0) = list.external[type#0] value elements=[%external#0]
    return %list.external#0

function list.external#1
  entry b0 params=[%external#0:shape#1(external_type#0)] captures=[]
  block b0 params=[%external#0:shape#1(external_type#0)]
    %list.external#0:shape#2(list_type#0) = list.external[type#0] value elements=[%external#0]
    return %list.external#0

function function.external#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.external#0:shape#3(fn(Int) -> external_type#0) = function[External] reference external#0
    return %function.external#0

function function.list.external#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.list.external#0:shape#4(fn(external_type#0) -> list_type#0) = function[List] closure target=list.external#1 captures=[]
    return %function.list.external#0
"#;
        let mut actual = String::new();
        let mut context = explain::ExplainContext::new_hosted(&execution, &mut actual);
        context.write(&HostedFunctionTablesExplanation::new(
            &execution.program.functions,
            &execution.host_functions,
        ));

        assert_eq!(actual, expected);

        let returned = execution
            .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
            .expect("external function tables should execute");
        assert_eq!(returned.inspect().to_string(), "#(1, [1], 2, [1], True)");
    }
}
