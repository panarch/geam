use crate::ast::SrcSpan;
use crate::parse::Token;
use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{error:?} at {location:?}")]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: SrcSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType {
    LexError {
        error: LexicalError,
    },
    ExpectedDefinition,
    ExpectedExpr,
    ExpectedName,
    ExpectedPattern,
    ExpectedType,
    ExpectedUpName,
    UnexpectedToken {
        token: Token,
        expected: Vec<EcoString>,
    },
    UnsupportedSyntax {
        syntax: EcoString,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{error:?} at {location:?}")]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: SrcSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidUnicodeEscapeError {
    MissingOpeningBrace,
    ExpectedHexDigitOrCloseBrace,
    InvalidNumberOfHexDigits,
    InvalidCodepoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexicalErrorType {
    BadStringEscape,
    InvalidUnicodeEscape(InvalidUnicodeEscapeError),
    DigitOutOfRadix,
    NumTrailingUnderscore,
    RadixIntNoValue,
    MissingExponent,
    UnexpectedStringEnd,
    UnrecognizedToken { tok: char },
}

pub(crate) fn parse_error<T>(error: ParseErrorType, location: SrcSpan) -> Result<T, ParseError> {
    Err(ParseError { error, location })
}
