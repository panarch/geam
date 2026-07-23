use crate::plan::execution::{
    ConstantId, CustomLocal, NilFunctionId, NilFunctionLocalId, NilListLocalId, NilLocalId,
    ParamLocal, TupleLocalId,
};

pub(crate) enum NilInstruction {
    Value,
    Constant(ConstantId<NilLocalId>),
    Call {
        function: NilFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: NilFunctionLocalId,
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
        list: NilListLocalId,
        index: usize,
    },
}
