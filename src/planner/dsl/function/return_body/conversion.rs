use super::FunctionReturn;
use crate::plan::{
    BitArrayFunctionExpr, BitArrayReturn, BoolFunctionExpr, BoolReturn, FloatFunctionExpr,
    FloatReturn, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, IntFunctionExpr, IntReturn,
    ListFunctionExpr, ListReturn, NilFunctionExpr, NilReturn, ReturnBody, StringFunctionExpr,
    StringReturn, TupleFunctionExpr, TupleReturn, UtfCodepointFunctionExpr, UtfCodepointReturn,
};
use crate::planner::dsl::expression::{
    BitArray, BitArrayFunction, Bool, BoolFunction, Float, FloatFunction, Function,
    FunctionFunction, Int, IntFunction, List, ListFunction, Nil, NilFunction, String,
    StringFunction, Tuple, TupleFunction, UtfCodepoint, UtfCodepointFunction,
};

impl From<Int> for FunctionReturn {
    fn from(value: Int) -> Self {
        Self::Int(ReturnBody::expr(value.into()))
    }
}

impl From<String> for FunctionReturn {
    fn from(value: String) -> Self {
        Self::String(ReturnBody::expr(value.into()))
    }
}

impl From<BitArray> for FunctionReturn {
    fn from(value: BitArray) -> Self {
        Self::BitArray(ReturnBody::expr(value.into()))
    }
}

impl From<UtfCodepoint> for FunctionReturn {
    fn from(value: UtfCodepoint) -> Self {
        Self::UtfCodepoint(ReturnBody::expr(value.into()))
    }
}

impl From<Float> for FunctionReturn {
    fn from(value: Float) -> Self {
        Self::Float(ReturnBody::expr(value.into()))
    }
}

impl From<Bool> for FunctionReturn {
    fn from(value: Bool) -> Self {
        Self::Bool(ReturnBody::expr(value.into()))
    }
}

impl From<Nil> for FunctionReturn {
    fn from(value: Nil) -> Self {
        Self::Nil(ReturnBody::expr(value.into()))
    }
}

impl From<Tuple> for FunctionReturn {
    fn from(value: Tuple) -> Self {
        let expression = crate::plan::TupleExpr::from(value);
        Self::Tuple {
            type_: expression.type_().to_vec(),
            body: TupleReturn::expr(expression),
        }
    }
}

impl From<List> for FunctionReturn {
    fn from(value: List) -> Self {
        let expression = crate::plan::ListExpr::from(value);
        Self::List(ListReturn::expr(expression))
    }
}

