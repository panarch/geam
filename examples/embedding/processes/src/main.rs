// The generated module also exposes the standalone entry and stdlib accessors.
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;
use geam::gleam_erlang::Configuration;
use geam::gleam_stdlib::{GleamStdlibRunState, IoOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()?;
    let host = TokioHost::new(executor.handle().clone());
    let program = geam_bindings::project::<Vec<IoOutput>>().compile()?;
    let erlang = Configuration {
        resources: program.package_resources().clone(),
    };
    let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
    let mut module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang,
    }
    .initialize();
    let mut echo = Vec::new();

    executor.block_on(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            let (pid, requests) = scope.call(&functions.start, ()).await?;
            let alias = requests.clone();
            assert!(scope.call(&functions.is_alive, (&pid,)).await?);
            let first = scope.call(&functions.add, (&requests, 20.into())).await?;
            let second = scope.call(&functions.add, (alias, 22.into())).await?;
            let stopped = scope.call(&functions.stop, (&pid, &requests)).await?;
            println!("first: {first}");
            println!("second: {second}");
            println!("stopped: {stopped}");
            Ok::<_, Box<dyn std::error::Error>>(())
        }),
    )?
}
