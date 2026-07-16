use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use ecow::EcoString;
use num_bigint::BigInt;
use std::rc::Rc;

use super::state::{ListValueId, RuntimeState};
use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionId,
    BoolFunctionLocalId, BoolLocalId, CustomConstructorId, CustomFunctionId, CustomFunctionLocal,
    CustomLocal, CustomTypeId, FloatFunctionId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionId, FunctionFunctionLocal, FunctionReturnFamily, FunctionType, IntFunctionId,
    IntFunctionLocalId, IntLocalId, ListFunctionId, ListFunctionLocal, NilFunctionId,
    NilFunctionLocalId, NilLocalId, ParamLocal, StringFunctionId, StringFunctionLocalId,
    StringLocalId, TupleFunctionId, TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::runtime) struct EvaluatedBitArray {
    bits: Rc<BitVec<u8, Msb0>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedCustomValue {
    constructor: CustomConstructorId,
    fields: Box<[EvaluatedValue]>,
}

impl EvaluatedCustomValue {
    pub(in crate::runtime) fn from_fields(
        constructor: CustomConstructorId,
        fields: Box<[EvaluatedValue]>,
    ) -> Self {
        Self {
            constructor,
            fields,
        }
    }

    pub(in crate::runtime) fn type_id(&self) -> CustomTypeId {
        self.constructor.type_id()
    }

    pub(in crate::runtime) fn constructor(&self) -> CustomConstructorId {
        self.constructor
    }

    pub(in crate::runtime) fn fields(&self) -> &[EvaluatedValue] {
        &self.fields
    }
}

impl EvaluatedBitArray {
    pub(in crate::runtime) fn new(mut bits: BitVec<u8, Msb0>) -> Self {
        bits.force_align();
        bits.set_uninitialized(false);
        Self {
            bits: Rc::new(bits),
        }
    }

    pub(in crate::runtime) fn bits(&self) -> &bitvec::slice::BitSlice<u8, Msb0> {
        &self.bits
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedValue {
    Int(BigInt),
    Float(f64),
    String(EcoString),
    BitArray(EvaluatedBitArray),
    UtfCodepoint(char),
    Custom(EvaluatedCustomValue),
    Bool(bool),
    Nil,
    Tuple(Vec<EvaluatedValue>),
    List(ListValueId),
    Function(EvaluatedFunctionValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedFunction<Id> {
    identity: EvaluatedFunctionIdentity,
    runtime_id: Id,
    params: Vec<ParamLocal>,
    captures: Vec<EvaluatedCapture>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::runtime) struct FunctionReferenceIdentity {
    table: FunctionTableIdentity,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionTableIdentity {
    Value(FunctionReturnFamily),
    List(ListFunctionReturnFamily),
    Function(FunctionReturnFamily),
    ReturningListFunction(ListFunctionReturnFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListFunctionReturnFamily {
    Int,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Float,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

#[derive(Debug, Clone)]
enum EvaluatedFunctionIdentity {
    Reference(FunctionReferenceIdentity),
    Instance(Rc<FunctionInstance>),
}

#[derive(Debug)]
struct FunctionInstance;

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
#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedCustomFunction {
    Function(EvaluatedFunction<CustomFunctionId>),
    Constructor(EvaluatedFunction<CustomConstructorId>),
}
pub(in crate::runtime) type EvaluatedBoolFunction = EvaluatedFunction<BoolFunctionId>;
pub(in crate::runtime) type EvaluatedNilFunction = EvaluatedFunction<NilFunctionId>;
pub(in crate::runtime) type EvaluatedTupleFunction = EvaluatedFunction<TupleFunctionId>;
pub(in crate::runtime) type EvaluatedListFunction = EvaluatedFunction<ListFunctionId>;
pub(in crate::runtime) type EvaluatedFunctionFunction = EvaluatedFunction<FunctionFunctionId>;

impl FunctionReferenceIdentity {
    fn value(family: FunctionReturnFamily, index: usize) -> Self {
        Self {
            table: FunctionTableIdentity::Value(family),
            index,
        }
    }

    fn list(family: ListFunctionReturnFamily, index: usize) -> Self {
        Self {
            table: FunctionTableIdentity::List(family),
            index,
        }
    }

    fn function(family: FunctionReturnFamily, index: usize) -> Self {
        Self {
            table: FunctionTableIdentity::Function(family),
            index,
        }
    }

    fn returning_list_function(family: ListFunctionReturnFamily, index: usize) -> Self {
        Self {
            table: FunctionTableIdentity::ReturningListFunction(family),
            index,
        }
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

impl FunctionReferenceId for FunctionFunctionId {
    fn reference_identity(&self) -> FunctionReferenceIdentity {
        match self {
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
            Self::Bool(id) => FunctionReferenceIdentity::function(FunctionReturnFamily::Bool, id.0),
            Self::Nil(id) => FunctionReferenceIdentity::function(FunctionReturnFamily::Nil, id.0),
            Self::Tuple(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Tuple, id.0)
            }
            Self::List(id) => match id {
                crate::plan::execution::ListFunctionFunctionId::Int { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Int,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::String { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::String,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::BitArray { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::BitArray,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::UtfCodepoint { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::UtfCodepoint,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Custom { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Custom,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Float { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Float,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Bool { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Bool,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Nil { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Nil,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Tuple { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Tuple,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::List { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::List,
                        id.0,
                    )
                }
                crate::plan::execution::ListFunctionFunctionId::Function { id, .. } => {
                    FunctionReferenceIdentity::returning_list_function(
                        ListFunctionReturnFamily::Function,
                        id.0,
                    )
                }
            },
            Self::Function(id) => {
                FunctionReferenceIdentity::function(FunctionReturnFamily::Function, id.index())
            }
        }
    }
}

pub(in crate::runtime) fn function_type_from_slots(
    plan: &crate::plan::execution::ExecutionPlan,
    params: &[crate::plan::execution::ParamSlot],
    return_: crate::plan::execution::ValueType,
) -> FunctionType {
    FunctionType::new(
        params
            .iter()
            .map(|param| plan.shape_value_type(param.shape()))
            .collect(),
        return_,
    )
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedFunctionValue {
    kind: EvaluatedFunctionValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedFunctionValueKind {
    Int(EvaluatedIntFunction),
    Float(EvaluatedFloatFunction),
    String(EvaluatedStringFunction),
    BitArray(EvaluatedBitArrayFunction),
    UtfCodepoint(EvaluatedUtfCodepointFunction),
    Custom(EvaluatedCustomFunction),
    Bool(EvaluatedBoolFunction),
    Nil(EvaluatedNilFunction),
    Tuple(EvaluatedTupleFunction),
    List(EvaluatedListFunction),
    Function(EvaluatedFunctionFunction),
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedCapture {
    kind: EvaluatedCaptureKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedCaptureKind {
    Int {
        local: IntLocalId,
        value: BigInt,
    },
    Float {
        local: FloatLocalId,
        value: f64,
    },
    String {
        local: StringLocalId,
        value: EcoString,
    },
    BitArray {
        local: BitArrayLocalId,
        value: EvaluatedBitArray,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: char,
    },
    Custom {
        local: CustomLocal,
        value: EvaluatedCustomValue,
    },
    Bool {
        local: BoolLocalId,
        value: bool,
    },
    Nil {
        local: NilLocalId,
    },
    Tuple {
        local: TupleLocalId,
        value: Vec<EvaluatedValue>,
    },
    List(EvaluatedListCapture),
    IntFunction {
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: EvaluatedListFunction,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedListCapture {
    Int {
        local: crate::plan::execution::IntListLocalId,
        value: super::state::IntListValueId,
    },
    String {
        local: crate::plan::execution::StringListLocalId,
        value: super::state::StringListValueId,
    },
    BitArray {
        local: crate::plan::execution::BitArrayListLocalId,
        value: super::state::BitArrayListValueId,
    },
    UtfCodepoint {
        local: crate::plan::execution::UtfCodepointListLocalId,
        value: super::state::UtfCodepointListValueId,
    },
    Custom {
        local: crate::plan::execution::CustomListLocalId,
        value: super::state::CustomListValueId,
    },
    Float {
        local: crate::plan::execution::FloatListLocalId,
        value: super::state::FloatListValueId,
    },
    Bool {
        local: crate::plan::execution::BoolListLocalId,
        value: super::state::BoolListValueId,
    },
    Nil {
        local: crate::plan::execution::NilListLocalId,
        value: super::state::NilListValueId,
    },
    Tuple {
        local: crate::plan::execution::TupleListLocalId,
        value: super::state::TupleListValueId,
    },
    List {
        local: crate::plan::execution::ListListLocalId,
        value: super::state::ListListValueId,
    },
    Function {
        local: crate::plan::execution::FunctionListLocalId,
        value: super::state::FunctionListValueId,
    },
}

impl EvaluatedValue {
    pub(in crate::runtime) fn value_type(&self, plan: &crate::ExecutionPlan) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom(value) => ValueType::Custom(plan.custom_value_type(value.type_id())),
            Self::Bool(_) => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::Tuple(values) => {
                ValueType::Tuple(values.iter().map(|value| value.value_type(plan)).collect())
            }
            Self::List(value) => plan.list_value_type(value.list_type()),
            Self::Function(value) => {
                ValueType::Function(Box::new(plan.function_type(value.type_())))
            }
        }
    }
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
            identity: EvaluatedFunctionIdentity::Instance(Rc::new(FunctionInstance)),
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
}

impl EvaluatedCustomFunction {
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

    pub(in crate::runtime) fn closure(
        runtime_id: CustomFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<EvaluatedCapture>,
        type_: FunctionType,
    ) -> Self {
        Self::Function(EvaluatedFunction::closure(
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

macro_rules! evaluated_function_value_from {
    ($function:ty, $variant:ident) => {
        impl From<$function> for EvaluatedFunctionValue {
            fn from(value: $function) -> Self {
                Self::from_kind(EvaluatedFunctionValueKind::$variant(value))
            }
        }
    };
}

evaluated_function_value_from!(EvaluatedIntFunction, Int);
evaluated_function_value_from!(EvaluatedFloatFunction, Float);
evaluated_function_value_from!(EvaluatedStringFunction, String);
evaluated_function_value_from!(EvaluatedBitArrayFunction, BitArray);
evaluated_function_value_from!(EvaluatedUtfCodepointFunction, UtfCodepoint);
evaluated_function_value_from!(EvaluatedCustomFunction, Custom);
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
            EvaluatedFunctionValueKind::Int(value) => value.type_(),
            EvaluatedFunctionValueKind::Float(value) => value.type_(),
            EvaluatedFunctionValueKind::String(value) => value.type_(),
            EvaluatedFunctionValueKind::BitArray(value) => value.type_(),
            EvaluatedFunctionValueKind::UtfCodepoint(value) => value.type_(),
            EvaluatedFunctionValueKind::Custom(value) => value.type_(),
            EvaluatedFunctionValueKind::Bool(value) => value.type_(),
            EvaluatedFunctionValueKind::Nil(value) => value.type_(),
            EvaluatedFunctionValueKind::Tuple(value) => value.type_(),
            EvaluatedFunctionValueKind::List(value) => value.type_(),
            EvaluatedFunctionValueKind::Function(value) => value.type_(),
        }
    }

    pub(in crate::runtime) fn with_type(self, type_: FunctionType) -> Self {
        let kind = match self.kind {
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
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Custom(_) => FunctionReturnFamily::Custom,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::List(_) => FunctionReturnFamily::List,
            Self::Function(_) => FunctionReturnFamily::Function,
        }
    }
}

impl EvaluatedCapture {
    pub(in crate::runtime) fn from_kind(kind: EvaluatedCaptureKind) -> Self {
        Self { kind }
    }

    pub(in crate::runtime) fn kind(&self) -> &EvaluatedCaptureKind {
        &self.kind
    }

    pub(in crate::runtime) fn int(local: IntLocalId, value: BigInt) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Int { local, value })
    }

    pub(in crate::runtime) fn float(local: FloatLocalId, value: f64) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Float { local, value })
    }

    pub(in crate::runtime) fn string(local: StringLocalId, value: EcoString) -> Self {
        Self::from_kind(EvaluatedCaptureKind::String { local, value })
    }

    pub(in crate::runtime) fn bit_array(local: BitArrayLocalId, value: EvaluatedBitArray) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BitArray { local, value })
    }

    pub(in crate::runtime) fn utf_codepoint(local: UtfCodepointLocalId, value: char) -> Self {
        Self::from_kind(EvaluatedCaptureKind::UtfCodepoint { local, value })
    }

    pub(in crate::runtime) fn custom(local: CustomLocal, value: EvaluatedCustomValue) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Custom { local, value })
    }

    pub(in crate::runtime) fn bool(local: BoolLocalId, value: bool) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Bool { local, value })
    }

    pub(in crate::runtime) fn nil(local: NilLocalId) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Nil { local })
    }

    pub(in crate::runtime) fn tuple(local: TupleLocalId, value: Vec<EvaluatedValue>) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Tuple { local, value })
    }

    pub(in crate::runtime) fn list(value: EvaluatedListCapture) -> Self {
        Self::from_kind(EvaluatedCaptureKind::List(value))
    }

    pub(in crate::runtime) fn int_function(
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::IntFunction { local, value })
    }

    pub(in crate::runtime) fn float_function(
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FloatFunction { local, value })
    }

    pub(in crate::runtime) fn string_function(
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::StringFunction { local, value })
    }

    pub(in crate::runtime) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BitArrayFunction { local, value })
    }

    pub(in crate::runtime) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::UtfCodepointFunction { local, value })
    }

    pub(in crate::runtime) fn custom_function(
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::CustomFunction { local, value })
    }

    pub(in crate::runtime) fn bool_function(
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BoolFunction { local, value })
    }

    pub(in crate::runtime) fn nil_function(
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::NilFunction { local, value })
    }

    pub(in crate::runtime) fn tuple_function(
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::TupleFunction { local, value })
    }

    pub(in crate::runtime) fn list_function(
        local: ListFunctionLocal,
        value: EvaluatedListFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::ListFunction { local, value })
    }

    pub(in crate::runtime) fn function_function(
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FunctionFunction { local, value })
    }
}

