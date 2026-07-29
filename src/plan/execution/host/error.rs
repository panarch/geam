use crate::plan::FunctionType;
use ecow::EcoString;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSpecializationError {
    package: EcoString,
    module: EcoString,
    function: EcoString,
    signature: FunctionType,
    reason: HostSpecializationErrorReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostSpecializationErrorReason {
    UndeterminedReturnStorage,
    UninhabitedCallbackArguments { callback: FunctionType },
}

impl HostSpecializationError {
    pub(in crate::plan::execution) fn undetermined_return_storage(
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
            reason: HostSpecializationErrorReason::UndeterminedReturnStorage,
        }
    }

    pub(in crate::plan::execution) fn uninhabited_callback_arguments(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
        callback: FunctionType,
    ) -> Self {
        Self {
            package,
            module,
            function,
            signature,
            reason: HostSpecializationErrorReason::UninhabitedCallbackArguments { callback },
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

    pub fn reason(&self) -> &HostSpecializationErrorReason {
        &self.reason
    }
}

impl fmt::Display for HostSpecializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.reason {
            HostSpecializationErrorReason::UndeterminedReturnStorage => write!(
                formatter,
                "host function `{}::{}.{}` has an executable specialization `{:?}` whose successful return storage cannot be determined",
                self.package, self.module, self.function, self.signature,
            ),
            HostSpecializationErrorReason::UninhabitedCallbackArguments { callback } => write!(
                formatter,
                "host function `{}::{}.{}` has an executable specialization `{:?}` that exposes callback `{:?}` with uninhabited arguments",
                self.package, self.module, self.function, self.signature, callback,
            ),
        }
    }
}

impl std::error::Error for HostSpecializationError {}

#[cfg(test)]
mod tests {
    use super::{HostSpecializationError, HostSpecializationErrorReason};
    use crate::{FunctionType, ValueType};

    #[test]
    fn exposes_the_undetermined_return_storage_specialization() {
        let signature = FunctionType::new(
            Vec::new(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let error = HostSpecializationError::undetermined_return_storage(
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
            error.reason(),
            &HostSpecializationErrorReason::UndeterminedReturnStorage,
        );
        assert_eq!(
            error.to_string(),
            "host function `host_support::host/generic.produce` has an executable specialization `FunctionType { arguments: [], return_: Parameter(TypeParameterId(0)) }` whose successful return storage cannot be determined",
        );
        assert_eq!(error.clone(), error);
    }

    #[test]
    fn exposes_the_uninhabited_callback_specialization() {
        let callback = FunctionType::new(
            vec![ValueType::Parameter(crate::plan::TypeParameterId(0))],
            ValueType::Int,
        );
        let signature = FunctionType::new(
            vec![ValueType::Function(Box::new(callback.clone()))],
            ValueType::Int,
        );
        let error = HostSpecializationError::uninhabited_callback_arguments(
            "host_support".into(),
            "host/function".into(),
            "apply".into(),
            signature.clone(),
            callback.clone(),
        );

        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/function");
        assert_eq!(error.function(), "apply");
        assert_eq!(error.signature(), &signature);
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                callback: callback.clone(),
            },
        );
        assert_eq!(
            error.to_string(),
            "host function `host_support::host/function.apply` has an executable specialization `FunctionType { arguments: [Function(FunctionType { arguments: [Parameter(TypeParameterId(0))], return_: Int })], return_: Int }` that exposes callback `FunctionType { arguments: [Parameter(TypeParameterId(0))], return_: Int }` with uninhabited arguments",
        );
        assert_eq!(error.clone(), error);
    }
}
