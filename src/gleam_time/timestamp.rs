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
    use crate::gleam_time::GleamTimeProfile;
    use crate::{FunctionType, ValueType};

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
}
