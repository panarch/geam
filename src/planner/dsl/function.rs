use crate::plan::{
    BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
    BoolFunctionReturn, BoolLocalId, BoolReturn, CallArg, Expr, FunctionExpr, FunctionExprKind,
    FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionLocalId,
    FunctionFunctionReturn, FunctionId, FunctionPlan, FunctionType, IntFunctionExpr,
    IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionReturn, IntLocalId,
    IntReturn, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    NilFunctionReturn, NilLocalId, NilReturn, Param, ParamLocal, ReturnBody, ReturnExpr, Step,
    StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
    StringFunctionReturn, StringLocalId, StringReturn, ValueType,
};
use crate::planner::context::FunctionRuntimeIds;
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Function, FunctionFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct FunctionDsl {
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: FunctionReturn,
}

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

pub(crate) fn function(
    name: impl Into<EcoString>,
    return_: impl Into<FunctionReturn>,
) -> FunctionDsl {
    FunctionDsl {
        name: name.into(),
        params: Vec::new(),
        steps: Vec::new(),
        return_: return_.into(),
    }
}

impl FunctionDsl {
    pub(crate) fn param_int(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::int(IntLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn discard_int_param(mut self, local: usize) -> Self {
        self.params
            .push(Param::discard(ParamLocal::int(IntLocalId(local))));
        self
    }

    pub(crate) fn param_string(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::string(StringLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::bool(BoolLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_nil(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::nil(NilLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_int_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::int_function(
                IntFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Int),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_string_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::string_function(
                StringFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::String),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::bool_function(
                BoolFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Bool),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_nil_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::nil_function(
                NilFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Nil),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_function_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        type_: FunctionType,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::function_function(FunctionFunctionLocalId(local), type_),
            name.into(),
        ));
        self
    }

    pub(crate) fn let_int(mut self, local: usize, name: impl Into<EcoString>, value: Int) -> Self {
        self.steps
            .push(Step::let_int(IntLocalId(local), name.into(), value.into()));
        self
    }

    pub(crate) fn let_string(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        value: String,
    ) -> Self {
        self.steps.push(Step::let_string(
            StringLocalId(local),
            name.into(),
            value.into(),
        ));
        self
    }

    pub(crate) fn let_bool(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        value: Bool,
    ) -> Self {
        self.steps.push(Step::let_bool(
            BoolLocalId(local),
            name.into(),
            value.into(),
        ));
        self
    }

    pub(crate) fn let_nil(mut self, local: usize, name: impl Into<EcoString>, value: Nil) -> Self {
        self.steps
            .push(Step::let_nil(NilLocalId(local), name.into(), value.into()));
        self
    }

    pub(crate) fn let_function_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        value: FunctionFunction,
    ) -> Self {
        self.steps.push(Step::let_function_function(
            FunctionFunctionLocalId(local),
            name.into(),
            value.into(),
        ));
        self
    }

    pub(crate) fn evaluate(mut self, value: impl Into<Expr>) -> Self {
        self.steps.push(Step::evaluate(value.into()));
        self
    }

    pub(crate) fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub(crate) fn build(
        self,
        id: FunctionId,
        runtime_ids: &mut FunctionRuntimeIds,
    ) -> FunctionPlan {
        let return_ = self.return_.build(runtime_ids);

        FunctionPlan::new(id, self.name, self.params, self.steps, return_)
    }
}

impl FunctionReturn {
    fn build(self, runtime_ids: &mut FunctionRuntimeIds) -> ReturnExpr {
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
    use super::*;
    use crate::plan::{
        BoolFunctionId, FunctionFunctionId, IntFunctionFunctionId, IntFunctionId, NilFunctionId,
        RuntimeFunctionId, StepKind, StringFunctionId,
    };
    use crate::planner::context::FunctionRuntimeIds;
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, function_function_ref, function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref,
    };

    #[test]
    fn function_dsl() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let function = function("main", int(1))
            .param_int(0, "a")
            .param_string(0, "b")
            .param_bool(0, "c")
            .param_nil(0, "d")
            .param_int_function(0, "f", [ValueType::Int])
            .param_string_function(0, "g", [ValueType::String])
            .param_bool_function(0, "h", [ValueType::Bool])
            .param_nil_function(0, "i", [ValueType::Nil])
            .param_function_function(
                0,
                "j",
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            )
            .let_int(1, "x", int(2))
            .let_string(1, "y", string("a"))
            .let_bool(1, "z", bool_(true))
            .let_nil(1, "n", nil())
            .let_function_function(
                1,
                "ff",
                crate::planner::dsl::expression::local_function_function(
                    0,
                    "j",
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::Int,
                        ))),
                    ),
                ),
            )
            .step(Step::evaluate(int(4).into()))
            .evaluate(int(3))
            .build(FunctionId::new(0), &mut runtime_ids);

        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 9);
        assert_eq!(function.steps().len(), 7);
        assert!(matches!(
            function.steps()[0].kind(),
            StepKind::LetInt { .. }
        ));
        assert!(matches!(function.steps()[6].kind(), StepKind::Evaluate(_)));
    }

    #[test]
    fn function_dsl_return_function_families() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let int_return = function("int", int_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(0), &mut runtime_ids);
        let string_return = function("string", string_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(1), &mut runtime_ids);
        let bool_return = function("bool", bool_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(2), &mut runtime_ids);
        let nil_return = function("nil", nil_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(3), &mut runtime_ids);
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_return = function(
            "function",
            function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                return_type.clone(),
            ),
        )
        .build(FunctionId::new(4), &mut runtime_ids);

        assert_eq!(
            int_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
        );
        assert_eq!(
            string_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(crate::plan::StringFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::String),
            },
        );
        assert_eq!(
            bool_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(crate::plan::BoolFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Bool),
            },
        );
        assert_eq!(
            nil_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(crate::plan::NilFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Nil),
            },
        );
        assert_eq!(
            function_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(crate::plan::FunctionFunctionFunctionId(0)),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(return_type)),
                ),
            },
        );
    }

    #[test]
    fn function_dsl_generic_function_return_families() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let int_return = function(
            "int",
            function_ref(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(0), &mut runtime_ids);
        let string_return = function(
            "string",
            function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(1), &mut runtime_ids);
        let bool_return = function(
            "bool",
            function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(2), &mut runtime_ids);
        let nil_return = function(
            "nil",
            function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(3), &mut runtime_ids);
        let function_return = function(
            "function",
            function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(4), &mut runtime_ids);

        assert_eq!(
            int_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
        );
        assert_eq!(
            string_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(crate::plan::StringFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::String),
            },
        );
        assert_eq!(
            bool_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(crate::plan::BoolFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Bool),
            },
        );
        assert_eq!(
            nil_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(crate::plan::NilFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Nil),
            },
        );
        assert_eq!(
            function_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(crate::plan::FunctionFunctionFunctionId(0)),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                ),
            },
        );
    }
}
