use super::{Observation, Profile, State, Stores};
use geam::host::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostExternalBinding,
    HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostProvider,
    HostProviderModule,
};
use std::sync::{Arc, Mutex};

pub(super) struct Probe;
impl HostProvider<Profile> for Probe {
    type State = Arc<Mutex<Observation>>;
    fn project(state: &mut State) -> &mut Self::State {
        &mut state.observed
    }
}

pub(super) struct Witness(Arc<Mutex<Observation>>);
impl Drop for Witness {
    fn drop(&mut self) {
        self.0.lock().unwrap().dropped_captures += 1;
    }
}

struct TokenSchema;
type Token = HostExternalType<TokenSchema>;
impl HostExternalSchema for TokenSchema {
    const PACKAGE: &'static str = "otp_service_fixture";
    const MODULE: &'static str = "otp_lifetime";
    const NAME: &'static str = "Token";
    const PARAMETER_COUNT: usize = 0;
}
impl HostExternalBinding<Profile, TokenSchema> for Probe {
    type Storage = Probe;
}
impl HostExternalStorage<Profile, TokenSchema> for Probe {
    type Payload = Witness;
    fn store(stores: &Stores) -> &HostExternalStore<Witness> {
        &stores.witnesses
    }
    fn source_equal(
        _: &geam::host::HostExternalEquality<'_>,
        left: &Witness,
        right: &Witness,
    ) -> bool {
        Arc::ptr_eq(&left.0, &right.0)
    }
    fn source_hash(_: &geam::host::HostExternalHashing<'_>, value: &Witness) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        Arc::as_ptr(&value.0).hash(&mut hash);
        hash.finish()
    }
    fn inspect(
        _: &geam::host::HostExternalInspection<'_>,
        _: &Witness,
    ) -> geam::provider::EcoString {
        "CaptureWitness".into()
    }
}

pub(super) fn provider() -> HostProviderModule<Profile> {
    HostProviderModule::new("otp_service_fixture", "otp_lifetime")
        .unwrap()
        .with_external_type::<Probe, TokenSchema>()
        .unwrap()
        .with_scoped_function::<Probe, (), Token, _>("token", token)
        .unwrap()
        .with_scoped_function::<Probe, (Token,), (), _>("touch", touch)
        .unwrap()
}

fn token<'call>(
    mut call: HostCall<'call, Profile, Probe, Token>,
) -> Result<HostCallCompletion<'call, Token>, HostCallError> {
    let witness = Witness(Arc::clone(call.state()));
    let value = call.create_external(witness);
    Ok(call.return_value(value))
}
fn touch<'call>(
    call: HostCall<'call, Profile, Probe, ()>,
    token: HostExternal<'call, Token>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    assert_eq!(
        call.external_payload(token)
            .0
            .lock()
            .unwrap()
            .dropped_captures,
        0
    );
    Ok(call.return_value(()))
}
