use crate::plan::{
    FunctionReturnFamily, FunctionType, PanicSite, SourceContext, SourceSpan, Value, ValueType,
};
use ecow::EcoString;
use miette::{Diagnostic, LabeledSpan, NamedSource, SourceCode};
use std::fmt;

pub(crate) type ExecutionResult<T> = Result<T, ExecutionError>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ExecutionError {
    #[error("{0}")]
    Panic(Panic),
    #[error("function return family mismatch (expected {expected}, got {actual})")]
    FunctionReturnFamilyMismatch {
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    },
    #[error("tuple index family mismatch (expected {expected:?}, got {actual:?})")]
    TupleIndexFamilyMismatch {
        expected: ValueType,
        actual: ValueType,
    },
}

#[derive(Debug, Clone)]
pub struct Panic {
    kind: PanicKind,
    message: PanicMessage,
    site: PanicSite,
    source: Option<Box<NamedSource<String>>>,
    details: Option<Box<PanicDetails>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanicKind {
    Panic,
    Todo,
    Assert,
    LetAssert,
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanicMessage {
    Default,
    Explicit(EcoString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanicDetails {
    LetAssert {
        value: Value,
        pattern_span: SourceSpan,
    },
}

impl Panic {
    pub(crate) fn new(
        kind: PanicKind,
        message: PanicMessage,
        site: PanicSite,
        source_context: Option<&SourceContext>,
        details: Option<PanicDetails>,
    ) -> Self {
        Self {
            kind,
            message,
            site,
            source: source_context
                .map(SourceContext::named_source)
                .map(Box::new),
            details: details.map(Box::new),
        }
    }

    pub fn kind(&self) -> PanicKind {
        self.kind
    }

    pub fn message(&self) -> &PanicMessage {
        &self.message
    }

    pub fn site(&self) -> &PanicSite {
        &self.site
    }

    pub fn details(&self) -> Option<&PanicDetails> {
        self.details.as_deref()
    }

    fn message_text(&self) -> std::borrow::Cow<'_, str> {
        self.message.text(self.kind)
    }

    fn primary_label(&self) -> String {
        format!(
            "{} in {}.{}",
            self.kind.label(),
            self.site.module(),
            self.site.function(),
        )
    }
}

impl PartialEq for Panic {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.message == other.message
            && self.site == other.site
            && self.details() == other.details()
            && named_source_eq(self.source.as_deref(), other.source.as_deref())
    }
}

fn named_source_eq(
    left: Option<&NamedSource<String>>,
    right: Option<&NamedSource<String>>,
) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.name() == right.name() && left.inner() == right.inner(),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}

impl fmt::Display for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.code(), self.message_text())
    }
}

impl std::error::Error for Panic {}

impl Diagnostic for Panic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(format!("geam::{}", self.kind.code())))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self.details.as_deref() {
            Some(PanicDetails::LetAssert { value, .. }) => {
                Some(Box::new(format!("failed value: {}", render_value(value))))
            }
            None => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source
            .as_deref()
            .map(|source| source as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.source.as_ref()?;

        let mut labels = vec![LabeledSpan::new_primary_with_span(
            Some(self.primary_label()),
            self.site.span().to_miette(),
        )];
        if let Some(PanicDetails::LetAssert { pattern_span, .. }) = self.details.as_deref() {
            labels.push(LabeledSpan::at(pattern_span.to_miette(), "pattern"));
        }

        Some(Box::new(labels.into_iter()))
    }
}

impl PanicKind {
    fn code(&self) -> &'static str {
        match self {
            Self::Panic => "panic",
            Self::Todo => "todo",
            Self::Assert => "assert",
            Self::LetAssert => "let_assert",
            Self::EmptyFunction => "empty_function",
            Self::EmptyBlock => "empty_block",
            Self::IncompleteUse => "incomplete_use",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Panic => "panic",
            Self::Todo => "todo",
            Self::Assert => "assert",
            Self::LetAssert => "let assert",
            Self::EmptyFunction => "empty function",
            Self::EmptyBlock => "empty block",
            Self::IncompleteUse => "incomplete use",
        }
    }
}

impl PanicMessage {
    pub(crate) fn from_optional_explicit(message: Option<EcoString>) -> Self {
        match message {
            Some(message) => Self::Explicit(message),
            None => Self::Default,
        }
    }

