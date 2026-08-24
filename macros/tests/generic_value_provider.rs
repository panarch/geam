use ecow::EcoString;
use geam_core::provider::{Call, Value};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ListValue, ModuleSource,
    PackageSource, PlanError, Value as RuntimeValue, ValueType, compile_typed_host_program,
    plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(
    package = "generic_values",
    modules = [generic_values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "generic_values", crate_path = geam_core)]
mod generic_values {
    use super::{BigInt, Call, EcoString, Value};

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    struct Token(EcoString);

    #[geam_macros::custom(input = ProblemInput)]
    #[allow(dead_code)]
    enum Problem {
        Missing,
        Label(EcoString),
    }

    #[geam_macros::function]
    fn identity<Item>(value: Value<Item>) -> Value<Item> {
        value
    }

    #[geam_macros::function]
    fn second<First, Second>(_first: Value<First>, second: Value<Second>) -> Value<Second> {
        second
    }

    #[geam_macros::function]
    fn compound<Item, Other>(
        value: Value<(Item, geam_core::List<Other>)>,
    ) -> Value<(Item, geam_core::List<Other>)> {
        value
    }

    #[geam_macros::function]
    fn optional<Item>(value: Value<Option<Item>>) -> Value<Option<Item>> {
        value
    }

    #[geam_macros::function]
    fn problem<Item>(value: Value<(ProblemInput, Item)>) -> Value<(ProblemInput, Item)> {
        value
    }

    #[geam_macros::function]
    fn problem_text(problem: ProblemInput) -> EcoString {
        match problem {
            ProblemInput::Missing => "missing".into(),
            ProblemInput::Label(label) => label,
        }
    }

    #[geam_macros::function]
    fn outcome<Item>(
        value: Value<Result<Item, ProblemInput>>,
    ) -> Value<Result<Item, ProblemInput>> {
        value
    }

    #[geam_macros::function]
    fn token(label: EcoString) -> Token {
        Token(label)
    }

    #[geam_macros::function]
    fn tokenized<Item>(value: Value<(Token, Item)>) -> Value<(Token, Item)> {
        value
    }

    #[geam_macros::function]
    fn pass_function<Item>(callback: Value<fn(Item) -> Item>) -> Value<fn(Item) -> Item> {
        callback
    }

