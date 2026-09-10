use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::plan::execution::function::{
    ListFunctionId as List, ProfiledCoreRuntimeFunctionId as Core,
    ProfiledFunctionFunctionId as Function, ProfiledRuntimeFunctionId as Root,
    RuntimeFunctionFunctionTarget, RuntimeListFunctionId,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::execution::Invocation;
use crate::runtime::state::list::ListValueId;
use crate::runtime::{EvaluatedValue, HostCallOrigin, RetainedValues};

pub(in crate::runtime) fn prepare_main<Profile: HostProfile>(
    plan: &HostedProgram<Profile>,
) -> Invocation<'_, HostedProgram<Profile>, EvaluatedValue> {
    macro_rules! call {
        ($id:expr, $map:expr) => {
            Invocation::execution(plan, $id, HostCallOrigin::Entry, RetainedValues::empty())
                .map(|value| Ok(($map)(value)))
        };
    }
    macro_rules! list {
        ($id:expr, $family:ident) => {
            call!($id, |value| EvaluatedValue::from(ListValueId::$family(
                value
            )))
        };
    }
    macro_rules! function {
        ($id:expr) => {
            Invocation::execution(plan, $id, HostCallOrigin::Entry, RetainedValues::empty())
                .map(|value| Ok(EvaluatedValue::Function(value.into())))
        };
    }
    match plan.main_runtime() {
        Root::External(id) => call!(id, EvaluatedValue::External),
        Root::Core(function) => match function {
            Core::Never(id) => {
                Invocation::execution(plan, id, HostCallOrigin::Entry, RetainedValues::empty())
                    .map(|never| match never {})
            }
            Core::Int(id) => call!(id, EvaluatedValue::Int),
            Core::Float(id) => call!(id, EvaluatedValue::Float),
            Core::String(id) => call!(id, EvaluatedValue::String),
            Core::BitArray(id) => call!(id, EvaluatedValue::BitArray),
            Core::UtfCodepoint(id) => call!(id, EvaluatedValue::UtfCodepoint),
            Core::Custom(id) => call!(id, EvaluatedValue::Custom),
            Core::Bool(id) => call!(id, EvaluatedValue::Bool),
            Core::Nil(id) => call!(id, |()| EvaluatedValue::Nil),
            Core::Tuple { id, .. } => call!(id, EvaluatedValue::Tuple),
            Core::List(id) => match id {
                RuntimeListFunctionId::External(id) => list!(id, External),
                RuntimeListFunctionId::Core(id) => match id {
                    List::Parameter(id) => list!(id, Parameter),
                    List::ParameterList(id) => list!(id, ParameterList),
                    List::Int(id) => list!(id, Int),
                    List::Float(id) => list!(id, Float),
                    List::String(id) => list!(id, String),
                    List::BitArray(id) => list!(id, BitArray),
                    List::UtfCodepoint(id) => list!(id, UtfCodepoint),
                    List::Custom(id) => list!(id, Custom),
                    List::Bool(id) => list!(id, Bool),
                    List::Nil(id) => list!(id, Nil),
                    List::Tuple(id) => list!(id, Tuple),
                    List::List(id) => list!(id, List),
                    List::Function(id) => list!(id, Function),
                },
            },
            Core::Function { id, .. } => match id {
                RuntimeFunctionFunctionTarget::Core(id) => match id {
                    Function::Generic(id) => function!(id),
                    Function::Never(id) => function!(id),
                    Function::Int(id) => function!(id),
                    Function::Float(id) => function!(id),
                    Function::String(id) => function!(id),
                    Function::BitArray(id) => function!(id),
                    Function::UtfCodepoint(id) => function!(id),
                    Function::Custom(id) => function!(id),
                    Function::Bool(id) => function!(id),
                    Function::Nil(id) => function!(id),
                    Function::Tuple(id) => function!(id),
                    Function::List(id) => function!(id),
                    Function::Function(id) => function!(id),
                    Function::External(never) => match never {},
                },
                RuntimeFunctionFunctionTarget::External(id) => match id {
                    crate::plan::execution::graph::ExternalFunctionCallTarget::Function(id) => {
                        function!(id)
                    }
                    crate::plan::execution::graph::ExternalFunctionCallTarget::ListFunction {
                        id,
                        ..
                    } => function!(id),
                },
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::execution_fixture::TestHost;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use crate::work_fixture::WorkComponent;
    use crate::{
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn hosted_main_preserves_scalar_values_and_source_failure() {
        for (source, expected) in [
            ("pub fn main() { 42 }", "42"),
            ("pub fn main() { 1.5 }", "1.5"),
            ("pub fn main() { \"text\" }", "\"text\""),
            ("pub fn main() { <<42>> }", "<<42>>"),
            (
                "pub fn main() { let assert <<c:utf8_codepoint>> = <<65>> c }",
                "'A'",
            ),
            (
                "pub type Item { Item(Int) } pub fn main() { Item(42) }",
                "Item(42)",
            ),
            ("pub fn main() { True }", "True"),
            ("pub fn main() { Nil }", "Nil"),
            ("pub fn main() { #(40, 2) }", "#(40, 2)"),
        ] {
            assert_main(source, Ok(expected));
        }
        assert_main(
            "pub fn main() { panic as \"entry failed\" }",
            Err("panic: entry failed"),
        );
    }

    #[test]
    fn hosted_main_materializes_each_list_family() {
        for (source, expected) in [
            ("pub fn main() -> List(a) { [] }", "[]"),
            ("pub fn main() -> List(List(a)) { [[]] }", "[[]]"),
            ("pub fn main() { [1, 2, 3] }", "[1, 2, 3]"),
            ("pub fn main() { [1.5] }", "[1.5]"),
            ("pub fn main() { [\"text\"] }", "[\"text\"]"),
            ("pub fn main() { [<<42>>] }", "[<<42>>]"),
            (
                "pub fn main() { let assert <<c:utf8_codepoint>> = <<65>> [c] }",
                "['A']",
            ),
            (
                "pub type Item { Item(Int) } pub fn main() { [Item(42)] }",
                "[Item(42)]",
            ),
            ("pub fn main() { [True] }", "[True]"),
            ("pub fn main() { [Nil] }", "[Nil]"),
            ("pub fn main() { [#(42, True)] }", "[#(42, True)]"),
            ("pub fn main() { [[1, 2, 3]] }", "[[1, 2, 3]]"),
            ("pub fn main() { [fn() { 42 }] }", "[//fn() { ... }]"),
        ] {
            assert_main(source, Ok(expected));
        }
    }

    #[test]
    fn hosted_main_returns_functions_without_invoking_them() {
        assert_main(
            "pub fn main() { fn(value) { value } }",
            Ok("//fn(a) { ... }"),
        );
        for source in [
            "pub fn main() { fn() { panic } }",
            "pub fn main() { fn() { 42 } }",
            "pub fn main() { fn() { 1.5 } }",
            "pub fn main() { fn() { \"text\" } }",
            "pub fn main() { fn() { <<42>> } }",
            "pub fn main() { fn() { let assert <<c:utf8_codepoint>> = <<65>> c } }",
            "pub type Item { Item(Int) } pub fn main() { fn() { Item(42) } }",
            "pub fn main() { fn() { True } }",
            "pub fn main() { fn() { Nil } }",
            "pub fn main() { fn() { #(42, True) } }",
            "pub fn main() { fn() { [42] } }",
            "pub fn main() { fn() { fn() { 42 } } }",
        ] {
            assert_main(source, Ok("//fn() { ... }"));
        }
    }

    #[test]
    fn hosted_main_preserves_external_list_and_function_families() {
        let mut state = ();
        assert!(std::ptr::eq(Profile::component_state(&mut state), &state));
        let stores = HostFutureStore::default();
        assert!(std::ptr::eq(Profile::component_stores(&stores), &stores));
        for (source, expected) in [
            ("pub fn main() { future.ready(42) }", "Work(...)"),
            ("pub fn main() -> List(future.Work(Int)) { [] }", "[]"),
            (
                "pub fn main() { fn() { future.ready(42) } }",
                "//fn() { ... }",
            ),
            (
                "pub fn main() { fn() { [future.ready(42)] } }",
                "//fn() { ... }",
            ),
        ] {
            let source = format!("import fixture/work as future\n{source}");
            assert_main(&source, Ok(expected));
        }
    }

    fn assert_main(source: &str, expected: Result<&str, &str>) {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("main", "main.gleam", source)],
                ),
            ],
            HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                .unwrap(),
        )
        .expect("typed source");
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap())
                .expect("sealed main");
        let host = TestHost::default();
        let mut echo = Vec::new();
        let result = host.block_on(execution.run_main(&host, &mut (), &mut echo));
        assert_eq!(
            result
                .map(|value| value.inspect().to_string())
                .map_err(|error| error.to_string()),
            expected.map(str::to_owned).map_err(str::to_owned),
            "{source}"
        );
        assert!(echo.is_empty());
    }
}
