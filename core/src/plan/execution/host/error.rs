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
    UninhabitedCallableCapture { capture: crate::plan::ValueType },
    UninhabitedCallbackArguments { callback: FunctionType },
    ConflictingNativeConversions { type_: crate::plan::ValueType },
}

impl HostSpecializationError {
    pub(in crate::plan::execution) fn uninhabited_callable_capture(
        template: &crate::plan::HostFunctionTemplate,
        signature: FunctionType,
        capture: crate::plan::ValueType,
    ) -> Self {
        Self {
            package: template.package().clone(),
            module: template.module().into(),
            function: template.name().into(),
            signature,
            reason: HostSpecializationErrorReason::UninhabitedCallableCapture { capture },
        }
    }

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

    pub(in crate::plan::execution) fn conflicting_native_conversions(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
        type_: crate::plan::ValueType,
    ) -> Self {
        Self {
            package,
            module,
            function,
            signature,
            reason: HostSpecializationErrorReason::ConflictingNativeConversions { type_ },
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
            HostSpecializationErrorReason::UninhabitedCallableCapture { capture } => write!(
                formatter,
                "host function `{}::{}.{}` constructs a native callable `{:?}` with uninhabited capture `{:?}`",
                self.package, self.module, self.function, self.signature, capture,
            ),
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
            HostSpecializationErrorReason::ConflictingNativeConversions { type_ } => write!(
                formatter,
                "host function `{}::{}.{}` has an executable specialization `{:?}` with conflicting native conversions for `{:?}`",
                self.package, self.module, self.function, self.signature, type_,
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
    fn displays_the_uninhabited_native_capture_with_its_specialization() {
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::Bool);
        let capture = ValueType::Parameter(crate::plan::TypeParameterId(0));
        let error = HostSpecializationError {
            package: "application".into(),
            module: "pricing".into(),
            function: "adjust".into(),
            signature: signature.clone(),
            reason: HostSpecializationErrorReason::UninhabitedCallableCapture {
                capture: capture.clone(),
            },
        };
        assert_eq!(error.package(), "application");
        assert_eq!(error.module(), "pricing");
        assert_eq!(error.function(), "adjust");
        assert_eq!(error.signature(), &signature);
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UninhabitedCallableCapture { capture },
        );
        assert_eq!(
            error.to_string(),
            "host function `application::pricing.adjust` constructs a native callable `FunctionType { arguments: [Int], return_: Bool }` with uninhabited capture `Parameter(TypeParameterId(0))`",
        );
        assert_eq!(error.clone(), error);
    }

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

    #[test]
    fn exposes_the_conflicting_native_conversion_specialization() {
        let signature = FunctionType::new(vec![ValueType::String], ValueType::Bool);
        let error = HostSpecializationError::conflicting_native_conversions(
            "host_support".into(),
            "host/native".into(),
            "convert".into(),
            signature.clone(),
            ValueType::String,
        );
        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/native");
        assert_eq!(error.function(), "convert");
        assert_eq!(error.signature(), &signature);
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::ConflictingNativeConversions {
                type_: ValueType::String,
            }
        );
        assert_eq!(
            error.to_string(),
            "host function `host_support::host/native.convert` has an executable specialization `FunctionType { arguments: [String], return_: Bool }` with conflicting native conversions for `String`"
        );
        assert_eq!(error.clone(), error);
    }
}