pub(in crate::runtime) fn values_equal(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &EvaluatedValue,
    right: &EvaluatedValue,
) -> bool {
    match (left, right) {
        (EvaluatedValue::Int(left), EvaluatedValue::Int(right)) => left == right,
        (EvaluatedValue::Float(left), EvaluatedValue::Float(right)) => left == right,
        (EvaluatedValue::String(left), EvaluatedValue::String(right)) => left == right,
        (EvaluatedValue::BitArray(left), EvaluatedValue::BitArray(right)) => left == right,
        (EvaluatedValue::UtfCodepoint(left), EvaluatedValue::UtfCodepoint(right)) => left == right,
        (EvaluatedValue::Custom(left), EvaluatedValue::Custom(right)) => {
            left.constructor == right.constructor
                && left.fields.len() == right.fields.len()
                && left
                    .fields
                    .iter()
                    .zip(&right.fields)
                    .all(|(left, right)| values_equal(plan, state, left, right))
        }
        (EvaluatedValue::Bool(left), EvaluatedValue::Bool(right)) => left == right,
        (EvaluatedValue::Nil, EvaluatedValue::Nil) => true,
        (EvaluatedValue::Tuple(left), EvaluatedValue::Tuple(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| values_equal(plan, state, left, right))
        }
        (EvaluatedValue::List(left), EvaluatedValue::List(right)) => {
            lists_equal(plan, state, left, right)
        }
        (EvaluatedValue::Function(left), EvaluatedValue::Function(right)) => {
            functions_equal(left, right)
        }
        _ => false,
    }
}

