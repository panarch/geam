mod codec;

use super::schema::{
    CodeunitPair, PercentDecodeError, PercentDecodeOk, PercentDecodeResult, QueryConstructions,
    QueryError, QueryOk, QueryPairIndex, QueryPairsIndex, QueryResult,
};
use crate::gleam_stdlib::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions, HostFailure, HostProvider,
};
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::marker::PhantomData;

pub(super) struct UriProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for UriProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

pub(super) fn pop_codeunit<'call, Profile>(
    call: HostCall<'call, Profile, UriProvider<Profile>, CodeunitPair>,
    string: EcoString,
) -> Result<HostCallCompletion<'call, CodeunitPair>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let Some(value) = string.chars().next() else {
        return Ok(call.return_tuple((BigInt::from(0), (string, ()))));
    };
    let rest = EcoString::from(&string[value.len_utf8()..]);

    Ok(call.return_tuple((BigInt::from(u32::from(value)), (rest, ()))))
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

pub(super) fn parse_query<'call, Profile>(
    mut call: HostCall<'call, Profile, UriProvider<Profile>, QueryResult>,
    constructions: HostConstructions<'call, QueryConstructions>,
    query: EcoString,
) -> Result<HostCallCompletion<'call, QueryResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let Some(pairs) = codec::parse_query(&query) else {
        return Ok(call.return_custom::<QueryError>(((), ())));
    };
    let pairs = pairs
        .into_iter()
        .map(|(key, value)| {
            call.construct_tuple(constructions.at::<QueryPairIndex>(), (key, (value, ())))
        })
        .collect::<Vec<_>>();
    let pairs = call.construct_list(constructions.at::<QueryPairsIndex>(), pairs);

    Ok(call.return_custom::<QueryOk>((pairs, ())))
}

pub(super) fn percent_encode(value: EcoString) -> EcoString {
    codec::percent_encode(&value)
}

pub(super) fn percent_decode<'call, Profile>(
    call: HostCall<'call, Profile, UriProvider<Profile>, PercentDecodeResult>,
    value: EcoString,
) -> Result<HostCallCompletion<'call, PercentDecodeResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match codec::percent_decode(&value) {
        Some(value) => call.return_custom::<PercentDecodeOk>((value, ())),
        None => call.return_custom::<PercentDecodeError>(((), ())),
    })
}

fn scalar_byte_index(string: &str, index: usize) -> Option<usize> {
    if index == string.chars().count() {
        return Some(string.len());
    }

    string.char_indices().nth(index).map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::{UriProvider, codeunit_slice, scalar_byte_index};
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        Value, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    const URI_SOURCE: &str = r#"
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

    #[test]
    fn uri_provider_projects_the_caller_owned_run_state() {
        let mut state = GleamStdlibRunState::from_seed([3; 32]);
        let expected = std::ptr::from_mut(&mut state);
        let projected =
            <UriProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

        assert!(std::ptr::eq(projected, expected));
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
                    URI_SOURCE,
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("synthetic URI source should seal");

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
}
