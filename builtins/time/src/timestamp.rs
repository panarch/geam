use super::source::split_system_time;
use super::{Component, GleamTimeHostProfile, TimeSource};
use crate::{HostProviderModule, HostRegistrationError};
use geam_core::provider::{Call, HostResult};
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/time/timestamp",
    crate_path = geam_core,
    profile = crate::GleamTimeHostProfile,
    component = crate::Component<Profile::Source>,
)]
mod provider {
    use super::{BigInt, Call, HostResult, TimeSource, split_system_time};

    #[geam_macros::function(profile = Profile)]
    fn get_system_time(
        #[geam_macros::call] call: &mut Call<Profile::Source>,
    ) -> HostResult<(BigInt, BigInt)> {
        let time = call.state_mut().system_time()?;
        Ok(split_system_time(time))
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamTimeHostProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::test_support::{ScriptedSource, TIMESTAMP_SOURCE, execution};
    use crate::{ExecutionError, HostError, HostFailure, InvariantError, ValueType};
    use crate::{GleamTimeProfile, GleamTimeRunState};
    use geam_stdlib::GleamStdlibRunState;

    #[test]
    fn registers_the_exact_timestamp_provider() {
        let provider = host_provider::<GleamTimeProfile>()
            .expect("official Time timestamp provider should register");
        let functions = provider.functions().collect::<Vec<_>>();

        assert_eq!(provider.package(), "gleam_time");
        assert_eq!(provider.module(), "gleam/time/timestamp");
        assert_eq!(provider.external_types().count(), 0);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name(), "get_system_time");
        assert!(functions[0].scheme().parameters().is_empty());
        assert!(functions[0].type_().argument_types().is_empty());
        assert_eq!(
            functions[0].type_().return_(),
            &ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        );
    }

    #[test]
    fn preserves_system_time_source_failures() {
        let source = format!("{TIMESTAMP_SOURCE}\npub fn main() {{\n  current_parts()\n}}\n",);
        let execution = execution::<ScriptedSource>(&source, "gleam/time/timestamp");
        let mut state = GleamTimeRunState::new(
            GleamStdlibRunState::from_seed([4; 32]),
            ScriptedSource {
                times: [Err(HostFailure::new("clock unavailable"))].into(),
                offsets: Default::default(),
            },
        );
        let error = execution
            .run_main(&mut state, &mut Vec::new())
            .expect_err("scripted clock failure should remain an execution error");
        let error = expect_clock_host_error(error);

        assert_eq!(error.package(), "gleam_time");
        assert_eq!(error.module(), "gleam/time/timestamp");
        assert_eq!(error.function(), "get_system_time");
        assert_eq!(error.failure().message(), "clock unavailable");
        assert_eq!(
            error
                .location()
                .path()
                .expect("synthetic clock failure should retain its source path")
                .as_str(),
            "src/gleam/time/timestamp.gleam",
        );
        assert_eq!(error.location().line(), Some(7));
    }

    #[test]
    #[should_panic(expected = "scripted clock failure should remain a host error")]
    fn clock_failure_assertion_rejects_other_execution_errors() {
        let _ = expect_clock_host_error(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        ));
    }

    fn expect_clock_host_error(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("scripted clock failure should remain a host error");
        };
        error
    }
}
