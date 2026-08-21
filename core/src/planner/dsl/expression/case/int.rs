use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, Nil, NilFunction,
    String, StringFunction,
};
use num_bigint::BigInt;

pub(crate) fn int_case_int(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Int)>,
    fallback: Int,
) -> Int {
    Int(IntExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_string(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, String)>,
    fallback: String,
) -> String {
    String(StringExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_float(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Float)>,
    fallback: Float,
) -> Float {
    Float(FloatExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_bool(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Bool)>,
    fallback: Bool,
) -> Bool {
    Bool(BoolExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_nil(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Nil)>,
    fallback: Nil,
) -> Nil {
    Nil(NilExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_int_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, IntFunction)>,
    fallback: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_string_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, StringFunction)>,
    fallback: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_float_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, FloatFunction)>,
    fallback: FloatFunction,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_bool_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, BoolFunction)>,
    fallback: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_nil_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, NilFunction)>,
    fallback: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_function_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, FunctionFunction)>,
    fallback: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        int_case_bool, int_case_bool_function, int_case_float, int_case_float_function,
        int_case_function_function, int_case_int, int_case_int_function, int_case_nil,
        int_case_nil_function, int_case_string, int_case_string_function,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, NilExpr,
        NilFunctionExpr, ParamLocal, StringExpr, StringFunctionExpr, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, function_function_ref, int,
        int_function_ref, nil, nil_function_ref, string, string_function_ref,
    };
    use num_bigint::BigInt;

    #[test]
    fn int_case_helpers_build_result_family_shapes() {
        assert_eq!(
            int_case_int(int(1), [(1, int(10))], int(0)).0,
            IntExpr::int_case(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                int(0).into(),
            ),
        );
        assert_eq!(
            int_case_string(int(1), [(1, string("one"))], string("other")).0,
            StringExpr::int_case(
                int(1).into(),
                vec![(BigInt::from(1), string("one").into())],
                string("other").into(),
            ),
        );
        assert_eq!(
            int_case_float(int(1), [(1, float(1.0))], float(0.0)).0,
            FloatExpr::int_case(
                int(1).into(),
                vec![(BigInt::from(1), float(1.0).into())],
                float(0.0).into(),
            ),
        );
        assert_eq!(
            int_case_bool(int(1), [(1, bool_(true))], bool_(false)).0,
            BoolExpr::int_case(
                int(1).into(),
                vec![(BigInt::from(1), bool_(true).into())],
                bool_(false).into(),
            ),
        );
        assert_eq!(
            int_case_nil(int(1), [(1, nil())], nil()).0,
            NilExpr::int_case(
                int(1).into(),
                vec![(BigInt::from(1), nil().into())],
                nil().into(),
            ),
        );
        assert_eq!(
            int_case_int_function(
                int(1),
                [(1, int_function_ref(0, Vec::<ParamLocal>::new()))],
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            IntFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    int_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                int_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            int_case_string_function(
                int(1),
                [(1, string_function_ref(0, Vec::<ParamLocal>::new()))],
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            StringFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    string_function_ref(0, Vec::<ParamLocal>::new()).into(),
                )],
                string_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            int_case_float_function(
                int(1),
                [(1, float_function_ref(0, Vec::<ParamLocal>::new()))],
                float_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            FloatFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    float_function_ref(0, Vec::<ParamLocal>::new()).into(),
                )],
                float_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            int_case_bool_function(
                int(1),
                [(1, bool_function_ref(0, Vec::<ParamLocal>::new()))],
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            BoolFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    bool_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                bool_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            int_case_nil_function(
                int(1),
                [(1, nil_function_ref(0, Vec::<ParamLocal>::new()))],
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            NilFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    nil_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                nil_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            int_case_function_function(
                int(1),
                [(
                    1,
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                )],
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )
            .0,
            FunctionFunctionExpr::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )
                    .into(),
                )],
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
                .into(),
            ),
        );
    }
}
