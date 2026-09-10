mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = geam::execution::TokioHost::new(executor.handle().clone());
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let mut module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        example_text_pattern: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = Vec::new();

    let matched = executor.block_on(module.with_execution(
        &host,
        &mut state,
        &mut echo,
        async |scope| {
            scope
                .call(&functions.matches, ("^[A-Z]+$".into(), "GEAM".into()))
                .await
        },
    ))??;
    match matched {
        Ok(matched) => println!("matched: {matched}"),
        Err(message) => println!("pattern error: {message}"),
    }
    Ok(())
}
