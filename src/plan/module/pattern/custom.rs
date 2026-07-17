use crate::plan::{AssertPattern, CustomConstructor};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomPattern {
    constructor: CustomConstructor,
    fields: Vec<AssertPattern>,
}

impl CustomPattern {
    pub(crate) fn new(constructor: CustomConstructor, fields: Vec<AssertPattern>) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(crate) fn fields(&self) -> &[AssertPattern] {
        &self.fields
    }

    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }
}
