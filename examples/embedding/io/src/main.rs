// Generated bindings expose a read-only stdlib accessor this example does not need.
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::HostedModuleBuilder;
use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project().compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;
    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
    }
    .initialize();
    let mut echo = Vec::new();

    let message = module.call(&functions.announce, ("Rust".into(),), &mut state, &mut echo)?;

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
    writeln!(stdout, "returned: {message}")?;
    Ok(())
}
