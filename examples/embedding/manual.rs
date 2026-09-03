use ecow::EcoString;
use geam::compile_typed_project;
use geam::embedding::{FunctionDeclaration, ModuleBuilder};
use num_bigint::BigInt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program = compile_typed_project(
        concat!(env!("CARGO_MANIFEST_DIR"), "/examples/embedding/manual"),
        "inventory_rules",
    )?;

    let builder = ModuleBuilder::from_program(program)?;
    let (mut bindings, label) =
        builder.function(FunctionDeclaration::<(EcoString, EcoString), EcoString>::new("label"))?;
    let double = bindings.function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))?;
    let choose = bindings.function(FunctionDeclaration::<(bool, f64, f64), f64>::new("choose"))?;
    let module = bindings.seal();

    let mut echo = Vec::new();
    let code = module.call(&label, ("SKU:".into(), "AB-12".into()), &mut echo)?;
    let second_code = module.call(&label, ("BIN:".into(), "C-4".into()), &mut echo)?;
    let quantity = module.call(&double, (21.into(),), &mut echo)?;
    let price = module.call(&choose, (true, 12.5, 9.0), &mut echo)?;

    assert_eq!(second_code, "BIN:C-4");
    assert!(echo.is_empty());
    println!("code: {code}");
    println!("quantity: {quantity}");
    println!("price: {price}");
    Ok(())
}
