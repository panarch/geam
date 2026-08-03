use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use super::EvaluatedCapture;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId,
    ExternalListFunctionId, FloatFunctionId, FunctionFunctionId, FunctionReturnFamily,
    GenericCallableId, IntFunctionId, ListFunctionId, NeverFunctionId, NilFunctionId,
    ProfiledFunctionFunctionId, RuntimeListFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::{CustomConstructorId, FunctionType};

static NEXT_FUNCTION_INSTANCE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedFunction<Id> {
    pub(super) identity: EvaluatedFunctionIdentity,
    runtime_id: Id,
    params: Vec<ParamLocal>,
    captures: Vec<EvaluatedCapture>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(in crate::runtime) enum FunctionReferenceIdentity {
    Table {
        table: FunctionTableIdentity,
        index: usize,
    },
    Generic(GenericCallableId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::runtime) enum FunctionTableIdentity {
    Value(FunctionReturnFamily),
    List(ListFunctionReturnFamily),
    Function(FunctionReturnFamily),
    ReturningListFunction(ListFunctionReturnFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::runtime) enum ListFunctionReturnFamily {
    Parameter,
    ParameterList,
    Int,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Float,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

#[derive(Debug, Clone)]
pub(super) enum EvaluatedFunctionIdentity {
    Reference(FunctionReferenceIdentity),
    Instance(Rc<FunctionInstance>),
}

#[derive(Debug)]
pub(super) struct FunctionInstance(pub(super) u64);

impl PartialEq for EvaluatedFunctionIdentity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Reference(left), Self::Reference(right)) => left == right,
            (Self::Instance(left), Self::Instance(right)) => Rc::ptr_eq(left, right),
            _ => false,
        }
    }
}

impl Eq for EvaluatedFunctionIdentity {}

pub(in crate::runtime) trait FunctionReferenceId {
    fn reference_identity(&self) -> FunctionReferenceIdentity;
}

pub(in crate::runtime) type EvaluatedIntFunction = EvaluatedFunction<IntFunctionId>;
pub(in crate::runtime) type EvaluatedFloatFunction = EvaluatedFunction<FloatFunctionId>;
pub(in crate::runtime) type EvaluatedStringFunction = EvaluatedFunction<StringFunctionId>;
pub(in crate::runtime) type EvaluatedBitArrayFunction = EvaluatedFunction<BitArrayFunctionId>;
pub(in crate::runtime) type EvaluatedUtfCodepointFunction =
    EvaluatedFunction<UtfCodepointFunctionId>;
pub(in crate::runtime) type EvaluatedGenericFunction = EvaluatedFunction<GenericCallableId>;
pub(in crate::runtime) type EvaluatedNeverFunction = EvaluatedFunction<NeverFunctionId>;
#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedCustomFunction {
    Function(EvaluatedFunction<CustomFunctionId>),
    Constructor(EvaluatedFunction<CustomConstructorId>),
}
pub(in crate::runtime) type EvaluatedExternalFunction = EvaluatedFunction<ExternalFunctionId>;
pub(in crate::runtime) type EvaluatedBoolFunction = EvaluatedFunction<BoolFunctionId>;
pub(in crate::runtime) type EvaluatedNilFunction = EvaluatedFunction<NilFunctionId>;
pub(in crate::runtime) type EvaluatedTupleFunction = EvaluatedFunction<TupleFunctionId>;
pub(in crate::runtime) type EvaluatedListFunction = EvaluatedFunction<RuntimeListFunctionId>;
pub(in crate::runtime) type EvaluatedExternalListFunction =
    EvaluatedFunction<ExternalListFunctionId>;
pub(in crate::runtime) type EvaluatedCoreFunctionFunction =
    EvaluatedFunction<ProfiledFunctionFunctionId<std::convert::Infallible>>;
pub(in crate::runtime) type EvaluatedExternalFunctionFunction =
    EvaluatedFunction<crate::plan::execution::graph::ExternalFunctionCallTarget>;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedFunctionFunction {
    Core(EvaluatedCoreFunctionFunction),
    External(EvaluatedExternalFunctionFunction),
}

impl FunctionReferenceIdentity {
    fn value(family: FunctionReturnFamily, index: usize) -> Self {
        Self::Table {
            table: FunctionTableIdentity::Value(family),
            index,
        }
    }

