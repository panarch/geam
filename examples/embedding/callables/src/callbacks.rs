use crate::declarations::AddOffset;
use crate::pricing::Pricing;
use geam::embedding::BigInt;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostCaptures, HostConstructions, HostProfile,
    HostProvider, HostProviderSet, HostRegistrationError, HostTypeList, HostTypeListEnd,
};

pub struct Application;
impl HostProfile for Application {
    type RunState = Pricing;
    type ExternalStores = ();
    type ExecutionState = ();
}
impl HostProvider<Application> for Application {
    type State = Pricing;
    fn project(state: &mut Pricing) -> &mut Pricing {
        state
    }
}

pub fn providers() -> Result<HostProviderSet<Application>, HostRegistrationError> {
    HostProviderSet::new([])?.with_callable::<Application, AddOffset, (BigInt,), _>(add_offset)
}

fn add_offset<'call>(
    mut call: HostCall<'call, Application, Application, BigInt>,
    captures: HostCaptures<'call, HostTypeList<BigInt, HostTypeListEnd>>,
    _: HostConstructions<'call, HostTypeListEnd>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let (offset, ()) = call.captures(captures);
    let price = crate::pricing::add_offset(call.state(), value, offset);
    Ok(call.return_value(price))
}
