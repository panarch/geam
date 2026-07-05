use super::FunctionReturn;
use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionReturn, CallArg, FunctionFunctionFunctionId,
    FunctionFunctionReturn, FunctionType, IntFunctionFunctionId, IntFunctionReturn,
    NilFunctionFunctionId, NilFunctionReturn, ReturnBody, Step, StringFunctionFunctionId,
    StringFunctionReturn,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, FunctionFunction, Int, IntFunction, NilFunction, String, StringFunction,
};
use num_bigint::BigInt;

pub(crate) fn int_function_return_expr(expression: IntFunction) -> IntFunctionReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn int_function_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> IntFunctionReturn {
    ReturnBody::tail_call(IntFunctionFunctionId(function), args.into_iter().collect())
}

pub(crate) fn int_function_return_bool_case(
    subject: Bool,
    true_: IntFunctionReturn,
    false_: IntFunctionReturn,
) -> IntFunctionReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn int_function_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, IntFunctionReturn)>,
    fallback: IntFunctionReturn,
) -> IntFunctionReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn int_function_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, IntFunctionReturn)>,
    fallback: IntFunctionReturn,
) -> IntFunctionReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn int_function_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: IntFunctionReturn,
) -> IntFunctionReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_int_function(type_: FunctionType, body: IntFunctionReturn) -> FunctionReturn {
    FunctionReturn::IntFunction { type_, body }
}

pub(crate) fn string_function_return_expr(expression: StringFunction) -> StringFunctionReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn string_function_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> StringFunctionReturn {
    ReturnBody::tail_call(
        StringFunctionFunctionId(function),
        args.into_iter().collect(),
    )
}

pub(crate) fn string_function_return_bool_case(
    subject: Bool,
    true_: StringFunctionReturn,
    false_: StringFunctionReturn,
) -> StringFunctionReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn string_function_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, StringFunctionReturn)>,
    fallback: StringFunctionReturn,
) -> StringFunctionReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn string_function_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, StringFunctionReturn)>,
    fallback: StringFunctionReturn,
) -> StringFunctionReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn string_function_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: StringFunctionReturn,
) -> StringFunctionReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_string_function(
    type_: FunctionType,
    body: StringFunctionReturn,
) -> FunctionReturn {
    FunctionReturn::StringFunction { type_, body }
}

pub(crate) fn bool_function_return_expr(expression: BoolFunction) -> BoolFunctionReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn bool_function_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> BoolFunctionReturn {
    ReturnBody::tail_call(BoolFunctionFunctionId(function), args.into_iter().collect())
}

pub(crate) fn bool_function_return_bool_case(
    subject: Bool,
    true_: BoolFunctionReturn,
    false_: BoolFunctionReturn,
) -> BoolFunctionReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn bool_function_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, BoolFunctionReturn)>,
    fallback: BoolFunctionReturn,
) -> BoolFunctionReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn bool_function_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, BoolFunctionReturn)>,
    fallback: BoolFunctionReturn,
) -> BoolFunctionReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn bool_function_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: BoolFunctionReturn,
) -> BoolFunctionReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_bool_function(
    type_: FunctionType,
    body: BoolFunctionReturn,
) -> FunctionReturn {
    FunctionReturn::BoolFunction { type_, body }
}

pub(crate) fn nil_function_return_expr(expression: NilFunction) -> NilFunctionReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn nil_function_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> NilFunctionReturn {
    ReturnBody::tail_call(NilFunctionFunctionId(function), args.into_iter().collect())
}

pub(crate) fn nil_function_return_bool_case(
    subject: Bool,
    true_: NilFunctionReturn,
    false_: NilFunctionReturn,
) -> NilFunctionReturn {
    ReturnBody::bool_case(subject.into(), true_, false_)
}

pub(crate) fn nil_function_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, NilFunctionReturn)>,
    fallback: NilFunctionReturn,
) -> NilFunctionReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn nil_function_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, NilFunctionReturn)>,
    fallback: NilFunctionReturn,
) -> NilFunctionReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn nil_function_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: NilFunctionReturn,
) -> NilFunctionReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_nil_function(type_: FunctionType, body: NilFunctionReturn) -> FunctionReturn {
    FunctionReturn::NilFunction { type_, body }
}

