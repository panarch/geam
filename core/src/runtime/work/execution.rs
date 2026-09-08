use super::request::{Reply, Requests, Sender};
use super::{Cancelled, Shared, Work, WorkFactory, WorkScope};
use crate::host::HostProfile;
use crate::plan::execution::TransferHostedExecution;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{
    StoredRuntimeValue, TransferCallable, TransferCallbackInputs, TransferValues,
};
use std::future::Future;
use std::task::Context;

pub(in crate::runtime) struct ExecutionWork<Profile: HostProfile> {
    work: WorkScope<Completion>,
    requests: Requests<Request<Profile>>,
}

pub(crate) struct WorkContext<Profile: HostProfile> {
    work: WorkFactory<Completion>,
    requests: Sender<Request<Profile>>,
}

pub(crate) type NativeCompletion =
    ExecutionResult<StoredRuntimeValue<TransferValues>, TransferValues>;
pub(crate) type Completion =
    Result<Shared<StoredRuntimeValue<TransferValues>>, Shared<crate::AsyncExecutionError>>;
pub(crate) type SourceWork = Work<Completion>;

pub(in crate::runtime) enum Request<Profile: HostProfile> {
    State(Box<dyn StateOperation<Profile>>),
    Runtime(Box<dyn RuntimeOperation<Profile>>),
    Callback {
        callable: TransferCallable,
        origin: HostCallOrigin,
        inputs: TransferCallbackInputs,
        reply: Reply<NativeCompletion>,
    },
}

pub(in crate::runtime) struct Delivery(Box<dyn FnOnce() + Send>);

pub(in crate::runtime) trait StateOperation<Profile: HostProfile>: Send {
    fn apply(self: Box<Self>, state: &mut Profile::RunState) -> Option<Delivery>;
}

struct StateRequest<Operation, Output> {
    operation: Operation,
    reply: Reply<Output>,
}

pub(in crate::runtime) trait RuntimeOperation<Profile: HostProfile>:
    Send
{
    fn apply(
        self: Box<Self>,
        plan: &TransferHostedExecution<Profile>,
        state: &mut RuntimeStateFor<'_, TransferHostedExecution<Profile>>,
    ) -> Option<Delivery>;
}

struct RuntimeRequest<Operation, Output> {
    operation: Operation,
    reply: Reply<Output>,
}

impl<Profile: HostProfile> ExecutionWork<Profile> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            work: WorkScope::new(),
            requests: Requests::new(),
        }
    }

    pub(in crate::runtime) fn context(&self) -> WorkContext<Profile> {
        WorkContext {
            work: self.work.factory(),
            requests: self.requests.sender(),
        }
    }

    pub(in crate::runtime) fn next(&self, cx: &mut Context<'_>) -> Option<Request<Profile>> {
        self.requests.next(cx)
    }
}

impl<Profile: HostProfile> WorkContext<Profile> {
    #[cfg(test)]
    pub(in crate::runtime) fn create(
        &self,
        native: impl Future<Output = NativeCompletion> + Send + 'static,
    ) -> SourceWork {
        self.compose(|_| async move {
            Ok(Shared::new(
                native.await.map(Shared::new).map_err(Shared::new),
            ))
        })
    }

    pub(crate) fn ready(&self, value: StoredRuntimeValue<TransferValues>) -> SourceWork {
        self.work.ready(Ok(Shared::new(value)))
    }

    pub(crate) fn map(
        &self,
        input: SourceWork,
        callable: TransferCallable,
        origin: HostCallOrigin,
    ) -> SourceWork {
        let context = self.clone();
        self.work.compose(|dependencies| async move {
            let completed = dependencies.observe(&input).await?;
            let value = completed.read(Clone::clone);
            match value {
                Ok(value) => {
                    let mut inputs = TransferCallbackInputs::new();
                    value.read(|value| inputs.push_value(value.value().clone()));
                    let result = context.invoke(callable, origin, inputs).await?;
                    Ok(Shared::new(result.map(Shared::new).map_err(Shared::new)))
                }
                Err(error) => Ok(Shared::new(Err(error))),
            }
        })
    }

