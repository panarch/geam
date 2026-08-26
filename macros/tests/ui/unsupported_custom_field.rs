#[geam_macros::provider(
    package = "customs",
    modules = [customs],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "customs", crate_path = geam_core)]
mod customs {
    #[geam_macros::custom]
    enum Status {
        Code(i64),
    }

    #[geam_macros::function]
    fn code() -> Status {
        Status::Code(1)
    }
}

fn main() {}
