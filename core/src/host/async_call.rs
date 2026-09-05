use super::{HostProfile, HostProvider, HostType, HostTypeList, HostTypeListEnd, HostTypeSequence};
use crate::runtime::{
    AsyncHostCallbackRequest, CallbackRequest, HostCallOrigin, ResumableCallback, TransferInputs,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::VecDeque;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::task::{Context, Poll, Waker};

/// A call-scoped host Future used by a resumable host function.
///
/// The Future may borrow its [`AsyncHostCall`] for the duration of one host
/// invocation. Geam does not choose or run an executor for it.
///
/// Values retained across suspension must be transferable:
///
/// ```compile_fail
/// use geam_core::AsyncHostFuture;
/// use std::{future::pending, rc::Rc};
/// let future = AsyncHostFuture::new(async {
///     let local = Rc::new(7);
///     pending::<()>().await;
///     drop(local);
/// });
/// ```
pub struct AsyncHostFuture<'call, Output> {
    inner: Pin<Box<dyn Future<Output = Output> + Send + 'call>>,
}

/// A non-cloneable capability for bounded operations during one async host call.
///
/// The capability cannot escape the invocation even though its requests are owned:
///
/// ```compile_fail
/// use geam_core::{AsyncHostCall, HostProfile, HostProvider};
/// fn escape<'call, Profile: HostProfile, Provider: HostProvider<Profile>>(
///     call: AsyncHostCall<'call, Profile, Provider>,
/// ) -> AsyncHostCall<'static, Profile, Provider> {
///     call
/// }
/// ```
pub struct AsyncHostCall<'call, Profile, Provider, Return = ()>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    port: Arc<AsyncHostRequestPort<Profile>>,
    origin: HostCallOrigin,
    lists: crate::runtime::TransferListStorage,
    lifetime: PhantomData<&'call mut ()>,
    provider: PhantomData<fn() -> Provider>,
    return_: PhantomData<fn() -> Return>,
}

/// An owned typed Gleam callable available during one async host invocation.
///
/// Calling it is mediated by the matching [`AsyncHostCall`], so the callable
/// can cross `.await` without retaining a borrow of the active Geam runtime.
/// It remains scoped to its originating host invocation:
///
/// ```compile_fail
/// use geam_core::{AsyncHostCallable, HostTypeListEnd, StatelessHostProfile};
/// fn escape<'call>(
///     value: AsyncHostCallable<'call, StatelessHostProfile, HostTypeListEnd, ()>,
/// ) -> AsyncHostCallable<'static, StatelessHostProfile, HostTypeListEnd, ()> {
///     value
/// }
/// ```
pub struct AsyncHostCallable<'call, Profile, Arguments, Return>
where
    Profile: HostProfile,
{
    target: AsyncHostCallbackTarget<Profile, Return>,
    lifetime: PhantomData<&'call mut ()>,
    arguments: PhantomData<fn(Arguments)>,
}

pub(crate) struct AsyncHostRequestContext<'call, Profile: HostProfile> {
    port: Arc<AsyncHostRequestPort<Profile>>,
    origin: HostCallOrigin,
    lists: crate::runtime::TransferListStorage,
    lifetime: PhantomData<&'call mut ()>,
}

pub(crate) struct AsyncHostRequestPort<Profile: HostProfile> {
    requests: Mutex<VecDeque<AsyncHostRequest<Profile>>>,
}

pub(crate) enum AsyncHostRequest<Profile: HostProfile> {
    Operation(Box<dyn AsyncHostOperationRequest<Profile> + Send>),
    Callback(Box<dyn AsyncHostCallbackRequest<Profile> + Send>),
}

pub(crate) trait AsyncHostOperationRequest<Profile: HostProfile> {
    fn service(
        self: Box<Self>,
        state: &mut Profile::RunState,
        stores: &mut Profile::ExternalStores,
    );
}

