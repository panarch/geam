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

    #[error(
        "host module {module} was registered by both package {first_package} and package {second_package}"
    )]
    DuplicateModule {
        module: EcoString,
        first_package: EcoString,
        second_package: EcoString,
    },
}
