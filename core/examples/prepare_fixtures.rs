use geam_core::embedding::{
    BigInt, FunctionDeclaration, HostedModuleBuilder, ModuleBuilder, StringValue,
};
use geam_core::{ModuleSource, PackageSource};
use std::error::Error;
use std::path::Path;

#[path = "../tests/fixtures/prepared/native_provider.rs"]
mod native_provider;
#[path = "../tests/support/work_fixture.rs"]
mod work_fixture;
#[path = "../tests/fixtures/prepared/work_provider.rs"]
mod work_provider;

fn main() -> Result<(), Box<dyn Error>> {
    let arithmetic = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        "pub fn main() { 21 * 2 }",
    )?;
    let (arithmetic, _) =
        ModuleBuilder::new(arithmetic)?.function(FunctionDeclaration::<(), BigInt>::new("main"))?;

    let values = geam_core::compile_typed_program(
        "example",
        [ModuleSource::new(
            "example",
            "src/example.gleam",
            include_str!("../tests/fixtures/prepared/values.gleam"),
        )],
    )?;
    let (mut values, _) = ModuleBuilder::from_program(values)?
        .function(FunctionDeclaration::<(), BigInt>::new("run"))?;
    values.function(FunctionDeclaration::<(), BigInt>::new("fail"))?;
    values.function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))?;

    let native = geam_core::compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "main",
                "src/main.gleam",
                include_str!("../tests/fixtures/prepared/native.gleam"),
            )],
        )],
        native_provider::hosts(),
    )?;
    let (mut native, _) = HostedModuleBuilder::new(native)?
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))?;
    native
        .function(FunctionDeclaration::<(StringValue,), (bool, StringValue)>::new("substring"))?;

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/prepared");
    for (name, data) in [
        ("arithmetic.rs", arithmetic.prepare().emit_rust()),
        ("values.rs", values.prepare().emit_rust()),
        ("native.rs", native.prepare()?.emit_rust()),
        ("work.rs", work_provider::prepare().emit_rust()),
        (
            "entry.rs",
            work_provider::prepare_entry("entry").emit_rust(),
        ),
        (
            "entry_work.rs",
            work_provider::prepare_entry("entry_work").emit_rust(),
        ),
        (
            "entry_failure.rs",
            work_provider::prepare_entry("entry_failure").emit_rust(),
        ),
    ] {
        let destination = root.join(name);
        std::fs::write(&destination, format!("{data}\n"))?;
        println!("{}", destination.display());
    }
    Ok(())
}
