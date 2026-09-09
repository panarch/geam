use crate::host::HostCodecScope;
use crate::host::{
    ExternalPayloadLease, HostCallArguments, HostCustomArgumentSlot, HostCustomToken,
    HostExternalArgumentSlot, HostExternalToken, HostFunctionArgumentSlot, HostFunctionToken,
    HostListArgumentSlot, HostListToken, HostProfile, HostScopedValue, HostTupleArgumentSlot,
    HostTupleToken, HostValueArgumentSlot, HostValueToken,
};
use crate::runtime::{StoredRuntimeList, StoredRuntimeValue};

pub(crate) trait HostTokenRuntime {
    fn int(&self, value: HostValueToken) -> num_bigint::BigInt;
    fn float(&self, value: HostValueToken) -> f64;
    fn string(&self, value: HostValueToken) -> ecow::EcoString;
    fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue;
    fn utf_codepoint(&self, value: HostValueToken) -> char;
    fn bool(&self, value: HostValueToken) -> bool;
    fn nil(&self, value: HostValueToken);
    fn list_token(&self, value: HostValueToken) -> HostListToken;
    fn tuple_token(&self, value: HostValueToken) -> HostTupleToken;
    fn custom_token(&self, value: HostValueToken) -> HostCustomToken;
    fn external_token(&self, value: HostValueToken) -> HostExternalToken;
    fn function_token(&self, value: HostValueToken) -> HostFunctionToken;
}

pub(crate) trait HostCallRuntime<Profile: HostProfile>: HostTokenRuntime {
    fn state(&mut self) -> &mut Profile::RunState;
    fn external_stores(&self) -> &Profile::ExternalStores;
    fn arguments(&self) -> &dyn HostCallArguments;
    fn scalar_context(&mut self) -> (&mut Profile::RunState, &dyn HostCallArguments);
    fn value(&self, slot: HostValueArgumentSlot) -> HostValueToken;
    fn list(&self, slot: HostListArgumentSlot) -> HostListToken;
    fn tuple(&self, slot: HostTupleArgumentSlot) -> HostTupleToken;
    fn custom(&self, slot: HostCustomArgumentSlot) -> HostCustomToken;
    fn external(&self, slot: HostExternalArgumentSlot) -> HostExternalToken;
    fn function(&self, slot: HostFunctionArgumentSlot) -> HostFunctionToken;

