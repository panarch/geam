#[geam_macros::provider(
    package = "macro_declarations",
    modules = [values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "macro_declarations/values", crate_path = geam_core)]
pub mod values {
    use ecow::EcoString;
    use geam_core::provider::ExternalPayload;
    use num_bigint::BigInt;
    use std::sync::Arc;

    #[geam_macros::external(name = "SavedText", manual)]
    pub struct SavedText {
        text: Arc<EcoString>,
    }

    impl SavedText {
        pub fn new(text: EcoString) -> Self {
            Self {
                text: Arc::new(text),
            }
        }
        pub fn text(&self) -> EcoString {
            (*self.text).clone()
        }
    }

    impl Clone for SavedText {
        fn clone(&self) -> Self {
            Self {
                text: self.text.clone(),
            }
        }
    }

    impl ExternalPayload for SavedText {
        fn source_equal(&self, other: &Self) -> bool {
            *self.text == *other.text
        }
        fn source_hash(&self) -> u64 {
            use std::hash::{DefaultHasher, Hash, Hasher};
            let mut hash = DefaultHasher::new();
            self.text.hash(&mut hash);
            hash.finish()
        }
        fn inspect(&self) -> EcoString {
            self.text()
        }
    }

    #[geam_macros::custom(input = SavedStatusInput)]
    #[derive(Clone)]
    pub enum SavedStatus {
        Empty,
        Saved(SavedText),
    }

    #[geam_macros::external(name = "Token")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Token(pub EcoString);

    #[geam_macros::custom(input = StatusInput)]
    pub enum Status {
        Ready,
        Count(BigInt),
        Tagged(Token),
    }
}
