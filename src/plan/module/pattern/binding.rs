use crate::plan::{AssertBinding, CustomConstructor, CustomValueShape, ListAssertTail, ValueType};

#[derive(Debug, Clone, PartialEq)]
enum CustomBindingProof {
    Exact(CustomValueShape),
    OnlyConstructor(CustomValueShape),
    ExhaustiveRemainder {
        source: CustomValueShape,
        excluded: Box<[usize]>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomBindingPattern {
    proof: CustomBindingProof,
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
    pub(crate) fn exact(
        source: CustomValueShape,
        constructor: CustomConstructor,
        fields: Vec<TotalBindingPattern>,
    ) -> Self {
        Self {
            proof: CustomBindingProof::Exact(source),
            constructor,
            fields,
        }
    }

    pub(crate) fn only_constructor(
        source: CustomValueShape,
        constructor: CustomConstructor,
        fields: Vec<TotalBindingPattern>,
    ) -> Self {
        Self {
            proof: CustomBindingProof::OnlyConstructor(source),
            constructor,
            fields,
        }
    }

    pub(crate) fn exhaustive_remainder(
        source: CustomValueShape,
        excluded: Vec<usize>,
        constructor: CustomConstructor,
        fields: Vec<TotalBindingPattern>,
    ) -> Self {
        Self {
            proof: CustomBindingProof::ExhaustiveRemainder {
                source,
                excluded: excluded.into_boxed_slice(),
            },
            constructor,
            fields,
        }
    }

    pub(crate) fn source_shape(&self) -> &CustomValueShape {
        self.proof.source_shape()
    }

    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    pub(crate) fn fields(&self) -> &[TotalBindingPattern] {
        &self.fields
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CustomValueShape,
        CustomConstructor,
        Vec<TotalBindingPattern>,
    ) {
        (
            self.proof.into_source_shape(),
            self.constructor,
            self.fields,
        )
    }
}

impl CustomBindingProof {
    pub(crate) fn source_shape(&self) -> &CustomValueShape {
        match self {
            Self::Exact(source) | Self::OnlyConstructor(source) => source,
            Self::ExhaustiveRemainder { source, .. } => source,
        }
    }

    pub(crate) fn into_source_shape(self) -> CustomValueShape {
        match self {
            Self::Exact(source) | Self::OnlyConstructor(source) => source,
            Self::ExhaustiveRemainder { source, .. } => source,
        }
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

#[cfg(test)]
mod tests {
    use super::{CustomBindingPattern, CustomBindingProof};
    use crate::plan::{
        CustomConstructor, CustomConstructorRefinement, CustomLocalId, CustomType, CustomTypeName,
        CustomValueShape, Step, StepKind,
    };

    #[test]
    fn custom_binding_proofs_preserve_exact_single_and_remainder_sources() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let exact_shape = CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(1),
        );
        let any_shape = CustomValueShape::any(type_.clone());
        let constructor = CustomConstructor::new(type_, "Second".into(), 1, Vec::new());

        let exact =
            CustomBindingPattern::exact(exact_shape.clone(), constructor.clone(), Vec::new());
        assert_eq!(exact.proof, CustomBindingProof::Exact(exact_shape.clone()));
        assert_eq!(
            exact.clone().into_parts(),
            (exact_shape.clone(), constructor.clone(), Vec::new()),
        );
        assert_eq!(
            Step::bind_custom_fields(CustomLocalId(3), exact.clone()).kind(),
            &StepKind::BindCustomFields {
                local: crate::plan::CustomLocal::from_shape(CustomLocalId(3), exact_shape.clone()),
                pattern: exact,
            },
        );

        let only = CustomBindingPattern::only_constructor(
            any_shape.clone(),
            constructor.clone(),
            Vec::new(),
        );
        assert_eq!(
            only.proof,
            CustomBindingProof::OnlyConstructor(any_shape.clone()),
        );
        assert_eq!(
            only.into_parts(),
            (any_shape.clone(), constructor.clone(), Vec::new()),
        );

        let remainder = CustomBindingPattern::exhaustive_remainder(
            any_shape.clone(),
            vec![0, 2],
            constructor.clone(),
            Vec::new(),
        );
        assert_eq!(
            remainder.proof,
            CustomBindingProof::ExhaustiveRemainder {
                source: any_shape.clone(),
                excluded: vec![0, 2].into_boxed_slice(),
            },
        );
        assert_eq!(remainder.into_parts(), (any_shape, constructor, Vec::new()),);
    }
}
