mod geam_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (module, functions) = geam_bindings::load()?;
    let mut echo = Vec::new();

    let value = module.call(&functions.double, (21.into(),), &mut echo)?;
    println!("{value}");
    Ok(())
}
