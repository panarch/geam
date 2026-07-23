use crate::plan::{
    CustomConstructor, CustomValueShape, FunctionReference, FunctionShape, TypeParameterId,
    TypeSubstitution, ValueShape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantGenericFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantIntFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantStringFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBitArrayFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantUtfCodepointFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantCustomFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFloatFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBoolFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantNilFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantTupleFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantListFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFunctionFunctionTemplateId(pub(super) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ConstantFunctionTemplate {
    Generic(ConstantGenericFunctionTemplateId),
    Int(ConstantIntFunctionTemplateId),
    String(ConstantStringFunctionTemplateId),
    BitArray(ConstantBitArrayFunctionTemplateId),
    UtfCodepoint(ConstantUtfCodepointFunctionTemplateId),
    Custom {
        template: ConstantCustomFunctionTemplateId,
        return_: CustomValueShape,
    },
    Float(ConstantFloatFunctionTemplateId),
    Bool(ConstantBoolFunctionTemplateId),
    Nil(ConstantNilFunctionTemplateId),
    Tuple {
        template: ConstantTupleFunctionTemplateId,
        return_: Box<[ValueShape]>,
    },
    List {
        template: ConstantListFunctionTemplateId,
        item: Box<ValueShape>,
    },
    Function {
        template: ConstantFunctionFunctionTemplateId,
        return_: Box<FunctionShape>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConstantFunctionTemplateSource<Id> {
    Generic(ConstantGenericFunctionTemplateId),
    Exact(Id),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TypedConstantFunctionInstantiation<Source, Return> {
    module: crate::plan::ModuleId,
    source: Source,
    substitution: TypeSubstitution,
    shape: FunctionShape,
    return_: Return,
}

pub(crate) type ConstantGenericFunctionInstantiation =
    TypedConstantFunctionInstantiation<ConstantGenericFunctionTemplateId, TypeParameterId>;
pub(crate) type ConstantIntFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantIntFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantStringFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantStringFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantBitArrayFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantBitArrayFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantUtfCodepointFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantUtfCodepointFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantCustomFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantCustomFunctionTemplateId>,
    CustomValueShape,
>;
pub(crate) type ConstantFloatFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantFloatFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantBoolFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantBoolFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantNilFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantNilFunctionTemplateId>,
    (),
>;
pub(crate) type ConstantTupleFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantTupleFunctionTemplateId>,
    Box<[ValueShape]>,
>;
pub(crate) type ConstantListFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantListFunctionTemplateId>,
    Box<ValueShape>,
>;
pub(crate) type ConstantFunctionFunctionInstantiation = TypedConstantFunctionInstantiation<
    ConstantFunctionTemplateSource<ConstantFunctionFunctionTemplateId>,
    Box<FunctionShape>,
>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ConstantFunctionInstantiation {
    Generic(ConstantGenericFunctionInstantiation),
    Int(ConstantIntFunctionInstantiation),
    String(ConstantStringFunctionInstantiation),
    BitArray(ConstantBitArrayFunctionInstantiation),
    UtfCodepoint(ConstantUtfCodepointFunctionInstantiation),
    Custom(ConstantCustomFunctionInstantiation),
    Float(ConstantFloatFunctionInstantiation),
    Bool(ConstantBoolFunctionInstantiation),
    Nil(ConstantNilFunctionInstantiation),
    Tuple(ConstantTupleFunctionInstantiation),
    List(ConstantListFunctionInstantiation),
    Function(ConstantFunctionFunctionInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TypedConstantFunctionValue<Target, Reference, Return> {
    shape: FunctionShape,
    return_: Return,
    kind: TypedConstantFunctionValueKind<Target, Reference>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum TypedConstantFunctionValueKind<Target, Reference> {
    Target(Target),
    Reference(Reference),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantCustomFunctionTarget {
    Reference(FunctionReference),
    Constructor(CustomConstructor),
}

pub(super) type ConstantGenericFunctionValue = TypedConstantFunctionValue<
    FunctionReference,
    ConstantGenericFunctionInstantiation,
    TypeParameterId,
>;
pub(super) type ConstantIntFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantIntFunctionInstantiation, ()>;
pub(super) type ConstantStringFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantStringFunctionInstantiation, ()>;
pub(super) type ConstantBitArrayFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantBitArrayFunctionInstantiation, ()>;
pub(super) type ConstantUtfCodepointFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantUtfCodepointFunctionInstantiation, ()>;
pub(super) type ConstantCustomFunctionValue = TypedConstantFunctionValue<
    ConstantCustomFunctionTarget,
    ConstantCustomFunctionInstantiation,
    CustomValueShape,
>;
pub(super) type ConstantFloatFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantFloatFunctionInstantiation, ()>;
pub(super) type ConstantBoolFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantBoolFunctionInstantiation, ()>;
pub(super) type ConstantNilFunctionValue =
    TypedConstantFunctionValue<FunctionReference, ConstantNilFunctionInstantiation, ()>;
pub(super) type ConstantTupleFunctionValue = TypedConstantFunctionValue<
    FunctionReference,
    ConstantTupleFunctionInstantiation,
    Box<[ValueShape]>,
>;
pub(super) type ConstantListFunctionValue = TypedConstantFunctionValue<
    FunctionReference,
    ConstantListFunctionInstantiation,
    Box<ValueShape>,
>;
pub(super) type ConstantFunctionFunctionValue = TypedConstantFunctionValue<
    FunctionReference,
    ConstantFunctionFunctionInstantiation,
    Box<FunctionShape>,
>;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantFunctionValue {
    Generic(ConstantGenericFunctionValue),
    Int(ConstantIntFunctionValue),
    String(ConstantStringFunctionValue),
    BitArray(ConstantBitArrayFunctionValue),
    UtfCodepoint(ConstantUtfCodepointFunctionValue),
    Custom(ConstantCustomFunctionValue),
    Float(ConstantFloatFunctionValue),
    Bool(ConstantBoolFunctionValue),
    Nil(ConstantNilFunctionValue),
    Tuple(ConstantTupleFunctionValue),
    List(ConstantListFunctionValue),
    Function(ConstantFunctionFunctionValue),
}

impl ConstantFunctionTemplate {
    pub(super) fn from_shape(shape: &FunctionShape, index: usize) -> Self {
        match shape.return_shape() {
            ValueShape::Parameter(_) => Self::Generic(ConstantGenericFunctionTemplateId(index)),
            ValueShape::Int => Self::Int(ConstantIntFunctionTemplateId(index)),
            ValueShape::String => Self::String(ConstantStringFunctionTemplateId(index)),
            ValueShape::BitArray => Self::BitArray(ConstantBitArrayFunctionTemplateId(index)),
            ValueShape::UtfCodepoint => {
                Self::UtfCodepoint(ConstantUtfCodepointFunctionTemplateId(index))
            }
            ValueShape::Custom(return_) => Self::Custom {
                template: ConstantCustomFunctionTemplateId(index),
                return_: return_.clone(),
            },
            ValueShape::Float => Self::Float(ConstantFloatFunctionTemplateId(index)),
            ValueShape::Bool => Self::Bool(ConstantBoolFunctionTemplateId(index)),
            ValueShape::Nil => Self::Nil(ConstantNilFunctionTemplateId(index)),
            ValueShape::Tuple(return_) => Self::Tuple {
                template: ConstantTupleFunctionTemplateId(index),
                return_: return_.clone(),
            },
            ValueShape::List(item) => Self::List {
                template: ConstantListFunctionTemplateId(index),
                item: item.clone(),
            },
            ValueShape::Function(return_) => Self::Function {
                template: ConstantFunctionFunctionTemplateId(index),
                return_: return_.clone(),
            },
        }
    }

    pub(super) fn instantiate(
        &self,
        module: crate::plan::ModuleId,
        substitution: TypeSubstitution,
        shape: FunctionShape,
    ) -> ConstantFunctionInstantiation {
        match self {
            Self::Generic(source) => ConstantFunctionInstantiation::from_generic_source(
                module,
                *source,
                substitution,
                shape,
            ),
            Self::Int(source) => {
                ConstantFunctionInstantiation::Int(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ))
            }
            Self::String(source) => ConstantFunctionInstantiation::String(
                TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ),
            ),
            Self::BitArray(source) => ConstantFunctionInstantiation::BitArray(
                TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ),
            ),
            Self::UtfCodepoint(source) => ConstantFunctionInstantiation::UtfCodepoint(
                TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ),
            ),
            Self::Custom { template, return_ } => ConstantFunctionInstantiation::Custom(
                TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*template),
                    substitution.clone(),
                    shape,
                    return_.substitute(&substitution),
                ),
            ),
            Self::Float(source) => {
                ConstantFunctionInstantiation::Float(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ))
            }
            Self::Bool(source) => {
                ConstantFunctionInstantiation::Bool(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ))
            }
            Self::Nil(source) => {
                ConstantFunctionInstantiation::Nil(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*source),
                    substitution,
                    shape,
                    (),
                ))
            }
            Self::Tuple { template, return_ } => {
                ConstantFunctionInstantiation::Tuple(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*template),
                    substitution.clone(),
                    shape,
                    return_
                        .iter()
                        .map(|shape| shape.substitute(&substitution))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ))
            }
            Self::List { template, item } => {
                ConstantFunctionInstantiation::List(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*template),
                    substitution.clone(),
                    shape,
                    Box::new(item.as_ref().substitute(&substitution)),
                ))
            }
            Self::Function { template, return_ } => ConstantFunctionInstantiation::Function(
                TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Exact(*template),
                    substitution.clone(),
                    shape,
                    Box::new(return_.as_ref().substitute(&substitution)),
                ),
            ),
        }
    }
}

