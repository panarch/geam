#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::plan::ValueType;
use geam_core::provider::{
    Call, Callback, FunctionValue, HostFailure, HostResult, StringValue, Value,
};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostSpecializationError,
    HostSpecializationErrorReason, HostedExecution, ModuleSource, PackageSource,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(package = "function_values", modules = [function_values], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "function_values", crate_path = geam_core)]
mod function_values {
    use super::{
        BigInt, Call, Callback, FunctionValue, HostFailure, HostResult, StringValue, Value,
    };

    #[geam_macros::custom(input = HolderInput)]
    enum Holder<Item> {
        Held(FunctionValue<fn(Value<Item>) -> StringValue>),
    }

    #[geam_macros::function]
    fn keep<Item>(
        value: FunctionValue<fn(Value<Item>) -> StringValue>,
    ) -> FunctionValue<fn(Value<Item>) -> StringValue> {
        value.clone()
    }

    #[geam_macros::function]
    fn has_callback<Item, Output>(value: FunctionValue<fn(Value<Item>) -> Value<Output>>) -> bool {
        value.callback().is_some()
    }

    #[geam_macros::function]
    fn pick<Item>(
        values: List<FunctionValue<fn(Value<Item>) -> StringValue>>,
    ) -> FunctionValue<fn(Value<Item>) -> StringValue> {
        let function = values.get(1).expect("second source function");
        drop(values);
        function
    }

    #[geam_macros::function]
    fn keep_list<Item>(
        values: List<FunctionValue<fn(Value<Item>) -> StringValue>>,
    ) -> List<FunctionValue<fn(Value<Item>) -> StringValue>> {
        values
    }

    #[geam_macros::function]
    fn wrap<Item>(value: FunctionValue<fn(Value<Item>) -> StringValue>) -> Holder<Item> {
        Holder::Held(value)
    }

    #[geam_macros::function]
    fn unwrap<Item>(value: HolderInput<Item>) -> FunctionValue<fn(Value<Item>) -> StringValue> {
        let HolderInput::Held(function) = value;
        function
    }

    #[geam_macros::function]
    fn compounds<Item>(
        value: (
            Option<FunctionValue<fn(Value<Item>) -> StringValue>>,
            Result<FunctionValue<fn(Value<Item>) -> StringValue>, StringValue>,
        ),
    ) -> (
        Option<FunctionValue<fn(Value<Item>) -> StringValue>>,
        Result<FunctionValue<fn(Value<Item>) -> StringValue>, StringValue>,
    ) {
        value
    }

    #[geam_macros::function]
    fn pick_compound<Item>(
        values: List<(
            Option<FunctionValue<fn(Value<Item>) -> StringValue>>,
            Result<FunctionValue<fn(Value<Item>) -> StringValue>, StringValue>,
        )>,
    ) -> (
        Option<FunctionValue<fn(Value<Item>) -> StringValue>>,
        Result<FunctionValue<fn(Value<Item>) -> StringValue>, StringValue>,
    ) {
        values.get(1).expect("second compound function item")
    }

    #[geam_macros::function(await)]
    async fn apply(
        #[geam_macros::call] call: &mut Call<()>,
        function: FunctionValue<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        let callback = function.callback().expect("concrete input is invocable");
        drop(function);
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function(await)]
    async fn empty_input<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        function: FunctionValue<fn(Vec<Value<Item>>) -> bool>,
    ) -> HostResult<bool> {
        call.invoke(
            &function.callback().expect("empty List(item) is inhabited"),
            (Vec::new(),),
        )
        .await
    }

    #[geam_macros::function(await)]
    async fn produced<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        factory: Callback<fn() -> FunctionValue<fn(Value<Item>) -> StringValue>>,
    ) -> HostResult<FunctionValue<fn(Value<Item>) -> StringValue>> {
        call.invoke(&factory, ()).await
    }

    #[geam_macros::function]
    fn nested_strict<Outer, Inner>(
        _function: FunctionValue<fn(Value<Outer>) -> Callback<fn(Value<Inner>) -> bool>>,
    ) -> bool {
        true
    }

    #[geam_macros::function]
    fn fail() -> HostResult<BigInt> {
        Err(HostFailure::new("retained callback failed").into())
    }
}

