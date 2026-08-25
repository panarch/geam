use ecow::EcoString;
use geam_core::provider::Stored;

#[geam_macros::provider(
    package = "generic_external_input_guard_escape",
    modules = [generic_external_input_guard_escape],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(
    path = "generic_external_input_guard_escape",
    crate_path = geam_core,
)]
mod generic_external_input_guard_escape {
    use super::{EcoString, Stored};

    #[geam_macros::external(
        name = "Box",
        parameters = [Item],
        input = BoxInput,
    )]
    struct BoxValue<Item> {
        #[geam_macros::stored]
        value: Stored<Item>,
    }

    fn retain<Input: 'static>(_: Input) {}

    #[geam_macros::function]
    fn escape(boxed: BoxInput<EcoString>) -> bool {
        retain(boxed);
        true
    }
}

fn main() {}
