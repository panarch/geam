use crate::plan::{
    BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn, BoolReturn,
    CallArg, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, FunctionFunctionFunctionId,
    FunctionFunctionReturn, FunctionType, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
    IntFunctionReturn, IntReturn, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId,
    NilFunctionReturn, NilReturn, ReturnBody, ReturnExpr, Step, StringFunctionExpr,
    StringFunctionFunctionId, StringFunctionId, StringFunctionReturn, StringReturn,
};
use crate::planner::context::FunctionRuntimeIds;
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Function, FunctionFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction,
};
use num_bigint::BigInt;

pub(crate) enum FunctionReturn {
    Int(IntReturn),
    String(StringReturn),
    Bool(BoolReturn),
    Nil(NilReturn),
    IntFunction {
        type_: FunctionType,
        body: IntFunctionReturn,
    },
    StringFunction {
        type_: FunctionType,
        body: StringFunctionReturn,
    },
    BoolFunction {
        type_: FunctionType,
        body: BoolFunctionReturn,
    },
    NilFunction {
        type_: FunctionType,
        body: NilFunctionReturn,
    },
    FunctionFunction {
        type_: FunctionType,
        body: FunctionFunctionReturn,
    },
}

impl FunctionReturn {
    pub(super) fn build(self, runtime_ids: &mut FunctionRuntimeIds) -> ReturnExpr {
        match self {
            Self::Int(body) => ReturnExpr::int_body(runtime_ids.next_int_id(), body),
            Self::String(body) => ReturnExpr::string_body(runtime_ids.next_string_id(), body),
            Self::Bool(body) => ReturnExpr::bool_body(runtime_ids.next_bool_id(), body),
            Self::Nil(body) => ReturnExpr::nil_body(runtime_ids.next_nil_id(), body),
            Self::IntFunction { type_, body } => {
                ReturnExpr::int_function_body(runtime_ids.next_int_function_id(), type_, body)
            }
            Self::StringFunction { type_, body } => {
                ReturnExpr::string_function_body(runtime_ids.next_string_function_id(), type_, body)
            }
            Self::BoolFunction { type_, body } => {
                ReturnExpr::bool_function_body(runtime_ids.next_bool_function_id(), type_, body)
            }
            Self::NilFunction { type_, body } => {
                ReturnExpr::nil_function_body(runtime_ids.next_nil_function_id(), type_, body)
            }
            Self::FunctionFunction { type_, body } => ReturnExpr::function_function_body(
                runtime_ids.next_function_function_id(),
                type_,
                body,
            ),
        }
    }
}

impl From<Int> for FunctionReturn {
    fn from(value: Int) -> Self {
        Self::Int(ReturnBody::expr(value.into()))
    }
}

impl From<String> for FunctionReturn {
    fn from(value: String) -> Self {
        Self::String(ReturnBody::expr(value.into()))
    }
}

impl From<Bool> for FunctionReturn {
    fn from(value: Bool) -> Self {
        Self::Bool(ReturnBody::expr(value.into()))
    }
}

impl From<Nil> for FunctionReturn {
    fn from(value: Nil) -> Self {
        Self::Nil(ReturnBody::expr(value.into()))
    }
}

impl From<IntFunction> for FunctionReturn {
    fn from(value: IntFunction) -> Self {
        let expression = IntFunctionExpr::from(value);
        Self::IntFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<StringFunction> for FunctionReturn {
    fn from(value: StringFunction) -> Self {
        let expression = StringFunctionExpr::from(value);
        Self::StringFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<BoolFunction> for FunctionReturn {
    fn from(value: BoolFunction) -> Self {
        let expression = BoolFunctionExpr::from(value);
        Self::BoolFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<NilFunction> for FunctionReturn {
    fn from(value: NilFunction) -> Self {
        let expression = NilFunctionExpr::from(value);
        Self::NilFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<FunctionFunction> for FunctionReturn {
    fn from(value: FunctionFunction) -> Self {
        let expression = FunctionFunctionExpr::from(value);
        Self::FunctionFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<Function> for FunctionReturn {
    fn from(value: Function) -> Self {
        match FunctionExpr::from(value).into_kind() {
            FunctionExprKind::Int(expression) => Self::IntFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::String(expression) => Self::StringFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Bool(expression) => Self::BoolFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Nil(expression) => Self::NilFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Function(expression) => Self::FunctionFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
        }
    }
}

impl From<IntReturn> for FunctionReturn {
    fn from(value: IntReturn) -> Self {
        Self::Int(value)
    }
}

impl From<StringReturn> for FunctionReturn {
    fn from(value: StringReturn) -> Self {
        Self::String(value)
    }
}

impl From<BoolReturn> for FunctionReturn {
    fn from(value: BoolReturn) -> Self {
        Self::Bool(value)
    }
}

impl From<NilReturn> for FunctionReturn {
    fn from(value: NilReturn) -> Self {
        Self::Nil(value)
    }
}

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

pub(crate) fn nil_return_block(
    steps: impl IntoIterator<Item = Step>,
    return_: NilReturn,
) -> NilReturn {
    ReturnBody::block(steps.into_iter().collect(), return_)
}

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
        bool_function_return_expr, bool_function_return_string_case, function_function_return_expr,
        function_function_return_string_case, int_function_return_expr,
        int_function_return_string_case, nil_function_return_expr, nil_function_return_string_case,
        string_function_return_expr, string_function_return_string_case,
    };
    use crate::plan::{
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionType, IntFunctionFunctionId,
        ParamLocal, ReturnBodyKind, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_function_ref, function_function_ref, int_function_ref, nil_function_ref, string,
        string_function_ref,
    };

    #[test]
    fn function_return_string_case_helpers_build_return_body_shapes() {
        assert!(matches!(
            int_function_return_string_case(
                string("key"),
                [(
                    "one",
                    int_function_return_expr(int_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                int_function_return_expr(int_function_ref(1, Vec::<ParamLocal>::new())),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            string_function_return_string_case(
                string("key"),
                [(
                    "one",
                    string_function_return_expr(string_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                string_function_return_expr(string_function_ref(1, Vec::<ParamLocal>::new())),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            bool_function_return_string_case(
                string("key"),
                [(
                    "one",
                    bool_function_return_expr(bool_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                bool_function_return_expr(bool_function_ref(1, Vec::<ParamLocal>::new())),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
        assert!(matches!(
            nil_function_return_string_case(
                string("key"),
                [(
                    "one",
                    nil_function_return_expr(nil_function_ref(0, Vec::<ParamLocal>::new())),
                )],
                nil_function_return_expr(nil_function_ref(1, Vec::<ParamLocal>::new())),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert!(matches!(
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
                        ValueType::Function(Box::new(returned_function_type)),
                    ),
                )),
            )
            .kind(),
            ReturnBodyKind::StringCase { .. },
        ));
    }
}
