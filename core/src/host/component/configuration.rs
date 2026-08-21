use ecow::EcoString;
use std::collections::BTreeMap;

/// Owned, read-only configuration supplied to one provider component.
#[derive(Debug, Clone, PartialEq)]
pub struct HostProviderConfiguration {
    values: BTreeMap<EcoString, HostProviderConfigurationValue>,
}

/// Portable value families accepted by provider component configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum HostProviderConfigurationValue {
    String(EcoString),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Self>),
    Table(HostProviderConfiguration),
}

impl HostProviderConfiguration {
    pub fn new(values: BTreeMap<EcoString, HostProviderConfigurationValue>) -> Self {
        Self { values }
    }

    pub fn empty() -> Self {
        Self::new(BTreeMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&HostProviderConfigurationValue> {
        self.values.get(key)
    }

    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&EcoString, &HostProviderConfigurationValue)> {
        self.values.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl HostProviderConfigurationValue {
    pub fn as_string(&self) -> Option<&EcoString> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_table(&self) -> Option<&HostProviderConfiguration> {
        match self {
            Self::Table(value) => Some(value),
            _ => None,
        }
    }
}

impl From<EcoString> for HostProviderConfigurationValue {
    fn from(value: EcoString) -> Self {
        Self::String(value)
    }
}

impl From<&str> for HostProviderConfigurationValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<i64> for HostProviderConfigurationValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for HostProviderConfigurationValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for HostProviderConfigurationValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<HostProviderConfigurationValue>> for HostProviderConfigurationValue {
    fn from(value: Vec<HostProviderConfigurationValue>) -> Self {
        Self::Array(value)
    }
}

impl From<HostProviderConfiguration> for HostProviderConfigurationValue {
    fn from(value: HostProviderConfiguration) -> Self {
        Self::Table(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostProviderConfiguration, HostProviderConfigurationValue};
    use ecow::EcoString;
    use std::collections::BTreeMap;

    #[test]
    fn configuration_preserves_every_owned_value_family() {
        let nested = HostProviderConfiguration::new(BTreeMap::from([(
            EcoString::from("enabled"),
            true.into(),
        )]));
        let configuration = HostProviderConfiguration::new(BTreeMap::from([
            (
                EcoString::from("array"),
                vec![1_i64.into(), false.into()].into(),
            ),
            (EcoString::from("float"), 1.5.into()),
            (EcoString::from("integer"), (-7_i64).into()),
            (EcoString::from("string"), "value".into()),
            (EcoString::from("table"), nested.into()),
        ]));

        assert!(!configuration.is_empty());
        assert_eq!(configuration.iter().len(), 5);
        assert_eq!(
            configuration
                .get("string")
                .and_then(|value| value.as_string()),
            Some(&EcoString::from("value"))
        );
        assert_eq!(
            configuration
                .get("integer")
                .and_then(HostProviderConfigurationValue::as_integer),
            Some(-7)
        );
        assert_eq!(
            configuration
                .get("float")
                .and_then(HostProviderConfigurationValue::as_float),
            Some(1.5)
        );
        let array = configuration
            .get("array")
            .and_then(HostProviderConfigurationValue::as_array)
            .expect("array should be present");
        assert_eq!(array[0].as_integer(), Some(1));
        assert_eq!(array[1].as_bool(), Some(false));
        assert_eq!(
            configuration
                .get("table")
                .and_then(HostProviderConfigurationValue::as_table)
                .and_then(|table| table.get("enabled"))
                .and_then(HostProviderConfigurationValue::as_bool),
            Some(true)
        );
        assert_eq!(configuration.clone(), configuration);
        assert!(HostProviderConfiguration::empty().is_empty());
    }

    #[test]
    fn configuration_accessors_do_not_coerce_value_families() {
        let value = HostProviderConfigurationValue::String("text".into());
        let owned = HostProviderConfigurationValue::from(EcoString::from("owned"));

        assert_eq!(value.as_string().map(EcoString::as_str), Some("text"));
        assert_eq!(owned.as_string().map(EcoString::as_str), Some("owned"));
        assert_eq!(value.as_integer(), None);
        assert_eq!(value.as_float(), None);
        assert_eq!(value.as_bool(), None);
        assert_eq!(value.as_array(), None);
        assert_eq!(value.as_table(), None);
        assert_eq!(HostProviderConfigurationValue::Integer(1).as_string(), None);
    }
}
