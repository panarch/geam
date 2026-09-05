use super::AsyncHostStoredValue;
use crate::runtime::TransferStoredRuntimeValue;
use ecow::EcoString;

pub(crate) struct TransferExternalEquality<'context> {
    equal: &'context dyn Fn(&TransferStoredRuntimeValue, &TransferStoredRuntimeValue) -> bool,
}

pub(crate) struct TransferExternalHashing<'context> {
    source_hash: &'context dyn Fn(&TransferStoredRuntimeValue) -> u64,
}

pub(crate) struct TransferExternalInspection<'context> {
    inspect: &'context dyn Fn(&TransferStoredRuntimeValue) -> EcoString,
}

/// Gleam equality for typed values retained in an async external payload.
pub struct AsyncHostExternalEquality<'context>(
    pub(super) &'context TransferExternalEquality<'context>,
);

/// Gleam hashing for typed values retained in an async external payload.
pub struct AsyncHostExternalHashing<'context>(
    pub(super) &'context TransferExternalHashing<'context>,
);

/// Gleam inspection for typed values retained in an async external payload.
pub struct AsyncHostExternalInspection<'context>(
    pub(super) &'context TransferExternalInspection<'context>,
);

impl<'context> TransferExternalEquality<'context> {
    pub(crate) fn new(
        equal: &'context dyn Fn(&TransferStoredRuntimeValue, &TransferStoredRuntimeValue) -> bool,
    ) -> Self {
        Self { equal }
    }

    pub(crate) fn stored_values_equal(
        &self,
        left: &TransferStoredRuntimeValue,
        right: &TransferStoredRuntimeValue,
    ) -> bool {
        (self.equal)(left, right)
    }
}

impl<'context> TransferExternalHashing<'context> {
    pub(crate) fn new(source_hash: &'context dyn Fn(&TransferStoredRuntimeValue) -> u64) -> Self {
        Self { source_hash }
    }

    pub(crate) fn stored_value_hash(&self, value: &TransferStoredRuntimeValue) -> u64 {
        (self.source_hash)(value)
    }
}

impl<'context> TransferExternalInspection<'context> {
    pub(crate) fn new(inspect: &'context dyn Fn(&TransferStoredRuntimeValue) -> EcoString) -> Self {
        Self { inspect }
    }

    pub(crate) fn inspect_stored_value(&self, value: &TransferStoredRuntimeValue) -> EcoString {
        (self.inspect)(value)
    }
}

impl AsyncHostExternalEquality<'_> {
    pub fn stored_values_equal<Type>(
        &self,
        left: &AsyncHostStoredValue<Type>,
        right: &AsyncHostStoredValue<Type>,
    ) -> bool {
        self.0.stored_values_equal(&left.value, &right.value)
    }
}

impl AsyncHostExternalHashing<'_> {
    pub fn stored_value_hash<Type>(&self, value: &AsyncHostStoredValue<Type>) -> u64 {
        self.0.stored_value_hash(&value.value)
    }
}

impl AsyncHostExternalInspection<'_> {
    pub fn inspect_stored_value<Type>(&self, value: &AsyncHostStoredValue<Type>) -> EcoString {
        self.0.inspect_stored_value(&value.value)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Counter, Echo, Envelope, Profile, Provider, make_counter};
    use super::{
        AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferStoredRuntimeValue,
    };
    use crate::embedding::{AsyncHostedModuleBuilder, FunctionDeclaration};
    use crate::{
        AsyncHostCall, AsyncHostExternal, AsyncHostExternalReturn, AsyncHostFuture,
        AsyncHostProviderModule, AsyncHostProviderSet, HostExternalType, ModuleSource,
        PackageSource, compile_typed_async_host_program,
    };
    use std::future::Future;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::task::{Context, Poll, Waker};

    #[test]
    fn typed_contexts_forward_the_exact_retained_values() {
        fn verify<'call>(
            mut call: AsyncHostCall<'call, Profile, Provider, HostExternalType<Envelope>>,
            value: AsyncHostExternal<'call, Counter>,
        ) -> AsyncHostFuture<'call, AsyncHostExternalReturn<'call, Envelope>> {
            AsyncHostFuture::new(async move {
                call.return_external(|payload| {
                    let left = payload.store_external(&value);
                    let right = payload.store_external(&value);
                    let equal = |a: &TransferStoredRuntimeValue, b: &TransferStoredRuntimeValue| {
                        assert!(std::ptr::eq(a, &left.value));
                        assert!(std::ptr::eq(b, &right.value));
                        false
                    };
                    let hash = |value: &TransferStoredRuntimeValue| {
                        assert!(std::ptr::eq(value, &left.value));
                        17
                    };
                    let inspect = |value: &TransferStoredRuntimeValue| {
                        assert!(std::ptr::eq(value, &right.value));
                        "retained counter".into()
                    };
                    let equal = TransferExternalEquality::new(&equal);
                    let hash = TransferExternalHashing::new(&hash);
                    let inspect = TransferExternalInspection::new(&inspect);
                    assert!(!AsyncHostExternalEquality(&equal).stored_values_equal(&left, &right));
                    assert_eq!(AsyncHostExternalHashing(&hash).stored_value_hash(&left), 17);
                    assert_eq!(
                        AsyncHostExternalInspection(&inspect).inspect_stored_value(&right),
                        "retained counter",
                    );
                    left
                })
                .await
            })
        }

        let provider = AsyncHostProviderModule::<Profile>::new("application", "library")
            .expect("context fixture identity")
            .with_external_type::<Provider, Counter>().expect("counter schema")
            .with_external_type::<Provider, Envelope>().expect("envelope schema")
            .with_scoped_async_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter)
            .expect("counter constructor")
            .with_scoped_async_function::<Provider, (HostExternalType<Counter>,), HostExternalType<Envelope>, _>("verify", verify)
            .expect("context verification");
        let source = r#"
@external(erlang, "native", "Counter")
pub type Counter
@external(erlang, "native", "Envelope")
pub type Envelope
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "verify")
fn verify(value: Counter) -> Envelope

pub fn run() {
  let _ = verify(make())
  Nil
}
"#;
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            AsyncHostProviderSet::with_providers([], [provider]).expect("context fixture provider"),
        )
        .expect("context fixture source");
        let (bindings, function) = AsyncHostedModuleBuilder::new(program)
            .expect("context fixture plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("context fixture binding");
        let mut module = bindings.seal();
        let mut state = Arc::new(AtomicUsize::new(0));
        let mut echo = Echo::default();
        let mut call = Box::pin(module.call_async(&function, (), &mut state, &mut echo));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(
            call.as_mut()
                .poll(&mut context)
                .map(|result| result.expect("context verification")),
            Poll::Ready(()),
        );
        drop(call);
        assert_eq!(state.load(Ordering::SeqCst), 1);
        assert!(echo.0.is_empty());
    }
}
