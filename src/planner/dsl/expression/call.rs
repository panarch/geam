use super::{Bool, Float, Int, IntFunction, List, Nil, String};
use crate::plan::{
    BoolExpr, BoolFunctionId, CallArg, FloatExpr, FloatFunctionId, FunctionType, IntExpr,
    IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, ListExpr, ListFunctionId, NilExpr,
    NilFunctionId, StringExpr, StringFunctionId, ValueType,
};

pub(crate) fn call_int(function: usize, args: impl IntoIterator<Item = CallArg>) -> Int {
    Int(IntExpr::call(
        IntFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_string(function: usize, args: impl IntoIterator<Item = CallArg>) -> String {
    String(StringExpr::call(
        StringFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_float(function: usize, args: impl IntoIterator<Item = CallArg>) -> Float {
    Float(FloatExpr::call(
        FloatFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_bool(function: usize, args: impl IntoIterator<Item = CallArg>) -> Bool {
    Bool(BoolExpr::call(
        BoolFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_nil(function: usize, args: impl IntoIterator<Item = CallArg>) -> Nil {
    Nil(NilExpr::call(
        NilFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_list(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    element_type: ValueType,
) -> List {
    List(ListExpr::call(
        ListFunctionId(function),
        args.into_iter().collect(),
        element_type,
    ))
}

pub(crate) fn call_int_returning_function(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    return_type: FunctionType,
) -> IntFunction {
    IntFunction(IntFunctionExpr::call(
        IntFunctionFunctionId(function),
        args.into_iter().collect(),
        return_type,
    ))
}

pub(crate) fn call_int_function(
    function: IntFunction,
    args: impl IntoIterator<Item = CallArg>,
) -> Int {
    Int(IntExpr::function_call(
        function.into(),
        args.into_iter().collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        call_bool, call_float, call_int, call_int_function, call_int_returning_function, call_list,
        call_nil, call_string,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionId, FloatExpr, FloatFunctionId, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, ListExpr, ListFunctionId, NilExpr,
        NilFunctionId, ParamLocal, StringExpr, StringFunctionId, ValueType,
    };
    use crate::planner::dsl::expression::{int, int_arg, int_function_ref, string, string_arg};

    #[test]
    fn direct_call_helpers_build_call_shapes() {
        assert_eq!(
            call_int(0, [int_arg(0, int(1))]).0,
            IntExpr::call(IntFunctionId(0), vec![int_arg(0, int(1))]),
        );
        assert_eq!(
            call_string(1, [string_arg(0, string("a"))]).0,
            StringExpr::call(StringFunctionId(1), vec![string_arg(0, string("a"))]),
        );
        assert_eq!(
            call_float(2, []).0,
            FloatExpr::call(FloatFunctionId(2), Vec::new()),
        );
        assert_eq!(
            call_bool(3, []).0,
            BoolExpr::call(BoolFunctionId(3), Vec::new()),
        );
        assert_eq!(
            call_nil(4, []).0,
            NilExpr::call(NilFunctionId(4), Vec::new()),
        );
        assert_eq!(
            call_list(5, [], ValueType::Int).0,
            ListExpr::call(ListFunctionId(5), Vec::new(), ValueType::Int),
        );
    }

    #[test]
    fn function_call_helpers_build_call_shapes() {
        let return_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            call_int_returning_function(6, [int_arg(0, int(1))], return_type.clone()).0,
            IntFunctionExpr::call(
                IntFunctionFunctionId(6),
                vec![int_arg(0, int(1))],
                return_type,
            ),
        );
        assert_eq!(
            call_int_function(
                int_function_ref(0, Vec::<ParamLocal>::new()),
                [int_arg(0, int(1))],
            )
            .0,
            IntExpr::function_call(
                int_function_ref(0, Vec::<ParamLocal>::new()).into(),
                vec![int_arg(0, int(1))],
            ),
        );
    }
}
