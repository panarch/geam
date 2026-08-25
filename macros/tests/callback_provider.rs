use ecow::EcoString;
use geam_core::provider::{Call, Callback, HostFailure, HostResult, List, Value};
use geam_core::{
    ExecutionError, HostComponentProfile, HostLocation, HostModule, HostProfile,
    HostProviderComponent, HostProviderComponentRegistration, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, PanicKind, PanicMessage, Value as RuntimeValue,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[derive(Default)]
pub struct RunState {
    entries: Vec<EcoString>,
}

#[geam_macros::provider(
    package = "callback_provider",
    state = RunState,
    modules = [callback_provider],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "callback_provider", crate_path = geam_core)]
mod callback_provider {
    use super::{
        BigInt, Call, Callback, EcoString, HostFailure, HostResult, List, RunState, Value,
    };

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    struct Token(EcoString);

    #[geam_macros::custom(input = DecisionInput)]
    enum Decision {
        Accepted(EcoString),
        Rejected,
    }

    #[geam_macros::function]
    fn record(#[geam_macros::call] call: &mut Call<RunState>, entry: EcoString) -> () {
        call.state_mut().entries.push(entry);
    }

    #[geam_macros::function]
    fn entries(#[geam_macros::call] call: &Call<RunState>) -> EcoString {
        call.state().entries.join("/").into()
    }

