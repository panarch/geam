use super::{
    BitArray, BitArrayFunction, Bool, BoolFunction, Float, FloatFunction, Function,
    FunctionFunction, Int, IntFunction, IntoParamLocal, IntoValueType, List, ListFunction, Nil,
    NilFunction, String, StringFunction, Tuple, TupleFunction,
};
use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, Expr, FloatExpr,
    FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr,
    ListFunctionExpr, LocalId, NilExpr, NilFunctionExpr, ParamLocal, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, ValueType,
};

impl From<Int> for Expr {
    fn from(value: Int) -> Self {
        Self::int(value.into())
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Self::string(value.into())
    }
}

impl From<BitArray> for Expr {
    fn from(value: BitArray) -> Self {
        Self::bit_array(value.into())
    }
}

impl From<Float> for Expr {
    fn from(value: Float) -> Self {
        Self::float(value.into())
    }
}

impl From<Bool> for Expr {
    fn from(value: Bool) -> Self {
        Self::bool(value.into())
    }
}

impl From<Nil> for Expr {
    fn from(value: Nil) -> Self {
        Self::nil(value.into())
    }
}

impl From<Tuple> for Expr {
    fn from(value: Tuple) -> Self {
        Self::tuple(value.into())
    }
}

impl From<List> for Expr {
    fn from(value: List) -> Self {
        Self::list(value.into())
    }
}

impl From<Function> for Expr {
    fn from(value: Function) -> Self {
        Self::function(value.into())
    }
}

impl From<Int> for IntExpr {
    fn from(value: Int) -> Self {
        value.0
    }
}

impl From<String> for StringExpr {
    fn from(value: String) -> Self {
        value.0
    }
}

impl From<BitArray> for BitArrayExpr {
    fn from(value: BitArray) -> Self {
        value.0
    }
}

impl From<Float> for FloatExpr {
    fn from(value: Float) -> Self {
        value.0
    }
}

