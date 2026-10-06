mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;
use geam::execution::{ExecutionOutcome, TokioHost};
use std::str::Utf8Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let mut module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        example_async_files: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = |output: geam::EchoOutput| eprintln!("{output}");

    let outcome =
        executor.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let doubled = scope.call(&functions.double, (21.into(),)).await?;
                println!("double: {doubled}");

                let path = concat!(env!("CARGO_MANIFEST_DIR"), "/message.txt");
                let work = scope.call(&functions.greeting, (path.into(),)).await?;
                println!("created");
                let result = scope.observe(&work).await?;
                result.read(|value| {
                    match value {
                        Ok(text) => println!("{}", text.as_str()?.trim_end()),
                        Err(error) => eprintln!("{error}"),
                    }
                    Ok::<_, Utf8Error>(())
                })?;
                let same_result = scope.observe(&work).await?;
                same_result.read(|value| {
                    match value {
                        Ok(text) => println!("again: {}", text.as_str()?.trim_end()),
                        Err(error) => eprintln!("{error}"),
                    }
                    Ok::<_, Utf8Error>(())
                })?;
                Ok::<_, Box<dyn std::error::Error>>(())
            }),
        )?;
    match outcome {
        ExecutionOutcome::Returned(result) => result,
        ExecutionOutcome::Exited(status) => {
            println!("Gleam exited with status {status}");
            Ok(())
        }
    }
}
