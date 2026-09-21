#[geam_macros::provider(
    package = "macro_work",
    modules = [values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "macro_work/values", crate_path = geam_core)]
pub mod values {
    use geam_core::provider::{BigInt, Callback, Future};

    #[geam_macros::custom(input = HolderInput)]
    pub enum Holder {
        Run(Callback<fn(BigInt) -> Future<BigInt>>),
    }

    #[geam_macros::custom(input = PendingInput)]
    pub enum Pending {
        Pending(Future<BigInt>, Vec<Future<BigInt>>),
    }
}
