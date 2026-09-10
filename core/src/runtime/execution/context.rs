use super::service::Request as ServiceRequest;
use super::{ServiceContext, Services};
use crate::host::HostProfile;
use crate::plan::execution::HostedProgram;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::work::Cancelled;
use crate::runtime::work::request::{Reply, Requests, Sender};
use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable, StoredRuntimeValue};
use std::future::Future;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Context;

pub(in crate::runtime) struct ExecutionServices<Profile: HostProfile> {
    initialized: OnceLock<Initialized<Profile>>,
}

struct Initialized<Profile: HostProfile> {
    services: Services<HostedProgram<Profile>>,
    callbacks: Requests<CallbackRequest>,
    callback_first: AtomicBool,
}

pub(crate) struct ExecutionContext<Profile: HostProfile> {
    services: ServiceContext<HostedProgram<Profile>>,
    callbacks: Sender<CallbackRequest>,
}

pub(crate) type NativeCompletion = crate::runtime::error::ExecutionResult<StoredRuntimeValue>;

pub(in crate::runtime) struct CallbackRequest {
    callable: RetainedCallable,
    origin: HostCallOrigin,
    inputs: CallbackInputs,
    reply: Reply<NativeCompletion>,
}

pub(in crate::runtime) enum Request<Profile: HostProfile> {
    Service(ServiceRequest<HostedProgram<Profile>>),
    Callback(CallbackRequest),
}

impl<Profile: HostProfile> ExecutionServices<Profile> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            initialized: OnceLock::new(),
        }
    }

    pub(in crate::runtime) fn context(&self) -> ExecutionContext<Profile> {
        let initialized = self.initialized.get_or_init(|| Initialized {
            services: Services::new(),
            callbacks: Requests::new(),
            callback_first: AtomicBool::new(false),
        });
        ExecutionContext {
            services: initialized.services.context(),
            callbacks: initialized.callbacks.sender(),
        }
    }

    pub(in crate::runtime) fn next(&self, cx: &mut Context<'_>) -> Option<Request<Profile>> {
        let initialized = self.initialized.get()?;
        if initialized
            .callback_first
            .fetch_xor(true, Ordering::Relaxed)
        {
            initialized
                .callbacks
                .next(cx)
                .map(Request::Callback)
                .or_else(|| initialized.services.next(cx).map(Request::Service))
        } else {
            initialized
                .services
                .next(cx)
                .map(Request::Service)
                .or_else(|| initialized.callbacks.next(cx).map(Request::Callback))
        }
    }

    pub(in crate::runtime) fn close(&self) {
        if let Some(initialized) = self.initialized.get() {
            initialized.services.close();
            initialized.callbacks.close();
        }
    }
}

impl<Profile: HostProfile> Clone for ExecutionContext<Profile> {
    fn clone(&self) -> Self {
        Self {
            services: self.services.clone(),
            callbacks: self.callbacks.clone(),
        }
    }
}

impl<Profile: HostProfile> ExecutionContext<Profile> {
    pub(in crate::runtime) fn services(&self) -> &ServiceContext<HostedProgram<Profile>> {
        &self.services
    }

    pub(crate) fn with_state<Output: Send + 'static, Function>(
        &self,
        function: Function,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Profile, Output, Function>
    where
        Function: FnOnce(&mut Profile::RunState) -> Output + Send + 'static,
    {
        self.services
            .submit(move |_, state| function(state.host_state()))
    }

    pub(in crate::runtime) fn with_runtime<Output: Send + 'static, Function>(
        &self,
        function: Function,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Profile, Output, Function>
    where
        Function: FnOnce(
                &HostedProgram<Profile>,
                &mut RuntimeStateFor<'_, HostedProgram<Profile>>,
            ) -> Output
            + Send
            + 'static,
    {
        self.services.submit(function)
    }

    pub(in crate::runtime) fn invoke(
        &self,
        callable: RetainedCallable,
        origin: HostCallOrigin,
        inputs: CallbackInputs,
    ) -> impl Future<Output = Result<NativeCompletion, Cancelled>> + Send + use<Profile> {
        let callbacks = self.callbacks.clone();
        async move {
            callbacks
                .submit(|reply| CallbackRequest {
                    callable,
                    origin,
                    inputs,
                    reply,
                })
                .await
        }
    }
}

impl CallbackRequest {
    pub(super) fn into_worker<Profile: HostProfile>(
        self,
        plan: std::sync::Arc<HostedProgram<Profile>>,
        context: ExecutionContext<Profile>,
        budget: std::num::NonZeroUsize,
    ) -> crate::execution::Worker {
        let Self {
            callable,
            origin,
            inputs,
            reply,
        } = self;
        super::worker::completing(reply, async move {
            let invocation = callable.with_value(|function| {
                crate::runtime::function::prepare_callable(
                    plan.as_ref(),
                    function,
                    origin,
                    inputs.into_arguments(),
                )
            });
            invocation
                .submit(context.services(), budget)
                .await
                .map(|result| {
                    result.map(|value| StoredRuntimeValue::new(value, plan.value_metadata()))
                })
        })
    }
}
