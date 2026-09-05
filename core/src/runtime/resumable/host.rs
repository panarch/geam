use super::ResumableState;
use crate::host::{
    AsyncHostCallError, AsyncHostRequest, AsyncHostRequestPort, HostProfile, ScopedAsyncHostFuture,
};
use crate::plan::execution::AsyncHostedExecution;
use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

enum HostProgress<Profile: HostProfile, Return> {
    Complete(Result<Return, AsyncHostCallError>),
    Request(AsyncHostRequest<Profile>),
}

#[expect(
    clippy::manual_async_fn,
    reason = "The root driver enforces its Send and call-lifetime contract in the return type."
)]
pub(in crate::runtime) fn drive_async_host<'call, Profile, Return>(
    mut future: ScopedAsyncHostFuture<'call, Return>,
    port: Arc<AsyncHostRequestPort<Profile>>,
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
) -> impl Future<Output = Result<Return, AsyncHostCallError>> + Send + 'call
where
    Profile: HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
    Return: Send + 'call,
{
    async move {
        loop {
            let progress = next_host_progress(&mut future, &port).await;
            match progress {
                HostProgress::Complete(output) => return output,
                HostProgress::Request(AsyncHostRequest::Operation(request)) => {
                    request.service(state.host, state.stores);
                }
                HostProgress::Request(AsyncHostRequest::Callback(request)) => {
                    request.service(plan, state).await;
                }
            }
        }
    }
}

async fn next_host_progress<Profile: HostProfile, Return>(
    future: &mut ScopedAsyncHostFuture<'_, Return>,
    port: &AsyncHostRequestPort<Profile>,
) -> HostProgress<Profile, Return> {
    poll_fn(|context| match Pin::new(&mut *future).poll(context) {
        Poll::Ready(output) => Poll::Ready(HostProgress::Complete(output)),
        Poll::Pending => match port.pop() {
            Some(request) => Poll::Ready(HostProgress::Request(request)),
            None => Poll::Pending,
        },
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::super::{RecordedEcho, ResumableState};
    use super::drive_async_host;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_async_host_program};
    use crate::host::{
        AsyncHostCall, AsyncHostFuture, AsyncHostModule, AsyncHostProviderSet,
        AsyncHostRequestContext, AsyncHostRequestPort, HostProfile, HostProvider,
        ScopedAsyncHostFuture,
    };
    use crate::plan::execution::AsyncHostedExecution;
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::runtime::HostCallOrigin;
    use std::cell::Cell;
    use std::future::Future;
    use std::sync::Arc;
    use std::task::{Context, Poll, Waker};

    struct Profile;
    struct Provider;

    struct State(Cell<usize>);

    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
    }

    impl HostProvider<Profile> for Provider {
        type State = State;

        fn project(state: &mut State) -> &mut Self::State {
            state
        }
    }

    #[test]
    fn services_each_request_before_polling_the_host_again() {
        let hosts = AsyncHostProviderSet::<Profile>::new(Vec::<AsyncHostModule<Profile>>::new())
            .expect("empty async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn run() { 0 }",
                )],
            )],
            hosts,
        )
        .expect("state driver source");
        let plan =
            crate::planner::plan_async_host_library_program(program).expect("state driver plan");
        let template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("public run function")
            .signature()
            .id();
        let entry = LibraryEntry::new(template, LibraryValueType::Int, Vec::new(), Vec::new());
        let (execution, _) = AsyncHostedExecution::from_library_plan(plan, entry, Vec::new());
        let port = AsyncHostRequestPort::<Profile>::new();
        let context = AsyncHostRequestContext::new(
            Arc::clone(&port),
            HostCallOrigin::Entry,
            &(),
            Default::default(),
        );
        let call = AsyncHostCall::<Profile, Provider>::new(context);
        let future = ScopedAsyncHostFuture::Infallible(AsyncHostFuture::new(async move {
            let mut call = call;
            let first = call.with_state(|state| {
                let value = state.0.get();
                state.0.set(value + 1);
                value
            });
            let first = first.await;
            let second = call
                .with_state(move |state| {
                    state.0.set(state.0.get() + first + 1);
                    state.0.get()
                })
                .await;
            let mut yielded = false;
            std::future::poll_fn(|context| {
                if yielded {
                    Poll::Ready(())
                } else {
                    yielded = true;
                    context.waker().wake_by_ref();
                    Poll::Pending
                }
            })
            .await;
            second
        }));
        let mut state = State(Cell::new(1));
        let mut echo = RecordedEcho::default();
        let mut stores = ();
        let mut runtime = ResumableState::<Profile>::new(&mut state, &mut stores, &mut echo);
        let mut context = Context::from_waker(Waker::noop());
        let mut driven = Box::pin(drive_async_host(future, port, &execution, &mut runtime));
        let mut pending = 0;
        let result = loop {
            match driven.as_mut().poll(&mut context) {
                Poll::Ready(result) => break result,
                Poll::Pending => pending += 1,
            }
        };
        assert_eq!(result.expect("state driver should complete"), 4);
        assert_eq!(pending, 1);
        drop(driven);
        drop(runtime);
        assert_eq!(state.0.get(), 4);
    }
}
