mod function;

use super::{Component, GleamStdlibHostProfile, GleamStdlibRunState};
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use geam_core::provider::{Call, HostResult, Value};
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/string",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
mod provider {
    use super::{BigInt, Call, EcoString, GleamStdlibRunState, HostResult, Value, function};
    use crate::string_tree;

    #[geam_macros::custom(input = DirectionInput)]
    #[allow(dead_code)]
    pub(super) enum Direction {
        Leading,
        Trailing,
    }

    #[geam_macros::function]
    fn length(string: EcoString) -> BigInt {
        function::length(string)
    }

    #[geam_macros::function]
    fn lowercase(string: EcoString) -> EcoString {
        function::lowercase(string)
    }

    #[geam_macros::function]
    fn uppercase(string: EcoString) -> EcoString {
        function::uppercase(string)
    }

    #[geam_macros::function]
    fn less_than(left: EcoString, right: EcoString) -> bool {
        function::less_than(left, right)
    }

    #[geam_macros::function]
    fn grapheme_slice(string: EcoString, index: BigInt, length: BigInt) -> HostResult<EcoString> {
        function::grapheme_slice(string, index, length).map_err(Into::into)
    }

    #[geam_macros::function]
    fn unsafe_byte_slice(
        string: EcoString,
        index: BigInt,
        length: BigInt,
    ) -> HostResult<EcoString> {
        function::unsafe_byte_slice(string, index, length).map_err(Into::into)
    }

    #[geam_macros::function]
    fn crop(string: EcoString, substring: EcoString) -> EcoString {
        function::crop(string, substring)
    }

    #[geam_macros::function]
    fn contains(haystack: EcoString, needle: EcoString) -> bool {
        function::contains(haystack, needle)
    }

    #[geam_macros::function]
    fn starts_with(string: EcoString, prefix: EcoString) -> bool {
        function::starts_with(string, prefix)
    }

    #[geam_macros::function]
    fn ends_with(string: EcoString, suffix: EcoString) -> bool {
        function::ends_with(string, suffix)
    }

    #[geam_macros::function]
    fn erl_split(string: EcoString, pattern: EcoString) -> Vec<EcoString> {
        function::erl_split(string, pattern)
    }

    #[geam_macros::function]
    fn erl_trim(string: EcoString, direction: DirectionInput) -> EcoString {
        function::erl_trim(string, matches!(direction, DirectionInput::Leading))
    }

    #[geam_macros::function]
    fn pop_grapheme(string: EcoString) -> Result<(EcoString, EcoString), ()> {
        function::pop_grapheme(string)
    }

    #[geam_macros::function]
    fn unsafe_int_to_utf_codepoint(value: BigInt) -> HostResult<char> {
        function::unsafe_int_to_utf_codepoint(value).map_err(Into::into)
    }

    #[geam_macros::function]
    fn from_utf_codepoints(values: geam_core::provider::List<char>) -> EcoString {
        let mut string = String::new();
        let mut index = 0;
        while let Some(value) = values.get(index) {
            string.push(value);
            index += 1;
        }
        string.into()
    }

    #[geam_macros::function]
    fn utf_codepoint_to_int(value: char) -> BigInt {
        function::utf_codepoint_to_int(value)
    }