    fn list_len(&self, value: HostListToken) -> usize;
    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken>;
    fn tuple_len(&self, value: HostTupleToken) -> usize;
    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]>;
    fn custom_constructor(&self, value: HostCustomToken) -> usize;
    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]>;
    fn take_custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]>;
    fn invoke(
        &mut self,
        function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, crate::HostCallError>;
    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool;
    fn source_hash(&self, value: HostScopedValue) -> u64;
    fn inspect(&self, value: HostScopedValue) -> ecow::EcoString;
    fn complete(&mut self, value: HostScopedValue) -> HostValueToken;
    fn build_list(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        values: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken;
    fn build_native_list(
        &mut self,
        type_: crate::plan::execution::type_::ListTypeId,
        values: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_native_custom(
        &mut self,
        constructor: crate::plan::execution::type_::CustomConstructorId,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_custom(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken;
    fn build_external(
        &mut self,
        type_: &crate::host::HostTypeDescriptor,
        value: ExternalPayloadLease,
    ) -> HostExternalToken;
    fn external_lease(&self, value: HostExternalToken) -> ExternalPayloadLease;
    fn resolve_host_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType>;
    fn retain_stored(&self, value: HostScopedValue) -> crate::runtime::StoredRuntimeValue;
    fn retain_list(&self, value: HostListToken) -> crate::runtime::StoredRuntimeList;
    fn restore_stored(&mut self, value: &crate::runtime::StoredRuntimeValue) -> HostValueToken;
    fn owns_stored(&self, value: &StoredRuntimeValue) -> bool;

    fn work(&self) -> crate::runtime::work::execution::WorkContext<Profile>;
    fn origin(&self) -> crate::runtime::HostCallOrigin;
    fn callable(&self, function: HostFunctionToken) -> crate::runtime::RetainedCallable;
    fn codec_scope(&self) -> HostCodecScope;
    fn stored_equal(&self, left: &StoredRuntimeValue, right: &StoredRuntimeValue) -> bool;
    fn native_equal(
        &self,
        left: &crate::runtime::NativeValue,
        right: &crate::runtime::NativeValue,
    ) -> bool;
    fn native_hash(&self, value: &crate::runtime::NativeValue) -> u64;
    fn native_tuple(&self, value: HostListToken) -> crate::runtime::NativeValue;
    fn stored_source_hash(&self, value: &StoredRuntimeValue) -> u64;
    fn stored_inspect(&self, value: &StoredRuntimeValue) -> ecow::EcoString;
    fn stored_list_len(&self, value: &StoredRuntimeValue) -> usize;
    fn stored_list_item(
        &self,
        value: &StoredRuntimeValue,
        index: usize,
    ) -> Option<StoredRuntimeValue>;
    fn restore_list(&mut self, value: &StoredRuntimeList) -> HostListToken;
    fn callback_inputs(&self, values: Box<[HostScopedValue]>) -> crate::runtime::CallbackInputs;
}

#[cfg(test)]
pub(crate) use crate::runtime::host_call_fixture as test;

#[cfg(test)]
mod tests {

    use crate::host::{HostCallRuntime, HostTokenRuntime};
    use crate::host::{
        HostExternalToken, HostScopedValue, HostStoredValue, HostTypeParameter, HostValueFamily,
        HostValueToken,
    };

    use super::test::{TestHostCallRuntime, TestRunState, token};
    #[test]
    fn test_runtime_preserves_external_tokens_and_payload_leases() {
        let store = crate::host::HostExternalStore::default();
        let source_hash = |_: &crate::host::HostExternalHashing<'_>, value: &usize| *value as u64;
        let inspect = |context: &crate::host::HostExternalInspection<'_>, value: &usize| {
            let stored = HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int((*value).into()),
            );
            format!("Resource({})", context.inspect_stored_value(&stored)).into()
        };
        let lease = store.insert(
            7usize,
            |_, left, right| left == right,
            source_hash,
            inspect,
            |_| None,
        );
        let equal_lease = store.insert(
            7usize,
            |_, left, right| left == right,
            source_hash,
            inspect,
            |_| None,
        );
        let identity = lease.identity();
        let mut state = TestRunState::default();
        let arguments = crate::host::function::CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(HostCallRuntime::external_stores(&runtime), &());
        assert_eq!(
            HostTokenRuntime::external_token(
                &runtime,
                HostValueToken {
                    family: HostValueFamily::External,
                    index: 0,
                },
            ),
            HostExternalToken(0),
        );
        let descriptor = crate::host::HostTypeDescriptor::External {
            schema: crate::host::HostExternalTypeSchema::new(
                "domain",
                "domain/resource",
                "Resource",
                0,
            ),
            arguments: Box::new([]),
        };
        let external = HostCallRuntime::build_external(&mut runtime, &descriptor, lease);
        assert_eq!(external, HostExternalToken(0));
        let list_item =
            HostCallRuntime::build_external(&mut runtime, &descriptor, equal_lease.clone());
        assert_eq!(list_item, HostExternalToken(1));
        let stored = HostCallRuntime::external_lease(&runtime, external);
        {
            let stored_equal =
                |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
            let equality = crate::host::RetainedValueEquality::new(&stored_equal);
            let left = HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            );
            let right = HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            );
            assert!(
                !crate::host::HostExternalEquality(&equality).stored_values_equal(&left, &right)
            );
            assert!(stored.source_equal(&equality, &equal_lease));
            assert!(equal_lease.source_equal(&equality, &stored));
        }
        let source_hash = |_: &crate::runtime::RetainedValueRef| 23;
        let inspect = |_: &crate::runtime::RetainedValueRef| "7".into();
        let hashing = crate::host::RetainedValueHashing::new(&source_hash);
        let inspection = crate::host::RetainedValueInspection::new(&inspect);
        assert_eq!(stored.source_hash(&hashing), 7);
        assert_eq!(stored.inspection(&inspection), "Resource(7)");
        assert_eq!(equal_lease.inspection(&inspection), "Resource(7)");
        assert_eq!(stored.identity(), identity,);
        assert_eq!(
            HostCallRuntime::complete(&mut runtime, HostScopedValue::External(external)).family,
            HostValueFamily::External,
        );
    }

    #[test]
    fn test_runtime_retains_and_restores_stored_values() {
        let mut state = TestRunState::default();
        let arguments = crate::host::function::CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let stored = HostCallRuntime::retain_stored(&runtime, HostScopedValue::Int(0.into()));

        assert_eq!(
            HostCallRuntime::restore_stored(&mut runtime, &stored),
            token(HostValueFamily::Int),
        );
    }

    #[test]
    fn test_runtime_resolves_only_monomorphic_host_types() {
        let mut state = TestRunState::default();
        let arguments = crate::host::function::CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            HostCallRuntime::resolve_host_type(
                &runtime,
                &crate::host::HostTypeDescriptor::of::<num_bigint::BigInt>(),
            ),
            Some(crate::plan::ValueType::Int),
        );
        assert_eq!(
            HostCallRuntime::resolve_host_type(
                &runtime,
                &crate::host::HostTypeDescriptor::of::<HostTypeParameter<0>>(),
            ),
            None,
        );
    }
}
