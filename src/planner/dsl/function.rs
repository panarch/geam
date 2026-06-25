use crate::plan::{
    BoolLocalId, Expr, FunctionId, FunctionPlan, IntLocalId, LocalId, NilLocalId, Param,
    ReturnExpr, Step, StringLocalId,
};
use crate::planner::dsl::expression::{Bool, Function, Int, Nil, String};
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
            .push(Param::new(LocalId::Int(IntLocalId(local)), name.into()));
        self
    }

    pub(crate) fn param_string(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::new(
            LocalId::String(StringLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params
            .push(Param::new(LocalId::Bool(BoolLocalId(local)), name.into()));
        self
    }

    pub(crate) fn param_nil(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params
            .push(Param::new(LocalId::Nil(NilLocalId(local)), name.into()));
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

    pub(crate) fn let_function(mut self, name: impl Into<EcoString>, value: Function) -> Self {
        self.steps
            .push(Step::let_function(name.into(), value.into()));
        self
    }

    pub(crate) fn evaluate(mut self, value: impl Into<Expr>) -> Self {
        self.steps.push(Step::evaluate(value.into()));
        self
    }

    pub(crate) fn build(self, id: FunctionId) -> FunctionPlan {
        FunctionPlan::new(id, self.name, self.params, self.steps, self.return_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{FunctionType, IntFunctionId, RuntimeFunctionId, StepKind, ValueType};
    use crate::planner::dsl::expression::{bool_, function_ref, int, nil, string};

    #[test]
    fn function_dsl() {
        let function = function("main", int(1))
            .param_int(0, "a")
            .param_string(0, "b")
            .param_bool(0, "c")
            .param_nil(0, "d")
            .let_int(1, "x", int(2))
            .let_string(1, "y", string("a"))
            .let_bool(1, "z", bool_(true))
            .let_nil(1, "n", nil())
            .let_function(
                "f",
                function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    FunctionType::new(Vec::new(), ValueType::Int),
                    [],
                ),
            )
            .evaluate(int(3))
            .build(FunctionId::new(0));

        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 4);
        assert_eq!(function.steps().len(), 6);
        assert!(matches!(
            function.steps()[0].kind(),
            StepKind::LetInt { .. }
        ));
        assert!(matches!(
            function.steps()[4].kind(),
            StepKind::LetFunction { .. },
        ));
        assert!(matches!(function.steps()[5].kind(), StepKind::Evaluate(_)));
    }
}
