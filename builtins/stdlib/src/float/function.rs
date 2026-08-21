use super::parse::{format, parse_literal};
use super::schema::{ParseError, ParseOk, ParseResult};
use crate::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{HostCall, HostCallCompletion, HostCallError, HostFailure, HostProvider};
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};
use std::marker::PhantomData;

pub(super) struct FloatProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for FloatProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

pub(super) fn parse<'call, Profile>(
    call: HostCall<'call, Profile, FloatProvider<Profile>, ParseResult>,
    source: EcoString,
) -> Result<HostCallCompletion<'call, ParseResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(match parse_literal(&source) {
        Some(value) => call.return_custom::<ParseOk>((value, ())),
        None => call.return_custom::<ParseError>(((), ())),
    })
}

pub(super) fn to_string(value: f64) -> EcoString {
    format(value)
}

pub(super) fn ceiling(value: f64) -> f64 {
    value.ceil()
}

pub(super) fn floor(value: f64) -> f64 {
    value.floor()
}

pub(super) fn js_round(value: f64) -> Result<BigInt, HostFailure> {
    BigInt::from_f64((value + 0.5).floor())
        .ok_or_else(|| HostFailure::new("float cannot be represented as an Int"))
}

pub(super) fn truncate(value: f64) -> Result<BigInt, HostFailure> {
    BigInt::from_f64(value.trunc())
        .ok_or_else(|| HostFailure::new("float cannot be represented as an Int"))
}

pub(crate) fn do_to_float(value: BigInt) -> Result<f64, HostFailure> {
    value
        .to_f64()
        .filter(|value| value.is_finite())
        .ok_or_else(|| HostFailure::new("Int cannot be represented as a finite Float"))
}

pub(super) fn do_power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

pub(super) fn random<'call, Profile>(
    mut call: HostCall<'call, Profile, FloatProvider<Profile>, f64>,
) -> Result<HostCallCompletion<'call, f64>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = call.state().random_float();
    Ok(call.return_value(value))
}

pub(super) fn do_log(value: f64) -> f64 {
    value.ln()
}

pub(super) fn exponential(value: f64) -> f64 {
    value.exp()
}

#[cfg(test)]
mod tests {
    use super::super::host_provider;
    use super::{
        FloatProvider, ceiling, do_log, do_power, do_to_float, exponential, floor, js_round,
        to_string, truncate,
    };
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    const FLOAT_DECLARATIONS: &str = r#"
@external(erlang, "gleam_stdlib", "parse_float")
pub fn parse(string: String) -> Result(Float, Nil)

@external(erlang, "gleam_stdlib", "float_to_string")
pub fn to_string(value: Float) -> String

@external(erlang, "math", "ceil")
pub fn ceiling(value: Float) -> Float

@external(erlang, "math", "floor")
pub fn floor(value: Float) -> Float

@external(erlang, "gleam_stdlib", "round")
@external(javascript, "../gleam_stdlib.mjs", "round")
pub fn js_round(value: Float) -> Int

@external(erlang, "erlang", "trunc")
pub fn truncate(value: Float) -> Int

@external(erlang, "erlang", "float")
pub fn do_to_float(value: Int) -> Float

@external(erlang, "math", "pow")
pub fn do_power(base: Float, exponent: Float) -> Float

@external(erlang, "rand", "uniform")
pub fn random() -> Float

@external(erlang, "math", "log")
pub fn do_log(value: Float) -> Float

@external(erlang, "math", "exp")
pub fn exponential(value: Float) -> Float
"#;

