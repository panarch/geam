use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    BitArrayFunctionValue, BitArrayValue, BoolFunctionValue, FloatFunctionValue,
    FunctionFunctionValue, IntFunctionValue, ListFunctionValue, ListValue, NilFunctionValue,
    StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue, Value,
};
use crate::plan::execution::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionFunctionLocalId, FunctionListLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, NilFunctionLocalId, NilListLocalId, NilLocalId,
    StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use crate::plan::{FunctionType, ValueType};

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
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayValue,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: char,
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
    List(CaptureListValue),
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
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionValue,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionValue,
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureListValue {
    Int {
        local: IntListLocalId,
        value: Vec<BigInt>,
    },
    String {
        local: StringListLocalId,
        value: Vec<EcoString>,
    },
    BitArray {
        local: BitArrayListLocalId,
        value: Vec<BitArrayValue>,
    },
    UtfCodepoint {
        local: UtfCodepointListLocalId,
        value: Vec<char>,
    },
    Float {
        local: FloatListLocalId,
        value: Vec<f64>,
    },
    Bool {
        local: BoolListLocalId,
        value: Vec<bool>,
    },
    Nil {
        local: NilListLocalId,
        len: usize,
    },
    Tuple {
        local: TupleListLocalId,
        item_type: Vec<ValueType>,
        value: Vec<Vec<Value>>,
    },
    List {
        local: ListListLocalId,
        item_type: Box<ValueType>,
        value: Vec<ListValue>,
    },
    Function {
        local: FunctionListLocalId,
        item_type: FunctionType,
        value: Vec<super::FunctionValue>,
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

    pub(crate) fn bit_array(local: BitArrayLocalId, value: BitArrayValue) -> Self {
        Self {
            kind: CaptureValueKind::BitArray { local, value },
        }
    }

    pub(crate) fn utf_codepoint(local: UtfCodepointLocalId, value: char) -> Self {
        Self {
            kind: CaptureValueKind::UtfCodepoint { local, value },
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

    pub(crate) fn list(value: CaptureListValue) -> Self {
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

    pub(crate) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::BitArrayFunction { local, value },
        }
    }

    pub(crate) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::UtfCodepointFunction { local, value },
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
}

#[cfg(test)]
mod tests {
    use super::{CaptureListValue, CaptureValue, CaptureValueKind};
    use crate::plan::execution::{
        BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
        BoolFunctionId, BoolFunctionLocalId, BoolLocalId, FloatFunctionId, FloatFunctionLocalId,
        FloatLocalId, FunctionFunctionId, FunctionFunctionLocalId, IntFunctionFunctionId,
        IntFunctionId, IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId,
        ListFunctionId, ListFunctionLocal, NilFunctionId, NilFunctionLocalId, NilLocalId,
        StringFunctionId, StringFunctionLocalId, StringLocalId, TupleFunctionId,
        TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionId, UtfCodepointFunctionLocalId,
        UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::{
        BitArrayFunctionValue, BitArrayValue, BoolFunctionValue, FloatFunctionValue,
        FunctionFunctionValue, IntFunctionValue, ListFunctionValue, NilFunctionValue,
        StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue, Value,
    };

    #[test]
    fn capture_value_constructors_preserve_every_output_family() {
        let list_plan = crate::runtime::plan_src("pub fn main() -> List(Int) { [] }");
        let list_function_id = ListFunctionId::Int(list_plan.int_list_function_id(0));
        let list_type_id = list_plan.int_list_function_id(0).type_id();
        let module_int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let execution_int_function_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let int_function = IntFunctionValue::new(
            IntFunctionId(0),
            Vec::new(),
            module_int_function_type.clone(),
        );
        let float_function = FloatFunctionValue::new_with_captures(
            FloatFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Float),
        );
        let string_function = StringFunctionValue::new_with_captures(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        );
        let bit_array_function = BitArrayFunctionValue::new_with_captures(
            BitArrayFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::BitArray),
        );
        let utf_codepoint_function = UtfCodepointFunctionValue::new_with_captures(
            UtfCodepointFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
        );
        let bool_function = BoolFunctionValue::new_with_captures(
            BoolFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Bool),
        );
        let nil_function = NilFunctionValue::new_with_captures(
            NilFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Nil),
        );
        let tuple_function = TupleFunctionValue::from_evaluated(
            TupleFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
        );
        let list_function = ListFunctionValue::new_with_captures(
            list_function_id.clone(),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
        );
        let function_function = FunctionFunctionValue::from_evaluated(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(module_int_function_type.clone())),
            ),
        );
        let list_function_local = ListFunctionLocal::Int {
            local: IntListFunctionLocalId(0),
            type_: execution_int_function_type,
            list_type: list_type_id,
        };

        let captures = [
            CaptureValue::int(IntLocalId(0), 1.into()),
            CaptureValue::float(FloatLocalId(0), 1.5),
            CaptureValue::string(StringLocalId(0), "one".into()),
            CaptureValue::bit_array(BitArrayLocalId(0), BitArrayValue::from_bytes(vec![1])),
            CaptureValue::utf_codepoint(UtfCodepointLocalId(0), '\u{10ffff}'),
            CaptureValue::bool(BoolLocalId(0), true),
            CaptureValue::nil(NilLocalId(0)),
            CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]),
            CaptureValue::list(CaptureListValue::Int {
                local: IntListLocalId(0),
                value: vec![1.into()],
            }),
            CaptureValue::list(CaptureListValue::BitArray {
                local: BitArrayListLocalId(0),
                value: vec![BitArrayValue::from_bytes(vec![2])],
            }),
            CaptureValue::list(CaptureListValue::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: vec!['\u{10ffff}'],
            }),
            CaptureValue::int_function(IntFunctionLocalId(0), int_function.clone()),
            CaptureValue::float_function(FloatFunctionLocalId(0), float_function.clone()),
            CaptureValue::string_function(StringFunctionLocalId(0), string_function.clone()),
            CaptureValue::bit_array_function(
                BitArrayFunctionLocalId(0),
                bit_array_function.clone(),
            ),
            CaptureValue::utf_codepoint_function(
                UtfCodepointFunctionLocalId(0),
                utf_codepoint_function.clone(),
            ),
            CaptureValue::bool_function(BoolFunctionLocalId(0), bool_function.clone()),
            CaptureValue::nil_function(NilFunctionLocalId(0), nil_function.clone()),
            CaptureValue::tuple_function(TupleFunctionLocalId(0), tuple_function.clone()),
            CaptureValue::list_function(list_function_local.clone(), list_function.clone()),
            CaptureValue::function_function(FunctionFunctionLocalId(0), function_function.clone()),
        ];
        let expected = [
            CaptureValueKind::Int {
                local: IntLocalId(0),
                value: 1.into(),
            },
            CaptureValueKind::Float {
                local: FloatLocalId(0),
                value: 1.5,
            },
            CaptureValueKind::String {
                local: StringLocalId(0),
                value: "one".into(),
            },
            CaptureValueKind::BitArray {
                local: BitArrayLocalId(0),
                value: BitArrayValue::from_bytes(vec![1]),
            },
            CaptureValueKind::UtfCodepoint {
                local: UtfCodepointLocalId(0),
                value: '\u{10ffff}',
            },
            CaptureValueKind::Bool {
                local: BoolLocalId(0),
                value: true,
            },
            CaptureValueKind::Nil {
                local: NilLocalId(0),
            },
            CaptureValueKind::Tuple {
                local: TupleLocalId(0),
                value: vec![Value::Int(1.into())],
            },
            CaptureValueKind::List(CaptureListValue::Int {
                local: IntListLocalId(0),
                value: vec![1.into()],
            }),
            CaptureValueKind::List(CaptureListValue::BitArray {
                local: BitArrayListLocalId(0),
                value: vec![BitArrayValue::from_bytes(vec![2])],
            }),
            CaptureValueKind::List(CaptureListValue::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: vec!['\u{10ffff}'],
            }),
            CaptureValueKind::IntFunction {
                local: IntFunctionLocalId(0),
                value: int_function.clone(),
            },
            CaptureValueKind::FloatFunction {
                local: FloatFunctionLocalId(0),
                value: float_function,
            },
            CaptureValueKind::StringFunction {
                local: StringFunctionLocalId(0),
                value: string_function,
            },
            CaptureValueKind::BitArrayFunction {
                local: BitArrayFunctionLocalId(0),
                value: bit_array_function,
            },
            CaptureValueKind::UtfCodepointFunction {
                local: UtfCodepointFunctionLocalId(0),
                value: utf_codepoint_function,
            },
            CaptureValueKind::BoolFunction {
                local: BoolFunctionLocalId(0),
                value: bool_function,
            },
            CaptureValueKind::NilFunction {
                local: NilFunctionLocalId(0),
                value: nil_function,
            },
            CaptureValueKind::TupleFunction {
                local: TupleFunctionLocalId(0),
                value: tuple_function,
            },
            CaptureValueKind::ListFunction {
                local: list_function_local,
                value: list_function,
            },
            CaptureValueKind::FunctionFunction {
                local: FunctionFunctionLocalId(0),
                value: function_function,
            },
        ];

        for (capture, expected) in captures.iter().zip(expected) {
            assert_eq!(capture.kind, expected);
        }
    }
}