impl From<Bool> for BoolExpr {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl From<Nil> for NilExpr {
    fn from(value: Nil) -> Self {
        value.0
    }
}

impl From<Tuple> for TupleExpr {
    fn from(value: Tuple) -> Self {
        value.0
    }
}

impl From<List> for ListExpr {
    fn from(value: List) -> Self {
        value.0
    }
}

impl From<Function> for FunctionExpr {
    fn from(value: Function) -> Self {
        value.0
    }
}

impl From<IntFunction> for Function {
    fn from(value: IntFunction) -> Self {
        Function(FunctionExpr::int(value.into()))
    }
}

impl From<IntFunction> for Expr {
    fn from(value: IntFunction) -> Self {
        Self::function(FunctionExpr::int(value.into()))
    }
}

impl From<IntFunction> for FunctionExpr {
    fn from(value: IntFunction) -> Self {
        FunctionExpr::int(value.into())
    }
}

impl From<IntFunction> for IntFunctionExpr {
    fn from(value: IntFunction) -> Self {
        value.0
    }
}

impl From<StringFunction> for StringFunctionExpr {
    fn from(value: StringFunction) -> Self {
        value.0
    }
}

impl From<BitArrayFunction> for BitArrayFunctionExpr {
    fn from(value: BitArrayFunction) -> Self {
        value.0
    }
}

impl From<FloatFunction> for FloatFunctionExpr {
    fn from(value: FloatFunction) -> Self {
        value.0
    }
}

impl From<StringFunction> for Expr {
    fn from(value: StringFunction) -> Self {
        Self::function(FunctionExpr::string(value.into()))
    }
}

impl From<BitArrayFunction> for Function {
    fn from(value: BitArrayFunction) -> Self {
        Function(FunctionExpr::bit_array(value.into()))
    }
}

impl From<BitArrayFunction> for Expr {
    fn from(value: BitArrayFunction) -> Self {
        Self::function(FunctionExpr::bit_array(value.into()))
    }
}

impl From<BitArrayFunction> for FunctionExpr {
    fn from(value: BitArrayFunction) -> Self {
        FunctionExpr::bit_array(value.into())
    }
}

impl From<FloatFunction> for Function {
    fn from(value: FloatFunction) -> Self {
        Function(FunctionExpr::float(value.into()))
    }
}

impl From<FloatFunction> for Expr {
    fn from(value: FloatFunction) -> Self {
        Self::function(FunctionExpr::float(value.into()))
    }
}

impl From<FloatFunction> for FunctionExpr {
    fn from(value: FloatFunction) -> Self {
        FunctionExpr::float(value.into())
    }
}

impl From<BoolFunction> for BoolFunctionExpr {
    fn from(value: BoolFunction) -> Self {
        value.0
    }
}

impl From<BoolFunction> for Expr {
    fn from(value: BoolFunction) -> Self {
        Self::function(FunctionExpr::bool(value.into()))
    }
}

impl From<NilFunction> for NilFunctionExpr {
    fn from(value: NilFunction) -> Self {
        value.0
    }
}

impl From<NilFunction> for Expr {
    fn from(value: NilFunction) -> Self {
        Self::function(FunctionExpr::nil(value.into()))
    }
}

impl From<TupleFunction> for TupleFunctionExpr {
    fn from(value: TupleFunction) -> Self {
        value.0
    }
}

impl From<ListFunction> for ListFunctionExpr {
    fn from(value: ListFunction) -> Self {
        value.0
    }
}

impl From<TupleFunction> for Function {
    fn from(value: TupleFunction) -> Self {
        Function(FunctionExpr::tuple(value.into()))
    }
}

impl From<ListFunction> for Function {
    fn from(value: ListFunction) -> Self {
        Function(FunctionExpr::list(value.into()))
    }
}

impl From<ListFunction> for Expr {
    fn from(value: ListFunction) -> Self {
        Self::function(FunctionExpr::list(value.into()))
    }
}

impl From<ListFunction> for FunctionExpr {
    fn from(value: ListFunction) -> Self {
        FunctionExpr::list(value.into())
    }
}

impl From<TupleFunction> for Expr {
    fn from(value: TupleFunction) -> Self {
        Self::function(FunctionExpr::tuple(value.into()))
    }
}

impl From<TupleFunction> for FunctionExpr {
    fn from(value: TupleFunction) -> Self {
        FunctionExpr::tuple(value.into())
    }
}

impl From<FunctionFunction> for Function {
    fn from(value: FunctionFunction) -> Self {
        Function(FunctionExpr::function(value.into()))
    }
}

impl From<FunctionFunction> for Expr {
    fn from(value: FunctionFunction) -> Self {
        Self::function(FunctionExpr::function(value.into()))
    }
}

impl From<FunctionFunction> for FunctionExpr {
    fn from(value: FunctionFunction) -> Self {
        FunctionExpr::function(value.into())
    }
}

impl From<FunctionFunction> for FunctionFunctionExpr {
    fn from(value: FunctionFunction) -> Self {
        value.0
    }
}

impl IntoValueType for ValueType {
    fn into_value_type(self) -> ValueType {
        self
    }
}

impl IntoValueType for LocalId {
    fn into_value_type(self) -> ValueType {
        match self {
            LocalId::Int(_) => ValueType::Int,
            LocalId::String(_) => ValueType::String,
            LocalId::BitArray(_) => ValueType::BitArray,
            LocalId::Float(_) => ValueType::Float,
            LocalId::Bool(_) => ValueType::Bool,
            LocalId::Nil(_) => ValueType::Nil,
        }
    }
}

impl IntoValueType for ParamLocal {
    fn into_value_type(self) -> ValueType {
        self.value_type()
    }
}

impl IntoParamLocal for LocalId {
    fn into_param_local(self) -> ParamLocal {
        match self {
            LocalId::Int(local) => ParamLocal::int(local),
            LocalId::String(local) => ParamLocal::string(local),
            LocalId::BitArray(local) => ParamLocal::bit_array(local),
            LocalId::Float(local) => ParamLocal::float(local),
            LocalId::Bool(local) => ParamLocal::bool(local),
            LocalId::Nil(local) => ParamLocal::nil(local),
        }
    }
}

impl IntoParamLocal for ParamLocal {
    fn into_param_local(self) -> ParamLocal {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{IntoParamLocal, IntoValueType};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionId, BitArrayFunctionReference,
        BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionReference, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntFunctionReference, ListExpr, ListFunctionExpr, ListFunctionId,
        ListFunctionReference, NilExpr, ParamLocal, StringExpr, TupleExpr, TupleFunctionExpr,
        TupleFunctionId, TupleFunctionReference, ValueType,
    };
    use crate::planner::dsl::expression::{
        Function, bit_array, bit_array_function_ref, bool_, float, float_function_ref,
        function_function_ref, int, int_function_ref, list, list_function_ref, nil, string, tuple,
        tuple_function_ref,
    };
    use num_bigint::BigInt;

