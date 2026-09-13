use crate::plan::execution::prepared::rust::{Emit, Rust};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockGraphExitId(pub usize);

impl BlockGraphExitId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl Emit for BlockGraphExitId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BlockGraphExitId", &[field_0]);
    }
}
