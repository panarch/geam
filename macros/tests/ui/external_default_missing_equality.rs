#[geam_macros::provider(
    package = "metrics",
    modules = [metrics],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "metrics", crate_path = geam_core)]
mod metrics {
    #[geam_macros::external(name = "Metrics")]
    #[derive(Hash)]
    struct Metrics;

    #[geam_macros::function]
    fn new() -> Metrics {
        Metrics
    }
}

fn main() {}
