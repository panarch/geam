use crate::plan::{AssertBinding, CustomConstructor, ListAssertTail, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomBindingPattern {
    constructor: CustomConstructor,
    fields: Vec<TotalBindingPattern>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TotalBindingPattern {
    type_: ValueType,
    kind: TotalBindingPatternKind,
}

#[derive(Debug, Clone, PartialEq)]
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
    pub(crate) fn new(constructor: CustomConstructor, fields: Vec<TotalBindingPattern>) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    pub(crate) fn fields(&self) -> &[TotalBindingPattern] {
        &self.fields
    }

    pub(crate) fn into_parts(self) -> (CustomConstructor, Vec<TotalBindingPattern>) {
        (self.constructor, self.fields)
    }
}

impl TotalBindingPattern {
    pub(crate) fn bind(binding: AssertBinding) -> Self {
        Self::new(
            binding.local().value_type(),
            TotalBindingPatternKind::Bind(binding),
        )
    }

    pub(crate) fn discard(type_: ValueType) -> Self {
        Self::new(type_, TotalBindingPatternKind::Discard)
    }

    pub(crate) fn tuple(elements: Vec<Self>) -> Self {
        let type_ = ValueType::Tuple(
            elements
                .iter()
                .map(|element| element.type_.clone())
                .collect(),
        );
        Self::new(type_, TotalBindingPatternKind::Tuple(elements))
    }

    pub(crate) fn list(element_type: ValueType, tail: ListAssertTail) -> Self {
        Self::new(
            ValueType::List(Box::new(element_type)),
            TotalBindingPatternKind::List(tail),
        )
    }

    pub(crate) fn custom(pattern: CustomBindingPattern) -> Self {
        let type_ = ValueType::Custom(pattern.constructor().type_().clone());
        Self::new(type_, TotalBindingPatternKind::Custom(pattern))
    }

    pub(crate) fn alias(pattern: Self, binding: AssertBinding) -> Self {
        let type_ = binding.local().value_type();
        Self::new(
            type_,
            TotalBindingPatternKind::Alias {
                pattern: Box::new(pattern),
                binding,
            },
        )
    }

    pub(crate) fn kind(&self) -> &TotalBindingPatternKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (ValueType, TotalBindingPatternKind) {
        (self.type_, self.kind)
    }

    fn new(type_: ValueType, kind: TotalBindingPatternKind) -> Self {
        Self { type_, kind }
    }
}
