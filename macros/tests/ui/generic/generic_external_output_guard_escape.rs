use geam_core::provider::{Call, Stored, Value};

#[geam_macros::provider(
    package = "generic_external_output_guard_escape",
    modules = [generic_external_output_guard_escape],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(
    path = "generic_external_output_guard_escape",
    crate_path = geam_core,
)]
mod generic_external_output_guard_escape {
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

    fn retain_context<Type, Context: 'static>(_: &Stored<Type, Context>) {}

    #[geam_macros::function]
    fn escape<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Item>,
    ) -> BoxValue<Item> {
        let boxed = BoxValue {
            value: call.store(value),
        };
        retain_context(&boxed.value);
        boxed
    }
}

fn main() {}
