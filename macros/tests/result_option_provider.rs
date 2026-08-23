use ecow::EcoString;
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, PlanError, Value, ValueType, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(
    package = "prelude_values",
    modules = [declarations, prelude_values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "declarations", crate_path = geam_core)]
mod declarations {
    use super::EcoString;

    #[geam_macros::external(name = "SharedToken")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct SharedToken(pub(super) EcoString);

    #[geam_macros::custom(input = SharedProblemInput)]
    pub enum SharedProblem {
        Missing,
        Label(EcoString),
    }

    #[geam_macros::function]
    fn shared_token(value: EcoString) -> SharedToken {
        SharedToken(value)
    }

    #[geam_macros::function]
    fn shared_problem(value: EcoString) -> SharedProblem {
        if value.is_empty() {
            SharedProblem::Missing
        } else {
            SharedProblem::Label(value)
        }
    }
}

#[geam_macros::module(path = "prelude_values", crate_path = geam_core)]
mod prelude_values {
    use super::{BigInt, EcoString, declarations};

    #[geam_macros::external(name = "Token")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Token(EcoString);

    #[geam_macros::custom(input = ParseErrorInput)]
    enum ParseError {
        Empty,
        Invalid(EcoString),
    }

    #[geam_macros::custom(input = OutcomeInput)]
    enum Outcome {
        Parsed(Result<BigInt, ParseError>),
        Optional(Option<(EcoString, BigInt)>),
    }

    #[geam_macros::function]
    fn parse(value: EcoString) -> Result<BigInt, ParseError> {
        if value.is_empty() {
            Err(ParseError::Empty)
        } else {
            value
                .parse::<i64>()
                .map(BigInt::from)
                .map_err(|_| ParseError::Invalid(value))
        }
    }

    #[geam_macros::function]
    fn result_text(value: Result<BigInt, ParseErrorInput>) -> EcoString {
        match value {
            Ok(value) => format!("ok:{value}").into(),
            Err(ParseErrorInput::Empty) => "error:empty".into(),
            Err(ParseErrorInput::Invalid(value)) => format!("error:{value}").into(),
        }
    }

    #[geam_macros::function]
    fn optional(value: BigInt, keep: bool) -> Option<(EcoString, BigInt)> {
        keep.then(|| ("kept".into(), value))
    }

    #[geam_macros::function]
    fn option_text(value: Option<(EcoString, BigInt)>) -> EcoString {
        value.map_or_else(
            || "none".into(),
            |(label, value)| format!("some:{label}:{value}").into(),
        )
    }

    #[geam_macros::function]
    fn token(value: EcoString) -> Result<Token, ParseError> {
        if value.is_empty() {
            Err(ParseError::Empty)
        } else {
            Ok(Token(value))
        }
    }

    #[geam_macros::function]
    fn token_text(value: Result<&Token, ParseErrorInput>) -> EcoString {
        match value {
            Ok(value) => format!("token:{}", value.0).into(),
            Err(ParseErrorInput::Empty) => "token:empty".into(),
            Err(ParseErrorInput::Invalid(value)) => format!("token:error:{value}").into(),
        }
    }

    #[geam_macros::function]
    fn declared_problem_text(value: Option<declarations::SharedProblemInput>) -> EcoString {
        match value {
            Some(declarations::SharedProblemInput::Missing) => "declared:missing".into(),
            Some(declarations::SharedProblemInput::Label(value)) => {
                format!("declared:{value}").into()
            }
            None => "declared:none".into(),
        }
    }

    #[geam_macros::function]
    fn declared_token_text(value: Option<&declarations::SharedToken>) -> EcoString {
        value.map_or_else(
            || "shared:none".into(),
            |value| format!("shared:{}", value.0).into(),
        )
    }

    #[geam_macros::function]
    fn parsed(value: Result<BigInt, ParseErrorInput>) -> Outcome {
        Outcome::Parsed(value.map_err(|error| match error {
            ParseErrorInput::Empty => ParseError::Empty,
            ParseErrorInput::Invalid(value) => ParseError::Invalid(value),
        }))
    }

