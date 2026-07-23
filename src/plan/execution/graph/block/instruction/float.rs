use crate::plan::execution::{
    ConstantId, CustomLocal, FloatFunctionId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
    ParamLocal, TupleLocalId,
};

pub(crate) enum FloatInstruction {
    Value(f64),
    Constant(ConstantId<FloatLocalId>),
    Call {
        function: FloatFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: FloatFunctionLocalId,
        args: Box<[ParamLocal]>,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: FloatListLocalId,
        index: usize,
    },
    Add {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Sub {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Mult {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Div {
        left: FloatLocalId,
        right: FloatLocalId,
    },
}
