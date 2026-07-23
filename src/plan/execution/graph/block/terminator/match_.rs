use super::{Edge, MatchEdge, MatchPattern};
use crate::plan::execution::ParamLocal;

pub(crate) struct Match {
    subject: ParamLocal,
    pattern: MatchPattern,
    success: MatchEdge,
    failure: Edge,
}

impl Match {
    pub(in crate::plan::execution) fn new(
        subject: ParamLocal,
        pattern: MatchPattern,
        success: MatchEdge,
        failure: Edge,
    ) -> Self {
        Self {
            subject,
            pattern,
            success,
            failure,
        }
    }

    pub(crate) fn subject(&self) -> &ParamLocal {
        &self.subject
    }

    pub(crate) fn pattern(&self) -> &MatchPattern {
        &self.pattern
    }

    pub(crate) fn success(&self) -> &MatchEdge {
        &self.success
    }

    pub(crate) fn failure(&self) -> &Edge {
        &self.failure
    }
}
