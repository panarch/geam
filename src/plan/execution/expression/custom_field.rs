use super::CustomExpr;
use crate::plan::execution::CustomConstructorId;

pub(crate) struct CustomFieldAccess {
    source: Box<CustomExpr>,
    index: usize,
    constructors: Vec<CustomConstructorId>,
}

impl CustomFieldAccess {
    pub(in crate::plan::execution) fn from_parts(
        source: CustomExpr,
        index: usize,
        constructors: Vec<CustomConstructorId>,
    ) -> Self {
        Self {
            source: Box::new(source),
            index,
            constructors,
        }
    }

    pub(crate) fn source(&self) -> &CustomExpr {
        &self.source
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn constructors(&self) -> &[CustomConstructorId] {
        &self.constructors
    }
}