pub(crate) fn function_function_return_expr(
    expression: FunctionFunction,
) -> FunctionFunctionReturn {
    ReturnBody::expr(expression.into())
}

pub(crate) fn function_function_return_tail_call(
    function: usize,
    args: impl IntoIterator<Item = CallArg>,
) -> FunctionFunctionReturn {
    ReturnBody::tail_call(
        FunctionFunctionFunctionId(function),
        args.into_iter().collect(),
    )
}

pub(crate) fn function_function_return_int_case(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, FunctionFunctionReturn)>,
    fallback: FunctionFunctionReturn,
) -> FunctionFunctionReturn {
    ReturnBody::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn function_function_return_string_case(
    subject: String,
    clauses: impl IntoIterator<Item = (&'static str, FunctionFunctionReturn)>,
    fallback: FunctionFunctionReturn,
) -> FunctionFunctionReturn {
    ReturnBody::string_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (value.into(), branch))
            .collect(),
        fallback,
    )
}

pub(crate) fn function_function_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: FunctionFunctionReturn,
) -> FunctionFunctionReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

pub(crate) fn return_function_function(
    type_: FunctionType,
    body: FunctionFunctionReturn,
) -> FunctionReturn {
    FunctionReturn::FunctionFunction { type_, body }
}

#[cfg(test)]
mod tests {
    use super::{
        bool_function_return_block, bool_function_return_bool_case, bool_function_return_expr,
        bool_function_return_int_case, bool_function_return_string_case,
        bool_function_return_tail_call, function_function_return_block,
        function_function_return_expr, function_function_return_int_case,
        function_function_return_string_case, function_function_return_tail_call,
        int_function_return_block, int_function_return_bool_case, int_function_return_expr,
        int_function_return_int_case, int_function_return_string_case,
        int_function_return_tail_call, nil_function_return_block, nil_function_return_bool_case,
        nil_function_return_expr, nil_function_return_int_case, nil_function_return_string_case,
        nil_function_return_tail_call, return_bool_function, return_function_function,
        return_int_function, return_nil_function, return_string_function,
        string_function_return_block, string_function_return_bool_case,
        string_function_return_expr, string_function_return_int_case,
        string_function_return_string_case, string_function_return_tail_call,
    };
    use crate::plan::{
        BoolFunctionFunctionId, CallArg, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionType, IntFunctionFunctionId, NilFunctionFunctionId, ParamLocal, ReturnBody, Step,
        StringFunctionFunctionId, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, function_function_ref, int, int_function_ref, nil_function_ref,
        string, string_function_ref,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_return_expr_helpers_build_return_body_shapes() {
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
            ReturnBody::expr(int_function_ref(0, Vec::<ParamLocal>::new()).into()),
        );
        assert_eq!(
            string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
            ReturnBody::expr(string_function_ref(0, Vec::<ParamLocal>::new()).into()),
        );
        assert_eq!(
            bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
            ReturnBody::expr(bool_function_ref(0, Vec::<ParamLocal>::new()).into()),
        );
        assert_eq!(
            nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
            ReturnBody::expr(nil_function_ref(0, Vec::<ParamLocal>::new()).into()),
        );
        assert_eq!(
            function_function_return_expr(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                returned_function_type,
            )),
            ReturnBody::expr(
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
                .into(),
            ),
        );
    }

    #[test]
    fn function_return_tail_call_helpers_build_return_body_shapes() {
        assert_eq!(
            int_function_return_tail_call(0, Vec::<CallArg>::new()),
            ReturnBody::tail_call(IntFunctionFunctionId(0), Vec::new()),
        );
        assert_eq!(
            string_function_return_tail_call(1, Vec::<CallArg>::new()),
            ReturnBody::tail_call(StringFunctionFunctionId(1), Vec::new()),
        );
        assert_eq!(
            bool_function_return_tail_call(2, Vec::<CallArg>::new()),
            ReturnBody::tail_call(BoolFunctionFunctionId(2), Vec::new()),
        );
        assert_eq!(
            nil_function_return_tail_call(3, Vec::<CallArg>::new()),
            ReturnBody::tail_call(NilFunctionFunctionId(3), Vec::new()),
        );
        assert_eq!(
            function_function_return_tail_call(4, Vec::<CallArg>::new()),
            ReturnBody::tail_call(FunctionFunctionFunctionId(4), Vec::new()),
        );
    }

    #[test]
    fn function_return_case_helpers_build_return_body_shapes() {
        assert_eq!(
            int_function_return_bool_case(
                bool_(true),
                int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            string_function_return_bool_case(
                bool_(true),
                string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            bool_function_return_bool_case(
                bool_(true),
                bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            nil_function_return_bool_case(
                bool_(true),
                nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::bool_case(
                bool_(true).into(),
                nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );

        assert_eq!(
            int_function_return_int_case(
                int(1),
                [(
                    1,
                    int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            string_function_return_int_case(
                int(1),
                [(
                    1,
                    string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            bool_function_return_int_case(
                int(1),
                [(
                    1,
                    bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            nil_function_return_int_case(
                int(1),
                [(
                    1,
                    nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            function_function_return_int_case(
                int(1),
                [(
                    1,
                    function_function_return_expr(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )),
                )],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )),
            ),
            ReturnBody::int_case(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    function_function_return_expr(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )),
                )],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )),
            ),
        );

        assert_eq!(
            int_function_return_string_case(
                string("key"),
                [(
                    "one",
                    int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            string_function_return_string_case(
                string("key"),
                [(
                    "one",
                    string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            bool_function_return_string_case(
                string("key"),
                [(
                    "one",
                    bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            nil_function_return_string_case(
                string("key"),
                [(
                    "one",
                    nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            ),
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            function_function_return_string_case(
                string("key"),
                [(
                    "one",
                    function_function_return_expr(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        returned_function_type.clone(),
                    )),
                )],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(returned_function_type.clone())),
                    ),
                )),
            ),
            ReturnBody::string_case(
                string("key").into(),
                vec![(
                    "one".into(),
                    function_function_return_expr(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        returned_function_type.clone(),
                    )),
                )],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(returned_function_type)),
                    ),
                )),
            ),
        );
    }

    #[test]
    fn function_return_block_helpers_build_return_body_shapes() {
        let step = Step::evaluate(int(0).into());
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            int_function_return_block(
                [step.clone()],
                int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::block(
                vec![step.clone()],
                int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            string_function_return_block(
                [step.clone()],
                string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::block(
                vec![step.clone()],
                string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            bool_function_return_block(
                [step.clone()],
                bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::block(
                vec![step.clone()],
                bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            nil_function_return_block(
                [step.clone()],
                nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            ReturnBody::block(
                vec![step.clone()],
                nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
            ),
        );
        assert_eq!(
            function_function_return_block(
                [step],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
            ),
            ReturnBody::block(
                vec![Step::evaluate(int(0).into())],
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )),
            ),
        );
    }

    #[test]
    fn function_return_wrapper_helpers_build_return_families() {
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            return_int_function(
                returned_function_type.clone(),
                int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            super::super::FunctionReturn::IntFunction {
                type_: returned_function_type.clone(),
                body: int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
            },
        );
        assert_eq!(
            return_string_function(
                returned_function_type.clone(),
                string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            super::super::FunctionReturn::StringFunction {
                type_: returned_function_type.clone(),
                body: string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
            },
        );
        assert_eq!(
            return_bool_function(
                returned_function_type.clone(),
                bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            super::super::FunctionReturn::BoolFunction {
                type_: returned_function_type.clone(),
                body: bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
            },
        );
        assert_eq!(
            return_nil_function(
                returned_function_type.clone(),
                nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
            ),
            super::super::FunctionReturn::NilFunction {
                type_: returned_function_type.clone(),
                body: nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
            },
        );
        assert_eq!(
            return_function_function(
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(returned_function_type.clone())),
                ),
                function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
            ),
            super::super::FunctionReturn::FunctionFunction {
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(returned_function_type.clone())),
                ),
                body: function_function_return_expr(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type,
                )),
            },
        );
    }
}
