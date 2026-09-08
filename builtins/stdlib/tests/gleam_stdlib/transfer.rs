use super::transfer_fixture::ObservedEcho;
use super::transfer_support::{Profile, RunState};
pub(super) use super::transfer_support::{assert_fixture, fixture};
use geam_core::{TransferHostProviderSet, compile_typed_transfer_host_project};
use geam_stdlib::GleamStdlibRunState;

#[test]
fn dict_callback_failure_preserves_source_origin_and_allows_the_next_call() {
    use geam_core::embedding::{
        AsyncCallError, FunctionDeclaration, WorkModuleBuilder, with_execution_scope,
    };
    use geam_core::{AsyncExecutionError, PanicMessage};
    use num_bigint::BigInt;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    let program = compile_typed_transfer_host_project(
        super::project_root(),
        "gleam_dict",
        TransferHostProviderSet::new(
            geam_stdlib::transfer_host_providers::<Profile>()
                .expect("stdlib transfer registration"),
        )
        .expect("stdlib provider set"),
    )
    .expect("official dict source linkage");
    let (mut bindings, map) = WorkModuleBuilder::new(program)
        .expect("dict plan")
        .function(FunctionDeclaration::<(bool,), BigInt>::new("map_probe"))
        .expect("typed probe");
    let fold = bindings
        .function(FunctionDeclaration::<(bool,), BigInt>::new("fold_probe"))
        .expect("fold probe");
    let update = bindings
        .function(FunctionDeclaration::<(bool,), BigInt>::new("update_probe"))
        .expect("update probe");
    let mut module = bindings.seal().expect("dict seal");
    let mut state = RunState {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        work: (),
    };
    let mut echo = ObservedEcho::default();
    let mut task = pin!(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        for (entry, message) in [
            (map, "map callback"),
            (fold, "fold callback"),
            (update, "update callback"),
        ] {
            let error = scope
                .call(&entry, (true,))
                .expect_err("nested source callback fails");
            assert!(
                matches!(error, AsyncCallError::Execution(AsyncExecutionError::Panic(ref error))
                if error.message() == &PanicMessage::Explicit(message.into())
                    && error.site().module() == "gleam_dict")
            );
            assert_eq!(
                scope.call(&entry, (false,)).expect("later valid call"),
                BigInt::from(42)
            );
        }
    }));
    assert!(matches!(
        task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(())
    ));
}