impl From<IntFunction> for FunctionReturn {
    fn from(value: IntFunction) -> Self {
        let expression = IntFunctionExpr::from(value);
        Self::IntFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<StringFunction> for FunctionReturn {
    fn from(value: StringFunction) -> Self {
        let expression = StringFunctionExpr::from(value);
        Self::StringFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<BitArrayFunction> for FunctionReturn {
    fn from(value: BitArrayFunction) -> Self {
        let expression = BitArrayFunctionExpr::from(value);
        Self::BitArrayFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<UtfCodepointFunction> for FunctionReturn {
    fn from(value: UtfCodepointFunction) -> Self {
        let expression = UtfCodepointFunctionExpr::from(value);
        Self::UtfCodepointFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<FloatFunction> for FunctionReturn {
    fn from(value: FloatFunction) -> Self {
        let expression = FloatFunctionExpr::from(value);
        Self::FloatFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<BoolFunction> for FunctionReturn {
    fn from(value: BoolFunction) -> Self {
        let expression = BoolFunctionExpr::from(value);
        Self::BoolFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<NilFunction> for FunctionReturn {
    fn from(value: NilFunction) -> Self {
        let expression = NilFunctionExpr::from(value);
        Self::NilFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<TupleFunction> for FunctionReturn {
    fn from(value: TupleFunction) -> Self {
        let expression = TupleFunctionExpr::from(value);
        Self::TupleFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<ListFunction> for FunctionReturn {
    fn from(value: ListFunction) -> Self {
        let expression = ListFunctionExpr::from(value);
        Self::ListFunction {
            item_type: expression.return_item_type(),
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<FunctionFunction> for FunctionReturn {
    fn from(value: FunctionFunction) -> Self {
        let expression = FunctionFunctionExpr::from(value);
        Self::FunctionFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<Function> for FunctionReturn {
    fn from(value: Function) -> Self {
        match FunctionExpr::from(value).into_kind() {
            FunctionExprKind::Int(expression) => Self::IntFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::String(expression) => Self::StringFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::BitArray(expression) => Self::BitArrayFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::UtfCodepoint(expression) => Self::UtfCodepointFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Float(expression) => Self::FloatFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Bool(expression) => Self::BoolFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Nil(expression) => Self::NilFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Tuple(expression) => Self::TupleFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::List(expression) => Self::ListFunction {
                item_type: expression.return_item_type(),
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Function(expression) => Self::FunctionFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
        }
    }
}

impl From<IntReturn> for FunctionReturn {
    fn from(value: IntReturn) -> Self {
        Self::Int(value)
    }
}

impl From<BitArrayReturn> for FunctionReturn {
    fn from(value: BitArrayReturn) -> Self {
        Self::BitArray(value)
    }
}

impl From<UtfCodepointReturn> for FunctionReturn {
    fn from(value: UtfCodepointReturn) -> Self {
        Self::UtfCodepoint(value)
    }
}

impl From<StringReturn> for FunctionReturn {
    fn from(value: StringReturn) -> Self {
        Self::String(value)
    }
}

impl From<FloatReturn> for FunctionReturn {
    fn from(value: FloatReturn) -> Self {
        Self::Float(value)
    }
}

impl From<BoolReturn> for FunctionReturn {
    fn from(value: BoolReturn) -> Self {
        Self::Bool(value)
    }
}

impl From<NilReturn> for FunctionReturn {
    fn from(value: NilReturn) -> Self {
        Self::Nil(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturn;
    use crate::plan::{
        BitArrayFunctionId, BitArrayReturn, BoolFunctionId, BoolReturn, Expr, FloatFunctionId,
        FloatReturn, FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId,
        IntReturn, ListFunctionId, ListReturn, NilFunctionId, NilReturn, ParamLocal, ReturnBody,
        RuntimeFunctionId, StringFunctionId, StringReturn, TupleFunctionId, UtfCodepointFunctionId,
        UtfCodepointReturn, ValueType,
    };
    use crate::planner::dsl::expression::{
        bit_array, bit_array_function_ref, bool_, bool_function_ref, float, float_function_ref,
        function_function_ref, function_ref, int, int_function_ref, list, list_function_ref,
        local_utf_codepoint, nil, nil_function_ref, string, string_function_ref, tuple,
        tuple_function_ref, utf_codepoint_function_ref,
    };

    #[test]
    fn value_conversions_build_function_return_families() {
        assert_eq!(
            FunctionReturn::from(int(1)),
            FunctionReturn::Int(ReturnBody::expr(int(1).into())),
        );
        assert_eq!(
            FunctionReturn::from(string("value")),
            FunctionReturn::String(ReturnBody::expr(string("value").into())),
        );
        assert_eq!(
            FunctionReturn::from(bit_array([])),
            FunctionReturn::BitArray(ReturnBody::expr(bit_array([]).into())),
        );
        assert_eq!(
            FunctionReturn::from(BitArrayReturn::expr(bit_array([]).into())),
            FunctionReturn::BitArray(BitArrayReturn::expr(bit_array([]).into())),
        );
        assert_eq!(
            FunctionReturn::from(local_utf_codepoint(0, "codepoint")),
            FunctionReturn::UtfCodepoint(ReturnBody::expr(
                local_utf_codepoint(0, "codepoint").into(),
            )),
        );
        assert_eq!(
            FunctionReturn::from(UtfCodepointReturn::expr(
                local_utf_codepoint(0, "codepoint").into(),
            )),
            FunctionReturn::UtfCodepoint(UtfCodepointReturn::expr(
                local_utf_codepoint(0, "codepoint").into(),
            )),
        );
        assert_eq!(
            FunctionReturn::from(float(1.5)),
            FunctionReturn::Float(ReturnBody::expr(float(1.5).into())),
        );
        assert_eq!(
            FunctionReturn::from(bool_(true)),
            FunctionReturn::Bool(ReturnBody::expr(bool_(true).into())),
        );
        assert_eq!(
            FunctionReturn::from(nil()),
            FunctionReturn::Nil(ReturnBody::expr(nil().into())),
        );
        assert_eq!(
            FunctionReturn::from(tuple([Expr::from(int(1))])),
            FunctionReturn::Tuple {
                type_: vec![ValueType::Int],
                body: ReturnBody::expr(tuple([Expr::from(int(1))]).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(list([int(1)], ValueType::Int)),
            FunctionReturn::List(ListReturn::expr(list([int(1)], ValueType::Int).into())),
        );
    }

    #[test]
    fn function_value_conversions_preserve_return_family() {
        assert_eq!(
            FunctionReturn::from(int_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::IntFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Int),
                body: ReturnBody::expr(int_function_ref(0, Vec::<ParamLocal>::new()).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(string_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::StringFunction {
                type_: FunctionType::new(Vec::new(), ValueType::String),
                body: ReturnBody::expr(string_function_ref(0, Vec::<ParamLocal>::new()).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(bit_array_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::BitArrayFunction {
                type_: FunctionType::new(Vec::new(), ValueType::BitArray),
                body: ReturnBody::expr(bit_array_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(utf_codepoint_function_ref(0, Vec::<ParamLocal>::new(),)),
            FunctionReturn::UtfCodepointFunction {
                type_: FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                body: ReturnBody::expr(
                    utf_codepoint_function_ref(0, Vec::<ParamLocal>::new()).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(float_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::FloatFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Float),
                body: ReturnBody::expr(float_function_ref(0, Vec::<ParamLocal>::new()).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(bool_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::BoolFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Bool),
                body: ReturnBody::expr(bool_function_ref(0, Vec::<ParamLocal>::new()).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(nil_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::NilFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Nil),
                body: ReturnBody::expr(nil_function_ref(0, Vec::<ParamLocal>::new()).into()),
            },
        );
        assert_eq!(
            FunctionReturn::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            )),
            FunctionReturn::TupleFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                body: ReturnBody::expr(
                    tuple_function_ref(0, Vec::<ParamLocal>::new(), [ValueType::Int]).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int
            )),
            FunctionReturn::ListFunction {
                item_type: ValueType::Int,
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                body: ReturnBody::expr(
                    list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )),
            FunctionReturn::FunctionFunction {
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
                body: ReturnBody::expr(
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )
                    .into(),
                ),
            },
        );
    }

    #[test]
    fn erased_function_value_conversion_preserves_return_family() {
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::IntFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Int),
                body: ReturnBody::expr(int_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::StringFunction {
                type_: FunctionType::new(Vec::new(), ValueType::String),
                body: ReturnBody::expr(string_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::BitArray(BitArrayFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::BitArrayFunction {
                type_: FunctionType::new(Vec::new(), ValueType::BitArray),
                body: ReturnBody::expr(bit_array_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::UtfCodepointFunction {
                type_: FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                body: ReturnBody::expr(
                    utf_codepoint_function_ref(0, Vec::<ParamLocal>::new()).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Float(FloatFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::FloatFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Float),
                body: ReturnBody::expr(float_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::BoolFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Bool),
                body: ReturnBody::expr(bool_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::NilFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Nil),
                body: ReturnBody::expr(nil_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Tuple {
                    id: TupleFunctionId(0),
                    return_type: vec![ValueType::Int],
                },
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::TupleFunction {
                type_: FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                body: ReturnBody::expr(
                    tuple_function_ref(0, Vec::<ParamLocal>::new(), [ValueType::Int]).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::List(ListFunctionId::from_item_type(
                    0,
                    crate::plan::ValueType::Int
                )),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::ListFunction {
                item_type: ValueType::Int,
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                body: ReturnBody::expr(
                    list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
                ),
            },
        );
        assert_eq!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::FunctionFunction {
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
                body: ReturnBody::expr(
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )
                    .into(),
                ),
            },
        );
    }

    #[test]
    fn return_body_conversions_keep_existing_body_family() {
        assert_eq!(
            FunctionReturn::from(IntReturn::expr(int(1).into())),
            FunctionReturn::Int(IntReturn::expr(int(1).into())),
        );
        assert_eq!(
            FunctionReturn::from(StringReturn::expr(string("value").into())),
            FunctionReturn::String(StringReturn::expr(string("value").into())),
        );
        assert_eq!(
            FunctionReturn::from(FloatReturn::expr(float(1.0).into())),
            FunctionReturn::Float(FloatReturn::expr(float(1.0).into())),
        );
        assert_eq!(
            FunctionReturn::from(BoolReturn::expr(bool_(true).into())),
            FunctionReturn::Bool(BoolReturn::expr(bool_(true).into())),
        );
        assert_eq!(
            FunctionReturn::from(NilReturn::expr(nil().into())),
            FunctionReturn::Nil(NilReturn::expr(nil().into())),
        );
    }
}