    pub(super) fn compose<Native>(
        &self,
        build: impl FnOnce(super::Dependencies<Completion>) -> Native,
    ) -> SourceWork
    where
        Native: Future<Output = Result<Shared<Completion>, Cancelled>> + Send + 'static,
    {
        self.work.compose(build)
    }

    pub(crate) fn with_state<Output: Send + 'static, Operation>(
        &self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Profile, Output, Operation>
    where
        Operation: FnOnce(&mut Profile::RunState) -> Output + Send + 'static,
    {
        let requests = self.requests.clone();
        async move {
            requests
                .submit(|reply| Request::State(Box::new(StateRequest { operation, reply })))
                .await
        }
    }

    pub(in crate::runtime) fn invoke(
        &self,
        callable: TransferCallable,
        origin: HostCallOrigin,
        inputs: TransferCallbackInputs,
    ) -> impl Future<Output = Result<NativeCompletion, Cancelled>> + Send + use<Profile> {
        let requests = self.requests.clone();
        async move {
            requests
                .submit(|reply| Request::Callback {
                    callable,
                    origin,
                    inputs,
                    reply,
                })
                .await
        }
    }

    pub(in crate::runtime) fn with_runtime<Output: Send + 'static, Operation>(
        &self,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Profile, Output, Operation>
    where
        Operation: FnOnce(
                &TransferHostedExecution<Profile>,
                &mut RuntimeStateFor<'_, TransferHostedExecution<Profile>>,
            ) -> Output
            + Send
            + 'static,
    {
        let requests = self.requests.clone();
        async move {
            requests
                .submit(|reply| Request::Runtime(Box::new(RuntimeRequest { operation, reply })))
                .await
        }
    }
}

impl<Profile: HostProfile> Clone for WorkContext<Profile> {
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            requests: self.requests.clone(),
        }
    }
}

impl<Profile: HostProfile> Request<Profile> {
    pub(in crate::runtime) fn service(
        self,
        plan: &TransferHostedExecution<Profile>,
        state: &mut RuntimeStateFor<'_, TransferHostedExecution<Profile>>,
    ) -> Option<Delivery> {
        match self {
            Self::State(operation) => operation.apply(state.host_state()),
            Self::Runtime(operation) => operation.apply(plan, state),
            Self::Callback {
                callable,
                origin,
                inputs,
                reply,
            } => {
                if reply.is_canceled() {
                    return None;
                }
                let arguments = inputs.into_arguments();
                let output = callable
                    .with_value(|function| {
                        crate::runtime::function::invoke_callable(
                            plan, state, function, origin, arguments,
                        )
                    })
                    .map(|value| {
                        let type_ = value.value_type(plan.value_metadata());
                        StoredRuntimeValue::new(value, type_)
                    });
                Some(Delivery(Box::new(move || {
                    let _ = reply.send(output);
                })))
            }
        }
    }
}

impl<Profile, Operation, Output> StateOperation<Profile> for StateRequest<Operation, Output>
where
    Profile: HostProfile,
    Operation: FnOnce(&mut Profile::RunState) -> Output + Send,
    Output: Send + 'static,
{
    fn apply(self: Box<Self>, state: &mut Profile::RunState) -> Option<Delivery> {
        let Self { operation, reply } = *self;
        if reply.is_canceled() {
            return None;
        }
        let output = operation(state);
        Some(Delivery(Box::new(move || {
            let _ = reply.send(output);
        })))
    }
}

impl Delivery {
    pub(in crate::runtime) fn deliver(self) {
        (self.0)();
    }
}

