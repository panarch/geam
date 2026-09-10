use geam::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder, HostedProject};
use geam::execution::TokioHost;
use geam::{EchoOutput, HostProfile, HostProviderSet};

struct Profile;
impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = ();
    type ExecutionState = ();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let project = HostedProject::<Profile>::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/gleam"),
        "geam_rust_embedding_execution",
        || HostProviderSet::from_providers([]),
    );
    let (mut bindings, spin) = HostedModuleBuilder::new(project.compile()?)?
        .function(FunctionDeclaration::<(), BigInt>::new("spin"))?;
    let double = bindings.function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))?;
    let mut module = bindings.seal()?;
    let (started, ready) = tokio::sync::oneshot::channel();
    let mut started = Some(started);
    let mut echo = move |_: EchoOutput| {
        if let Some(started) = started.take() {
            let _ = started.send(());
        }
    };

    executor.block_on(module.with_execution(&host, &mut (), &mut echo, async |scope| {
        let mut spinning = Box::pin(scope.call(&spin, ()));
        tokio::select! {
            result = &mut spinning => return Err(format!("spin returned unexpectedly: {result:?}").into()),
            ready = ready => ready?,
        }
        println!("Rust made progress while Gleam was running");
        drop(spinning);
        let value = scope.call(&double, (21.into(),)).await?;
        println!("after cancellation: {value}");
        Ok::<_, Box<dyn std::error::Error>>(())
    }))?
}
