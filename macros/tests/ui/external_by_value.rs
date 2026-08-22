use geam_core::provider::{Configuration, InitializationError};

fn initialize(_: &Configuration) -> Result<(), InitializationError> {
    Ok(())
}

#[geam_macros::provider(
    id = "external-by-value",
    package = "metrics",
    state = (),
    initialize = initialize,
    modules = [metrics],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "metrics", crate_path = geam_core)]
mod metrics {
    use geam_core::provider::ExternalPayload;

    #[geam_macros::external(name = "Metrics", manual)]
    struct Metrics;

    impl ExternalPayload for Metrics {
        fn source_equal(&self, _: &Self) -> bool {
            true
        }

        fn source_hash(&self) -> u64 {
            0
        }

        fn inspect(&self) -> ecow::EcoString {
            "Metrics".into()
        }
    }

    #[geam_macros::function]
    fn identity(metrics: Metrics) -> Metrics {
        metrics
    }
}

fn main() {}
