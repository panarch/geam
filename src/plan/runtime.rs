use super::{
    BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, FunctionFunctionExpr,
    FunctionFunctionFunctionId, FunctionPlan, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
    IntFunctionId, NilExpr, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, RuntimeFunction,
    RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionFunctionId, StringFunctionId,
};
use crate::plan::ReturnExprKind;

pub(super) struct RuntimePlan {
    main: RuntimeFunctionId,
    int_functions: Vec<RuntimeFunction<IntExpr>>,
    string_functions: Vec<RuntimeFunction<StringExpr>>,
    bool_functions: Vec<RuntimeFunction<BoolExpr>>,
    nil_functions: Vec<RuntimeFunction<NilExpr>>,
    int_function_functions: Vec<RuntimeFunction<IntFunctionExpr>>,
    string_function_functions: Vec<RuntimeFunction<StringFunctionExpr>>,
    bool_function_functions: Vec<RuntimeFunction<BoolFunctionExpr>>,
    nil_function_functions: Vec<RuntimeFunction<NilFunctionExpr>>,
    function_function_functions: Vec<RuntimeFunction<FunctionFunctionExpr>>,
}

impl RuntimePlan {
    pub(super) fn new(main: &FunctionPlan, functions: &[FunctionPlan]) -> Self {
        let mut runtime = RuntimePlanBuilder::default();
        let main_runtime = main.return_().runtime_id();
        runtime.push(main);

        for function in functions {
            runtime.push(function);
        }

        runtime.finish(main_runtime)
    }

    pub(super) fn main(&self) -> RuntimeFunctionId {
        self.main.clone()
    }

    pub(super) fn int_function(&self, id: IntFunctionId) -> &RuntimeFunction<IntExpr> {
        &self.int_functions[id.0]
    }

    pub(super) fn string_function(&self, id: StringFunctionId) -> &RuntimeFunction<StringExpr> {
        &self.string_functions[id.0]
    }

    pub(super) fn bool_function(&self, id: BoolFunctionId) -> &RuntimeFunction<BoolExpr> {
        &self.bool_functions[id.0]
    }

    pub(super) fn nil_function(&self, id: NilFunctionId) -> &RuntimeFunction<NilExpr> {
        &self.nil_functions[id.0]
    }

    pub(super) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &RuntimeFunction<IntFunctionExpr> {
        &self.int_function_functions[id.0]
    }

    pub(super) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &RuntimeFunction<StringFunctionExpr> {
        &self.string_function_functions[id.0]
    }

    pub(super) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &RuntimeFunction<BoolFunctionExpr> {
        &self.bool_function_functions[id.0]
    }

    pub(super) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &RuntimeFunction<NilFunctionExpr> {
        &self.nil_function_functions[id.0]
    }

    pub(super) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &RuntimeFunction<FunctionFunctionExpr> {
        &self.function_function_functions[id.0]
    }
}

#[derive(Default)]
struct RuntimePlanBuilder {
    int_functions: Vec<(usize, RuntimeFunction<IntExpr>)>,
    string_functions: Vec<(usize, RuntimeFunction<StringExpr>)>,
    bool_functions: Vec<(usize, RuntimeFunction<BoolExpr>)>,
    nil_functions: Vec<(usize, RuntimeFunction<NilExpr>)>,
    int_function_functions: Vec<(usize, RuntimeFunction<IntFunctionExpr>)>,
    string_function_functions: Vec<(usize, RuntimeFunction<StringFunctionExpr>)>,
    bool_function_functions: Vec<(usize, RuntimeFunction<BoolFunctionExpr>)>,
    nil_function_functions: Vec<(usize, RuntimeFunction<NilFunctionExpr>)>,
    function_function_functions: Vec<(usize, RuntimeFunction<FunctionFunctionExpr>)>,
}

impl RuntimePlanBuilder {
    fn push(&mut self, function: &FunctionPlan) {
        runtime_function(function, self);
    }

    fn finish(self, main: RuntimeFunctionId) -> RuntimePlan {
        RuntimePlan {
            main,
            int_functions: sort_functions(self.int_functions),
            string_functions: sort_functions(self.string_functions),
            bool_functions: sort_functions(self.bool_functions),
            nil_functions: sort_functions(self.nil_functions),
            int_function_functions: sort_functions(self.int_function_functions),
            string_function_functions: sort_functions(self.string_function_functions),
            bool_function_functions: sort_functions(self.bool_function_functions),
            nil_function_functions: sort_functions(self.nil_function_functions),
            function_function_functions: sort_functions(self.function_function_functions),
        }
    }
}

fn runtime_function(function: &FunctionPlan, runtime_functions: &mut RuntimePlanBuilder) {
    match function.return_().kind() {
        ReturnExprKind::Int {
            runtime_id,
            expression,
        } => {
            runtime_functions.int_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::String {
            runtime_id,
            expression,
        } => {
            runtime_functions.string_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::Bool {
            runtime_id,
            expression,
        } => {
            runtime_functions.bool_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::Nil {
            runtime_id,
            expression,
        } => {
            runtime_functions.nil_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::IntFunction {
            runtime_id,
            expression,
        } => {
            runtime_functions.int_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::StringFunction {
            runtime_id,
            expression,
        } => {
            runtime_functions.string_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::BoolFunction {
            runtime_id,
            expression,
        } => {
            runtime_functions.bool_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::NilFunction {
            runtime_id,
            expression,
        } => {
            runtime_functions.nil_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
        ReturnExprKind::FunctionFunction {
            runtime_id,
            expression,
        } => {
            runtime_functions.function_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    expression.clone(),
                ),
            ));
        }
    }
}

fn sort_functions<Return>(
    mut functions: Vec<(usize, RuntimeFunction<Return>)>,
) -> Vec<RuntimeFunction<Return>> {
    functions.sort_by_key(|(index, _)| *index);
    functions
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}
