use crate::plan::execution::{
    ConstantId, CustomLocal, ParamLocal, TupleFunctionId, TupleFunctionLocalId, TupleListLocalId,
    TupleLocalId,
};

pub(crate) enum TupleInstruction {
    Value(Box<[ParamLocal]>),
    Constant(ConstantId<TupleLocalId>),
    Call {
        function: TupleFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: TupleFunctionLocalId,
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
        list: TupleListLocalId,
        index: usize,
    },
}
