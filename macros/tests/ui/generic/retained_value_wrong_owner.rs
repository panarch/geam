use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Index0, Inspection, Retained, RetainedExternalPayload,
};
use geam_core::provider::{Call, Value};

#[geam_macros::provider(
    package = "retained_value_wrong_owner",
    modules = [retained_value_wrong_owner],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "retained_value_wrong_owner", crate_path = geam_core)]
mod retained_value_wrong_owner {
    use super::{
        Call, EcoString, Equality, Hashing, Index0, Inspection, Retained,
        RetainedExternalPayload, Value,
    };

    struct FirstPayload {
        value: Retained<FirstPayload, Index0>,
    }

    struct SecondPayload {
        value: Retained<SecondPayload, Index0>,
    }

    macro_rules! payload_semantics {
        ($payload:ty) => {
            impl RetainedExternalPayload for $payload {
                fn source_equal(
                    &self,
                    _context: &Equality<'_>,
                    _other: &Self,
                ) -> bool {
                    true
                }

                fn source_hash(&self, _context: &Hashing<'_>) -> u64 {
                    0
                }

                fn inspect(&self, _context: &Inspection<'_>) -> EcoString {
                    EcoString::new()
                }
            }
        };
    }

    payload_semantics!(FirstPayload);
    payload_semantics!(SecondPayload);

    #[geam_macros::external(
        name = "First",
        parameters = [Item],
        input = FirstInput,
        payload = FirstPayload,
        manual,
    )]
    struct First<Item>;

    #[geam_macros::external(
        name = "Second",
        parameters = [Item],
        input = SecondInput,
        payload = SecondPayload,
        manual,
    )]
    struct Second<Item>;

    #[geam_macros::function]
    fn wrong<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        first: FirstInput<Item>,
        second: SecondInput<Item>,
    ) -> Value<Item> {
        let stored = first.stored_item(|_| &second.payload().value);
        call.restore(stored)
    }
}

fn main() {}
