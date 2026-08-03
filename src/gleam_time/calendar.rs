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
    use crate::gleam_time::GleamTimeProfile;
    use crate::{FunctionType, ValueType};

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
}
