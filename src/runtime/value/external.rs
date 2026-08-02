use crate::host::ExternalPayloadLease;
use crate::plan::ExternalType;
use ecow::EcoString;

#[derive(Clone)]
pub struct ExternalValue {
    type_: ExternalType,
    identity: ExternalValueIdentity,
    inspection: EcoString,
    _lease: ExternalPayloadLease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalValueIdentity(u64);

impl ExternalValue {
    pub(crate) fn from_evaluated(type_: ExternalType, lease: ExternalPayloadLease) -> Self {
        Self {
            type_,
            identity: ExternalValueIdentity(lease.id()),
            inspection: lease.inspect().clone(),
            _lease: lease,
        }
    }

    pub fn type_(&self) -> &ExternalType {
        &self.type_
    }

    pub fn identity(&self) -> ExternalValueIdentity {
        self.identity
    }

    pub fn inspection(&self) -> &EcoString {
        &self.inspection
    }
}

impl PartialEq for ExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl std::fmt::Debug for ExternalValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalValue")
            .field("type_", &self.type_)
            .field("identity", &self.identity)
            .field("inspection", &self.inspection)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalValue;
    use crate::host::HostExternalStore;
    use crate::plan::{ExternalType, ExternalTypeName};

    #[test]
    fn opaque_external_value_exposes_identity_and_inspection_without_payload_access() {
        let store = HostExternalStore::default();
        let first = store.insert(
            7usize,
            |_, left, right| left == right,
            7,
            "Resource(7)".into(),
        );
        let second = store.insert(
            7usize,
            |_, left, right| left == right,
            7,
            "Resource(7)".into(),
        );
        let type_ = ExternalType::new(
            ExternalTypeName::new("domain".into(), "domain/resource".into(), "Resource".into()),
            Vec::new(),
        );
        let stored_equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| false;
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert_eq!(first.source_hash(), 7);
        assert_eq!(first.inspect(), "Resource(7)");
        assert_eq!(second.inspect(), "Resource(7)");
        let first = ExternalValue::from_evaluated(type_.clone(), first);
        let second = ExternalValue::from_evaluated(type_.clone(), second);
        let cloned = first.clone();
        let debug = format!("{first:?}");

        assert_eq!(first.type_(), &type_);
        assert_eq!(first.inspection(), "Resource(7)");
        assert_eq!(first, cloned);
        assert_ne!(first, second);
        assert_eq!(first.identity(), cloned.identity());
        assert!(debug.contains("ExternalValue"));
        assert!(debug.contains("Resource(7)"));
        assert!(!debug.contains("7usize"));
    }
}
