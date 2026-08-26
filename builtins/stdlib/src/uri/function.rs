mod codec;

use crate::HostFailure;
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
pub(super) fn pop_codeunit(string: EcoString) -> (BigInt, EcoString) {
    let Some(value) = string.chars().next() else {
        return (BigInt::from(0), string);
    };
    let rest = EcoString::from(&string[value.len_utf8()..]);

    (BigInt::from(u32::from(value)), rest)
}

pub(super) fn codeunit_slice(
    string: EcoString,
    from: BigInt,
    length: BigInt,
) -> Result<EcoString, HostFailure> {
    let from = from
        .to_usize()
        .ok_or_else(|| HostFailure::new("URI string slice index is not representable"))?;
    let length = length
        .to_usize()
        .ok_or_else(|| HostFailure::new("URI string slice length is not representable"))?;
    let end = from
        .checked_add(length)
        .ok_or_else(|| HostFailure::new("URI string slice range is not representable"))?;
    let from = scalar_byte_index(&string, from)
        .ok_or_else(|| HostFailure::new("URI string slice starts outside the string"))?;
    let end = scalar_byte_index(&string, end)
        .ok_or_else(|| HostFailure::new("URI string slice ends outside the string"))?;

    Ok(EcoString::from(&string[from..end]))
}

pub(super) fn parse_query(query: EcoString) -> Result<Vec<(EcoString, EcoString)>, ()> {
    codec::parse_query(&query).ok_or(())
}

pub(super) fn percent_encode(value: EcoString) -> EcoString {
    codec::percent_encode(&value)
}

pub(super) fn percent_decode(value: EcoString) -> Result<EcoString, ()> {
    codec::percent_decode(&value).ok_or(())
}

fn scalar_byte_index(string: &str, index: usize) -> Option<usize> {
    if index == string.chars().count() {
        return Some(string.len());
    }

    string.char_indices().nth(index).map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::{codeunit_slice, scalar_byte_index};
    use crate::{ExecutionError, GleamStdlibProfile, GleamStdlibRunState, ValueType};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use geam_core::{HostError, InvariantError};
    use num_bigint::BigInt;

    const URI_DECLARATIONS: &str = r#"
@external(erlang, "gleam_stdlib", "string_pop_codeunit")
fn pop_codeunit(string: String) -> #(Int, String)

@external(erlang, "binary", "part")
fn codeunit_slice(string: String, from: Int, length: Int) -> String

@external(erlang, "gleam_stdlib", "parse_query")
pub fn parse_query(query: String) -> Result(List(#(String, String)), Nil)

@external(erlang, "gleam_stdlib", "percent_encode")
pub fn percent_encode(value: String) -> String

@external(erlang, "gleam_stdlib", "percent_decode")
pub fn percent_decode(value: String) -> Result(String, Nil)
"#;

    const URI_MAIN: &str = r#"
pub fn main() {
  let #(codepoint, rest) = pop_codeunit("ñrest")
  assert codepoint == 241
  assert rest == "rest"

  let #(empty_codepoint, empty_rest) = pop_codeunit("")
  assert empty_codepoint == 0
  assert empty_rest == ""
  assert codeunit_slice("año", 1, 2) == "ño"
  assert parse_query("a+b=1&a+b=2") == Ok([#("a b", "1"), #("a b", "2")])
  assert parse_query("%C2") == Error(Nil)
  assert percent_encode("ñ +") == "%C3%B1%20+"
  assert percent_decode("%C3%B1%20+") == Ok("ñ +")
  assert percent_decode("%C2") == Error(Nil)
}
"#;

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let provider = super::super::host_provider::<GleamStdlibProfile>()
            .expect("synthetic URI provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/uri",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/uri",
                    "src/gleam/uri.gleam",
                    source,
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<GleamStdlibProfile>>::new(),
                [provider],
            )
            .expect("synthetic URI provider module should be unique"),
        )
        .expect("synthetic URI source should compile");
        let plan = plan_host_program(typed).expect("synthetic URI source should plan");
        HostedExecution::try_from_module_plan(plan).expect("synthetic URI source should seal")
    }

    #[test]
    fn slices_uri_strings_by_unicode_scalar_index() {
        assert_eq!(
            codeunit_slice("año".into(), 1.into(), 1.into()),
            Ok("ñ".into())
        );
        assert_eq!(
            codeunit_slice("año".into(), 1.into(), 2.into()),
            Ok("ño".into())
        );
        assert_eq!(
            codeunit_slice("año".into(), 3.into(), 0.into()),
            Ok("".into())
        );
    }

    #[test]
    fn rejects_unrepresentable_or_out_of_bounds_uri_slices() {
        for (from, length, message) in [
            (
                BigInt::from(-1),
                BigInt::from(1),
                "URI string slice index is not representable",
            ),
            (
                BigInt::from(0),
                BigInt::from(-1),
                "URI string slice length is not representable",
            ),
            (
                BigInt::from(usize::MAX),
                BigInt::from(1),
                "URI string slice range is not representable",
            ),
            (
                BigInt::from(4),
                BigInt::from(0),
                "URI string slice starts outside the string",
            ),
            (
                BigInt::from(2),
                BigInt::from(2),
                "URI string slice ends outside the string",
            ),
        ] {
            assert_eq!(
                codeunit_slice("año".into(), from, length)
                    .expect_err("invalid URI string slice should fail")
                    .message(),
                message,
            );
        }

        assert_eq!(scalar_byte_index("año", usize::MAX), None);
    }

    #[test]
    fn executes_every_uri_provider_through_the_typed_hosted_pipeline() {
        let source = format!("{URI_DECLARATIONS}\n{URI_MAIN}");
        let execution = execution(&source);

        assert_eq!(
            execution
                .run_main(
                    &mut GleamStdlibRunState::from_seed([5; 32]),
                    &mut Vec::new(),
                )
                .expect("synthetic URI source should run"),
            Value::Nil,
        );
    }

    #[test]
    fn preserves_invalid_slices_through_the_uri_host_adapter() {
        let source =
            format!("{URI_DECLARATIONS}\npub fn main() {{ codeunit_slice(\"año\", -1, 1) }}");
        let execution = execution(&source);
        let error = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([5; 32]),
                &mut Vec::new(),
            )
            .expect_err("invalid URI slice should fail");
        let error = expect_uri_host_error(error);

        assert_eq!(error.package(), "gleam_stdlib");
        assert_eq!(error.module(), "gleam/uri");
        assert_eq!(error.function(), "codeunit_slice");
        assert_eq!(
            error.failure().message(),
            "URI string slice index is not representable",
        );
    }

    #[test]
    #[should_panic(expected = "invalid URI slice should remain a host failure")]
    fn uri_host_failure_assertion_rejects_other_execution_errors() {
        let _ = expect_uri_host_error(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::String,
                index: 1,
                length: 0,
            },
        ));
    }

    fn expect_uri_host_error(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("invalid URI slice should remain a host failure");
        };
        error
    }
}
