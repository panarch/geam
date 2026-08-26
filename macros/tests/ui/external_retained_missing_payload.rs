#[geam_macros::provider(
    package = "dynamic",
    modules = [dynamic],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "dynamic", crate_path = geam_core)]
mod dynamic {
    #[geam_macros::external(name = "Dynamic", retained)]
    struct Dynamic;

    #[geam_macros::function]
    fn new() -> Dynamic {
        Dynamic
    }
}

fn main() {}
