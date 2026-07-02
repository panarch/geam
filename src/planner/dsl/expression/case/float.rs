use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, Nil, NilFunction,
    String, StringFunction,
};

pub(crate) fn float_case_int(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, Int)>,
    fallback: Int,
) -> Int {
    Int(IntExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_string(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, String)>,
    fallback: String,
) -> String {
    String(StringExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_float(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, Float)>,
    fallback: Float,
) -> Float {
    Float(FloatExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_bool(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, Bool)>,
    fallback: Bool,
) -> Bool {
    Bool(BoolExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_nil(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, Nil)>,
    fallback: Nil,
) -> Nil {
    Nil(NilExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_int_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, IntFunction)>,
    fallback: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_string_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, StringFunction)>,
    fallback: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_float_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, FloatFunction)>,
    fallback: FloatFunction,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_bool_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, BoolFunction)>,
    fallback: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_nil_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, NilFunction)>,
    fallback: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn float_case_function_function(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, FunctionFunction)>,
    fallback: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::float_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value, branch.into()))
            .collect(),
        fallback.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        float_case_bool, float_case_bool_function, float_case_float, float_case_float_function,
        float_case_function_function, float_case_int, float_case_int_function, float_case_nil,
        float_case_nil_function, float_case_string, float_case_string_function,
    };
    use crate::plan::{
        BoolExprKind, BoolFunctionExprKind, FloatExprKind, FloatFunctionExprKind,
        FunctionFunctionId, FunctionType, IntExprKind, IntFunctionExprKind, IntFunctionFunctionId,
        NilExprKind, NilFunctionExprKind, ParamLocal, StringExprKind, StringFunctionExprKind,
        ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, function_function_ref, int,
        int_function_ref, nil, nil_function_ref, string, string_function_ref,
    };

    #[test]
    fn float_case_helpers_build_result_family_shapes() {
        assert!(matches!(
            float_case_int(float(1.0), [(1.0, int(10))], int(0))
                .0
                .kind(),
            IntExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_string(float(1.0), [(1.0, string("hit"))], string("miss"))
                .0
                .kind(),
            StringExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_float(float(1.0), [(1.0, float(10.0))], float(0.0))
                .0
                .kind(),
            FloatExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_bool(float(1.0), [(1.0, bool_(true))], bool_(false))
                .0
                .kind(),
            BoolExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_nil(float(1.0), [(1.0, nil())], nil()).0.kind(),
            NilExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_int_function(
                float(1.0),
                [(1.0, int_function_ref(0, Vec::<ParamLocal>::new()))],
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            IntFunctionExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_string_function(
                float(1.0),
                [(1.0, string_function_ref(0, Vec::<ParamLocal>::new()))],
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            StringFunctionExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_float_function(
                float(1.0),
                [(1.0, float_function_ref(0, Vec::<ParamLocal>::new()))],
                float_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            FloatFunctionExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_bool_function(
                float(1.0),
                [(1.0, bool_function_ref(0, Vec::<ParamLocal>::new()))],
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            BoolFunctionExprKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_case_nil_function(
                float(1.0),
                [(1.0, nil_function_ref(0, Vec::<ParamLocal>::new()))],
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            NilFunctionExprKind::FloatCase { .. },
        ));
        assert_eq!(
            float_case_function_function(
                float(1.0),
                [(
                    1.0,
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