    fn execution(
        source: &str,
        modules: impl IntoIterator<Item = HostModule<GleamStdlibProfile>>,
    ) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{FLOAT_DECLARATIONS}\n{source}");
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official float provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/float",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/float",
                    "src/gleam/float.gleam",
                    source,
                )],
            )],
            HostProviderSet::with_providers(modules, [provider])
                .expect("float provider module should be unique"),
        )
        .expect("synthetic float source should compile");
        let plan = plan_host_program(typed).expect("synthetic float source should plan");
        HostedExecution::try_from_module_plan(plan).expect("synthetic float execution should seal")
    }

    #[test]
    fn projects_only_the_stdlib_random_state() {
        let mut state = GleamStdlibRunState::from_seed([3; 32]);
        let projected =
            <FloatProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

        assert!(std::ptr::eq(projected, &state));
    }

    #[test]
    fn implements_float_math_and_checked_conversions() {
        assert_eq!(to_string(2.0), "2.0");
        assert_eq!(ceiling(2.3), 3.0);
        assert_eq!(floor(2.7), 2.0);
        assert_eq!(js_round(2.5), Ok(BigInt::from(3)));
        assert_eq!(truncate(-2.9), Ok(BigInt::from(-2)));
        assert_eq!(do_to_float(BigInt::from(7)), Ok(7.0));
        assert_eq!(do_power(2.0, 3.0), 8.0);
        assert_eq!(do_log(std::f64::consts::E), 1.0);
        assert_eq!(exponential(1.0), std::f64::consts::E);
    }

    #[test]
    fn rejects_non_finite_or_unrepresentable_conversions() {
        assert_eq!(
            js_round(f64::NAN)
                .expect_err("NaN should not convert")
                .to_string(),
            "float cannot be represented as an Int",
        );
        assert_eq!(
            truncate(f64::INFINITY)
                .expect_err("infinity should not convert")
                .to_string(),
            "float cannot be represented as an Int",
        );
        assert_eq!(
            do_to_float(BigInt::from(10u8).pow(1000))
                .expect_err("an overflowing Int should not convert")
                .to_string(),
            "Int cannot be represented as a finite Float",
        );
    }

    #[test]
    fn executes_every_float_provider_with_reproducible_caller_state() {
        let source = r#"
pub fn main() {
  #(
    parse("2.5"),
    parse("2"),
    to_string(2.0),
    ceiling(2.3),
    floor(2.7),
    js_round(2.5),
    truncate(-2.9),
    do_to_float(7),
    do_power(2.0, 3.0),
    random(),
    do_log(1.0),
    exponential(0.0),
  )
}
"#;
        let execution = execution(source, Vec::<HostModule<GleamStdlibProfile>>::new());
        let mut expected_state = GleamStdlibRunState::from_seed([5; 32]);
        let expected_random = expected_state.random_float();
        let expected = format!(
            r#"#(Ok(2.5), Error(Nil), "2.0", 3.0, 2.0, 3, -2, 7.0, 8.0, {expected_random:?}, 0.0, 1.0)"#,
        );
        let mut first_state = GleamStdlibRunState::from_seed([5; 32]);
        let mut second_state = GleamStdlibRunState::from_seed([5; 32]);

        let first = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("float providers should run");
        let second = execution
            .run_main(&mut second_state, &mut Vec::new())
            .expect("the same seed should reproduce float providers");
        let advanced = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("reusing state should advance its random stream");

        assert_eq!(first.inspect().to_string(), expected);
        assert_eq!(second, first);
        assert_ne!(advanced, first);
    }

    #[test]
    fn reports_checked_conversion_failures_at_the_float_provider() {
        let cases = [
            (
                "import host/float_values\npub fn main() { js_round(float_values.nan()) }",
                "host function gleam_stdlib::gleam/float.js_round failed: float cannot be represented as an Int",
            ),
            (
                "import host/float_values\npub fn main() { truncate(float_values.infinity()) }",
                "host function gleam_stdlib::gleam/float.truncate failed: float cannot be represented as an Int",
            ),
            (
                "pub fn main() { do_to_float(10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000) }",
                "host function gleam_stdlib::gleam/float.do_to_float failed: Int cannot be represented as a finite Float",
            ),
        ];

        for (source, expected) in cases {
            let values = HostModule::<GleamStdlibProfile>::new_for_profile(
                "gleam_stdlib",
                "host/float_values",
            )
            .expect("float value module should be valid")
            .with_function("nan", || f64::NAN)
            .expect("NaN function should be valid")
            .with_function("infinity", || f64::INFINITY)
            .expect("infinity function should be valid");
            let execution = execution(source, [values]);
            let error = execution
                .run_main(
                    &mut GleamStdlibRunState::from_seed([0; 32]),
                    &mut Vec::new(),
                )
                .expect_err("checked conversion should fail");

            assert_eq!(error.to_string(), expected);
        }
    }
}
