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
        bool_arg, bool_function_arg, capture_tuple, float_arg, float_function_arg, int_arg,
        int_function_arg, int_function_call_arg, nil_arg, nil_function_arg, string_arg,
        string_function_arg, tuple_arg, tuple_function_arg,
    };
    use crate::plan::{CallArgKind, CaptureArgKind, Expr, ParamLocal};
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
    };

    #[test]
    fn call_arg_helpers_build_typed_arg_shapes() {
        assert!(matches!(int_arg(0, int(1)).kind(), CallArgKind::Int { .. },));
        assert!(matches!(
            int_function_call_arg(0, int(1)).kind(),
            CallArgKind::Int { .. },
        ));
        assert!(matches!(
            int_function_arg(0, int_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::IntFunction { .. },
        ));
        assert!(matches!(
            string_arg(0, string("a")).kind(),
            CallArgKind::String { .. },
        ));
        assert!(matches!(
            string_function_arg(0, string_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::StringFunction { .. },
        ));
        assert!(matches!(
            float_arg(0, float(1.0)).kind(),
            CallArgKind::Float { .. },
        ));
        assert!(matches!(
            float_function_arg(0, float_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::FloatFunction { .. },
        ));
        assert!(matches!(
            bool_arg(0, bool_(true)).kind(),
            CallArgKind::Bool { .. },
        ));
        assert!(matches!(
            bool_function_arg(0, bool_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::BoolFunction { .. },
        ));
        assert!(matches!(nil_arg(0, nil()).kind(), CallArgKind::Nil { .. },));
        assert!(matches!(
            nil_function_arg(0, nil_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::NilFunction { .. },
        ));
        assert!(matches!(
            tuple_arg(0, tuple([Expr::from(int(1)), Expr::from(string("one"))])).kind(),
            CallArgKind::Tuple { .. },
        ));
        assert!(matches!(
            capture_tuple(0, tuple([Expr::from(int(1)), Expr::from(string("one"))])).kind(),
            CaptureArgKind::Tuple { .. },
        ));
        assert!(matches!(
            tuple_function_arg(
                0,
                tuple_function_ref(0, Vec::<ParamLocal>::new(), [crate::plan::ValueType::Int]),
            )
            .kind(),
            CallArgKind::TupleFunction { .. },
        ));
    }
}
