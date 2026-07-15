use super::CustomExpr;
use crate::plan::CustomConstructor;
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFieldAccess {
    source: Box<CustomExpr>,
    index: usize,
    label: Option<EcoString>,
    constructors: Vec<CustomConstructor>,
}

impl CustomFieldAccess {
    pub(crate) fn new(
        source: CustomExpr,
        index: usize,
        label: Option<EcoString>,
        constructors: Vec<CustomConstructor>,
    ) -> Self {
        Self {
            source: Box::new(source),
            index,
            label,
            constructors,
        }
    }

    pub(crate) fn source(&self) -> &CustomExpr {
        &self.source
    }

    pub(crate) fn into_parts(
        self,
    ) -> (CustomExpr, usize, Option<EcoString>, Vec<CustomConstructor>) {
        (*self.source, self.index, self.label, self.constructors)
    }
}
