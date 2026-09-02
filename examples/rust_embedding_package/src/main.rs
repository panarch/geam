// Generated bindings expose stdlib accessors this example does not need.
#[allow(dead_code)]
mod geam_bindings;

use geam::embedding::{EcoString, HostedModuleBuilder};
use geam::gleam_stdlib::GleamStdlibRunState;

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

    let first = module.call(
        &functions.first,
        (vec![EcoString::from("Gleam"), EcoString::from("Rust")],),
        &mut state,
        &mut echo,
    )?;
    let empty = module.call(
        &functions.first,
        (Vec::<EcoString>::new(),),
        &mut state,
        &mut echo,
    )?;

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
