use super::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn, BoolReturn,
    FunctionFunctionFunctionId, FunctionFunctionReturn, FunctionPlan, IntFunctionFunctionId,
    IntFunctionId, IntFunctionReturn, IntReturn, NilFunctionFunctionId, NilFunctionId,
    NilFunctionReturn, NilReturn, RuntimeFunction, RuntimeFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringFunctionReturn, StringReturn,
};
use crate::plan::ReturnExprKind;

pub(super) struct RuntimePlan {
    main: RuntimeFunctionId,
    int_functions: Vec<RuntimeFunction<IntReturn>>,
    string_functions: Vec<RuntimeFunction<StringReturn>>,
    bool_functions: Vec<RuntimeFunction<BoolReturn>>,
    nil_functions: Vec<RuntimeFunction<NilReturn>>,
    int_function_functions: Vec<RuntimeFunction<IntFunctionReturn>>,
    string_function_functions: Vec<RuntimeFunction<StringFunctionReturn>>,
    bool_function_functions: Vec<RuntimeFunction<BoolFunctionReturn>>,
    nil_function_functions: Vec<RuntimeFunction<NilFunctionReturn>>,
    function_function_functions: Vec<RuntimeFunction<FunctionFunctionReturn>>,
}

impl RuntimePlan {
    pub(super) fn new(
        main: &FunctionPlan,
        functions: &[FunctionPlan],
        anonymous_functions: &[FunctionPlan],
    ) -> Self {
        let mut runtime = RuntimePlanBuilder::default();
        let main_runtime = main.return_().runtime_id();
        runtime.push(main);

        for function in functions {
            runtime.push(function);
        }
        for function in anonymous_functions {
            runtime.push(function);
        }

        runtime.finish(main_runtime)
    }

    pub(super) fn main(&self) -> RuntimeFunctionId {
        self.main.clone()
    }

    pub(super) fn int_function(&self, id: IntFunctionId) -> &RuntimeFunction<IntReturn> {
        &self.int_functions[id.0]
    }

    pub(super) fn string_function(&self, id: StringFunctionId) -> &RuntimeFunction<StringReturn> {
        &self.string_functions[id.0]
    }

    pub(super) fn bool_function(&self, id: BoolFunctionId) -> &RuntimeFunction<BoolReturn> {
        &self.bool_functions[id.0]
    }

    pub(super) fn nil_function(&self, id: NilFunctionId) -> &RuntimeFunction<NilReturn> {
        &self.nil_functions[id.0]
    }

    pub(super) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &RuntimeFunction<IntFunctionReturn> {
        &self.int_function_functions[id.0]
    }

    pub(super) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &RuntimeFunction<StringFunctionReturn> {
        &self.string_function_functions[id.0]
    }

    pub(super) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &RuntimeFunction<BoolFunctionReturn> {
        &self.bool_function_functions[id.0]
    }

    pub(super) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &RuntimeFunction<NilFunctionReturn> {
        &self.nil_function_functions[id.0]
    }

    pub(super) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &RuntimeFunction<FunctionFunctionReturn> {
        &self.function_function_functions[id.0]
    }
}

#[derive(Default)]
struct RuntimePlanBuilder {
    int_functions: Vec<(usize, RuntimeFunction<IntReturn>)>,
    string_functions: Vec<(usize, RuntimeFunction<StringReturn>)>,
    bool_functions: Vec<(usize, RuntimeFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, RuntimeFunction<NilReturn>)>,
    int_function_functions: Vec<(usize, RuntimeFunction<IntFunctionReturn>)>,
    string_function_functions: Vec<(usize, RuntimeFunction<StringFunctionReturn>)>,
    bool_function_functions: Vec<(usize, RuntimeFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, RuntimeFunction<NilFunctionReturn>)>,
    function_function_functions: Vec<(usize, RuntimeFunction<FunctionFunctionReturn>)>,
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
        ReturnExprKind::Int { runtime_id, body } => {
            runtime_functions.int_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::String { runtime_id, body } => {
            runtime_functions.string_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Bool { runtime_id, body } => {
            runtime_functions.bool_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Nil { runtime_id, body } => {
            runtime_functions.nil_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::IntFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.int_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::StringFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.string_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::BoolFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.bool_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::NilFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.nil_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::FunctionFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.function_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
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
