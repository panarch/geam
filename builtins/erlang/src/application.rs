use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostProviderModule, HostRegistrationError,
};
use geam_stdlib::provider_support::{GleamError, GleamOk, GleamResult};

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/application")
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (EcoString,), GleamResult<EcoString, ()>, _>(
            "priv_directory",
            priv_directory::<Profile>,
        ))
}

fn priv_directory<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, GleamResult<EcoString, ()>>,
    package: EcoString,
) -> Result<HostCallCompletion<'call, GleamResult<EcoString, ()>>, HostCallError> {
    match call
        .state()
        .resources
        .get(&package)
        .and_then(|path| path.to_str())
        .map(EcoString::from)
    {
        Some(path) => Ok(call.return_custom::<GleamOk<EcoString, ()>>((path, ()))),
        None => Ok(call.return_custom::<GleamError<EcoString, ()>>(((), ()))),
    }
}