    #[geam_macros::function]
    fn around<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> Value<Item>>,
    ) -> HostResult<Value<Item>> {
        call.state_mut().entries.push("before".into());
        let returned = call.invoke(callback, ())?;
        call.state_mut().entries.push("after".into());
        Ok(returned)
    }

    #[geam_macros::function]
    fn apply<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(Value<Item>) -> Value<Item>>,
        value: Value<Item>,
    ) -> HostResult<Value<Item>> {
        call.invoke(callback, (value,))
    }

    #[geam_macros::function]
    fn rotate(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(EcoString, BigInt) -> (BigInt, EcoString)>,
        label: EcoString,
        number: BigInt,
    ) -> HostResult<(BigInt, EcoString)> {
        call.invoke(callback, (label, number))
    }

    #[geam_macros::function]
    fn decide(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(Token, Decision) -> DecisionInput>,
        label: EcoString,
    ) -> HostResult<Decision> {
        let returned = call.invoke(callback, (Token(label.clone()), Decision::Accepted(label)))?;
        Ok(match returned {
            DecisionInput::Accepted(label) => Decision::Accepted(label),
            DecisionInput::Rejected => Decision::Rejected,
        })
    }

    #[geam_macros::function]
    fn list_total(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn(Vec<((BigInt, EcoString), self::Token)>) -> geam_core::List<BigInt>>,
    ) -> HostResult<BigInt> {
        let values = call.invoke(
            callback,
            (vec![
                ((1.into(), "one".into()), Token("first".into())),
                ((2.into(), "two".into()), Token("second".into())),
                ((3.into(), "three".into()), Token("third".into())),
            ],),
        )?;
        let mut total = BigInt::from(0);
        for index in 0..values.len() {
            total += values.get(index).expect("callback List index must exist");
        }
        Ok(total)
    }

    #[geam_macros::function]
    fn classify(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<
            fn((EcoString, BigInt), Result<EcoString, Decision>, Option<BigInt>) -> Option<BigInt>,
        >,
    ) -> HostResult<Option<BigInt>> {
        call.invoke(
            callback,
            (
                ("pair".into(), 2.into()),
                Ok("result".into()),
                Some(7.into()),
            ),
        )
    }

    #[geam_macros::function]
    fn inspect_callback<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> (Value<Item>, List<EcoString>)>,
    ) -> HostResult<(Value<Item>, BigInt)> {
        let (value, messages) = call.invoke(callback, ())?;
        Ok((value, messages.len().into()))
    }

    #[geam_macros::function]
    fn fail() -> HostResult<()> {
        Err(HostFailure::new("callback provider failed").into())
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

#[derive(Default)]
struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const SOURCE: &str = r#"
import gleam/option.{type Option}

@external(erlang, "callback_provider", "record")
fn record(entry: String) -> Nil

@external(erlang, "callback_provider", "entries")
fn entries() -> String

@external(erlang, "callback_provider", "around")
fn around(callback: fn() -> item) -> item

@external(erlang, "callback_provider", "apply")
fn apply(callback: fn(item) -> item, value: item) -> item

@external(erlang, "callback_provider", "rotate")
fn rotate(callback: fn(String, Int) -> #(Int, String), label: String, number: Int) -> #(Int, String)

@external(erlang, "callback_provider", "Token")
pub type Token

pub type Decision {
  Accepted(String)
  Rejected
}

@external(erlang, "callback_provider", "decide")
fn decide(callback: fn(Token, Decision) -> Decision, label: String) -> Decision

@external(erlang, "callback_provider", "list_total")
fn list_total(callback: fn(List(#(#(Int, String), Token))) -> List(Int)) -> Int

@external(erlang, "callback_provider", "classify")
fn classify(callback: fn(#(String, Int), Result(String, Decision), Option(Int)) -> Option(Int)) -> Option(Int)

@external(erlang, "callback_provider", "inspect_callback")
fn inspect_callback(callback: fn() -> #(item, List(String))) -> #(item, Int)

@external(erlang, "callback_provider", "fail")
fn fail() -> Nil

fn body() {
  record("inside")
  41
}

fn increment(value: Int) -> Int {
  value + 1
}

fn rotate_value(label: String, number: Int) -> #(Int, String) {
  #(number + 1, label <> "!")
}

fn keep_decision(_token: Token, decision: Decision) -> Decision {
  decision
}

fn keep_list(values: List(#(#(Int, String), Token))) -> List(Int) {
  case values {
    [] -> []
    [#(#(value, _), _), ..rest] -> [value, ..keep_list(rest)]
  }
}

fn classify_values(pair, result, optional) {
  assert pair == #("pair", 2)
  assert result == Ok("result")
  optional
}

fn callback_pair() {
  #(42, ["first", "second"])
}

fn fail_callback() {
  fail()
}

fn panic_callback() -> Int {
  panic as "callback panic"
}

pub fn main() {
  #(
    around(body),
    apply(increment, 4),
    rotate(rotate_value, "tag", 8),
    decide(keep_decision, "accepted"),
    list_total(keep_list),
    classify(classify_values),
    inspect_callback(callback_pair),
    entries(),
  )
}
"#;

const OPTION_SOURCE: &str = r#"
pub type Option(value) {
  Some(value)
  None
}
"#;

fn execution(source: &str) -> HostedExecution<Profile> {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored callback component should register");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("macro-authored callback module should be unique");
    let typed = compile_typed_host_program(
        "callback_provider",
        "callback_provider",
        [
            PackageSource::new(
                "callback_provider",
                ["gleam_stdlib"],
                [ModuleSource::new(
                    "callback_provider",
                    "src/callback_provider.gleam",
                    source,
                )],
            ),
            PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "gleam/option",
                    "src/gleam/option.gleam",
                    OPTION_SOURCE,
                )],
            ),
        ],
        hosts,
    )
    .expect("complete callback provider source should compile");
    let plan = plan_host_program(typed).expect("matching callback provider should plan");
    HostedExecution::try_from_module_plan(plan).expect("matching callback provider should seal")
}

#[test]
fn callbacks_reenter_the_component_and_preserve_typed_results() {
    let returned = execution(SOURCE)
        .run_main(&mut ProfileState::default(), &mut Vec::new())
        .expect("typed callbacks should execute");

    let RuntimeValue::Tuple(values) = returned else {
        panic!("callback result should preserve the complete tuple");
    };
    assert_eq!(values[0], RuntimeValue::Int(41.into()));
    assert_eq!(values[1], RuntimeValue::Int(5.into()));
    assert_eq!(
        values[2],
        RuntimeValue::Tuple(vec![
            RuntimeValue::Int(9.into()),
            RuntimeValue::String("tag!".into()),
        ]),
    );
    let RuntimeValue::Custom(decision) = &values[3] else {
        panic!("callback custom return should preserve its constructor");
    };
    assert_eq!(decision.constructor_name().as_str(), "Accepted");
    assert_eq!(decision.constructor_index(), 0);
    assert_eq!(decision.fields().len(), 1);
    assert_eq!(
        decision.fields()[0].value(),
        &RuntimeValue::String("accepted".into()),
    );
    assert_eq!(values[4], RuntimeValue::Int(6.into()));
    let RuntimeValue::Custom(optional) = &values[5] else {
        panic!("callback Option return should preserve its constructor");
    };
    assert_eq!(optional.constructor_name().as_str(), "Some");
    assert_eq!(optional.constructor_index(), 0);
    assert_eq!(optional.fields().len(), 1);
    assert_eq!(optional.fields()[0].value(), &RuntimeValue::Int(7.into()));
    assert_eq!(
        values[6],
        RuntimeValue::Tuple(vec![
            RuntimeValue::Int(42.into()),
            RuntimeValue::Int(2.into())
        ]),
    );
    assert_eq!(
        values[7],
        RuntimeValue::String("before/inside/after".into()),
    );
}

#[test]
fn nested_provider_failure_remains_the_original_execution_error() {
    let source = SOURCE.replace(
        "#(\n    around(body),\n    apply(increment, 4),\n    rotate(rotate_value, \"tag\", 8),\n    decide(keep_decision, \"accepted\"),\n    list_total(keep_list),\n    classify(classify_values),\n    inspect_callback(callback_pair),\n    entries(),\n  )",
        "around(fail_callback)",
    );
    let error = execution(&source)
        .run_main(&mut ProfileState::default(), &mut Vec::new())
        .expect_err("nested provider failure should stop the outer callback");

    let ExecutionError::Host(error) = error else {
        panic!("nested provider failure should remain a host error");
    };
    assert_eq!(error.package().as_str(), "callback_provider");
    assert_eq!(error.module().as_str(), "callback_provider");
    assert_eq!(error.function().as_str(), "fail");
    assert_eq!(
        error.failure().message().as_str(),
        "callback provider failed"
    );
    let HostLocation::Resolved { site, path, line } = error.location() else {
        panic!("source callback host failure should preserve its source call site");
    };
    assert_eq!(site.module().as_str(), "callback_provider");
    assert_eq!(site.function().as_str(), "fail_callback");
    assert_eq!(path.as_str(), "src/callback_provider.gleam");
    assert_eq!(*line, 77);
}

#[test]
fn nested_source_panic_is_not_rewrapped_as_a_host_failure() {
    let source = SOURCE.replace(
        "#(\n    around(body),\n    apply(increment, 4),\n    rotate(rotate_value, \"tag\", 8),\n    decide(keep_decision, \"accepted\"),\n    list_total(keep_list),\n    classify(classify_values),\n    inspect_callback(callback_pair),\n    entries(),\n  )",
        "around(panic_callback)",
    );
    let error = execution(&source)
        .run_main(&mut ProfileState::default(), &mut Vec::new())
        .expect_err("nested source panic should stop the outer callback");

    let ExecutionError::Panic(panic) = error else {
        panic!("nested source panic should preserve its source error");
    };
    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(
        panic.message(),
        &PanicMessage::Explicit("callback panic".into()),
    );
    assert_eq!(panic.site().module().as_str(), "callback_provider");
    assert_eq!(panic.site().function().as_str(), "panic_callback");
}
