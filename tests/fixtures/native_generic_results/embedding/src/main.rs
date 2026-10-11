mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;
use geam::execution::TokioHost;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
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
            native_generic_results_fixture: HostProviderConfiguration::empty(),
        }
        .initialize()?;
        let mut echo = Vec::new();
        for _ in 0..2 {
            executor
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        let error = scope.call(&functions.known_failure, ()).await.unwrap_err();
                        assert!(
                            error
                                .to_string()
                                .contains("payload does not match the registered target")
                        );
                        let error = scope
                            .call(&functions.known_pair_failure, ())
                            .await
                            .unwrap_err();
                        assert!(
                            error
                                .to_string()
                                .contains("pair does not match the registered target")
                        );
                        let error = scope
                            .call(&functions.known_result_failure, ())
                            .await
                            .unwrap_err();
                        assert!(
                            error
                                .to_string()
                                .contains("result does not match the registered target")
                        );
                        let error = scope
                            .call(&functions.known_list_failure, ())
                            .await
                            .unwrap_err();
                        assert!(
                            error
                                .to_string()
                                .contains("list does not match the registered target")
                        );
                        scope.call(&functions.main, ()).await
                    }),
                )?
                .try_into_value()
                .expect("fixture completes normally")?;
        }
        assert!(echo.is_empty());
        assert_eq!(state.stdlib().io_outputs().len(), 2);
        for output in state.stdlib_mut().take_io_outputs() {
            print!("{}", output.text());
        }
    }
    Ok(())
}
