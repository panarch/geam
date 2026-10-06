use crate::execution::ExitStatus;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::work::Cancelled;
use crate::runtime::work::request::{Reply, Requests, Sender};
use std::future::Future;
use std::sync::{Arc, OnceLock};
use std::task::Context;

pub(in crate::runtime) struct Services<Plan: ExecutableRuntimePlan> {
    requests: Requests<Request<Plan>>,
    captures: crate::runtime::CaptureStorage,
    exit: Arc<OnceLock<ExitStatus>>,
}

pub(in crate::runtime) struct ServiceContext<Plan: ExecutableRuntimePlan> {
    requests: Sender<Request<Plan>>,
    captures: crate::runtime::CaptureStorage,
    unit: Option<crate::execution::ExecutionUnit>,
    exit: Arc<OnceLock<ExitStatus>>,
}

pub(in crate::runtime) struct Request<Plan: ExecutableRuntimePlan> {
    unit: Option<crate::execution::ExecutionUnit>,
    operation: Box<dyn Operation<Plan>>,
}

trait Operation<Plan: ExecutableRuntimePlan>: Send {
    fn allowance(&self) -> usize {
        1
    }

    fn apply(
        self: Box<Self>,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        allowance: usize,
    ) -> Option<Delivery>;
}

struct TypedOperation<Function, Output> {
    function: Function,
    reply: Reply<Output>,
}

struct BoundedOperation<Function, Output> {
    function: Function,
    reply: Reply<Output>,
    allowance: usize,
}

pub(in crate::runtime) struct Delivery(Box<dyn FnOnce() + Send>);

impl<Plan: ExecutableRuntimePlan> Services<Plan> {
    pub(in crate::runtime) fn new(captures: crate::runtime::CaptureStorage) -> Self {
        Self {
            requests: Requests::new(),
            captures,
            exit: Arc::default(),
        }
    }

    pub(in crate::runtime) fn context(&self) -> ServiceContext<Plan> {
        ServiceContext {
            requests: self.requests.sender(),
            captures: self.captures.clone(),
            unit: None,
            exit: Arc::clone(&self.exit),
        }
    }

    pub(super) fn next(&self, cx: &mut Context<'_>) -> Option<Request<Plan>> {
        self.requests.next(cx)
    }

    pub(super) fn close(&self) {
        self.requests.close();
    }

    pub(super) fn exit_status(&self) -> Option<ExitStatus> {
        self.exit.get().copied()
    }
}

impl<Plan: ExecutableRuntimePlan> Request<Plan> {
    pub(in crate::runtime) fn unit(&self) -> Option<&crate::execution::ExecutionUnit> {
        self.unit.as_ref()
    }

    pub(super) fn allowance(&self, available: usize) -> usize {
        self.operation.allowance().max(1).min(available)
    }

    pub(super) fn service(
        self,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        allowance: usize,
    ) -> Option<Delivery> {
        if self.unit.as_ref().is_some_and(|unit| !unit.is_active()) {
            return None;
        }
        self.operation.apply(plan, state, allowance)
    }
}

impl<Plan: ExecutableRuntimePlan> Clone for ServiceContext<Plan> {
    fn clone(&self) -> Self {
        Self {
            requests: self.requests.clone(),
            captures: self.captures.clone(),
            unit: self.unit.clone(),
            exit: Arc::clone(&self.exit),
        }
    }
}

impl<Plan: ExecutableRuntimePlan> ServiceContext<Plan> {
    pub(super) fn submit_bounded<Output, Function>(
        &self,
        allowance: usize,
        function: Function,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Plan, Output, Function>
    where
        Output: Send + 'static,
        Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>, usize) -> Output + Send + 'static,
    {
        let requests = self.requests.clone();
        let unit = self.unit.clone();
        async move {
            requests
                .submit(|reply| Request {
                    unit,
                    operation: Box::new(BoundedOperation {
                        function,
                        reply,
                        allowance,
                    }),
                })
                .await
        }
    }

    // Only an admitted native-result service records termination. It does not
    // wake while host borrows are live; the domain closes after dispatch returns.
    pub(in crate::runtime) fn request_exit(&self, status: ExitStatus) {
        let _ = self.exit.set(status);
    }

    pub(in crate::runtime) fn captures(&self) -> &crate::runtime::CaptureStorage {
        &self.captures
    }

    pub(super) fn same_domain(&self, other: &Self) -> bool {
        self.requests.same_queue(&other.requests)
    }

    pub(super) fn unit(&self) -> Option<&crate::execution::ExecutionUnit> {
        self.unit.as_ref()
    }

