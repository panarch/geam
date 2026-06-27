use super::{
    BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, FunctionFunctionExpr,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionPlan, IntExpr, IntFunctionExpr,
    IntFunctionFunctionId, IntFunctionId, NilExpr, NilFunctionExpr, NilFunctionFunctionId,
    NilFunctionId, RuntimeFunction, RuntimeFunctionId, StringExpr, StringFunctionExpr,
    StringFunctionFunctionId, StringFunctionId,
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
            int_function_functions: runtime.int_function_functions,
            string_function_functions: runtime.string_function_functions,
            bool_function_functions: runtime.bool_function_functions,
            nil_function_functions: runtime.nil_function_functions,
            function_function_functions: runtime.function_function_functions,
        }
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
        ReturnExprKind::Function(return_) => {
            function_function(runtime_functions, function, return_)
        }
    }
}

fn function_function(
    runtime_functions: &mut RuntimePlanBuilder,
    function: &FunctionPlan,
    return_: &crate::plan::FunctionExpr,
) -> RuntimeFunctionId {
    let (id, return_type) = match return_.kind() {
        crate::plan::FunctionExprKind::Int(return_) => {
            let id = IntFunctionFunctionId(runtime_functions.int_function_functions.len());
            runtime_functions
                .int_function_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            (FunctionFunctionId::Int(id), return_.type_().clone())
        }
        crate::plan::FunctionExprKind::String(return_) => {
            let id = StringFunctionFunctionId(runtime_functions.string_function_functions.len());
            runtime_functions
                .string_function_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            (FunctionFunctionId::String(id), return_.type_().clone())
        }
        crate::plan::FunctionExprKind::Bool(return_) => {
            let id = BoolFunctionFunctionId(runtime_functions.bool_function_functions.len());
            runtime_functions
                .bool_function_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            (FunctionFunctionId::Bool(id), return_.type_().clone())
        }
        crate::plan::FunctionExprKind::Nil(return_) => {
            let id = NilFunctionFunctionId(runtime_functions.nil_function_functions.len());
            runtime_functions
                .nil_function_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            (FunctionFunctionId::Nil(id), return_.type_().clone())
        }
        crate::plan::FunctionExprKind::Function(return_) => {
            let id =
                FunctionFunctionFunctionId(runtime_functions.function_function_functions.len());
            runtime_functions
                .function_function_functions
                .push(RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    return_.clone(),
                ));
            (FunctionFunctionId::Function(id), return_.type_().clone())
        }
    };

    RuntimeFunctionId::Function { id, return_type }
}
