use super::parse::{decimal, format_radix, radix};
use super::schema::{ParseError, ParseOk, ParseResult};
use crate::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{HostCall, HostCallCompletion, HostCallError, HostFailure, HostProvider};
use ecow::EcoString;
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
use std::marker::PhantomData;

pub(super) struct IntProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for IntProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

pub(super) fn parse<'call, Profile>(
    call: HostCall<'call, Profile, IntProvider<Profile>, ParseResult>,
    source: EcoString,
) -> Result<HostCallCompletion<'call, ParseResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match decimal(&source) {
        Some(value) => call.return_custom::<ParseOk>((value, ())),
        None => call.return_custom::<ParseError>(((), ())),
    })
}

pub(super) fn do_base_parse<'call, Profile>(
    call: HostCall<'call, Profile, IntProvider<Profile>, ParseResult>,
    source: EcoString,
    base: BigInt,
) -> Result<HostCallCompletion<'call, ParseResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match radix(&source, &base) {
        Some(value) => call.return_custom::<ParseOk>((value, ())),
        None => call.return_custom::<ParseError>(((), ())),
    })
}

pub(super) fn to_string(value: BigInt) -> EcoString {
    value.to_string().into()
}

pub(super) fn do_to_base_string(value: BigInt, base: BigInt) -> Result<EcoString, HostFailure> {
    format_radix(&value, &base)
        .ok_or_else(|| HostFailure::new("base must be an Int from 2 through 36"))
}

pub(super) fn to_float(value: BigInt) -> Result<f64, HostFailure> {
    super::super::float::do_to_float(value)
}

pub(super) fn bitwise_and(left: BigInt, right: BigInt) -> BigInt {
    left & right
}

pub(super) fn bitwise_not(value: BigInt) -> BigInt {
    !value
}

pub(super) fn bitwise_or(left: BigInt, right: BigInt) -> BigInt {
    left | right
}

pub(super) fn bitwise_exclusive_or(left: BigInt, right: BigInt) -> BigInt {
    left ^ right
}

pub(super) fn bitwise_shift_left(value: BigInt, shift: BigInt) -> Result<BigInt, HostFailure> {
    let reversed = shift.sign() == Sign::Minus;
    let count = shift
        .magnitude()
        .to_usize()
        .ok_or_else(|| HostFailure::new("bit shift count cannot be represented by this host"))?;
    Ok(if reversed {
        value >> count
    } else {
        value << count
    })
}

pub(super) fn bitwise_shift_right(value: BigInt, shift: BigInt) -> Result<BigInt, HostFailure> {
    let reversed = shift.sign() == Sign::Minus;
    let count = shift
        .magnitude()
        .to_usize()
        .ok_or_else(|| HostFailure::new("bit shift count cannot be represented by this host"))?;
    Ok(if reversed {
        value << count
    } else {
        value >> count
    })
}

