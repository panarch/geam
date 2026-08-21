use super::{
    Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction, Tuple, TupleFunction,
};
use crate::plan::{
    CallArg, CaptureArg, Expr, FloatLocalId, IntLocalId, ParamLocal, TupleLocalId, ValueType,
};

pub(crate) fn int_arg(value: Int) -> CallArg {
    CallArg::new(Expr::int(value.into()))
}

pub(crate) fn capture_int(index: usize) -> CaptureArg {
    CaptureArg::new(ParamLocal::int(IntLocalId(index)))
}

pub(crate) fn int_function_call_arg(value: Int) -> CallArg {
    CallArg::new(Expr::int(value.into()))
}

pub(crate) fn int_function_arg(value: IntFunction) -> CallArg {
    CallArg::new(value.into())
}

pub(crate) fn string_arg(value: String) -> CallArg {
    CallArg::new(Expr::string(value.into()))
}

pub(crate) fn string_function_arg(value: StringFunction) -> CallArg {
    CallArg::new(value.into())
}

pub(crate) fn float_arg(value: Float) -> CallArg {
    CallArg::new(Expr::float(value.into()))
}

pub(crate) fn capture_float(index: usize) -> CaptureArg {
    CaptureArg::new(ParamLocal::float(FloatLocalId(index)))
}

pub(crate) fn float_function_arg(value: FloatFunction) -> CallArg {
    CallArg::new(value.into())
}

pub(crate) fn bool_arg(value: Bool) -> CallArg {
    CallArg::new(Expr::bool(value.into()))
}

pub(crate) fn bool_function_arg(value: BoolFunction) -> CallArg {
    CallArg::new(value.into())
}

pub(crate) fn nil_arg(value: Nil) -> CallArg {
    CallArg::new(Expr::nil(value.into()))
}

pub(crate) fn nil_function_arg(value: NilFunction) -> CallArg {
    CallArg::new(value.into())
}

pub(crate) fn tuple_arg(value: Tuple) -> CallArg {
    CallArg::new(Expr::tuple(value.into()))
}

pub(crate) fn capture_tuple(
    index: usize,
    type_: impl IntoIterator<Item = ValueType>,
) -> CaptureArg {
    CaptureArg::new(ParamLocal::tuple(
        TupleLocalId(index),
        type_.into_iter().collect(),
    ))
}

pub(crate) fn tuple_function_arg(value: TupleFunction) -> CallArg {
    CallArg::new(value.into())
}

#[cfg(test)]
mod tests {
    use super::{
        bool_arg, bool_function_arg, capture_float, capture_int, capture_tuple, float_arg,
        float_function_arg, int_arg, int_function_arg, int_function_call_arg, nil_arg,
        nil_function_arg, string_arg, string_function_arg, tuple_arg, tuple_function_arg,
    };
    use crate::plan::{CallArg, CaptureArg, Expr, ParamLocal};
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
    };

    #[test]
    fn argument_helpers_preserve_call_expressions_and_capture_locals() {
        assert_eq!(int_arg(int(1)), CallArg::new(Expr::from(int(1))));
        assert_eq!(
            capture_int(2),
            CaptureArg::new(ParamLocal::int(crate::plan::IntLocalId(2)))
        );
        assert_eq!(
            int_function_call_arg(int(3)),
            CallArg::new(Expr::from(int(3)))
        );
        assert_eq!(
            int_function_arg(int_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::new(Expr::from(int_function_ref(0, Vec::<ParamLocal>::new(),))),
        );
        assert_eq!(
            string_arg(string("a")),
            CallArg::new(Expr::from(string("a")))
        );
        assert_eq!(
            string_function_arg(string_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::new(Expr::from(
                string_function_ref(0, Vec::<ParamLocal>::new(),)
            )),
        );
        assert_eq!(float_arg(float(1.0)), CallArg::new(Expr::from(float(1.0))));
        assert_eq!(
            capture_float(2),
            CaptureArg::new(ParamLocal::float(crate::plan::FloatLocalId(2)))
        );
        assert_eq!(
            float_function_arg(float_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::new(Expr::from(float_function_ref(0, Vec::<ParamLocal>::new(),))),
        );
        assert_eq!(bool_arg(bool_(true)), CallArg::new(Expr::from(bool_(true))));
        assert_eq!(
            bool_function_arg(bool_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::new(Expr::from(bool_function_ref(0, Vec::<ParamLocal>::new(),))),
        );
        assert_eq!(nil_arg(nil()), CallArg::new(Expr::from(nil())));
        assert_eq!(
            nil_function_arg(nil_function_ref(0, Vec::<ParamLocal>::new())),
            CallArg::new(Expr::from(nil_function_ref(0, Vec::<ParamLocal>::new(),))),
        );

        assert_eq!(
            tuple_arg(tuple([Expr::from(int(1)), Expr::from(string("one"))])),
            CallArg::new(Expr::from(tuple([
                Expr::from(int(1)),
                Expr::from(string("one")),
            ])))
        );
        assert_eq!(
            capture_tuple(
                2,
                [crate::plan::ValueType::Int, crate::plan::ValueType::String]
            ),
            CaptureArg::new(ParamLocal::tuple(
                crate::plan::TupleLocalId(2),
                vec![crate::plan::ValueType::Int, crate::plan::ValueType::String],
            ))
        );
        assert_eq!(
            tuple_function_arg(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [crate::plan::ValueType::Int],
            )),
            CallArg::new(Expr::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [crate::plan::ValueType::Int],
            ))),
        );
    }
}
