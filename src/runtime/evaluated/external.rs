use crate::host::ExternalPayloadLease;
use crate::plan::execution::type_::ExternalTypeId;

#[derive(Clone)]
pub(in crate::runtime) struct EvaluatedExternalValue {
    type_id: ExternalTypeId,
    lease: ExternalPayloadLease,
}

impl EvaluatedExternalValue {
    pub(in crate::runtime) fn new(type_id: ExternalTypeId, lease: ExternalPayloadLease) -> Self {
        Self { type_id, lease }
    }

    pub(in crate::runtime) fn type_id(&self) -> ExternalTypeId {
        self.type_id
    }

    pub(in crate::runtime) fn lease(&self) -> &ExternalPayloadLease {
        &self.lease
    }

    pub(in crate::runtime) fn source_equal(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.lease.source_equal(&other.lease)
    }

    pub(in crate::runtime) fn into_parts(self) -> (ExternalTypeId, ExternalPayloadLease) {
        (self.type_id, self.lease)
    }
}

impl PartialEq for EvaluatedExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.lease.id() == other.lease.id()
    }
}

impl std::fmt::Debug for EvaluatedExternalValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EvaluatedExternalValue")
            .field("type_id", &self.type_id)
            .field("identity", &self.lease.id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluatedExternalValue;
    use crate::host::HostExternalStore;
    use crate::plan::execution::type_::ExternalTypeId;

    #[test]
    fn evaluated_external_value_separates_source_equality_from_runtime_identity() {
        let store = HostExternalStore::default();
        let first = store.insert(
            7usize,
            |left, right| left == right,
            |value| format!("Resource({value})").into(),
        );
        let second = store.insert(
            7usize,
            |left, right| left == right,
            |value| format!("Resource({value})").into(),
        );
        let first = EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second = EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
        let other_type =
            EvaluatedExternalValue::new(ExternalTypeId::new(1), second.lease().clone());

        assert_ne!(first, second);
        assert!(first.source_equal(&second));
        assert!(second.source_equal(&first));
        assert!(!first.source_equal(&other_type));
        assert_eq!(first.lease().inspect(), "Resource(7)");
        assert_eq!(second.lease().inspect(), "Resource(7)");
        assert!(format!("{first:?}").contains("EvaluatedExternalValue"));

        let (type_id, lease) = first.clone().into_parts();
        assert_eq!(type_id, ExternalTypeId::new(0));
        assert_eq!(lease.id(), first.lease().id());
        assert_eq!(first.type_id(), ExternalTypeId::new(0));
    }
}
