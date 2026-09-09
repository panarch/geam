use geam_core::provider::{BigInt, Call, Callback};

fn require_static<Value: 'static>(_value: Value) {}

#[geam_macros::provider(
    package = "async_callback_escape",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "async_callback_escape/native", crate_path = geam_core)]
mod native {
    use super::{BigInt, Call, Callback, require_static};

    #[geam_macros::function]
    fn escape(
        #[geam_macros::call] _call: &mut Call<()>,
        callback: Callback<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> geam_core::provider::HostResult<BigInt> {
        require_static(callback);
        Ok(value)
    }
}

fn main() {}
