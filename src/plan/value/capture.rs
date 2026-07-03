use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, IntFunctionValue,
    ListFunctionValue, ListValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue, Value,
};
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocalId, ListLocalId, NilFunctionLocalId,
    NilLocalId, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
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
    List {
        local: ListLocalId,
        value: ListValue,
    },
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
        local: ListFunctionLocalId,
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

    pub(crate) fn list(local: ListLocalId, value: ListValue) -> Self {
        Self {
            kind: CaptureValueKind::List { local, value },
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

    pub(crate) fn list_function(local: ListFunctionLocalId, value: ListFunctionValue) -> Self {
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

#[cfg(test)]
mod tests {
    use super::{CaptureValue, CaptureValueKind};
    use crate::plan::{
        FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue, FloatLocalId, ListFunctionId,
        ListFunctionLocalId, ListFunctionValue, ListLocalId, ListValue, ParamLocal,
        TupleFunctionId, TupleFunctionLocalId, TupleFunctionValue, TupleLocalId, Value, ValueType,
    };

    #[test]
    fn capture_value_preserves_float_function_shape() {
        let value = CaptureValue::float_function(
            FloatFunctionLocalId(0),
            FloatFunctionValue::new(FloatFunctionId(0), vec![float_param(0)]),
        );

        assert!(matches!(
            value.kind(),
            CaptureValueKind::FloatFunction { .. }
        ));
    }

    #[test]
    fn capture_value_preserves_tuple_shapes() {
        let tuple = CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]);
        assert!(matches!(tuple.kind(), CaptureValueKind::Tuple { .. }));

        let function = CaptureValue::tuple_function(
            TupleFunctionLocalId(0),
            TupleFunctionValue::new(
                TupleFunctionId(0),
                vec![tuple_param(0)],
                vec![ValueType::Int],
            ),
        );
        assert!(matches!(
            function.kind(),
            CaptureValueKind::TupleFunction { .. }
        ));
    }

    #[test]
    fn capture_value_preserves_list_shapes() {
        let list = CaptureValue::list(
            ListLocalId(0),
            ListValue::new(ValueType::Int, vec![Value::Int(1.into())]),
        );
        assert!(matches!(list.kind(), CaptureValueKind::List { .. }));

        let function = CaptureValue::list_function(
            ListFunctionLocalId(0),
            ListFunctionValue::new(ListFunctionId(0), vec![list_param(0)], ValueType::Int),
        );
        assert!(matches!(
            function.kind(),
            CaptureValueKind::ListFunction { .. }
        ));
    }

    fn float_param(index: usize) -> ParamLocal {
        ParamLocal::float(FloatLocalId(index))
    }

    fn tuple_param(index: usize) -> ParamLocal {
        ParamLocal::tuple(TupleLocalId(index), vec![ValueType::Int])
    }

    fn list_param(index: usize) -> ParamLocal {
        ParamLocal::list(ListLocalId(index), ValueType::Int)
    }
}
