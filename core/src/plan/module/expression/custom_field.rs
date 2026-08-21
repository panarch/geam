use super::CustomExpr;
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFieldAccess {
    source: Box<CustomExpr>,
    index: usize,
    label: Option<EcoString>,
}

impl CustomFieldAccess {
    pub(crate) fn new(source: CustomExpr, index: usize, label: Option<EcoString>) -> Self {
        Self {
            source: Box::new(source),
            index,
            label,
        }
    }

    pub(crate) fn source(&self) -> &CustomExpr {
        &self.source
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
