use geam_core::provider::BigInt;

#[geam_macros::provider(
    package = "async_shared_call",
    state = BigInt,
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "async_shared_call/native", crate_path = geam_core)]
mod native {
    use super::BigInt;

    #[geam_macros::function]
    async fn read(
        #[geam_macros::call] call: &geam_core::provider::Call<BigInt>,
    ) -> BigInt {
        std::future::ready(()).await;
        call.state().clone()
    }
}

fn main() {}
