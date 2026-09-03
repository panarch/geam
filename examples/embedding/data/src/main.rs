mod geam_bindings;

use geam::embedding::{BigInt, EcoString, ModuleBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project().compile()?;
    let builder = ModuleBuilder::from_program(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal();
    let mut echo = Vec::new();

    let rows: Vec<(EcoString, BigInt)> = vec![
        ("A-1".into(), 3.into()),
        ("B-2".into(), (-1).into()),
        ("C-3".into(), 4.into()),
    ];
    let reviewed = module.call(&functions.review, (rows,), &mut echo)?;
    let total = module.call(&functions.total, (&reviewed,), &mut echo)?;

    for row in reviewed.iter() {
        match row {
            Ok((code, quantity)) => println!("accepted: {code} ({quantity})"),
            Err(message) => println!("rejected: {message}"),
        }
    }
    println!("total: {total}");
    Ok(())
}
