use crate::plan::FunctionType;
use ecow::EcoString;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSpecializationError {
    package: EcoString,
    module: EcoString,
    function: EcoString,
    signature: FunctionType,
}

impl HostSpecializationError {
    pub(in crate::plan::execution) fn new(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
    ) -> Self {
        Self {
            package,
            module,
            function,
            signature,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn signature(&self) -> &FunctionType {
        &self.signature
    }
}

impl fmt::Display for HostSpecializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "host function `{}::{}.{}` has an executable specialization `{:?}` whose successful return storage cannot be determined",
            self.package, self.module, self.function, self.signature,
        )
    }
}

impl std::error::Error for HostSpecializationError {}

#[cfg(test)]
mod tests {
    use super::HostSpecializationError;
    use crate::{FunctionType, ValueType};

    #[test]
    fn exposes_the_unrepresentable_specialization_identity() {
        let signature = FunctionType::new(
            Vec::new(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let error = HostSpecializationError::new(
            "host_support".into(),
            "host/generic".into(),
            "produce".into(),
            signature.clone(),
        );

        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/generic");
        assert_eq!(error.function(), "produce");
        assert_eq!(error.signature(), &signature);
        assert_eq!(
            error.to_string(),
            "host function `host_support::host/generic.produce` has an executable specialization `FunctionType { arguments: [], return_: Parameter(TypeParameterId(0)) }` whose successful return storage cannot be determined",
        );
        assert_eq!(error.clone(), error);
    }
}