    fn text(&self, kind: PanicKind) -> std::borrow::Cow<'_, str> {
        match self {
            Self::Explicit(message) => std::borrow::Cow::Borrowed(message.as_str()),
            Self::Default => std::borrow::Cow::Borrowed(kind.default_message()),
        }
    }
}

impl PanicKind {
    fn default_message(&self) -> &'static str {
        match self {
            Self::Panic => "`panic` expression evaluated.",
            Self::Todo => "`todo` expression evaluated. This code has not yet been implemented.",
            Self::Assert => "Assertion failed.",
            Self::LetAssert => "Pattern match failed, no pattern matched the value.",
            Self::EmptyFunction => "Function body is empty.",
            Self::EmptyBlock => "Block is empty.",
            Self::IncompleteUse => "Use callback is incomplete.",
        }
    }
}

impl Diagnostic for ExecutionError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Panic(panic) => panic.code(),
            Self::FunctionReturnFamilyMismatch { .. } => {
                Some(Box::new("geam::function_return_family_mismatch"))
            }
            Self::TupleIndexFamilyMismatch { .. } => Some(Box::new("geam::tuple_index_mismatch")),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Panic(panic) => panic.help(),
            Self::FunctionReturnFamilyMismatch { .. } | Self::TupleIndexFamilyMismatch { .. } => {
                None
            }
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            Self::Panic(panic) => panic.source_code(),
            Self::FunctionReturnFamilyMismatch { .. } | Self::TupleIndexFamilyMismatch { .. } => {
                None
            }
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Self::Panic(panic) => panic.labels(),
            Self::FunctionReturnFamilyMismatch { .. } | Self::TupleIndexFamilyMismatch { .. } => {
                None
            }
        }
    }
}

impl ExecutionError {
    #[cfg(test)]
    pub(crate) fn panic(kind: PanicKind) -> Self {
        Self::Panic(Panic::new(
            kind,
            PanicMessage::Default,
            PanicSite::unknown(),
            None,
            None,
        ))
    }

    pub(crate) fn source_panic(
        source_context: Option<&SourceContext>,
        kind: PanicKind,
        message: Option<EcoString>,
        site: PanicSite,
    ) -> Self {
        Self::Panic(Panic::new(
            kind,
            PanicMessage::from_optional_explicit(message),
            site,
            source_context,
            None,
        ))
    }

    pub(crate) fn let_assert_panic(
        source_context: Option<&SourceContext>,
        message: Option<EcoString>,
        site: PanicSite,
        value: Value,
        pattern_span: SourceSpan,
    ) -> Self {
        Self::Panic(Panic::new(
            PanicKind::LetAssert,
            PanicMessage::from_optional_explicit(message),
            site,
            source_context,
            Some(PanicDetails::LetAssert {
                value,
                pattern_span,
            }),
        ))
    }

