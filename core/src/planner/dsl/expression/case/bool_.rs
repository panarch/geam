use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Float, FloatFunction, FunctionFunction, Int, IntFunction, Nil, NilFunction,
    String, StringFunction,
};

pub(crate) fn bool_case_int(subject: Bool, true_: Int, false_: Int) -> Int {
    Int(IntExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_string(subject: Bool, true_: String, false_: String) -> String {
    String(StringExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_float(subject: Bool, true_: Float, false_: Float) -> Float {
    Float(FloatExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_bool(subject: Bool, true_: Bool, false_: Bool) -> Bool {
    Bool(BoolExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_nil(subject: Bool, true_: Nil, false_: Nil) -> Nil {
    Nil(NilExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_int_function(
    subject: Bool,
    true_: IntFunction,
    false_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_string_function(
    subject: Bool,
    true_: StringFunction,
    false_: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_float_function(
    subject: Bool,
    true_: FloatFunction,
    false_: FloatFunction,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_bool_function(
    subject: Bool,
    true_: BoolFunction,
    false_: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_nil_function(
    subject: Bool,
    true_: NilFunction,
    false_: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_function_function(
    subject: Bool,
    true_: FunctionFunction,
    false_: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        bool_case_bool, bool_case_bool_function, bool_case_float, bool_case_float_function,
        bool_case_function_function, bool_case_int, bool_case_int_function, bool_case_nil,
        bool_case_nil_function, bool_case_string, bool_case_string_function,
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
    fn bool_case_helpers_build_result_family_shapes() {
        assert_eq!(
            bool_case_int(bool_(true), int(1), int(0)).0,
            IntExpr::bool_case(bool_(true).into(), int(1).into(), int(0).into()),
        );
        assert_eq!(
            bool_case_string(bool_(true), string("a"), string("b")).0,
            StringExpr::bool_case(bool_(true).into(), string("a").into(), string("b").into()),
        );
        assert_eq!(
            bool_case_float(bool_(true), float(1.0), float(0.0)).0,
            FloatExpr::bool_case(bool_(true).into(), float(1.0).into(), float(0.0).into()),
        );
        assert_eq!(
            bool_case_bool(bool_(true), bool_(true), bool_(false)).0,
            BoolExpr::bool_case(bool_(true).into(), bool_(true).into(), bool_(false).into()),
        );
        assert_eq!(
            bool_case_nil(bool_(true), nil(), nil()).0,
            NilExpr::bool_case(bool_(true).into(), nil().into(), nil().into()),
        );
        assert_eq!(
            bool_case_int_function(
                bool_(true),
                int_function_ref(0, Vec::<ParamLocal>::new()),
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            IntFunctionExpr::bool_case(
                bool_(true).into(),
                int_function_ref(0, Vec::<ParamLocal>::new()).into(),
                int_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_case_string_function(
                bool_(true),
                string_function_ref(0, Vec::<ParamLocal>::new()),
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            StringFunctionExpr::bool_case(
                bool_(true).into(),
                string_function_ref(0, Vec::<ParamLocal>::new()).into(),
                string_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_case_float_function(
                bool_(true),
                float_function_ref(0, Vec::<ParamLocal>::new()),
                float_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            FloatFunctionExpr::bool_case(
                bool_(true).into(),
                float_function_ref(0, Vec::<ParamLocal>::new()).into(),
                float_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_case_bool_function(
                bool_(true),
                bool_function_ref(0, Vec::<ParamLocal>::new()),
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            BoolFunctionExpr::bool_case(
                bool_(true).into(),
                bool_function_ref(0, Vec::<ParamLocal>::new()).into(),
                bool_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_case_nil_function(
                bool_(true),
                nil_function_ref(0, Vec::<ParamLocal>::new()),
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0,
            NilFunctionExpr::bool_case(
                bool_(true).into(),
                nil_function_ref(0, Vec::<ParamLocal>::new()).into(),
                nil_function_ref(1, Vec::<ParamLocal>::new()).into(),
            ),
        );
        assert_eq!(
            bool_case_function_function(
                bool_(true),
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )
            .0,
            FunctionFunctionExpr::bool_case(
                bool_(true).into(),
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
                .into(),
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
