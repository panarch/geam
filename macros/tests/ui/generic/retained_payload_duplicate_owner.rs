use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Inspection, RetainedExternalPayload,
};

#[geam_macros::provider(
    package = "retained_payload_duplicate_owner",
    modules = [retained_payload_duplicate_owner],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(
    path = "retained_payload_duplicate_owner",
    crate_path = geam_core,
)]
mod retained_payload_duplicate_owner {
    use super::{EcoString, Equality, Hashing, Inspection, RetainedExternalPayload};

    struct SharedPayload;

    impl RetainedExternalPayload for SharedPayload {
        fn source_equal(&self, _context: &Equality<'_>, _other: &Self) -> bool {
            true
        }

        fn source_hash(&self, _context: &Hashing<'_>) -> u64 {
            0
        }

        fn inspect(&self, _context: &Inspection<'_>) -> EcoString {
            "Shared".into()
        }
    }

    #[geam_macros::external(
        name = "First",
        parameters = [Item],
        input = FirstInput,
        payload = SharedPayload,
        manual,
    )]
    struct First<Item>;

    #[geam_macros::external(
        name = "Second",
        parameters = [Item],
        input = SecondInput,
        payload = SharedPayload,
        manual,
    )]
    struct Second<Item>;
}

fn main() {}