    #[geam_macros::function]
    fn semantics<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        left: Value<Item>,
        right: Value<Item>,
    ) -> (bool, bool, EcoString) {
        let equal = call.equal(&left, &right);
        let same_hash = call.source_hash(&left) == call.source_hash(&right);
        let inspected = call.inspect(&left);
        (equal, same_hash, inspected)
    }

    #[geam_macros::function]
    fn hash<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> BigInt {
        call.source_hash(&value).into()
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

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
import gleam/option.{type Option, Some}

@external(erlang, "generic_values", "identity")
fn identity(value: item) -> item

@external(erlang, "generic_values", "second")
fn second(first: first, second: second) -> second

@external(erlang, "generic_values", "compound")
fn compound(value: #(item, List(other))) -> #(item, List(other))

@external(erlang, "generic_values", "optional")
fn optional(value: Option(item)) -> Option(item)

pub type Problem {
  Missing
  Label(String)
}

@external(erlang, "generic_values", "Token")
pub type Token

@external(erlang, "generic_values", "problem")
fn problem(value: #(Problem, item)) -> #(Problem, item)

@external(erlang, "generic_values", "problem_text")
fn problem_text(problem: Problem) -> String

@external(erlang, "generic_values", "outcome")
fn outcome(value: Result(item, Problem)) -> Result(item, Problem)

@external(erlang, "generic_values", "token")
fn token(label: String) -> Token

@external(erlang, "generic_values", "tokenized")
fn tokenized(value: #(Token, item)) -> #(Token, item)

@external(erlang, "generic_values", "pass_function")
fn pass_function(callback: fn(item) -> item) -> fn(item) -> item

@external(erlang, "generic_values", "semantics")
fn semantics(left: item, right: item) -> #(Bool, Bool, String)

@external(erlang, "generic_values", "hash")
fn hash(value: item) -> Int

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let same = semantics(#("alpha", [1, 2]), #("alpha", [1, 2]))
  let different = semantics(#("alpha", [1]), #("beta", [2]))
  let callback = pass_function(increment)
  let problem_ok = case problem(#(Label("bad"), 9)) {
    #(Label(label), value) ->
      label == "bad" && value == 9 && problem_text(Label(label)) == "bad"
    _ -> False
  }
  let outcome_ok = case outcome(Ok("kept")) {
    Ok(value) -> value == "kept"
    Error(_) -> False
  }
  let source_token = token("blue")
  let #(passed_token, token_value) = tokenized(#(source_token, True))
  #(
    identity(7),
    second("discarded", True),
    compound(#("kept", [1, 2, 3])),
    optional(Some("present")),
    callback(4),
    same,
    different.0,
    hash([1, 2]) == hash([1, 2]),
    problem_ok,
    outcome_ok,
    source_token == passed_token && token_value,
  )
}
"#;

const OPTION_SOURCE: &str = r#"
pub type Option(value) {
  Some(value)
  None
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored generic component should register")
}

fn execution(source: &str) -> Result<HostedExecution<Profile>, PlanError> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored generic module should be unique");
    let typed = compile_typed_host_program(
        "generic_values",
        "generic_values",
        [
            PackageSource::new(
                "generic_values",
                ["gleam_stdlib"],
                [ModuleSource::new(
                    "generic_values",
                    "src/generic_values.gleam",
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
    .expect("complete generic provider source should compile");
    let plan = plan_host_program(typed)?;
    Ok(HostedExecution::try_from_module_plan(plan).expect("matching generic provider should seal"))
}

#[test]
fn generic_schema_uses_return_first_parameter_order_and_recursive_shapes() {
    let providers = providers();
    let functions = providers[0].functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        [
            "identity",
            "second",
            "compound",
            "optional",
            "problem",
            "problem_text",
            "outcome",
            "token",
            "tokenized",
            "pass_function",
            "semantics",
            "hash",
        ],
    );

    let identity = functions[0].type_();
    let [ValueType::Parameter(identity_argument)] = identity.argument_types() else {
        panic!("identity argument should be generic");
    };
    let ValueType::Parameter(identity_return) = identity.return_() else {
        panic!("identity return should be generic");
    };
    assert_eq!(identity_argument, identity_return);

    let second = functions[1].type_();
    let [
        ValueType::Parameter(first),
        ValueType::Parameter(second_argument),
    ] = second.argument_types()
    else {
        panic!("second should expose two generic arguments");
    };
    let ValueType::Parameter(second_return) = second.return_() else {
        panic!("second return should be generic");
    };
    assert_ne!(first, second_argument);
    assert_eq!(second_argument, second_return);

    let compound = functions[2].type_();
    assert_eq!(compound.argument_types(), [compound.return_().clone()]);
    let ValueType::Tuple(elements) = compound.return_() else {
        panic!("compound should preserve its tuple shape");
    };
    assert!(matches!(elements[0], ValueType::Parameter(_)));
    let ValueType::List(item) = &elements[1] else {
        panic!("compound should preserve its List shape");
    };
    assert!(matches!(item.as_ref(), ValueType::Parameter(_)));
}

#[test]
fn generic_values_pass_through_every_runtime_family_without_reconstruction() {
    let execution = execution(SOURCE).expect("matching generic provider should plan");
    let returned = execution
        .run_main(&mut ProfileState { component: () }, &mut Vec::new())
        .expect("generic provider should execute");
    let RuntimeValue::Tuple(values) = returned else {
        panic!("main should return the complete generic result");
    };
    assert_eq!(values[0], RuntimeValue::Int(7.into()));
    assert_eq!(values[1], RuntimeValue::Bool(true));
    assert_eq!(
        values[2],
        RuntimeValue::Tuple(vec![
            RuntimeValue::String("kept".into()),
            RuntimeValue::List(ListValue::int(vec![1.into(), 2.into(), 3.into()])),
        ]),
    );
    assert!(matches!(values[3], RuntimeValue::Custom(_)));
    assert_eq!(values[4], RuntimeValue::Int(5.into()));
    let RuntimeValue::Tuple(same) = &values[5] else {
        panic!("source semantics should return one tuple");
    };
    assert_eq!(same[0], RuntimeValue::Bool(true));
    assert_eq!(same[1], RuntimeValue::Bool(true));
    assert!(matches!(&same[2], RuntimeValue::String(value) if value.starts_with("#(")));
    assert_eq!(values[6], RuntimeValue::Bool(false));
    assert_eq!(values[7], RuntimeValue::Bool(true));
    assert_eq!(values[8], RuntimeValue::Bool(true));
    assert_eq!(values[9], RuntimeValue::Bool(true));
    assert_eq!(values[10], RuntimeValue::Bool(true));
}

#[test]
fn generic_scheme_mismatch_remains_a_structured_link_error() {
    let mismatched = SOURCE.replace(
        "fn identity(value: item) -> item",
        "fn identity(value: item) -> Bool",
    );
    let error = match execution(&mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched generic return should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("generic mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "generic_values");
    assert_eq!(module.as_str(), "generic_values");
    assert_eq!(function.as_str(), "identity");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("generic mismatch should preserve the exact schemes");
    };
    assert_eq!(expected_scheme.parameters().len(), 1);
    assert_eq!(actual_scheme.parameters().len(), 1);
    assert!(matches!(expected_type.return_(), ValueType::Bool));
    assert!(matches!(actual_type.return_(), ValueType::Parameter(_)));
}
