use crate::plan::{AssertPattern, CustomConstructor, TotalBindingPattern};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomPattern {
    constructor: CustomConstructor,
    fields: Vec<AssertPattern>,
    total_fields: Option<Vec<TotalBindingPattern>>,
}

impl CustomPattern {
    pub(crate) fn new(
        constructor: CustomConstructor,
        fields: Vec<AssertPattern>,
        total_fields: Option<Vec<TotalBindingPattern>>,
    ) -> Self {
        Self {
            constructor,
            fields,
            total_fields,
        }
    }

    pub(crate) fn fields(&self) -> &[AssertPattern] {
        &self.fields
    }

    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    pub(crate) fn total_fields(&self) -> Option<&[TotalBindingPattern]> {
        self.total_fields.as_deref()
    }
}
