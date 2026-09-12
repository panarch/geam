use super::{Clock, Configuration, NativeState, State, providers};
use geam::embedding::{
    BigInt, CustomType, FunctionDeclaration, FutureType, HostedModuleBuilder, NamedTypeSchema,
};
use geam::gleam_stdlib::{GleamStdlibRunState, IoOutput};
use std::cell::Cell;
use std::task::Poll;
use std::time::Duration;

struct Service;

impl NamedTypeSchema for Service {
    const PACKAGE: &'static str = "geam_future_builtins_test";
    const MODULE: &'static str = "process_work";
    const NAME: &'static str = "Service";
}

#[test]
fn shared_work_outlives_its_creator_and_one_abandoned_observer() {
    let program = geam::frontend::compile_typed_host_project(
        super::project_root(),
        "process_work",
        providers(),
    )
    .unwrap();
    let (bindings, create) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "from_exited_creator",
        ))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let host = super::execution_fixture::TestHost::default();
    let (abandon, cancelled) = futures_channel::oneshot::channel();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        clock: Clock(Cell::new(100)),
        native: NativeState::default(),
        work: (),
        json: (),
        erlang: Configuration::default(),
    };
    let mut echo = Vec::new();
    let mut running =
        Box::pin(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let work = scope.call(&create, (20.into(),)).await.unwrap();
                let observer = Box::pin(scope.observe(&work));
                let futures_util::future::Either::Right((Ok(()), observer)) =
                    futures_util::future::select(observer, cancelled).await
                else {
                    panic!("the callback must still be waiting when its observer is dropped");
                };
                drop(observer);
                let alias = work.clone();
                let (left, right) =
                    futures_util::future::join(scope.observe(&work), scope.observe(&alias)).await;
                let left = left.unwrap();
                let right = right.unwrap();
                left.read(|value| right.read(|other| assert!(std::ptr::eq(value, other))));
                left.read(Clone::clone)
            }),
        );
    assert!(host.poll(running.as_mut()).is_pending());
    abandon.send(()).unwrap();
    assert!(host.poll(running.as_mut()).is_pending());
    host.advance(Duration::from_millis(9));
    assert!(host.poll(running.as_mut()).is_pending());
    host.advance(Duration::from_millis(1));
    assert_eq!(
        host.poll(running.as_mut()).map(Result::unwrap),
        Poll::Ready(BigInt::from(42))
    );
    drop(running);
    assert_eq!(
        state
            .stdlib
            .io_outputs()
            .iter()
            .map(IoOutput::text)
            .collect::<Vec<_>>(),
        ["callback\n"]
    );
    assert!(echo.is_empty());
}

#[test]
fn concurrent_deferred_callbacks_have_independent_processes_and_subjects() {
    let program = geam::frontend::compile_typed_host_project(
        super::project_root(),
        "process_work",
        providers(),
    )
    .unwrap();
    let (bindings, create) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), FutureType<BigInt>>::new(
            "concurrent",
        ))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let host = super::execution_fixture::TestHost::default();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        clock: Clock(Cell::new(100)),
        native: NativeState::default(),
        work: (),
        json: (),
        erlang: Configuration::default(),
    };
    let mut echo = Vec::new();
    let mut running =
        Box::pin(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let work = scope.call(&create, ()).await.unwrap();
                scope.observe(&work).await.unwrap().read(Clone::clone)
            }),
        );
    assert!(host.poll(running.as_mut()).is_pending());
    host.advance(Duration::from_millis(10));
    assert_eq!(
        host.poll(running.as_mut()).map(Result::unwrap),
        Poll::Ready(BigInt::from(42))
    );
    drop(running);
    assert!(state.stdlib.io_outputs().is_empty());
    assert!(echo.is_empty());
}

#[test]
fn last_work_owner_cancels_its_callback_but_not_an_independent_service() {
    let program = geam::frontend::compile_typed_host_project(
        super::project_root(),
        "process_work",
        providers(),
    )
    .unwrap();
    let (mut bindings, create) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<
            (),
            (CustomType<Service>, FutureType<BigInt>),
        >::new("cancellable"))
        .unwrap();
    let status = bindings
        .function(FunctionDeclaration::<(CustomType<Service>,), bool>::new(
            "status",
        ))
        .unwrap();
    let cancelled = bindings
        .function(FunctionDeclaration::<(CustomType<Service>,), bool>::new(
            "await_cancellation",
        ))
        .unwrap();
    let stop = bindings
        .function(FunctionDeclaration::<(CustomType<Service>,), ()>::new(
            "stop",
        ))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let host = super::execution_fixture::TestHost::default();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        clock: Clock(Cell::new(100)),
        native: NativeState::default(),
        work: (),
        json: (),
        erlang: Configuration::default(),
    };
    let mut echo = Vec::new();
    let mut running = Box::pin(module.with_execution(
        &host,
        &mut state,
        &mut echo,
        async |scope| {
            let (service, work) = scope.call(&create, ()).await.unwrap();
            let observer = Box::pin(scope.observe(&work));
            let ready = Box::pin(scope.call(&status, (&service,)));
            let futures_util::future::Either::Right((Ok(false), observer)) =
                futures_util::future::select(observer, ready).await
            else {
                panic!("the service must monitor the pending callback before cancelling its work");
            };
            drop(observer);
            assert!(!scope.call(&status, (&service,)).await.unwrap());
            drop(work);
            assert!(scope.call(&cancelled, (&service,)).await.unwrap());
            assert!(scope.call(&status, (&service,)).await.unwrap());
            scope.call(&stop, (service,)).await.unwrap();
        },
    ));
    assert_eq!(
        host.poll(running.as_mut()).map(Result::unwrap),
        Poll::Ready(())
    );
    drop(running);
    assert!(state.stdlib.io_outputs().is_empty());
    assert!(echo.is_empty());
}
