use crate::plan::execution::{
    ConstantId, CustomConstructorId, CustomFunctionId, CustomFunctionLocal, CustomListLocalId,
    CustomLocal, ParamLocal, TupleLocalId,
};

pub(crate) enum CustomInstruction {
    Construct {
        constructor: CustomConstructorId,
        fields: Box<[ParamLocal]>,
    },
    Constant(ConstantId<CustomLocal>),
    Call {
        function: CustomFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: CustomFunctionLocal,
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
        list: CustomListLocalId,
        index: usize,
    },
}
