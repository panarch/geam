use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use ecow::EcoString;
use num_bigint::BigInt;
use std::rc::Rc;

use super::state::{ListValueId, RuntimeState};
use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionId,
    BoolFunctionLocalId, BoolLocalId, FloatFunctionId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionId, FunctionFunctionLocalId, FunctionReturnFamily, FunctionType, IntFunctionId,
    IntFunctionLocalId, IntLocalId, ListFunctionId, ListFunctionLocal, NilFunctionId,
    NilFunctionLocalId, NilLocalId, ParamLocal, StringFunctionId, StringFunctionLocalId,
    StringLocalId, TupleFunctionId, TupleFunctionLocalId, TupleLocalId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::runtime) struct EvaluatedBitArray {
    bits: Rc<BitVec<u8, Msb0>>,
}

impl EvaluatedBitArray {
    pub(in crate::runtime) fn new(bits: BitVec<u8, Msb0>) -> Self {
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
    Bool(bool),
    Nil,
    Tuple(Vec<EvaluatedValue>),
    List(ListValueId),
    Function(EvaluatedFunctionValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedFunction<Id> {
    runtime_id: Id,
    params: Vec<ParamLocal>,
    captures: Vec<EvaluatedCapture>,
    type_: FunctionType,
}

pub(in crate::runtime) type EvaluatedIntFunction = EvaluatedFunction<IntFunctionId>;
pub(in crate::runtime) type EvaluatedFloatFunction = EvaluatedFunction<FloatFunctionId>;
pub(in crate::runtime) type EvaluatedStringFunction = EvaluatedFunction<StringFunctionId>;
pub(in crate::runtime) type EvaluatedBitArrayFunction = EvaluatedFunction<BitArrayFunctionId>;
pub(in crate::runtime) type EvaluatedBoolFunction = EvaluatedFunction<BoolFunctionId>;
pub(in crate::runtime) type EvaluatedNilFunction = EvaluatedFunction<NilFunctionId>;
pub(in crate::runtime) type EvaluatedTupleFunction = EvaluatedFunction<TupleFunctionId>;
pub(in crate::runtime) type EvaluatedListFunction = EvaluatedFunction<ListFunctionId>;
pub(in crate::runtime) type EvaluatedFunctionFunction = EvaluatedFunction<FunctionFunctionId>;

pub(in crate::runtime) fn function_type(
    params: &[ParamLocal],
    return_: crate::plan::execution::ValueType,
) -> FunctionType {
    FunctionType::new(params.iter().map(ParamLocal::value_type).collect(), return_)
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
        local: FunctionFunctionLocalId,
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

impl<Id: Clone> EvaluatedFunction<Id> {
    pub(in crate::runtime) fn new(
        runtime_id: Id,
        params: Vec<ParamLocal>,
        captures: Vec<EvaluatedCapture>,
        type_: FunctionType,
    ) -> Self {
        Self {
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
            EvaluatedFunctionValueKind::Bool(value) => value.type_(),
            EvaluatedFunctionValueKind::Nil(value) => value.type_(),
            EvaluatedFunctionValueKind::Tuple(value) => value.type_(),
            EvaluatedFunctionValueKind::List(value) => value.type_(),
            EvaluatedFunctionValueKind::Function(value) => value.type_(),
        }
    }
}

impl EvaluatedFunctionValueKind {
    pub(in crate::runtime) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
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
        local: FunctionFunctionLocalId,
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
            functions_equal(plan, state, left, right)
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

fn functions_equal(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &EvaluatedFunctionValue,
    right: &EvaluatedFunctionValue,
) -> bool {
    match (left.kind(), right.kind()) {
        (EvaluatedFunctionValueKind::Int(left), EvaluatedFunctionValueKind::Int(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (EvaluatedFunctionValueKind::Float(left), EvaluatedFunctionValueKind::Float(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (EvaluatedFunctionValueKind::String(left), EvaluatedFunctionValueKind::String(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (
            EvaluatedFunctionValueKind::BitArray(left),
            EvaluatedFunctionValueKind::BitArray(right),
        ) => function_values_equal(plan, state, left, right),
        (EvaluatedFunctionValueKind::Bool(left), EvaluatedFunctionValueKind::Bool(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (EvaluatedFunctionValueKind::Nil(left), EvaluatedFunctionValueKind::Nil(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (EvaluatedFunctionValueKind::Tuple(left), EvaluatedFunctionValueKind::Tuple(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (EvaluatedFunctionValueKind::List(left), EvaluatedFunctionValueKind::List(right)) => {
            function_values_equal(plan, state, left, right)
        }
        (
            EvaluatedFunctionValueKind::Function(left),
            EvaluatedFunctionValueKind::Function(right),
        ) => function_values_equal(plan, state, left, right),
        _ => false,
    }
}

fn function_values_equal<Id: PartialEq>(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &EvaluatedFunction<Id>,
    right: &EvaluatedFunction<Id>,
) -> bool {
    left.runtime_id == right.runtime_id
        && left.params == right.params
        && left.type_ == right.type_
        && left.captures.len() == right.captures.len()
        && left
            .captures
            .iter()
            .zip(&right.captures)
            .all(|(left, right)| captures_equal(plan, state, left, right))
}

fn captures_equal(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &EvaluatedCapture,
    right: &EvaluatedCapture,
) -> bool {
    match (left.kind(), right.kind()) {
        (
            EvaluatedCaptureKind::Int {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::Int {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && left == right,
        (
            EvaluatedCaptureKind::Float {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::Float {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && left == right,
        (
            EvaluatedCaptureKind::String {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::String {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && left == right,
        (
            EvaluatedCaptureKind::Bool {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::Bool {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && left == right,
        (EvaluatedCaptureKind::Nil { local: left }, EvaluatedCaptureKind::Nil { local: right }) => {
            left == right
        }
        (
            EvaluatedCaptureKind::Tuple {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::Tuple {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && values_equal(
                    plan,
                    state,
                    &EvaluatedValue::Tuple(left.clone()),
                    &EvaluatedValue::Tuple(right.clone()),
                )
        }
        (EvaluatedCaptureKind::List(left), EvaluatedCaptureKind::List(right)) => {
            list_captures_equal(plan, state, left, right)
        }
        (
            EvaluatedCaptureKind::IntFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::IntFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::FloatFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::FloatFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::StringFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::StringFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::BoolFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::BoolFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::NilFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::NilFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::TupleFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::TupleFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::ListFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::ListFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        (
            EvaluatedCaptureKind::FunctionFunction {
                local: left_local,
                value: left,
            },
            EvaluatedCaptureKind::FunctionFunction {
                local: right_local,
                value: right,
            },
        ) => left_local == right_local && function_values_equal(plan, state, left, right),
        _ => false,
    }
}

fn list_captures_equal(
    plan: &crate::ExecutionPlan,
    state: &RuntimeState,
    left: &EvaluatedListCapture,
    right: &EvaluatedListCapture,
) -> bool {
    match (left, right) {
        (
            EvaluatedListCapture::Int {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Int {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::String {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::String {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::Float {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Float {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::Bool {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Bool {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::Nil {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Nil {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::Tuple {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Tuple {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::List {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::List {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        (
            EvaluatedListCapture::Function {
                local: left_local,
                value: left,
            },
            EvaluatedListCapture::Function {
                local: right_local,
                value: right,
            },
        ) => {
            left_local == right_local
                && lists_equal(plan, state, &left.clone().into(), &right.clone().into())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
        EvaluatedFloatFunction, EvaluatedFunctionFunction, EvaluatedFunctionValue,
        EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction,
        EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedValue, values_equal,
    };
    use crate::plan::ValueType;
    use crate::plan::execution::{
        BitArrayFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId,
        FloatFunctionId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionListLocalId, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionId,
        ListFunctionLocal, ListListLocalId, NilFunctionId, NilFunctionLocalId, NilListLocalId,
        NilLocalId, ParamLocal, StringFunctionId, StringFunctionLocalId, StringListLocalId,
        StringLocalId, TupleFunctionId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    };
    use crate::runtime::state::{ListValueId, RuntimeState};

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() { 0 }
"#;

    #[test]
    fn evaluated_value_type_preserves_every_runtime_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let function = EvaluatedIntFunction::new(
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
        let int_function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let function_pairs = [
            (
                EvaluatedFunctionValue::from(int_function.clone()),
                EvaluatedFunctionValue::from(int_function.clone()),
            ),
            (
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::new(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Float,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedFloatFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedStringFunction::new(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::String,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedStringFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::new(
                    BitArrayFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::BitArray,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBitArrayFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::new(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Bool,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedBoolFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedNilFunction::new(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    crate::plan::execution::FunctionType::new(
                        Vec::new(),
                        crate::plan::execution::ValueType::Nil,
                    ),
                )),
                EvaluatedFunctionValue::from(EvaluatedNilFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedTupleFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedListFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedListFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::new(
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
                EvaluatedFunctionValue::from(EvaluatedFunctionFunction::new(
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
            assert!(
                values_equal(
                    &plan,
                    &state,
                    &EvaluatedValue::Function(left),
                    &EvaluatedValue::Function(right),
                ),
                "matching function families must compare equal",
            );
        }
        assert!(
            !values_equal(
                &plan,
                &state,
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(int_function.clone())),
                &EvaluatedValue::Function(EvaluatedFunctionValue::from(
                    EvaluatedFloatFunction::new(
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
            "different function families must not compare equal",
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
            assert!(
                values_equal(
                    &plan,
                    &state,
                    &EvaluatedValue::List(left),
                    &EvaluatedValue::List(right),
                ),
                "matching list families must compare equal",
            );
        }
        assert!(
            !values_equal(
                &plan,
                &state,
                &EvaluatedValue::List(ListValueId::Int(int_lists.0)),
                &EvaluatedValue::List(ListValueId::String(string_lists.0)),
            ),
            "different list families must not compare equal",
        );
        assert!(
            values_equal(
                &plan,
                &state,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
            ),
            "matching tuple values must compare equal",
        );
        assert!(
            !values_equal(
                &plan,
                &state,
                &EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                &EvaluatedValue::Tuple(Vec::new()),
            ),
            "different tuple lengths must not compare equal",
        );
        assert!(
            !values_equal(
                &plan,
                &state,
                &EvaluatedValue::Int(1.into()),
                &EvaluatedValue::String("one".into()),
            ),
            "different value families must not compare equal",
        );
    }

    #[test]
    fn semantic_function_equality_covers_every_capture_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let execution_int_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let captured_int_function = EvaluatedIntFunction::new(
            IntFunctionId(1),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let captured_float_function = EvaluatedFloatFunction::new(
            FloatFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Float,
            ),
        );
        let captured_string_function = EvaluatedStringFunction::new(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::String,
            ),
        );
        let captured_bool_function = EvaluatedBoolFunction::new(
            BoolFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Bool,
            ),
        );
        let captured_nil_function = EvaluatedNilFunction::new(
            NilFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Nil,
            ),
        );
        let captured_tuple_function = EvaluatedTupleFunction::new(
            TupleFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Tuple(vec![
                    crate::plan::execution::ValueType::Int,
                ]),
            ),
        );
        let list_function_id = ListFunctionId::Int(plan.int_list_function_id(0));
        let captured_list_function = EvaluatedListFunction::new(
            list_function_id.clone(),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::List(
                    plan.int_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let captured_function_function = EvaluatedFunctionFunction::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Function(Box::new(execution_int_type.clone())),
            ),
        );
        let left_int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let right_int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let left_string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let right_string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let left_float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let right_float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let left_bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let right_bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let left_nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let right_nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let left_tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let right_tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let left_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let right_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let left_nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![left_child.into_core()],
        );
        let right_nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![right_child.into_core()],
        );
        let left_function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(captured_int_function.clone())],
        );
        let right_function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(captured_int_function.clone())],
        );
        let list_function_local = ListFunctionLocal::Int {
            local: IntListFunctionLocalId(0),
            type_: execution_int_type.clone(),
            list_type: plan.int_list_function_id(0).type_id(),
        };
        let captures = [
            (
                EvaluatedCapture::int(IntLocalId(0), 1.into()),
                EvaluatedCapture::int(IntLocalId(0), 1.into()),
            ),
            (
                EvaluatedCapture::float(FloatLocalId(0), 1.5),
                EvaluatedCapture::float(FloatLocalId(0), 1.5),
            ),
            (
                EvaluatedCapture::string(StringLocalId(0), "one".into()),
                EvaluatedCapture::string(StringLocalId(0), "one".into()),
            ),
            (
                EvaluatedCapture::bool(BoolLocalId(0), true),
                EvaluatedCapture::bool(BoolLocalId(0), true),
            ),
            (
                EvaluatedCapture::nil(NilLocalId(0)),
                EvaluatedCapture::nil(NilLocalId(0)),
            ),
            (
                EvaluatedCapture::tuple(TupleLocalId(0), vec![EvaluatedValue::Int(1.into())]),
                EvaluatedCapture::tuple(TupleLocalId(0), vec![EvaluatedValue::Int(1.into())]),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Int {
                    local: IntListLocalId(0),
                    value: left_int_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Int {
                    local: IntListLocalId(0),
                    value: right_int_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::String {
                    local: StringListLocalId(0),
                    value: left_string_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::String {
                    local: StringListLocalId(0),
                    value: right_string_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Float {
                    local: FloatListLocalId(0),
                    value: left_float_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Float {
                    local: FloatListLocalId(0),
                    value: right_float_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Bool {
                    local: BoolListLocalId(0),
                    value: left_bool_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Bool {
                    local: BoolListLocalId(0),
                    value: right_bool_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Nil {
                    local: NilListLocalId(0),
                    value: left_nil_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Nil {
                    local: NilListLocalId(0),
                    value: right_nil_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                    local: TupleListLocalId(0),
                    value: left_tuple_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                    local: TupleListLocalId(0),
                    value: right_tuple_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::List {
                    local: ListListLocalId(0),
                    value: left_nested_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::List {
                    local: ListListLocalId(0),
                    value: right_nested_list,
                }),
            ),
            (
                EvaluatedCapture::list(EvaluatedListCapture::Function {
                    local: FunctionListLocalId(0),
                    value: left_function_list,
                }),
                EvaluatedCapture::list(EvaluatedListCapture::Function {
                    local: FunctionListLocalId(0),
                    value: right_function_list,
                }),
            ),
            (
                EvaluatedCapture::int_function(
                    IntFunctionLocalId(0),
                    captured_int_function.clone(),
                ),
                EvaluatedCapture::int_function(
                    IntFunctionLocalId(0),
                    captured_int_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::float_function(
                    FloatFunctionLocalId(0),
                    captured_float_function.clone(),
                ),
                EvaluatedCapture::float_function(
                    FloatFunctionLocalId(0),
                    captured_float_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::string_function(
                    StringFunctionLocalId(0),
                    captured_string_function.clone(),
                ),
                EvaluatedCapture::string_function(
                    StringFunctionLocalId(0),
                    captured_string_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::bool_function(
                    BoolFunctionLocalId(0),
                    captured_bool_function.clone(),
                ),
                EvaluatedCapture::bool_function(
                    BoolFunctionLocalId(0),
                    captured_bool_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::nil_function(
                    NilFunctionLocalId(0),
                    captured_nil_function.clone(),
                ),
                EvaluatedCapture::nil_function(
                    NilFunctionLocalId(0),
                    captured_nil_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::tuple_function(
                    TupleFunctionLocalId(0),
                    captured_tuple_function.clone(),
                ),
                EvaluatedCapture::tuple_function(
                    TupleFunctionLocalId(0),
                    captured_tuple_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::list_function(
                    list_function_local.clone(),
                    captured_list_function.clone(),
                ),
                EvaluatedCapture::list_function(
                    list_function_local,
                    captured_list_function.clone(),
                ),
            ),
            (
                EvaluatedCapture::function_function(
                    FunctionFunctionLocalId(0),
                    captured_function_function.clone(),
                ),
                EvaluatedCapture::function_function(
                    FunctionFunctionLocalId(0),
                    captured_function_function,
                ),
            ),
        ];

        for (left_capture, right_capture) in captures {
            let left =
                EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                    IntFunctionId(0),
                    Vec::new(),
                    vec![left_capture],
                    execution_int_type.clone(),
                )));
            let right =
                EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                    IntFunctionId(0),
                    Vec::new(),
                    vec![right_capture],
                    execution_int_type.clone(),
                )));
            assert!(
                values_equal(&plan, &state, &left, &right),
                "matching capture families must compare equal",
            );
        }

        let mismatched_capture =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(0),
                Vec::new(),
                vec![EvaluatedCapture::int(IntLocalId(0), 1.into())],
                execution_int_type.clone(),
            )));
        let mismatched_kind =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(0),
                Vec::new(),
                vec![EvaluatedCapture::string(StringLocalId(0), "one".into())],
                execution_int_type.clone(),
            )));
        assert!(
            !values_equal(&plan, &state, &mismatched_capture, &mismatched_kind,),
            "different capture families must not compare equal",
        );

        let mismatched_list_capture =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(0),
                Vec::new(),
                vec![EvaluatedCapture::list(EvaluatedListCapture::Int {
                    local: IntListLocalId(0),
                    value: state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]),
                })],
                execution_int_type.clone(),
            )));
        let mismatched_list_capture_kind =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(0),
                Vec::new(),
                vec![EvaluatedCapture::list(EvaluatedListCapture::String {
                    local: StringListLocalId(0),
                    value: state.string(
                        plan.string_list_function_id(0).type_id(),
                        vec!["one".into()],
                    ),
                })],
                execution_int_type.clone(),
            )));
        assert!(
            !values_equal(
                &plan,
                &state,
                &mismatched_list_capture,
                &mismatched_list_capture_kind,
            ),
            "different captured list families must not compare equal",
        );

        let different_id =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(1),
                Vec::new(),
                Vec::new(),
                execution_int_type.clone(),
            )));
        let different_params =
            EvaluatedValue::Function(EvaluatedFunctionValue::from(EvaluatedIntFunction::new(
                IntFunctionId(0),
                vec![ParamLocal::Int(IntLocalId(0))],
                Vec::new(),
                crate::plan::execution::FunctionType::new(
                    vec![crate::plan::execution::ValueType::Int],
                    crate::plan::execution::ValueType::Int,
                ),
            )));
        let base = EvaluatedValue::Function(EvaluatedFunctionValue::from(
            EvaluatedIntFunction::new(IntFunctionId(0), Vec::new(), Vec::new(), execution_int_type),
        ));
        assert!(
            !values_equal(&plan, &state, &base, &different_id),
            "different function ids must not compare equal",
        );
        assert!(
            !values_equal(&plan, &state, &base, &different_params),
            "different function parameters must not compare equal",
        );
    }
}