impl<Profile, Operation, Output> RuntimeOperation<Profile> for RuntimeRequest<Operation, Output>
where
    Profile: HostProfile,
    Operation: FnOnce(
            &TransferHostedExecution<Profile>,
            &mut RuntimeStateFor<'_, TransferHostedExecution<Profile>>,
        ) -> Output
        + Send,
    Output: Send + 'static,
{
    fn apply(
        self: Box<Self>,
        plan: &TransferHostedExecution<Profile>,
        state: &mut RuntimeStateFor<'_, TransferHostedExecution<Profile>>,
    ) -> Option<Delivery> {
        let Self { operation, reply } = *self;
        if reply.is_canceled() {
            return None;
        }
        let result = operation(plan, state);
        Some(Delivery(Box::new(move || {
            let _ = reply.send(result);
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionWork, WorkContext};
    use crate::frontend::compile_typed_transfer_host_program;
    use crate::host::{HostProfile, TransferHostProviderModule, TransferHostProviderSet};
    use crate::plan::execution::TransferHostedExecution;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::{LibraryEntry, LibraryValueType, ValueType};
    use crate::runtime::evaluated::{EvaluatedFunctionValueKind, EvaluatedValue};
    use crate::runtime::function::{InvocableFunctionValue, run_tuple};
    use crate::runtime::state::{RuntimeState, TransferRuntimeHost};
    use crate::runtime::work::{Cancelled, Shared};
    use crate::runtime::{
        HostCallOrigin, TransferCallable, TransferCallbackInputs, TransferInputs,
    };
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Profile;

    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = ();
    }

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn callback_program(
        source: &str,
        providers: Vec<TransferHostProviderModule<Profile>>,
    ) -> (TransferHostedExecution<Profile>, TupleFunctionId) {
        let providers = TransferHostProviderSet::new(providers).expect("provider set");
        let typed = compile_typed_transfer_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            providers,
        )
        .expect("callback source");
        let plan =
            crate::planner::plan_transfer_host_library_program(typed).expect("callback plan");
        let entry = plan
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("source entry")
            .gleam_body()
            .expect("Gleam body");
        let entry = LibraryEntry::new(
            entry.id(),
            tuple_entry_type(entry.signature().shape().type_().return_().clone()),
            Vec::new(),
            Vec::new(),
        );
        let (execution, entries) =
            TransferHostedExecution::from_library_plan(plan, entry, Vec::new())
                .expect("sealed callback program");
        (execution, *entries.tuples[0].function())
    }

    fn tuple_entry_type(value: ValueType) -> LibraryValueType {
        let ValueType::Tuple(elements) = value else {
            panic!("fixture returns a tuple of callbacks");
        };
        LibraryValueType::Tuple(elements)
    }

    fn int_callback(
        plan: &TransferHostedExecution<Profile>,
        entry: TupleFunctionId,
        host: &mut Cell<usize>,
        echo: &mut Vec<crate::EchoOutput>,
        context: WorkContext<Profile>,
    ) -> TransferCallable {
        let mut runtime = RuntimeState::with_host(
            echo,
            TransferRuntimeHost::<Profile>::new(host, &(), context),
        );
        let values = run_tuple(
            plan,
            &mut runtime,
            entry,
            HostCallOrigin::Entry,
            TransferInputs::empty().into_retained(),
        )
        .expect("source constructs an owned closure");
        let [EvaluatedValue::Function(function)] = values.as_slice() else {
            panic!("fixture returns one function");
        };
        let EvaluatedFunctionValueKind::Int(function) = function.kind() else {
            panic!("fixture callback returns Int");
        };
        TransferCallable::new(InvocableFunctionValue::Int(function.clone()))
    }

    async fn invoke_after_state_request(
        context: WorkContext<Profile>,
        callback: TransferCallable,
    ) -> Result<super::NativeCompletion, Cancelled> {
        let previous = context
            .with_state(|state| {
                let previous = state.get();
                state.set(previous + 1);
                previous
            })
            .await?;
        let mut arguments = TransferCallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(previous.into()));
        context
            .invoke(callback, HostCallOrigin::Entry, arguments)
            .await
    }

    #[test]
    fn owned_callback_runs_after_its_creator_returns_using_the_original_state_and_echo() {
        let (plan, entry) = callback_program(
            r#"
    pub fn make() {
      let captured = 40
      #(fn(value: Int) {
        echo value
        captured + value
      })
    }
    "#,
            Vec::new(),
        );
        let mut host = Cell::new(2);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let context = execution.context();
        let native = invoke_after_state_request(context.clone(), callback);
        let work = context.compose(|_| async move {
            native
                .await
                .map(|result| Shared::new(result.map(Shared::new).map_err(Shared::new)))
        });
        let wake = Arc::new(WakeCount::default());
        let waker = Waker::from(wake.clone());
        let mut cx = Context::from_waker(&waker);
        let mut observer = pin!(work.observe());

        assert!(observer.as_mut().poll(&mut cx).is_pending());
        assert_eq!(host.get(), 2);
        let state_request = execution.next(&mut cx).expect("state request");
        let delivery = {
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut host, &(), execution.context()),
            );
            state_request
                .service(&plan, &mut runtime)
                .expect("live state request")
        };
        assert_eq!(host.get(), 3);
        let wakes_before_delivery = wake.0.load(Ordering::SeqCst);
        delivery.deliver();
        assert!(wake.0.load(Ordering::SeqCst) > wakes_before_delivery);
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        assert!(echo.is_empty());

        let callback_request = execution.next(&mut cx).expect("owned callback request");
        let delivery = {
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut host, &(), execution.context()),
            );
            callback_request
                .service(&plan, &mut runtime)
                .expect("live callback request")
        };
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(BigInt::from(2)));
        delivery.deliver();
        let result = completed_observation(observer.as_mut().poll(&mut cx));
        result.read(|result| {
            let value = result.as_ref().ok().expect("successful callback");
            value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42))));
        });

        drop(execution);
        let mut repeated = pin!(work.observe());
        let repeated = completed_observation(repeated.as_mut().poll(&mut cx));
        repeated.read(|result| {
            let value = result.as_ref().ok().expect("cached success");
            value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(BigInt::from(42))));
        });
        assert_eq!(host.get(), 3);
        assert_eq!(echo.len(), 1);
    }

    #[test]
    fn a_closed_execution_cancels_a_pending_callback_without_an_execution_error() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { value + 1 }) }",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let context = execution.context();
        let mut arguments = TransferCallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(41.into()));
        let mut request = pin!(context.invoke(callback, HostCallOrigin::Entry, arguments));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        drop(execution);
        assert_eq!(
            request.as_mut().poll(&mut cx).map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn closing_the_execution_cancels_a_retained_native_context_before_state_access() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { echo value value + 1 }) }",
            Vec::new(),
        );
        let mut host = Cell::new(41);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let mut request = Box::pin(invoke_after_state_request(execution.context(), callback));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        drop(execution);
        assert_eq!(
            request.as_mut().poll(&mut cx).map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert_eq!(host.get(), 41);
        assert!(echo.is_empty());
    }

    fn int_codec(
        plan: &TransferHostedExecution<Profile>,
        entry: crate::plan::execution::function::IntFunctionId,
    ) -> Option<crate::host::TransferHostCodecScope> {
        use crate::plan::execution::function::{ExecutionFunctionEntry, ExecutionFunctionRef};
        use crate::plan::execution::host::HostedFunctionTarget;
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        match plan.int_function(entry).as_ref() {
            ExecutionFunctionRef::Host(HostedFunctionTarget::Value(function)) => {
                Some(crate::host::TransferHostCodecScope::new(Arc::clone(
                    plan.host_value_function(function).metadata_handle(),
                )))
            }
            _ => None,
        }
    }

    #[test]
    fn owned_callback_requests_terminate_at_each_conversion_boundary() {
        use crate::host::{HostFutureError, HostProvider, TransferHostCall};
        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn increment<'call>(
            mut call: TransferHostCall<'call, Profile, Provider, BigInt>,
            value: BigInt,
        ) -> Result<crate::HostCallCompletion<'call, BigInt>, crate::AsyncHostCallError> {
            let next = call.state().get() + 1;
            call.state().set(next);
            Ok(call.return_value(value + 1))
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("module")
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>("increment", increment)
            .expect("native function");
        let typed = compile_typed_transfer_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "increment")