#[cfg(test)]
mod tests {
    use super::super::host_provider;
    use super::{
        IntProvider, bitwise_and, bitwise_exclusive_or, bitwise_not, bitwise_or,
        bitwise_shift_left, bitwise_shift_right, do_to_base_string, to_float, to_string,
    };
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostFailure, HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    const INT_DECLARATIONS: &str = r#"
@external(erlang, "gleam_stdlib", "parse_int")
pub fn parse(value: String) -> Result(Int, Nil)

@external(erlang, "gleam_stdlib", "int_from_base_string")
pub fn do_base_parse(value: String, base: Int) -> Result(Int, Nil)

@external(erlang, "erlang", "integer_to_binary")
pub fn to_string(value: Int) -> String

@external(erlang, "erlang", "integer_to_binary")
pub fn do_to_base_string(value: Int, base: Int) -> String

@external(erlang, "erlang", "float")
pub fn to_float(value: Int) -> Float

@external(erlang, "erlang", "band")
pub fn bitwise_and(left: Int, right: Int) -> Int

@external(erlang, "erlang", "bnot")
pub fn bitwise_not(value: Int) -> Int

@external(erlang, "erlang", "bor")
pub fn bitwise_or(left: Int, right: Int) -> Int

@external(erlang, "erlang", "bxor")
pub fn bitwise_exclusive_or(left: Int, right: Int) -> Int

@external(erlang, "erlang", "bsl")
pub fn bitwise_shift_left(value: Int, shift: Int) -> Int

@external(erlang, "erlang", "bsr")
pub fn bitwise_shift_right(value: Int, shift: Int) -> Int
"#;

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{INT_DECLARATIONS}\n{source}");
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official int provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/int",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/int",
                    "src/gleam/int.gleam",
                    source,
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<GleamStdlibProfile>>::new(),
                [provider],
            )
            .expect("int provider module should be unique"),
        )
        .expect("synthetic int source should compile");
        let plan = plan_host_program(typed).expect("synthetic int source should plan");
        HostedExecution::try_from_module_plan(plan).expect("synthetic int execution should seal")
    }

    #[test]
    fn projects_the_complete_stdlib_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected =
            <IntProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

        assert!(std::ptr::eq(projected, &state));
    }

    #[test]
    fn implements_integer_formatting_conversion_and_bitwise_operations() {
        let large = BigInt::from(10u8).pow(100);

        assert_eq!(to_string(large.clone()), large.to_string());
        assert_eq!(do_to_base_string(255.into(), 16.into()), Ok("FF".into()));
        assert_eq!(to_float(7.into()), Ok(7.0));
        assert_eq!(bitwise_and(5.into(), 3.into()), BigInt::from(1));
        assert_eq!(bitwise_not(5.into()), BigInt::from(-6));
        assert_eq!(bitwise_or(5.into(), 2.into()), BigInt::from(7));
        assert_eq!(bitwise_exclusive_or(5.into(), 3.into()), BigInt::from(6));
        assert_eq!(bitwise_shift_left(1.into(), 5.into()), Ok(32.into()));
        assert_eq!(bitwise_shift_left(8.into(), (-1).into()), Ok(4.into()));
        assert_eq!(bitwise_shift_right(32.into(), 2.into()), Ok(8.into()));
        assert_eq!(bitwise_shift_right(8.into(), (-1).into()), Ok(16.into()));
    }

    #[test]
    fn executes_every_integer_provider_through_the_hosted_pipeline() {
        let execution = execution(
            r#"
pub fn main() {
  #(
    parse("12"),
    parse("bad"),
    do_base_parse("-FF", 16),
    do_base_parse("2", 2),
    to_string(123),
    do_to_base_string(255, 16),
    to_float(7),
    bitwise_and(5, 3),
    bitwise_not(5),
    bitwise_or(5, 2),
    bitwise_exclusive_or(5, 3),
    bitwise_shift_left(1, 5),
    bitwise_shift_right(32, 2),
  )
}
"#,
        );

        let value = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("integer providers should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"#(Ok(12), Error(Nil), Ok(-255), Error(Nil), "123", "FF", 7.0, 1, -6, 7, 6, 32, 8)"#,
        );
    }

    #[test]
    fn rejects_unrepresentable_conversion_bases_and_shift_counts() {
        let huge = BigInt::from(10u8).pow(1000);

        assert_eq!(
            to_float(huge.clone()),
            Err(HostFailure::new(
                "Int cannot be represented as a finite Float"
            )),
        );
        assert_eq!(
            do_to_base_string(1.into(), huge.clone()),
            Err(HostFailure::new("base must be an Int from 2 through 36")),
        );
        assert_eq!(
            bitwise_shift_left(1.into(), huge.clone()),
            Err(HostFailure::new(
                "bit shift count cannot be represented by this host"
            )),
        );
        assert_eq!(
            bitwise_shift_right(1.into(), -huge),
            Err(HostFailure::new(
                "bit shift count cannot be represented by this host"
            )),
        );
    }
}
