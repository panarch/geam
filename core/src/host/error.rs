use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HostRegistrationError {
    #[error("host module name {module} is invalid")]
    InvalidModuleName { module: EcoString },

    #[error("host function name {function} in module {module} is invalid")]
    InvalidFunctionName {
        module: EcoString,
        function: EcoString,
    },

    #[error("host function {function} was registered more than once in module {module}")]
    DuplicateFunction {
        module: EcoString,
        function: EcoString,
    },

    #[error("host external type name {type_} in module {module} is invalid")]
    InvalidExternalTypeName { module: EcoString, type_: EcoString },

    #[error("host external type {type_} was registered more than once in module {module}")]
    DuplicateExternalType { module: EcoString, type_: EcoString },

    #[error(
        "host function {function} uses type parameter indices {parameters:?}; indices must be contiguous from zero"
    )]
    NonContiguousTypeParameters {
        function: EcoString,
        parameters: Box<[usize]>,
    },

    #[error(
        "host function {function} registers construction type parameter indices {parameters:?} that do not occur in its signature"
    )]
    UnboundConstructionTypeParameters {
        function: EcoString,
        parameters: Box<[usize]>,
    },

    #[error("host function {function} registers native conversion for {type_:?} more than once")]
    DuplicateNativeConversion {
        function: EcoString,
        type_: crate::plan::ValueType,
    },

    #[error(
        "host module {module} was registered by both package {first_package} and package {second_package}"
    )]
    DuplicateModule {
        module: EcoString,
        first_package: EcoString,
        second_package: EcoString,
    },
}

#[cfg(test)]
mod tests {
    use super::HostRegistrationError;
    use crate::plan::{ExternalType, ExternalTypeName, ValueType};

    #[test]
    fn duplicate_native_conversion_identifies_the_declared_target() {
        let error = HostRegistrationError::DuplicateNativeConversion {
            function: "convert".into(),
            type_: ValueType::External(ExternalType::new(
                ExternalTypeName::new("host_support".into(), "host/native".into(), "Name".into()),
                Vec::new(),
            )),
        };
        assert_eq!(
            error.to_string(),
            "host function convert registers native conversion for External(ExternalType { name: ExternalTypeName { package: \"host_support\", module: \"host/native\", name: \"Name\" }, arguments: [] }) more than once"
        );
    }
}
