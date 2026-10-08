use geam_core::embedding::BigInt;
use geam_core::provider::{ProviderValueContext, Value};
use geam_core::{
    HostCall, HostCallContinuation, HostCallError, HostConstructions, HostFailure, HostList,
    HostListType, HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule,
    HostProviderSet, HostTypeListEnd,
};
use std::future::poll_fn;
use std::task::Poll;

pub struct Profile;
struct Provider;

impl HostProfile for Profile {
    type RunState = Vec<&'static str>;
    type ExternalStores = ();
    type ExecutionState = ();
}

impl HostProvider<Profile> for Provider {
    type State = Vec<&'static str>;

    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

type Ints = HostListType<BigInt>;

pub fn hosts() -> HostProviderSet<Profile> {
    HostProviderSet::from_providers([HostProviderModule::new("example", "example")
        .unwrap()
        .with_resumable_function::<Provider, (Ints, bool), Ints, HostTypeListEnd, _>("hold", hold)
        .unwrap()])
    .unwrap()
}

fn hold<'call>(
    mut call: HostCall<'call, Profile, Provider, Ints>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    values: HostList<'call, BigInt>,
    fail: bool,
) -> Result<HostCallContinuation<'call, Ints>, HostCallError> {
    call.state().push("entered");
    if fail {
        return Err(HostFailure::new("list native failure").into());
    }
    let values = Value::<Ints, ProviderValueContext<Ints>>::from_host(&call, values);
    Ok(call.resume(constructions, move |_context| {
        Box::pin(async move {
            let mut waiting = true;
            poll_fn(|context| {
                if waiting {
                    waiting = false;
                    context.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(())
                }
            })
            .await;
            Ok(HostOwnedCompletion::new(
                move |mut call: HostCall<'_, Profile, Provider, Ints>, _| {
                    call.state().push("completed");
                    let values = values.into_host(&mut call)?;
                    Ok(call.return_value(values))
                },
            ))
        })
    }))
}
