use super::{GleamTimeHostProfile, TimeProvider, TimeSource};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostProviderModule, HostRegistrationError,
};
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamTimeHostProfile,
{
    HostProviderModule::new("gleam_time", "gleam/time/calendar").and_then(|provider| {
        provider.with_scoped_function::<TimeProvider<Profile>, (), BigInt, _>(
            "local_time_offset_seconds",
            local_time_offset_seconds::<Profile>,
        )
    })
}

fn local_time_offset_seconds<'call, Profile>(
    mut call: HostCall<'call, Profile, TimeProvider<Profile>, BigInt>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>
where
    Profile: GleamTimeHostProfile,
{
    let seconds = call.state().local_offset_seconds()?;
    Ok(call.return_value(BigInt::from(seconds)))
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibRunState;
    use crate::gleam_time::test_support::{CALENDAR_SOURCE, ScriptedSource, execution};
    use crate::gleam_time::{GleamTimeProfile, GleamTimeRunState};
    use crate::{ExecutionError, FunctionType, HostError, HostFailure, InvariantError, ValueType};

    #[test]
    fn registers_the_exact_calendar_provider() {
        let provider = host_provider::<GleamTimeProfile>()
            .expect("official Time calendar provider should register");
        let functions = provider.functions().collect::<Vec<_>>();

        assert_eq!(provider.package(), "gleam_time");
        assert_eq!(provider.module(), "gleam/time/calendar");
        assert_eq!(provider.external_types().count(), 0);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name(), "local_time_offset_seconds");
        assert!(functions[0].scheme().is_monomorphic());
        assert_eq!(
            functions[0].type_(),
            &FunctionType::new(Vec::new(), ValueType::Int),
        );
    }

    #[test]
    fn preserves_local_offset_source_failures() {
        let source = format!("{CALENDAR_SOURCE}\npub fn main() {{\n  current_offset()\n}}\n",);
        let execution = execution::<ScriptedSource>(&source, "gleam/time/calendar");
        let mut state = GleamTimeRunState::new(
            GleamStdlibRunState::from_seed([4; 32]),
            ScriptedSource {
                times: Default::default(),
                offsets: [Err(HostFailure::new("offset unavailable"))].into(),
            },
        );
        let error = execution
            .run_main(&mut state, &mut Vec::new())
            .expect_err("scripted offset failure should remain an execution error");
        let error = expect_offset_host_error(error);

        assert_eq!(error.package(), "gleam_time");
        assert_eq!(error.module(), "gleam/time/calendar");
        assert_eq!(error.function(), "local_time_offset_seconds");
        assert_eq!(error.failure().message(), "offset unavailable");
        assert_eq!(
            error
                .location()
                .path()
                .expect("synthetic offset failure should retain its source path")
                .as_str(),
            "src/gleam/time/calendar.gleam",
        );
        assert_eq!(error.location().line(), Some(7));
    }

    #[test]
    #[should_panic(expected = "scripted offset failure should remain a host error")]
    fn offset_failure_assertion_rejects_other_execution_errors() {
        let _ = expect_offset_host_error(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        ));
    }

    fn expect_offset_host_error(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("scripted offset failure should remain a host error");
        };
        error
    }
}