    pub(crate) fn function_return_family_mismatch(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> Self {
        Self::FunctionReturnFamilyMismatch { expected, actual }
    }

    pub(crate) fn tuple_index_family_mismatch(expected: ValueType, actual: ValueType) -> Self {
        Self::TupleIndexFamilyMismatch { expected, actual }
    }
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::Float(value) => format!("Float({value:?})"),
        Value::String(value) => format!("String({value:?})"),
        Value::Bool(value) => format!("Bool({value})"),
        Value::Nil => "Nil".into(),
        Value::Tuple(values) => format!(
            "Tuple([{}])",
            values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::List(value) => format!(
            "List({})([{}])",
            render_value_type(value.element_type()),
            value
                .values()
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Function(function) => {
            let type_ = function.type_();
            format!("Function({})", render_function_type(&type_))
        }
    }
}

fn render_function_type(type_: &FunctionType) -> String {
    let arguments = type_
        .argument_types()
        .iter()
        .map(render_value_type)
        .collect::<Vec<_>>()
        .join(", ");

    format!("fn({arguments}) -> {}", render_value_type(type_.return_()))
}

fn render_value_type(type_: &ValueType) -> String {
    match type_ {
        ValueType::Int => "Int".into(),
        ValueType::Float => "Float".into(),
        ValueType::String => "String".into(),
        ValueType::Bool => "Bool".into(),
        ValueType::Nil => "Nil".into(),
        ValueType::Tuple(types) => format!(
            "#({})",
            types
                .iter()
                .map(render_value_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValueType::List(element) => format!("List({})", render_value_type(element)),
        ValueType::Function(type_) => render_function_type(type_),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionError, Panic, PanicDetails, PanicKind, PanicMessage, render_value,
        render_value_type,
    };
    use crate::plan::{
        FunctionReturnFamily, FunctionType, FunctionValue, IntFunctionId, IntLocalId, ListValue,
        PanicSite, ParamLocal, RuntimeFunctionId, SourceContext, SourceSpan, Value, ValueType,
    };
    use miette::Diagnostic;

    #[test]
    fn panic_display_uses_kind_and_default_or_explicit_message() {
        for (error, expected) in [
            (
                ExecutionError::panic(PanicKind::Panic),
                "panic: `panic` expression evaluated.",
            ),
            (
                ExecutionError::source_panic(
                    None,
                    PanicKind::Panic,
                    Some("boom".into()),
                    PanicSite::unknown(),
                ),
                "panic: boom",
            ),
            (
                ExecutionError::panic(PanicKind::Todo),
                "todo: `todo` expression evaluated. This code has not yet been implemented.",
            ),
            (
                ExecutionError::panic(PanicKind::Assert),
                "assert: Assertion failed.",
            ),
            (
                ExecutionError::panic(PanicKind::LetAssert),
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                ExecutionError::panic(PanicKind::EmptyFunction),
                "empty_function: Function body is empty.",
            ),
            (
                ExecutionError::panic(PanicKind::EmptyBlock),
                "empty_block: Block is empty.",
            ),
            (
                ExecutionError::panic(PanicKind::IncompleteUse),
                "incomplete_use: Use callback is incomplete.",
            ),
        ] {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn panic_accessors_preserve_kind_message_site_and_details() {
        let site = PanicSite::new("main".into(), "main".into(), SourceSpan::new(12, 18));
        let details = PanicDetails::LetAssert {
            value: Value::List(crate::plan::ListValue::new(ValueType::Int, Vec::new())),
            pattern_span: SourceSpan::new(23, 32),
        };
        let panic = Panic::new(
            PanicKind::LetAssert,
            PanicMessage::Explicit("not empty".into()),
            site.clone(),
            None,
            Some(details.clone()),
        );

        assert_eq!(panic.kind(), PanicKind::LetAssert);
        assert_eq!(panic.message(), &PanicMessage::Explicit("not empty".into()),);
        assert_eq!(panic.site(), &site);
        assert_eq!(panic.details(), Some(&details));
    }

    #[test]
    fn panic_equality_includes_source_context() {
        let source = SourceContext::new("main.gleam", "pub fn main() { panic }");
        let same_source = SourceContext::new("main.gleam", "pub fn main() { panic }");
        let different_path = SourceContext::new("other.gleam", "pub fn main() { panic }");
        let different_source = SourceContext::new("main.gleam", "pub fn main() { todo }");
        let site = PanicSite::new("main".into(), "main".into(), SourceSpan::new(16, 21));

        assert_eq!(
            ExecutionError::source_panic(Some(&source), PanicKind::Panic, None, site.clone()),
            ExecutionError::source_panic(Some(&same_source), PanicKind::Panic, None, site.clone()),
        );
        assert_ne!(
            ExecutionError::source_panic(Some(&source), PanicKind::Panic, None, site.clone()),
            ExecutionError::source_panic(
                Some(&different_path),
                PanicKind::Panic,
                None,
                site.clone()
            ),
        );
        assert_ne!(
            ExecutionError::source_panic(Some(&source), PanicKind::Panic, None, site.clone()),
            ExecutionError::source_panic(Some(&different_source), PanicKind::Panic, None, site),
        );
        assert_ne!(
            ExecutionError::source_panic(
                Some(&source),
                PanicKind::Panic,
                None,
                PanicSite::unknown(),
            ),
            ExecutionError::panic(PanicKind::Panic),
        );
        assert_eq!(
            ExecutionError::panic(PanicKind::Todo),
            ExecutionError::panic(PanicKind::Todo),
        );
    }

    #[test]
    fn source_less_panic_diagnostic_has_no_source_labels_or_help() {
        let error = ExecutionError::panic(PanicKind::Panic);

        assert_eq!(
            error.code().map(|code| code.to_string()),
            Some("geam::panic".into()),
        );
        assert!(error.help().is_none());
        assert!(error.source_code().is_none());
        assert!(error.labels().is_none());
    }

    #[test]
    fn panic_diagnostic_has_source_labels_and_failed_value_help() {
        let source = SourceContext::new(
            "main.gleam",
            "pub fn main() {\n  let assert [x, ..] = []\n}",
        );
        let panic = Panic::new(
            PanicKind::LetAssert,
            PanicMessage::Default,
            PanicSite::new("main".into(), "main".into(), SourceSpan::new(18, 43)),
            Some(&source),
            Some(PanicDetails::LetAssert {
                value: Value::List(crate::plan::ListValue::new(ValueType::Int, Vec::new())),
                pattern_span: SourceSpan::new(29, 36),
            }),
        );
        let labels = panic
            .labels()
            .expect("source-backed panic should have labels")
            .collect::<Vec<_>>();

        assert_eq!(
            panic.code().map(|code| code.to_string()),
            Some("geam::let_assert".into())
        );
        assert!(panic.source_code().is_some());
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].label(), Some("let assert in main.main"));
        assert_eq!(labels[0].offset(), 18);
        assert_eq!(labels[0].len(), 25);
        assert_eq!(labels[1].label(), Some("pattern"));
        assert_eq!(labels[1].offset(), 29);
        assert_eq!(
            panic.help().map(|help| help.to_string()),
            Some("failed value: List(Int)([])".into()),
        );
    }

    #[test]
    fn source_backed_panic_without_details_has_one_primary_label() {
        let source = SourceContext::new("main.gleam", "pub fn main() {\n  assert False\n}");
        let panic = Panic::new(
            PanicKind::Assert,
            PanicMessage::Default,
            PanicSite::new("main".into(), "main".into(), SourceSpan::new(18, 30)),
            Some(&source),
            None,
        );
        let labels = panic
            .labels()
            .expect("source-backed panic should have labels")
            .collect::<Vec<_>>();

        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].label(), Some("assert in main.main"));
        assert_eq!(labels[0].offset(), 18);
        assert_eq!(labels[0].len(), 12);
        assert!(panic.help().is_none());
    }

    #[test]
    fn invariant_diagnostics_have_codes_without_source_labels_or_help() {
        for (error, expected_code) in [
            (
                ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Int,
                    FunctionReturnFamily::String,
                ),
                "geam::function_return_family_mismatch",
            ),
            (
                ExecutionError::tuple_index_family_mismatch(ValueType::Int, ValueType::String),
                "geam::tuple_index_mismatch",
            ),
        ] {
            assert_eq!(
                error.code().map(|code| code.to_string()),
                Some(expected_code.into()),
            );
            assert!(error.help().is_none());
            assert!(error.source_code().is_none());
            assert!(error.labels().is_none());
        }
    }

    #[test]
    fn render_value_preserves_every_runtime_value_family() {
        let function = Value::Function(FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(IntLocalId(0))],
        ));

