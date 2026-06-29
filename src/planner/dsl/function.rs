use crate::plan::{
    BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolLocalId, Expr, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, FunctionFunctionLocalId, FunctionId, FunctionPlan,
    FunctionType, IntExpr, IntFunctionExpr, IntFunctionLocalId, IntLocalId, NilExpr,
    NilFunctionExpr, NilFunctionLocalId, NilLocalId, Param, ParamLocal, ReturnExpr, Step,
    StringExpr, StringFunctionExpr, StringFunctionLocalId, StringLocalId, ValueType,
};
use crate::planner::context::FunctionRuntimeIds;
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Function, FunctionFunction, Int, IntFunction, Nil, NilFunction, String,
    StringFunction,
};
use ecow::EcoString;

pub(crate) struct FunctionDsl {
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: FunctionReturn,
}

pub(crate) enum FunctionReturn {
    Int(IntExpr),
    String(StringExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    IntFunction(IntFunctionExpr),
    StringFunction(StringFunctionExpr),
    BoolFunction(BoolFunctionExpr),
    NilFunction(NilFunctionExpr),
    FunctionFunction(FunctionFunctionExpr),
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
        self.params
            .push(Param::new(ParamLocal::int(IntLocalId(local)), name.into()));
        self
    }

    pub(crate) fn param_string(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::new(
            ParamLocal::string(StringLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::new(
            ParamLocal::bool(BoolLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_nil(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params
            .push(Param::new(ParamLocal::nil(NilLocalId(local)), name.into()));
        self
    }

    pub(crate) fn param_int_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::new(
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
        self.params.push(Param::new(
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
        self.params.push(Param::new(
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
        self.params.push(Param::new(
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
        self.params.push(Param::new(
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
            Self::Int(expression) => ReturnExpr::int(runtime_ids.next_int_id(), expression),
            Self::String(expression) => {
                ReturnExpr::string(runtime_ids.next_string_id(), expression)
            }
            Self::Bool(expression) => ReturnExpr::bool(runtime_ids.next_bool_id(), expression),
            Self::Nil(expression) => ReturnExpr::nil(runtime_ids.next_nil_id(), expression),
            Self::IntFunction(expression) => {
                ReturnExpr::int_function(runtime_ids.next_int_function_id(), expression)
            }
            Self::StringFunction(expression) => {
                ReturnExpr::string_function(runtime_ids.next_string_function_id(), expression)
            }
            Self::BoolFunction(expression) => {
                ReturnExpr::bool_function(runtime_ids.next_bool_function_id(), expression)
            }
            Self::NilFunction(expression) => {
                ReturnExpr::nil_function(runtime_ids.next_nil_function_id(), expression)
            }
            Self::FunctionFunction(expression) => {
                ReturnExpr::function_function(runtime_ids.next_function_function_id(), expression)
            }
        }
    }
}

impl From<Int> for FunctionReturn {
    fn from(value: Int) -> Self {
        Self::Int(value.into())
    }
}

impl From<String> for FunctionReturn {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<Bool> for FunctionReturn {
    fn from(value: Bool) -> Self {
        Self::Bool(value.into())
    }
}

impl From<Nil> for FunctionReturn {
    fn from(value: Nil) -> Self {
        Self::Nil(value.into())
    }
}

impl From<IntFunction> for FunctionReturn {
    fn from(value: IntFunction) -> Self {
        Self::IntFunction(value.into())
    }
}

impl From<StringFunction> for FunctionReturn {
    fn from(value: StringFunction) -> Self {
        Self::StringFunction(value.into())
    }
}

impl From<BoolFunction> for FunctionReturn {
    fn from(value: BoolFunction) -> Self {
        Self::BoolFunction(value.into())
    }
}

impl From<NilFunction> for FunctionReturn {
    fn from(value: NilFunction) -> Self {
        Self::NilFunction(value.into())
    }
}

impl From<FunctionFunction> for FunctionReturn {
    fn from(value: FunctionFunction) -> Self {
        Self::FunctionFunction(value.into())
    }
}

impl From<Function> for FunctionReturn {
    fn from(value: Function) -> Self {
        match FunctionExpr::from(value).into_kind() {
            FunctionExprKind::Int(expression) => Self::IntFunction(expression),
            FunctionExprKind::String(expression) => Self::StringFunction(expression),
            FunctionExprKind::Bool(expression) => Self::BoolFunction(expression),
            FunctionExprKind::Nil(expression) => Self::NilFunction(expression),
            FunctionExprKind::Function(expression) => Self::FunctionFunction(expression),
        }
    }
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
