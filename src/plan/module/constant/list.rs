use super::{
    ConstantBitArrayValue, ConstantBoolValue, ConstantCustomValue, ConstantFloatValue,
    ConstantFunctionValue, ConstantIntValue, ConstantNilValue, ConstantStringValue,
    ConstantTupleValue,
};
use crate::plan::{CustomValueShape, FunctionShape, TypeParameterId, TypeSubstitution, ValueShape};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantGenericListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantIntListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantStringListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantBitArrayListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantUtfCodepointListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantCustomListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantFloatListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantBoolListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantNilListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantTupleListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantListListTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConstantFunctionListTemplateId(pub(super) usize);

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
    List {
        id: ConstantListListTemplateId,
        shape: Box<ValueShape>,
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
            ValueShape::List(shape) => Self::List {
                id: ConstantListListTemplateId(index),
                shape,
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
            Self::List { shape, .. } => ValueShape::List(shape.clone()),
            Self::Function { shape, .. } => ValueShape::Function(Box::new(shape.clone())),
        }
    }

    pub(super) fn instantiate(&self, substitution: TypeSubstitution) -> ConstantListInstantiation {
        match self {
            Self::Generic { id, parameter } => ConstantListInstantiation::from_generic_source(
                *id,
                substitution.clone(),
                ValueShape::Parameter(*parameter).substitute(&substitution),
            ),
            Self::Int(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Int,
            ),
            Self::String(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::String,
            ),
            Self::BitArray(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::BitArray,
            ),
            Self::UtfCodepoint(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::UtfCodepoint,
            ),
            Self::Custom { id, shape } => {
                ConstantListInstantiation::Custom(TypedConstantListInstantiation::new(
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape.substitute(&substitution),
                ))
            }
            Self::Float(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Float,
            ),
            Self::Bool(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Bool,
            ),
            Self::Nil(id) => Self::leaf_instantiation(
                ConstantListTemplateSource::Exact(*id),
                substitution,
                ConstantListInstantiation::Nil,
            ),
            Self::Tuple { id, shape } => {
                ConstantListInstantiation::Tuple(TypedConstantListInstantiation::new(
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape
                        .iter()
                        .map(|shape| shape.substitute(&substitution))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ))
            }
            Self::List { id, shape } => {
                ConstantListInstantiation::List(TypedConstantListInstantiation::new(
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    Box::new(shape.substitute(&substitution)),
                ))
            }
            Self::Function { id, shape } => {
                ConstantListInstantiation::Function(TypedConstantListInstantiation::new(
                    ConstantListTemplateSource::Exact(*id),
                    substitution.clone(),
                    shape.substitute(&substitution),
                ))
            }
        }
    }

    fn leaf_instantiation<Id>(
        source: ConstantListTemplateSource<Id>,
        substitution: TypeSubstitution,
        into: fn(
            TypedConstantListInstantiation<ConstantListTemplateSource<Id>, ()>,
        ) -> ConstantListInstantiation,
    ) -> ConstantListInstantiation {
        into(TypedConstantListInstantiation::new(
            source,
            substitution,
            (),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum ConstantListTemplateSource<Id> {
    Generic(ConstantGenericListTemplateId),
    Exact(Id),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TypedConstantListInstantiation<Source, Shape> {
    source: Source,
    substitution: TypeSubstitution,
    item_shape: Shape,
}

pub(super) type ConstantGenericListInstantiation =
    TypedConstantListInstantiation<ConstantGenericListTemplateId, TypeParameterId>;
pub(super) type ConstantIntListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantIntListTemplateId>, ()>;
pub(super) type ConstantStringListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantStringListTemplateId>, ()>;
pub(super) type ConstantBitArrayListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantBitArrayListTemplateId>, ()>;
pub(super) type ConstantUtfCodepointListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantUtfCodepointListTemplateId>,
    (),
>;
pub(super) type ConstantCustomListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantCustomListTemplateId>,
    CustomValueShape,
>;
pub(super) type ConstantFloatListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantFloatListTemplateId>, ()>;
pub(super) type ConstantBoolListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantBoolListTemplateId>, ()>;
pub(super) type ConstantNilListInstantiation =
    TypedConstantListInstantiation<ConstantListTemplateSource<ConstantNilListTemplateId>, ()>;
pub(super) type ConstantTupleListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantTupleListTemplateId>,
    Box<[ValueShape]>,
>;
pub(super) type ConstantListListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantListListTemplateId>,
    Box<ValueShape>,
>;
pub(super) type ConstantFunctionListInstantiation = TypedConstantListInstantiation<
    ConstantListTemplateSource<ConstantFunctionListTemplateId>,
    FunctionShape,
>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum ConstantListInstantiation {
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
    Value {
        elements: Box<[Element]>,
        tail: Option<Box<TypedConstantListValue<Element, Reference, Shape>>>,
    },
    Reference(Reference),
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
pub(super) type ConstantListListValue =
    TypedConstantListValue<ConstantListValue, ConstantListListInstantiation, Box<ValueShape>>;
pub(super) type ConstantFunctionListValue =
    TypedConstantListValue<ConstantFunctionValue, ConstantFunctionListInstantiation, FunctionShape>;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantListValue {
    Generic(ConstantGenericListValue),
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
    fn from_generic_source(
        template: ConstantGenericListTemplateId,
        substitution: TypeSubstitution,
        item_shape: ValueShape,
    ) -> Self {
        match item_shape {
            ValueShape::Parameter(parameter) => Self::Generic(TypedConstantListInstantiation::new(
                template,
                substitution,
                parameter,
            )),
            ValueShape::Int => Self::Int(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::String => Self::String(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::BitArray => Self::BitArray(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::UtfCodepoint => Self::UtfCodepoint(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Custom(shape) => Self::Custom(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                shape,
            )),
            ValueShape::Float => Self::Float(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Bool => Self::Bool(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Nil => Self::Nil(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                (),
            )),
            ValueShape::Tuple(shape) => Self::Tuple(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                shape,
            )),
            ValueShape::List(shape) => Self::List(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                shape,
            )),
            ValueShape::Function(shape) => Self::Function(TypedConstantListInstantiation::new(
                ConstantListTemplateSource::Generic(template),
                substitution,
                *shape,
            )),
        }
    }
}

impl<Source, Shape> TypedConstantListInstantiation<Source, Shape> {
    fn new(source: Source, substitution: TypeSubstitution, item_shape: Shape) -> Self {
        Self {
            source,
            substitution,
            item_shape,
        }
    }

    pub(super) fn source(&self) -> &Source {
        &self.source
    }

    pub(super) fn substitution(&self) -> &TypeSubstitution {
        &self.substitution
    }

    pub(super) fn item_shape(&self) -> &Shape {
        &self.item_shape
    }
}

impl<Id: Copy> TypedConstantListInstantiation<ConstantListTemplateSource<Id>, ()> {
    pub(super) fn substitute_leaf(&self, outer: &TypeSubstitution) -> Self {
        Self::new(self.source, self.substitution.substitute(outer), ())
    }
}

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantCustomListTemplateId>,
        CustomValueShape,
    >
{
    pub(super) fn substitute_custom(&self, outer: &TypeSubstitution) -> Self {
        Self::new(
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
        Self::new(
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

impl
    TypedConstantListInstantiation<
        ConstantListTemplateSource<ConstantListListTemplateId>,
        Box<ValueShape>,
    >
{
    pub(super) fn substitute_list(&self, outer: &TypeSubstitution) -> Self {
        Self::new(
            self.source,
            self.substitution.substitute(outer),
            Box::new(self.item_shape.substitute(outer)),
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
        Self::new(
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

    pub(super) fn int(elements: Vec<ConstantIntValue>, tail: Option<ConstantIntListValue>) -> Self {
        Self::Int(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn string(
        elements: Vec<ConstantStringValue>,
        tail: Option<ConstantStringListValue>,
    ) -> Self {
        Self::String(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn bit_array(
        elements: Vec<ConstantBitArrayValue>,
        tail: Option<ConstantBitArrayListValue>,
    ) -> Self {
        Self::BitArray(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn utf_codepoint() -> Self {
        Self::UtfCodepoint(ConstantUtfCodepointListValue::empty())
    }

    pub(super) fn custom(
        item_shape: CustomValueShape,
        elements: Vec<ConstantCustomValue>,
        tail: Option<ConstantCustomListValue>,
    ) -> Self {
        Self::Custom(TypedConstantListValue::value(item_shape, elements, tail))
    }

    pub(super) fn float(
        elements: Vec<ConstantFloatValue>,
        tail: Option<ConstantFloatListValue>,
    ) -> Self {
        Self::Float(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn bool(
        elements: Vec<ConstantBoolValue>,
        tail: Option<ConstantBoolListValue>,
    ) -> Self {
        Self::Bool(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn nil(elements: Vec<ConstantNilValue>, tail: Option<ConstantNilListValue>) -> Self {
        Self::Nil(TypedConstantListValue::value((), elements, tail))
    }

    pub(super) fn tuple(
        item_shape: Box<[ValueShape]>,
        elements: Vec<ConstantTupleValue>,
        tail: Option<ConstantTupleListValue>,
    ) -> Self {
        Self::Tuple(TypedConstantListValue::value(item_shape, elements, tail))
    }

    pub(super) fn list(
        item_shape: ValueShape,
        elements: Vec<ConstantListValue>,
        tail: Option<ConstantListListValue>,
    ) -> Self {
        Self::List(TypedConstantListValue::value(
            Box::new(item_shape),
            elements,
            tail,
        ))
    }

    pub(super) fn function(
        item_shape: FunctionShape,
        elements: Vec<ConstantFunctionValue>,
        tail: Option<ConstantFunctionListValue>,
    ) -> Self {
        Self::Function(TypedConstantListValue::value(item_shape, elements, tail))
    }

    pub(super) fn reference(instantiation: ConstantListInstantiation) -> Self {
        match instantiation {
            ConstantListInstantiation::Generic(value) => {
                Self::Generic(ConstantGenericListValue::reference(value))
            }
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
            Self::Int(_) => ValueShape::Int,
            Self::String(_) => ValueShape::String,
            Self::BitArray(_) => ValueShape::BitArray,
            Self::UtfCodepoint(_) => ValueShape::UtfCodepoint,
            Self::Custom(value) => ValueShape::Custom(value.item_shape().clone()),
            Self::Float(_) => ValueShape::Float,
            Self::Bool(_) => ValueShape::Bool,
            Self::Nil(_) => ValueShape::Nil,
            Self::Tuple(value) => ValueShape::Tuple(value.item_shape().clone()),
            Self::List(value) => ValueShape::List(value.item_shape().clone()),
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
}

impl<Element, Reference, Shape> TypedConstantListValue<Element, Reference, Shape> {
    fn value(item_shape: Shape, elements: Vec<Element>, tail: Option<Self>) -> Self {
        Self {
            item_shape,
            kind: TypedConstantListValueKind::Value {
                elements: elements.into_boxed_slice(),
                tail: tail.map(Box::new),
            },
        }
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
