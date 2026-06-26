use super::{
    BoolExpr, BoolFunctionId, FunctionPlan, IntExpr, IntFunctionId, NilExpr, NilFunctionId,
    RuntimeFunction, RuntimeFunctionId, StringExpr, StringFunctionId,
};
use crate::plan::ReturnExprKind;

pub(super) struct RuntimePlan {
    main: RuntimeFunctionId,
    int_functions: Vec<RuntimeFunction<IntExpr>>,
    string_functions: Vec<RuntimeFunction<StringExpr>>,
    bool_functions: Vec<RuntimeFunction<BoolExpr>>,
    nil_functions: Vec<RuntimeFunction<NilExpr>>,
}

impl RuntimePlan {
    pub(super) fn new(main: &FunctionPlan, functions: &[FunctionPlan]) -> Self {
        let mut runtime = RuntimePlanBuilder::default();
        let main = runtime.push(main);

        for function in functions {
            runtime.push(function);
        }

        Self {
            main,
            int_functions: runtime.int_functions,
            string_functions: runtime.string_functions,
            bool_functions: runtime.bool_functions,
            nil_functions: runtime.nil_functions,
        }
    }

    pub(super) fn main(&self) -> RuntimeFunctionId {
        self.main
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
}

#[derive(Default)]
struct RuntimePlanBuilder {
    int_functions: Vec<RuntimeFunction<IntExpr>>,
    string_functions: Vec<RuntimeFunction<StringExpr>>,
    bool_functions: Vec<RuntimeFunction<BoolExpr>>,
    nil_functions: Vec<RuntimeFunction<NilExpr>>,
}

impl RuntimePlanBuilder {
    fn push(&mut self, function: &FunctionPlan) -> RuntimeFunctionId {
        runtime_function(function, self)
    }
}

fn runtime_function(
    function: &FunctionPlan,
    runtime_functions: &mut RuntimePlanBuilder,
) -> RuntimeFunctionId {
    match function.return_().kind() {
        ReturnExprKind::Int(return_) => {
            let id = IntFunctionId(runtime_functions.int_functions.len());
            runtime_functions.int_functions.push(RuntimeFunction::new(
                function.frame_layout(),
                function.steps().to_vec(),
                return_.clone(),
            ));
            RuntimeFunctionId::Int(id)
        }
        ReturnExprKind::String(return_) => {
            let id = StringFunctionId(runtime_functions.string_functions.len());
            runtime_functions
                .string_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            RuntimeFunctionId::String(id)
        }
        ReturnExprKind::Bool(return_) => {
            let id = BoolFunctionId(runtime_functions.bool_functions.len());
            runtime_functions.bool_functions.push(RuntimeFunction::new(
                function.frame_layout(),
                function.steps().to_vec(),
                return_.clone(),
            ));
            RuntimeFunctionId::Bool(id)
        }
        ReturnExprKind::Nil(return_) => {
            let id = NilFunctionId(runtime_functions.nil_functions.len());
            runtime_functions.nil_functions.push(RuntimeFunction::new(
                function.frame_layout(),
                function.steps().to_vec(),
                return_.clone(),
            ));
            RuntimeFunctionId::Nil(id)
        }
    }
}
