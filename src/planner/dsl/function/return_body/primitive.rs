use super::FunctionReturn;
use crate::plan::{
    BoolFunctionId, BoolReturn, CallArg, FloatFunctionId, FloatReturn, IntFunctionId, IntReturn,
    ListFunctionId, ListReturn, NilFunctionId, NilReturn, ReturnBody, Step, StringFunctionId,
    StringReturn, ValueType,
};
use crate::planner::dsl::expression::{Bool, Float, Int, List, Nil, String};
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

pub(crate) fn list_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    element_type: ValueType,
) -> ListReturn {
    ListReturn::tail_call(
        ListFunctionId::from_item_type(function, element_type),
        args.into_iter().collect(),
    )
}

pub(crate) fn list_return_expr(expression: List) -> ListReturn {
    ListReturn::expr(expression.into())
}

pub(crate) fn list_return_bool_case(
    subject: Bool,
    true_: ListReturn,
    false_: ListReturn,
) -> ListReturn {
    ListReturn::try_bool_case(subject.into(), true_, false_)
        .expect("list return case branches must share an item family")
}

pub(crate) fn list_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, ListReturn)>,
    fallback: ListReturn,
) -> ListReturn {
    ListReturn::try_int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
    .expect("list return case branches must share an item family")
}

pub(crate) fn list_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, ListReturn)>,
    fallback: ListReturn,
) -> ListReturn {
    ListReturn::try_string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
    .expect("list return case branches must share an item family")
}

pub(crate) fn list_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, ListReturn)>,
    fallback: ListReturn,
) -> ListReturn {
    ListReturn::try_float_case(subject.into(), clauses.into_iter().collect(), fallback)
        .expect("list return case branches must share an item family")
}

