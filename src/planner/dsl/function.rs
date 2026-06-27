use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, Expr, FunctionFunctionLocalId, FunctionId, FunctionPlan,
    FunctionType, IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, Param,
    ParamLocal, ReturnExpr, Step, StringFunctionLocalId, StringLocalId, ValueType,
};
use crate::planner::dsl::expression::{Bool, FunctionFunction, Int, Nil, String};
use ecow::EcoString;

pub(crate) struct FunctionDsl {
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: ReturnExpr,
}

pub(crate) fn function(name: impl Into<EcoString>, return_: impl Into<ReturnExpr>) -> FunctionDsl {
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

    pub(crate) fn build(self, id: FunctionId) -> FunctionPlan {
        FunctionPlan::new(id, self.name, self.params, self.steps, self.return_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::StepKind;
    use crate::planner::dsl::expression::{bool_, int, nil, string};

    #[test]
    fn function_dsl() {
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
            .build(FunctionId::new(0));

        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 9);
        assert_eq!(function.steps().len(), 7);
        assert!(matches!(
            function.steps()[0].kind(),
            StepKind::LetInt { .. }
        ));
        assert!(matches!(function.steps()[6].kind(), StepKind::Evaluate(_)));
    }
}
