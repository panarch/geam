use geam_core::provider::{Configuration, InitializationError};

fn initialize(_: &Configuration) -> Result<(), InitializationError> {
    Ok(())
}

#[geam_macros::provider(
    id = "external-non-struct",
    package = "metrics",
    state = (),
    initialize = initialize,
    modules = [metrics],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "metrics", crate_path = geam_core)]
mod metrics {
    #[geam_macros::external(name = "Metrics")]
    enum Metrics {}
}

fn main() {}
