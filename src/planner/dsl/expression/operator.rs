use super::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, IntoValueType,
    Nil, NilFunction, String, StringFunction, Tuple, TupleFunction,
};
use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr,
    FunctionType, IntExpr, IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, ValueType,
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
        BoolExprKind, BoolFunctionExprKind, FloatExprKind, FloatFunctionExprKind,
        FunctionFunctionExprKind, FunctionType, IntExprKind, IntFunctionExprKind, NilExprKind,
        NilFunctionExprKind, StringExprKind, StringFunctionExprKind, TupleExprKind,
        TupleFunctionExprKind, ValueType,
    };
    use crate::planner::dsl::expression::{bool_, float, int, local_tuple, string};

    #[test]
    fn int_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            int(1).add_int(int(2)).0.kind(),
            IntExprKind::Add { .. },
        ));
        assert!(matches!(
            int(1).sub_int(int(2)).0.kind(),
            IntExprKind::Sub { .. },
        ));
        assert!(matches!(
            int(1).mult_int(int(2)).0.kind(),
            IntExprKind::Mult { .. },
        ));
        assert!(matches!(
            int(1).div_int(int(2)).0.kind(),
            IntExprKind::Div { .. },
        ));
        assert!(matches!(
            int(1).remainder_int(int(2)).0.kind(),
            IntExprKind::Remainder { .. },
        ));
        assert!(matches!(
            int(1).negate_int().0.kind(),
            IntExprKind::Negate(_)
        ));
    }

    #[test]
    fn float_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            float(1.0).add_float(float(2.0)).0.kind(),
            FloatExprKind::Add { .. },
        ));
        assert!(matches!(
            float(1.0).sub_float(float(2.0)).0.kind(),
            FloatExprKind::Sub { .. },
        ));
        assert!(matches!(
            float(1.0).mult_float(float(2.0)).0.kind(),
            FloatExprKind::Mult { .. },
        ));
        assert!(matches!(
            float(1.0).div_float(float(2.0)).0.kind(),
            FloatExprKind::Div { .. },
        ));
    }

    #[test]
    fn bool_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            int(1).lt_int(int(2)).0.kind(),
            BoolExprKind::LtInt { .. },
        ));
        assert!(matches!(
            int(1).lte_int(int(2)).0.kind(),
            BoolExprKind::LtEqInt { .. },
        ));
        assert!(matches!(
            int(2).gt_int(int(1)).0.kind(),
            BoolExprKind::GtInt { .. },
        ));
        assert!(matches!(
            int(2).gte_int(int(1)).0.kind(),
            BoolExprKind::GtEqInt { .. },
        ));
        assert!(matches!(
            float(1.0).lt_float(float(2.0)).0.kind(),
            BoolExprKind::LtFloat { .. },
        ));
        assert!(matches!(
            float(1.0).lte_float(float(2.0)).0.kind(),
            BoolExprKind::LtEqFloat { .. },
        ));
        assert!(matches!(
            float(2.0).gt_float(float(1.0)).0.kind(),
            BoolExprKind::GtFloat { .. },
        ));
        assert!(matches!(
            float(2.0).gte_float(float(1.0)).0.kind(),
            BoolExprKind::GtEqFloat { .. },
        ));
        assert!(matches!(
            equal(int(1), int(1)).0.kind(),
            BoolExprKind::Equal { .. },
        ));
        assert!(matches!(
            not_equal(bool_(true), bool_(false)).0.kind(),
            BoolExprKind::NotEqual { .. },
        ));
        assert!(matches!(
            bool_(true).and_bool(bool_(false)).0.kind(),
            BoolExprKind::And { .. },
        ));
        assert!(matches!(
            bool_(true).or_bool(bool_(false)).0.kind(),
            BoolExprKind::Or { .. },
        ));
        assert!(matches!(
            bool_(true).negate_bool().0.kind(),
            BoolExprKind::Not(_)
        ));
    }

    #[test]
    fn string_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            string("a").concatenate(string("b")).0.kind(),
            StringExprKind::Concatenate { .. },
        ));
    }

    #[test]
    fn tuple_projection_helpers_build_projection_shapes() {
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::Int])
                .index_int(0)
                .0
                .kind(),
            IntExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::String])
                .index_string(0)
                .0
                .kind(),
            StringExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::Float])
                .index_float(0)
                .0
                .kind(),
            FloatExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::Bool])
                .index_bool(0)
                .0
                .kind(),
            BoolExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::Nil])
                .index_nil(0)
                .0
                .kind(),
            NilExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(0, "pair", [ValueType::Tuple(vec![ValueType::Int])])
                .index_tuple(0, [ValueType::Int])
                .0
                .kind(),
            TupleExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type([ValueType::Int], ValueType::Int)]
            )
            .index_int_function(0, [ValueType::Int])
            .0
            .kind(),
            IntFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type([ValueType::String], ValueType::String)]
            )
            .index_string_function(0, [ValueType::String])
            .0
            .kind(),
            StringFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type([ValueType::Float], ValueType::Float)]
            )
            .index_float_function(0, [ValueType::Float])
            .0
            .kind(),
            FloatFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type([ValueType::Bool], ValueType::Bool)]
            )
            .index_bool_function(0, [ValueType::Bool])
            .0
            .kind(),
            BoolFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type([ValueType::Nil], ValueType::Nil)]
            )
            .index_nil_function(0, [ValueType::Nil])
            .0
            .kind(),
            NilFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type(
                    [ValueType::Int],
                    ValueType::Tuple(vec![ValueType::Int])
                )]
            )
            .index_tuple_function(0, [ValueType::Int], [ValueType::Int])
            .0
            .kind(),
            TupleFunctionExprKind::TupleIndex { .. },
        ));
        assert!(matches!(
            local_tuple(
                0,
                "pair",
                [function_value_type(
                    [ValueType::Int],
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int
                    )))
                )]
            )
            .index_function_function(
                0,
                [ValueType::Int],
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .0
            .kind(),
            FunctionFunctionExprKind::TupleIndex { .. },
        ));
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
