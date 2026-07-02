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
    fn bool_case_helpers_build_result_family_shapes() {
        assert!(matches!(
            bool_case_int(bool_(true), int(1), int(0)).0.kind(),
            IntExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_string(bool_(true), string("a"), string("b"))
                .0
                .kind(),
            StringExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_float(bool_(true), float(1.0), float(0.0))
                .0
                .kind(),
            FloatExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_bool(bool_(true), bool_(true), bool_(false))
                .0
                .kind(),
            BoolExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_nil(bool_(true), nil(), nil()).0.kind(),
            NilExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_int_function(
                bool_(true),
                int_function_ref(0, Vec::<ParamLocal>::new()),
                int_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            IntFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_string_function(
                bool_(true),
                string_function_ref(0, Vec::<ParamLocal>::new()),
                string_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            StringFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_float_function(
                bool_(true),
                float_function_ref(0, Vec::<ParamLocal>::new()),
                float_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            FloatFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_bool_function(
                bool_(true),
                bool_function_ref(0, Vec::<ParamLocal>::new()),
                bool_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            BoolFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_nil_function(
                bool_(true),
                nil_function_ref(0, Vec::<ParamLocal>::new()),
                nil_function_ref(1, Vec::<ParamLocal>::new()),
            )
            .0
            .kind(),
            NilFunctionExprKind::BoolCase { .. },
        ));
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
