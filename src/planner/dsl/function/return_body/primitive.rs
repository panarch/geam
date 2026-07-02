use crate::plan::{
    BoolFunctionId, BoolReturn, CallArg, FloatFunctionId, FloatReturn, IntFunctionId, IntReturn,
    NilFunctionId, NilReturn, ReturnBody, Step, StringFunctionId, StringReturn,
};
use crate::planner::dsl::expression::{Bool, Float, Int, Nil, String};
use num_bigint::BigInt;

pub(crate) fn int_return_expr(expression: Int) -> IntReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn int_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> IntReturn {
    ReturnBody::tail_call(IntFunctionId(function), args.into_iter().collect())
}

pub(crate) fn int_return_bool_case(
    subject: Bool,
    true_: IntReturn,
    false_: IntReturn,
) -> IntReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn int_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, IntReturn)>,
    fallback: IntReturn,
) -> IntReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn int_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, IntReturn)>,
    fallback: IntReturn,
) -> IntReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn int_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, IntReturn)>,
    fallback: IntReturn,
) -> IntReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn int_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: IntReturn,
) -> IntReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn bool_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> BoolReturn {
    ReturnBody::tail_call(BoolFunctionId(function), args.into_iter().collect())
}

pub(crate) fn bool_return_expr(expression: Bool) -> BoolReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn bool_return_bool_case(
    subject: Bool,
    true_: BoolReturn,
    false_: BoolReturn,
) -> BoolReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn bool_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, BoolReturn)>,
    fallback: BoolReturn,
) -> BoolReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn bool_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, BoolReturn)>,
    fallback: BoolReturn,
) -> BoolReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn bool_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, BoolReturn)>,
    fallback: BoolReturn,
) -> BoolReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn bool_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: BoolReturn,
) -> BoolReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn string_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> StringReturn {
    ReturnBody::tail_call(StringFunctionId(function), args.into_iter().collect())
}

pub(crate) fn string_return_expr(expression: String) -> StringReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn float_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> FloatReturn {
    ReturnBody::tail_call(FloatFunctionId(function), args.into_iter().collect())
}

pub(crate) fn float_return_expr(expression: Float) -> FloatReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn float_return_bool_case(
    subject: Bool,
    true_: FloatReturn,
    false_: FloatReturn,
) -> FloatReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn float_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, FloatReturn)>,
    fallback: FloatReturn,
) -> FloatReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn float_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, FloatReturn)>,
    fallback: FloatReturn,
) -> FloatReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn float_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, FloatReturn)>,
    fallback: FloatReturn,
) -> FloatReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn float_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: FloatReturn,
) -> FloatReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn string_return_bool_case(
    subject: Bool,
    true_: StringReturn,
    false_: StringReturn,
) -> StringReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn string_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, StringReturn)>,
    fallback: StringReturn,
) -> StringReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn string_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, StringReturn)>,
    fallback: StringReturn,
) -> StringReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn string_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, StringReturn)>,
    fallback: StringReturn,
) -> StringReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn string_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: StringReturn,
) -> StringReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn nil_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> NilReturn {
    ReturnBody::tail_call(NilFunctionId(function), args.into_iter().collect())
}

pub(crate) fn nil_return_expr(expression: Nil) -> NilReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn nil_return_bool_case(
    subject: Bool,
    true_: NilReturn,
    false_: NilReturn,
) -> NilReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn nil_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, NilReturn)>,
    fallback: NilReturn,
) -> NilReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn nil_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, NilReturn)>,
    fallback: NilReturn,
) -> NilReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn nil_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, NilReturn)>,
    fallback: NilReturn,
) -> NilReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn nil_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: NilReturn,
) -> NilReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

#[cfg(test)]
mod tests {
    use super::{
        bool_return_block, bool_return_bool_case, bool_return_expr, bool_return_float_case,
        bool_return_int_case, bool_return_string_case, bool_return_tail_call, float_return_block,
        float_return_bool_case, float_return_expr, float_return_float_case, float_return_int_case,
        float_return_string_case, float_return_tail_call, int_return_block, int_return_bool_case,
        int_return_expr, int_return_float_case, int_return_int_case, int_return_string_case,
        int_return_tail_call, nil_return_block, nil_return_bool_case, nil_return_expr,
        nil_return_float_case, nil_return_int_case, nil_return_string_case, nil_return_tail_call,
        string_return_block, string_return_bool_case, string_return_expr, string_return_float_case,
        string_return_int_case, string_return_string_case, string_return_tail_call,
    };
    use crate::plan::{CallArg, ReturnBodyKind, Step};
    use crate::planner::dsl::expression::{bool_, float, int, nil, string};

    #[test]
    fn primitive_return_expr_helpers_build_expr_shapes() {
        assert!(matches!(
            int_return_expr(int(1)).kind(),
            ReturnBodyKind::Expr(_),
        ));
        assert!(matches!(
            string_return_expr(string("value")).kind(),
            ReturnBodyKind::Expr(_),
        ));
        assert!(matches!(
            float_return_expr(float(1.0)).kind(),
            ReturnBodyKind::Expr(_),
        ));
        assert!(matches!(
            bool_return_expr(bool_(true)).kind(),
            ReturnBodyKind::Expr(_),
        ));
        assert!(matches!(
            nil_return_expr(nil()).kind(),
            ReturnBodyKind::Expr(_),
        ));
    }

