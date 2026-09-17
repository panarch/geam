use crate::{Component, GleamErlangHostProfile};
use geam_core::StringValue;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostProviderModule, HostRegistrationError,
};
use geam_stdlib::provider_support::{GleamError, GleamOk, GleamResult};

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/application")
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (StringValue,), GleamResult<StringValue, ()>, _>(
            "priv_directory",
            priv_directory::<Profile>,
        ))
}

fn priv_directory<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, GleamResult<StringValue, ()>>,
    package: StringValue,
) -> Result<HostCallCompletion<'call, GleamResult<StringValue, ()>>, HostCallError> {
    match call
        .state()
        .resources
        .get(package.as_str())
        .and_then(|path| path.to_str())
        .map(StringValue::from)
    {
        Some(path) => Ok(call.return_custom::<GleamOk<StringValue, ()>>((path, ()))),
        None => Ok(call.return_custom::<GleamError<StringValue, ()>>(((), ()))),
    }
}