struct Profile;
#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}
#[derive(Default)]
struct State {
    component: <Component as HostProviderComponent>::RunState,
}
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = ProfileStores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &ProfileStores) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }
    fn component_state(state: &mut State) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const DECLARATIONS: &str = r#"
import gleam/option.{type Option, Some, None}
@external(erlang, "function_values", "keep")
fn keep(value: fn(item) -> String) -> fn(item) -> String
@external(erlang, "function_values", "has_callback")
fn has_callback(value: fn(item) -> output) -> Bool
@external(erlang, "function_values", "pick")
fn pick(values: List(fn(item) -> String)) -> fn(item) -> String
@external(erlang, "function_values", "keep_list")
fn keep_list(values: List(fn(item) -> String)) -> List(fn(item) -> String)
pub type Holder(item) { Held(fn(item) -> String) }
@external(erlang, "function_values", "wrap")
fn wrap(value: fn(item) -> String) -> Holder(item)
@external(erlang, "function_values", "unwrap")
fn unwrap(value: Holder(item)) -> fn(item) -> String
@external(erlang, "function_values", "compounds")
fn compounds(value: #(Option(fn(item) -> String), Result(fn(item) -> String, String))) -> #(Option(fn(item) -> String), Result(fn(item) -> String, String))
@external(erlang, "function_values", "pick_compound")
fn pick_compound(values: List(#(Option(fn(item) -> String), Result(fn(item) -> String, String)))) -> #(Option(fn(item) -> String), Result(fn(item) -> String, String))
@external(erlang, "function_values", "apply")
fn apply(function: fn(Int) -> Int, value: Int) -> Int
@external(erlang, "function_values", "empty_input")
fn empty_input(function: fn(List(item)) -> Bool) -> Bool
@external(erlang, "function_values", "produced")
fn produced(factory: fn() -> fn(item) -> String) -> fn(item) -> String
@external(erlang, "function_values", "nested_strict")
fn nested_strict(function: fn(outer) -> fn(inner) -> Bool) -> Bool
@external(erlang, "function_values", "fail")
fn fail() -> Int
"#;

fn execution(body: &str) -> Result<HostedExecution<Profile>, HostSpecializationError> {
    let source = format!("{DECLARATIONS}\n{body}");
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers).unwrap();
    let typed = compile_typed_host_program(
        "function_values",
        "function_values",
        [
            PackageSource::new(
                "function_values",
                ["gleam_stdlib"],
                [ModuleSource::new(
                    "function_values",
                    "src/function_values.gleam",
                    source,
                )],
            ),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    "pub type Option(item) { Some(item) None }",
                )],
            ),
        ],
        hosts,
    )
    .unwrap();
    HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap())
}

fn run(body: &str, expected: &str) {
    let mut execution = execution(body).unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = State::default();
    let mut echo = Vec::new();
    let result = host
        .block_on(execution.run_main(&host, &mut state, &mut echo))
        .unwrap();
    assert_eq!(result.inspect().to_string(), expected);
    assert!(echo.is_empty());
}

#[test]
fn symbolic_function_values_preserve_identity_and_captures_through_owned_and_nested_views() {
    run(
        r#"
pub fn main() {
  let label = "retained"
  let function = fn(_) { label }
  let alias = keep(function)
  let selected = pick([fn(_) { "unused" }, function])
  let assert [listed] = keep_list([function])
  let assert #(Some(optional), Ok(result)) = compounds(#(Some(function), Ok(function)))
  let assert #(Some(lazy_optional), Ok(lazy_result)) = pick_compound([#(None, Error("unused")), #(Some(function), Ok(function))])
  let custom = unwrap(wrap(function))
  let returned = produced(fn() { function })
  #(has_callback(function), alias == function, selected == function, listed == function, optional == function, result == function, lazy_optional == function, lazy_result == function, custom == function, returned == function)
}
"#,
        "#(False, True, True, True, True, True, True, True, True, True)",
    );
}

