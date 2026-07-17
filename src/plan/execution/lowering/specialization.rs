use crate::plan::module::{FunctionInstantiation, FunctionTemplateId, TypeSubstitution};
use crate::plan::{CustomConstructorRefinement, CustomTypeName, FunctionShape, ValueShape};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SpecializationKey {
    template: FunctionTemplateId,
    substitution: ConcreteTypeSubstitution,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct ConcreteTypeSubstitution {
    arguments: Box<[ConcreteValueShape]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct ConcreteFunctionShape {
    arguments: Box<[ConcreteValueShape]>,
    return_: Box<ConcreteValueShape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct ConcreteCustomValueShape {
    name: CustomTypeName,
    arguments: Box<[ConcreteValueShape]>,
    constructor: CustomConstructorRefinement,
}

pub(super) struct ConcreteCustomConstructor {
    type_: ConcreteCustomValueShape,
    name: EcoString,
    index: usize,
    fields: Box<[ConcreteCustomConstructorField]>,
}

pub(super) struct ConcreteCustomConstructorField {
    label: Option<EcoString>,
    shape: ConcreteValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum ConcreteValueShape {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[ConcreteValueShape]>),
    List(Box<ConcreteValueShape>),
    Function(Box<ConcreteFunctionShape>),
    Custom(ConcreteCustomValueShape),
}

impl SpecializationKey {
    pub(super) fn monomorphic(template: FunctionTemplateId) -> Self {
        Self {
            template,
            substitution: ConcreteTypeSubstitution::empty(),
        }
    }

    pub(super) fn from_instantiation(
        instantiation: &FunctionInstantiation,
        outer: &ConcreteTypeSubstitution,
    ) -> (Self, ConcreteFunctionShape) {
        let substitution =
            ConcreteTypeSubstitution::instantiate(instantiation.substitution(), outer);
        let shape = ConcreteFunctionShape::instantiate(instantiation.shape(), outer);
        (
            Self {
                template: instantiation.template(),
                substitution,
            },
            shape,
        )
    }

    pub(super) fn template(&self) -> FunctionTemplateId {
        self.template
    }

    pub(super) fn substitution(&self) -> &ConcreteTypeSubstitution {
        &self.substitution
    }
}

impl ConcreteTypeSubstitution {
    pub(super) fn empty() -> Self {
        Self {
            arguments: Box::new([]),
        }
    }

    fn instantiate(substitution: &TypeSubstitution, outer: &ConcreteTypeSubstitution) -> Self {
        Self {
            arguments: substitution
                .arguments()
                .iter()
                .map(|shape| ConcreteValueShape::instantiate(shape, outer))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn get(&self, parameter: crate::plan::TypeParameterId) -> &ConcreteValueShape {
        &self.arguments[parameter.index()]
    }
}

impl ConcreteFunctionShape {
    pub(super) fn new(arguments: Vec<ConcreteValueShape>, return_: ConcreteValueShape) -> Self {
        Self {
            arguments: arguments.into_boxed_slice(),
            return_: Box::new(return_),
        }
    }

    pub(super) fn instantiate(
        shape: &FunctionShape,
        substitution: &ConcreteTypeSubstitution,
    ) -> Self {
        Self {
            arguments: shape
                .argument_shapes()
                .iter()
                .map(|shape| ConcreteValueShape::instantiate(shape, substitution))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            return_: Box::new(ConcreteValueShape::instantiate(
                shape.return_shape(),
                substitution,
            )),
        }
    }

    pub(super) fn arguments(&self) -> &[ConcreteValueShape] {
        &self.arguments
    }

    pub(super) fn return_(&self) -> &ConcreteValueShape {
        &self.return_
    }

    pub(super) fn to_module_shape(&self) -> FunctionShape {
        FunctionShape::new(
            self.arguments
                .iter()
                .map(ConcreteValueShape::to_module_shape)
                .collect(),
            self.return_.to_module_shape(),
        )
    }
}

impl ConcreteCustomValueShape {
    pub(super) fn instantiate(
        shape: &crate::plan::CustomValueShape,
        substitution: &ConcreteTypeSubstitution,
    ) -> Self {
        Self {
            name: shape.type_name().clone(),
            arguments: shape
                .arguments()
                .iter()
                .map(|shape| ConcreteValueShape::instantiate(shape, substitution))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            constructor: shape.constructor(),
        }
    }

    pub(super) fn arguments(&self) -> &[ConcreteValueShape] {
        &self.arguments
    }

    pub(super) fn constructor(&self) -> CustomConstructorRefinement {
        self.constructor
    }

    pub(super) fn to_module_shape(&self) -> crate::plan::CustomValueShape {
        crate::plan::CustomValueShape::new(
            self.name.clone(),
            self.arguments
                .iter()
                .map(ConcreteValueShape::to_module_shape)
                .collect(),
            self.constructor,
        )
    }
}

impl ConcreteCustomConstructor {
    pub(super) fn instantiate(
        constructor: crate::plan::CustomConstructor,
        substitution: &ConcreteTypeSubstitution,
    ) -> Self {
        let (type_, name, index, fields) = constructor.into_parts();
        Self {
            type_: ConcreteCustomValueShape::instantiate(
                &crate::plan::CustomValueShape::any(type_),
                substitution,
            ),
            name,
            index,
            fields: fields
                .into_iter()
                .map(|field| {
                    let (label, type_) = field.into_parts();
                    ConcreteCustomConstructorField {
                        label,
                        shape: ConcreteValueShape::instantiate(
                            &ValueShape::from_value_type(type_),
                            substitution,
                        ),
                    }
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ConcreteCustomValueShape,
        EcoString,
        usize,
        Box<[ConcreteCustomConstructorField]>,
    ) {
        (self.type_, self.name, self.index, self.fields)
    }
}

impl ConcreteCustomConstructorField {
    pub(super) fn into_parts(self) -> (Option<EcoString>, ConcreteValueShape) {
        (self.label, self.shape)
    }
}

impl ConcreteValueShape {
    pub(super) fn instantiate(shape: &ValueShape, substitution: &ConcreteTypeSubstitution) -> Self {
        match shape {
            ValueShape::Parameter(parameter) => substitution.get(*parameter).clone(),
            ValueShape::Int => Self::Int,
            ValueShape::Float => Self::Float,
            ValueShape::String => Self::String,
            ValueShape::BitArray => Self::BitArray,
            ValueShape::UtfCodepoint => Self::UtfCodepoint,
            ValueShape::Bool => Self::Bool,
            ValueShape::Nil => Self::Nil,
            ValueShape::Tuple(elements) => Self::Tuple(
                elements
                    .iter()
                    .map(|shape| Self::instantiate(shape, substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            ValueShape::List(item) => Self::List(Box::new(Self::instantiate(item, substitution))),
            ValueShape::Function(function) => Self::Function(Box::new(
                ConcreteFunctionShape::instantiate(function, substitution),
            )),
            ValueShape::Custom(custom) => {
                Self::Custom(ConcreteCustomValueShape::instantiate(custom, substitution))
            }
        }
    }

    pub(super) fn to_module_shape(&self) -> ValueShape {
        match self {
            Self::Int => ValueShape::Int,
            Self::Float => ValueShape::Float,
            Self::String => ValueShape::String,
            Self::BitArray => ValueShape::BitArray,
            Self::UtfCodepoint => ValueShape::UtfCodepoint,
            Self::Bool => ValueShape::Bool,
            Self::Nil => ValueShape::Nil,
            Self::Tuple(elements) => ValueShape::Tuple(
                elements
                    .iter()
                    .map(Self::to_module_shape)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::List(item) => ValueShape::List(Box::new(item.to_module_shape())),
            Self::Function(function) => ValueShape::Function(Box::new(function.to_module_shape())),
            Self::Custom(custom) => ValueShape::Custom(custom.to_module_shape()),
        }
    }

    pub(super) fn value_type(&self) -> crate::plan::ValueType {
        self.to_module_shape().value_type()
    }
}

#[cfg(test)]
mod tests {
    use super::{ConcreteTypeSubstitution, ConcreteValueShape, SpecializationKey};
    use crate::plan::{
        CustomConstructorRefinement, CustomTypeName, CustomValueShape, FunctionShape,
        FunctionTemplateId, TypeParameterId, TypeScheme, ValueShape,
    };

    #[test]
    fn concrete_specialization_preserves_recursive_shape_metadata() {
        let substitution = ConcreteTypeSubstitution::instantiate(
            &TypeScheme::new(2)
                .try_substitution(vec![ValueShape::Int, ValueShape::String])
                .expect("two arguments should match the scheme"),
            &ConcreteTypeSubstitution::empty(),
        );
        let shape = ValueShape::Tuple(
            vec![
                ValueShape::Parameter(TypeParameterId(0)),
                ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(1)))),
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::Parameter(TypeParameterId(1))],
                    ValueShape::Parameter(TypeParameterId(0)),
                ))),
                ValueShape::Custom(CustomValueShape::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    vec![ValueShape::Parameter(TypeParameterId(0))],
                    CustomConstructorRefinement::Exact(1),
                )),
            ]
            .into_boxed_slice(),
        );
        let concrete = ConcreteValueShape::instantiate(&shape, &substitution);

        assert_eq!(
            concrete.to_module_shape(),
            shape.substitute(
                &TypeScheme::new(2)
                    .try_substitution(vec![ValueShape::Int, ValueShape::String])
                    .expect("two arguments should match the scheme")
            )
        );
        assert_eq!(
            concrete.value_type(),
            concrete.to_module_shape().value_type()
        );
    }

    #[test]
    fn monomorphic_specialization_key_has_empty_substitution() {
        let key = SpecializationKey::monomorphic(FunctionTemplateId::new(7));

        assert_eq!(key.template(), FunctionTemplateId::new(7));
        assert_eq!(key.substitution().arguments.as_ref(), &[]);
    }
}
