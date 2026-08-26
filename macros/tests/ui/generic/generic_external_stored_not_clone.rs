use geam_core::provider::{Call, Stored, Value};

#[geam_macros::provider(
    package = "generic_external_stored_not_clone",
    modules = [generic_external_stored_not_clone],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(
    path = "generic_external_stored_not_clone",
    crate_path = geam_core,
)]
mod generic_external_stored_not_clone {
    use super::{Call, Stored, Value};

    #[geam_macros::external(
        name = "Box",
        parameters = [Item],
        input = BoxInput,
    )]
    struct BoxValue<Item> {
        #[geam_macros::stored]
        value: Stored<Item>,
    }

    #[geam_macros::function]
    fn duplicate<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        boxed: BoxInput<Item>,
    ) -> Value<Item> {
        let stored = boxed.value();
        let duplicate = stored.clone();
        let _ = duplicate;
        call.restore(stored)
    }
}

fn main() {}
