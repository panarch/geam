use geam_core::provider::advanced::{Equality, Hashing, Inspection, RetainedExternalPayload, StoredDynamic};
use geam_core::provider::{Call, Callback};

#[geam_macros::provider(package = "opaque", modules = [messages, opaque], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "messages", crate_path = geam_core)]
mod messages {
    use super::Callback;

    #[geam_macros::custom(input = MessageInput)]
    pub enum Message {
        Reply(Callback<fn() -> ()>),
    }
}

#[geam_macros::module(path = "opaque", crate_path = geam_core)]
mod opaque {
    use super::{Call, Callback, Equality, Hashing, Inspection, RetainedExternalPayload, StoredDynamic};

    #[geam_macros::custom(input = MessageInput)]
    enum Message {
        Reply(Callback<fn() -> ()>),
    }

    #[geam_macros::custom(input = BatchInput)]
    enum Batch {
        Batch(Vec<(bool, super::messages::Message)>),
    }

    #[geam_macros::external(name = "Dynamic", retained)]
    struct Dynamic { value: StoredDynamic<Dynamic> }

    impl RetainedExternalPayload for Dynamic {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }
        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }
        fn inspect(&self, context: &Inspection<'_>) -> ecow::EcoString {
            self.value.inspect(context)
        }
    }

    #[geam_macros::function]
    fn recover_callback(#[geam_macros::call] call: &mut Call<()>, value: &Dynamic) -> bool {
        call.restore_dynamic::<Message, Dynamic>(&value.value).is_some()
    }

    #[geam_macros::function]
    fn recover_nested_callback(#[geam_macros::call] call: &mut Call<()>, value: &Dynamic) -> bool {
        call.restore_dynamic::<Batch, Dynamic>(&value.value).is_some()
    }
}

fn main() {}
