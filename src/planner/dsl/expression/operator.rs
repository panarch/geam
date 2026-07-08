use super::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, IntoValueType,
    List, ListFunction, Nil, NilFunction, String, StringFunction, Tuple, TupleFunction,
};
use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr,
    FunctionType, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr,
    StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr, ValueType,
};

pub(crate) fn equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::equal(left.into(), right.into()))
}

pub(crate) fn not_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::not_equal(left.into(), right.into()))
}

impl Int {
    pub(crate) fn add_int(self, right: Self) -> Self {
        Self(IntExpr::add(self.into(), right.into()))
    }

    pub(crate) fn sub_int(self, right: Self) -> Self {
        Self(IntExpr::sub(self.into(), right.into()))
    }

    pub(crate) fn mult_int(self, right: Self) -> Self {
        Self(IntExpr::mult(self.into(), right.into()))
    }

    pub(crate) fn div_int(self, right: Self) -> Self {
        Self(IntExpr::div(self.into(), right.into()))
    }

    pub(crate) fn remainder_int(self, right: Self) -> Self {
        Self(IntExpr::remainder(self.into(), right.into()))
    }

    pub(crate) fn lt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lt_int(self.into(), right.into()))
    }

    pub(crate) fn lte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lte_int(self.into(), right.into()))
    }

    pub(crate) fn gt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gt_int(self.into(), right.into()))
    }

    pub(crate) fn gte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gte_int(self.into(), right.into()))
    }

    pub(crate) fn negate_int(self) -> Self {
        Self(IntExpr::negate(self.into()))
    }
}

impl Float {
    pub(crate) fn add_float(self, right: Self) -> Self {
        Self(FloatExpr::add(self.into(), right.into()))
    }

    pub(crate) fn sub_float(self, right: Self) -> Self {
        Self(FloatExpr::sub(self.into(), right.into()))
    }

    pub(crate) fn mult_float(self, right: Self) -> Self {
        Self(FloatExpr::mult(self.into(), right.into()))
    }

    pub(crate) fn div_float(self, right: Self) -> Self {
        Self(FloatExpr::div(self.into(), right.into()))
    }

    pub(crate) fn lt_float(self, right: Self) -> Bool {
        Bool(BoolExpr::lt_float(self.into(), right.into()))
    }

    pub(crate) fn lte_float(self, right: Self) -> Bool {
        Bool(BoolExpr::lte_float(self.into(), right.into()))
    }

    pub(crate) fn gt_float(self, right: Self) -> Bool {
        Bool(BoolExpr::gt_float(self.into(), right.into()))
    }

    pub(crate) fn gte_float(self, right: Self) -> Bool {
        Bool(BoolExpr::gte_float(self.into(), right.into()))
    }
}

impl String {
    pub(crate) fn concatenate(self, right: Self) -> Self {
        Self(StringExpr::concatenate(self.into(), right.into()))
    }
}

impl Tuple {
    pub(crate) fn index_int(self, index: usize) -> Int {
        Int(IntExpr::tuple_index(self.into(), index))
    }

    pub(crate) fn index_string(self, index: usize) -> String {
        String(StringExpr::tuple_index(self.into(), index))
    }

    pub(crate) fn index_float(self, index: usize) -> Float {
        Float(FloatExpr::tuple_index(self.into(), index))
    }

    pub(crate) fn index_bool(self, index: usize) -> Bool {
        Bool(BoolExpr::tuple_index(self.into(), index))
    }

    pub(crate) fn index_nil(self, index: usize) -> Nil {
        Nil(NilExpr::tuple_index(self.into(), index))
    }

    pub(crate) fn index_tuple(
        self,
        index: usize,
        type_: impl IntoIterator<Item = impl IntoValueType>,
    ) -> Tuple {
        Tuple(TupleExpr::tuple_index(
            self.into(),
            index,
            type_
                .into_iter()
                .map(IntoValueType::into_value_type)
                .collect(),
        ))
    }

    pub(crate) fn index_list(self, index: usize, element_type: impl IntoValueType) -> List {
        List(ListExpr::tuple_index(
            self.into(),
            index,
            element_type.into_value_type(),
        ))
    }

