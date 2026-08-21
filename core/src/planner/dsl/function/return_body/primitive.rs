use super::{FunctionReturn, tail_call_instantiation};
use crate::plan::{
    BoolReturn, CallArg, FloatReturn, IntReturn, ListReturn, NilReturn, ReturnBody, Step,
    StringReturn, UtfCodepointReturn, ValueShape, ValueType,
};
use crate::planner::dsl::expression::{Bool, Float, Int, List, Nil, String, UtfCodepoint};
use num_bigint::BigInt;

pub(crate) fn int_return_expr(expression: Int) -> IntReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn utf_codepoint_return_expr(expression: UtfCodepoint) -> UtfCodepointReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn utf_codepoint_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> UtfCodepointReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::UtfCodepoint),
        args,
    )
}

pub(crate) fn utf_codepoint_return_bool_case(
    subject: Bool,
    true_: UtfCodepointReturn,
    false_: UtfCodepointReturn,
) -> UtfCodepointReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn utf_codepoint_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, UtfCodepointReturn)>,
    fallback: UtfCodepointReturn,
) -> UtfCodepointReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn utf_codepoint_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, UtfCodepointReturn)>,
    fallback: UtfCodepointReturn,
) -> UtfCodepointReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn utf_codepoint_return_float_case(
    subject: Float,
    clauses: impl IntoIterator<Item = (f64, UtfCodepointReturn)>,
    fallback: UtfCodepointReturn,
) -> UtfCodepointReturn {
    ReturnBody::float_case(subject.into(), clauses.into_iter().collect(), fallback)
}

pub(crate) fn utf_codepoint_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: UtfCodepointReturn,
) -> UtfCodepointReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn int_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> IntReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::Int),
        args,
    )
}

pub(crate) fn int_return_tail_call_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> IntReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        crate::plan::FunctionCallTarget::new(
            tail_call_instantiation(function, &args, ValueShape::Int),
            site,
        ),
        args,
    )
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
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::Bool),
        args,
    )
}

pub(crate) fn bool_return_tail_call_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> BoolReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        crate::plan::FunctionCallTarget::new(
            tail_call_instantiation(function, &args, ValueShape::Bool),
            site,
        ),
        args,
    )
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
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::String),
        args,
    )
}

pub(crate) fn string_return_tail_call_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> StringReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        crate::plan::FunctionCallTarget::new(
            tail_call_instantiation(function, &args, ValueShape::String),
            site,
        ),
        args,
    )
}

pub(crate) fn string_return_expr(expression: String) -> StringReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn float_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> FloatReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::Float),
        args,
    )
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
    let args = args.into_iter().collect::<Vec<_>>();
    let function = tail_call_instantiation(
        function,
        &args,
        ValueShape::List(Box::new(ValueShape::from_value_type(element_type.clone()))),
    );
    ListReturn::tail_call(function, element_type, args)
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

pub(crate) fn return_list(body: ListReturn) -> FunctionReturn {
    FunctionReturn::List(body)
}

pub(crate) fn nil_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> NilReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        tail_call_instantiation(function, &args, ValueShape::Nil),
        args,
    )
}

pub(crate) fn nil_return_tail_call_at(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
    site: crate::plan::HostCallSite,
) -> NilReturn {
    let args = args.into_iter().collect::<Vec<_>>();
    ReturnBody::tail_call(
        crate::plan::FunctionCallTarget::new(
            tail_call_instantiation(function, &args, ValueShape::Nil),
            site,
        ),
        args,
    )
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
        utf_codepoint_return_block, utf_codepoint_return_bool_case, utf_codepoint_return_expr,
        utf_codepoint_return_float_case, utf_codepoint_return_int_case,
        utf_codepoint_return_string_case, utf_codepoint_return_tail_call,
    };
    use crate::plan::{
        CallArg, FunctionShape, ListReturn, ReturnBody, Step, ValueShape,
        monomorphic_function_instantiation,
    };
    use crate::planner::dsl::expression::{
        bool_, float, int, list, local_utf_codepoint, nil, string,
    };
    use num_bigint::BigInt;

    fn tail_call(template: usize, return_shape: ValueShape) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(template, FunctionShape::new(Vec::new(), return_shape))
    }

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
    fn utf_codepoint_return_helpers_build_exact_shapes() {
        let first = utf_codepoint_return_expr(local_utf_codepoint(0, "first"));
        let second = utf_codepoint_return_expr(local_utf_codepoint(1, "second"));

        assert_eq!(
            first,
            ReturnBody::expr(local_utf_codepoint(0, "first").into()),
        );
        assert_eq!(
            utf_codepoint_return_tail_call(2, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(2, ValueShape::UtfCodepoint), Vec::new()),
        );
        assert_eq!(
            utf_codepoint_return_bool_case(bool_(true), first.clone(), second.clone()),
            ReturnBody::bool_case(bool_(true).into(), first.clone(), second.clone()),
        );
        assert_eq!(
            utf_codepoint_return_int_case(int(1), [(1, first.clone())], second.clone()),
            ReturnBody::int_case(
                int(1).into(),
                vec![(BigInt::from(1), first.clone())],
                second.clone(),
            ),
        );
        assert_eq!(
            utf_codepoint_return_string_case(
                string("key"),
                [("one", first.clone())],
                second.clone(),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![("one".into(), first.clone())],
                second.clone(),
            ),
        );
        assert_eq!(
            utf_codepoint_return_float_case(float(1.0), [(1.0, first.clone())], second.clone(),),
            ReturnBody::float_case(
                float(1.0).into(),
                vec![(1.0, first.clone())],
                second.clone(),
            ),
        );
        assert_eq!(
            utf_codepoint_return_block([Step::evaluate(int(1).into())], first.clone()),
            ReturnBody::block(vec![Step::evaluate(int(1).into())], first),
        );
    }

    #[test]
    fn primitive_return_tail_call_helpers_build_tail_call_shapes() {
        assert_eq!(
            int_return_tail_call(0, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(0, ValueShape::Int), Vec::new()),
        );
        assert_eq!(
            string_return_tail_call(1, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(1, ValueShape::String), Vec::new()),
        );
        assert_eq!(
            float_return_tail_call(2, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(2, ValueShape::Float), Vec::new()),
        );
        assert_eq!(
            bool_return_tail_call(3, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(3, ValueShape::Bool), Vec::new()),
        );
        assert_eq!(
            nil_return_tail_call(4, Vec::<CallArg>::new()),
            ReturnBody::tail_call(tail_call(4, ValueShape::Nil), Vec::new()),
        );
        let list_function = tail_call(5, ValueShape::List(Box::new(ValueShape::Int)));
        assert_eq!(
            list_return_tail_call(5, Vec::<CallArg>::new(), crate::plan::ValueType::Int),
            ListReturn::tail_call(list_function, crate::plan::ValueType::Int, Vec::new()),
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
