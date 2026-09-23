use super as admission;
use crate::embedding::BindingError;
use std::convert::Infallible;
use std::fmt;

/// A prepared-data or Rust declaration mismatch detected before execution.
#[derive(Debug)]
pub struct PreparedError {
    kind: Box<Kind>,
}

#[derive(Debug)]
enum Kind {
    Provider(crate::HostRegistrationError),
    Format(admission::FormatError),
    Artifact(admission::Error<Infallible>),
    Hosted(admission::Error<admission::hosts::NativeError>),
    Registration(admission::hosts::NativeError),
    Selection(admission::SelectionError),
}

impl fmt::Display for PreparedError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind.as_ref() {
            Kind::Provider(error) => error.fmt(output),
            Kind::Format(admission::FormatError { expected, found }) => write!(
                output,
                "prepared format {found} is incompatible with format {expected}; regenerate the prepared program"
            ),
            Kind::Artifact(error) => write!(
                output,
                "invalid prepared program: {error:?}; regenerate the prepared program"
            ),
            Kind::Hosted(error) => write!(
                output,
                "invalid prepared program: {error:?}; regenerate the prepared program"
            ),
            Kind::Registration(error) => write!(
                output,
                "prepared provider registration mismatch: {error:?}; regenerate with the matching providers"
            ),
            Kind::Selection(admission::SelectionError::Binding(error)) => error.fmt(output),
            Kind::Selection(admission::SelectionError::Callable { name, error }) => write!(
                output,
                "function {name} has incompatible prepared callable contracts: {error:?}; regenerate with the matching declarations"
            ),
            Kind::Selection(admission::SelectionError::Input { name, error }) => write!(
                output,
                "function {name} has incompatible prepared Rust inputs: {error:?}; regenerate with the matching declarations"
            ),
        }
    }
}

impl std::error::Error for PreparedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind.as_ref() {
            Kind::Provider(error) => Some(error),
            Kind::Selection(admission::SelectionError::Binding(error)) => Some(error),
            Kind::Format(_)
            | Kind::Artifact(_)
            | Kind::Hosted(_)
            | Kind::Registration(_)
            | Kind::Selection(admission::SelectionError::Input { .. })
            | Kind::Selection(admission::SelectionError::Callable { .. }) => None,
        }
    }
}

impl From<crate::HostRegistrationError> for PreparedError {
    fn from(error: crate::HostRegistrationError) -> Self {
        Self {
            kind: Box::new(Kind::Provider(error)),
        }
    }
}

impl From<admission::Error<Infallible>> for PreparedError {
    fn from(error: admission::Error<Infallible>) -> Self {
        Self {
            kind: Box::new(Kind::Artifact(error)),
        }
    }
}

impl From<admission::FormatError> for PreparedError {
    fn from(error: admission::FormatError) -> Self {
        Self {
            kind: Box::new(Kind::Format(error)),
        }
    }
}

impl From<admission::Error<admission::hosts::NativeError>> for PreparedError {
    fn from(error: admission::Error<admission::hosts::NativeError>) -> Self {
        Self {
            kind: Box::new(Kind::Hosted(error)),
        }
    }
}

impl From<admission::hosts::NativeError> for PreparedError {
    fn from(error: admission::hosts::NativeError) -> Self {
        Self {
            kind: Box::new(Kind::Registration(error)),
        }
    }
}

impl From<admission::SelectionError> for PreparedError {
    fn from(error: admission::SelectionError) -> Self {
        Self {
            kind: Box::new(Kind::Selection(error)),
        }
    }
}

impl From<BindingError> for PreparedError {
    fn from(error: BindingError) -> Self {
        Self::from(admission::SelectionError::Binding(error))
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingError, Infallible, PreparedError, admission};
    use std::error::Error;

    #[test]
    fn distinguishes_artifact_and_declaration_failures_before_execution() {
        let cases = [
            (
                PreparedError::from(crate::HostRegistrationError::InvalidModuleName {
                    module: "invalid!".into(),
                }),
                "host module name invalid! is invalid",
                true,
            ),
            (
                PreparedError::from(admission::FormatError {
                    expected: 1,
                    found: 2,
                }),
                "prepared format 2 is incompatible with format 1; regenerate the prepared program",
                false,
            ),
            (
                PreparedError::from(admission::Error::<Infallible>::Entries(
                    admission::entry::EntryError::Empty,
                )),
                "invalid prepared program: Entries(Empty); regenerate the prepared program",
                false,
            ),
            (
                PreparedError::from(admission::Error::<admission::hosts::NativeError>::Entries(
                    admission::entry::EntryError::Empty,
                )),
                "invalid prepared program: Entries(Empty); regenerate the prepared program",
                false,
            ),
            (
                PreparedError::from(admission::hosts::NativeError::Registration {
                    package: "package".into(),
                    module: "module".into(),
                    function: "load".into(),
                    reason: admission::hosts::RegistrationError::Missing,
                }),
                "prepared provider registration mismatch: Registration { package: \"package\", module: \"module\", function: \"load\", reason: Missing }; regenerate with the matching providers",
                false,
            ),
            (
                PreparedError::from(BindingError::MissingFunction {
                    name: "missing".into(),
                }),
                "function missing does not exist in the Gleam module",
                true,
            ),
            (
                PreparedError::from(admission::SelectionError::Input {
                    name: "echo".into(),
                    error: admission::input::InputError::VariantCount {
                        expected: 1,
                        actual: 0,
                    },
                }),
                "function echo has incompatible prepared Rust inputs: VariantCount { expected: 1, actual: 0 }; regenerate with the matching declarations",
                false,
            ),
            (
                PreparedError::from(admission::SelectionError::Callable {
                    name: "echo".into(),
                    error: admission::callables::CallableError::Count {
                        expected: 1,
                        actual: 0,
                    },
                }),
                "function echo has incompatible prepared callable contracts: Count { expected: 1, actual: 0 }; regenerate with the matching declarations",
                false,
            ),
        ];
        for (error, expected, has_source) in cases {
            assert_eq!(error.to_string(), expected);
            assert_eq!(error.source().is_some(), has_source);
        }
    }
}
