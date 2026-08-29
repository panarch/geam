use ecow::EcoString;
use geam::embedding::{FunctionDeclaration, HostedModuleBuilder};
use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet, compile_typed_host_project};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let providers = host_providers::<GleamStdlibProfile>()?;
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)?;
    let program = compile_typed_host_project(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/rust_hosted_embedding"
        ),
        "text_rules",
        hosts,
    )?;

    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, normalize) = builder.function(
        FunctionDeclaration::<(EcoString,), EcoString>::new("normalize"),
    )?;
    let module = bindings.seal()?;

    let mut state = GleamStdlibRunState::from_seed([0; 32]);
    let mut echo = Vec::new();
    let normalized = module.call(&normalize, ("Geam + GLEAM".into(),), &mut state, &mut echo)?;

    assert_eq!(normalized, "geam + gleam");
    assert!(echo.is_empty());
    println!("normalized: {normalized}");
    Ok(())
}
