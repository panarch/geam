use super::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr,
};
use crate::plan::execution::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
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
        local: FunctionFunctionLocalId,
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
        local: FunctionFunctionLocalId,
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
