use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, Nil, NilFunction,
    String, StringFunction,
};

pub(crate) fn string_case_int(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, Int)>,
    fallback: Int,
) -> Int {
    Int(IntExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_string(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, String)>,
    fallback: String,
) -> String {
    String(StringExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_float(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, Float)>,
    fallback: Float,
) -> Float {
    Float(FloatExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_bool(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, Bool)>,
    fallback: Bool,
) -> Bool {
    Bool(BoolExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_nil(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, Nil)>,
    fallback: Nil,
) -> Nil {
    Nil(NilExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_int_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, IntFunction)>,
    fallback: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_string_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, StringFunction)>,
    fallback: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_float_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, FloatFunction)>,
    fallback: FloatFunction,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_bool_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, BoolFunction)>,
    fallback: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_nil_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, NilFunction)>,
    fallback: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn string_case_function_function(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, FunctionFunction)>,
    fallback: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        string_case_bool, string_case_bool_function, string_case_float, string_case_float_function,
        string_case_function_function, string_case_int, string_case_int_function, string_case_nil,
        string_case_nil_function, string_case_string, string_case_string_function,
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

    #[test]
    fn string_case_helpers_build_result_family_shapes() {
        assert_eq!(
            string_case_int(string("key"), [("one", int(10))], int(0)).0,
            IntExpr::string_case(
                string("key").into(),
                vec![("one".into(), int(10).into())],
                int(0).into(),
            ),
        );
        assert_eq!(
            string_case_string(string("key"), [("one", string("hit"))], string("miss")).0,
            StringExpr::string_case(
                string("key").into(),
                vec![("one".into(), string("hit").into())],
                string("miss").into(),
            ),
        );
        assert_eq!(
            string_case_float(string("key"), [("one", float(1.0))], float(0.0)).0,
            FloatExpr::string_case(
                string("key").into(),
                vec![("one".into(), float(1.0).into())],
                float(0.0).into(),
            ),
        );
        assert_eq!(
            string_case_bool(string("key"), [("one", bool_(true))], bool_(false)).0,
            BoolExpr::string_case(
                string("key").into(),
                vec![("one".into(), bool_(true).into())],
                bool_(false).into(),
            ),
        );
        assert_eq!(
            string_case_nil(string("key"), [("one", nil())], nil()).0,
            NilExpr::string_case(
                string("key").into(),
                vec![("one".into(), nil().into())],
                nil().into(),
            ),
        );
        assert_eq!(
            string_case_int_function(
                string("key"),
                [("one", int_function_ref(0, Vec::<ParamLocal>::new()))],
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            IntFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    int_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                int_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_case_string_function(
                string("key"),
                [("one", string_function_ref(0, Vec::<ParamLocal>::new()))],
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            StringFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    string_function_ref(0, Vec::<ParamLocal>::new()).into(),
                )],
                string_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_case_float_function(
                string("key"),
                [("one", float_function_ref(0, Vec::<ParamLocal>::new()))],
                float_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            FloatFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    float_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                float_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_case_bool_function(
                string("key"),
                [("one", bool_function_ref(0, Vec::<ParamLocal>::new()))],
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            BoolFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    bool_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                bool_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_case_nil_function(
                string("key"),
                [("one", nil_function_ref(0, Vec::<ParamLocal>::new()))],
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            NilFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    nil_function_ref(0, Vec::<ParamLocal>::new()).into()
                )],
                nil_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            string_case_function_function(
                string("key"),
                [(
                    "one",
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
            FunctionFunctionExpr::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
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
