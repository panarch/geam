#[geam_macros::provider(
    package = "tuples",
    modules = [tuples],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "tuples", crate_path = geam_core)]
mod tuples {
    #[geam_macros::function]
    fn enabled(value: (i64, bool)) -> bool {
        value.1
    }
}

fn main() {}
