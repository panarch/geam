use crate::plan::PanicSite;
use crate::plan::execution::StringLocalId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceStopKind {
    Panic,
    Todo,
    Assert,
    EmptyFunction,
    EmptyBlock,
    IncompleteUse,
}

pub(crate) struct SourceStop {
    kind: SourceStopKind,
    message: Option<StringLocalId>,
    site: PanicSite,
}

impl SourceStop {
    pub(in crate::plan::execution) fn new(
        kind: SourceStopKind,
        message: Option<StringLocalId>,
        site: PanicSite,
    ) -> Self {
        Self {
            kind,
            message,
            site,
        }
    }

    pub(crate) fn kind(&self) -> SourceStopKind {
        self.kind
    }

    pub(crate) fn message(&self) -> Option<StringLocalId> {
        self.message
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }
}
