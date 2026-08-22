struct RunState;

#[geam_macros::provider(
    package = "missing_default",
    state = RunState,
    modules = [missing_default],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "missing_default", crate_path = geam_core)]
mod missing_default {}

fn main() {}
