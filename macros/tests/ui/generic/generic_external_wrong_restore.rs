use geam_core::provider::{Call, Stored, Value};

#[geam_macros::provider(
    package = "generic_external_wrong_restore",
    modules = [generic_external_wrong_restore],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(
    path = "generic_external_wrong_restore",
    crate_path = geam_core,
)]
mod generic_external_wrong_restore {
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
    fn wrong<StoredItem, ReturnedItem>(
        #[geam_macros::call] call: &mut Call<()>,
        boxed: BoxInput<StoredItem>,
    ) -> Value<ReturnedItem> {
        call.restore(boxed.value())
    }
}

fn main() {}
