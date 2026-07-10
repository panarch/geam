use super::StringExpr;
use crate::plan::PanicSite;

pub(crate) struct PanicExpr {
    kind: PanicExprKind,
    site: PanicSite,
}

pub(crate) enum PanicExprKind {
    Panic { message: Option<Box<StringExpr>> },
    Todo { message: Option<Box<StringExpr>> },
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

impl PanicExpr {
    pub(in crate::plan::execution) fn from_parts(site: PanicSite, kind: PanicExprKind) -> Self {
        Self { site, kind }
    }

    pub(crate) fn kind(&self) -> &PanicExprKind {
        &self.kind
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }
}
