use geam_core::__macro_support::ProviderTransferRetainedContext;
use geam_core::provider::advanced::{LocalRetainedContext, RetainedContext};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

/// Storage policy shared by the immutable standard-library and JSON graphs.
#[doc(hidden)]
pub trait StorageContext: RetainedContext {
    type Shared<Value>: Clone + Deref<Target = Value> + AsRef<Value>;

    fn share<Value>(value: Value) -> Self::Shared<Value>;
    fn try_unwrap<Value>(value: Self::Shared<Value>) -> Result<Value, Self::Shared<Value>>;
}

impl StorageContext for LocalRetainedContext {
    type Shared<Value> = Rc<Value>;

    fn share<Value>(value: Value) -> Self::Shared<Value> {
        Rc::new(value)
    }

    fn try_unwrap<Value>(value: Self::Shared<Value>) -> Result<Value, Self::Shared<Value>> {
        Rc::try_unwrap(value)
    }
}

impl StorageContext for ProviderTransferRetainedContext {
    type Shared<Value> = Arc<Value>;

    fn share<Value>(value: Value) -> Self::Shared<Value> {
        Arc::new(value)
    }

    fn try_unwrap<Value>(value: Self::Shared<Value>) -> Result<Value, Self::Shared<Value>> {
        Arc::try_unwrap(value)
    }
}

#[cfg(test)]
mod tests {
    use super::StorageContext;
    use geam_core::__macro_support::ProviderTransferRetainedContext;
    use geam_core::provider::advanced::LocalRetainedContext;
    use std::rc::Rc;

    #[test]
    fn local_graphs_retain_the_original_non_send_allocation() {
        let item = LocalRetainedContext::share(Rc::new(42));
        let alias = item.clone();
        assert!(std::ptr::eq(item.as_ref(), alias.as_ref()));
        let item = LocalRetainedContext::try_unwrap(item).expect_err("alias owns the allocation");
        drop(alias);
        assert_eq!(
            *LocalRetainedContext::try_unwrap(item).expect("last owner"),
            42
        );
    }

    #[test]
    fn transferable_graphs_keep_the_same_allocation_across_workers() {
        let item = ProviderTransferRetainedContext::share(42);
        let alias = item.clone();
        assert!(std::ptr::eq(item.as_ref(), alias.as_ref()));
        let item = ProviderTransferRetainedContext::try_unwrap(item)
            .expect_err("alias owns the allocation");
        drop(alias);
        assert_eq!(
            std::thread::spawn(move || {
                ProviderTransferRetainedContext::try_unwrap(item).expect("last owner")
            })
            .join()
            .expect("worker"),
            42
        );
    }
}
