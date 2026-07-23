use super::{
    ConstantBitArrayValue, ConstantBoolValue, ConstantCustomValue, ConstantFloatValue,
    ConstantFunctionValue, ConstantIntValue, ConstantListConstructionError, ConstantNilValue,
    ConstantStringValue, ConstantTupleValue,
};
use crate::plan::{
    CustomValueShape, FunctionShape, TypeParameterId, TypeSubstitution, ValueRepresentation,
    ValueShape, ValueStorageShape,
};
use vec1::Vec1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantGenericListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantIntListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantStringListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBitArrayListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantUtfCodepointListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantCustomListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFloatListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBoolListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantNilListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantTupleListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantParameterListListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantListListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFunctionListTemplateId(pub(super) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum ConstantListTemplate {
    Generic {
        id: ConstantGenericListTemplateId,
        parameter: TypeParameterId,
    },
    Int(ConstantIntListTemplateId),
    String(ConstantStringListTemplateId),
    BitArray(ConstantBitArrayListTemplateId),
    UtfCodepoint(ConstantUtfCodepointListTemplateId),
    Custom {
        id: ConstantCustomListTemplateId,
        shape: CustomValueShape,
    },
    Float(ConstantFloatListTemplateId),
    Bool(ConstantBoolListTemplateId),
    Nil(ConstantNilListTemplateId),
    Tuple {
        id: ConstantTupleListTemplateId,
        shape: Box<[ValueShape]>,
    },
    ParameterList {
        id: ConstantParameterListListTemplateId,
        parameter: TypeParameterId,
    },
    List {
        id: ConstantListListTemplateId,
        shape: ValueStorageShape,
    },
    Function {
        id: ConstantFunctionListTemplateId,
        shape: FunctionShape,
    },
}

impl ConstantListTemplate {
    pub(super) fn from_item_shape(shape: ValueShape, index: usize) -> Self {
        match shape {
            ValueShape::Parameter(parameter) => Self::Generic {
                id: ConstantGenericListTemplateId(index),
                parameter,
            },
            ValueShape::Int => Self::Int(ConstantIntListTemplateId(index)),
            ValueShape::String => Self::String(ConstantStringListTemplateId(index)),
            ValueShape::BitArray => Self::BitArray(ConstantBitArrayListTemplateId(index)),
            ValueShape::UtfCodepoint => {
                Self::UtfCodepoint(ConstantUtfCodepointListTemplateId(index))
            }
            ValueShape::Custom(shape) => Self::Custom {
                id: ConstantCustomListTemplateId(index),
                shape,
            },
            ValueShape::Float => Self::Float(ConstantFloatListTemplateId(index)),
            ValueShape::Bool => Self::Bool(ConstantBoolListTemplateId(index)),
            ValueShape::Nil => Self::Nil(ConstantNilListTemplateId(index)),
            ValueShape::Tuple(shape) => Self::Tuple {
                id: ConstantTupleListTemplateId(index),
                shape,
            },
            ValueShape::List(shape) => match shape.representation() {
                ValueRepresentation::Uninhabited(parameter) => Self::ParameterList {
                    id: ConstantParameterListListTemplateId(index),
                    parameter,
                },
                ValueRepresentation::Stored(shape) => Self::List {
                    id: ConstantListListTemplateId(index),
                    shape,
                },
            },
            ValueShape::Function(shape) => Self::Function {
                id: ConstantFunctionListTemplateId(index),
                shape: *shape,
            },
        }
    }

    pub(super) fn item_shape(&self) -> ValueShape {
        match self {
            Self::Generic { parameter, .. } => ValueShape::Parameter(*parameter),
            Self::Int(_) => ValueShape::Int,
            Self::String(_) => ValueShape::String,
            Self::BitArray(_) => ValueShape::BitArray,
            Self::UtfCodepoint(_) => ValueShape::UtfCodepoint,
            Self::Custom { shape, .. } => ValueShape::Custom(shape.clone()),
            Self::Float(_) => ValueShape::Float,
            Self::Bool(_) => ValueShape::Bool,
            Self::Nil(_) => ValueShape::Nil,
            Self::Tuple { shape, .. } => ValueShape::Tuple(shape.clone()),
            Self::ParameterList { parameter, .. } => {
                ValueShape::List(Box::new(ValueShape::Parameter(*parameter)))
            }
            Self::List { shape, .. } => ValueShape::List(Box::new(shape.to_value_shape())),
            Self::Function { shape, .. } => ValueShape::Function(Box::new(shape.clone())),
        }
    }

    pub(super) fn instantiate(
        &self,
        module: crate::plan::ModuleId,
        substitution: TypeSubstitution,
    ) -> ConstantListInstantiation {
        match self {
            Self::Generic { id, parameter } => ConstantListInstantiation::from_generic_source(
                module,
                *id,
                substitution.clone(),
                ValueShape::Parameter(*parameter).substitute(&substitution),
            ),
            Self::Int(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Int,
            ),
            Self::String(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::String,
            ),
            Self::BitArray(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::BitArray,
            ),
            Self::UtfCodepoint(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::UtfCodepoint,
            ),
            Self::Custom { id, shape } => {
                ConstantListInstantiation::Custom(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape.substitute(&substitution),
                ))
            }
            Self::Float(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Float,
            ),
            Self::Bool(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Bool,
            ),
            Self::Nil(id) => Self::leaf_instantiation(
                module,
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Nil,
            ),
            Self::Tuple { id, shape } => {
                ConstantListInstantiation::Tuple(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape
                        .iter()
                        .map(|shape| shape.substitute(&substitution))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ))
            }
            Self::ParameterList { id, parameter } => {
                match ValueShape::Parameter(*parameter)
                    .substitute(&substitution)
                    .representation()
                {
                    ValueRepresentation::Uninhabited(parameter) => {
                        ConstantListInstantiation::ParameterList(
                            TypedConstantListInstantiation::in_module(
                                module,
                                ConstantListTemplateSource::Exact(*id),
                                substitution,
                                parameter,
                            ),
                        )
                    }
                    ValueRepresentation::Stored(shape) => {
                        ConstantListInstantiation::List(TypedConstantListInstantiation::in_module(
                            module,
                            ConstantNestedListTemplateSource::ParameterList(*id),
                            substitution,
                            shape,
                        ))
                    }
                }
            }
            Self::List { id, shape } => {
                ConstantListInstantiation::List(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantNestedListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape.substitute(&substitution),
                ))
            }
            Self::Function { id, shape } => {
                ConstantListInstantiation::Function(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape.substitute(&substitution),
                ))
            }
        }
    }

    fn leaf_instantiation<Id>(
        module: crate::plan::ModuleId,
        source: ConstantListTemplateSource<Id>,
        substitution: TypeSubstitution,
        into: fn(
            TypedConstantListInstantiation<ConstantListTemplateSource<Id>, ()>,
        ) -> ConstantListInstantiation,
    ) -> ConstantListInstantiation {
        into(TypedConstantListInstantiation::in_module(
            module,
            source,
            substitution,
            (),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConstantListTemplateSource<Id> {
    Generic(ConstantGenericListTemplateId),
    Exact(Id),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConstantNestedListTemplateSource {
    Generic(ConstantGenericListTemplateId),
    ParameterList(ConstantParameterListListTemplateId),
    Exact(ConstantListListTemplateId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TypedConstantListInstantiation<Source, Shape> {
    module: crate::plan::ModuleId,
    source: Source,
    substitution: TypeSubstitution,
    item_shape: Shape,
}

pub(crate) type ConstantGenericListInstantiation =
    TypedConstantListInstantiation<ConstantGenericListTemplateId, TypeParameterId>;
pub(crate) type ConstantIntListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantIntListTemplateId>, ()>;
pub(crate) type ConstantStringListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantStringListTemplateId>, ()>;
pub(crate) type ConstantBitArrayListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantBitArrayListTemplateId>, ()>;
pub(crate) type ConstantUtfCodepointListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantUtfCodepointListTemplateId>,
    (),
>;
pub(crate) type ConstantCustomListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantCustomListTemplateId>,
    CustomValueShape,
>;
pub(crate) type ConstantFloatListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantFloatListTemplateId>, ()>;
pub(crate) type ConstantBoolListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantBoolListTemplateId>, ()>;
pub(crate) type ConstantNilListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantNilListTemplateId>, ()>;
pub(crate) type ConstantTupleListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantTupleListTemplateId>,
    Box<[ValueShape]>,
>;
pub(crate) type ConstantParameterListListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantParameterListListTemplateId>,
    TypeParameterId,
>;
pub(crate) type ConstantListListInstantiation =
    TypedConstantListInstantiation<ConstantNestedListTemplateSource, ValueStorageShape>;
pub(crate) type ConstantFunctionListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantFunctionListTemplateId>,
    FunctionShape,
>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ConstantListInstantiation {
    Generic(ConstantGenericListInstantiation),
    Int(ConstantIntListInstantiation),
    String(ConstantStringListInstantiation),
    BitArray(ConstantBitArrayListInstantiation),
    UtfCodepoint(ConstantUtfCodepointListInstantiation),
    Custom(ConstantCustomListInstantiation),
    Float(ConstantFloatListInstantiation),
    Bool(ConstantBoolListInstantiation),
    Nil(ConstantNilListInstantiation),
    Tuple(ConstantTupleListInstantiation),
    ParameterList(ConstantParameterListListInstantiation),
    List(ConstantListListInstantiation),
    Function(ConstantFunctionListInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ConstantGenericListValue {
    parameter: TypeParameterId,
    kind: ConstantGenericListValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantGenericListValueKind {
    Empty,
    Reference(ConstantGenericListInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ConstantUtfCodepointListValue {
    kind: ConstantUtfCodepointListValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantUtfCodepointListValueKind {
    Empty,
    Reference(ConstantUtfCodepointListInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TypedConstantListValue<Element, Reference, Shape> {
    item_shape: Shape,
    kind: TypedConstantListValueKind<Element, Reference, Shape>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum TypedConstantListValueKind<Element, Reference, Shape> {
    Value(Box<[Element]>),
    Spread {
        elements: Vec1<Element>,
        tail: Box<TypedConstantListValue<Element, Reference, Shape>>,
    },
    Reference(Reference),
}

pub(super) enum ConstantListParts<Element, Tail> {
    Value(Vec<Element>),
    Spread { elements: Vec1<Element>, tail: Tail },
}

impl<Element, Tail> ConstantListParts<Element, Tail> {
    pub(super) fn try_from_parts(
        elements: Vec<Element>,
        tail: Option<Tail>,
    ) -> Result<Self, ConstantListConstructionError> {
        match tail {
            Some(tail) => Vec1::try_from_vec(elements)
                .map(|elements| Self::Spread { elements, tail })
                .map_err(|_| ConstantListConstructionError::SpreadWithoutElements),
            None => Ok(Self::Value(elements)),
        }
    }

    pub(super) fn try_map<MappedElement, MappedTail, Error>(
        self,
        map_element: impl FnMut(Element) -> Result<MappedElement, Error>,
        map_tail: impl FnOnce(Tail) -> Result<MappedTail, Error>,
    ) -> Result<ConstantListParts<MappedElement, MappedTail>, Error> {
        match self {
            Self::Value(elements) => elements
                .into_iter()
                .map(map_element)
                .collect::<Result<Vec<_>, _>>()
                .map(ConstantListParts::Value),
            Self::Spread { elements, tail } => {
                let elements = elements.try_mapped(map_element)?;
                let tail = map_tail(tail)?;
                Ok(ConstantListParts::Spread { elements, tail })
            }
        }
    }
}

pub(super) type ConstantIntListValue =
    TypedConstantListValue<ConstantIntValue, ConstantIntListInstantiation, ()>;
pub(super) type ConstantStringListValue =
    TypedConstantListValue<ConstantStringValue, ConstantStringListInstantiation, ()>;
pub(super) type ConstantBitArrayListValue =
    TypedConstantListValue<ConstantBitArrayValue, ConstantBitArrayListInstantiation, ()>;
pub(super) type ConstantCustomListValue =
    TypedConstantListValue<ConstantCustomValue, ConstantCustomListInstantiation, CustomValueShape>;
pub(super) type ConstantFloatListValue =
    TypedConstantListValue<ConstantFloatValue, ConstantFloatListInstantiation, ()>;
pub(super) type ConstantBoolListValue =
    TypedConstantListValue<ConstantBoolValue, ConstantBoolListInstantiation, ()>;
pub(super) type ConstantNilListValue =
    TypedConstantListValue<ConstantNilValue, ConstantNilListInstantiation, ()>;
pub(super) type ConstantTupleListValue =
    TypedConstantListValue<ConstantTupleValue, ConstantTupleListInstantiation, Box<[ValueShape]>>;
pub(super) type ConstantParameterListListValue = TypedConstantListValue<
    ConstantGenericListValue,
    ConstantParameterListListInstantiation,
    TypeParameterId,
>;
pub(super) type ConstantListListValue = TypedConstantListValue<
    ConstantStoredListValue,
    ConstantListListInstantiation,
    ValueStorageShape,
>;
pub(super) type ConstantFunctionListValue =
    TypedConstantListValue<ConstantFunctionValue, ConstantFunctionListInstantiation, FunctionShape>;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantListValue {
    Generic(ConstantGenericListValue),
    ParameterList(ConstantParameterListListValue),
    Int(ConstantIntListValue),
    String(ConstantStringListValue),
    BitArray(ConstantBitArrayListValue),
    UtfCodepoint(ConstantUtfCodepointListValue),
    Custom(ConstantCustomListValue),
    Float(ConstantFloatListValue),
    Bool(ConstantBoolListValue),
    Nil(ConstantNilListValue),
    Tuple(ConstantTupleListValue),
    List(ConstantListListValue),
    Function(ConstantFunctionListValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantStoredListValue {
    ParameterList(ConstantParameterListListValue),
    Int(ConstantIntListValue),
    String(ConstantStringListValue),
    BitArray(ConstantBitArrayListValue),
    UtfCodepoint(ConstantUtfCodepointListValue),
    Custom(ConstantCustomListValue),
    Float(ConstantFloatListValue),
    Bool(ConstantBoolListValue),
    Nil(ConstantNilListValue),
    Tuple(ConstantTupleListValue),
    List(ConstantListListValue),
    Function(ConstantFunctionListValue),
}

impl ConstantListInstantiation {
    pub(super) fn module(&self) -> crate::plan::ModuleId {
        match self {
            Self::Generic(value) => value.module(),
            Self::Int(value) => value.module(),
            Self::String(value) => value.module(),
            Self::BitArray(value) => value.module(),
            Self::UtfCodepoint(value) => value.module(),
            Self::Custom(value) => value.module(),
            Self::Float(value) => value.module(),
            Self::Bool(value) => value.module(),
            Self::Nil(value) => value.module(),
            Self::Tuple(value) => value.module(),
            Self::ParameterList(value) => value.module(),
            Self::List(value) => value.module(),
            Self::Function(value) => value.module(),
        }
    }

    pub(super) fn substitute(&self, outer: &TypeSubstitution) -> Self {
        match self {
            Self::Generic(value) => Self::from_generic_source(
                value.module(),
                *value.source(),
                value.substitution().substitute(outer),
                ValueShape::Parameter(*value.item_shape()).substitute(outer),
            ),
            Self::Int(value) => Self::Int(value.substitute_leaf(outer)),
            Self::String(value) => Self::String(value.substitute_leaf(outer)),
            Self::BitArray(value) => Self::BitArray(value.substitute_leaf(outer)),
            Self::UtfCodepoint(value) => Self::UtfCodepoint(value.substitute_leaf(outer)),
            Self::Custom(value) => Self::Custom(value.substitute_custom(outer)),
            Self::Float(value) => Self::Float(value.substitute_leaf(outer)),
            Self::Bool(value) => Self::Bool(value.substitute_leaf(outer)),
            Self::Nil(value) => Self::Nil(value.substitute_leaf(outer)),
            Self::Tuple(value) => Self::Tuple(value.substitute_tuple(outer)),
            Self::ParameterList(value) => {
                match ValueShape::Parameter(*value.item_shape())
                    .substitute(outer)
                    .representation()
                {
                    ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(value.retarget_parameter(outer, parameter))
                    }
                    ValueRepresentation::Stored(shape) => {
                        Self::List(value.retarget_stored(outer, shape))
                    }
                }
            }
            Self::List(value) => Self::List(value.substitute_list(outer)),
            Self::Function(value) => Self::Function(value.substitute_function(outer)),
        }
    }

    fn from_generic_source(
        module: crate::plan::ModuleId,
        template: ConstantGenericListTemplateId,
        substitution: TypeSubstitution,
        item_shape: ValueShape,
    ) -> Self {
        match item_shape {
            ValueShape::Parameter(parameter) => {
                Self::Generic(TypedConstantListInstantiation::in_module(
                    module,
                    template,
                    substitution,
                    parameter,
                ))
            }
            ValueShape::Int => Self::Int(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::String => Self::String(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::BitArray => Self::BitArray(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::UtfCodepoint => {
                Self::UtfCodepoint(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantListTemplateSource::Generic(template),
                    substitution,
                    (),
                ))
            }
            ValueShape::Custom(shape) => Self::Custom(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                shape,
            )),
            ValueShape::Float => Self::Float(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Bool => Self::Bool(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Nil => Self::Nil(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Tuple(shape) => Self::Tuple(TypedConstantListInstantiation::in_module(
                module,
                ConstantListTemplateSource::Generic(template),
                substitution,
                shape,
            )),
            ValueShape::List(shape) => match shape.representation() {
                ValueRepresentation::Uninhabited(parameter) => {
                    Self::ParameterList(TypedConstantListInstantiation::in_module(
                        module,
                        ConstantListTemplateSource::Generic(template),
                        substitution,
                        parameter,
                    ))
                }
                ValueRepresentation::Stored(shape) => {
                    Self::List(TypedConstantListInstantiation::in_module(
                        module,
                        ConstantNestedListTemplateSource::Generic(template),
                        substitution,
                        shape,
                    ))
                }
            },
            ValueShape::Function(shape) => {
                Self::Function(TypedConstantListInstantiation::in_module(
                    module,
                    ConstantListTemplateSource::Generic(template),
                    substitution,
                    *shape,
                ))
            }
        }
    }
}

impl<Source, Shape> TypedConstantListInstantiation<Source, Shape> {
    #[cfg(test)]
    fn new(source: Source, substitution: TypeSubstitution, item_shape: Shape) -> Self {
        Self::in_module(
            crate::plan::ModuleId::root(),
            source,
            substitution,
            item_shape,
        )
    }

    fn in_module(
        module: crate::plan::ModuleId,
        source: Source,
        substitution: TypeSubstitution,
        item_shape: Shape,
    ) -> Self {
        Self {
            module,
            source,
            substitution,
            item_shape,
        }
    }

    pub(crate) fn source(&self) -> &Source {
        &self.source
    }

    pub(crate) fn module(&self) -> crate::plan::ModuleId {
        self.module
    }

    pub(crate) fn substitution(&self) -> &TypeSubstitution {
        &self.substitution
    }

    pub(crate) fn item_shape(&self) -> &Shape {
        &self.item_shape
    }
}

impl TypedConstantListInstantiation<ConstantGenericListTemplateId, TypeParameterId> {
    pub(super) fn retarget<Id, Shape>(
        &self,
        outer: &TypeSubstitution,
        item_shape: Shape,
    ) -> TypedConstantListInstantiation<ConstantListTemplateSource<Id>, Shape> {
        TypedConstantListInstantiation::in_module(
            self.module,
            ConstantListTemplateSource::Generic(self.source),
            self.substitution.substitute(outer),
            item_shape,
        )
    }

    pub(super) fn retarget_generic(
        &self,
        outer: &TypeSubstitution,
        parameter: TypeParameterId,
    ) -> Self {
        TypedConstantListInstantiation::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            parameter,
        )
    }

    pub(super) fn retarget_nested(
        &self,
        outer: &TypeSubstitution,
        item_shape: ValueStorageShape,
    ) -> ConstantListListInstantiation {
        TypedConstantListInstantiation::in_module(
            self.module,
            ConstantNestedListTemplateSource::Generic(self.source),
            self.substitution.substitute(outer),
            item_shape,
        )
    }
}

impl<Id: Copy> TypedConstantListInstantiation<ConstantListTemplateSource<Id>, ()> {
    pub(super) fn substitute_leaf(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            (),
        )
    }
}

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantCustomListTemplateId>,
        CustomValueShape,
    >
{
    pub(super) fn substitute_custom(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            self.item_shape.substitute(outer),
        )
    }
}

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantTupleListTemplateId>,
        Box<[ValueShape]>,
    >
{
    pub(super) fn substitute_tuple(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            self.item_shape
                .iter()
                .map(|shape| shape.substitute(outer))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}

impl TypedConstantListInstantiation<ConstantNestedListTemplateSource, ValueStorageShape> {
    pub(super) fn substitute_list(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            self.item_shape.substitute(outer),
        )
    }
}

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantParameterListListTemplateId>,
        TypeParameterId,
    >
{
    pub(super) fn retarget_parameter(
        &self,
        outer: &TypeSubstitution,
        parameter: TypeParameterId,
    ) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            parameter,
        )
    }

    pub(super) fn retarget_stored(
        &self,
        outer: &TypeSubstitution,
        shape: ValueStorageShape,
    ) -> ConstantListListInstantiation {
        let source = match self.source {
            ConstantListTemplateSource::Generic(source) => {
                ConstantNestedListTemplateSource::Generic(source)
            }
            ConstantListTemplateSource::Exact(source) => {
                ConstantNestedListTemplateSource::ParameterList(source)
            }
        };
        TypedConstantListInstantiation::in_module(
            self.module,
            source,
            self.substitution.substitute(outer),
            shape,
        )
    }
}

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantFunctionListTemplateId>,
        FunctionShape,
    >
{
    pub(super) fn substitute_function(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            self.item_shape.substitute(outer),
        )
    }
}

impl ConstantListValue {
    pub(super) fn generic(parameter: TypeParameterId) -> Self {
        Self::Generic(ConstantGenericListValue::empty(parameter))
    }

    pub(super) fn int(parts: ConstantListParts<ConstantIntValue, ConstantIntListValue>) -> Self {
        Self::Int(TypedConstantListValue::value((), parts))
    }

    pub(super) fn parameter_list(
        parameter: TypeParameterId,
        parts: ConstantListParts<ConstantGenericListValue, ConstantParameterListListValue>,
    ) -> Self {
        Self::ParameterList(TypedConstantListValue::value(parameter, parts))
    }

    pub(super) fn string(
        parts: ConstantListParts<ConstantStringValue, ConstantStringListValue>,
    ) -> Self {
        Self::String(TypedConstantListValue::value((), parts))
    }

    pub(super) fn bit_array(
        parts: ConstantListParts<ConstantBitArrayValue, ConstantBitArrayListValue>,
    ) -> Self {
        Self::BitArray(TypedConstantListValue::value((), parts))
    }

    pub(super) fn utf_codepoint() -> Self {
        Self::UtfCodepoint(ConstantUtfCodepointListValue::empty())
    }

    pub(super) fn custom(
        item_shape: CustomValueShape,
        parts: ConstantListParts<ConstantCustomValue, ConstantCustomListValue>,
    ) -> Self {
        Self::Custom(TypedConstantListValue::value(item_shape, parts))
    }

    pub(super) fn float(
        parts: ConstantListParts<ConstantFloatValue, ConstantFloatListValue>,
    ) -> Self {
        Self::Float(TypedConstantListValue::value((), parts))
    }

    pub(super) fn bool(parts: ConstantListParts<ConstantBoolValue, ConstantBoolListValue>) -> Self {
        Self::Bool(TypedConstantListValue::value((), parts))
    }

    pub(super) fn nil(parts: ConstantListParts<ConstantNilValue, ConstantNilListValue>) -> Self {
        Self::Nil(TypedConstantListValue::value((), parts))
    }

    pub(super) fn tuple(
        item_shape: Box<[ValueShape]>,
        parts: ConstantListParts<ConstantTupleValue, ConstantTupleListValue>,
    ) -> Self {
        Self::Tuple(TypedConstantListValue::value(item_shape, parts))
    }

    pub(super) fn list(
        item_shape: ValueStorageShape,
        parts: ConstantListParts<ConstantStoredListValue, ConstantListListValue>,
    ) -> Self {
        Self::List(TypedConstantListValue::value(item_shape, parts))
    }

    pub(super) fn function(
        item_shape: FunctionShape,
        parts: ConstantListParts<ConstantFunctionValue, ConstantFunctionListValue>,
    ) -> Self {
        Self::Function(TypedConstantListValue::value(item_shape, parts))
    }

    pub(super) fn reference(instantiation: ConstantListInstantiation) -> Self {
        match instantiation {
            ConstantListInstantiation::Generic(value) => {
                Self::Generic(ConstantGenericListValue::reference(value))
            }
            ConstantListInstantiation::ParameterList(value) => Self::ParameterList(
                TypedConstantListValue::reference(*value.item_shape(), value),
            ),
            ConstantListInstantiation::Int(value) => {
                Self::Int(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::String(value) => {
                Self::String(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::BitArray(value) => {
                Self::BitArray(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::UtfCodepoint(value) => {
                Self::UtfCodepoint(ConstantUtfCodepointListValue::reference(value))
            }
            ConstantListInstantiation::Custom(value) => Self::Custom(
                TypedConstantListValue::reference(value.item_shape().clone(), value),
            ),
            ConstantListInstantiation::Float(value) => {
                Self::Float(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::Bool(value) => {
                Self::Bool(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::Nil(value) => {
                Self::Nil(TypedConstantListValue::reference((), value))
            }
            ConstantListInstantiation::Tuple(value) => Self::Tuple(
                TypedConstantListValue::reference(value.item_shape().clone(), value),
            ),
            ConstantListInstantiation::List(value) => Self::List(
                TypedConstantListValue::reference(value.item_shape().clone(), value),
            ),
            ConstantListInstantiation::Function(value) => Self::Function(
                TypedConstantListValue::reference(value.item_shape().clone(), value),
            ),
        }
    }

    pub(super) fn item_shape(&self) -> ValueShape {
        match self {
            Self::Generic(value) => ValueShape::Parameter(value.parameter()),
            Self::ParameterList(value) => {
                ValueShape::List(Box::new(ValueShape::Parameter(*value.item_shape())))
            }
            Self::Int(_) => ValueShape::Int,
            Self::String(_) => ValueShape::String,
            Self::BitArray(_) => ValueShape::BitArray,
            Self::UtfCodepoint(_) => ValueShape::UtfCodepoint,
            Self::Custom(value) => ValueShape::Custom(value.item_shape().clone()),
            Self::Float(_) => ValueShape::Float,
            Self::Bool(_) => ValueShape::Bool,
            Self::Nil(_) => ValueShape::Nil,
            Self::Tuple(value) => ValueShape::Tuple(value.item_shape().clone()),
            Self::List(value) => ValueShape::List(Box::new(value.item_shape().to_value_shape())),
            Self::Function(value) => ValueShape::Function(Box::new(value.item_shape().clone())),
        }
    }

    pub(super) fn shape(&self) -> ValueShape {
        ValueShape::List(Box::new(self.item_shape()))
    }

    pub(super) fn into_int(self) -> Option<ConstantIntListValue> {
        match self {
            Self::Int(value) => Some(value),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(super) fn into_generic(self) -> Option<ConstantGenericListValue> {
        match self {
            Self::Generic(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn into_string(self) -> Option<ConstantStringListValue> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn into_bit_array(self) -> Option<ConstantBitArrayListValue> {
        match self {
            Self::BitArray(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn into_float(self) -> Option<ConstantFloatListValue> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn into_bool(self) -> Option<ConstantBoolListValue> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn into_nil(self) -> Option<ConstantNilListValue> {
        match self {
            Self::Nil(value) => Some(value),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(super) fn into_stored(self) -> Result<ConstantStoredListValue, ConstantGenericListValue> {
        match self {
            Self::Generic(value) => Err(value),
            Self::ParameterList(value) => Ok(ConstantStoredListValue::ParameterList(value)),
            Self::Int(value) => Ok(ConstantStoredListValue::Int(value)),
            Self::String(value) => Ok(ConstantStoredListValue::String(value)),
            Self::BitArray(value) => Ok(ConstantStoredListValue::BitArray(value)),
            Self::UtfCodepoint(value) => Ok(ConstantStoredListValue::UtfCodepoint(value)),
            Self::Custom(value) => Ok(ConstantStoredListValue::Custom(value)),
            Self::Float(value) => Ok(ConstantStoredListValue::Float(value)),
            Self::Bool(value) => Ok(ConstantStoredListValue::Bool(value)),
            Self::Nil(value) => Ok(ConstantStoredListValue::Nil(value)),
            Self::Tuple(value) => Ok(ConstantStoredListValue::Tuple(value)),
            Self::List(value) => Ok(ConstantStoredListValue::List(value)),
            Self::Function(value) => Ok(ConstantStoredListValue::Function(value)),
        }
    }
}

impl<Element, Reference, Shape> TypedConstantListValue<Element, Reference, Shape> {
    fn value(item_shape: Shape, parts: ConstantListParts<Element, Self>) -> Self {
        let kind = match parts {
            ConstantListParts::Spread { elements, tail } => TypedConstantListValueKind::Spread {
                elements,
                tail: Box::new(tail),
            },
            ConstantListParts::Value(elements) => {
                TypedConstantListValueKind::Value(elements.into_boxed_slice())
            }
        };
        Self { item_shape, kind }
    }

    fn reference(item_shape: Shape, reference: Reference) -> Self {
        Self {
            item_shape,
            kind: TypedConstantListValueKind::Reference(reference),
        }
    }

    pub(super) fn item_shape(&self) -> &Shape {
        &self.item_shape
    }

    pub(super) fn kind(&self) -> &TypedConstantListValueKind<Element, Reference, Shape> {
        &self.kind
    }
}

impl ConstantGenericListValue {
    fn empty(parameter: TypeParameterId) -> Self {
        Self {
            parameter,
            kind: ConstantGenericListValueKind::Empty,
        }
    }

    fn reference(reference: ConstantGenericListInstantiation) -> Self {
        Self {
            parameter: *reference.item_shape(),
            kind: ConstantGenericListValueKind::Reference(reference),
        }
    }

    pub(super) fn parameter(&self) -> TypeParameterId {
        self.parameter
    }

    pub(super) fn kind(&self) -> &ConstantGenericListValueKind {
        &self.kind
    }
}

impl ConstantUtfCodepointListValue {
    fn empty() -> Self {
        Self {
            kind: ConstantUtfCodepointListValueKind::Empty,
        }
    }

    fn reference(reference: ConstantUtfCodepointListInstantiation) -> Self {
        Self {
            kind: ConstantUtfCodepointListValueKind::Reference(reference),
        }
    }

    pub(super) fn kind(&self) -> &ConstantUtfCodepointListValueKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConstantGenericListTemplateId, ConstantListParts, ConstantListTemplateSource,
        ConstantListValue, ConstantNestedListTemplateSource, ConstantParameterListListTemplateId,
        ConstantStoredListValue, TypedConstantListInstantiation,
    };
    use crate::plan::{
        CustomConstructorRefinement, CustomTypeName, CustomValueShape, FunctionShape,
        TypeParameterId, TypeSubstitution, ValueShape, ValueStorageShape,
    };

    #[test]
    fn spread_mapping_propagates_element_and_tail_failures() {
        let spread = ConstantListParts::Spread {
            elements: vec1::vec1![1, 2],
            tail: 3,
        };
        let mapped = spread
            .try_map(
                |element| Ok::<_, &'static str>(element * 2),
                |tail| Ok(tail + 1),
            )
            .expect("exact spread mapping should succeed");
        let summaries = [mapped, ConstantListParts::Value(vec![9])]
            .into_iter()
            .map(|parts| match parts {
                ConstantListParts::Spread { elements, tail } => {
                    (elements.into_iter().collect::<Vec<_>>(), Some(tail))
                }
                ConstantListParts::Value(elements) => (elements, None),
            })
            .collect::<Vec<_>>();
        assert_eq!(summaries, vec![(vec![2, 4], Some(4)), (vec![9], None)]);

        let element_error = ConstantListParts::Spread {
            elements: vec1::vec1![1, 2],
            tail: 3,
        }
        .try_map(
            |element| {
                if element == 2 {
                    Err("element")
                } else {
                    Ok(element)
                }
            },
            Ok,
        )
        .err();
        assert_eq!(element_error, Some("element"));

        let tail_error = ConstantListParts::Spread {
            elements: vec1::vec1![1, 2],
            tail: 3,
        }
        .try_map(Ok, |_| Err::<i32, _>("tail"))
        .err();
        assert_eq!(tail_error, Some("tail"));
    }

    #[test]
    fn stored_conversion_preserves_every_list_family() {
        let parameter = TypeParameterId(0);
        let custom = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
            CustomConstructorRefinement::Any,
        );
        let function = FunctionShape::new(Vec::new(), ValueShape::Int);
        let cases = vec![
            (
                ConstantListValue::generic(parameter),
                ValueShape::Parameter(parameter),
            ),
            (
                ConstantListValue::parameter_list(parameter, ConstantListParts::Value(Vec::new())),
                ValueShape::List(Box::new(ValueShape::Parameter(parameter))),
            ),
            (
                ConstantListValue::int(ConstantListParts::Value(Vec::new())),
                ValueShape::Int,
            ),
            (
                ConstantListValue::string(ConstantListParts::Value(Vec::new())),
                ValueShape::String,
            ),
            (
                ConstantListValue::bit_array(ConstantListParts::Value(Vec::new())),
                ValueShape::BitArray,
            ),
            (ConstantListValue::utf_codepoint(), ValueShape::UtfCodepoint),
            (
                ConstantListValue::custom(custom.clone(), ConstantListParts::Value(Vec::new())),
                ValueShape::Custom(custom),
            ),
            (
                ConstantListValue::float(ConstantListParts::Value(Vec::new())),
                ValueShape::Float,
            ),
            (
                ConstantListValue::bool(ConstantListParts::Value(Vec::new())),
                ValueShape::Bool,
            ),
            (
                ConstantListValue::nil(ConstantListParts::Value(Vec::new())),
                ValueShape::Nil,
            ),
            (
                ConstantListValue::tuple(
                    vec![ValueShape::Int].into_boxed_slice(),
                    ConstantListParts::Value(Vec::new()),
                ),
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
            ),
            (
                ConstantListValue::list(
                    ValueStorageShape::Int,
                    ConstantListParts::Value(Vec::new()),
                ),
                ValueShape::List(Box::new(ValueShape::Int)),
            ),
            (
                ConstantListValue::function(function.clone(), ConstantListParts::Value(Vec::new())),
                ValueShape::Function(Box::new(function)),
            ),
        ];

        for (value, expected) in cases {
            let actual = match value.into_stored() {
                Err(value) => ValueShape::Parameter(value.parameter()),
                Ok(value) => match value {
                    ConstantStoredListValue::ParameterList(value) => {
                        ValueShape::List(Box::new(ValueShape::Parameter(*value.item_shape())))
                    }
                    ConstantStoredListValue::Int(_) => ValueShape::Int,
                    ConstantStoredListValue::String(_) => ValueShape::String,
                    ConstantStoredListValue::BitArray(_) => ValueShape::BitArray,
                    ConstantStoredListValue::UtfCodepoint(_) => ValueShape::UtfCodepoint,
                    ConstantStoredListValue::Custom(value) => {
                        ValueShape::Custom(value.item_shape().clone())
                    }
                    ConstantStoredListValue::Float(_) => ValueShape::Float,
                    ConstantStoredListValue::Bool(_) => ValueShape::Bool,
                    ConstantStoredListValue::Nil(_) => ValueShape::Nil,
                    ConstantStoredListValue::Tuple(value) => {
                        ValueShape::Tuple(value.item_shape().clone())
                    }
                    ConstantStoredListValue::List(value) => {
                        ValueShape::List(Box::new(value.item_shape().to_value_shape()))
                    }
                    ConstantStoredListValue::Function(value) => {
                        ValueShape::Function(Box::new(value.item_shape().clone()))
                    }
                },
            };
            assert_eq!(actual, expected);
        }

        assert_eq!(
            ConstantListValue::int(ConstantListParts::Value(Vec::new())).into_generic(),
            None,
        );
    }

    #[test]
    fn parameter_list_retarget_preserves_generic_and_exact_sources() {
        let parameter = TypeParameterId(0);
        let substitution = TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]);
        let outer = TypeSubstitution::from_arguments(vec![ValueShape::Int]);
        let generic = ConstantGenericListTemplateId(2);
        let exact = ConstantParameterListListTemplateId(3);
        let generic_value = TypedConstantListInstantiation::new(
            ConstantListTemplateSource::Generic(generic),
            substitution.clone(),
            parameter,
        );
        let exact_value = TypedConstantListInstantiation::new(
            ConstantListTemplateSource::Exact(exact),
            substitution,
            parameter,
        );

        assert_eq!(
            generic_value
                .retarget_stored(&outer, ValueStorageShape::Int)
                .source(),
            &ConstantNestedListTemplateSource::Generic(generic),
        );
        assert_eq!(
            exact_value
                .retarget_stored(&outer, ValueStorageShape::Int)
                .source(),
            &ConstantNestedListTemplateSource::ParameterList(exact),
        );
    }
}