type AsyncHostCallbackTarget<Profile, Return> = Box<
    dyn FnOnce(
            TransferInputs,
            HostCallOrigin,
            Weak<Mutex<AsyncHostCallbackCompletion<Return>>>,
        ) -> Box<dyn AsyncHostCallbackRequest<Profile> + Send>
        + Send
        + 'static,
>;

struct OperationRequest<Profile, Operation, Output>
where
    Profile: HostProfile,
{
    operation: Operation,
    completion: Weak<Mutex<OperationCompletion<Output>>>,
    marker: PhantomData<Profile>,
}

struct OperationFuture<'request, Profile, Operation, Output>
where
    Profile: HostProfile,
{
    port: Arc<AsyncHostRequestPort<Profile>>,
    state: OperationFutureState<Operation>,
    completion: Arc<Mutex<OperationCompletion<Output>>>,
    lifetime: PhantomData<&'request mut ()>,
}

enum OperationFutureState<Operation> {
    Unqueued(Operation),
    Queued,
}

struct OperationCompletion<Output> {
    output: Option<Output>,
    waker: Waker,
}

pub(crate) struct AsyncHostCallbackCompletion<Output> {
    output: Option<Result<Output, super::AsyncHostCallError>>,
    waker: Waker,
}

struct CallbackRequestFuture<Profile, Return>
where
    Profile: HostProfile,
{
    port: Arc<AsyncHostRequestPort<Profile>>,
    state: CallbackRequestFutureState<Profile, Return>,
    completion: Arc<Mutex<AsyncHostCallbackCompletion<Return>>>,
}

enum CallbackRequestFutureState<Profile, Return>
where
    Profile: HostProfile,
{
    Unqueued {
        target: AsyncHostCallbackTarget<Profile, Return>,
        inputs: TransferInputs,
        origin: HostCallOrigin,
    },
    Queued,
}

