use super::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomFunctionExpr,
    CustomLocalExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr,
    ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
    TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use crate::plan::execution::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocal,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};

pub struct CallArg {
    kind: CallArgKind,
}

pub(crate) enum CallArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    Custom(CustomLocalExpr),
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List(ListLocalExpr),
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionExpr,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: CustomFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: FunctionFunctionExpr,
    },
}

pub(crate) struct CaptureArg {
    kind: CaptureArgKind,
}

pub(crate) enum CaptureArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    Custom(CustomLocalExpr),
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List(ListLocalExpr),
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionExpr,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: CustomFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: FunctionFunctionExpr,
    },
}

impl CallArg {
    pub(in crate::plan::execution) fn from_kind(kind: CallArgKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &CallArgKind {
        &self.kind
    }
}

impl CaptureArg {
    pub(in crate::plan::execution) fn from_kind(kind: CaptureArgKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &CaptureArgKind {
        &self.kind
    }
}
