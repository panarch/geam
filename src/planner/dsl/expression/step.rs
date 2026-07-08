use super::{
    Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, List, Nil, NilFunction, String,
    StringFunction, Tuple,
};
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, Expr, FloatFunctionLocalId, FloatLocalId, IntFunctionLocalId,
    IntLocalId, NilFunctionLocalId, NilLocalId, Step, StringFunctionLocalId, StringLocalId,
    TupleLocalId,
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
    let local = super::local::list_local(local, value.0.element_type().clone());
    Step::let_list(local, name.into(), value.into())
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
    use crate::plan::{
        BoolFunctionLocalId, BoolLocalId, Expr, FloatFunctionLocalId, FloatLocalId,
        IntFunctionLocalId, IntListLocalId, IntLocalId, ListLocal, NilFunctionLocalId, NilLocalId,
        Step, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleLocalId, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, int, int_function_ref, list, nil,
        nil_function_ref, string, string_function_ref, tuple,
    };

    #[test]
    fn step_helpers_build_step_shapes() {
        assert_eq!(
            let_int_step(0, "x", int(1)),
            Step::let_int(IntLocalId(0), "x".into(), int(1).into()),
        );
        assert_eq!(
            let_string_step(1, "name", string("a")),
            Step::let_string(StringLocalId(1), "name".into(), string("a").into()),
        );
        assert_eq!(
            let_float_step(2, "ratio", float(1.0)),
            Step::let_float(FloatLocalId(2), "ratio".into(), float(1.0).into()),
        );
        assert_eq!(
            let_bool_step(3, "ok", bool_(true)),
            Step::let_bool(BoolLocalId(3), "ok".into(), bool_(true).into()),
        );
        assert_eq!(
            let_nil_step(4, "done", nil()),
            Step::let_nil(NilLocalId(4), "done".into(), nil().into()),
        );
        assert_eq!(
            let_tuple_step(
                5,
                "pair",
                tuple([Expr::from(int(1)), Expr::from(string("one"))])
            ),
            Step::let_tuple(
                TupleLocalId(5),
                "pair".into(),
                tuple([Expr::from(int(1)), Expr::from(string("one"))]).into(),
            ),
        );
        assert_eq!(
            let_list_step(6, "values", list([int(1)], ValueType::Int)),
            Step::let_list(
                ListLocal::int(IntListLocalId(6)),
                "values".into(),
                list([int(1)], ValueType::Int).into(),
            ),
        );
        assert_eq!(
            let_list_step(7, "names", list([string("a")], ValueType::String)),
            Step::let_list(
                ListLocal::string(StringListLocalId(7)),
                "names".into(),
                list([string("a")], ValueType::String).into(),
            ),
        );
        assert_eq!(
            let_int_function_step(
                0,
                "f",
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            ),
            Step::let_int_function(
                IntFunctionLocalId(0),
                "f".into(),
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]).into(),
            ),
        );
        assert_eq!(
            let_string_function_step(
                1,
                "f",
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            ),
            Step::let_string_function(
                StringFunctionLocalId(1),
                "f".into(),
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                )
                .into(),
            ),
        );
        assert_eq!(
            let_float_function_step(
                2,
                "f",
                float_function_ref(
                    0,
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
                ),
            ),
            Step::let_float_function(
                FloatFunctionLocalId(2),
                "f".into(),
                float_function_ref(
                    0,
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
                )
                .into(),
            ),
        );
        assert_eq!(
            let_bool_function_step(
                3,
                "f",
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            ),
            Step::let_bool_function(
                BoolFunctionLocalId(3),
                "f".into(),
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))])
                    .into(),
            ),
        );
        assert_eq!(
            let_nil_function_step(
                4,
                "f",
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            ),
            Step::let_nil_function(
                NilFunctionLocalId(4),
                "f".into(),
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]).into(),
            ),
        );
        assert_eq!(evaluate_step(int(1)), Step::evaluate(Expr::from(int(1))));
    }
}
