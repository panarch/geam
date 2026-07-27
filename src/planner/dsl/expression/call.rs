use super::{Bool, Float, Int, IntFunction, List, Nil, String};
use crate::plan::{
    BoolExpr, CallArg, FloatExpr, FunctionInstantiation, FunctionShape, FunctionType, IntExpr,
    IntFunctionExpr, ListExpr, NilExpr, StringExpr, ValueShape, ValueType,
    monomorphic_function_instantiation,
};

fn instantiation(template: usize, args: &[CallArg], return_: ValueShape) -> FunctionInstantiation {
    monomorphic_function_instantiation(
        template,
        FunctionShape::new(args.iter().map(CallArg::parameter_shape).collect(), return_),
    )
}

pub(crate) fn call_int(function: usize, args: impl IntoIterator<Item = CallArg>) -> Int {
    let args = args.into_iter().collect::<Vec<_>>();
    Int(IntExpr::call(
        instantiation(function, &args, ValueShape::Int),
        args,
    ))
}

pub(crate) fn call_int_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> Int {
    let args = args.into_iter().collect::<Vec<_>>();
    Int(IntExpr::call_at(
        instantiation(function, &args, ValueShape::Int),
        args,
        site,
    ))
}

pub(crate) fn call_string(function: usize, args: impl IntoIterator<Item = CallArg>) -> String {
    let args = args.into_iter().collect::<Vec<_>>();
    String(StringExpr::call(
        instantiation(function, &args, ValueShape::String),
        args,
    ))
}

pub(crate) fn call_float(function: usize, args: impl IntoIterator<Item = CallArg>) -> Float {
    let args = args.into_iter().collect::<Vec<_>>();
    Float(FloatExpr::call(
        instantiation(function, &args, ValueShape::Float),
        args,
    ))
}

pub(crate) fn call_bool(function: usize, args: impl IntoIterator<Item = CallArg>) -> Bool {
    let args = args.into_iter().collect::<Vec<_>>();
    Bool(BoolExpr::call(
        instantiation(function, &args, ValueShape::Bool),
        args,
    ))
}

pub(crate) fn call_bool_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> Bool {
    let args = args.into_iter().collect::<Vec<_>>();
    Bool(BoolExpr::call_at(
        instantiation(function, &args, ValueShape::Bool),
        args,
        site,
    ))
}

pub(crate) fn call_nil(function: usize, args: impl IntoIterator<Item = CallArg>) -> Nil {
    let args = args.into_iter().collect::<Vec<_>>();
    Nil(NilExpr::call(
        instantiation(function, &args, ValueShape::Nil),
        args,
    ))
}

pub(crate) fn call_list(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    element_type: ValueType,
) -> List {
    let args = args.into_iter().collect::<Vec<_>>();
    List(ListExpr::call(
        instantiation(
            function,
            &args,
            ValueShape::List(Box::new(ValueShape::from_value_type(element_type.clone()))),
        ),
        args,
        ValueShape::from_value_type(element_type),
    ))
}

pub(crate) fn call_int_returning_function(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    return_type: FunctionType,
) -> IntFunction {
    let args = args.into_iter().collect::<Vec<_>>();
    IntFunction(IntFunctionExpr::call(
        instantiation(
            function,
            &args,
            ValueShape::Function(Box::new(FunctionShape::from_function_type(
                return_type.clone(),
            ))),
        ),
        args,
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

pub(crate) fn call_int_function_at(
    function: IntFunction,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> Int {
    Int(IntExpr::function_call_at(
        function.into(),
        args.into_iter().collect(),
        site,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        call_bool, call_float, call_int, call_int_function, call_int_returning_function, call_list,
        call_nil, call_string, instantiation,
    };
    use crate::plan::{
        BoolExpr, FloatExpr, FunctionShape, FunctionType, IntExpr, IntFunctionExpr, ListExpr,
        NilExpr, ParamLocal, StringExpr, ValueShape, ValueType,
    };
    use crate::planner::dsl::expression::{int, int_arg, int_function_ref, string, string_arg};

    #[test]
    fn direct_call_helpers_build_call_shapes() {
        assert_eq!(
            call_int(0, [int_arg(int(1))]).0,
            IntExpr::call(
                instantiation(0, &[int_arg(int(1))], ValueShape::Int),
                vec![int_arg(int(1))]
            ),
        );
        assert_eq!(
            call_string(1, [string_arg(string("a"))]).0,
            StringExpr::call(
                instantiation(1, &[string_arg(string("a"))], ValueShape::String),
                vec![string_arg(string("a"))]
            ),
        );
        assert_eq!(
            call_float(2, []).0,
            FloatExpr::call(instantiation(2, &[], ValueShape::Float), Vec::new()),
        );
        assert_eq!(
            call_bool(3, []).0,
            BoolExpr::call(instantiation(3, &[], ValueShape::Bool), Vec::new()),
        );
        assert_eq!(
            call_nil(4, []).0,
            NilExpr::call(instantiation(4, &[], ValueShape::Nil), Vec::new()),
        );
        assert_eq!(
            call_list(5, [], ValueType::Int).0,
            ListExpr::call(
                instantiation(5, &[], ValueShape::List(Box::new(ValueShape::Int)),),
                Vec::new(),
                ValueShape::Int,
            ),
        );
    }

    #[test]
    fn function_call_helpers_build_call_shapes() {
        let return_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            call_int_returning_function(6, [int_arg(int(1))], return_type.clone()).0,
            IntFunctionExpr::call(
                instantiation(
                    6,
                    &[int_arg(int(1))],
                    ValueShape::Function(Box::new(FunctionShape::from_function_type(
                        return_type.clone(),
                    ))),
                ),
                vec![int_arg(int(1))],
                return_type,
            ),
        );
        assert_eq!(
            call_int_function(
                int_function_ref(0, Vec::<ParamLocal>::new()),
                [int_arg(int(1))],
            )
            .0,
            IntExpr::function_call(
                int_function_ref(0, Vec::<ParamLocal>::new()).into(),
                vec![int_arg(int(1))],
            ),
        );
    }
}