pub(crate) fn list_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: ListReturn,
) -> ListReturn {
    ListReturn::try_block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_list(element_type: ValueType, body: ListReturn) -> FunctionReturn {
    FunctionReturn::List { element_type, body }
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
        int_return_tail_call, list_return_block, list_return_bool_case, list_return_expr,
        list_return_float_case, list_return_int_case, list_return_string_case,
        list_return_tail_call, nil_return_block, nil_return_bool_case, nil_return_expr,
        nil_return_float_case, nil_return_int_case, nil_return_string_case, nil_return_tail_call,
        string_return_block, string_return_bool_case, string_return_expr, string_return_float_case,
        string_return_int_case, string_return_string_case, string_return_tail_call,
    };
    use crate::plan::{
        BoolFunctionId, CallArg, FloatFunctionId, IntFunctionId, ListFunctionId, ListReturn,
        NilFunctionId, ReturnBody, Step, StringFunctionId,
    };
    use crate::planner::dsl::expression::{bool_, float, int, list, nil, string};
    use num_bigint::BigInt;

    #[test]
    fn primitive_return_expr_helpers_build_expr_shapes() {
        assert_eq!(int_return_expr(int(1)), ReturnBody::expr(int(1).into()));
        assert_eq!(
            string_return_expr(string("value")),
            ReturnBody::expr(string("value").into()),
        );
        assert_eq!(
            float_return_expr(float(1.0)),
            ReturnBody::expr(float(1.0).into())
        );
        assert_eq!(
            bool_return_expr(bool_(true)),
            ReturnBody::expr(bool_(true).into()),
        );
        assert_eq!(nil_return_expr(nil()), ReturnBody::expr(nil().into()));
        assert_eq!(
            list_return_expr(list([int(1)], crate::plan::ValueType::Int)),
            ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
        );
    }

    #[test]
    fn primitive_return_tail_call_helpers_build_tail_call_shapes() {
        assert_eq!(
            int_return_tail_call(0, Vec::<CallArg>::new()),
            ReturnBody::tail_call(IntFunctionId(0), Vec::new()),
        );
        assert_eq!(
            string_return_tail_call(1, Vec::<CallArg>::new()),
            ReturnBody::tail_call(StringFunctionId(1), Vec::new()),
        );
        assert_eq!(
            float_return_tail_call(2, Vec::<CallArg>::new()),
            ReturnBody::tail_call(FloatFunctionId(2), Vec::new()),
        );
        assert_eq!(
            bool_return_tail_call(3, Vec::<CallArg>::new()),
            ReturnBody::tail_call(BoolFunctionId(3), Vec::new()),
        );
        assert_eq!(
            nil_return_tail_call(4, Vec::<CallArg>::new()),
            ReturnBody::tail_call(NilFunctionId(4), Vec::new()),
        );
        assert_eq!(
            list_return_tail_call(5, Vec::<CallArg>::new(), crate::plan::ValueType::Int),
            ListReturn::tail_call(
                ListFunctionId::from_item_type(5, crate::plan::ValueType::Int),
                Vec::new(),
            ),
        );
    }

    #[test]
    fn primitive_return_case_helpers_build_case_shapes() {
        assert_eq!(
            int_return_bool_case(
                bool_(true),
                int_return_expr(int(1)),
                int_return_expr(int(0)),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                int_return_expr(int(1)),
                int_return_expr(int(0)),
            ),
        );
        assert_eq!(
            string_return_bool_case(
                bool_(true),
                string_return_expr(string("true")),
                string_return_expr(string("false")),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                string_return_expr(string("true")),
                string_return_expr(string("false")),
            ),
        );
        assert_eq!(
            float_return_bool_case(
                bool_(true),
                float_return_expr(float(1.0)),
                float_return_expr(float(0.0)),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                float_return_expr(float(1.0)),
                float_return_expr(float(0.0)),
            ),
        );
        assert_eq!(
            bool_return_bool_case(
                bool_(true),
                bool_return_expr(bool_(true)),
                bool_return_expr(bool_(false)),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                bool_return_expr(bool_(true)),
                bool_return_expr(bool_(false)),
            ),
        );
        assert_eq!(
            nil_return_bool_case(bool_(true), nil_return_expr(nil()), nil_return_expr(nil())),
            ReturnBody::bool_case(
                bool_(true).into(),
                nil_return_expr(nil()),
                nil_return_expr(nil()),
            ),
        );
        assert_eq!(
            list_return_bool_case(
                bool_(true),
                list_return_expr(list([int(1)], crate::plan::ValueType::Int)),
                list_return_expr(list([int(0)], crate::plan::ValueType::Int)),
            ),
            ListReturn::try_bool_case(
                bool_(true).into(),
                ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
                ListReturn::expr(list([int(0)], crate::plan::ValueType::Int).into()),
            )
            .expect("list bool-case branches should share an item family"),
        );

        assert_eq!(
            int_return_int_case(
                int(1),
                [(1, int_return_expr(int(1)))],
                int_return_expr(int(0))
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), int_return_expr(int(1)))],
                int_return_expr(int(0)),
            ),
        );
        assert_eq!(
            string_return_int_case(
                int(1),
                [(1, string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
        );
        assert_eq!(
            float_return_int_case(
                int(1),
                [(1, float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
        );
        assert_eq!(
            bool_return_int_case(
                int(1),
                [(1, bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
        );
        assert_eq!(
            nil_return_int_case(
                int(1),
                [(1, nil_return_expr(nil()))],
                nil_return_expr(nil())
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), nil_return_expr(nil()))],
                nil_return_expr(nil()),
            ),
        );
        assert_eq!(
            list_return_int_case(
                int(1),
                [(
                    1,
                    list_return_expr(list([int(1)], crate::plan::ValueType::Int))
                )],
                list_return_expr(list([int(0)], crate::plan::ValueType::Int)),
            ),
            ListReturn::try_int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
                )],
                ListReturn::expr(list([int(0)], crate::plan::ValueType::Int).into()),
            )
            .expect("list int-case branches should share an item family"),
        );

        assert_eq!(
            int_return_string_case(
                string("key"),
                [("one", int_return_expr(int(1)))],
                int_return_expr(int(0)),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), int_return_expr(int(1)))],
                int_return_expr(int(0)),
            ),
        );
        assert_eq!(
            int_return_float_case(
                float(1.0),
                [(1.0, int_return_expr(int(1)))],
                int_return_expr(int(0)),
            ),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, int_return_expr(int(1)))],
                int_return_expr(int(0)),
            ),
        );
        assert_eq!(
            string_return_string_case(
                string("key"),
                [("one", string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
        );
        assert_eq!(
            float_return_string_case(
                string("key"),
                [("one", float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
        );
        assert_eq!(
            bool_return_string_case(
                string("key"),
                [("one", bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
        );
        assert_eq!(
            bool_return_float_case(
                float(1.0),
                [(1.0, bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, bool_return_expr(bool_(true)))],
                bool_return_expr(bool_(false)),
            ),
        );
        assert_eq!(
            string_return_float_case(
                float(1.0),
                [(1.0, string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, string_return_expr(string("one")))],
                string_return_expr(string("other")),
            ),
        );
        assert_eq!(
            nil_return_string_case(
                string("key"),
                [("one", nil_return_expr(nil()))],
                nil_return_expr(nil()),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), nil_return_expr(nil()))],
                nil_return_expr(nil()),
            ),
        );
        assert_eq!(
            list_return_string_case(
                string("key"),
                [(
                    "one",
                    list_return_expr(list([int(1)], crate::plan::ValueType::Int))
                )],
                list_return_expr(list([int(0)], crate::plan::ValueType::Int)),
            ),
            ListReturn::try_string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
                )],
                ListReturn::expr(list([int(0)], crate::plan::ValueType::Int).into()),
            )
            .expect("list string-case branches should share an item family"),
        );
        assert_eq!(
            nil_return_float_case(
                float(1.0),
                [(1.0, nil_return_expr(nil()))],
                nil_return_expr(nil()),
            ),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, nil_return_expr(nil()))],
                nil_return_expr(nil()),
            ),
        );
        assert_eq!(
            float_return_float_case(
                float(1.0),
                [(1.0, float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, float_return_expr(float(1.0)))],
                float_return_expr(float(0.0)),
            ),
        );
        assert_eq!(
            list_return_float_case(
                float(1.0),
                [(
                    1.0,
                    list_return_expr(list([int(1)], crate::plan::ValueType::Int))
                )],
                list_return_expr(list([int(0)], crate::plan::ValueType::Int)),
            ),
            ListReturn::try_float_case(
                float(1.0).into(),
                vec![(
                    1.0,
                    ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
                )],
                ListReturn::expr(list([int(0)], crate::plan::ValueType::Int).into()),
            )
            .expect("list float-case branches should share an item family"),
        );
    }

    #[test]
    fn primitive_return_block_helpers_build_block_shapes() {
        let step = Step::evaluate(int(0).into());

        assert_eq!(
            int_return_block([step.clone()], int_return_expr(int(1))),
            ReturnBody::block(vec![step.clone()], int_return_expr(int(1))),
        );
        assert_eq!(
            string_return_block([step.clone()], string_return_expr(string("value"))),
            ReturnBody::block(vec![step.clone()], string_return_expr(string("value"))),
        );
        assert_eq!(
            float_return_block([step.clone()], float_return_expr(float(1.0))),
            ReturnBody::block(vec![step.clone()], float_return_expr(float(1.0))),
        );
        assert_eq!(
            bool_return_block([step.clone()], bool_return_expr(bool_(true))),
            ReturnBody::block(vec![step.clone()], bool_return_expr(bool_(true))),
        );
        assert_eq!(
            nil_return_block([step.clone()], nil_return_expr(nil())),
            ReturnBody::block(vec![step], nil_return_expr(nil())),
        );
        assert_eq!(
            list_return_block(
                Vec::<Step>::new(),
                list_return_expr(list([int(1)], crate::plan::ValueType::Int)),
            ),
            ListReturn::try_block(
                Vec::new(),
                ListReturn::expr(list([int(1)], crate::plan::ValueType::Int).into()),
            ),
        );
    }
}
