mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    let host = TokioHost::new(executor.handle().clone());
    let mut executions = Vec::new();
    if std::env::args().nth(1).as_deref() != Some("--prepared") {
        let program = geam_bindings::project().compile()?;
        let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
        executions.push((bindings.seal()?, functions));
    }
    let (bindings, functions) = geam_bindings::load()?;
    executions.push((bindings, functions));
    for (mut module, functions) in executions {
        let mut state = geam_bindings::RunStateInputs {
            stdlib: geam::gleam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: geam::gleam_erlang::Configuration::default(),
            gleam_otp: HostProviderConfiguration::empty(),
        }
        .initialize()?;
        let mut echo = Vec::new();
        executor.block_on(module.with_execution(
            &host,
            &mut state,
            &mut echo,
            async |scope| scope.call(&functions.main, ()).await,
        ))??;
        assert!(echo.is_empty());
        assert_eq!(state.stdlib().io_outputs().len(), 3);
        for output in state.stdlib_mut().take_io_outputs() {
            print!("{}", output.text());
        }
    }
    Ok(())
}