    pub(super) fn with_unit(&self, unit: Option<crate::execution::ExecutionUnit>) -> Self {
        Self {
            requests: self.requests.clone(),
            captures: self.captures.clone(),
            unit,
            exit: Arc::clone(&self.exit),
        }
    }

    pub(in crate::runtime) fn submit<Output, Function>(
        &self,
        function: Function,
    ) -> impl Future<Output = Result<Output, Cancelled>> + Send + use<Plan, Output, Function>
    where
        Output: Send + 'static,
        Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>) -> Output + Send + 'static,
    {
        let requests = self.requests.clone();
        let unit = self.unit.clone();
        async move {
            requests
                .submit(|reply| Request {
                    unit,
                    operation: Box::new(TypedOperation { function, reply }),
                })
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
        _allowance: usize,
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

impl<Plan, Function, Output> Operation<Plan> for BoundedOperation<Function, Output>
where
    Plan: ExecutableRuntimePlan,
    Function: FnOnce(&Plan, &mut RuntimeStateFor<'_, Plan>, usize) -> Output + Send,
    Output: Send + 'static,
{
    fn allowance(&self) -> usize {
        self.allowance
    }

    fn apply(
        self: Box<Self>,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        allowance: usize,
    ) -> Option<Delivery> {
        let Self {
            function, reply, ..
        } = *self;
        if reply.is_canceled() {
            return None;
        }
        let output = function(plan, state, allowance);
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

#[cfg(test)]
mod tests {
    use super::Services;
    use crate::execution::UnitOwner;
    use crate::runtime::state::RuntimeState;
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Waker};

    #[test]
    fn bounded_native_loop_requests_obey_the_service_limit_and_deliver_after_the_owner_borrow() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let services = Services::new(Default::default());
        let context = services.context();
        let mut cx = Context::from_waker(Waker::noop());
        for requested in [0, 1, 3, 1024] {
            for available in [1, 2, 7, 1024] {
                let mut result = pin!(context.submit_bounded(requested, |_, _, granted| granted));
                assert_eq!(result.as_mut().poll(&mut cx), Poll::Pending);
                let request = services.next(&mut cx).unwrap();
                let granted = request.allowance(available);
                assert_eq!(granted, requested.max(1).min(available));
                let delivery = {
                    let mut echo = Vec::new();
                    request
                        .service(&plan, &mut RuntimeState::new(&mut echo), granted)
                        .unwrap()
                };
                assert_eq!(result.as_mut().poll(&mut cx), Poll::Pending);
                delivery.deliver();
                assert_eq!(result.as_mut().poll(&mut cx), Poll::Ready(Ok(granted)));
            }
        }
        let mut ordinary = pin!(context.submit(|_, _| 42));
        assert_eq!(ordinary.as_mut().poll(&mut cx), Poll::Pending);
        let request = services.next(&mut cx).unwrap();
        assert_eq!(request.allowance(1024), 1);
        request
            .service(&plan, &mut RuntimeState::new(&mut Vec::new()), 1)
            .unwrap()
            .deliver();
        assert_eq!(ordinary.as_mut().poll(&mut cx), Poll::Ready(Ok(42)));
    }

    #[test]
    fn native_loop_requests_admit_the_body_only_while_the_waiter_and_unit_are_live() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let services = Services::<crate::ExecutionPlan>::new(Default::default());
        let (finished, _) = futures_channel::mpsc::unbounded();
        let owner = UnitOwner::new(finished);
        let unit = owner.handle();
        let context = services.context().with_unit(Some(unit.clone()));
        let calls = Arc::new(AtomicUsize::new(0));
        let mut cx = Context::from_waker(Waker::noop());
        let submit = || {
            let observed = Arc::clone(&calls);
            Box::pin(context.submit_bounded(1024, move |_, _, _| {
                observed.fetch_add(1, Ordering::SeqCst);
            }))
        };
        let mut admitted = submit();
        assert_eq!(admitted.as_mut().poll(&mut cx), Poll::Pending);
        services
            .next(&mut cx)
            .unwrap()
            .service(&plan, &mut RuntimeState::new(&mut Vec::new()), 1024)
            .unwrap()
            .deliver();
        assert_eq!(admitted.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        for cancel_unit in [false, true] {
            let mut result = submit();
            assert_eq!(result.as_mut().poll(&mut cx), Poll::Pending);
            let request = services.next(&mut cx).unwrap();
            if cancel_unit {
                assert!(unit.cancel());
            } else {
                drop(result);
            }
            assert!(
                request
                    .service(&plan, &mut RuntimeState::new(&mut Vec::new()), 1024)
                    .is_none()
            );
            assert_eq!(calls.load(Ordering::SeqCst), 1);
        }
    }
}
