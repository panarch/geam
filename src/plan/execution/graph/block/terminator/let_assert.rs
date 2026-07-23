use crate::plan::execution::{ParamLocal, StringLocalId};
use crate::plan::{PanicSite, SourceSpan};

pub(crate) struct LetAssertPanic {
    subject: ParamLocal,
    message: Option<StringLocalId>,
    site: PanicSite,
    pattern_span: SourceSpan,
}

impl LetAssertPanic {
    pub(in crate::plan::execution) fn new(
        subject: ParamLocal,
        message: Option<StringLocalId>,
        site: PanicSite,
        pattern_span: SourceSpan,
    ) -> Self {
        Self {
            subject,
            message,
            site,
            pattern_span,
        }
    }

    pub(crate) fn subject(&self) -> &ParamLocal {
        &self.subject
    }

    pub(crate) fn message(&self) -> Option<StringLocalId> {
        self.message
    }

    pub(crate) fn site(&self) -> &PanicSite {
        &self.site
    }

    pub(crate) fn pattern_span(&self) -> &SourceSpan {
        &self.pattern_span
    }
}
