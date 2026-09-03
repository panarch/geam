mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        example_text_pattern: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = Vec::new();

    let matched = module.call(
        &functions.matches,
        ("^[A-Z]+$".into(), "GEAM".into()),
        &mut state,
        &mut echo,
    )?;
    match matched {
        Ok(matched) => println!("matched: {matched}"),
        Err(message) => println!("pattern error: {message}"),
    }
    Ok(())
}