    #[test]
    fn primitive_return_tail_call_helpers_build_tail_call_shapes() {
        assert!(matches!(
            int_return_tail_call(0, Vec::<CallArg>::new()).kind(),
            ReturnBodyKind::TailCall { .. },
        ));
        assert!(matches!(
            string_return_tail_call(0, Vec::<CallArg>::new()).kind(),
            ReturnBodyKind::TailCall { .. },
        ));
        assert!(matches!(
            float_return_tail_call(0, Vec::<CallArg>::new()).kind(),
            ReturnBodyKind::TailCall { .. },
        ));
        assert!(matches!(
            bool_return_tail_call(0, Vec::<CallArg>::new()).kind(),
            ReturnBodyKind::TailCall { .. },
        ));
        assert!(matches!(
            nil_return_tail_call(0, Vec::<CallArg>::new()).kind(),
            ReturnBodyKind::TailCall { .. },
        ));
    }

    #[test]
    fn primitive_return_case_helpers_build_case_shapes() {
        assert!(matches!(
            int_return_bool_case(
                bool_(true),
                int_return_expr(int(1)),
                int_return_expr(int(0)),
            )
            .kind(),
            ReturnBodyKind::BoolCase { .. },
        ));
        assert!(matches!(
            string_return_bool_case(
                bool_(true),
                string_return_expr(string("true")),
                string_return_expr(string("false")),
            )
            .kind(),
            ReturnBodyKind::BoolCase { .. },
        ));
        assert!(matches!(
            float_return_bool_case(
                bool_(true),
                float_return_expr(float(1.0)),
                float_return_expr(float(0.0)),
            )
            .kind(),
            ReturnBodyKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_return_bool_case(
                bool_(true),
                bool_return_expr(bool_(true)),
                bool_return_expr(bool_(false)),
            )
            .kind(),
            ReturnBodyKind::BoolCase { .. },
        ));
        assert!(matches!(
            nil_return_bool_case(bool_(true), nil_return_expr(nil()), nil_return_expr(nil()))
                .kind(),
            ReturnBodyKind::BoolCase { .. },
        ));

        assert!(matches!(
            int_return_int_case(
                int(1),
                [(1, int_return_expr(int(1)))],
                int_return_expr(int(0))
            )
            .kind(),
            ReturnBodyKind::IntCase { .. },
        ));
        assert!(matches!(
            string_return_int_case(
                int(1),
                [(1, string_return_expr(string("one")))],
                string_return_expr(string("other")),
            )
            .kind(),
            ReturnBodyKind::IntCase { .. },
        ));
        assert!(matches!(
            float_return_int_case(
                int(1),
                [(1, float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            )
            .kind(),
            ReturnBodyKind::IntCase { .. },
        ));
        assert!(matches!(
            bool_return_int_case(
                int(1),
                [(1, bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            )
            .kind(),
            ReturnBodyKind::IntCase { .. },
        ));
        assert!(matches!(
            nil_return_int_case(
                int(1),
                [(1, nil_return_expr(nil()))],
                nil_return_expr(nil())
            )
            .kind(),
            ReturnBodyKind::IntCase { .. },
        ));

        assert!(matches!(
            int_return_string_case(
                string("key"),
                [("one", int_return_expr(int(1)))],
                int_return_expr(int(0)),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            int_return_float_case(
                float(1.0),
                [(1.0, int_return_expr(int(1)))],
                int_return_expr(int(0)),
            )
            .kind(),
            ReturnBodyKind::FloatCase { .. },
        ));
        assert!(matches!(
            string_return_string_case(
                string("key"),
                [("one", string_return_expr(string("one")))],
                string_return_expr(string("other")),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            float_return_string_case(
                string("key"),
                [("one", float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            bool_return_string_case(
                string("key"),
                [("one", bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            bool_return_float_case(
                float(1.0),
                [(1.0, bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            )
            .kind(),
            ReturnBodyKind::FloatCase { .. },
        ));
        assert!(matches!(
            string_return_float_case(
                float(1.0),
                [(1.0, string_return_expr(string("one")))],
                string_return_expr(string("other")),
            )
            .kind(),
            ReturnBodyKind::FloatCase { .. },
        ));
        assert!(matches!(
            nil_return_string_case(
                string("key"),
                [("one", nil_return_expr(nil()))],
                nil_return_expr(nil()),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            nil_return_float_case(
                float(1.0),
                [(1.0, nil_return_expr(nil()))],
                nil_return_expr(nil()),
            )
            .kind(),
            ReturnBodyKind::FloatCase { .. },
        ));
        assert!(matches!(
            float_return_float_case(
                float(1.0),
                [(1.0, float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            )
            .kind(),
            ReturnBodyKind::FloatCase { .. },
        ));
    }

    #[test]
    fn primitive_return_block_helpers_build_block_shapes() {
        let step = Step::evaluate(int(0).into());

        assert!(matches!(
            int_return_block([step.clone()], int_return_expr(int(1))).kind(),
            ReturnBodyKind::Block { .. },
        ));
        assert!(matches!(
            string_return_block([step.clone()], string_return_expr(string("value"))).kind(),
            ReturnBodyKind::Block { .. },
        ));
        assert!(matches!(
            float_return_block([step.clone()], float_return_expr(float(1.0))).kind(),
            ReturnBodyKind::Block { .. },
        ));
        assert!(matches!(
            bool_return_block([step.clone()], bool_return_expr(bool_(true))).kind(),
            ReturnBodyKind::Block { .. },
        ));
        assert!(matches!(
            nil_return_block([step], nil_return_expr(nil())).kind(),
            ReturnBodyKind::Block { .. },
        ));
    }
}