impl<Source, Return> TypedConstantFunctionInstantiation<Source, Return> {
    #[cfg(test)]
    fn new(
        source: Source,
        substitution: TypeSubstitution,
        shape: FunctionShape,
        return_: Return,
    ) -> Self {
        Self::in_module(
            crate::plan::ModuleId::root(),
            source,
            substitution,
            shape,
            return_,
        )
    }

    fn in_module(
        module: crate::plan::ModuleId,
        source: Source,
        substitution: TypeSubstitution,
        shape: FunctionShape,
        return_: Return,
    ) -> Self {
        Self {
            module,
            source,
            substitution,
            shape,
            return_,
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

    pub(crate) fn shape(&self) -> &FunctionShape {
        &self.shape
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}

pub(crate) trait SubstituteFunctionReturn: Sized {
    fn substitute(&self, substitution: &TypeSubstitution) -> Self;
}

impl SubstituteFunctionReturn for () {
    fn substitute(&self, _substitution: &TypeSubstitution) -> Self {}
}

impl SubstituteFunctionReturn for CustomValueShape {
    fn substitute(&self, substitution: &TypeSubstitution) -> Self {
        self.substitute(substitution)
    }
}

impl SubstituteFunctionReturn for Box<[ValueShape]> {
    fn substitute(&self, substitution: &TypeSubstitution) -> Self {
        self.iter()
            .map(|shape| shape.substitute(substitution))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

impl SubstituteFunctionReturn for Box<ValueShape> {
    fn substitute(&self, substitution: &TypeSubstitution) -> Self {
        Box::new(self.as_ref().substitute(substitution))
    }
}

impl SubstituteFunctionReturn for Box<FunctionShape> {
    fn substitute(&self, substitution: &TypeSubstitution) -> Self {
        Box::new(self.as_ref().substitute(substitution))
    }
}

impl<Source: Copy, Return: SubstituteFunctionReturn>
    TypedConstantFunctionInstantiation<Source, Return>
{
    pub(super) fn substitute(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.source,
            self.substitution.substitute(outer),
            self.shape.substitute(outer),
            self.return_.substitute(outer),
        )
    }
}

impl ConstantGenericFunctionInstantiation {
    pub(super) fn specialize<Id, Return>(
        &self,
        outer: &TypeSubstitution,
        return_: Return,
    ) -> TypedConstantFunctionInstantiation<ConstantFunctionTemplateSource<Id>, Return> {
        TypedConstantFunctionInstantiation::in_module(
            self.module(),
            ConstantFunctionTemplateSource::Generic(*self.source()),
            self.substitution().substitute(outer),
            self.shape().substitute(outer),
            return_,
        )
    }

    pub(super) fn substitute_generic(
        &self,
        outer: &TypeSubstitution,
        return_: TypeParameterId,
    ) -> Self {
        TypedConstantFunctionInstantiation::in_module(
            self.module(),
            *self.source(),
            self.substitution().substitute(outer),
            self.shape().substitute(outer),
            return_,
        )
    }
}

impl ConstantFunctionInstantiation {
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
            Self::List(value) => value.module(),
            Self::Function(value) => value.module(),
        }
    }

    fn from_generic_source(
        module: crate::plan::ModuleId,
        source: ConstantGenericFunctionTemplateId,
        substitution: TypeSubstitution,
        shape: FunctionShape,
    ) -> Self {
        match shape.return_shape().clone() {
            ValueShape::Parameter(parameter) => {
                Self::Generic(TypedConstantFunctionInstantiation::in_module(
                    module,
                    source,
                    substitution,
                    shape,
                    parameter,
                ))
            }
            ValueShape::Int => Self::Int(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::String => Self::String(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::BitArray => Self::BitArray(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::UtfCodepoint => {
                Self::UtfCodepoint(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Generic(source),
                    substitution,
                    shape,
                    (),
                ))
            }
            ValueShape::Custom(return_) => {
                Self::Custom(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Generic(source),
                    substitution,
                    shape,
                    return_,
                ))
            }
            ValueShape::Float => Self::Float(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::Bool => Self::Bool(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::Nil => Self::Nil(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                (),
            )),
            ValueShape::Tuple(return_) => {
                Self::Tuple(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Generic(source),
                    substitution,
                    shape,
                    return_,
                ))
            }
            ValueShape::List(item) => Self::List(TypedConstantFunctionInstantiation::in_module(
                module,
                ConstantFunctionTemplateSource::Generic(source),
                substitution,
                shape,
                item,
            )),
            ValueShape::Function(return_) => {
                Self::Function(TypedConstantFunctionInstantiation::in_module(
                    module,
                    ConstantFunctionTemplateSource::Generic(source),
                    substitution,
                    shape,
                    return_,
                ))
            }
        }
    }

    pub(super) fn substitute(&self, outer: &TypeSubstitution) -> Self {
        match self {
            Self::Generic(value) => Self::from_generic_source(
                value.module(),
                *value.source(),
                value.substitution().substitute(outer),
                value.shape().substitute(outer),
            ),
            Self::Int(value) => Self::Int(value.substitute(outer)),
            Self::String(value) => Self::String(value.substitute(outer)),
            Self::BitArray(value) => Self::BitArray(value.substitute(outer)),
            Self::UtfCodepoint(value) => Self::UtfCodepoint(value.substitute(outer)),
            Self::Custom(value) => Self::Custom(value.substitute(outer)),
            Self::Float(value) => Self::Float(value.substitute(outer)),
            Self::Bool(value) => Self::Bool(value.substitute(outer)),
            Self::Nil(value) => Self::Nil(value.substitute(outer)),
            Self::Tuple(value) => Self::Tuple(value.substitute(outer)),
            Self::List(value) => Self::List(value.substitute(outer)),
            Self::Function(value) => Self::Function(value.substitute(outer)),
        }
    }
}

impl<Target, Reference, Return> TypedConstantFunctionValue<Target, Reference, Return> {
    fn target(shape: FunctionShape, return_: Return, target: Target) -> Self {
        Self {
            shape,
            return_,
            kind: TypedConstantFunctionValueKind::Target(target),
        }
    }

    fn reference(shape: FunctionShape, return_: Return, reference: Reference) -> Self {
        Self {
            shape,
            return_,
            kind: TypedConstantFunctionValueKind::Reference(reference),
        }
    }

    pub(super) fn shape(&self) -> &FunctionShape {
        &self.shape
    }

    pub(super) fn kind(&self) -> &TypedConstantFunctionValueKind<Target, Reference> {
        &self.kind
    }
}

impl ConstantFunctionValue {
    pub(super) fn function_reference(shape: FunctionShape, target: FunctionReference) -> Self {
        match shape.return_shape().clone() {
            ValueShape::Parameter(parameter) => {
                Self::Generic(TypedConstantFunctionValue::target(shape, parameter, target))
            }
            ValueShape::Int => Self::Int(TypedConstantFunctionValue::target(shape, (), target)),
            ValueShape::String => {
                Self::String(TypedConstantFunctionValue::target(shape, (), target))
            }
            ValueShape::BitArray => {
                Self::BitArray(TypedConstantFunctionValue::target(shape, (), target))
            }
            ValueShape::UtfCodepoint => {
                Self::UtfCodepoint(TypedConstantFunctionValue::target(shape, (), target))
            }
            ValueShape::Custom(return_) => Self::Custom(TypedConstantFunctionValue::target(
                shape,
                return_,
                ConstantCustomFunctionTarget::Reference(target),
            )),
            ValueShape::Float => Self::Float(TypedConstantFunctionValue::target(shape, (), target)),
            ValueShape::Bool => Self::Bool(TypedConstantFunctionValue::target(shape, (), target)),
            ValueShape::Nil => Self::Nil(TypedConstantFunctionValue::target(shape, (), target)),
            ValueShape::Tuple(return_) => {
                Self::Tuple(TypedConstantFunctionValue::target(shape, return_, target))
            }
            ValueShape::List(item) => {
                Self::List(TypedConstantFunctionValue::target(shape, item, target))
            }
            ValueShape::Function(return_) => {
                Self::Function(TypedConstantFunctionValue::target(shape, return_, target))
            }
        }
    }

    pub(super) fn constructor(
        shape: FunctionShape,
        return_: CustomValueShape,
        constructor: CustomConstructor,
    ) -> Self {
        Self::Custom(TypedConstantFunctionValue::target(
            shape,
            return_,
            ConstantCustomFunctionTarget::Constructor(constructor),
        ))
    }

    pub(super) fn reference(instantiation: ConstantFunctionInstantiation) -> Self {
        match instantiation {
            ConstantFunctionInstantiation::Generic(value) => {
                let shape = value.shape().clone();
                let return_ = *value.return_();
                Self::Generic(TypedConstantFunctionValue::reference(shape, return_, value))
            }
            ConstantFunctionInstantiation::Int(value) => {
                let shape = value.shape().clone();
                Self::Int(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::String(value) => {
                let shape = value.shape().clone();
                Self::String(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::BitArray(value) => {
                let shape = value.shape().clone();
                Self::BitArray(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::UtfCodepoint(value) => {
                let shape = value.shape().clone();
                Self::UtfCodepoint(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::Custom(value) => {
                let shape = value.shape().clone();
                let return_ = value.return_().clone();
                Self::Custom(TypedConstantFunctionValue::reference(shape, return_, value))
            }
            ConstantFunctionInstantiation::Float(value) => {
                let shape = value.shape().clone();
                Self::Float(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::Bool(value) => {
                let shape = value.shape().clone();
                Self::Bool(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::Nil(value) => {
                let shape = value.shape().clone();
                Self::Nil(TypedConstantFunctionValue::reference(shape, (), value))
            }
            ConstantFunctionInstantiation::Tuple(value) => {
                let shape = value.shape().clone();
                let return_ = value.return_().clone();
                Self::Tuple(TypedConstantFunctionValue::reference(shape, return_, value))
            }
            ConstantFunctionInstantiation::List(value) => {
                let shape = value.shape().clone();
                let return_ = value.return_().clone();
                Self::List(TypedConstantFunctionValue::reference(shape, return_, value))
            }
            ConstantFunctionInstantiation::Function(value) => {
                let shape = value.shape().clone();
                let return_ = value.return_().clone();
                Self::Function(TypedConstantFunctionValue::reference(shape, return_, value))
            }
        }
    }

    pub(super) fn shape(&self) -> &FunctionShape {
        match self {
            Self::Generic(value) => value.shape(),
            Self::Int(value) => value.shape(),
            Self::String(value) => value.shape(),
            Self::BitArray(value) => value.shape(),
            Self::UtfCodepoint(value) => value.shape(),
            Self::Custom(value) => value.shape(),
            Self::Float(value) => value.shape(),
            Self::Bool(value) => value.shape(),
            Self::Nil(value) => value.shape(),
            Self::Tuple(value) => value.shape(),
            Self::List(value) => value.shape(),
            Self::Function(value) => value.shape(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstantGenericFunctionTemplateId, TypedConstantFunctionInstantiation};
    use crate::plan::{FunctionShape, TypeParameterId, TypeSubstitution, ValueShape};

    #[test]
    fn generic_function_reference_substitution_preserves_symbolic_return_owner() {
        let parameter = TypeParameterId(0);
        let return_parameter = TypeParameterId(2);
        let inner = TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]);
        let outer =
            TypeSubstitution::from_arguments(vec![ValueShape::List(Box::new(ValueShape::Int))]);
        let source = ConstantGenericFunctionTemplateId(3);
        let value = TypedConstantFunctionInstantiation::new(
            source,
            inner,
            FunctionShape::new(
                vec![ValueShape::Parameter(parameter)],
                ValueShape::Parameter(parameter),
            ),
            parameter,
        );

        assert_eq!(
            value.substitute_generic(&outer, return_parameter),
            TypedConstantFunctionInstantiation::new(
                source,
                TypeSubstitution::from_arguments(vec![ValueShape::List(
                    Box::new(ValueShape::Int,)
                )]),
                FunctionShape::new(
                    vec![ValueShape::List(Box::new(ValueShape::Int))],
                    ValueShape::List(Box::new(ValueShape::Int)),
                ),
                return_parameter,
            ),
        );
    }
}
