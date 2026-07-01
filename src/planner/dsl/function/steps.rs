use super::FunctionDsl;
use crate::plan::{
    BoolLocalId, Expr, FunctionFunctionLocalId, IntLocalId, NilLocalId, Step, StringLocalId,
};
use crate::planner::dsl::expression::{Bool, FunctionFunction, Int, Nil, String};
use ecow::EcoString;

impl FunctionDsl {
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
}