pub(crate) trait AsyncHostCallbackArguments: HostTypeSequence {
    fn append_inputs(values: Self::Values<'static>, inputs: &mut TransferInputs);

    fn into_inputs(values: Self::Values<'static>) -> TransferInputs {
        let mut inputs = TransferInputs::empty();
        Self::append_inputs(values, &mut inputs);
        inputs
    }
}

trait AsyncHostCallbackArgument: HostType {
    fn push(value: Self::Value<'static>, inputs: &mut TransferInputs);
}

impl AsyncHostCallbackArguments for HostTypeListEnd {
    fn append_inputs((): Self::Values<'static>, _inputs: &mut TransferInputs) {}
}

impl<Head, Tail> AsyncHostCallbackArguments for HostTypeList<Head, Tail>
where
    Head: AsyncHostCallbackArgument,
    Tail: AsyncHostCallbackArguments,
{
    fn append_inputs((head, tail): Self::Values<'static>, inputs: &mut TransferInputs) {
        Head::push(head, inputs);
        Tail::append_inputs(tail, inputs);
    }
}

macro_rules! callback_argument {
    ($type:ty, $push:ident) => {
        impl AsyncHostCallbackArgument for $type {
            fn push(value: Self::Value<'static>, inputs: &mut TransferInputs) {
                inputs.$push(value);
            }
        }
    };
}

callback_argument!(BigInt, push_int);
callback_argument!(f64, push_float);
callback_argument!(EcoString, push_string);
callback_argument!(crate::BitArrayValue, push_bit_array);
callback_argument!(char, push_utf_codepoint);
callback_argument!(bool, push_bool);

impl AsyncHostCallbackArgument for () {
    fn push((): Self::Value<'static>, inputs: &mut TransferInputs) {
        inputs.push_nil();
    }
}

impl<'call, Output> AsyncHostFuture<'call, Output> {
    /// Boxes a call-scoped Future returned by a scoped async host function.
    ///
    /// The Future may borrow values tied to the current host invocation, but it
    /// must remain `Send` so the caller's task may move between executor workers.
    pub fn new(future: impl Future<Output = Output> + Send + 'call) -> Self {
        Self {
            inner: Box::pin(future),
        }
    }
}

impl<Output> Future for AsyncHostFuture<'_, Output> {
    type Output = Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().inner.as_mut().poll(context)
    }
}

impl<'call, Profile, Provider, Return> AsyncHostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    pub(crate) fn new(context: AsyncHostRequestContext<'call, Profile>) -> Self {
        Self {
            port: context.port,
            origin: context.origin,
            lists: context.lists,
            lifetime: PhantomData,
            provider: PhantomData,
            return_: PhantomData,
        }
    }

    /// Requests a typed external payload read through the root driver.
    ///
    /// The driver runs the view synchronously and returns an owned value. The
    /// host Future never borrows the module's stores or the payload across `.await`.
    pub fn with_external<'request, Schema, Arguments, Output>(
        &'request mut self,
        value: &super::AsyncHostExternal<'call, Schema, Arguments>,
        view: impl FnOnce(
            &<Provider::Storage as super::AsyncHostExternalStorage<Profile, Schema>>::Payload,
        ) -> Output
        + Send
        + 'static,
    ) -> impl Future<Output = Output> + Send + 'request
    where
        Schema: super::HostExternalSchema,
        Provider: super::AsyncHostExternalBinding<Profile, Schema>,
        Output: Send + 'static,
    {
        use super::AsyncHostExternalStorage;
        let lease = value.lease().clone();
        OperationFuture::<Profile, _, _>::new(Arc::clone(&self.port), move |_, stores| {
            Provider::Storage::store(stores).with_view(&lease, view)
        })
    }

    /// Computes a Gleam source hash for one external value and its retained graph.
    ///
    /// Equal source values have equal hashes. The hash is not a stable encoding.
    pub fn source_hash<Schema, Arguments>(
        &mut self,
        value: &super::AsyncHostExternal<'call, Schema, Arguments>,
    ) -> u64 {
        value.source_hash(&self.lists)
    }

    /// Runs one non-async operation against this provider's state.
    ///
    /// The operation and its result are owned. A reference derived from state
    /// therefore cannot remain live across the surrounding host Future's next
    /// `.await`.
    ///
    /// ```compile_fail
    /// use geam_core::{AsyncHostCall, HostProfile, HostProvider};
    /// struct Profile;
    /// struct Provider;
    /// impl HostProfile for Profile {
    ///     type RunState = String;
    ///     type ExternalStores = ();
    /// }
    /// impl HostProvider<Profile> for Provider {
    ///     type State = String;
    ///     fn project(state: &mut String) -> &mut String { state }
    /// }
    /// async fn borrow_state(mut call: AsyncHostCall<'_, Profile, Provider>) {
    ///     let borrowed = call.with_state(|state| state.as_str()).await;
    ///     println!("{borrowed}");
    /// }
    /// ```
    pub fn with_state<'request, Operation, Output>(
        &'request mut self,
        operation: Operation,
    ) -> impl Future<Output = Output> + Send + 'request
    where
        Profile::RunState: Send,
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
        Output: Send + 'static,
    {
        OperationFuture::<Profile, _, Output>::new(Arc::clone(&self.port), move |state, _| {
            operation(Provider::project(state))
        })
    }

    /// Invokes one owned typed Gleam callback through this call's root driver.
    ///
    /// Arguments are owned before the request can become pending. The callback
    /// executes in the same Geam session and may itself reach an async host
    /// function.
    #[allow(private_bounds)]
    pub fn invoke<'request, Arguments, CallbackReturn>(
        &'request mut self,
        function: AsyncHostCallable<'call, Profile, Arguments, CallbackReturn>,
        arguments: Arguments::Values<'static>,
    ) -> impl Future<Output = Result<CallbackReturn, super::AsyncHostCallError>> + Send + 'request
    where
        Arguments: AsyncHostCallbackArguments,
        CallbackReturn: Send + 'static,
    {
        CallbackRequestFuture::new(
            Arc::clone(&self.port),
            function.target,
            Arguments::into_inputs(arguments),
            self.origin.clone(),
        )
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    AsyncHostCall<'call, Profile, Provider, super::HostExternalType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: super::AsyncHostExternalBinding<Profile, Schema>,
    Schema: super::HostExternalSchema,
{
    /// Builds an immutable payload and requests its declared external return.
    ///
    /// Only this bounded builder can retain call-scoped values in the new payload.
    /// Construction finishes before the request is queued; the root driver owns
    /// insertion into the module's store when the request is awaited.
    pub fn return_external<'request>(
        &'request mut self,
        build: impl FnOnce(&mut super::AsyncHostExternalPayloadBuilder<'call>)
            -> <Provider::Storage as super::AsyncHostExternalStorage<Profile, Schema>>::Payload,
    ) -> impl Future<Output = super::AsyncHostExternalReturn<'call, Schema, Arguments>> + Send + 'request
    {
        use super::AsyncHostExternalStorage;
        let payload = build(&mut super::AsyncHostExternalPayloadBuilder::new());
        let operation =
            OperationFuture::<Profile, _, _>::new(Arc::clone(&self.port), move |_, stores| {
                Provider::Storage::store(stores)
                    .insert::<Profile, Schema, Provider::Storage>(payload)
            });
        async move { super::AsyncHostExternalReturn::new(operation.await) }
    }
}

impl<'call, Profile: HostProfile> AsyncHostRequestContext<'call, Profile> {
    pub(crate) fn new(
        port: Arc<AsyncHostRequestPort<Profile>>,
        origin: HostCallOrigin,
        _scope: &'call (),
        lists: crate::runtime::TransferListStorage,
    ) -> Self {
        Self {
            port,
            origin,
            lists,
            lifetime: PhantomData,
        }
    }
}

impl<'call, Profile, Arguments, Return> AsyncHostCallable<'call, Profile, Arguments, Return>
where
    Profile: HostProfile,
{
    pub(crate) fn new<Function>(function: Function) -> Self
    where
        Function: ResumableCallback<Profile, Output = Return>,
        Return: Send + 'static,
    {
        Self {
            target: Box::new(move |inputs, origin, completion| {
                Box::new(CallbackRequest::new(function, origin, inputs, completion))
            }),
            lifetime: PhantomData,
            arguments: PhantomData,
        }
    }
}

impl<Profile: HostProfile> AsyncHostRequestPort<Profile> {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            requests: Mutex::new(VecDeque::new()),
        })
    }

    fn push_operation(&self, request: impl AsyncHostOperationRequest<Profile> + Send + 'static) {
        lock(&self.requests).push_back(AsyncHostRequest::Operation(Box::new(request)));
    }

    fn push_callback(&self, request: Box<dyn AsyncHostCallbackRequest<Profile> + Send>) {
        lock(&self.requests).push_back(AsyncHostRequest::Callback(request));
    }

    pub(crate) fn pop(&self) -> Option<AsyncHostRequest<Profile>> {
        lock(&self.requests).pop_front()
    }
}

