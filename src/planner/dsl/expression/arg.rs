use super::{
    Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction, Tuple, TupleFunction,
};
use crate::plan::{CallArg, CaptureArg, Expr};

pub(crate) fn int_arg(value: Int) -> CallArg {
    CallArg::new(Expr::int(value.into()))
}

pub(crate) fn capture_int(value: Int) -> CaptureArg {
    CaptureArg::new(Expr::int(value.into()))
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

pub(crate) fn capture_float(value: Float) -> CaptureArg {
    CaptureArg::new(Expr::float(value.into()))
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

pub(crate) fn capture_tuple(value: Tuple) -> CaptureArg {
    CaptureArg::new(Expr::tuple(value.into()))
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
    fn argument_helpers_keep_only_source_expressions() {
        assert_eq!(int_arg(int(1)), CallArg::new(Expr::from(int(1))));
        assert_eq!(capture_int(int(2)), CaptureArg::new(Expr::from(int(2))));
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
            capture_float(float(2.0)),
            CaptureArg::new(Expr::from(float(2.0)))
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
            capture_tuple(tuple([Expr::from(int(1)), Expr::from(string("one"))])),
            CaptureArg::new(Expr::from(tuple([
                Expr::from(int(1)),
                Expr::from(string("one")),
            ])))
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
