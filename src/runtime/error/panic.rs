use crate::plan::{PanicSite, SourceContext, SourceSpan};
use crate::runtime::Value;
use ecow::EcoString;
use miette::NamedSource;
use num_bigint::BigInt;
use std::fmt;

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
    BitArraySegment,
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
    BitArraySegment {
        reason: BitArraySegmentPanicReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitArraySegmentPanicReason {
    InvalidFloatSize { bit_size: BigInt },
    InsufficientBits { requested: usize, available: usize },
    SizeOutOfRange { bit_size: BigInt },
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

    pub(in crate::runtime::error) fn source(&self) -> Option<&NamedSource<String>> {
        self.source.as_deref()
    }

    pub(in crate::runtime::error) fn message_text(&self) -> std::borrow::Cow<'_, str> {
        self.message.text(self.kind)
    }

    pub(in crate::runtime::error) fn primary_label(&self) -> String {
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

impl PanicKind {
    pub(in crate::runtime::error) fn code(&self) -> &'static str {
        match self {
            Self::Panic => "panic",
            Self::Todo => "todo",
            Self::Assert => "assert",
            Self::LetAssert => "let_assert",
            Self::BitArraySegment => "bit_array_segment",
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
            Self::BitArraySegment => "bit array segment",
            Self::EmptyFunction => "empty function",
            Self::EmptyBlock => "empty block",
            Self::IncompleteUse => "incomplete use",
        }
    }

    fn default_message(&self) -> &'static str {
        match self {
            Self::Panic => "`panic` expression evaluated.",
            Self::Todo => "`todo` expression evaluated. This code has not yet been implemented.",
            Self::Assert => "Assertion failed.",
            Self::LetAssert => "Pattern match failed, no pattern matched the value.",
            Self::BitArraySegment => "BitArray segment construction failed.",
            Self::EmptyFunction => "Function body is empty.",
            Self::EmptyBlock => "Block is empty.",
            Self::IncompleteUse => "Use callback is incomplete.",
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

#[cfg(test)]
mod tests {
    use super::{BitArraySegmentPanicReason, Panic, PanicDetails, PanicKind, PanicMessage};
    use crate::plan::{PanicSite, SourceContext, SourceSpan, ValueType};
    use crate::runtime::ExecutionError;
    use crate::runtime::Value;

    #[test]
    fn panic_display_uses_kind_and_default_or_explicit_message() {
        for (error, expected) in [
            (
                ExecutionError::source_panic(None, PanicKind::Panic, None, PanicSite::unknown()),
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
                ExecutionError::source_panic(None, PanicKind::Todo, None, PanicSite::unknown()),
                "todo: `todo` expression evaluated. This code has not yet been implemented.",
            ),
            (
                ExecutionError::source_panic(None, PanicKind::Assert, None, PanicSite::unknown()),
                "assert: Assertion failed.",
            ),
            (
                ExecutionError::source_panic(
                    None,
                    PanicKind::LetAssert,
                    None,
                    PanicSite::unknown(),
                ),
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                ExecutionError::bit_array_segment_panic(
                    None,
                    BitArraySegmentPanicReason::InvalidFloatSize {
                        bit_size: 24.into(),
                    },
                    PanicSite::unknown(),
                ),
                "bit_array_segment: BitArray segment construction failed.",
            ),
            (
                ExecutionError::source_panic(
                    None,
                    PanicKind::EmptyFunction,
                    None,
                    PanicSite::unknown(),
                ),
                "empty_function: Function body is empty.",
            ),
            (
                ExecutionError::source_panic(
                    None,
                    PanicKind::EmptyBlock,
                    None,
                    PanicSite::unknown(),
                ),
                "empty_block: Block is empty.",
            ),
            (
                ExecutionError::source_panic(
                    None,
                    PanicKind::IncompleteUse,
                    None,
                    PanicSite::unknown(),
                ),
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
            value: Value::List(crate::runtime::ListValue::empty(ValueType::Int)),
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
            ExecutionError::source_panic(None, PanicKind::Panic, None, PanicSite::unknown()),
        );
        assert_eq!(
            ExecutionError::source_panic(None, PanicKind::Todo, None, PanicSite::unknown()),
            ExecutionError::source_panic(None, PanicKind::Todo, None, PanicSite::unknown()),
        );
    }
}
