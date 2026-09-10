use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::work::Cancelled;
use crate::runtime::work::request::{Reply, Requests, Sender};
use std::future::Future;
use std::task::Context;

pub(in crate::runtime) struct Services<Plan: ExecutableRuntimePlan> {
    requests: Requests<Request<Plan>>,
}

pub(in crate::runtime) struct ServiceContext<Plan: ExecutableRuntimePlan> {
    requests: Sender<Request<Plan>>,
}

pub(in crate::runtime) struct Request<Plan: ExecutableRuntimePlan>(Box<dyn Operation<Plan>>);

trait Operation<Plan: ExecutableRuntimePlan>: Send {
    fn apply(
        self: Box<Self>,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
    ) -> Option<Delivery>;
}

struct TypedOperation<Function, Output> {
    function: Function,
    reply: Reply<Output>,
}

pub(in crate::runtime) struct Delivery(Box<dyn FnOnce() + Send>);

impl<Plan: ExecutableRuntimePlan> Services<Plan> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub(in crate::runtime) fn context(&self) -> ServiceContext<Plan> {
        ServiceContext {
            requests: self.requests.sender(),
        }
    }

    pub(super) fn next(&self, cx: &mut Context<'_>) -> Option<Request<Plan>> {
        self.requests.next(cx)
    }

    pub(super) fn close(&self) {
        self.requests.close();
    }
}

impl<Plan: ExecutableRuntimePlan> Request<Plan> {
    pub(in crate::runtime) fn service(
        self,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
    ) -> Option<Delivery> {
        self.0.apply(plan, state)
    }
}

impl<Plan: ExecutableRuntimePlan> Clone for ServiceContext<Plan> {
    fn clone(&self) -> Self {
        Self {
            requests: self.requests.clone(),
        }
    }
}

impl<Plan: ExecutableRuntimePlan> ServiceContext<Plan> {
    pub(in crate::runtime) fn submit<Output, Function>(
        &self,
        function: Function,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Plan, Output, Function>
    where
        Output: Send + 'static,
        Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>) -> Output + Send + 'static,
    {
        let requests = self.requests.clone();
        async move {
            requests
                .submit(|reply| Request(Box::new(TypedOperation { function, reply })))
                .await
        }
    }
}

impl<Plan, Function, Output> Operation<Plan> for TypedOperation<Function, Output>
where
    Plan: ExecutableRuntimePlan,
    Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>) -> Output + Send,
    Output: Send + 'static,
{
    fn apply(
        self: Box<Self>,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
    ) -> Option<Delivery> {
        let Self { function, reply } = *self;
        if reply.is_canceled() {
            return None;
        }
        let output = function(plan, state);
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
