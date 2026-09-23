use super::{
    ExecutionClock, ExecutionMetadata, ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit,
};
use std::task::{Context, Poll};

/// Two statically selected service owners in one execution domain.
///
/// Nest the remaining services in `rest`, ending with `()`. A concrete host
/// projects each unique service directly from these fields. Both owners receive
/// every lifecycle event and one bounded poll, even when the first makes progress.
#[derive(Default)]
pub struct ExecutionServices<First, Rest> {
    pub first: First,
    pub rest: Rest,
}

impl<First: HostExecutionState, Rest: HostExecutionState> HostExecutionState
    for ExecutionServices<First, Rest>
{
    fn initialize(&mut self, metadata: ExecutionMetadata<'_>) {
        self.first.initialize(metadata);
        self.rest.initialize(metadata);
    }

    fn started(&mut self, unit: ExecutionUnit) {
        self.first.started(unit.clone());
        self.rest.started(unit);
    }

    fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit) {
        self.first.finished(unit, exit);
        self.rest.finished(unit, exit);
    }

    fn close(&mut self) {
        self.first.close();
        self.rest.close();
    }

    fn poll(&mut self, cx: &mut Context<'_>, clock: ExecutionClock<'_>) -> Poll<()> {
        let first = self.first.poll(cx, clock);
        let rest = self.rest.poll(cx, clock);
        if first.is_ready() || rest.is_ready() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionServices;
    use crate::execution::{
        ExecutionClock, ExecutionHost, ExecutionMetadata, ExecutionUnit, ExecutionUnitId,
        HostExecutionState, UnitExit,
    };
    use crate::execution_fixture::TestHost;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};
    use std::time::Instant;

    #[derive(Default)]
    struct Service {
        events: Arc<Mutex<Vec<String>>>,
        label: &'static str,
        ready: bool,
        polls: usize,
        now: Option<Instant>,
        waiter: Option<Waker>,
    }

    impl HostExecutionState for Service {
        fn initialize(&mut self, metadata: ExecutionMetadata<'_>) {
            let tags = metadata.native_constructor_tags().collect::<Vec<_>>();
            self.events
                .lock()
                .unwrap()
                .push(format!("{} initialize {tags:?}", self.label));
        }

        fn started(&mut self, unit: ExecutionUnit) {
            assert!(unit.is_active());
            self.events
                .lock()
                .unwrap()
                .push(format!("{} started", self.label));
        }

        fn finished(&mut self, _: ExecutionUnitId, exit: &UnitExit) {
            self.events
                .lock()
                .unwrap()
                .push(format!("{} finished {exit:?}", self.label));
        }

        fn close(&mut self) {
            self.events
                .lock()
                .unwrap()
                .push(format!("{} close", self.label));
        }

        fn poll(&mut self, cx: &mut Context<'_>, clock: ExecutionClock<'_>) -> Poll<()> {
            self.polls += 1;
            self.now = Some(clock.now());
            self.waiter = Some(cx.waker().clone());
            if self.ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    #[test]
    fn every_service_polls_and_registers_its_wake_despite_earlier_progress() {
        let host = TestHost::default();
        let clock = ExecutionClock::new(&host);
        let mut cx = Context::from_waker(Waker::noop());
        for (first, rest, expected) in [
            (false, false, Poll::Pending),
            (false, true, Poll::Ready(())),
            (true, false, Poll::Ready(())),
            (true, true, Poll::Ready(())),
        ] {
            let mut services = ExecutionServices::<Service, Service>::default();
            services.first.ready = first;
            services.rest.ready = rest;
            assert_eq!(services.poll(&mut cx, clock), expected);
            assert_eq!((services.first.polls, services.rest.polls), (1, 1));
            assert_eq!(services.first.now, Some(host.now()));
            assert_eq!(services.rest.now, Some(host.now()));
            assert!(
                services
                    .first
                    .waiter
                    .as_ref()
                    .unwrap()
                    .will_wake(cx.waker())
            );
            assert!(services.rest.waiter.as_ref().unwrap().will_wake(cx.waker()));
        }
    }

    #[test]
    fn nested_services_observe_each_domain_lifecycle_once_in_declared_order() {
        struct Profile;
        impl crate::HostProfile for Profile {
            type RunState = Arc<Mutex<Vec<String>>>;
            type ExternalStores = ();
            type ExecutionState = ExecutionServices<Service, ExecutionServices<Service, ()>>;

            fn initialize_execution(events: &mut Self::RunState) -> Self::ExecutionState {
                ExecutionServices {
                    first: Service {
                        events: Arc::clone(events),
                        label: "first",
                        ..Service::default()
                    },
                    rest: ExecutionServices {
                        first: Service {
                            events: Arc::clone(events),
                            label: "second",
                            ..Service::default()
                        },
                        rest: (),
                    },
                }
            }
        }
        let program = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub type State { Ready } pub fn main() { Ready }",
                )],
            )],
            crate::HostProviderSet::<Profile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let mut execution = crate::HostedExecution::try_from_module_plan(
            crate::plan_host_program(program).unwrap(),
        )
        .unwrap();
        let host = TestHost::default();
        let mut events = Arc::new(Mutex::new(Vec::new()));
        for _ in 0..2 {
            let result = host
                .block_on(execution.run_main(&host, &mut events, &mut Vec::new()))
                .unwrap();
            assert_eq!(result.inspect().to_string(), "Ready");
            assert_eq!(
                *events.lock().unwrap(),
                [
                    "first initialize [\"ready\"]",
                    "second initialize [\"ready\"]",
                    "first started",
                    "second started",
                    "first finished Completed",
                    "second finished Completed",
                    "first close",
                    "second close",
                ]
            );
            events.lock().unwrap().clear();
        }
    }
}
