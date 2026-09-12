use geam_core::host::{
    HostCall, HostCallContinuation, HostCallError, HostCallable, HostConstructions,
    HostFunctionType, HostOwnedCompletion, HostProviderModule, HostProviderSet, HostTupleType,
    HostTypeList, HostTypeListEnd,
};
use geam_core::{HostedExecution, compile_typed_host_project, plan_host_program};
use geam_erlang::{Component, Configuration, GleamErlangProfile, GleamErlangRunState};
use num_bigint::BigInt;
use std::task::Poll;
use std::time::Duration;

type One<T> = HostTypeList<T, HostTypeListEnd>;
type Pair = HostTupleType<HostTypeList<BigInt, One<BigInt>>>;

fn both<'call>(
    call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, Pair>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    callback: HostCallable<'call, One<BigInt>, BigInt>,
) -> Result<HostCallContinuation<'call, Pair>, HostCallError> {
    let callback = call.owned_callable(callback, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let (left, right) = futures_util::future::try_join(
                callback.invoke(&context, |_, _| (20.into(), ()), |_, _, value| Ok(value)),
                callback.invoke(&context, |_, _| (22.into(), ()), |_, _, value| Ok(value)),
            )
            .await?;
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_tuple((left, (right, ()))))
            }))
        })
    }))
}

#[test]
fn overlapping_ordinary_native_callbacks_share_the_callers_process_without_lost_waits() {
    let mut providers = geam_stdlib::host_providers::<GleamErlangProfile>().unwrap();
    providers.extend(geam_erlang::host_providers::<GleamErlangProfile>().unwrap());
    providers.push(HostProviderModule::new("geam_erlang_test", "callback_overlap")
        .unwrap()
        .with_resumable_function::<
            Component<GleamErlangProfile>,
            (HostFunctionType<One<BigInt>, BigInt>,),
            Pair,
            HostTypeListEnd,
            _,
        >("both", both).unwrap());
    let typed = compile_typed_host_project(
        super::project_root(),
        "callback_overlap",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let plan = plan_host_program(typed).unwrap();
    let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
    let mut state = GleamErlangRunState {
        stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
    };
    let host = super::execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    {
        let mut running = std::pin::pin!(execution.run_main(&host, &mut state, &mut echo));
        assert!(host.poll(running.as_mut()).is_pending());
        host.advance(Duration::from_millis(9));
        assert!(host.poll(running.as_mut()).is_pending());
        host.advance(Duration::from_millis(1));
        assert_eq!(
            host.poll(running.as_mut())
                .map(|result| result.unwrap().inspect().to_string()),
            Poll::Ready("#(20, 22)".into())
        );
    }
    assert!(echo.is_empty());
}
