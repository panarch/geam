use crate::plan::execution::{AssertPattern, CustomConstructorId};

pub(crate) struct CustomPattern {
    constructor: CustomConstructorId,
    fields: Vec<AssertPattern>,
}

impl CustomPattern {
    pub(in crate::plan::execution) fn new(
        constructor: CustomConstructorId,
        fields: Vec<AssertPattern>,
    ) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(crate) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(crate) fn fields(&self) -> &[AssertPattern] {
        &self.fields
    }
}
