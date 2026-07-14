use crate::plan::execution::{AssertBinding, CustomConstructorId, ListAssertTail, ValueType};

pub(crate) struct CustomBindingPattern {
    constructor: CustomConstructorId,
    fields: Vec<TotalBindingPattern>,
}

pub(crate) struct TotalBindingPattern {
    type_: ValueType,
    kind: TotalBindingPatternKind,
}

pub(crate) enum TotalBindingPatternKind {
    Bind(AssertBinding),
    Discard,
    Tuple(Vec<TotalBindingPattern>),
    List(ListAssertTail),
    Custom(CustomBindingPattern),
    Alias {
        pattern: Box<TotalBindingPattern>,
        binding: AssertBinding,
    },
}

impl CustomBindingPattern {
    pub(in crate::plan::execution) fn new(
        constructor: CustomConstructorId,
        fields: Vec<TotalBindingPattern>,
    ) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(crate) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(crate) fn fields(&self) -> &[TotalBindingPattern] {
        &self.fields
    }
}

impl TotalBindingPattern {
    pub(in crate::plan::execution) fn new(type_: ValueType, kind: TotalBindingPatternKind) -> Self {
        Self { type_, kind }
    }

    pub(crate) fn type_(&self) -> &ValueType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &TotalBindingPatternKind {
        &self.kind
    }
}
