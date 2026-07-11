use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, IntFunctionValue,
    ListFunctionValue, ListLocalValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue,
    Value,
};
use crate::plan::execution::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureValue {
    kind: CaptureValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureValueKind {
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
    Bool {
        local: BoolLocalId,
        value: bool,
    },
    Nil {
        local: NilLocalId,
    },
    Tuple {
        local: TupleLocalId,
        value: Vec<Value>,
    },
    List(ListLocalValue),
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionValue,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionValue,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionValue,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionValue,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionValue,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: ListFunctionValue,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    },
}

impl CaptureValue {
    pub(crate) fn int(local: IntLocalId, value: BigInt) -> Self {
        Self {
            kind: CaptureValueKind::Int { local, value },
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: f64) -> Self {
        Self {
            kind: CaptureValueKind::Float { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: EcoString) -> Self {
        Self {
            kind: CaptureValueKind::String { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: bool) -> Self {
        Self {
            kind: CaptureValueKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId) -> Self {
        Self {
            kind: CaptureValueKind::Nil { local },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: Vec<Value>) -> Self {
        Self {
            kind: CaptureValueKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(value: ListLocalValue) -> Self {
        Self {
            kind: CaptureValueKind::List(value),
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::IntFunction { local, value },
        }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn string_function(
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::StringFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function(local: ListFunctionLocal, value: ListFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CaptureValueKind {
        &self.kind
    }
}
