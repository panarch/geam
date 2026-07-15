use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, FunctionFunctionId,
    IntFunctionId, ListFunctionId, NilFunctionId, ParamSlot, RuntimeFunctionId, StringFunctionId,
    TupleFunctionId, UtfCodepointFunctionId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionReference {
    runtime_id: RuntimeFunctionId,
    params: Vec<ParamSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypedFunctionReference<Function> {
    function: Function,
    params: Vec<ParamSlot>,
}

pub(crate) type IntFunctionReference = TypedFunctionReference<IntFunctionId>;
pub(crate) type FloatFunctionReference = TypedFunctionReference<FloatFunctionId>;
pub(crate) type StringFunctionReference = TypedFunctionReference<StringFunctionId>;
pub(crate) type BitArrayFunctionReference = TypedFunctionReference<BitArrayFunctionId>;
pub(crate) type UtfCodepointFunctionReference = TypedFunctionReference<UtfCodepointFunctionId>;
pub(crate) type CustomFunctionReference = TypedFunctionReference<CustomFunctionId>;
pub(crate) type BoolFunctionReference = TypedFunctionReference<BoolFunctionId>;
pub(crate) type NilFunctionReference = TypedFunctionReference<NilFunctionId>;
pub(crate) type TupleFunctionReference = TypedFunctionReference<TupleFunctionId>;
pub(crate) type ListFunctionReference = TypedFunctionReference<ListFunctionId>;
pub(crate) type FunctionFunctionReference = TypedFunctionReference<FunctionFunctionId>;

impl FunctionReference {
    pub(crate) fn from_slots(runtime_id: RuntimeFunctionId, params: Vec<ParamSlot>) -> Self {
        Self { runtime_id, params }
    }

    #[cfg(test)]
    pub(crate) fn new(runtime_id: RuntimeFunctionId, params: Vec<super::ParamLocal>) -> Self {
        Self::from_slots(
            runtime_id,
            params.into_iter().map(ParamSlot::from_local).collect(),
        )
    }

    pub(crate) fn into_parts(self) -> (RuntimeFunctionId, Vec<ParamSlot>) {
        (self.runtime_id, self.params)
    }
}

impl<Function> TypedFunctionReference<Function> {
    pub(crate) fn from_slots(function: Function, params: Vec<ParamSlot>) -> Self {
        Self { function, params }
    }

    #[cfg(test)]
    pub(crate) fn new(function: Function, params: Vec<super::ParamLocal>) -> Self {
        Self::from_slots(
            function,
            params.into_iter().map(ParamSlot::from_local).collect(),
        )
    }

    pub(crate) fn into_parts(self) -> (Function, Vec<ParamSlot>) {
        (self.function, self.params)
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }
}
