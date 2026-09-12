use crate::schema::{Atom, AtomSchema};
use crate::{Component, GleamErlangHostProfile};
use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal,
    HostProviderModule, HostRegistrationError, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::provider_support::{Dynamic, DynamicSchema, GleamError, GleamOk, GleamResult};

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/atom")
        .and_then(|module| module.with_external_type::<Component<Profile>, AtomSchema>())
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (EcoString,), Atom, _>("create", create::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (EcoString,), GleamResult<Atom, ()>, HostTypeList<Atom, HostTypeListEnd>, _>("get", get::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Atom,), EcoString, _>("to_string", to_string::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Atom,), Dynamic, _>("to_dynamic", to_dynamic::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Dynamic,), Atom, _>("cast_from_dynamic", cast_from_dynamic::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Dynamic,), bool, _>("is_atom", is_atom::<Profile>))
}

fn create<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Atom>,
    name: EcoString,
) -> Result<HostCallCompletion<'call, Atom>, HostCallError> {
    let name = Profile::erlang_execution(call.execution_state()).intern(name)?;
    let value = call.create_external(NativeValue::symbol(name));
    Ok(call.return_value(value))
}

fn get<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, GleamResult<Atom, ()>>,
    constructions: HostConstructions<'call, HostTypeList<Atom, HostTypeListEnd>>,
    name: EcoString,
) -> Result<HostCallCompletion<'call, GleamResult<Atom, ()>>, HostCallError> {
    match Profile::erlang_execution(call.execution_state()).existing_atom(&name) {
        Some(name) => {
            let value = call.construct_external(
                constructions.at::<HostTypeIndex0>(),
                NativeValue::symbol(name),
            );
            Ok(call.return_custom::<GleamOk<Atom, ()>>((value, ())))
        }
        None => Ok(call.return_custom::<GleamError<Atom, ()>>(((), ()))),
    }
}

fn to_string<'call, Profile: GleamErlangHostProfile>(
    call: HostCall<'call, Profile, Component<Profile>, EcoString>,
    atom: HostExternal<'call, Atom>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError> {
    let value = call
        .external_payload(atom)
        .as_symbol()
        .ok_or_else(|| geam_core::HostFailure::new("atom_to_binary requires an atom"))?;
    Ok(call.return_value(value))
}

fn to_dynamic<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Dynamic>,
    atom: HostExternal<'call, Atom>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
    let value = call.external_payload(atom).clone();
    let value = call.create_external(geam_stdlib::Dynamic::from_native(value));
    Ok(call.return_value(value))
}

fn cast_from_dynamic<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Atom>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, Atom>, HostCallError> {
    let value = call
        .external_payload::<DynamicSchema, HostTypeListEnd>(value)
        .native_value()
        .clone();
    let value = call.create_external(value);
    Ok(call.return_value(value))
}

fn is_atom<'call, Profile: GleamErlangHostProfile>(
    call: HostCall<'call, Profile, Component<Profile>, bool>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let value = call
        .external_payload(value)
        .native_value()
        .as_symbol()
        .is_some();
    Ok(call.return_value(value))
}
