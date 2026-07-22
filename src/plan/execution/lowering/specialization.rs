use crate::plan::module::{FunctionInstantiation, FunctionTemplateId, TypeSubstitution};
use crate::plan::{CustomConstructorRefinement, CustomTypeName, FunctionShape, ValueShape};
use ecow::EcoString;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SpecializationKey {
    template: FunctionTemplateId,
    substitution: SpecializedTypeSubstitution,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SpecializedTypeSubstitution {
    arguments: Box<[SpecializedValueShape]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SpecializedFunctionShape {
    arguments: Box<[SpecializedValueShape]>,
    return_: Box<SpecializedValueShape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SpecializedCustomValueShape {
    name: CustomTypeName,
    arguments: Box<[SpecializedValueShape]>,
    constructor: CustomConstructorRefinement,
}

pub(super) struct SpecializedCustomConstructor {
    type_: SpecializedCustomValueShape,
    name: EcoString,
    index: usize,
    fields: Box<[SpecializedCustomConstructorField]>,
}

pub(super) struct SpecializedCustomConstructorField {
    label: Option<EcoString>,
    shape: SpecializedValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum SpecializedValueShape {
    Parameter(crate::plan::TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[SpecializedValueShape]>),
    List(Box<SpecializedValueShape>),
    Function(Box<SpecializedFunctionShape>),
    Custom(SpecializedCustomValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum StoredValueShape {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[SpecializedValueShape]>),
    List(Box<SpecializedValueShape>),
    Function(Box<SpecializedFunctionShape>),
    Custom(SpecializedCustomValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ValueRepresentation {
    Uninhabited(crate::plan::TypeParameterId),
    Stored(StoredValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum UninhabitedValueShape {
    Parameter(crate::plan::TypeParameterId),
    Tuple(UninhabitedTupleValueShape),
    Custom(UninhabitedCustomValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UninhabitedTupleValueShape {
    elements: Box<[SpecializedValueShape]>,
    diverging: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UninhabitedCustomValueShape {
    shape: SpecializedCustomValueShape,
    divergence: CustomConstructorDivergence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CustomConstructorDivergence {
    Exact { field: usize },
    Every { fields: Box<[usize]> },
}

enum ConstructorInhabitation {
    Inhabited,
    Uninhabited { field: usize },
}

enum CustomInhabitation {
    Inhabited,
    Uninhabited(CustomConstructorDivergence),
}

pub(super) enum CompoundInhabitation<T> {
    Inhabited,
    Uninhabited(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CustomConstructorMatch {
    Impossible,
    Certain,
    Dynamic,
}

pub(super) enum ValueInhabitation {
    Inhabited(StoredValueShape),
    Uninhabited(UninhabitedValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StorageRepresentation {
    Parameter(crate::plan::TypeParameterId),
    Stored(StoredValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionRepresentation {
    Symbolic,
    Never(UninhabitedValueShape),
    Executable(StoredValueShape),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionArgumentsRepresentation {
    Symbolic,
    Inhabited,
}

pub(super) struct RepresentationContext {
    custom_types: HashMap<CustomTypeName, crate::plan::CustomTypeDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CustomRepresentationKey {
    name: CustomTypeName,
    arguments: Box<[bool]>,
    constructor: CustomConstructorRefinement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Representability<T> {
    Inhabited(T),
    Uninhabited,
}

impl ValueInhabitation {
    pub(super) fn into_representability(self) -> Representability<StoredValueShape> {
        match self {
            Self::Inhabited(shape) => Representability::Inhabited(shape),
            Self::Uninhabited(_) => Representability::Uninhabited,
        }
    }
}

impl ValueRepresentation {
    pub(super) fn into_representability(self) -> Representability<StoredValueShape> {
        match self {
            Self::Stored(shape) => Representability::Inhabited(shape),
            Self::Uninhabited(_) => Representability::Uninhabited,
        }
    }
}

impl<T> Representability<T> {
    pub(super) fn map<U>(self, map: impl FnOnce(T) -> U) -> Representability<U> {
        match self {
            Self::Inhabited(value) => Representability::Inhabited(map(value)),
            Self::Uninhabited => Representability::Uninhabited,
        }
    }

    pub(super) fn and_then<U>(
        self,
        next: impl FnOnce(T) -> Representability<U>,
    ) -> Representability<U> {
        match self {
            Self::Inhabited(value) => next(value),
            Self::Uninhabited => Representability::Uninhabited,
        }
    }

    pub(super) fn zip_with<U, V>(
        self,
        other: Representability<U>,
        map: impl FnOnce(T, U) -> V,
    ) -> Representability<V> {
        self.and_then(|left| other.map(|right| map(left, right)))
    }

    pub(super) fn collect(
        values: impl IntoIterator<Item = Representability<T>>,
    ) -> Representability<Vec<T>> {
        let mut inhabited = Vec::new();
        for value in values {
            match value {
                Self::Inhabited(value) => inhabited.push(value),
                Self::Uninhabited => return Representability::Uninhabited,
            }
        }
        Representability::Inhabited(inhabited)
    }
}

impl SpecializationKey {
    pub(super) fn monomorphic(template: FunctionTemplateId) -> Self {
        Self {
            template,
            substitution: SpecializedTypeSubstitution::empty(),
        }
    }

    pub(super) fn from_instantiation(
        instantiation: &FunctionInstantiation,
        outer: &SpecializedTypeSubstitution,
    ) -> (Self, SpecializedFunctionShape) {
        let substitution =
            SpecializedTypeSubstitution::instantiate(instantiation.substitution(), outer);
        let shape = SpecializedFunctionShape::instantiate(instantiation.shape(), outer);
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

    pub(super) fn substitution(&self) -> &SpecializedTypeSubstitution {
        &self.substitution
    }
}

impl SpecializedTypeSubstitution {
    pub(super) fn empty() -> Self {
        Self {
            arguments: Box::new([]),
        }
    }

    pub(super) fn instantiate(
        substitution: &TypeSubstitution,
        outer: &SpecializedTypeSubstitution,
    ) -> Self {
        Self {
            arguments: substitution
                .arguments()
                .iter()
                .map(|shape| SpecializedValueShape::instantiate(shape, outer))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn resolve(&self, parameter: crate::plan::TypeParameterId) -> SpecializedValueShape {
        match self.arguments.get(parameter.index()) {
            Some(shape) => shape.clone(),
            None => SpecializedValueShape::Parameter(parameter),
        }
    }

    pub(super) fn arguments(&self) -> &[SpecializedValueShape] {
        &self.arguments
    }

    pub(super) fn to_module_substitution(&self) -> TypeSubstitution {
        TypeSubstitution::from_arguments(
            self.arguments
                .iter()
                .map(SpecializedValueShape::to_module_shape)
                .collect(),
        )
    }
}

impl SpecializedFunctionShape {
    pub(super) fn new(
        arguments: Vec<SpecializedValueShape>,
        return_: SpecializedValueShape,
    ) -> Self {
        Self {
            arguments: arguments.into_boxed_slice(),
            return_: Box::new(return_),
        }
    }

    pub(super) fn instantiate(
        shape: &FunctionShape,
        substitution: &SpecializedTypeSubstitution,
    ) -> Self {
        Self {
            arguments: shape
                .argument_shapes()
                .iter()
                .map(|shape| SpecializedValueShape::instantiate(shape, substitution))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            return_: Box::new(SpecializedValueShape::instantiate(
                shape.return_shape(),
                substitution,
            )),
        }
    }

    pub(super) fn arguments(&self) -> &[SpecializedValueShape] {
        &self.arguments
    }

    pub(super) fn return_(&self) -> &SpecializedValueShape {
        &self.return_
    }

    pub(super) fn representation(
        &self,
        representations: &RepresentationContext,
    ) -> FunctionRepresentation {
        if self
            .arguments
            .iter()
            .any(|argument| !representations.is_inhabited(argument))
        {
            return FunctionRepresentation::Symbolic;
        }

        match representations.inhabitation(&self.return_) {
            ValueInhabitation::Uninhabited(shape) => FunctionRepresentation::Never(shape),
            ValueInhabitation::Inhabited(return_) => FunctionRepresentation::Executable(return_),
        }
    }

    pub(super) fn arguments_representation(
        &self,
        representations: &RepresentationContext,
    ) -> FunctionArgumentsRepresentation {
        if self
            .arguments
            .iter()
            .all(|argument| representations.is_inhabited(argument))
        {
            FunctionArgumentsRepresentation::Inhabited
        } else {
            FunctionArgumentsRepresentation::Symbolic
        }
    }

    pub(super) fn to_module_shape(&self) -> FunctionShape {
        FunctionShape::new(
            self.arguments
                .iter()
                .map(SpecializedValueShape::to_module_shape)
                .collect(),
            self.return_.to_module_shape(),
        )
    }
}

impl SpecializedCustomValueShape {
    pub(super) fn instantiate(
        shape: &crate::plan::CustomValueShape,
        substitution: &SpecializedTypeSubstitution,
    ) -> Self {
        Self {
            name: shape.type_name().clone(),
            arguments: shape
                .arguments()
                .iter()
                .map(|shape| SpecializedValueShape::instantiate(shape, substitution))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            constructor: shape.constructor(),
        }
    }

    pub(super) fn arguments(&self) -> &[SpecializedValueShape] {
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
                .map(SpecializedValueShape::to_module_shape)
                .collect(),
            self.constructor,
        )
    }
}

impl SpecializedCustomConstructor {
    pub(super) fn instantiate(
        constructor: crate::plan::CustomConstructor,
        substitution: &SpecializedTypeSubstitution,
    ) -> Self {
        let (type_, name, index, fields) = constructor.into_parts();
        Self {
            type_: SpecializedCustomValueShape::instantiate(
                &crate::plan::CustomValueShape::any(type_),
                substitution,
            ),
            name,
            index,
            fields: fields
                .into_iter()
                .map(|field| {
                    let (label, type_) = field.into_parts();
                    SpecializedCustomConstructorField {
                        label,
                        shape: SpecializedValueShape::instantiate(
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
        SpecializedCustomValueShape,
        EcoString,
        usize,
        Box<[SpecializedCustomConstructorField]>,
    ) {
        (self.type_, self.name, self.index, self.fields)
    }
}

impl SpecializedCustomConstructorField {
    pub(super) fn into_parts(self) -> (Option<EcoString>, SpecializedValueShape) {
        (self.label, self.shape)
    }
}

impl SpecializedValueShape {
    pub(super) fn instantiate(
        shape: &ValueShape,
        substitution: &SpecializedTypeSubstitution,
    ) -> Self {
        match shape {
            ValueShape::Parameter(parameter) => substitution.resolve(*parameter),
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
                SpecializedFunctionShape::instantiate(function, substitution),
            )),
            ValueShape::Custom(custom) => Self::Custom(SpecializedCustomValueShape::instantiate(
                custom,
                substitution,
            )),
        }
    }

    pub(super) fn to_module_shape(&self) -> ValueShape {
        match self {
            Self::Parameter(parameter) => ValueShape::Parameter(*parameter),
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

    pub(super) fn storage_representation(&self) -> StorageRepresentation {
        match self {
            Self::Parameter(parameter) => StorageRepresentation::Parameter(*parameter),
            Self::Int => StorageRepresentation::Stored(StoredValueShape::Int),
            Self::Float => StorageRepresentation::Stored(StoredValueShape::Float),
            Self::String => StorageRepresentation::Stored(StoredValueShape::String),
            Self::BitArray => StorageRepresentation::Stored(StoredValueShape::BitArray),
            Self::UtfCodepoint => StorageRepresentation::Stored(StoredValueShape::UtfCodepoint),
            Self::Bool => StorageRepresentation::Stored(StoredValueShape::Bool),
            Self::Nil => StorageRepresentation::Stored(StoredValueShape::Nil),
            Self::Tuple(elements) => {
                StorageRepresentation::Stored(StoredValueShape::Tuple(elements.clone()))
            }
            Self::List(item) => StorageRepresentation::Stored(StoredValueShape::List(item.clone())),
            Self::Function(function) => {
                StorageRepresentation::Stored(StoredValueShape::Function(function.clone()))
            }
            Self::Custom(custom) => {
                StorageRepresentation::Stored(StoredValueShape::Custom(custom.clone()))
            }
        }
    }
}

impl RepresentationContext {
    pub(super) fn stored_shape(&self, shape: &SpecializedValueShape) -> Option<StoredValueShape> {
        match self.representation(shape) {
            ValueRepresentation::Stored(shape) => Some(shape),
            ValueRepresentation::Uninhabited(_) => None,
        }
    }

    pub(super) fn new(custom_types: Vec<crate::plan::CustomTypeDefinition>) -> Self {
        Self {
            custom_types: custom_types
                .into_iter()
                .map(|definition| (definition.name().clone(), definition))
                .collect(),
        }
    }

    pub(super) fn representation(&self, shape: &SpecializedValueShape) -> ValueRepresentation {
        self.representation_with(shape)
    }

    pub(super) fn is_inhabited(&self, shape: &SpecializedValueShape) -> bool {
        matches!(self.inhabitation(shape), ValueInhabitation::Inhabited(_))
    }

    pub(super) fn inhabitation(&self, shape: &SpecializedValueShape) -> ValueInhabitation {
        let mut visiting = HashSet::new();
        let mut known_inhabited = HashSet::new();
        self.inhabitation_with(shape, &mut visiting, &mut known_inhabited)
    }

    pub(super) fn tuple_inhabitation(
        &self,
        elements: &[SpecializedValueShape],
    ) -> CompoundInhabitation<UninhabitedTupleValueShape> {
        let mut visiting = HashSet::new();
        let mut known_inhabited = HashSet::new();
        self.tuple_inhabitation_with(elements, &mut visiting, &mut known_inhabited)
    }

    pub(super) fn custom_inhabitation(
        &self,
        custom: &SpecializedCustomValueShape,
    ) -> CompoundInhabitation<UninhabitedCustomValueShape> {
        let mut visiting = HashSet::new();
        let mut known_inhabited = HashSet::new();
        match self.custom_inhabitation_with(custom, &mut visiting, &mut known_inhabited) {
            CustomInhabitation::Inhabited => CompoundInhabitation::Inhabited,
            CustomInhabitation::Uninhabited(divergence) => {
                CompoundInhabitation::Uninhabited(UninhabitedCustomValueShape {
                    shape: custom.clone(),
                    divergence,
                })
            }
        }
    }

    pub(super) fn custom_has_value(&self, custom: &SpecializedCustomValueShape) -> bool {
        matches!(
            self.custom_inhabitation(custom),
            CompoundInhabitation::Inhabited
        )
    }

    pub(super) fn custom_constructor_match(
        &self,
        source: &SpecializedCustomValueShape,
        constructor: usize,
    ) -> CustomConstructorMatch {
        match source.constructor {
            CustomConstructorRefinement::Exact(actual) => {
                if actual == constructor {
                    CustomConstructorMatch::Certain
                } else {
                    CustomConstructorMatch::Impossible
                }
            }
            CustomConstructorRefinement::Any => {
                let exact = |index| SpecializedCustomValueShape {
                    name: source.name.clone(),
                    arguments: source.arguments.clone(),
                    constructor: CustomConstructorRefinement::Exact(index),
                };
                if !self.custom_has_value(&exact(constructor)) {
                    return CustomConstructorMatch::Impossible;
                }

                let constructor_count = if is_result(&source.name) {
                    2
                } else {
                    self.custom_types[&source.name].constructors().len()
                };
                if (0..constructor_count)
                    .filter(|index| *index != constructor)
                    .all(|index| !self.custom_has_value(&exact(index)))
                {
                    CustomConstructorMatch::Certain
                } else {
                    CustomConstructorMatch::Dynamic
                }
            }
        }
    }

    fn representation_with(&self, shape: &SpecializedValueShape) -> ValueRepresentation {
        match shape {
            SpecializedValueShape::Parameter(parameter) => {
                ValueRepresentation::Uninhabited(*parameter)
            }
            SpecializedValueShape::Int => ValueRepresentation::Stored(StoredValueShape::Int),
            SpecializedValueShape::Float => ValueRepresentation::Stored(StoredValueShape::Float),
            SpecializedValueShape::String => ValueRepresentation::Stored(StoredValueShape::String),
            SpecializedValueShape::BitArray => {
                ValueRepresentation::Stored(StoredValueShape::BitArray)
            }
            SpecializedValueShape::UtfCodepoint => {
                ValueRepresentation::Stored(StoredValueShape::UtfCodepoint)
            }
            SpecializedValueShape::Bool => ValueRepresentation::Stored(StoredValueShape::Bool),
            SpecializedValueShape::Nil => ValueRepresentation::Stored(StoredValueShape::Nil),
            SpecializedValueShape::Tuple(elements) => {
                ValueRepresentation::Stored(StoredValueShape::Tuple(elements.clone()))
            }
            SpecializedValueShape::List(item) => {
                ValueRepresentation::Stored(StoredValueShape::List(item.clone()))
            }
            SpecializedValueShape::Function(function) => {
                ValueRepresentation::Stored(StoredValueShape::Function(function.clone()))
            }
            SpecializedValueShape::Custom(custom) => {
                ValueRepresentation::Stored(StoredValueShape::Custom(custom.clone()))
            }
        }
    }

    fn inhabitation_with(
        &self,
        shape: &SpecializedValueShape,
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> ValueInhabitation {
        match shape {
            SpecializedValueShape::Parameter(parameter) => {
                ValueInhabitation::Uninhabited(UninhabitedValueShape::Parameter(*parameter))
            }
            SpecializedValueShape::Int => ValueInhabitation::Inhabited(StoredValueShape::Int),
            SpecializedValueShape::Float => ValueInhabitation::Inhabited(StoredValueShape::Float),
            SpecializedValueShape::String => ValueInhabitation::Inhabited(StoredValueShape::String),
            SpecializedValueShape::BitArray => {
                ValueInhabitation::Inhabited(StoredValueShape::BitArray)
            }
            SpecializedValueShape::UtfCodepoint => {
                ValueInhabitation::Inhabited(StoredValueShape::UtfCodepoint)
            }
            SpecializedValueShape::Bool => ValueInhabitation::Inhabited(StoredValueShape::Bool),
            SpecializedValueShape::Nil => ValueInhabitation::Inhabited(StoredValueShape::Nil),
            SpecializedValueShape::List(item) => {
                ValueInhabitation::Inhabited(StoredValueShape::List(item.clone()))
            }
            SpecializedValueShape::Function(function) => {
                ValueInhabitation::Inhabited(StoredValueShape::Function(function.clone()))
            }
            SpecializedValueShape::Tuple(elements) => {
                match self.tuple_inhabitation_with(elements, visiting, known_inhabited) {
                    CompoundInhabitation::Inhabited => {
                        ValueInhabitation::Inhabited(StoredValueShape::Tuple(elements.clone()))
                    }
                    CompoundInhabitation::Uninhabited(shape) => {
                        ValueInhabitation::Uninhabited(UninhabitedValueShape::Tuple(shape))
                    }
                }
            }
            SpecializedValueShape::Custom(custom) => {
                match self.custom_inhabitation_with(custom, visiting, known_inhabited) {
                    CustomInhabitation::Inhabited => {
                        ValueInhabitation::Inhabited(StoredValueShape::Custom(custom.clone()))
                    }
                    CustomInhabitation::Uninhabited(divergence) => ValueInhabitation::Uninhabited(
                        UninhabitedValueShape::Custom(UninhabitedCustomValueShape {
                            shape: custom.clone(),
                            divergence,
                        }),
                    ),
                }
            }
        }
    }

    fn tuple_inhabitation_with(
        &self,
        elements: &[SpecializedValueShape],
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> CompoundInhabitation<UninhabitedTupleValueShape> {
        for (index, element) in elements.iter().enumerate() {
            if matches!(
                self.inhabitation_with(element, visiting, known_inhabited),
                ValueInhabitation::Uninhabited(_)
            ) {
                return CompoundInhabitation::Uninhabited(UninhabitedTupleValueShape {
                    elements: elements.to_vec().into_boxed_slice(),
                    diverging: index,
                });
            }
        }
        CompoundInhabitation::Inhabited
    }

    fn custom_inhabitation_with(
        &self,
        custom: &SpecializedCustomValueShape,
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> CustomInhabitation {
        let arguments = custom
            .arguments()
            .iter()
            .map(|argument| self.shape_has_value(argument, visiting, known_inhabited))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        if is_result(&custom.name) {
            return match custom.constructor {
                CustomConstructorRefinement::Any if arguments.iter().any(|argument| *argument) => {
                    CustomInhabitation::Inhabited
                }
                CustomConstructorRefinement::Any => {
                    CustomInhabitation::Uninhabited(CustomConstructorDivergence::Every {
                        fields: vec![0, 0].into_boxed_slice(),
                    })
                }
                CustomConstructorRefinement::Exact(index) if arguments[index] => {
                    CustomInhabitation::Inhabited
                }
                CustomConstructorRefinement::Exact(_) => {
                    CustomInhabitation::Uninhabited(CustomConstructorDivergence::Exact { field: 0 })
                }
            };
        }

        let definition = &self.custom_types[&custom.name];
        let key = CustomRepresentationKey {
            name: custom.name.clone(),
            arguments: arguments.clone(),
            constructor: custom.constructor,
        };
        visiting.insert(key.clone());
        let inhabitation = match custom.constructor {
            CustomConstructorRefinement::Any => {
                let mut fields = Vec::with_capacity(definition.constructors().len());
                for constructor in definition.constructors() {
                    match self.constructor_inhabitation(
                        constructor,
                        &arguments,
                        visiting,
                        known_inhabited,
                    ) {
                        ConstructorInhabitation::Inhabited => {
                            visiting.remove(&key);
                            return CustomInhabitation::Inhabited;
                        }
                        ConstructorInhabitation::Uninhabited { field } => fields.push(field),
                    }
                }
                CustomInhabitation::Uninhabited(CustomConstructorDivergence::Every {
                    fields: fields.into_boxed_slice(),
                })
            }
            CustomConstructorRefinement::Exact(index) => {
                match self.constructor_inhabitation(
                    &definition.constructors()[index],
                    &arguments,
                    visiting,
                    known_inhabited,
                ) {
                    ConstructorInhabitation::Inhabited => CustomInhabitation::Inhabited,
                    ConstructorInhabitation::Uninhabited { field } => {
                        CustomInhabitation::Uninhabited(CustomConstructorDivergence::Exact {
                            field,
                        })
                    }
                }
            }
        };
        visiting.remove(&key);
        inhabitation
    }

    fn constructor_inhabitation(
        &self,
        constructor: &crate::plan::CustomConstructorDefinition,
        arguments: &[bool],
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> ConstructorInhabitation {
        for (field, definition) in constructor.fields().iter().enumerate() {
            if !self.template_inhabited(definition.type_(), arguments, visiting, known_inhabited) {
                return ConstructorInhabitation::Uninhabited { field };
            }
        }
        ConstructorInhabitation::Inhabited
    }

    fn shape_has_value(
        &self,
        shape: &SpecializedValueShape,
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> bool {
        match shape {
            SpecializedValueShape::Parameter(_) => false,
            SpecializedValueShape::Int
            | SpecializedValueShape::Float
            | SpecializedValueShape::String
            | SpecializedValueShape::BitArray
            | SpecializedValueShape::UtfCodepoint
            | SpecializedValueShape::Bool
            | SpecializedValueShape::Nil
            | SpecializedValueShape::List(_)
            | SpecializedValueShape::Function(_) => true,
            SpecializedValueShape::Tuple(elements) => elements
                .iter()
                .all(|element| self.shape_has_value(element, visiting, known_inhabited)),
            SpecializedValueShape::Custom(custom) => {
                let arguments = custom
                    .arguments()
                    .iter()
                    .map(|argument| self.shape_has_value(argument, visiting, known_inhabited))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                self.custom_key_inhabited(
                    CustomRepresentationKey {
                        name: custom.name.clone(),
                        arguments,
                        constructor: custom.constructor,
                    },
                    visiting,
                    known_inhabited,
                )
            }
        }
    }

    fn custom_key_inhabited(
        &self,
        key: CustomRepresentationKey,
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> bool {
        if known_inhabited.contains(&key) {
            return true;
        }
        if !visiting.insert(key.clone()) {
            return false;
        }

        let inhabited = if is_result(&key.name) {
            match key.constructor {
                CustomConstructorRefinement::Any => key.arguments.iter().any(|argument| *argument),
                CustomConstructorRefinement::Exact(index) => key.arguments[index],
            }
        } else {
            let definition = &self.custom_types[&key.name];
            match key.constructor {
                CustomConstructorRefinement::Any => {
                    definition.constructors().iter().any(|constructor| {
                        constructor.fields().iter().all(|field| {
                            self.template_inhabited(
                                field.type_(),
                                &key.arguments,
                                visiting,
                                known_inhabited,
                            )
                        })
                    })
                }
                CustomConstructorRefinement::Exact(index) => definition.constructors()[index]
                    .fields()
                    .iter()
                    .all(|field| {
                        self.template_inhabited(
                            field.type_(),
                            &key.arguments,
                            visiting,
                            known_inhabited,
                        )
                    }),
            }
        };

        visiting.remove(&key);
        if inhabited {
            known_inhabited.insert(key);
        }
        inhabited
    }

    fn template_inhabited(
        &self,
        template: &crate::plan::CustomTypeTemplate,
        arguments: &[bool],
        visiting: &mut HashSet<CustomRepresentationKey>,
        known_inhabited: &mut HashSet<CustomRepresentationKey>,
    ) -> bool {
        use crate::plan::CustomTypeTemplate as T;

        match template {
            T::Int
            | T::Float
            | T::String
            | T::BitArray
            | T::UtfCodepoint
            | T::Bool
            | T::Nil
            | T::List(_)
            | T::Function { .. } => true,
            T::Tuple(elements) => elements.iter().all(|element| {
                self.template_inhabited(element, arguments, visiting, known_inhabited)
            }),
            T::Custom {
                name,
                arguments: templates,
            } => {
                let arguments = templates
                    .iter()
                    .map(|template| {
                        self.template_inhabited(template, arguments, visiting, known_inhabited)
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                self.custom_key_inhabited(
                    CustomRepresentationKey {
                        name: name.clone(),
                        arguments,
                        constructor: CustomConstructorRefinement::Any,
                    },
                    visiting,
                    known_inhabited,
                )
            }
            T::Parameter(parameter) => arguments[parameter.0],
        }
    }
}

impl UninhabitedCustomValueShape {
    pub(super) fn diverging_field(&self, constructor: usize) -> usize {
        match &self.divergence {
            CustomConstructorDivergence::Exact { field } => *field,
            CustomConstructorDivergence::Every { fields } => fields[constructor],
        }
    }
}

impl UninhabitedTupleValueShape {
    pub(super) fn diverging(&self) -> usize {
        self.diverging
    }
}

fn is_result(name: &CustomTypeName) -> bool {
    name.package().is_empty() && name.module() == "gleam" && name.name() == "Result"
}

impl StoredValueShape {
    pub(super) fn instantiate(
        shape: &crate::plan::ValueStorageShape,
        substitution: &SpecializedTypeSubstitution,
    ) -> Self {
        match shape {
            crate::plan::ValueStorageShape::Int => Self::Int,
            crate::plan::ValueStorageShape::Float => Self::Float,
            crate::plan::ValueStorageShape::String => Self::String,
            crate::plan::ValueStorageShape::BitArray => Self::BitArray,
            crate::plan::ValueStorageShape::UtfCodepoint => Self::UtfCodepoint,
            crate::plan::ValueStorageShape::Bool => Self::Bool,
            crate::plan::ValueStorageShape::Nil => Self::Nil,
            crate::plan::ValueStorageShape::Tuple(elements) => Self::Tuple(
                elements
                    .iter()
                    .map(|element| SpecializedValueShape::instantiate(element, substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            crate::plan::ValueStorageShape::List(item) => Self::List(Box::new(
                SpecializedValueShape::instantiate(item, substitution),
            )),
            crate::plan::ValueStorageShape::Function(function) => Self::Function(Box::new(
                SpecializedFunctionShape::instantiate(function, substitution),
            )),
            crate::plan::ValueStorageShape::Custom(custom) => Self::Custom(
                SpecializedCustomValueShape::instantiate(custom, substitution),
            ),
        }
    }

    pub(super) fn to_specialized(&self) -> SpecializedValueShape {
        match self {
            Self::Int => SpecializedValueShape::Int,
            Self::Float => SpecializedValueShape::Float,
            Self::String => SpecializedValueShape::String,
            Self::BitArray => SpecializedValueShape::BitArray,
            Self::UtfCodepoint => SpecializedValueShape::UtfCodepoint,
            Self::Bool => SpecializedValueShape::Bool,
            Self::Nil => SpecializedValueShape::Nil,
            Self::Tuple(elements) => SpecializedValueShape::Tuple(elements.clone()),
            Self::List(item) => SpecializedValueShape::List(item.clone()),
            Self::Function(function) => SpecializedValueShape::Function(function.clone()),
            Self::Custom(custom) => SpecializedValueShape::Custom(custom.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomConstructorDivergence, CustomConstructorMatch, FunctionRepresentation,
        Representability, RepresentationContext, SpecializationKey, SpecializedCustomValueShape,
        SpecializedFunctionShape, SpecializedTypeSubstitution, SpecializedValueShape,
        StoredValueShape, UninhabitedCustomValueShape, UninhabitedTupleValueShape,
        UninhabitedValueShape, ValueRepresentation,
    };
    use crate::plan::{
        CustomConstructorDefinition, CustomConstructorRefinement, CustomFieldDefinition,
        CustomTypeDefinition, CustomTypeName, CustomTypeParameterId, CustomTypePublicity,
        CustomTypeTemplate, CustomValueShape, FunctionShape, FunctionTemplateId, TypeParameterId,
        TypeScheme, ValueShape,
    };

    fn custom_shape(
        name: CustomTypeName,
        constructor: CustomConstructorRefinement,
    ) -> SpecializedCustomValueShape {
        SpecializedCustomValueShape {
            name,
            arguments: vec![SpecializedValueShape::Parameter(TypeParameterId(0))]
                .into_boxed_slice(),
            constructor,
        }
    }

    fn representation_context() -> (RepresentationContext, CustomTypeName, CustomTypeName) {
        let phantom = CustomTypeName::new("geam".into(), "main".into(), "Phantom".into());
        let choice = CustomTypeName::new("geam".into(), "main".into(), "Choice".into());
        let definitions = vec![
            CustomTypeDefinition::new(
                phantom.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Phantom".into(),
                    0,
                    Vec::new(),
                )],
            ),
            CustomTypeDefinition::new(
                choice.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![
                    CustomConstructorDefinition::new("Empty".into(), 0, Vec::new()),
                    CustomConstructorDefinition::new(
                        "Filled".into(),
                        1,
                        vec![CustomFieldDefinition::new(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        )],
                    ),
                ],
            ),
        ];
        (RepresentationContext::new(definitions), phantom, choice)
    }

    #[test]
    fn concrete_specialization_preserves_recursive_shape_metadata() {
        let substitution = SpecializedTypeSubstitution::instantiate(
            &TypeScheme::new(2)
                .try_substitution(vec![ValueShape::Int, ValueShape::String])
                .expect("two arguments should match the scheme"),
            &SpecializedTypeSubstitution::empty(),
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
        let concrete = SpecializedValueShape::instantiate(&shape, &substitution);

        assert_eq!(
            concrete.to_module_shape(),
            shape.substitute(
                &TypeScheme::new(2)
                    .try_substitution(vec![ValueShape::Int, ValueShape::String])
                    .expect("two arguments should match the scheme")
            )
        );
        assert_eq!(
            concrete.to_module_shape().value_type(),
            concrete.to_module_shape().value_type()
        );
    }

    #[test]
    fn monomorphic_specialization_key_has_empty_substitution() {
        let key = SpecializationKey::monomorphic(FunctionTemplateId::new(7));

        assert_eq!(key.template(), FunctionTemplateId::new(7));
        assert_eq!(key.substitution().arguments.as_ref(), &[]);
    }

    #[test]
    fn partial_specialization_preserves_body_local_parameters() {
        let substitution = SpecializedTypeSubstitution::instantiate(
            &TypeScheme::new(1)
                .try_substitution(vec![ValueShape::Int])
                .expect("one argument should match the scheme"),
            &SpecializedTypeSubstitution::empty(),
        );

        assert_eq!(
            SpecializedValueShape::instantiate(
                &ValueShape::Parameter(TypeParameterId(0)),
                &substitution,
            ),
            SpecializedValueShape::Int,
        );
        assert_eq!(
            SpecializedValueShape::instantiate(
                &ValueShape::Parameter(TypeParameterId(1)),
                &substitution,
            ),
            SpecializedValueShape::Parameter(TypeParameterId(1)),
        );
    }

    #[test]
    fn representation_preserves_empty_only_and_phantom_storage() {
        let (context, phantom, choice) = representation_context();
        let parameter = SpecializedValueShape::Parameter(TypeParameterId(0));
        let parameter_list = SpecializedValueShape::List(Box::new(parameter.clone()));
        let phantom = custom_shape(phantom, CustomConstructorRefinement::Exact(0));
        let choice = custom_shape(choice, CustomConstructorRefinement::Any);
        let result = SpecializedCustomValueShape {
            name: CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            arguments: vec![
                SpecializedValueShape::Parameter(TypeParameterId(0)),
                SpecializedValueShape::Int,
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };

        assert_eq!(
            context.representation(&parameter),
            ValueRepresentation::Uninhabited(TypeParameterId(0))
        );
        assert_eq!(
            context.representation(&SpecializedValueShape::Tuple(
                vec![parameter.clone()].into_boxed_slice()
            )),
            ValueRepresentation::Stored(StoredValueShape::Tuple(
                vec![parameter.clone()].into_boxed_slice(),
            ))
        );
        assert_eq!(
            context.representation(&parameter_list),
            ValueRepresentation::Stored(StoredValueShape::List(Box::new(parameter)))
        );
        assert_eq!(
            context.representation(&SpecializedValueShape::Custom(phantom.clone())),
            ValueRepresentation::Stored(StoredValueShape::Custom(phantom))
        );
        assert_eq!(
            context.representation(&SpecializedValueShape::Custom(choice.clone())),
            ValueRepresentation::Stored(StoredValueShape::Custom(choice))
        );
        assert_eq!(
            context.representation(&SpecializedValueShape::Custom(result.clone())),
            ValueRepresentation::Stored(StoredValueShape::Custom(result))
        );
    }

    #[test]
    fn uninhabited_values_propagate_through_specialization_combinators() {
        let mapped = Representability::<usize>::Uninhabited.map(std::convert::identity);
        let chained = Representability::<Representability<usize>>::Uninhabited
            .and_then(std::convert::identity);
        let collected: Representability<Vec<usize>> = Representability::collect(vec![
            Representability::Inhabited(1),
            Representability::Uninhabited,
            Representability::Inhabited(3),
        ]);
        let inhabited: Representability<Vec<usize>> = Representability::collect(vec![
            Representability::Inhabited(1),
            Representability::Inhabited(2),
        ]);

        assert_eq!(mapped, Representability::Uninhabited);
        assert_eq!(chained, Representability::Uninhabited);
        assert_eq!(collected, Representability::Uninhabited);
        assert_eq!(inhabited, Representability::Inhabited(vec![1, 2]));
        assert_eq!(
            Representability::Inhabited(Representability::Inhabited(2))
                .and_then(std::convert::identity),
            Representability::Inhabited(2),
        );
    }

    #[test]
    fn custom_representation_memoization_breaks_recursive_type_cycles() {
        let context = RepresentationContext::new(Vec::new());
        let key = super::CustomRepresentationKey {
            name: CustomTypeName::new("geam".into(), "main".into(), "Recursive".into()),
            arguments: Vec::new().into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };

        let mut visiting = std::collections::HashSet::from([key.clone()]);
        assert!(!context.custom_key_inhabited(
            key.clone(),
            &mut visiting,
            &mut std::collections::HashSet::new(),
        ));

        let mut known_inhabited = std::collections::HashSet::from([key.clone()]);
        assert!(context.custom_key_inhabited(
            key,
            &mut std::collections::HashSet::new(),
            &mut known_inhabited,
        ));
    }

    #[test]
    fn uninhabited_function_representation_preserves_exact_proofs() {
        let (context, _, choice) = representation_context();
        let parameter = SpecializedValueShape::Parameter(TypeParameterId(0));

        assert_eq!(
            SpecializedFunctionShape::new(
                Vec::new(),
                SpecializedValueShape::Tuple(vec![parameter.clone()].into_boxed_slice()),
            )
            .representation(&context),
            FunctionRepresentation::Never(UninhabitedValueShape::Tuple(
                UninhabitedTupleValueShape {
                    elements: vec![parameter.clone()].into_boxed_slice(),
                    diverging: 0,
                },
            )),
        );
        let choice = custom_shape(choice, CustomConstructorRefinement::Exact(1));
        assert_eq!(
            SpecializedFunctionShape::new(
                Vec::new(),
                SpecializedValueShape::Custom(choice.clone()),
            )
            .representation(&context),
            FunctionRepresentation::Never(UninhabitedValueShape::Custom(
                UninhabitedCustomValueShape {
                    shape: choice,
                    divergence: CustomConstructorDivergence::Exact { field: 0 },
                },
            )),
        );
    }

    #[test]
    fn custom_value_analysis_distinguishes_phantom_and_parameter_fields() {
        let (context, phantom, choice) = representation_context();
        let result = CustomTypeName::new("".into(), "gleam".into(), "Result".into());

        assert!(context.custom_has_value(&custom_shape(
            phantom,
            CustomConstructorRefinement::Exact(0),
        )));
        assert!(!context.custom_has_value(&custom_shape(
            choice.clone(),
            CustomConstructorRefinement::Exact(1),
        )));
        assert!(context.custom_has_value(&custom_shape(
            choice.clone(),
            CustomConstructorRefinement::Any,
        )));

        let exact_ok = SpecializedValueShape::Custom(SpecializedCustomValueShape {
            name: result.clone(),
            arguments: vec![
                SpecializedValueShape::Int,
                SpecializedValueShape::Parameter(TypeParameterId(1)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Exact(0),
        });
        let exact_error = SpecializedValueShape::Custom(SpecializedCustomValueShape {
            name: result,
            arguments: vec![
                SpecializedValueShape::Int,
                SpecializedValueShape::Parameter(TypeParameterId(1)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Exact(1),
        });
        assert!(context.custom_has_value(&SpecializedCustomValueShape {
            name: choice.clone(),
            arguments: vec![exact_ok].into_boxed_slice(),
            constructor: CustomConstructorRefinement::Exact(1),
        }));
        assert!(!context.custom_has_value(&SpecializedCustomValueShape {
            name: choice,
            arguments: vec![exact_error].into_boxed_slice(),
            constructor: CustomConstructorRefinement::Exact(1),
        }));
    }

    #[test]
    fn result_inhabitation_preserves_any_and_exact_constructor_proofs() {
        let context = RepresentationContext::new(Vec::new());
        let result = CustomTypeName::new("".into(), "gleam".into(), "Result".into());
        let unresolved = SpecializedCustomValueShape {
            name: result.clone(),
            arguments: vec![
                SpecializedValueShape::Parameter(TypeParameterId(0)),
                SpecializedValueShape::Parameter(TypeParameterId(1)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };

        let proof = UninhabitedCustomValueShape {
            shape: unresolved.clone(),
            divergence: CustomConstructorDivergence::Every {
                fields: vec![0, 0].into_boxed_slice(),
            },
        };
        let proof_of = |inhabitation| match inhabitation {
            super::CompoundInhabitation::Uninhabited(proof) => Some(proof),
            super::CompoundInhabitation::Inhabited => None,
        };
        assert_eq!(
            proof_of(context.custom_inhabitation(&unresolved)),
            Some(proof.clone()),
        );
        assert_eq!(proof_of(super::CompoundInhabitation::Inhabited), None);
        assert_eq!(proof.diverging_field(0), 0);
        assert_eq!(proof.diverging_field(1), 0);

        let inhabited = SpecializedCustomValueShape {
            name: result.clone(),
            arguments: vec![
                SpecializedValueShape::Int,
                SpecializedValueShape::Parameter(TypeParameterId(1)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };
        assert_eq!(proof_of(context.custom_inhabitation(&inhabited)), None);

        let exact_error = SpecializedCustomValueShape {
            name: result.clone(),
            arguments: unresolved.arguments.clone(),
            constructor: CustomConstructorRefinement::Exact(1),
        };
        assert_eq!(
            proof_of(context.custom_inhabitation(&exact_error)),
            Some(UninhabitedCustomValueShape {
                shape: exact_error,
                divergence: CustomConstructorDivergence::Exact { field: 0 },
            }),
        );

        let exact_ok = SpecializedCustomValueShape {
            name: result,
            arguments: vec![
                SpecializedValueShape::Int,
                SpecializedValueShape::Parameter(TypeParameterId(1)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Exact(0),
        };
        assert!(context.custom_has_value(&exact_ok));
    }

    #[test]
    fn custom_constructor_match_distinguishes_exact_and_inhabited_candidates() {
        let context = RepresentationContext::new(Vec::new());
        let result = CustomTypeName::new("".into(), "gleam".into(), "Result".into());
        let partially_inhabited = SpecializedCustomValueShape {
            name: result.clone(),
            arguments: vec![
                SpecializedValueShape::Int,
                SpecializedValueShape::Parameter(TypeParameterId(0)),
            ]
            .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };

        assert_eq!(
            context.custom_constructor_match(&partially_inhabited, 0),
            CustomConstructorMatch::Certain,
        );
        assert_eq!(
            context.custom_constructor_match(&partially_inhabited, 1),
            CustomConstructorMatch::Impossible,
        );

        let fully_inhabited = SpecializedCustomValueShape {
            name: result,
            arguments: vec![SpecializedValueShape::Int, SpecializedValueShape::String]
                .into_boxed_slice(),
            constructor: CustomConstructorRefinement::Any,
        };
        assert_eq!(
            context.custom_constructor_match(&fully_inhabited, 0),
            CustomConstructorMatch::Dynamic,
        );
        assert_eq!(
            context.custom_constructor_match(&fully_inhabited, 1),
            CustomConstructorMatch::Dynamic,
        );

        let exact = SpecializedCustomValueShape {
            constructor: CustomConstructorRefinement::Exact(0),
            ..fully_inhabited
        };
        assert_eq!(
            context.custom_constructor_match(&exact, 0),
            CustomConstructorMatch::Certain,
        );
        assert_eq!(
            context.custom_constructor_match(&exact, 1),
            CustomConstructorMatch::Impossible,
        );
    }

    #[test]
    fn function_representation_requires_inhabited_arguments_and_return() {
        let (context, phantom, choice) = representation_context();
        let parameter = SpecializedValueShape::Parameter(TypeParameterId(0));

        assert_eq!(
            SpecializedFunctionShape::new(Vec::new(), SpecializedValueShape::Int)
                .representation(&context),
            FunctionRepresentation::Executable(StoredValueShape::Int),
        );
        assert_eq!(
            SpecializedFunctionShape::new(vec![parameter.clone()], SpecializedValueShape::Int)
                .representation(&context),
            FunctionRepresentation::Symbolic,
        );
        assert_eq!(
            SpecializedFunctionShape::new(Vec::new(), parameter.clone()).representation(&context),
            FunctionRepresentation::Never(UninhabitedValueShape::Parameter(TypeParameterId(0))),
        );
        assert_eq!(
            SpecializedFunctionShape::new(
                vec![SpecializedValueShape::Tuple(
                    vec![parameter.clone()].into_boxed_slice(),
                )],
                SpecializedValueShape::Int,
            )
            .representation(&context),
            FunctionRepresentation::Symbolic,
        );
        assert_eq!(
            SpecializedFunctionShape::new(
                Vec::new(),
                SpecializedValueShape::List(Box::new(parameter.clone())),
            )
            .representation(&context),
            FunctionRepresentation::Executable(StoredValueShape::List(
                Box::new(parameter.clone(),)
            )),
        );
        let phantom = custom_shape(phantom, CustomConstructorRefinement::Exact(0));
        assert_eq!(
            SpecializedFunctionShape::new(
                Vec::new(),
                SpecializedValueShape::Custom(phantom.clone()),
            )
            .representation(&context),
            FunctionRepresentation::Executable(StoredValueShape::Custom(phantom)),
        );
        let choice = custom_shape(choice, CustomConstructorRefinement::Any);
        assert_eq!(
            SpecializedFunctionShape::new(
                Vec::new(),
                SpecializedValueShape::Custom(choice.clone()),
            )
            .representation(&context),
            FunctionRepresentation::Executable(StoredValueShape::Custom(choice)),
        );
    }
}
