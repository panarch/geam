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
    use geam_core::provider::advanced::{LocalRetainedContext, ProviderTransferRetainedContext, RetainedContext};
    use num_bigint::BigInt;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::sync::Arc;

    pub trait TextContext: RetainedContext {
        type Text: Clone + Deref<Target = EcoString>;
        fn store(value: EcoString) -> Self::Text;
    }

    impl TextContext for LocalRetainedContext {
        type Text = Rc<EcoString>;
        fn store(value: EcoString) -> Self::Text { Rc::new(value) }
    }

    impl TextContext for ProviderTransferRetainedContext {
        type Text = Arc<EcoString>;
        fn store(value: EcoString) -> Self::Text { Arc::new(value) }
    }

    #[geam_macros::external(name = "SavedText", manual, context = Context)]
    pub struct SavedText<Context: TextContext = LocalRetainedContext> {
        text: Context::Text,
    }

    impl<Context: TextContext> SavedText<Context> {
        pub fn new(text: EcoString) -> Self { Self { text: Context::store(text) } }
        pub fn text(&self) -> EcoString { (*self.text).clone() }
    }

    impl<Context: TextContext> Clone for SavedText<Context> {
        fn clone(&self) -> Self { Self { text: self.text.clone() } }
    }

    impl<Context: TextContext> ExternalPayload for SavedText<Context> {
        fn source_equal(&self, other: &Self) -> bool { *self.text == *other.text }
        fn source_hash(&self) -> u64 {
            use std::hash::{DefaultHasher, Hash, Hasher};
            let mut hash = DefaultHasher::new();
            self.text.hash(&mut hash);
            hash.finish()
        }
        fn inspect(&self) -> EcoString { self.text() }
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
