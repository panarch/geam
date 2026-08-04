mod function;
mod schema;

use self::function::{
    StringProvider, byte_size, contains, crop, do_inspect, ends_with, erl_split, erl_trim,
    from_utf_codepoints, grapheme_slice, length, less_than, lowercase, pop_grapheme, remove_prefix,
    remove_suffix, starts_with, unsafe_byte_slice, unsafe_int_to_utf_codepoint, uppercase,
    utf_codepoint_to_int,
};
use self::schema::{
    Direction, InspectValue, PopConstructions, PopResult, StringList, UtfCodepointList,
};
use super::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/string")
        .and_then(|provider| provider.with_function("length", length))
        .and_then(|provider| provider.with_function("lowercase", lowercase))
        .and_then(|provider| provider.with_function("uppercase", uppercase))
        .and_then(|provider| provider.with_function("less_than", less_than))
        .and_then(|provider| {
            provider.with_fallible_function::<(EcoString, BigInt, BigInt), EcoString, _>(
                "grapheme_slice",
                grapheme_slice,
            )
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(EcoString, BigInt, BigInt), EcoString, _>(
                "unsafe_byte_slice",
                unsafe_byte_slice,
            )
        })
        .and_then(|provider| provider.with_function("crop", crop))
        .and_then(|provider| provider.with_function("contains", contains))
        .and_then(|provider| provider.with_function("starts_with", starts_with))
        .and_then(|provider| provider.with_function("ends_with", ends_with))
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringProvider<Profile>,
                (EcoString, EcoString),
                StringList,
                _,
            >("erl_split", erl_split::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringProvider<Profile>,
                (EcoString, Direction),
                EcoString,
                _,
            >("erl_trim", erl_trim::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function_and_constructions::<
                StringProvider<Profile>,
                (EcoString,),
                PopResult,
                PopConstructions,
                _,
            >("pop_grapheme", pop_grapheme::<Profile>)
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(BigInt,), char, _>(
                "unsafe_int_to_utf_codepoint",
                unsafe_int_to_utf_codepoint,
            )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringProvider<Profile>, (UtfCodepointList,), EcoString, _>(
                    "from_utf_codepoints",
                    from_utf_codepoints::<Profile>,
                )
        })
        .and_then(|provider| provider.with_function("utf_codepoint_to_int", utf_codepoint_to_int))
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringProvider<Profile>,
                (InspectValue,),
                super::string_tree::StringTree,
                _,
            >("do_inspect", do_inspect::<Profile>)
        })
        .and_then(|provider| provider.with_function("byte_size", byte_size))
        .and_then(|provider| provider.with_function("remove_prefix", remove_prefix))
        .and_then(|provider| provider.with_function("remove_suffix", remove_suffix))
}

#[cfg(test)]
mod tests {
    use super::function::StringProvider;
    use super::host_provider;
    use crate::gleam_stdlib::string_tree::StringTreeSchema;
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostModule, HostProviderModule, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;

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
        let source = format!(
            r#"{STRING_DECLARATIONS}

pub fn main() {{
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
}}
"#,
        );
        let string_tree =
            HostProviderModule::<GleamStdlibProfile>::new("gleam_stdlib", "gleam/string_tree")
                .and_then(
                    HostProviderModule::with_external_type::<
                        StringProvider<GleamStdlibProfile>,
                        StringTreeSchema,
                    >,
                )
                .expect("synthetic StringTree storage should register");
        let string = host_provider::<GleamStdlibProfile>()
            .expect("official string provider should register");
        let hosts = HostProviderSet::with_providers(
            Vec::<HostModule<GleamStdlibProfile>>::new(),
            [string_tree, string],
        )
        .expect("synthetic string providers should be unique");
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
                        "pub type StringTree",
                    ),
                    ModuleSource::new("gleam/string", "src/gleam/string.gleam", source),
                ],
            )],
            hosts,
        )
        .expect("synthetic string source should compile");
        let plan = plan_host_program(typed).expect("synthetic string source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("string execution should seal");
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
}
