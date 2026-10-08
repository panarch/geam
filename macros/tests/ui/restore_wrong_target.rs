#[geam_macros::provider(package = "restoration", modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "restoration", crate_path = geam_core)]
mod native {
    use super::Owner;
    use geam_core::provider::{BigInt, Call, HostResult, Restore, StringValue, Value};

    #[geam_macros::function]
    fn wrong<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::restore] _restore: Restore<BigInt>,
        value: Value<Item>,
    ) -> HostResult<Option<StringValue>> {
        let retained = call.store_dynamic::<_, Owner>(value);
        let value = retained.native_view();
        let restore = Restore::<StringValue>::declaration();
        Ok(call.restore_native(&restore, &value))
    }
}

struct Owner;
impl geam_core::provider::ProviderStoredOwner for Owner {}

fn main() {}
