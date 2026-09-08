use crate::runtime::{StoredRuntimeValue, TransferStoredRuntimeValue, TransferValues};
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
    pub(crate) fn provider_stored_values_equal(
        &self,
        left: &StoredRuntimeValue<TransferValues>,
        right: &StoredRuntimeValue<TransferValues>,
    ) -> bool {
        self.0.stored_values_equal(
            &left.transfer_semantic_value(),
            &right.transfer_semantic_value(),
        )
    }
}

impl AsyncHostExternalHashing<'_> {
    pub(crate) fn provider_stored_value_hash(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
    ) -> u64 {
        self.0.stored_value_hash(&value.transfer_semantic_value())
    }
}

impl AsyncHostExternalInspection<'_> {
    pub(crate) fn provider_inspect_stored_value(
        &self,
        value: &StoredRuntimeValue<TransferValues>,
    ) -> EcoString {
        self.0
            .inspect_stored_value(&value.transfer_semantic_value())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Counter, Echo, Envelope, Profile, Provider, make_counter, program};
    use super::{
        AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferStoredRuntimeValue,
    };
    use crate::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
    use crate::host::{TransferHostCall, TransferHostProviderModule};
    use crate::{AsyncHostCallError, HostCallCompletion, HostExternal, HostExternalType};
    use futures_util::FutureExt;
    use std::cell::Cell;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn contexts_forward_retained_values_through_their_source_semantic_boundary() {
        fn verify<'call>(
            mut call: TransferHostCall<'call, Profile, Provider, HostExternalType<Envelope>>,
            value: HostExternal<'call, HostExternalType<Counter>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Envelope>>, AsyncHostCallError>
        {
            let left = call.retain_value::<HostExternalType<Counter>>(value);
            let right = call.retain_value::<HostExternalType<Counter>>(value);
            let left_semantics = left.transfer_semantic_value();
            let right_semantics = right.transfer_semantic_value();
            let equal = |a: &TransferStoredRuntimeValue, b: &TransferStoredRuntimeValue| {
                assert!(std::ptr::eq(a, &left_semantics));
                assert!(std::ptr::eq(b, &right_semantics));
                false
            };
            let hash = |value: &TransferStoredRuntimeValue| {
                assert!(std::ptr::eq(value, &left_semantics));
                17
            };
            let inspect = |value: &TransferStoredRuntimeValue| {
                assert!(std::ptr::eq(value, &right_semantics));
                "retained counter".into()
            };
            assert!(
                !TransferExternalEquality::new(&equal)
                    .stored_values_equal(&left_semantics, &right_semantics)
            );
            assert_eq!(
                TransferExternalHashing::new(&hash).stored_value_hash(&left_semantics),
                17
            );
            assert_eq!(
                TransferExternalInspection::new(&inspect).inspect_stored_value(&right_semantics),
                "retained counter"
            );
            let calls = Cell::new(0);
            let equal = |_: &TransferStoredRuntimeValue, _: &TransferStoredRuntimeValue| {
                calls.set(calls.get() + 1);
                false
            };
            let hash = |_: &TransferStoredRuntimeValue| {
                calls.set(calls.get() + 1);
                17
            };
            let inspect = |_: &TransferStoredRuntimeValue| {
                calls.set(calls.get() + 1);
                "retained counter".into()
            };
            assert!(
                !AsyncHostExternalEquality(&TransferExternalEquality::new(&equal))
                    .provider_stored_values_equal(&left, &right)
            );
            assert_eq!(
                AsyncHostExternalHashing(&TransferExternalHashing::new(&hash))
                    .provider_stored_value_hash(&left),
                17
            );
            assert_eq!(
                AsyncHostExternalInspection(&TransferExternalInspection::new(&inspect))
                    .provider_inspect_stored_value(&right),
                "retained counter"
            );
            assert_eq!(calls.get(), 3);
            let value = call.create_external_with_binding::<Provider>(left);
            Ok(call.return_value(value))
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library").expect("identity")
            .with_external_type::<Provider, Counter>().expect("counter")
            .with_external_type::<Provider, Envelope>().expect("envelope")
            .with_scoped_function::<Provider, (), HostExternalType<Counter>, _>("make", make_counter).expect("make")
            .with_scoped_function::<Provider, (HostExternalType<Counter>,), HostExternalType<Envelope>, _>("verify", verify).expect("verify");
        let source = r#"
pub type Counter
pub type Envelope
@external(erlang, "native", "make")
fn make() -> Counter
@external(erlang, "native", "verify")
fn verify(value: Counter) -> Envelope
pub fn run() { let _ = verify(make()) Nil }
"#;
        let (bindings, run) = WorkModuleBuilder::new(program(source, provider))
            .expect("plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("seal");
        let mut state = (Arc::new(AtomicUsize::new(0)), ());
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            module
                .attach(guard, &mut state, &mut echo)
                .call(&run, ())
                .expect("verify");
        })
        .now_or_never()
        .expect("ordinary provider call");
        assert_eq!(state.0.load(Ordering::SeqCst), 1);
        assert!(echo.0.is_empty());
    }
}
