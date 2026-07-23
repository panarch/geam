use super::super::Edge;
use crate::plan::execution::FloatLocalId;

pub(crate) struct FloatSwitch {
    subject: FloatLocalId,
    clauses: Box<[(f64, Edge)]>,
    fallback: Edge,
}

impl FloatSwitch {
    pub(in crate::plan::execution) fn new(
        subject: FloatLocalId,
        clauses: Box<[(f64, Edge)]>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> FloatLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(f64, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}
