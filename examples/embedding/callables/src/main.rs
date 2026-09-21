mod callbacks;
mod declarations;
// The generated default profile remains available; this app supplies its own.
#[allow(dead_code)]
mod geam_bindings;
mod pricing;

use geam::embedding::{BigInt, HostedModuleBuilder};
use geam::execution::TokioHost;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let mut executions = Vec::new();
    if std::env::args().nth(1).as_deref() != Some("--prepared") {
        let program = geam_bindings::project(callbacks::providers).compile()?;
        let (mut dynamic, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
        let factory = dynamic.callable::<declarations::AddOffset>()?;
        executions.push(("dynamic", dynamic.seal()?, functions, factory));
    }
    let (mut prepared, prepared_functions) = geam_bindings::load(callbacks::providers()?)?;
    let prepared_factory = prepared.callable::<declarations::AddOffset>()?;
    executions.push((
        "prepared",
        prepared.seal(),
        prepared_functions,
        prepared_factory,
    ));
    for (mode, mut module, functions, factory) in executions {
        let mut state = pricing::Pricing::default();
        let mut echo = Vec::new();
        let (first, second, third) = executor.block_on(module.with_execution(
            &host,
            &mut state,
            &mut echo,
            async |scope| {
                let callback = scope.construct(&factory, (BigInt::from(10), ()))?;
                let first = scope
                    .call(&functions.calculate, (BigInt::from(5), &callback))
                    .await?;
                let alias = scope.call(&functions.keep, (&callback,)).await?;
                let second = scope.invoke(&alias, (BigInt::from(7),)).await?;
                let saved = scope.call(&functions.save, (&callback,)).await?;
                let restored = scope.call(&functions.restore, (&saved,)).await?;
                let wrapped = scope.call(&functions.wrapped, (&restored,)).await?;
                let third = scope.invoke(&wrapped, (BigInt::from(9),)).await?;
                Ok::<_, geam::embedding::CallError>((first, second, third))
            },
        ))??;
        assert_eq!(state.calls.get(), 3);
        assert!(echo.is_empty());
        println!("{mode}: {first}, {second}, {third}");
    }
    Ok(())
}