    fn list(family: ListFunctionReturnFamily, index: usize) -> Self {
        Self::Table {
            table: FunctionTableIdentity::List(family),
            index,
        }
    }

    fn function(family: FunctionReturnFamily, index: usize) -> Self {
        Self::Table {
            table: FunctionTableIdentity::Function(family),
            index,
        }
    }

    fn returning_list_function(family: ListFunctionReturnFamily, index: usize) -> Self {
        Self::Table {
            table: FunctionTableIdentity::ReturningListFunction(family),
            index,
        }
    }
}

impl FunctionReferenceId for GenericCallableId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::Generic(self.clone())
    }
}

impl FunctionReferenceId for NeverFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Never, self.0)
    }
}

impl FunctionReferenceId for IntFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Int, self.0)
    }
}

impl FunctionReferenceId for FloatFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Float, self.0)
    }
}

impl FunctionReferenceId for StringFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::String, self.0)
    }
}

impl FunctionReferenceId for BitArrayFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::BitArray, self.0)
    }
}

impl FunctionReferenceId for UtfCodepointFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::UtfCodepoint, self.0)
    }
}

impl FunctionReferenceId for CustomFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Custom, self.index())
    }
}

impl FunctionReferenceId for ExternalFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::External, self.index())
    }
}

impl FunctionReferenceId for BoolFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Bool, self.0)
    }
}

impl FunctionReferenceId for NilFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Nil, self.0)
    }
}

impl FunctionReferenceId for TupleFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        FunctionReferenceIdentity::value(FunctionReturnFamily::Tuple, self.0)
    }
}

impl FunctionReferenceId for ListFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        match self {
            Self::Parameter(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Parameter, id.index())
            }
            Self::ParameterList(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::ParameterList, id.index())
            }
            Self::Int(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Int, id.index())
            }
            Self::String(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::String, id.index())
            }
            Self::BitArray(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::BitArray, id.index())
            }
            Self::UtfCodepoint(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::UtfCodepoint, id.index())
            }
            Self::Custom(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Custom, id.index())
            }
            Self::Float(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Float, id.index())
            }
            Self::Bool(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Bool, id.index())
            }
            Self::Nil(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Nil, id.index())
            }
            Self::Tuple(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Tuple, id.index())
            }
            Self::List(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::List, id.index())
            }
            Self::Function(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Function, id.index())
            }
        }
    }
}

impl FunctionReferenceId for RuntimeListFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        match self {
            Self::Core(id) => id.reference_identity(),
            Self::External(id) => {
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::External, id.index())
            }
        }
    }
}

impl FunctionReferenceId for FunctionFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        match self {
            Self::Generic(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Generic, id.index())
            }
            Self::Never(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Never, id.index())
            }
            Self::Int(id) => FunctionReferenceIdentity::function(FunctionReturnFamily::Int, id.0),
            Self::Float(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Float, id.0)
            }
            Self::String(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::String, id.0)
            }
            Self::BitArray(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::BitArray, id.0)
            }
            Self::UtfCodepoint(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::UtfCodepoint, id.0)
            }
            Self::Custom(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Custom, id.index())
            }
            Self::External(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::External, id.index())
            }
            Self::Bool(id) => FunctionReferenceIdentity::function(FunctionReturnFamily::Bool, id.0),
            Self::Nil(id) => FunctionReferenceIdentity::function(FunctionReturnFamily::Nil, id.0),
            Self::Tuple(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Tuple, id.0)
            }
            Self::List(id) => match id {
                crate::plan::execution::function::ListFunctionFunctionId::Parameter {
                    id, ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Parameter,
                    id.0,
                ),
                crate::plan::execution::function::ListFunctionFunctionId::ParameterList {
                    id,
                    ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::ParameterList,
                    id.0,
                ),
                crate::plan::execution::function::ListFunctionFunctionId::Int { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Int,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::String { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::String,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::BitArray {
                    id, ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::BitArray,
                    id.0,
                ),
                crate::plan::execution::function::ListFunctionFunctionId::UtfCodepoint {
                    id,
                    ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::UtfCodepoint,
                    id.0,
                ),
                crate::plan::execution::function::ListFunctionFunctionId::Custom { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Custom,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::External {
                    id, ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::External,
                    id.0,
                ),
                crate::plan::execution::function::ListFunctionFunctionId::Float { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Float,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::Bool { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Bool,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::Nil { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Nil,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::Tuple { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Tuple,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::List { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::List,
                        id.0,
                    )
                }
                crate::plan::execution::function::ListFunctionFunctionId::Function {
                    id, ..
                } => FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Function,
                    id.0,
                ),
            },
            Self::Function(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Function, id.index())
            }
        }
    }
}

impl FunctionReferenceId for ProfiledFunctionFunctionId<std::convert::Infallible> {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        use crate::plan::execution::function::ExecutionGraphProfile;

        <std::convert::Infallible as ExecutionGraphProfile>::function_function(self)
            .reference_identity()
    }
}

