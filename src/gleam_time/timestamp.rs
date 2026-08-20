use super::source::split_system_time;
use super::{GleamTimeHostProfile, TimeProvider, TimeSource};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostProviderModule, HostRegistrationError,
    HostTupleType, HostTypeList, HostTypeListEnd,
};
use num_bigint::BigInt;

type TimestampPartsElements = HostTypeList<BigInt, HostTypeList<BigInt, HostTypeListEnd>>;
type TimestampParts = HostTupleType<TimestampPartsElements>;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamTimeHostProfile,
{
    HostProviderModule::new("gleam_time", "gleam/time/timestamp").and_then(|provider| {
        provider.with_scoped_function::<TimeProvider<Profile>, (), TimestampParts, _>(
            "get_system_time",
            get_system_time::<Profile>,
        )
    })
}

fn get_system_time<'call, Profile>(
    mut call: HostCall<'call, Profile, TimeProvider<Profile>, TimestampParts>,
) -> Result<HostCallCompletion<'call, TimestampParts>, HostCallError>
where
    Profile: GleamTimeHostProfile,
{
    let time = call.state().system_time()?;
    let (seconds, nanoseconds) = split_system_time(time);
    Ok(call.return_tuple((seconds, (nanoseconds, ()))))
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibRunState;
    use crate::gleam_time::test_support::{ScriptedSource, TIMESTAMP_SOURCE, execution};
    use crate::gleam_time::{GleamTimeProfile, GleamTimeRunState};
    use crate::{ExecutionError, FunctionType, HostError, HostFailure, InvariantError, ValueType};

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
        assert!(functions[0].scheme().is_monomorphic());
        assert_eq!(
            functions[0].type_(),
            &FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
            ),
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
