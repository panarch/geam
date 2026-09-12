use super::transfer_fixture::ObservedEcho;
use super::transfer_support::{Profile, RunState};
pub(super) use super::transfer_support::{assert_fixture, fixture};
use geam_core::{HostProviderSet, compile_typed_host_project};
use geam_stdlib::GleamStdlibRunState;

#[test]
fn dict_callback_failure_preserves_source_origin_and_allows_the_next_call() {
    let execution_host = crate::execution_fixture::TestHost::default();

    use geam_core::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
    use geam_core::{ExecutionError, PanicMessage};
    use num_bigint::BigInt;
    use std::pin::pin;
    use std::task::Poll;

    let program = compile_typed_host_project(
        super::project_root(),
        "gleam_dict",
        HostProviderSet::from_providers(
            geam_stdlib::host_providers::<Profile>().expect("stdlib transfer registration"),
        )
        .expect("stdlib provider set"),
    )
    .expect("official dict source linkage");
    let (mut bindings, map) = HostedModuleBuilder::new(program)
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
    let mut task =
        pin!(
            module.with_execution(&execution_host, &mut state, &mut echo, async |scope| {
                for (entry, message) in [
                    (map, "map callback"),
                    (fold, "fold callback"),
                    (update, "update callback"),
                ] {
                    let error = scope
                        .call(&entry, (true,))
                        .await
                        .expect_err("nested source callback fails");
                    assert!(
                        matches!(error, CallError::Execution(ExecutionError::Panic(ref error))
                if error.message() == &PanicMessage::Explicit(message.into())
                    && error.site().module() == "gleam_dict")
                    );
                    assert_eq!(
                        scope
                            .call(&entry, (false,))
                            .await
                            .expect("later valid call"),
                        BigInt::from(42)
                    );
                }
            })
        );
    assert!(matches!(
        execution_host.poll(task.as_mut()),
        Poll::Ready(Ok(()))
    ));
}