    #[geam_macros::function]
    fn optional_outcome(value: Option<(EcoString, BigInt)>) -> Outcome {
        Outcome::Optional(value)
    }

    #[geam_macros::function]
    fn outcome_text(value: OutcomeInput) -> EcoString {
        match value {
            OutcomeInput::Parsed(Ok(value)) => format!("parsed:{value}").into(),
            OutcomeInput::Parsed(Err(ParseErrorInput::Empty)) => "parsed:empty".into(),
            OutcomeInput::Parsed(Err(ParseErrorInput::Invalid(value))) => {
                format!("parsed:error:{value}").into()
            }
            OutcomeInput::Optional(Some((label, value))) => {
                format!("optional:{label}:{value}").into()
            }
            OutcomeInput::Optional(None) => "optional:none".into(),
        }
    }

    #[geam_macros::function]
    fn result_items(values: geam_core::List<Result<BigInt, ParseErrorInput>>) -> EcoString {
        match values.get(0) {
            Some(Ok(value)) => format!("first:{value}").into(),
            Some(Err(ParseErrorInput::Empty)) => "first:empty".into(),
            Some(Err(ParseErrorInput::Invalid(value))) => format!("first:error:{value}").into(),
            None => "first:none".into(),
        }
    }

    #[geam_macros::function]
    fn results() -> Vec<Result<BigInt, ParseError>> {
        vec![Ok(3.into()), Err(ParseError::Empty)]
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
import declarations
import gleam/option.{type Option, None, Some}

@external(erlang, "prelude_values", "Token")
pub type Token

pub type ParseError {
  Empty
  Invalid(String)
}

pub type Outcome {
  Parsed(Result(Int, ParseError))
  Optional(Option(#(String, Int)))
}

@external(erlang, "prelude_values", "parse")
fn parse(value: String) -> Result(Int, ParseError)

@external(erlang, "prelude_values", "result_text")
fn result_text(value: Result(Int, ParseError)) -> String

@external(erlang, "prelude_values", "optional")
fn optional(value: Int, keep: Bool) -> Option(#(String, Int))

@external(erlang, "prelude_values", "option_text")
fn option_text(value: Option(#(String, Int))) -> String

@external(erlang, "prelude_values", "token")
fn token(value: String) -> Result(Token, ParseError)

@external(erlang, "prelude_values", "token_text")
fn token_text(value: Result(Token, ParseError)) -> String

@external(erlang, "prelude_values", "declared_problem_text")
fn declared_problem_text(value: Option(declarations.SharedProblem)) -> String

@external(erlang, "prelude_values", "declared_token_text")
fn declared_token_text(value: Option(declarations.SharedToken)) -> String

@external(erlang, "prelude_values", "parsed")
fn parsed(value: Result(Int, ParseError)) -> Outcome

@external(erlang, "prelude_values", "outcome_text")
fn outcome_text(value: Outcome) -> String

@external(erlang, "prelude_values", "optional_outcome")
fn optional_outcome(value: Option(#(String, Int))) -> Outcome

@external(erlang, "prelude_values", "result_items")
fn result_items(values: List(Result(Int, ParseError))) -> String

@external(erlang, "prelude_values", "results")
fn results() -> List(Result(Int, ParseError))

pub fn main() {
  assert result_text(parse("12")) == "ok:12"
  assert result_text(parse("")) == "error:empty"
  assert result_text(parse("bad")) == "error:bad"
  assert option_text(optional(7, True)) == "some:kept:7"
  assert option_text(optional(7, False)) == "none"
  assert token_text(token("blue")) == "token:blue"
  assert token_text(token("")) == "token:empty"
  assert declared_problem_text(Some(declarations.shared_problem("blue"))) == "declared:blue"
  assert declared_problem_text(Some(declarations.shared_problem(""))) == "declared:missing"
  assert declared_problem_text(None) == "declared:none"
  assert declared_token_text(Some(declarations.shared_token("green"))) == "shared:green"
  assert declared_token_text(None) == "shared:none"
  assert outcome_text(parsed(parse("9"))) == "parsed:9"
  assert outcome_text(optional_outcome(optional(5, True))) == "optional:kept:5"
  assert result_items([]) == "first:none"
  assert result_items(results()) == "first:3"
  True
}
"#;

const DECLARATIONS_SOURCE: &str = r#"
@external(erlang, "prelude_values", "SharedToken")
pub type SharedToken

pub type SharedProblem {
  Missing
  Label(String)
}

@external(erlang, "prelude_values", "shared_token")
pub fn shared_token(value: String) -> SharedToken

@external(erlang, "prelude_values", "shared_problem")
pub fn shared_problem(value: String) -> SharedProblem
"#;

const OPTION_SOURCE: &str = r#"
pub type Option(value) {
  Some(value)
  None
}
"#;

fn execution(source: &str) -> Result<HostedExecution<Profile>, PlanError> {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored prelude component should register");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("macro-authored module should be unique");
    let typed = compile_typed_host_program(
        "prelude_values",
        "prelude_values",
        [
            PackageSource::new(
                "prelude_values",
                ["gleam_stdlib"],
                [
                    ModuleSource::new(
                        "declarations",
                        "src/declarations.gleam",
                        DECLARATIONS_SOURCE,
                    ),
                    ModuleSource::new("prelude_values", "src/prelude_values.gleam", source),
                ],
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
    .expect("complete Result and Option source should compile");
    let plan = plan_host_program(typed)?;
    Ok(HostedExecution::try_from_module_plan(plan)
        .expect("matching Result and Option providers should seal"))
}

#[test]
fn source_result_and_option_values_link_and_execute_through_generated_codecs() {
    let execution = execution(SOURCE).expect("matching Result and Option providers should plan");

    assert_eq!(
        execution.run_main(&mut ProfileState { component: () }, &mut Vec::new()),
        Ok(Value::Bool(true)),
    );
}

#[test]
fn result_payload_mismatch_remains_a_structured_link_error() {
    let mismatched = SOURCE
        .replace(
            "fn result_text(value: Result(Int, ParseError)) -> String",
            "fn result_text(value: Result(String, ParseError)) -> String",
        )
        .replace(
            "assert result_text(parse(\"12\")) == \"ok:12\"",
            "assert result_text(Ok(\"12\")) == \"ok:12\"",
        )
        .replace(
            "assert result_text(parse(\"\")) == \"error:empty\"",
            "assert result_text(Error(Empty)) == \"error:empty\"",
        )
        .replace(
            "assert result_text(parse(\"bad\")) == \"error:bad\"",
            "assert result_text(Error(Invalid(\"bad\"))) == \"error:bad\"",
        );
    let error = match execution(&mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched Result payload should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("Result mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "prelude_values");
    assert_eq!(module.as_str(), "prelude_values");
    assert_eq!(function.as_str(), "result_text");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("Result linkage error should preserve the exact scheme mismatch");
    };
    assert!(expected_scheme.parameters().is_empty());
    assert!(actual_scheme.parameters().is_empty());

    let [ValueType::Custom(expected_result)] = expected_type.argument_types() else {
        panic!("source argument should remain the prelude Result type");
    };
    assert_eq!(expected_result.type_name().package().as_str(), "");
    assert_eq!(expected_result.type_name().module().as_str(), "gleam");
    assert_eq!(expected_result.type_name().name().as_str(), "Result");
    assert_eq!(expected_result.arguments()[0], ValueType::String);

    let [ValueType::Custom(actual_result)] = actual_type.argument_types() else {
        panic!("provider argument should remain the prelude Result type");
    };
    assert_eq!(actual_result.type_name(), expected_result.type_name());
    assert_eq!(actual_result.arguments()[0], ValueType::Int);
    assert_eq!(actual_result.arguments()[1], expected_result.arguments()[1]);
    assert_eq!(expected_type.return_(), &ValueType::String);
    assert_eq!(actual_type.return_(), &ValueType::String);
}
