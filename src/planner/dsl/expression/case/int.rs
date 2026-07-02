use crate::plan::{
    BoolExpr, BoolFunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, FunctionFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction,
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
        int_case_bool, int_case_bool_function, int_case_function_function, int_case_int,
        int_case_int_function, int_case_nil, int_case_nil_function, int_case_string,
        int_case_string_function,
    };
    use crate::plan::{
        BoolExprKind, BoolFunctionExprKind, FunctionFunctionId, FunctionType, IntExprKind,
        IntFunctionExprKind, IntFunctionFunctionId, NilExprKind, NilFunctionExprKind, ParamLocal,
        StringExprKind, StringFunctionExprKind, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, function_function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref,
    };

    #[test]
    fn int_case_helpers_build_result_family_shapes() {
        assert!(matches!(
            int_case_int(int(1), [(1, int(10))], int(0)).0.kind(),
            IntExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_string(int(1), [(1, string("one"))], string("other"))
                .0
                .kind(),
            StringExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_bool(int(1), [(1, bool_(true))], bool_(false))
                .0
                .kind(),
            BoolExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_nil(int(1), [(1, nil())], nil()).0.kind(),
            NilExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_int_function(
                int(1),
                [(1, int_function_ref(0, Vec::<ParamLocal>::new()))],
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            IntFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_string_function(
                int(1),
                [(1, string_function_ref(0, Vec::<ParamLocal>::new()))],
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            StringFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_bool_function(
                int(1),
                [(1, bool_function_ref(0, Vec::<ParamLocal>::new()))],
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            BoolFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_nil_function(
                int(1),
                [(1, nil_function_ref(0, Vec::<ParamLocal>::new()))],
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            NilFunctionExprKind::IntCase { .. },
        ));
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
            .0
            .type_(),
            &FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
            ),
        );
    }
}