    pub(crate) fn index_int_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
    ) -> IntFunction {
        IntFunction(IntFunctionExpr::tuple_index(
            self.into(),
            index,
            primitive_function_type(params, ValueType::Int),
        ))
    }

    pub(crate) fn index_string_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
    ) -> StringFunction {
        StringFunction(StringFunctionExpr::tuple_index(
            self.into(),
            index,
            primitive_function_type(params, ValueType::String),
        ))
    }

    pub(crate) fn index_float_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
    ) -> FloatFunction {
        FloatFunction(FloatFunctionExpr::tuple_index(
            self.into(),
            index,
            primitive_function_type(params, ValueType::Float),
        ))
    }

    pub(crate) fn index_bool_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
    ) -> BoolFunction {
        BoolFunction(BoolFunctionExpr::tuple_index(
            self.into(),
            index,
            primitive_function_type(params, ValueType::Bool),
        ))
    }

    pub(crate) fn index_nil_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
    ) -> NilFunction {
        NilFunction(NilFunctionExpr::tuple_index(
            self.into(),
            index,
            primitive_function_type(params, ValueType::Nil),
        ))
    }

    pub(crate) fn index_tuple_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
        return_type: impl IntoIterator<Item = impl IntoValueType>,
    ) -> TupleFunction {
        TupleFunction(TupleFunctionExpr::tuple_index(
            self.into(),
            index,
            FunctionType::new(
                params
                    .into_iter()
                    .map(IntoValueType::into_value_type)
                    .collect(),
                ValueType::Tuple(
                    return_type
                        .into_iter()
                        .map(IntoValueType::into_value_type)
                        .collect(),
                ),
            ),
        ))
    }

    pub(crate) fn index_list_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
        return_type: impl IntoValueType,
    ) -> ListFunction {
        let item_type = return_type.into_value_type();
        ListFunction(ListFunctionExpr::tuple_index(
            self.into(),
            index,
            FunctionType::new(
                params
                    .into_iter()
                    .map(IntoValueType::into_value_type)
                    .collect(),
                ValueType::List(Box::new(item_type.clone())),
            ),
            item_type,
        ))
    }

    pub(crate) fn index_function_function(
        self,
        index: usize,
        params: impl IntoIterator<Item = impl IntoValueType>,
        return_type: FunctionType,
    ) -> FunctionFunction {
        FunctionFunction(FunctionFunctionExpr::tuple_index(
            self.into(),
            index,
            FunctionType::new(
                params
                    .into_iter()
                    .map(IntoValueType::into_value_type)
                    .collect(),
                ValueType::Function(Box::new(return_type)),
            ),
        ))
    }
}

fn primitive_function_type(
    params: impl IntoIterator<Item = impl IntoValueType>,
    return_: ValueType,
) -> FunctionType {
    FunctionType::new(
        params
            .into_iter()
            .map(IntoValueType::into_value_type)
            .collect(),
        return_,
    )
}

impl Bool {
    pub(crate) fn and_bool(self, right: Self) -> Self {
        Self(BoolExpr::and(self.into(), right.into()))
    }

    pub(crate) fn or_bool(self, right: Self) -> Self {
        Self(BoolExpr::or(self.into(), right.into()))
    }