impl<'request, Profile, Operation, Output> OperationFuture<'request, Profile, Operation, Output>
where
    Profile: HostProfile,
    Operation:
        FnOnce(&mut Profile::RunState, &mut Profile::ExternalStores) -> Output + Send + 'static,
    Output: Send + 'static,
{
    fn new(port: Arc<AsyncHostRequestPort<Profile>>, operation: Operation) -> Self {
        Self {
            port,
            state: OperationFutureState::Unqueued(operation),
            completion: Arc::new(Mutex::new(OperationCompletion {
                output: None,
                waker: Waker::noop().clone(),
            })),
            lifetime: PhantomData,
        }
    }
}

impl<Profile, Operation, Output> Future for OperationFuture<'_, Profile, Operation, Output>
where
    Profile: HostProfile,
    Operation:
        FnOnce(&mut Profile::RunState, &mut Profile::ExternalStores) -> Output + Send + 'static,
    Output: Send + 'static,
{
    type Output = Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        {
            let mut completion = lock(&this.completion);
            if let Some(output) = completion.output.take() {
                return Poll::Ready(output);
            }
            completion.waker = context.waker().clone();
        }

        let state = std::mem::replace(&mut this.state, OperationFutureState::Queued);
        if let OperationFutureState::Unqueued(operation) = state {
            this.port.push_operation(OperationRequest::<Profile, _, _> {
                operation,
                completion: Arc::downgrade(&this.completion),
                marker: PhantomData,
            });
        }
        Poll::Pending
    }
}

