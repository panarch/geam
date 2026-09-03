// Generated bindings can expose accessors this application does not need.
#[allow(dead_code)]
mod geam_bindings;
mod inventory;

use geam::HostProviderConfiguration;
use geam::embedding::{BigInt, EcoString, HostedModuleBuilder};
use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;

    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
        example_text_pattern: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = Vec::new();

    let rows: Vec<(EcoString, BigInt)> = vec![
        (" ab-12 ".into(), 3.into()),
        ("invalid".into(), 2.into()),
        (" c-7 ".into(), 4.into()),
        ("D-1".into(), (-1).into()),
    ];
    let review = inventory::review(&module, &functions, rows, &mut state, &mut echo)?;

    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    for output in state.stdlib_mut().take_io_outputs() {
        match output.stream() {
            IoStream::Stdout => stdout.write_all(output.text().as_bytes())?,
            IoStream::Stderr => stderr.write_all(output.text().as_bytes())?,
        }
    }
    for output in echo {
        writeln!(stderr, "{output}")?;
    }

    review.write_report(&mut stdout)?;
    Ok(())
}
