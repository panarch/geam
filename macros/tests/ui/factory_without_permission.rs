#[geam_macros::provider(package = "permission", modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "permission", crate_path = geam_core)]
mod native {
    use geam_core::provider::{BigInt, Call, Callback, Factory, HostResult};

    #[geam_macros::callable(factory = Add)]
    fn add(#[geam_macros::capture] offset: BigInt, value: BigInt) -> BigInt {
        offset + value
    }

    #[geam_macros::function]
    fn wrong(
        #[geam_macros::call] call: &mut Call<()>,
    ) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
        let factory = Factory::<Add>::declaration();
        call.create(&factory, (1.into(),))
    }
}

fn main() {}
