use super::Edge;

pub(crate) struct Jump {
    edge: Edge,
}

impl Jump {
    pub(in crate::plan::execution) fn new(edge: Edge) -> Self {
        Self { edge }
    }

    pub(crate) fn edge(&self) -> &Edge {
        &self.edge
    }
}