impl<Profile, Operation, Output> Unpin for OperationFuture<'_, Profile, Operation, Output> where
    Profile: HostProfile
{
}

impl<Profile, Operation, Output> AsyncHostOperationRequest<Profile>
    for OperationRequest<Profile, Operation, Output>
where
    Profile: HostProfile,
    Operation:
        FnOnce(&mut Profile::RunState, &mut Profile::ExternalStores) -> Output + Send + 'static,
    Output: Send + 'static,
{
    fn service(
        self: Box<Self>,
        state: &mut Profile::RunState,
        stores: &mut Profile::ExternalStores,
    ) {
        let Self {
            operation,
            completion,
            marker: _,
        } = *self;
        let Some(completion) = completion.upgrade() else {
            return;
        };
        let output = operation(state, stores);
        let waker = {
            let mut completion = lock(&completion);
            completion.output = Some(output);
            completion.waker.clone()
        };
        waker.wake();
    }
}

impl<Profile, Return> CallbackRequestFuture<Profile, Return>
where
    Profile: HostProfile,
{
    fn new(
        port: Arc<AsyncHostRequestPort<Profile>>,
        target: AsyncHostCallbackTarget<Profile, Return>,
        inputs: TransferInputs,
        origin: HostCallOrigin,
    ) -> Self {
        Self {
            port,
            state: CallbackRequestFutureState::Unqueued {
                target,
                inputs,
                origin,
            },
            completion: Arc::new(Mutex::new(AsyncHostCallbackCompletion {
                output: None,
                waker: Waker::noop().clone(),
            })),
        }
    }
}

impl<Profile, Return> Future for CallbackRequestFuture<Profile, Return>
where
    Profile: HostProfile,
    Return: Send + 'static,
{
    type Output = Result<Return, super::AsyncHostCallError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        {
            let mut completion = lock(&this.completion);
            if let Some(output) = completion.output.take() {
                return Poll::Ready(output);
            }
            completion.waker = context.waker().clone();
        }

        let state = std::mem::replace(&mut this.state, CallbackRequestFutureState::Queued);
        if let CallbackRequestFutureState::Unqueued {
            target,
            inputs,
            origin,
        } = state
        {
            let request = target(inputs, origin, Arc::downgrade(&this.completion));
            this.port.push_callback(request);
        }
        Poll::Pending
    }
}

impl<Profile, Return> Unpin for CallbackRequestFuture<Profile, Return> where Profile: HostProfile {}

impl<Output> AsyncHostCallbackCompletion<Output> {
    pub(crate) fn complete(
        completion: Arc<Mutex<Self>>,
        output: Result<Output, super::AsyncHostCallError>,
    ) {
        let waker = {
            let mut completion = lock(&completion);
            completion.output = Some(output);
            completion.waker.clone()
        };
        waker.wake();
    }
}

fn lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[cfg(test)]
mod tests {
    use super::{AsyncHostCall, AsyncHostRequest, AsyncHostRequestContext, AsyncHostRequestPort};
    use crate::runtime::HostCallOrigin;
    use crate::{HostProfile, HostProvider};
    use std::cell::Cell;
    use std::future::Future;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Profile;
    struct Provider;

    struct State {
        value: Cell<usize>,
    }

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

    struct WakeFlag(AtomicBool);

    impl Wake for WakeFlag {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    fn service_next(port: &AsyncHostRequestPort<Profile>, state: &mut State) -> bool {
        match port.pop() {
            Some(AsyncHostRequest::Operation(request)) => {
                request.service(state, &mut ());
                true
            }
            Some(AsyncHostRequest::Callback(_)) | None => false,
        }
    }