    pub(crate) fn negate_bool(self) -> Self {
        Self(BoolExpr::not(self.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{equal, not_equal};
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr,
        FunctionType, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr,
        NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr, ValueType,
    };
    use crate::planner::dsl::expression::{bool_, float, int, local_tuple, string};

    #[test]
    fn int_operator_helpers_build_operator_shapes() {
        assert_eq!(
            int(1).add_int(int(2)).0,
            IntExpr::add(int(1).into(), int(2).into())
        );
        assert_eq!(
            int(1).sub_int(int(2)).0,
            IntExpr::sub(int(1).into(), int(2).into())
        );
        assert_eq!(
            int(1).mult_int(int(2)).0,
            IntExpr::mult(int(1).into(), int(2).into()),
        );
        assert_eq!(
            int(1).div_int(int(2)).0,
            IntExpr::div(int(1).into(), int(2).into())
        );
        assert_eq!(
            int(1).remainder_int(int(2)).0,
            IntExpr::remainder(int(1).into(), int(2).into()),
        );
        assert_eq!(int(1).negate_int().0, IntExpr::negate(int(1).into()));
    }

    #[test]
    fn float_operator_helpers_build_operator_shapes() {
        assert_eq!(
            float(1.0).add_float(float(2.0)).0,
            FloatExpr::add(float(1.0).into(), float(2.0).into()),
        );
        assert_eq!(
            float(1.0).sub_float(float(2.0)).0,
            FloatExpr::sub(float(1.0).into(), float(2.0).into()),
        );
        assert_eq!(
            float(1.0).mult_float(float(2.0)).0,
            FloatExpr::mult(float(1.0).into(), float(2.0).into()),
        );
        assert_eq!(
            float(1.0).div_float(float(2.0)).0,
            FloatExpr::div(float(1.0).into(), float(2.0).into()),
        );
    }

    #[test]
    fn bool_operator_helpers_build_operator_shapes() {
        assert_eq!(
            int(1).lt_int(int(2)).0,
            BoolExpr::lt_int(int(1).into(), int(2).into())
        );
        assert_eq!(
            int(1).lte_int(int(2)).0,
            BoolExpr::lte_int(int(1).into(), int(2).into()),
        );
        assert_eq!(
            int(2).gt_int(int(1)).0,
            BoolExpr::gt_int(int(2).into(), int(1).into())
        );
        assert_eq!(
            int(2).gte_int(int(1)).0,
            BoolExpr::gte_int(int(2).into(), int(1).into()),
        );
        assert_eq!(
            float(1.0).lt_float(float(2.0)).0,
            BoolExpr::lt_float(float(1.0).into(), float(2.0).into()),
        );
        assert_eq!(
            float(1.0).lte_float(float(2.0)).0,
            BoolExpr::lte_float(float(1.0).into(), float(2.0).into()),
        );
        assert_eq!(
            float(2.0).gt_float(float(1.0)).0,
            BoolExpr::gt_float(float(2.0).into(), float(1.0).into()),
        );
        assert_eq!(
            float(2.0).gte_float(float(1.0)).0,
            BoolExpr::gte_float(float(2.0).into(), float(1.0).into()),
        );
        assert_eq!(
            equal(int(1), int(1)).0,
            BoolExpr::equal(int(1).into(), int(1).into())
        );
        assert_eq!(
            not_equal(bool_(true), bool_(false)).0,
            BoolExpr::not_equal(bool_(true).into(), bool_(false).into()),
        );
        assert_eq!(
            bool_(true).and_bool(bool_(false)).0,
            BoolExpr::and(bool_(true).into(), bool_(false).into()),
        );
        assert_eq!(
            bool_(true).or_bool(bool_(false)).0,
            BoolExpr::or(bool_(true).into(), bool_(false).into()),
        );
        assert_eq!(
            bool_(true).negate_bool().0,
            BoolExpr::not(bool_(true).into())
        );
    }

    #[test]
    fn string_operator_helpers_build_operator_shapes() {
        assert_eq!(
            string("a").concatenate(string("b")).0,
            StringExpr::concatenate(string("a").into(), string("b").into()),
        );
    }

    #[test]
    fn tuple_projection_helpers_build_projection_shapes() {
        let pair = local_tuple(0, "pair", [ValueType::Int]);
        assert_eq!(
            pair.index_int(0).0,
            IntExpr::tuple_index(local_tuple(0, "pair", [ValueType::Int]).into(), 0)
        );

        let pair = local_tuple(0, "pair", [ValueType::String]);
        assert_eq!(
            pair.index_string(1).0,
            StringExpr::tuple_index(local_tuple(0, "pair", [ValueType::String]).into(), 1)
        );

        let pair = local_tuple(0, "pair", [ValueType::Float]);
        assert_eq!(
            pair.index_float(2).0,
            FloatExpr::tuple_index(local_tuple(0, "pair", [ValueType::Float]).into(), 2)
        );

        let pair = local_tuple(0, "pair", [ValueType::Bool]);
        assert_eq!(
            pair.index_bool(3).0,
            BoolExpr::tuple_index(local_tuple(0, "pair", [ValueType::Bool]).into(), 3)
        );

        let pair = local_tuple(0, "pair", [ValueType::Nil]);
        assert_eq!(
            pair.index_nil(4).0,
            NilExpr::tuple_index(local_tuple(0, "pair", [ValueType::Nil]).into(), 4)
        );

        let pair = local_tuple(0, "pair", [ValueType::Tuple(vec![ValueType::Int])]);
        assert_eq!(
            pair.index_tuple(5, [ValueType::Int]).0,
            TupleExpr::tuple_index(
                local_tuple(0, "pair", [ValueType::Tuple(vec![ValueType::Int])]).into(),
                5,
                vec![ValueType::Int],
            ),
        );

        let pair = local_tuple(0, "pair", [ValueType::List(Box::new(ValueType::Int))]);
        assert_eq!(
            pair.index_list(6, ValueType::Int).0,
            ListExpr::tuple_index(
                local_tuple(0, "pair", [ValueType::List(Box::new(ValueType::Int))]).into(),
                6,
                ValueType::Int,
            ),
        );

        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Int], ValueType::Int)],
        );
        assert_eq!(
            pair.index_int_function(7, [ValueType::Int]).0,
            IntFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Int], ValueType::Int)]
                )
                .into(),
                7,
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
        );

        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::String], ValueType::String)],
        );
        assert_eq!(
            pair.index_string_function(8, [ValueType::String]).0,
            StringFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::String], ValueType::String)]
                )
                .into(),
                8,
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
        );

        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Float], ValueType::Float)],
        );
        assert_eq!(
            pair.index_float_function(9, [ValueType::Float]).0,
            FloatFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Float], ValueType::Float)]
                )
                .into(),
                9,
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );

        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Bool], ValueType::Bool)],
        );
        assert_eq!(
            pair.index_bool_function(10, [ValueType::Bool]).0,
            BoolFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Bool], ValueType::Bool)]
                )
                .into(),
                10,
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            ),
        );

        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Nil], ValueType::Nil)],
        );
        assert_eq!(
            pair.index_nil_function(11, [ValueType::Nil]).0,
            NilFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Nil], ValueType::Nil)]
                )
                .into(),
                11,
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            ),
        );

        let tuple_return = ValueType::Tuple(vec![ValueType::Int]);
        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Int], tuple_return.clone())],
        );
        assert_eq!(
            pair.index_tuple_function(12, [ValueType::Int], [ValueType::Int])
                .0,
            TupleFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Int], tuple_return)]
                )
                .into(),
                12,
                FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int])),
            ),
        );

        let list_return = ValueType::List(Box::new(ValueType::Int));
        let pair = local_tuple(
            0,
            "pair",
            [function_value_type([ValueType::Int], list_return.clone())],
        );
        assert_eq!(
            pair.index_list_function(13, [ValueType::Int], ValueType::Int)
                .0,
            ListFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type([ValueType::Int], list_return)]
                )
                .into(),
                13,
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::List(Box::new(ValueType::Int)),
                ),
                ValueType::Int,
            ),
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let pair = local_tuple(
            0,
            "pair",
            [function_value_type(
                [ValueType::Int],
                ValueType::Function(Box::new(returned_function_type.clone())),
            )],
        );
        assert_eq!(
            pair.index_function_function(14, [ValueType::Int], returned_function_type.clone())
                .0,
            FunctionFunctionExpr::tuple_index(
                local_tuple(
                    0,
                    "pair",
                    [function_value_type(
                        [ValueType::Int],
                        ValueType::Function(Box::new(returned_function_type.clone())),
                    )]
                )
                .into(),
                14,
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Function(Box::new(returned_function_type)),
                ),
            ),
        );
    }

    fn function_value_type(
        params: impl IntoIterator<Item = ValueType>,
        return_: ValueType,
    ) -> ValueType {
        ValueType::Function(Box::new(FunctionType::new(
            params.into_iter().collect(),
            return_,
        )))
    }
}
