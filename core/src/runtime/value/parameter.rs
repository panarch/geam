use crate::plan::TypeParameterId;
use crate::plan::execution::runtime::RuntimeValueMetadata;
use crate::runtime::{NativeValue, RuntimeListStorage, ValueRetention};
use ecow::EcoString;
use std::fmt;
use std::sync::Arc;

/// A native payload whose source type remains a planning-bound parameter.
///
/// Registered native conversion creates this value. Retaining it neither
/// selects a concrete source type nor grants construction or invocation rights.
#[derive(Clone)]
pub struct ParameterValue(Arc<ParameterPayload>);

struct ParameterPayload {
    parameter: TypeParameterId,
    native: NativeValue,
    owner: ValueRetention,
    lists: RuntimeListStorage,
    requires_execution: bool,
}

impl ParameterValue {
    pub fn parameter(&self) -> TypeParameterId {
        self.0.parameter
    }

    pub(in crate::runtime) fn new(
        parameter: TypeParameterId,
        native: NativeValue,
        metadata: RuntimeValueMetadata<'_>,
        lists: &RuntimeListStorage,
        requires_execution: bool,
    ) -> Self {
        Self(Arc::new(ParameterPayload {
            parameter,
            native,
            owner: ValueRetention::new(metadata),
            lists: lists.clone(),
            requires_execution,
        }))
    }

    pub(in crate::runtime) fn native(&self) -> &NativeValue {
        &self.0.native
    }

    pub(in crate::runtime) fn requires_execution(&self) -> bool {
        self.0.requires_execution
    }

    pub(in crate::runtime) fn source_hash(&self) -> u64 {
        crate::runtime::native::value_hash(&self.0.lists, &self.0.native)
    }

    pub(in crate::runtime) fn inspection(&self) -> EcoString {
        self.0.native.inspect_retained(&self.0.lists)
    }
}

impl PartialEq for ParameterValue {
    fn eq(&self, other: &Self) -> bool {
        self.parameter() == other.parameter()
            && self
                .0
                .owner
                .as_borrowed()
                .metadata
                .shares_owner(other.0.owner.as_borrowed().metadata)
            && crate::runtime::native::values_equal(&self.0.lists, self.native(), other.native())
    }
}

impl fmt::Debug for ParameterValue {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output
            .debug_struct("ParameterValue")
            .field("parameter", &self.parameter())
            .field("native", &self.inspection())
            .field("requires_execution", &self.requires_execution())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::ParameterValue;
    use crate::plan::TypeParameterId;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::{NativeKind, NativeValues, RuntimeListStorage};
    use std::sync::Arc;

    #[test]
    fn parameter_payload_preserves_binding_owner_contents_and_shared_storage() {
        let source = "pub fn main() -> #(List(a), List(b)) { #([], []) }";
        let plan = crate::runtime::plan_src(source);
        let lists = RuntimeListStorage::default();
        let values = NativeValues::new(&lists, plan.value_metadata());
        let root = crate::StringValue::from("hidden:abcdefghijklmnopqrstuvwxyz:hidden");
        let text = root.slice(7..33);
        let pointer = text.as_ptr();
        let native = values.string(text);
        let parameter = TypeParameterId::scoped(0, 0);
        let value: ParameterValue = values.parameter(parameter, &native, false);
        let alias = value.clone();
        let equal = values.parameter(
            parameter,
            &values.string("abcdefghijklmnopqrstuvwxyz".into()),
            false,
        );
        let different_payload = values.parameter(parameter, &values.integer(42.into()), false);
        let different_binding = values.parameter(TypeParameterId::scoped(0, 1), &native, false);
        let foreign_plan = crate::runtime::plan_src(source);
        let foreign = NativeValues::new(&lists, foreign_plan.value_metadata())
            .parameter(parameter, &native, false);

        assert_eq!(value.parameter(), parameter);
        assert_eq!(value.native().kind(), NativeKind::Binary);
        assert!(!value.requires_execution());
        assert_eq!(value, equal);
        assert_ne!(value, different_payload);
        assert_ne!(value, different_binding);
        assert_ne!(value, foreign);
        assert_eq!(value.source_hash(), equal.source_hash());
        assert_eq!(value.source_hash(), values.hash(&native));
        assert_eq!(value.inspection(), "\"abcdefghijklmnopqrstuvwxyz\"");
        assert_eq!(
            format!("{value:?}"),
            r#"ParameterValue { parameter: TypeParameterId { scope: 0, index: 0 }, native: "\"abcdefghijklmnopqrstuvwxyz\"", requires_execution: false }"#,
        );
        assert!(Arc::ptr_eq(&value.0, &alias.0));
        assert_eq!(Arc::strong_count(&value.0), 2);
        drop(value);
        drop(native);
        drop(root);
        drop(plan);
        assert_eq!(Arc::strong_count(&alias.0), 1);
        assert_eq!(
            alias.native().as_string().map(|value| value.as_ptr()),
            Some(pointer)
        );
        assert_eq!(alias.inspection(), "\"abcdefghijklmnopqrstuvwxyz\"");
    }
}
