use super::{
    BoolFunctionId, FloatFunctionId, FunctionFunctionId, IntFunctionId, ListFunctionId,
    NilFunctionId, ParamLocal, RuntimeFunctionId, StringFunctionId, TupleFunctionId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionReference {
    runtime_id: RuntimeFunctionId,
    params: Vec<ParamLocal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypedFunctionReference<Function> {
    function: Function,
    params: Vec<ParamLocal>,
}

pub(crate) type IntFunctionReference = TypedFunctionReference<IntFunctionId>;
pub(crate) type FloatFunctionReference = TypedFunctionReference<FloatFunctionId>;
pub(crate) type StringFunctionReference = TypedFunctionReference<StringFunctionId>;
pub(crate) type BoolFunctionReference = TypedFunctionReference<BoolFunctionId>;
pub(crate) type NilFunctionReference = TypedFunctionReference<NilFunctionId>;
pub(crate) type TupleFunctionReference = TypedFunctionReference<TupleFunctionId>;
pub(crate) type ListFunctionReference = TypedFunctionReference<ListFunctionId>;
pub(crate) type FunctionFunctionReference = TypedFunctionReference<FunctionFunctionId>;

impl FunctionReference {
    pub(crate) fn new(runtime_id: RuntimeFunctionId, params: Vec<ParamLocal>) -> Self {
        Self { runtime_id, params }
    }

    pub(crate) fn into_parts(self) -> (RuntimeFunctionId, Vec<ParamLocal>) {
        (self.runtime_id, self.params)
    }
}

impl<Function> TypedFunctionReference<Function> {
    pub(crate) fn new(function: Function, params: Vec<ParamLocal>) -> Self {
        Self { function, params }
    }

    pub(crate) fn into_parts(self) -> (Function, Vec<ParamLocal>) {
        (self.function, self.params)
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}
