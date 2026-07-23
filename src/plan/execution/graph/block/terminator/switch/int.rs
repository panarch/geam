use super::super::Edge;
use crate::plan::execution::IntLocalId;
use num_bigint::BigInt;

pub(crate) struct IntSwitch {
    subject: IntLocalId,
    clauses: Box<[(BigInt, Edge)]>,
    fallback: Edge,
}

impl IntSwitch {
    pub(in crate::plan::execution) fn new(
        subject: IntLocalId,
        clauses: Box<[(BigInt, Edge)]>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> IntLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(BigInt, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}
