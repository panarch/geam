use super::{
    Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction, Tuple, TupleFunction,
};
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, CallArg, CaptureArg, FloatFunctionLocalId, FloatLocalId,
    IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, StringFunctionLocalId,
    StringLocalId, TupleFunctionLocalId, TupleLocalId,
};

pub(crate) fn int_arg(local: usize, value: Int) -> CallArg {
    CallArg::int(IntLocalId(local), value.into())
}

pub(crate) fn capture_int(local: usize, value: Int) -> CaptureArg {
    CaptureArg::int(IntLocalId(local), value.into())
}

pub(crate) fn int_function_call_arg(local: usize, value: Int) -> CallArg {
    CallArg::int(IntLocalId(local), value.into())
}

pub(crate) fn int_function_arg(local: usize, value: IntFunction) -> CallArg {
    CallArg::int_function(IntFunctionLocalId(local), value.into())
}

pub(crate) fn string_arg(local: usize, value: String) -> CallArg {
    CallArg::string(StringLocalId(local), value.into())
}

pub(crate) fn string_function_arg(local: usize, value: StringFunction) -> CallArg {
    CallArg::string_function(StringFunctionLocalId(local), value.into())
}

pub(crate) fn float_arg(local: usize, value: Float) -> CallArg {
    CallArg::float(FloatLocalId(local), value.into())
}

pub(crate) fn capture_float(local: usize, value: Float) -> CaptureArg {
    CaptureArg::float(FloatLocalId(local), value.into())
}

pub(crate) fn float_function_arg(local: usize, value: FloatFunction) -> CallArg {
    CallArg::float_function(FloatFunctionLocalId(local), value.into())
}

pub(crate) fn bool_arg(local: usize, value: Bool) -> CallArg {
    CallArg::bool(BoolLocalId(local), value.into())
}

pub(crate) fn bool_function_arg(local: usize, value: BoolFunction) -> CallArg {
    CallArg::bool_function(BoolFunctionLocalId(local), value.into())
}

pub(crate) fn nil_arg(local: usize, value: Nil) -> CallArg {
    CallArg::nil(NilLocalId(local), value.into())
}

pub(crate) fn nil_function_arg(local: usize, value: NilFunction) -> CallArg {
    CallArg::nil_function(NilFunctionLocalId(local), value.into())
}

pub(crate) fn tuple_arg(local: usize, value: Tuple) -> CallArg {
    CallArg::tuple(TupleLocalId(local), value.into())
}

pub(crate) fn capture_tuple(local: usize, value: Tuple) -> CaptureArg {
    CaptureArg::tuple(TupleLocalId(local), value.into())
}

pub(crate) fn tuple_function_arg(local: usize, value: TupleFunction) -> CallArg {
    CallArg::tuple_function(TupleFunctionLocalId(local), value.into())
}

#[cfg(test)]
mod tests {
    use super::{
        bool_arg, bool_function_arg, capture_float, capture_int, capture_tuple, float_arg,
        float_function_arg, int_arg, int_function_arg, int_function_call_arg, nil_arg,
        nil_function_arg, string_arg, string_function_arg, tuple_arg, tuple_function_arg,
    };
    use crate::plan::{
        BoolFunctionLocalId, BoolLocalId, CallArg, CaptureArg, Expr, FloatFunctionLocalId,
        FloatLocalId, IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, ParamLocal,
        StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
    };

    #[test]
    fn call_arg_helpers_build_typed_arg_shapes() {
        assert_eq!(
            int_arg(0, int(1)),
            CallArg::int(IntLocalId(0), int(1).into())
        );
        assert_eq!(
            capture_int(1, int(2)),
            CaptureArg::int(IntLocalId(1), int(2).into()),
        );
        assert_eq!(
            int_function_call_arg(2, int(3)),
            CallArg::int(IntLocalId(2), int(3).into()),
        );
        assert_eq!(
            int_function_arg(3, int_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::int_function(
                IntFunctionLocalId(3),
                int_function_ref(0, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_arg(4, string("a")),
            CallArg::string(StringLocalId(4), string("a").into()),
        );
        assert_eq!(
            string_function_arg(5, string_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::string_function(
                StringFunctionLocalId(5),
                string_function_ref(0, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            float_arg(6, float(1.0)),
            CallArg::float(FloatLocalId(6), float(1.0).into()),
        );
        assert_eq!(
            capture_float(7, float(2.0)),
            CaptureArg::float(FloatLocalId(7), float(2.0).into()),
        );
        assert_eq!(
            float_function_arg(8, float_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::float_function(
                FloatFunctionLocalId(8),
                float_function_ref(0, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_arg(9, bool_(true)),
            CallArg::bool(BoolLocalId(9), bool_(true).into()),
        );
        assert_eq!(
            bool_function_arg(10, bool_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::bool_function(
                BoolFunctionLocalId(10),
                bool_function_ref(0, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            nil_arg(11, nil()),
            CallArg::nil(NilLocalId(11), nil().into())
        );
        assert_eq!(
            nil_function_arg(12, nil_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::nil_function(
                NilFunctionLocalId(12),
                nil_function_ref(0, Vec::<ParamLocal>::new()).into(),
            ),
        );

        let tuple_value_expr = crate::plan::TupleExpr::value(
            vec![Expr::from(int(1)), Expr::from(string("one"))],
            vec![crate::plan::ValueType::Int, crate::plan::ValueType::String],
        );
        assert_eq!(
            tuple_arg(13, tuple([Expr::from(int(1)), Expr::from(string("one"))])),
            CallArg::tuple(TupleLocalId(13), tuple_value_expr.clone()),
        );
        assert_eq!(
            capture_tuple(14, tuple([Expr::from(int(1)), Expr::from(string("one"))])),
            CaptureArg::tuple(TupleLocalId(14), tuple_value_expr),
        );
        assert_eq!(
            tuple_function_arg(
                0,
                tuple_function_ref(0, Vec::<ParamLocal>::new(), [crate::plan::ValueType::Int]),
            ),
            CallArg::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_ref(0, Vec::<ParamLocal>::new(), [crate::plan::ValueType::Int])
                    .into(),
            ),
        );
    }
}