        for (value, expected) in [
            (Value::Int(1.into()), "Int(1)"),
            (Value::Float(1.5), "Float(1.5)"),
            (Value::String("one".into()), "String(\"one\")"),
            (Value::Bool(true), "Bool(true)"),
            (Value::Nil, "Nil"),
            (
                Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())]),
                "Tuple([Int(1), String(\"one\")])",
            ),
            (
                Value::List(ListValue::new(
                    ValueType::Int,
                    vec![Value::Int(1.into()), Value::Int(2.into())],
                )),
                "List(Int)([Int(1), Int(2)])",
            ),
            (function, "Function(fn(Int) -> Int)"),
        ] {
            assert_eq!(render_value(&value), expected);
        }
    }

    #[test]
    fn render_value_type_preserves_compound_shapes() {
        assert_eq!(
            render_value_type(&ValueType::Tuple(vec![ValueType::Int, ValueType::String])),
            "#(Int, String)",
        );
        assert_eq!(
            render_value_type(&ValueType::List(Box::new(ValueType::Bool))),
            "List(Bool)",
        );
        assert_eq!(
            render_value_type(&ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Nil,
            )))),
            "fn(Float) -> Nil",
        );
    }

    #[test]
    fn function_return_family_mismatch_display() {
        let error = ExecutionError::function_return_family_mismatch(
            crate::plan::FunctionReturnFamily::Int,
            crate::plan::FunctionReturnFamily::String,
        );

        assert_eq!(
            error.to_string(),
            "function return family mismatch (expected Int, got String)",
        );
    }

    #[test]
    fn tuple_index_family_mismatch_display() {
        let error = ExecutionError::tuple_index_family_mismatch(
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::String,
        );

        assert_eq!(
            error.to_string(),
            "tuple index family mismatch (expected Tuple([Int]), got String)",
        );
    }
}
