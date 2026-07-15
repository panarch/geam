use super::CustomExpr;

pub(crate) struct CustomFieldAccess {
    source: Box<CustomExpr>,
    index: usize,
}

impl CustomFieldAccess {
    pub(in crate::plan::execution) fn from_parts(source: CustomExpr, index: usize) -> Self {
        Self {
            source: Box::new(source),
            index,
        }
    }

    pub(crate) fn source(&self) -> &CustomExpr {
        &self.source
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
