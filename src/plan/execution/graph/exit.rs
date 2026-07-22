#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GraphExitId(usize);

impl GraphExitId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
