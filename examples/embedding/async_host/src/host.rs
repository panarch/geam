use geam::embedding::BigInt;
use geam::{
    AsyncHostCall, AsyncHostCallError, AsyncHostCallable, AsyncHostFuture, AsyncHostModule,
    AsyncHostProviderModule, AsyncHostProviderSet, HostFunctionType, HostProfile, HostProvider,
    HostRegistrationError, HostTypeList, HostTypeListEnd,
};
use std::future::poll_fn;
use std::task::Poll;

pub(super) struct Profile;

pub(super) struct RunState {
    offset: BigInt,
    completed: usize,
}

struct State;

type IntCallbackArguments = HostTypeList<BigInt, HostTypeListEnd>;
type IntCallback = HostFunctionType<IntCallbackArguments, BigInt>;

impl RunState {
    pub(super) fn new(offset: impl Into<BigInt>) -> Self {
        Self {
            offset: offset.into(),
            completed: 0,
        }
    }

    pub(super) fn completed(&self) -> usize {
        self.completed
    }
}

impl HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = ();
}

impl HostProvider<Profile> for State {
    type State = RunState;

    fn project(state: &mut RunState) -> &mut Self::State {
        state
    }
}

pub(super) fn async_hosts() -> Result<AsyncHostProviderSet<Profile>, HostRegistrationError> {
    let provider = AsyncHostProviderModule::<Profile>::new(
        "geam_rust_embedding_async_host",
        "geam_rust_embedding_async_host",
    )?
    .with_async_function("pause", pause)?
    .with_fallible_scoped_async_function::<State, (IntCallback, BigInt), BigInt, _>(
        "around", around,
    )?;
    AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule<Profile>>::new(), [provider])
}

async fn pause(value: BigInt) -> BigInt {
    yield_once().await;
    value
}

fn around<'call>(
    mut call: AsyncHostCall<'call, Profile, State, BigInt>,
    callback: AsyncHostCallable<'call, Profile, IntCallbackArguments, BigInt>,
    value: BigInt,
) -> AsyncHostFuture<'call, Result<BigInt, AsyncHostCallError>> {
    AsyncHostFuture::new(async move {
        let offset = call.with_state(|state| state.offset.clone()).await;
        let value = call.invoke(&callback, (value + offset, ())).await?;
        let value = call
            .with_state(move |state| {
                state.completed += 1;
                value + BigInt::from(state.completed)
            })
            .await;
        Ok(value)
    })
}

async fn yield_once() {
    let mut pending = true;
    poll_fn(move |context| {
        if std::mem::take(&mut pending) {
            context.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
    .await
}
