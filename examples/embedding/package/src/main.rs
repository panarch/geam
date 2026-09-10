// Generated bindings expose stdlib accessors this example does not need.
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::{EcoString, HostedModuleBuilder};
use geam::gleam_stdlib::GleamStdlibRunState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = geam::execution::TokioHost::new(executor.handle().clone());
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let mut module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
    }
    .initialize();
    let mut echo = Vec::new();

    let (first, empty) = executor.block_on(module.with_execution(
        &host,
        &mut state,
        &mut echo,
        async |scope| {
            let first = scope
                .call(
                    &functions.first,
                    (vec![EcoString::from("Gleam"), EcoString::from("Rust")],),
                )
                .await?;
            let empty = scope
                .call(&functions.first, (Vec::<EcoString>::new(),))
                .await?;
            Ok::<_, geam::embedding::CallError>((first, empty))
        },
    ))??;

    match first {
        Some(value) => println!("first: {value}"),
        None => println!("first: none"),
    }
    match empty {
        Some(value) => println!("empty: {value}"),
        None => println!("empty: none"),
    }
    Ok(())
}