#[test]
fn concrete_and_inhabited_list_inputs_project_existing_typed_callbacks() {
    run(
        r#"
pub fn main() {
  let captured = 7
  let function = fn(value) { value + captured }
  let selected = pick([fn(_) { "unused" }, fn(value) { case value { True -> "kept" False -> "other" } }])
  #(has_callback(function), apply(function, 5), apply(function, 6), selected(True), empty_input(fn(items) { items == [] }))
}
"#,
        "#(True, 12, 13, \"kept\", True)",
    );
}

#[test]
fn never_returning_functions_keep_an_invocable_input_representation() {
    run(
        r#"pub fn main() { has_callback(fn(_: Int) { panic as "not invoked" }) }"#,
        "True",
    );
}

#[test]
fn a_storable_outer_function_does_not_relax_nested_strict_callback_permissions() {
    let error = execution(r#"pub fn main() { nested_strict(fn(_) { fn(_) { True } }) }"#)
        .err()
        .expect("nested strict input must remain rejected");
    assert_eq!(error.package(), "function_values");
    assert_eq!(error.module(), "function_values");
    assert_eq!(error.function(), "nested_strict");
    let callback = match error.reason() {
        HostSpecializationErrorReason::UninhabitedCallbackArguments { callback } => Some(callback),
        _ => None,
    };
    let callback = callback.expect("nested strict input must remain rejected");
    assert!(matches!(
        callback.argument_types(),
        [ValueType::Parameter(_)]
    ));
    assert_eq!(callback.return_(), &ValueType::Bool);
}

#[test]
fn projected_callbacks_preserve_original_panic_and_provider_failure_origins() {
    use geam_core::execution::RunError;
    use geam_core::{ExecutionError, PanicKind, PanicMessage};
    for source_panic in [true, false] {
        let body = if source_panic {
            r#"fn body(_: Int) { panic as "retained source panic" }
pub fn main() { apply(body, 7) }"#
        } else {
            "fn body(_: Int) { fail() }\npub fn main() { apply(body, 7) }"
        };
        let mut execution = execution(body).unwrap();
        let host = execution_fixture::TestHost::default();
        let mut state = State::default();
        let mut echo = Vec::new();
        let error = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .expect_err("projected callback must propagate its original failure");
        let panic = match &error {
            RunError::Execution(ExecutionError::Panic(panic)) => Some(panic),
            _ => None,
        };
        let failure = match &error {
            RunError::Execution(ExecutionError::Host(error)) => Some(error),
            _ => None,
        };
        if source_panic {
            let panic = panic.expect("source panic keeps its failure domain");
            assert!(failure.is_none());
            assert_eq!(panic.kind(), PanicKind::Panic);
            assert_eq!(
                panic.message(),
                &PanicMessage::Explicit("retained source panic".into())
            );
            assert_eq!(panic.site().module(), "function_values");
            assert_eq!(panic.site().function(), "body");
        } else {
            let error = failure.expect("provider failure keeps its failure domain");
            assert!(panic.is_none());
            assert_eq!(error.package(), "function_values");
            assert_eq!(error.module(), "function_values");
            assert_eq!(error.function(), "fail");
            assert_eq!(error.failure().message(), "retained callback failed");
            assert_eq!(error.location().site().unwrap().function(), "body");
            assert_eq!(
                error.location().path().unwrap().as_str(),
                "src/function_values.gleam"
            );
        }
        assert!(echo.is_empty());
    }
}