fn lists_equal(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &ListValueId,
    right: &ListValueId,
) -> bool {
    if left.list_type() != right.list_type() {
        return false;
    }

    let left = state.evaluated_values(plan, left);
    let right = state.evaluated_values(plan, right);
    left.len() == right.len()
        && left
            .iter()
            .zip(&right)
            .all(|(left, right)| values_equal(plan, state, left, right))
}

fn functions_equal(left: &EvaluatedFunctionValue, right: &EvaluatedFunctionValue) -> bool {
    match (left.kind(), right.kind()) {
        (EvaluatedFunctionValueKind::Int(left), EvaluatedFunctionValueKind::Int(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Float(left), EvaluatedFunctionValueKind::Float(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::String(left), EvaluatedFunctionValueKind::String(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedFunctionValueKind::BitArray(left),
            EvaluatedFunctionValueKind::BitArray(right),
        ) => function_values_equal(left, right),
        (
            EvaluatedFunctionValueKind::UtfCodepoint(left),
            EvaluatedFunctionValueKind::UtfCodepoint(right),
        ) => function_values_equal(left, right),
        (EvaluatedFunctionValueKind::Custom(left), EvaluatedFunctionValueKind::Custom(right)) => {
            custom_function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Bool(left), EvaluatedFunctionValueKind::Bool(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Nil(left), EvaluatedFunctionValueKind::Nil(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::Tuple(left), EvaluatedFunctionValueKind::Tuple(right)) => {
            function_values_equal(left, right)
        }
        (EvaluatedFunctionValueKind::List(left), EvaluatedFunctionValueKind::List(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedFunctionValueKind::Function(left),
            EvaluatedFunctionValueKind::Function(right),
        ) => function_values_equal(left, right),
        _ => false,
    }
}

fn function_values_equal<Id>(left: &EvaluatedFunction<Id>, right: &EvaluatedFunction<Id>) -> bool {
    left.identity == right.identity
}

fn custom_function_values_equal(
    left: &EvaluatedCustomFunction,
    right: &EvaluatedCustomFunction,
) -> bool {
    match (left, right) {
        (EvaluatedCustomFunction::Function(left), EvaluatedCustomFunction::Function(right)) => {
            function_values_equal(left, right)
        }
        (
            EvaluatedCustomFunction::Constructor(left),
            EvaluatedCustomFunction::Constructor(right),
        ) => function_values_equal(left, right),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
        EvaluatedCustomFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
        EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNilFunction,
        EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction,
        EvaluatedValue, FunctionReferenceId, FunctionReferenceIdentity, ListFunctionReturnFamily,
        values_equal,
    };
    use crate::plan::ValueType;
    use crate::plan::execution::{
        BitArrayFunctionId, BitArrayListFunctionFunctionId, BoolFunctionId,
        BoolListFunctionFunctionId, CustomListFunctionFunctionId, FloatFunctionId,
        FloatListFunctionFunctionId, FunctionFunctionId, FunctionListFunctionFunctionId,
        IntFunctionFunctionId, IntFunctionId, IntListFunctionFunctionId, IntLocalId,
        ListFunctionFunctionId, ListFunctionId, ListListFunctionFunctionId, NilFunctionId,
        NilListFunctionFunctionId, ParamLocal, StringFunctionId, StringListFunctionFunctionId,
        TupleFunctionId, TupleListFunctionFunctionId, UtfCodepointFunctionId,
        UtfCodepointListFunctionFunctionId,
    };
    use crate::runtime::state::{ListValueId, RuntimeState};
    use bitvec::order::Msb0;
    use bitvec::view::BitView;

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
fn take_function_function(value: fn() -> fn() -> Int) { 0 }
pub fn main() { 0 }
"#;

    #[test]
    fn evaluated_bit_array_aligns_owned_slices() {
        let source = [0x77u8];
        let value = EvaluatedBitArray::new(source.view_bits::<Msb0>()[4..6].to_bitvec());

        assert_eq!(value.bits.as_raw_slice(), &[0b0100_0000]);
        assert_eq!(value.bits.len(), 2);
    }

    #[test]
    fn evaluated_value_type_preserves_every_runtime_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );
        let values = [
            EvaluatedValue::Int(1.into()),
            EvaluatedValue::Float(1.5),
            EvaluatedValue::String("one".into()),
            EvaluatedValue::BitArray(EvaluatedBitArray::new(bitvec::vec::BitVec::new())),
            EvaluatedValue::UtfCodepoint('\u{10ffff}'),
            EvaluatedValue::Bool(true),
            EvaluatedValue::Nil,
            EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            EvaluatedValue::List(ListValueId::Int(list)),
            EvaluatedValue::Function(EvaluatedFunctionValue::from(function)),
        ];
        let expected = [
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::Int,
            ))),
        ];

        for (value, expected) in values.iter().zip(expected) {
            assert_eq!(value.value_type(&plan), expected);
        }
    }

    #[test]
    fn semantic_value_equality_covers_every_list_and_function_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let execution_int_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let int_function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let custom_type = plan.custom_list_function_id(0).type_id().item_type();
        let custom_function = EvaluatedCustomFunction::reference(
            plan.custom_function_id(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Custom(custom_type),
            ),
        );
        let constructor_id = plan.custom_constructor_id(0, 0);
        let constructor = plan.custom_constructor(constructor_id);
        let constructor_function = EvaluatedCustomFunction::constructor(
            constructor_id,
            crate::plan::execution::FunctionType::new(
                constructor
                    .fields()
                    .iter()
                    .map(|field| field.type_().clone())
                    .collect(),
                crate::plan::execution::ValueType::Custom(constructor_id.type_id()),
            ),
        );
        let function_pairs = [
            (
                EvaluatedFunctionValue::from(int_function.clone()),
                EvaluatedFunctionValue::from(int_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Float,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::reference(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Float,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::String,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedStringFunction::reference(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::String,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::BitArray,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::reference(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::BitArray,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::UtfCodepoint,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedUtfCodepointFunction::reference(
                    UtfCodepointFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::UtfCodepoint,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(custom_function.clone()),
                EvaluatedFunctionValue::from(custom_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(constructor_function.clone()),
                EvaluatedFunctionValue::from(constructor_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Bool,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::reference(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Bool,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Nil,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedNilFunction::reference(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Nil,
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Tuple(vec![
                            crate::plan::execution::ValueType::Int,
                        ]),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::reference(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Tuple(vec![
                            crate::plan::execution::ValueType::Int,
                        ]),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    ListFunctionId::Int(plan.int_list_function_id(0)),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedListFunction::reference(
                    ListFunctionId::Int(plan.int_list_function_id(0)),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::List(
                            plan.int_list_function_id(0).type_id().list_type(),
                        ),
                    ),
                )),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::reference(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Function(Box::new(
                            execution_int_type.clone(),
                        )),
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::reference(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Function(Box::new(
                            execution_int_type.clone(),
                        )),
                    ),
                )),
            ),
        ];

        for (left, right) in function_pairs {
            let family = left.kind().family();
            assert_eq!(family, right.kind().family());
            assert_eq!(
                values_equal(
                    &plan,
                    &state,
                    &EvaluatedValue::Function(left),
                    &EvaluatedValue::Function(right),
                ),
                true,
            );
        }
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(custom_function)),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(constructor_function)),
            ),
            false,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(int_function.clone())),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                    EvaluatedFloatFunction::reference(
                        FloatFunctionId(0),
                        Vec::new(),
                        Vec::new(),
                        crate::plan::execution::FunctionType::new(
                            Vec::new(),
                            crate::plan::execution::ValueType::Float,
                        ),
                    ),
                )),
            ),
            false,
        );

        let int_lists = (
            state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
            state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
        );
        let string_lists = (
            state.string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
            state.string(
                plan.string_list_function_id(0).type_id(),
                vec!["one".into()],
            ),
        );
        let float_lists = (
            state.float(plan.float_list_function_id(0).type_id(), vec![1.5]),
            state.float(plan.float_list_function_id(0).type_id(), vec![1.5]),
        );
        let utf_codepoint_lists = (
            state.utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
            state.utf_codepoint(plan.utf_codepoint_list_function_id(0).type_id(), vec!['a']),
        );
        let bool_lists = (
            state.bool(plan.bool_list_function_id(0).type_id(), vec![true]),
            state.bool(plan.bool_list_function_id(0).type_id(), vec![true]),
        );
        let nil_lists = (
            state.nil(plan.nil_list_function_id(0).type_id(), 1),
            state.nil(plan.nil_list_function_id(0).type_id(), 1),
        );
        let tuple_lists = (
            state.tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
            state.tuple(
                plan.tuple_list_function_id(0).type_id(),
                vec![vec![EvaluatedValue::Int(1.into())]],
            ),
        );
        let left_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let right_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_lists = (
            state.list(
                plan.list_list_function_id(0).type_id(),
                vec![left_child.into_core()],
            ),
            state.list(
                plan.list_list_function_id(0).type_id(),
                vec![right_child.into_core()],
            ),
        );
        let function_lists = (
            state.function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
            state.function(
                plan.function_list_function_id(0).type_id(),
                vec![EvaluatedFunctionValue::from(int_function.clone())],
            ),
        );
        let list_pairs = [
            (
                ListValueId::Int(int_lists.0.clone()),
                ListValueId::Int(int_lists.1.clone()),
            ),
            (
                ListValueId::String(string_lists.0.clone()),
                ListValueId::String(string_lists.1.clone()),
            ),
            (
                ListValueId::UtfCodepoint(utf_codepoint_lists.0.clone()),
                ListValueId::UtfCodepoint(utf_codepoint_lists.1.clone()),
            ),
            (
                ListValueId::Float(float_lists.0.clone()),
                ListValueId::Float(float_lists.1.clone()),
            ),
            (
                ListValueId::Bool(bool_lists.0.clone()),
                ListValueId::Bool(bool_lists.1.clone()),
            ),
            (
                ListValueId::Nil(nil_lists.0.clone()),
                ListValueId::Nil(nil_lists.1.clone()),
            ),
            (
                ListValueId::Tuple(tuple_lists.0.clone()),
                ListValueId::Tuple(tuple_lists.1.clone()),
            ),
            (
                ListValueId::List(nested_lists.0.clone()),
                ListValueId::List(nested_lists.1.clone()),
            ),
            (
                ListValueId::Function(function_lists.0.clone()),
                ListValueId::Function(function_lists.1.clone()),
            ),
        ];

        for (left, right) in list_pairs {
            assert_eq!(
                values_equal(
                    &plan,
                    &state,
                    &EvaluatedValue::List(left),
                    &EvaluatedValue::List(right),
                ),
                true,
            );
        }
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::List(ListValueId::Int(int_lists.0)),
                &EvaluatedValue::List(ListValueId::String(string_lists.0)),
            ),
            false,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            ),
            true,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &EvaluatedValue::Tuple(Vec::new()),
            ),
            false,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Int(1.into()),
                &EvaluatedValue::String("one".into()),
            ),
            false,
        );
    }

    #[test]
    fn function_identity_distinguishes_references_and_instances() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let state = RuntimeState::new();
        let int_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
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
            crate::plan::execution::FunctionType::new(
                vec![crate::plan::execution::ValueType::Int],
                crate::plan::execution::ValueType::Int,
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

        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(reference.clone())),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                    same_target_with_different_metadata,
                )),
            ),
            true,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(reference)),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(different_target)),
            ),
            false,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                    reference_for_instance_comparison,
                )),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure.clone())),
            ),
            false,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure.clone())),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(same_closure)),
            ),
            true,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(closure)),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(separate_closure)),
            ),
            false,
        );
    }

    #[test]
    fn list_function_reference_identity_uses_item_family_table_target() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let ids = [
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
        let type_ = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let ids = [
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
        let state = RuntimeState::new();
        let constructor_id = plan.custom_constructor_id(0, 0);
        let constructor = plan.custom_constructor(constructor_id);
        let type_ = crate::plan::execution::FunctionType::new(
            constructor
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect(),
            crate::plan::execution::ValueType::Custom(constructor_id.type_id()),
        );
        let first = EvaluatedCustomFunction::constructor(constructor_id, type_.clone());
        let same = first.clone();
        let separate = EvaluatedCustomFunction::constructor(constructor_id, type_);

        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(first.clone())),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(same)),
            ),
            true,
        );
        assert_eq!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(first)),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(separate)),
            ),
            false,
        );
    }
}
