mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::HostedModuleBuilder;
use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program = geam_bindings::project()?.compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;

    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
        example_text_pattern: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = Vec::new();

    let first = module.call(
        &functions.format_words,
        ("MATCHES: ".into(),),
        &mut state,
        &mut echo,
    )?;
    let second = module.call(
        &functions.format_words,
        ("AGAIN: ".into(),),
        &mut state,
        &mut echo,
    )?;
    let words = module.call(
        &functions.contains_only_words,
        ("Geam and Gleam".into(),),
        &mut state,
        &mut echo,
    )?;
    let numbers = module.call(
        &functions.contains_only_words,
        ("Geam 2026".into(),),
        &mut state,
        &mut echo,
    )?;
    let selected = module.call(
        &functions.choose_price,
        (true, 12.5, 9.0),
        &mut state,
        &mut echo,
    )?;

    assert_eq!(first, "MATCHES: GEAM, GLEAM");
    assert_eq!(second, "AGAIN: GEAM, GLEAM");
    assert!(words);
    assert!(!numbers);
    assert_eq!(selected, 12.5);
    assert!(echo.is_empty());

    assert_eq!(state.stdlib().io_outputs().len(), 2);
    let outputs = state.stdlib_mut().take_io_outputs();
    assert_eq!(outputs.len(), 2);
    for output in outputs {
        assert_eq!(output.stream(), IoStream::Stdout);
        assert_eq!(output.text(), "formatting words\n");
    }

    println!("{first}");
    Ok(())
}