impl FunctionReferenceId for crate::plan::execution::graph::ExternalFunctionCallTarget {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        self.runtime_id().reference_identity()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedFunctionValue {
    kind: EvaluatedFunctionValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedFunctionValueKind {
    Generic(EvaluatedGenericFunction),
    Never(EvaluatedNeverFunction),
    Int(EvaluatedIntFunction),
    Float(EvaluatedFloatFunction),
    String(EvaluatedStringFunction),
    BitArray(EvaluatedBitArrayFunction),
    UtfCodepoint(EvaluatedUtfCodepointFunction),
    Custom(EvaluatedCustomFunction),
    External(EvaluatedExternalFunction),
    Bool(EvaluatedBoolFunction),
    Nil(EvaluatedNilFunction),
    Tuple(EvaluatedTupleFunction),
    List(EvaluatedListFunction),
    Function(EvaluatedFunctionFunction),
}

impl<Id: Clone + FunctionReferenceId> EvaluatedFunction<Id> {
    pub(in crate::runtime) fn reference(
        runtime_id: Id,
        params: Vec<ParamLocal>,
        captures: Vec<EvaluatedCapture>,
        type_: FunctionType,
    ) -> Self {
        let identity = EvaluatedFunctionIdentity::Reference(runtime_id.reference_identity());
        Self {
            identity,
            runtime_id,
            params,
            captures,
            type_,
        }
    }
}

impl<Id: Clone> EvaluatedFunction<Id> {
    pub(in crate::runtime) fn closure(
        runtime_id: Id,
        params: Vec<ParamLocal>,
        captures: Vec<EvaluatedCapture>,
        type_: FunctionType,
    ) -> Self {
        Self {
            identity: EvaluatedFunctionIdentity::Instance(Rc::new(FunctionInstance(
                NEXT_FUNCTION_INSTANCE_ID.fetch_add(1, Ordering::Relaxed),
            ))),
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(in crate::runtime) fn runtime_id(&self) -> Id {
        self.runtime_id.clone()
    }

    pub(in crate::runtime) fn params(&self) -> &[ParamLocal] {
        &self.params
    }

    pub(in crate::runtime) fn captures(&self) -> &[EvaluatedCapture] {
        &self.captures
    }

    pub(in crate::runtime) fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(in crate::runtime) fn with_type(mut self, type_: FunctionType) -> Self {
        self.type_ = type_;
        self
    }

    pub(in crate::runtime) fn map_runtime_id<NewId>(
        self,
        map: impl FnOnce(Id) -> NewId,
    ) -> EvaluatedFunction<NewId> {
        EvaluatedFunction {
            identity: self.identity,
            runtime_id: map(self.runtime_id),
            params: self.params,
            captures: self.captures,
            type_: self.type_,
        }
    }
}

impl EvaluatedCustomFunction {
    #[cfg(test)]
    pub(in crate::runtime) fn reference(
        runtime_id: CustomFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<EvaluatedCapture>,
        type_: FunctionType,
    ) -> Self {
        Self::Function(EvaluatedFunction::reference(
            runtime_id, params, captures, type_,
        ))
    }

    pub(in crate::runtime) fn constructor(
        constructor: CustomConstructorId,
        type_: FunctionType,
    ) -> Self {
        Self::Constructor(EvaluatedFunction::closure(
            constructor,
            Vec::new(),
            Vec::new(),
            type_,
        ))
    }

    pub(in crate::runtime) fn params(&self) -> &[ParamLocal] {
        match self {
            Self::Function(value) => value.params(),
            Self::Constructor(value) => value.params(),
        }
    }

    pub(in crate::runtime) fn captures(&self) -> &[EvaluatedCapture] {
        match self {
            Self::Function(value) => value.captures(),
            Self::Constructor(value) => value.captures(),
        }
    }

    pub(in crate::runtime) fn type_(&self) -> &FunctionType {
        match self {
            Self::Function(value) => value.type_(),
            Self::Constructor(value) => value.type_(),
        }
    }

    pub(in crate::runtime) fn with_type(self, type_: FunctionType) -> Self {
        match self {
            Self::Function(value) => Self::Function(value.with_type(type_)),
            Self::Constructor(value) => Self::Constructor(value.with_type(type_)),
        }
    }
}

impl EvaluatedFunctionFunction {
    pub(super) fn identity(&self) -> &EvaluatedFunctionIdentity {
        match self {
            Self::Core(value) => &value.identity,
            Self::External(value) => &value.identity,
        }
    }

    pub(in crate::runtime) fn type_(&self) -> &FunctionType {
        match self {
            Self::Core(value) => value.type_(),
            Self::External(value) => value.type_(),
        }
    }

    pub(in crate::runtime) fn params(&self) -> &[ParamLocal] {
        match self {
            Self::Core(value) => value.params(),
            Self::External(value) => value.params(),
        }
    }

    pub(in crate::runtime) fn captures(&self) -> &[EvaluatedCapture] {
        match self {
            Self::Core(value) => value.captures(),
            Self::External(value) => value.captures(),
        }
    }

    pub(in crate::runtime) fn with_type(self, type_: FunctionType) -> Self {
        match self {
            Self::Core(value) => Self::Core(value.with_type(type_)),
            Self::External(value) => Self::External(value.with_type(type_)),
        }
    }
}

macro_rules! evaluated_function_value_from {
    ($function:ty, $variant:ident) => {
        impl From<$function> for EvaluatedFunctionValue {
            fn from(value: $function) -> Self {
                Self::from_kind(EvaluatedFunctionValueKind::$variant(value))
            }
        }
    };
}

evaluated_function_value_from!(EvaluatedGenericFunction, Generic);
evaluated_function_value_from!(EvaluatedNeverFunction, Never);
evaluated_function_value_from!(EvaluatedIntFunction, Int);
evaluated_function_value_from!(EvaluatedFloatFunction, Float);
evaluated_function_value_from!(EvaluatedStringFunction, String);
evaluated_function_value_from!(EvaluatedBitArrayFunction, BitArray);
evaluated_function_value_from!(EvaluatedUtfCodepointFunction, UtfCodepoint);
evaluated_function_value_from!(EvaluatedCustomFunction, Custom);
evaluated_function_value_from!(EvaluatedExternalFunction, External);
evaluated_function_value_from!(EvaluatedBoolFunction, Bool);
evaluated_function_value_from!(EvaluatedNilFunction, Nil);
evaluated_function_value_from!(EvaluatedTupleFunction, Tuple);
evaluated_function_value_from!(EvaluatedListFunction, List);
evaluated_function_value_from!(EvaluatedFunctionFunction, Function);

impl EvaluatedFunctionValue {
    pub(in crate::runtime) fn from_kind(kind: EvaluatedFunctionValueKind) -> Self {
        Self { kind }
    }

    pub(in crate::runtime) fn kind(&self) -> &EvaluatedFunctionValueKind {
        &self.kind
    }

    pub(in crate::runtime) fn type_(&self) -> &FunctionType {
        match &self.kind {
            EvaluatedFunctionValueKind::Generic(value) => value.type_(),
            EvaluatedFunctionValueKind::Never(value) => value.type_(),
            EvaluatedFunctionValueKind::Int(value) => value.type_(),
            EvaluatedFunctionValueKind::Float(value) => value.type_(),
            EvaluatedFunctionValueKind::String(value) => value.type_(),
            EvaluatedFunctionValueKind::BitArray(value) => value.type_(),
            EvaluatedFunctionValueKind::UtfCodepoint(value) => value.type_(),
            EvaluatedFunctionValueKind::Custom(value) => value.type_(),
            EvaluatedFunctionValueKind::External(value) => value.type_(),
            EvaluatedFunctionValueKind::Bool(value) => value.type_(),
            EvaluatedFunctionValueKind::Nil(value) => value.type_(),
            EvaluatedFunctionValueKind::Tuple(value) => value.type_(),
            EvaluatedFunctionValueKind::List(value) => value.type_(),
            EvaluatedFunctionValueKind::Function(value) => value.type_(),
        }
    }

    pub(in crate::runtime) fn with_type(self, type_: FunctionType) -> Self {
        let kind = match self.kind {
            EvaluatedFunctionValueKind::Generic(value) => {
                EvaluatedFunctionValueKind::Generic(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Never(value) => {
                EvaluatedFunctionValueKind::Never(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Int(value) => {
                EvaluatedFunctionValueKind::Int(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Float(value) => {
                EvaluatedFunctionValueKind::Float(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::String(value) => {
                EvaluatedFunctionValueKind::String(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::BitArray(value) => {
                EvaluatedFunctionValueKind::BitArray(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::UtfCodepoint(value) => {
                EvaluatedFunctionValueKind::UtfCodepoint(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Custom(value) => {
                EvaluatedFunctionValueKind::Custom(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::External(value) => {
                EvaluatedFunctionValueKind::External(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Bool(value) => {
                EvaluatedFunctionValueKind::Bool(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Nil(value) => {
                EvaluatedFunctionValueKind::Nil(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Tuple(value) => {
                EvaluatedFunctionValueKind::Tuple(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::List(value) => {
                EvaluatedFunctionValueKind::List(value.with_type(type_))
            }
            EvaluatedFunctionValueKind::Function(value) => {
                EvaluatedFunctionValueKind::Function(value.with_type(type_))
            }
        };
        Self { kind }
    }
}

impl EvaluatedFunctionValueKind {
    pub(in crate::runtime) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Generic(_) => FunctionReturnFamily::Generic,
            Self::Never(_) => FunctionReturnFamily::Never,
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Custom(_) => FunctionReturnFamily::Custom,
            Self::External(_) => FunctionReturnFamily::External,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::List(_) => FunctionReturnFamily::List,
            Self::Function(_) => FunctionReturnFamily::Function,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::source::values_equal;
    use super::super::{EvaluatedCapture, EvaluatedValue};
    use super::{
        EvaluatedCustomFunction, EvaluatedFunctionValue, EvaluatedIntFunction, FunctionReferenceId,
        FunctionReferenceIdentity, ListFunctionReturnFamily,
    };
    use crate::plan::execution::function::{
        BitArrayListFunctionFunctionId, BoolListFunctionFunctionId, CustomListFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionFunctionId, FunctionListFunctionFunctionId,
        IntFunctionId, IntListFunctionFunctionId, ListFunctionFunctionId, ListFunctionId,
        ListListFunctionFunctionId, NilListFunctionFunctionId, ParameterListFunctionFunctionId,
        ParameterListListFunctionFunctionId, StringListFunctionFunctionId,
        TupleListFunctionFunctionId, UtfCodepointListFunctionFunctionId,
    };
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::runtime::state::RuntimeState;

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
pub type Boxed { Boxed(Int) }
fn customs() -> List(Boxed) { [] }
fn custom() -> Boxed { Boxed(1) }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn parameters(values: List(value)) { values }
fn parameter_lists(values: List(List(value))) { values }
fn take_function_function(value: fn() -> fn() -> Int) { 0 }
pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    customs,
    custom,
    floats,
    bools,
    nils,
    tuples,
    lists,
    functions,
    take_function_function,
  )
  let _ = parameters([])
  let _ = parameter_lists([[]])
  0
}
"#;
    #[test]
    fn function_identity_distinguishes_references_and_instances() {
        let mut echo = Vec::new();
        let state = RuntimeState::new(&mut echo);
        let int_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Int,
        );
        let reference = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            int_type.clone(),
        );
        let same_target_with_different_metadata = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            vec![ParamLocal::Int(IntLocalId(0))],
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                vec![crate::plan::execution::type_::ValueType::Int],
                crate::plan::execution::type_::ValueType::Int,
            ),
        );
        let different_target = EvaluatedIntFunction::reference(
            IntFunctionId(1),
            Vec::new(),
            Vec::new(),
            int_type.clone(),
        );
        let reference_for_instance_comparison = reference.clone();
        let closure = EvaluatedIntFunction::closure(
            IntFunctionId(0),
            Vec::new(),
            vec![EvaluatedCapture::int(IntLocalId(0), 1.into())],
            int_type.clone(),
        );
        let same_closure = closure.clone();
        let separate_closure = EvaluatedIntFunction::closure(
            IntFunctionId(0),
            Vec::new(),
            vec![EvaluatedCapture::int(IntLocalId(0), 1.into())],
            int_type,
        );

        assert!(values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(reference.clone())),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                same_target_with_different_metadata,
            )),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(reference)),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(different_target)),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                reference_for_instance_comparison,
            )),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure.clone())),
        ));
        assert!(values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure.clone())),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(same_closure)),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure)),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(separate_closure)),
        ));
    }

    #[test]
    fn list_function_reference_identity_uses_item_family_table_target() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let ids = [
            ListFunctionId::Parameter(plan.parameter_list_function_id(0)),
            ListFunctionId::ParameterList(plan.parameter_list_list_function_id(0)),
            ListFunctionId::Int(plan.int_list_function_id(0)),
            ListFunctionId::String(plan.string_list_function_id(0)),
            ListFunctionId::BitArray(plan.bit_array_list_function_id(0)),
            ListFunctionId::UtfCodepoint(plan.utf_codepoint_list_function_id(0)),
            ListFunctionId::Custom(plan.custom_list_function_id(0)),
            ListFunctionId::Float(plan.float_list_function_id(0)),
            ListFunctionId::Bool(plan.bool_list_function_id(0)),
            ListFunctionId::Nil(plan.nil_list_function_id(0)),
            ListFunctionId::Tuple(plan.tuple_list_function_id(0)),
            ListFunctionId::List(plan.list_list_function_id(0)),
            ListFunctionId::Function(plan.function_list_function_id(0)),
        ];

        assert_eq!(
            ids.map(|id| id.reference_identity()),
            [
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Parameter, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::ParameterList, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Int, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::String, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::BitArray, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::UtfCodepoint, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Custom, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Float, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Bool, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Nil, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Tuple, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::List, 0),
                FunctionReferenceIdentity::list(ListFunctionReturnFamily::Function, 0),
            ],
        );
    }

    #[test]
    fn function_returning_list_reference_identity_uses_table_target() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let type_ = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Int,
        );
        let ids = [
            ListFunctionFunctionId::Parameter {
                id: ParameterListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.parameter_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::ParameterList {
                id: ParameterListListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.parameter_list_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.int_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.string_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::BitArray {
                id: BitArrayListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.bit_array_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::UtfCodepoint {
                id: UtfCodepointListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Custom {
                id: CustomListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.custom_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.float_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.bool_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.nil_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.tuple_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.list_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(0),
                type_,
                list_type: plan.function_list_function_id(0).type_id(),
            },
        ];

        assert_eq!(
            ids.map(|id| FunctionFunctionId::List(id).reference_identity()),
            [
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Parameter,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::ParameterList,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Int,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::String,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::BitArray,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::UtfCodepoint,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Custom,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Float,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Bool,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Nil,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Tuple,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::List,
                    0,
                ),
                FunctionReferenceIdentity::returning_list_function(
                    ListFunctionReturnFamily::Function,
                    0,
                ),
            ],
        );
    }

    #[test]
    fn constructor_callable_identity_is_fresh_and_clone_preserving() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut echo = Vec::new();
        let state = RuntimeState::new(&mut echo);
        let constructor_id = plan.custom_constructor_id(0, 0);
        let constructor = plan.custom_constructor(constructor_id);
        let type_ = crate::plan::execution::type_::FunctionType::new(
            constructor
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect(),
            crate::plan::execution::type_::ValueType::Custom(constructor_id.type_id()),
        );
        let first = EvaluatedCustomFunction::constructor(constructor_id, type_.clone());
        let same = first.clone();
        let separate = EvaluatedCustomFunction::constructor(constructor_id, type_);

        assert!(values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(first.clone())),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(same)),
        ));
        assert!(!values_equal(
            state.lists(),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(first)),
            &EvaluatedValue::Function(EvaluatedFunctionValue::from(separate)),
        ));
    }
}
