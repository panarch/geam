use super::Edge;
use crate::plan::execution::BoolLocalId;

pub(crate) struct BoolBranch {
    subject: BoolLocalId,
    true_: Edge,
    false_: Edge,
}

impl BoolBranch {
    pub(in crate::plan::execution) fn new(subject: BoolLocalId, true_: Edge, false_: Edge) -> Self {
        Self {
            subject,
            true_,
            false_,
        }
    }

    pub(crate) fn subject(&self) -> BoolLocalId {
        self.subject
    }

    pub(crate) fn true_(&self) -> &Edge {
        &self.true_
    }

    pub(crate) fn false_(&self) -> &Edge {
        &self.false_
    }
}
