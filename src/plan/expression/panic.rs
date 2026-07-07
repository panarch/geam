use super::StringExpr;
use crate::plan::PanicSite;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PanicExpr {
    kind: PanicExprKind,
    site: PanicSite,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PanicExprKind {
    Panic { message: Option<Box<StringExpr>> },
    Todo { message: Option<Box<StringExpr>> },
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

impl PanicExpr {
    #[cfg(test)]
    pub(crate) fn panic(message: Option<StringExpr>) -> Self {
        Self::panic_at(message, PanicSite::unknown())
    }

    pub(crate) fn panic_at(message: Option<StringExpr>, site: PanicSite) -> Self {
        Self {
            kind: PanicExprKind::Panic {
                message: message.map(Box::new),
            },
            site,
        }
    }

    #[cfg(test)]
    pub(crate) fn todo(message: Option<StringExpr>) -> Self {
        Self::todo_at(message, PanicSite::unknown())
    }

    pub(crate) fn todo_at(message: Option<StringExpr>, site: PanicSite) -> Self {
        Self {
            kind: PanicExprKind::Todo {
                message: message.map(Box::new),
            },
            site,
        }
    }

    #[cfg(test)]
    pub(crate) fn empty_function() -> Self {
        Self::empty_function_at(PanicSite::unknown())
    }

    pub(crate) fn empty_function_at(site: PanicSite) -> Self {
        Self {
            kind: PanicExprKind::EmptyFunction,
            site,
        }
    }

    #[cfg(test)]
    pub(crate) fn empty_block() -> Self {
        Self::empty_block_at(PanicSite::unknown())
    }

    pub(crate) fn empty_block_at(site: PanicSite) -> Self {
        Self {
            kind: PanicExprKind::EmptyBlock,
            site,
        }
    }

    #[cfg(test)]
    pub(crate) fn incomplete_use() -> Self {
        Self::incomplete_use_at(PanicSite::unknown())
    }

    pub(crate) fn incomplete_use_at(site: PanicSite) -> Self {
        Self {
            kind: PanicExprKind::IncompleteUse,
            site,
        }
    }

    pub(crate) fn kind(&self) -> &PanicExprKind {
        &self.kind
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }

    pub(crate) fn message(&self) -> Option<&StringExpr> {
        match &self.kind {
            PanicExprKind::Panic { message } | PanicExprKind::Todo { message } => {
                message.as_deref()
            }
            PanicExprKind::EmptyFunction
            | PanicExprKind::EmptyBlock
            | PanicExprKind::IncompleteUse => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PanicExpr, PanicExprKind};
    use crate::plan::StringExpr;

    #[test]
    fn panic_expr_preserves_kind_and_message() {
        let expression = PanicExpr::panic(Some(StringExpr::value("message".into())));

        assert_eq!(
            expression.kind(),
            &PanicExprKind::Panic {
                message: Some(Box::new(StringExpr::value("message".into()))),
            },
        );
        assert_eq!(
            expression.message(),
            Some(&StringExpr::value("message".into())),
        );
    }

    #[test]
    fn generated_todo_kinds_have_no_message() {
        for expression in [
            PanicExpr::empty_function(),
            PanicExpr::empty_block(),
            PanicExpr::incomplete_use(),
        ] {
            assert_eq!(expression.message(), None);
        }
    }
}
