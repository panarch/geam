use crate::plan::FunctionType;
use ecow::EcoString;
use thiserror::Error;

/// A failure while selecting typed functions from a Gleam module.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BindingError {
    #[error("function {name} does not exist in the Gleam module")]
    MissingFunction { name: EcoString },
    #[error("function {name} is not public")]
    NonPublicFunction { name: EcoString },
    #[error("function {name} was selected more than once")]
    DuplicateFunction { name: EcoString },
    #[error("generic function {name} cannot be embedded without a concrete specialization")]
    GenericFunction { name: EcoString },
    #[error("function {name} has type {found:?}, expected {expected:?}")]
    SignatureMismatch {
        name: EcoString,
        expected: FunctionType,
        found: FunctionType,
    },
}

#[cfg(test)]
mod tests {
    use super::BindingError;
    use crate::{FunctionType, ValueType};

    #[test]
    fn displays_each_named_binding_failure() {
        let cases = [
            (
                BindingError::MissingFunction {
                    name: "missing".into(),
                },
                "function missing does not exist in the Gleam module",
            ),
            (
                BindingError::NonPublicFunction {
                    name: "private".into(),
                },
                "function private is not public",
            ),
            (
                BindingError::DuplicateFunction {
                    name: "duplicate".into(),
                },
                "function duplicate was selected more than once",
            ),
            (
                BindingError::GenericFunction {
                    name: "identity".into(),
                },
                "generic function identity cannot be embedded without a concrete specialization",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
            assert_eq!(error.clone(), error);
        }
    }

    #[test]
    fn displays_an_exact_signature_mismatch() {
        let expected = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let found = FunctionType::new(vec![ValueType::String], ValueType::String);
        let error = BindingError::SignatureMismatch {
            name: "convert".into(),
            expected: expected.clone(),
            found: found.clone(),
        };

        assert_eq!(
            error.to_string(),
            "function convert has type FunctionType { arguments: [String], return_: String }, expected FunctionType { arguments: [Int], return_: Int }",
        );
        assert_eq!(
            error,
            BindingError::SignatureMismatch {
                name: "convert".into(),
                expected,
                found,
            },
        );
    }
}