    #[geam_macros::function(profile = Profile)]
    fn do_inspect<Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: Value<Item>,
    ) -> string_tree::StringTreePayload {
        function::do_inspect(call.inspect(&value))
    }

    #[geam_macros::function]
    fn byte_size(string: EcoString) -> BigInt {
        function::byte_size(string)
    }

    #[geam_macros::function]
    fn remove_prefix(string: EcoString, prefix: EcoString) -> EcoString {
        function::remove_prefix(string, prefix)
    }

    #[geam_macros::function]
    fn remove_suffix(string: EcoString, suffix: EcoString) -> EcoString {
        function::remove_suffix(string, suffix)
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::string_tree::{self, STRING_TREE_DECLARATIONS};
    use crate::{
        ExecutionError, HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        ValueType, compile_typed_host_program, plan_host_program,
    };
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use ecow::EcoString;
    use geam_core::{HostError, InvariantError};

    const STRING_DECLARATIONS: &str = r#"
import gleam/string_tree.{type StringTree}

type Direction {
  Leading
  Trailing
}

pub type Person {
  Person(name: String)
}

@external(erlang, "host", "length")
fn length(string: String) -> Int

@external(erlang, "host", "lowercase")
fn lowercase(string: String) -> String

@external(erlang, "host", "uppercase")
fn uppercase(string: String) -> String

@external(erlang, "host", "less_than")
fn less_than(left: String, right: String) -> Bool

@external(erlang, "host", "grapheme_slice")
fn grapheme_slice(string: String, index: Int, length: Int) -> String

@external(erlang, "host", "unsafe_byte_slice")
fn unsafe_byte_slice(string: String, index: Int, length: Int) -> String

@external(erlang, "host", "crop")
fn crop(string: String, substring: String) -> String

@external(erlang, "host", "contains")
fn contains(haystack: String, needle: String) -> Bool

@external(erlang, "host", "starts_with")
fn starts_with(string: String, prefix: String) -> Bool

@external(erlang, "host", "ends_with")
fn ends_with(string: String, suffix: String) -> Bool

@external(erlang, "host", "erl_split")
fn erl_split(string: String, pattern: String) -> List(String)

@external(erlang, "host", "erl_trim")
fn erl_trim(string: String, direction: Direction) -> String

@external(erlang, "host", "pop_grapheme")
fn pop_grapheme(string: String) -> Result(#(String, String), Nil)

@external(erlang, "host", "unsafe_int_to_utf_codepoint")
fn unsafe_int_to_utf_codepoint(value: Int) -> UtfCodepoint

@external(erlang, "host", "from_utf_codepoints")
fn from_utf_codepoints(values: List(UtfCodepoint)) -> String

@external(erlang, "host", "utf_codepoint_to_int")
fn utf_codepoint_to_int(value: UtfCodepoint) -> Int

@external(erlang, "host", "do_inspect")
fn do_inspect(value: value) -> StringTree

@external(erlang, "host", "byte_size")
fn byte_size(string: String) -> Int

@external(erlang, "host", "remove_prefix")
fn remove_prefix(string: String, prefix: String) -> String

@external(erlang, "host", "remove_suffix")
fn remove_suffix(string: String, suffix: String) -> String
"#;

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{STRING_DECLARATIONS}\n{source}");
        let string_tree = string_tree::host_provider::<GleamStdlibProfile>()
            .expect("official string tree provider should register");
        let string = host_provider::<GleamStdlibProfile>()
            .expect("official string provider should register");
        let hosts = HostProviderSet::with_providers(
            Vec::<HostModule<GleamStdlibProfile>>::new(),
            [string_tree, string],
        )
        .expect("string providers should be unique");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/string",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [
                    ModuleSource::new(
                        "gleam/string_tree",
                        "src/gleam/string_tree.gleam",
                        STRING_TREE_DECLARATIONS,
                    ),
                    ModuleSource::new("gleam/string", "src/gleam/string.gleam", source),
                ],
            )],
            hosts,
        )
        .expect("synthetic string source should compile");
        let plan = plan_host_program(typed).expect("synthetic string source should plan");
        HostedExecution::try_from_module_plan(plan).expect("string execution should seal")
    }

    #[test]
    fn registers_the_exact_official_string_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official string provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/string");
        assert_eq!(provider.external_types().count(), 0);
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "length",
                "lowercase",
                "uppercase",
                "less_than",
                "grapheme_slice",
                "unsafe_byte_slice",
                "crop",
                "contains",
                "starts_with",
                "ends_with",
                "erl_split",
                "erl_trim",
                "pop_grapheme",
                "unsafe_int_to_utf_codepoint",
                "from_utf_codepoints",
                "utf_codepoint_to_int",
                "do_inspect",
                "byte_size",
                "remove_prefix",
                "remove_suffix",
            ],
        );
    }

    #[test]
    fn executes_every_string_provider_through_the_hosted_pipeline() {
        let execution = execution(
            r#"
pub fn main() {
  assert length("A👍🏽é") == 3
  assert lowercase("Gleam İ") == "gleam i̇"
  assert uppercase("Gleam ß") == "GLEAM SS"
  assert less_than("A", "B")
  assert grapheme_slice("A👍🏽é", 1, 1) == "👍🏽"
  assert unsafe_byte_slice("a👍b", 1, 4) == "👍"
  assert crop("The Lone Gunmen", "Lone") == "Lone Gunmen"
  assert contains("theory", "ory")
  assert starts_with("theory", "the")
  assert ends_with("theory", "ory")
  assert erl_split("a,b,c", ",") == ["a", "b,c"]
  assert erl_split("abc", "") == ["abc"]
  assert erl_trim("  hats", Leading) == "hats"
  assert erl_trim("hats  ", Trailing) == "hats"
  assert pop_grapheme("👍🏽rest") == Ok(#("👍🏽", "rest"))
  assert pop_grapheme("") == Error(Nil)
  let codepoint = unsafe_int_to_utf_codepoint(65)
  assert from_utf_codepoints([codepoint]) == "A"
  assert utf_codepoint_to_int(codepoint) == 65
  assert byte_size("👍") == 4
  assert remove_prefix("@lpil", "@") == "lpil"
  assert remove_suffix("Hello!", "!") == "Hello"
  let inspected = do_inspect(#(1, "one"))
  let _ = do_inspect(Person(name: "Kim"))
  inspected
}
"#,
        );
        let value = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("string providers should run");

        assert_eq!(
            value.inspect().to_string(),
            r##"string_tree.from_string("#(1, \"one\")")"##,
        );
    }

    #[test]
    fn preserves_host_failures_through_the_generated_string_adapters() {
        let cases = [
            (
                r#"pub fn main() { grapheme_slice("abc", -1, 1) }"#,
                "grapheme_slice",
                "string grapheme slice requires non-negative bounds",
            ),
            (
                r#"pub fn main() { unsafe_byte_slice("👍", 1, 1) }"#,
                "unsafe_byte_slice",
                "string byte slice is outside UTF-8 boundaries",
            ),
            (
                r#"pub fn main() { unsafe_int_to_utf_codepoint(-1) }"#,
                "unsafe_int_to_utf_codepoint",
                "integer is not a valid Unicode codepoint",
            ),
        ];

        for (source, function, reason) in cases {
            let error = execution(source)
                .run_main(
                    &mut GleamStdlibRunState::from_seed([0; 32]),
                    &mut Vec::new(),
                )
                .expect_err("invalid string input should fail");
            let error = expect_string_host_error(error);

            assert_eq!(error.package(), "gleam_stdlib");
            assert_eq!(error.module(), "gleam/string");
            assert_eq!(error.function(), function);
            assert_eq!(error.failure().message(), reason);
        }
    }

    #[test]
    #[should_panic(expected = "invalid string input should remain a host failure")]
    fn string_host_failure_assertion_rejects_other_execution_errors() {
        let _ = expect_string_host_error(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::String,
                index: 1,
                length: 0,
            },
        ));
    }

    fn expect_string_host_error(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("invalid string input should remain a host failure");
        };
        error
    }
}