    #[test]
    fn services_owned_state_operations_in_fifo_order() {
        fn require_send<Value: Send>(value: Value) -> Value {
            value
        }

        let port = AsyncHostRequestPort::<Profile>::new();
        let mut call = AsyncHostCall::<Profile, Provider>::new(AsyncHostRequestContext::new(
            Arc::clone(&port),
            HostCallOrigin::Entry,
            &(),
            Default::default(),
        ));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let mut state = State {
            value: Cell::new(1),
        };

        let mut first = Box::pin(require_send(call.with_state(|state| {
            let value = state.value.get();
            state.value.set(value + 1);
            value
        })));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);
        assert_eq!(first.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(first.as_mut().poll(&mut context), Poll::Pending);
        assert!(service_next(&port, &mut state));
        assert!(!service_next(&port, &mut state));
        assert!(wake.0.swap(false, Ordering::SeqCst));
        assert_eq!(first.as_mut().poll(&mut context), Poll::Ready(1));
        drop(first);

        let mut second = Box::pin(call.with_state(|state| {
            let value = state.value.get();
            state.value.set(value * 3);
            value
        }));
        assert_eq!(second.as_mut().poll(&mut context), Poll::Pending);
        assert!(service_next(&port, &mut state));
        assert_eq!(second.as_mut().poll(&mut context), Poll::Ready(2));
        assert_eq!(state.value.get(), 6);
        assert!(!service_next(&port, &mut state));
    }

    #[test]
    fn poisoned_request_queue_recovers_without_a_host_error() {
        let port = AsyncHostRequestPort::<Profile>::new();
        let poisoned = Arc::clone(&port);
        let _ = std::panic::catch_unwind(move || {
            let _guard = poisoned.requests.lock().expect("request queue lock");
            panic!("poison request queue");
        });
        let mut call = AsyncHostCall::<Profile, Provider>::new(AsyncHostRequestContext::new(
            Arc::clone(&port),
            HostCallOrigin::Entry,
            &(),
            Default::default(),
        ));
        let mut request = Box::pin(call.with_state(|state| state.value.get()));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert_eq!(request.as_mut().poll(&mut context), Poll::Pending);
        let mut state = State {
            value: Cell::new(7),
        };
        assert!(service_next(&port, &mut state));
        assert_eq!(request.as_mut().poll(&mut context), Poll::Ready(7));
    }

    #[test]
    fn cancellation_drops_unserviced_operations_and_serviced_outputs_once() {
        fn set_one(state: &mut State) {
            state.value.set(1);
        }

        struct DropValue(Arc<AtomicUsize>);

        impl Drop for DropValue {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let port = AsyncHostRequestPort::<Profile>::new();
        let mut call = AsyncHostCall::<Profile, Provider>::new(AsyncHostRequestContext::new(
            Arc::clone(&port),
            HostCallOrigin::Entry,
            &(),
            Default::default(),
        ));
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(Arc::clone(&wake));
        let mut context = Context::from_waker(&waker);
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = State {
            value: Cell::new(0),
        };

        let mut cancelled = Box::pin(call.with_state(set_one));
        assert_eq!(cancelled.as_mut().poll(&mut context), Poll::Pending);
        drop(cancelled);
        assert!(service_next(&port, &mut state));
        assert_eq!(state.value.get(), 0);

        let mut applied = Box::pin(call.with_state(set_one));
        assert_eq!(applied.as_mut().poll(&mut context), Poll::Pending);
        assert!(service_next(&port, &mut state));
        assert_eq!(applied.as_mut().poll(&mut context), Poll::Ready(()));
        assert_eq!(state.value.get(), 1);
        drop(applied);

        let mut completed = Box::pin(call.with_state({
            let drops = Arc::clone(&drops);
            move |_state| DropValue(drops)
        }));
        let _ = completed.as_mut().poll(&mut context);
        assert!(service_next(&port, &mut state));
        drop(completed);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
