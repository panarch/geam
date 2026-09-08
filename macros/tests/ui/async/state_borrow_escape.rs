use geam_core::provider::{BigInt, Call};

#[geam_macros::provider(
    package = "async_state_borrow_escape",
    state = BigInt,
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "async_state_borrow_escape/native", crate_path = geam_core)]
mod native {
    use super::{BigInt, Call};

    #[geam_macros::function]
    async fn read(
        #[geam_macros::call] call: &mut Call<BigInt>,
    ) -> geam_core::provider::HostResult<BigInt> {
        let state = call.with_state(|state| state).await?;
        std::future::ready(()).await;
        Ok(state.clone())
    }
}

fn main() {}
