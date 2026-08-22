use geam_core::provider::{Configuration, InitializationError};

fn initialize(_: &Configuration) -> Result<(), InitializationError> {
    Ok(())
}

#[geam_macros::provider(
    id = "external-missing-payload",
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
    struct Metrics;

    #[geam_macros::function]
    fn new() -> Metrics {
        Metrics
    }
}

fn main() {}