    #[test]
    fn value_type_conversions() {
        assert_eq!(ValueType::Int.into_value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::int(crate::plan::IntLocalId(0)).into_value_type(),
            ValueType::Int,
        );

        assert_eq!(
            crate::plan::LocalId::Int(crate::plan::IntLocalId(0)).into_value_type(),
            ValueType::Int,
        );
        assert_eq!(
            crate::plan::LocalId::String(crate::plan::StringLocalId(1)).into_value_type(),
            ValueType::String,
        );
        assert_eq!(
            crate::plan::LocalId::BitArray(crate::plan::BitArrayLocalId(5)).into_value_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            crate::plan::LocalId::Float(crate::plan::FloatLocalId(2)).into_value_type(),
            ValueType::Float,
        );
        assert_eq!(
            crate::plan::LocalId::Bool(crate::plan::BoolLocalId(3)).into_value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            crate::plan::LocalId::Nil(crate::plan::NilLocalId(4)).into_value_type(),
            ValueType::Nil,
        );

        assert_eq!(
            crate::plan::LocalId::Int(crate::plan::IntLocalId(0)).into_param_local(),
            ParamLocal::int(crate::plan::IntLocalId(0)),
        );
        assert_eq!(
            crate::plan::LocalId::String(crate::plan::StringLocalId(1)).into_param_local(),
            ParamLocal::string(crate::plan::StringLocalId(1)),
        );
        assert_eq!(
            crate::plan::LocalId::BitArray(crate::plan::BitArrayLocalId(5)).into_param_local(),
            ParamLocal::bit_array(crate::plan::BitArrayLocalId(5)),
        );
        assert_eq!(
            crate::plan::LocalId::Float(crate::plan::FloatLocalId(2)).into_param_local(),
            ParamLocal::float(crate::plan::FloatLocalId(2)),
        );
        assert_eq!(
            crate::plan::LocalId::Bool(crate::plan::BoolLocalId(3)).into_param_local(),
            ParamLocal::bool(crate::plan::BoolLocalId(3)),
        );
        assert_eq!(
            crate::plan::LocalId::Nil(crate::plan::NilLocalId(4)).into_param_local(),
            ParamLocal::nil(crate::plan::NilLocalId(4)),
        );
    }

    #[test]
    fn wrapper_conversions_preserve_result_families() {
        assert_eq!(
            Expr::from(int(1)),
            Expr::int(IntExpr::value(BigInt::from(1)))
        );
        assert_eq!(
            Expr::from(string("a")),
            Expr::string(StringExpr::value("a".into())),
        );
        assert_eq!(
            Expr::from(bit_array([])),
            Expr::bit_array(BitArrayExpr::value(Vec::new())),
        );
        assert_eq!(Expr::from(float(1.5)), Expr::float(FloatExpr::value(1.5)));
        assert_eq!(Expr::from(bool_(true)), Expr::bool(BoolExpr::value(true)));
        assert_eq!(Expr::from(nil()), Expr::nil(NilExpr::value()));
        assert_eq!(
            Expr::from(tuple([Expr::from(int(1)), Expr::from(string("one"))])),
            Expr::tuple(TupleExpr::value(
                vec![Expr::from(int(1)), Expr::from(string("one"))],
                vec![ValueType::Int, ValueType::String],
            )),
        );
        assert_eq!(
            Expr::from(list([int(1)], ValueType::Int)),
            Expr::list(ListExpr::value(vec![Expr::from(int(1))], ValueType::Int)),
        );

        assert_eq!(
            Expr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
            Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                IntFunctionReference::new(IntFunctionId(0), Vec::new()),
            ))),
        );
        assert_eq!(
            Expr::from(bit_array_function_ref(0, Vec::<ParamLocal>::new())),
            Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::reference(
                BitArrayFunctionReference::new(BitArrayFunctionId(0), Vec::new()),
            ))),
        );
        assert_eq!(
            FunctionExpr::from(bit_array_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::bit_array(BitArrayFunctionExpr::reference(
                BitArrayFunctionReference::new(BitArrayFunctionId(0), Vec::new()),
            )),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(bit_array_function_ref(
                0,
                Vec::<ParamLocal>::new(),
            ))),
            FunctionExpr::bit_array(BitArrayFunctionExpr::reference(
                BitArrayFunctionReference::new(BitArrayFunctionId(0), Vec::new()),
            )),
        );
        assert_eq!(
            Expr::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int
            )),
            Expr::function(FunctionExpr::list(ListFunctionExpr::reference(
                ListFunctionReference::new(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new()
                ),
            ))),
        );
        assert_eq!(
            FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
                IntFunctionId(0),
                Vec::new(),
            ))),
        );
        assert_eq!(
            FunctionExpr::from(float_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::float(crate::plan::FloatFunctionExpr::reference(
                crate::plan::FloatFunctionReference::new(
                    crate::plan::FloatFunctionId(0),
                    Vec::new()
                ),
            )),
        );
        assert_eq!(
            FunctionExpr::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int]
            )),
            FunctionExpr::tuple(TupleFunctionExpr::reference(
                TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                vec![ValueType::Int],
            )),
        );
        assert_eq!(
            FunctionExpr::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int
            )),
            FunctionExpr::list(ListFunctionExpr::reference(ListFunctionReference::new(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                Vec::new()
            ))),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(float_function_ref(
                0,
                Vec::<ParamLocal>::new()
            ))),
            FunctionExpr::float(crate::plan::FloatFunctionExpr::reference(
                crate::plan::FloatFunctionReference::new(
                    crate::plan::FloatFunctionId(0),
                    Vec::new()
                ),
            )),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            ))),
            FunctionExpr::tuple(TupleFunctionExpr::reference(
                TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                vec![ValueType::Int],
            )),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int,
            ))),
            FunctionExpr::list(ListFunctionExpr::reference(ListFunctionReference::new(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                Vec::new()
            ))),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ))),
            FunctionExpr::function(FunctionFunctionExpr::reference(
                FunctionFunctionReference::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                ),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )),
        );
    }
}
