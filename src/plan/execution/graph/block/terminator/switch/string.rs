use super::super::Edge;
use crate::plan::execution::StringLocalId;
use ecow::EcoString;

pub(crate) struct StringSwitch {
    subject: StringLocalId,
    clauses: Box<[(EcoString, Edge)]>,
    fallback: Edge,
}

impl StringSwitch {
    pub(in crate::plan::execution) fn new(
        subject: StringLocalId,
        clauses: Box<[(EcoString, Edge)]>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> StringLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(EcoString, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}
