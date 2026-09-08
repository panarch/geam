use ecow::EcoString;
use geam_core::provider::{BigInt, ExternalPayload};

#[geam_macros::provider(
    package = "async_external_borrow_escape",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "async_external_borrow_escape/native", crate_path = geam_core)]
mod native {
    use super::{BigInt, EcoString, ExternalPayload};

    #[geam_macros::external(name = "Counter", manual)]
    struct Counter(BigInt);

    impl ExternalPayload for Counter {
        fn source_equal(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn source_hash(&self) -> u64 {
            0
        }

        fn inspect(&self) -> EcoString {
            "Counter".into()
        }
    }

    #[geam_macros::function]
    async fn read(counter: &Counter) -> BigInt {
        let value = counter.with(|counter| &counter.0);
        std::future::ready(()).await;
        value.clone()
    }
}

fn main() {}
