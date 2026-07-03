use super::{
    Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, List, Nil, NilFunction, String,
    StringFunction, Tuple,
};
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, Expr, FloatFunctionLocalId, FloatLocalId, IntFunctionLocalId,
    IntLocalId, ListLocalId, NilFunctionLocalId, NilLocalId, Step, StringFunctionLocalId,
    StringLocalId, TupleLocalId,
};
use ecow::EcoString;

pub(crate) fn let_int_step(local: usize, name: impl Into<EcoString>, value: Int) -> Step {
    Step::let_int(IntLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_step(local: usize, name: impl Into<EcoString>, value: String) -> Step {
    Step::let_string(StringLocalId(local), name.into(), value.into())
}

pub(crate) fn let_float_step(local: usize, name: impl Into<EcoString>, value: Float) -> Step {
    Step::let_float(FloatLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_step(local: usize, name: impl Into<EcoString>, value: Bool) -> Step {
    Step::let_bool(BoolLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_step(local: usize, name: impl Into<EcoString>, value: Nil) -> Step {
    Step::let_nil(NilLocalId(local), name.into(), value.into())
}

pub(crate) fn let_tuple_step(local: usize, name: impl Into<EcoString>, value: Tuple) -> Step {
    Step::let_tuple(TupleLocalId(local), name.into(), value.into())
}

pub(crate) fn let_list_step(local: usize, name: impl Into<EcoString>, value: List) -> Step {
    Step::let_list(ListLocalId(local), name.into(), value.into())
}

pub(crate) fn let_int_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: IntFunction,
) -> Step {
    Step::let_int_function(IntFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: StringFunction,
) -> Step {
    Step::let_string_function(StringFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_float_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: FloatFunction,
) -> Step {
    Step::let_float_function(FloatFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: BoolFunction,
) -> Step {
    Step::let_bool_function(BoolFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: NilFunction,
) -> Step {
    Step::let_nil_function(NilFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn evaluate_step(value: impl Into<Expr>) -> Step {
    Step::evaluate(value.into())
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_step, let_bool_function_step, let_bool_step, let_float_function_step,
        let_float_step, let_int_function_step, let_int_step, let_list_step, let_nil_function_step,
        let_nil_step, let_string_function_step, let_string_step, let_tuple_step,
    };
    use crate::plan::Expr;
    use crate::plan::StepKind;
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, int, int_function_ref, list, nil,
        nil_function_ref, string, string_function_ref, tuple,
    };

    #[test]
    fn step_helpers_build_step_shapes() {
        assert!(matches!(
            let_int_step(0, "x", int(1)).kind(),
            StepKind::LetInt { .. },
        ));
        assert!(matches!(
            let_string_step(0, "x", string("a")).kind(),
            StepKind::LetString { .. },
        ));
        assert!(matches!(
            let_float_step(0, "x", float(1.0)).kind(),
            StepKind::LetFloat { .. },
        ));
        assert!(matches!(
            let_bool_step(0, "x", bool_(true)).kind(),
            StepKind::LetBool { .. },
        ));
        assert!(matches!(
            let_nil_step(0, "x", nil()).kind(),
            StepKind::LetNil { .. },
        ));
        assert!(matches!(
            let_tuple_step(
                0,
                "x",
                tuple([Expr::from(int(1)), Expr::from(string("one"))])
            )
            .kind(),
            StepKind::LetTuple { .. },
        ));
        assert!(matches!(
            let_list_step(0, "x", list([int(1)], crate::plan::ValueType::Int)).kind(),
            StepKind::LetList { .. },
        ));
        assert!(matches!(
            let_int_function_step(
                0,
                "f",
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .kind(),
            StepKind::LetIntFunction { .. },
        ));
        assert!(matches!(
            let_string_function_step(
                0,
                "f",
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            )
            .kind(),
            StepKind::LetStringFunction { .. },
        ));
        assert!(matches!(
            let_float_function_step(
                0,
                "f",
                float_function_ref(
                    0,
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
                ),
            )
            .kind(),
            StepKind::LetFloatFunction { .. },
        ));
        assert!(matches!(
            let_bool_function_step(
                0,
                "f",
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            )
            .kind(),
            StepKind::LetBoolFunction { .. },
        ));
        assert!(matches!(
            let_nil_function_step(
                0,
                "f",
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            )
            .kind(),
            StepKind::LetNilFunction { .. },
        ));
        assert!(matches!(
            evaluate_step(int(1)).kind(),
            StepKind::Evaluate(_),
        ));
    }
}