fn increment(value: Int) -> Int
pub fn plain(value: Int) { value }
pub fn make() { #(fn(value: Int) {
  echo value
  case value {
    -1 -> panic as "source rejected"
    _ -> increment(value)
  }
}) }
"#,
                )],
            )],
            TransferHostProviderSet::new([provider]).expect("providers"),
        )
        .expect("ordinary callback source");
        let library = crate::planner::plan_transfer_host_library_program(typed).expect("plan");
        let entry = |name: &str, return_| {
            let function = library
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .expect("declared function");
            LibraryEntry::new(function.signature().id(), return_, Vec::new(), Vec::new())
        };
        let make = library
            .functions()
            .iter()
            .find(|function| function.name() == "make")
            .expect("make");
        let return_type = tuple_entry_type(
            make.gleam_body()
                .expect("source body")
                .signature()
                .shape()
                .type_()
                .return_()
                .clone(),
        );
        let first = entry("make", return_type);
        let remaining = vec![
            entry("increment", LibraryValueType::Int),
            entry("plain", LibraryValueType::Int),
        ];
        let (plan, entries) = TransferHostedExecution::from_library_plan(library, first, remaining)
            .expect("sealed callbacks");
        let codec =
            int_codec(&plan, *entries.ints[0].function()).expect("source-selected native codec");
        assert!(int_codec(&plan, *entries.ints[1].function()).is_none());

        for (serviced, input, decode_fails) in [
            (0, 41, false),
            (1, 41, false),
            (2, 41, false),
            (3, 41, false),
            (3, -1, false),
            (3, 41, true),
        ] {
            let mut state = Cell::new(0);
            let mut echo = Vec::new();
            let execution = ExecutionWork::<Profile>::new();
            let callback = int_callback(
                &plan,
                *entries.tuples[0].function(),
                &mut state,
                &mut echo,
                execution.context(),
            );
            let context = execution.context();
            let mut invocation = Box::pin(context.invoke_owned(
                callback,
                codec.clone(),
                HostCallOrigin::Entry,
                move |_| {
                    let mut values = TransferCallbackInputs::new();
                    values.push_value(EvaluatedValue::Int(input.into()));
                    values
                },
                move |runtime, value| {
                    let value = runtime.int(value);
                    if decode_fails {
                        Err(crate::HostFailure::new("decoder rejected").into())
                    } else {
                        Ok(value)
                    }
                },
            ));
            let mut cx = Context::from_waker(Waker::noop());
            let mut result = invocation.as_mut().poll(&mut cx);
            for _ in 0..serviced {
                if result.is_ready() {
                    break;
                }
                let request = execution.next(&mut cx).expect("next conversion boundary");
                let mut runtime = RuntimeState::with_host(
                    &mut echo,
                    TransferRuntimeHost::<Profile>::new(&mut state, &(), execution.context()),
                );
                request
                    .service(&plan, &mut runtime)
                    .expect("live request")
                    .deliver();
                result = invocation.as_mut().poll(&mut cx);
            }
            if serviced < 3 {
                assert!(result.is_pending());
                drop(execution);
                result = invocation.as_mut().poll(&mut cx);
                assert_eq!(
                    result.map(|value| value.expect_err("closed endpoint").to_string()),
                    Poll::Ready(HostFutureError::Cancelled.to_string())
                );
            } else {
                if input == -1 {
                    assert_eq!(
                        result.map(|result| result
                            .expect_err("source panic")
                            .to_string()
                            .contains("source rejected")),
                        Poll::Ready(true)
                    );
                } else if decode_fails {
                    assert_eq!(
                        result.map(|result| result.expect_err("decoder failure").to_string()),
                        Poll::Ready("decoder rejected".to_owned())
                    );
                } else {
                    assert_eq!(
                        result.map(|result| result.expect("decoded completion")),
                        Poll::Ready(BigInt::from(42))
                    );
                }
            }
            assert_eq!(state.get(), usize::from(serviced >= 2 && input != -1));
            assert_eq!(echo.len(), usize::from(serviced >= 2));
        }
        for close in [false, true] {
            let execution = ExecutionWork::<Profile>::new();
            let context = execution.context();
            let value = Shared::new(crate::runtime::StoredRuntimeValue::new(
                EvaluatedValue::Int(42.into()),
                ValueType::Int,
            ));
            let mut decoded = Box::pin(context.decode_completion(
                value,
                codec.clone(),
                HostCallOrigin::Entry,
                |runtime, value| Ok(runtime.int(value)),
            ));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(decoded.as_mut().poll(&mut cx).is_pending());
            let mut state = Cell::new(0);
            let mut echo = Vec::new();
            if close {
                drop(execution);
            } else {
                let request = execution.next(&mut cx).expect("completion decoder request");
                let mut runtime = RuntimeState::with_host(
                    &mut echo,
                    TransferRuntimeHost::<Profile>::new(&mut state, &(), context),
                );
                request
                    .service(&plan, &mut runtime)
                    .expect("live completion decoder")
                    .deliver();
            }
            assert_eq!(
                decoded
                    .as_mut()
                    .poll(&mut cx)
                    .map(|value| value.map_err(|error| error.to_string())),
                Poll::Ready(if close {
                    Err(HostFutureError::Cancelled.to_string())
                } else {
                    Ok(BigInt::from(42))
                })
            );
            assert_eq!(state.get(), 0);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn an_abandoned_state_request_is_not_applied_after_the_driver_claims_it() {
        let (plan, _) =
            callback_program("pub fn make() { #(fn(value: Int) { value }) }", Vec::new());
        for cancel in [false, true] {
            let execution = ExecutionWork::<Profile>::new();
            let context = execution.context();
            let mut request = Box::pin(context.with_state(|state| state.set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(request.as_mut().poll(&mut cx).is_pending());
            let operation = execution.next(&mut cx).expect("claimed request");
            let receiver = if cancel {
                drop(request);
                None
            } else {
                Some(request)
            };
            let mut state = Cell::new(1);
            let mut echo = Vec::new();
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut state, &(), context),
            );
            let delivery = operation.service(&plan, &mut runtime);
            assert_eq!(delivery.is_none(), cancel);
            if let Some(delivery) = delivery {
                delivery.deliver();
            }
            if let Some(mut receiver) = receiver {
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
            }
            assert_eq!(state.get(), if cancel { 1 } else { 99 });
        }
    }

    #[test]
    fn a_claimed_runtime_request_executes_only_while_its_receiver_is_alive() {
        let (plan, _) =
            callback_program("pub fn make() { #(fn(value: Int) { value }) }", Vec::new());
        for cancel in [false, true] {
            let execution = ExecutionWork::<Profile>::new();
            let context = execution.context();
            let mut request = Box::pin(context.with_runtime(|_, state| state.host_state().set(99)));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(request.as_mut().poll(&mut cx).is_pending());
            let operation = execution.next(&mut cx).expect("claimed runtime request");
            let receiver = if cancel {
                drop(request);
                None
            } else {
                Some(request)
            };
            let mut state = Cell::new(1);
            let mut echo = Vec::new();
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut state, &(), context),
            );
            let delivery = operation.service(&plan, &mut runtime);
            assert_eq!(delivery.is_none(), cancel);
            if let Some(delivery) = delivery {
                delivery.deliver();
            }
            if let Some(mut receiver) = receiver {
                assert_eq!(receiver.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
            }
            assert_eq!(state.get(), if cancel { 1 } else { 99 });
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn source_work_forwards_shared_completion_without_replaying_native_work() {
        let execution = ExecutionWork::<Profile>::new();
        let context = execution.context();
        let polls = Arc::new(AtomicUsize::new(0));
        let work = context.create({
            let polls = polls.clone();
            async move {
                polls.fetch_add(1, Ordering::SeqCst);
                Ok(crate::runtime::StoredRuntimeValue::new(
                    EvaluatedValue::Int(42.into()),
                    ValueType::Int,
                ))
            }
        });
        let mut cx = Context::from_waker(Waker::noop());
        let forward =
            context.compose(|dependencies| async move { dependencies.observe(&work).await });
        let mut first = pin!(forward.observe());
        let value = completed_observation(first.as_mut().poll(&mut cx));
        let copied: Shared<super::Completion> = value.clone();
        copied.read(|value| assert!(value.is_ok()));
        assert_eq!(polls.load(Ordering::SeqCst), 1);

        let ready = context.ready(crate::runtime::StoredRuntimeValue::new(
            EvaluatedValue::Int(7.into()),
            ValueType::Int,
        ));
        drop(execution);
        let mut ready_observer = pin!(ready.observe());
        let value = completed_observation(ready_observer.as_mut().poll(&mut cx));
        value.read(|value| {
            let value = value.as_ref().ok().expect("ready success");
            value.read(|value| assert_eq!(value.value(), &EvaluatedValue::Int(7.into())));
        });
    }

    struct NativeProvider;

    impl crate::host::HostProvider<Profile> for NativeProvider {
        type State = Cell<usize>;

        fn project(state: &mut Cell<usize>) -> &mut Self::State {
            state
        }
    }

    fn fail_native<'call>(
        mut call: crate::host::TransferHostCall<'call, Profile, NativeProvider, BigInt>,
        value: BigInt,
    ) -> Result<crate::host::HostCallCompletion<'call, BigInt>, crate::AsyncHostCallError> {
        let state = call.state();
        state.set(state.get() + 1);
        Err(crate::HostFailure::new(format!("native rejected {value}")).into())
    }

    #[test]
    fn delayed_native_failure_keeps_its_provider_and_source_location_and_is_shared() {
        let native =
            TransferHostProviderModule::<Profile>::new_for_profile("application", "library")
                .expect("native module")
                .with_scoped_function::<NativeProvider, (BigInt,), BigInt, _>("native", fail_native)
                .expect("native function");
        let (plan, entry) = callback_program(
            "@external(erlang, \"native\", \"fail\")\nfn native(value: Int) -> Int\n\npub fn make() {\n  let captured = 40\n  #(fn(value: Int) {\n    echo captured\n    native(captured + value)\n  })\n}\n",
            vec![native],
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let context = execution.context();
        let mut arguments = TransferCallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(2.into()));
        let native = context.invoke(callback, HostCallOrigin::Entry, arguments);
        let work = context.compose(|_| async move {
            native
                .await
                .map(|result| Shared::new(result.map(Shared::new).map_err(Shared::new)))
        });
        let mut cx = Context::from_waker(Waker::noop());
        let mut observer = pin!(work.observe());
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        let request = execution.next(&mut cx).expect("callback request");
        let delivery = {
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut host, &(), execution.context()),
            );
            request.service(&plan, &mut runtime).expect("live receiver")
        };
        delivery.deliver();
        let completion = completed_observation(observer.as_mut().poll(&mut cx));
        completion.read(|result| {
            let error = result.as_ref().err().expect("native failure");
            error.read(|error| {
                let error = host_error(error);
                assert_eq!(error.package(), "application");
                assert_eq!(error.module(), "library");
                assert_eq!(error.function(), "native");
                assert_eq!(error.failure().message(), "native rejected 42");
                assert_eq!(
                    error.location().path().map(|path| path.as_str()),
                    Some("src/library.gleam")
                );
                assert_eq!(error.location().line(), Some(8));
            });
        });
        assert_eq!(host.get(), 1);
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(40.into()));
        drop(execution);
        let mut repeated = pin!(work.observe());
        assert_eq!(
            repeated.as_mut().poll(&mut cx).map(|result| result.is_ok()),
            Poll::Ready(true)
        );
        assert_eq!(host.get(), 1);
    }

    #[test]
    fn delayed_source_panic_is_not_relabelled_as_cancellation_or_native_failure() {
        let (plan, entry) = callback_program(
            "pub fn make() {\n  #(fn(value: Int) {\n    echo value\n    let assert True = value > 0\n    value\n  })\n}\n",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let context = execution.context();
        let mut arguments = TransferCallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int((-1).into()));
        let native = context.invoke(callback, HostCallOrigin::Entry, arguments);
        let work = context.compose(|_| async move {
            native
                .await
                .map(|result| Shared::new(result.map(Shared::new).map_err(Shared::new)))
        });
        let mut cx = Context::from_waker(Waker::noop());
        let mut observer = pin!(work.observe());
        assert!(observer.as_mut().poll(&mut cx).is_pending());
        let request = execution.next(&mut cx).expect("callback request");
        let delivery = {
            let mut runtime = RuntimeState::with_host(
                &mut echo,
                TransferRuntimeHost::<Profile>::new(&mut host, &(), execution.context()),
            );
            request.service(&plan, &mut runtime).expect("live receiver")
        };
        delivery.deliver();
        let completion = completed_observation(observer.as_mut().poll(&mut cx));
        completion.read(|result| {
            let error = result.as_ref().err().expect("failed assertion");
            error.read(|error| {
                let panic = source_panic(error);
                assert_eq!(panic.kind(), crate::PanicKind::LetAssert);
                assert_eq!(panic.site().module(), "library");
            });
        });
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int((-1).into()));
        assert_eq!(host.get(), 0);
    }

    #[test]
    fn dropping_a_claimed_callback_request_does_not_execute_its_source() {
        let (plan, entry) = callback_program(
            "pub fn make() { #(fn(value: Int) { echo value value }) }",
            Vec::new(),
        );
        let mut host = Cell::new(0);
        let mut echo = Vec::new();
        let execution = ExecutionWork::<Profile>::new();
        let callback = int_callback(&plan, entry, &mut host, &mut echo, execution.context());
        let context = execution.context();
        let mut arguments = TransferCallbackInputs::new();
        arguments.push_value(crate::runtime::EvaluatedValue::Int(42.into()));
        let mut request = Box::pin(context.invoke(callback, HostCallOrigin::Entry, arguments));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        let request_to_service = execution.next(&mut cx).expect("claimed request");
        drop(request);
        let mut runtime = RuntimeState::with_host(
            &mut echo,
            TransferRuntimeHost::<Profile>::new(&mut host, &(), execution.context()),
        );
        assert!(request_to_service.service(&plan, &mut runtime).is_none());
        drop(runtime);
        assert!(echo.is_empty());
    }

    fn completed_observation<Value>(poll: Poll<Result<Value, Cancelled>>) -> Value {
        match poll {
            Poll::Ready(value) => value.expect("live observation"),
            Poll::Pending => panic!("fixture observation is still pending"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture observation is still pending")]
    fn completed_observation_rejects_a_pending_fixture() {
        completed_observation::<()>(Poll::Pending);
    }

    #[test]
    #[should_panic(expected = "live observation")]
    fn completed_observation_rejects_a_cancelled_fixture() {
        completed_observation::<()>(Poll::Ready(Err(Cancelled)));
    }

    #[test]
    #[should_panic(expected = "fixture returns a tuple of callbacks")]
    fn callback_fixture_rejects_a_non_tuple_entry() {
        callback_program("pub fn make() { 42 }", Vec::new());
    }

    #[test]
    #[should_panic(expected = "fixture returns one function")]
    fn callback_fixture_rejects_a_non_function_tuple() {
        let (plan, entry) = callback_program("pub fn make() { #(42) }", Vec::new());
        let work = ExecutionWork::<Profile>::new();
        int_callback(
            &plan,
            entry,
            &mut Cell::new(0),
            &mut Vec::new(),
            work.context(),
        );
    }

    #[test]
    #[should_panic(expected = "fixture callback returns Int")]
    fn callback_fixture_rejects_another_return_family() {
        let (plan, entry) =
            callback_program("pub fn make() { #(fn(_value: Int) { Nil }) }", Vec::new());
        let work = ExecutionWork::<Profile>::new();
        int_callback(
            &plan,
            entry,
            &mut Cell::new(0),
            &mut Vec::new(),
            work.context(),
        );
    }
    fn host_error(error: &crate::AsyncExecutionError) -> &crate::HostError {
        match error {
            crate::AsyncExecutionError::Host(error) => error,
            _ => panic!("fixture must fail in a host provider"),
        }
    }

    fn source_panic(error: &crate::AsyncExecutionError) -> &crate::Panic<crate::AsyncPanicValue> {
        match error {
            crate::AsyncExecutionError::Panic(error) => error,
            _ => panic!("fixture must stop with a source panic"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture must fail in a host provider")]
    fn host_error_rejects_an_invariant_fixture() {
        host_error(&crate::AsyncExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: crate::ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }

    #[test]
    #[should_panic(expected = "fixture must stop with a source panic")]
    fn source_panic_rejects_an_invariant_fixture() {
        source_panic(&crate::AsyncExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: crate::ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }
}
