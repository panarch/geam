mod geam_bindings;

use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let program = geam_bindings::project().compile()?;
    let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
    let mut module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {}.initialize();
    let mut echo = Vec::new();

    executor.block_on(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            let original = scope.call(&functions.start, (40.into(),)).await?;
            let alias = original.clone();
            let next = scope.call(&functions.next, (&original,)).await?;
            let before = scope.call(&functions.total, (alias,)).await?;
            let after = scope.call(&functions.total, (next,)).await?;
            println!("original: {before}");
            println!("next: {after}");
            Ok::<_, Box<dyn std::error::Error>>(())
        }),
    )?
}
