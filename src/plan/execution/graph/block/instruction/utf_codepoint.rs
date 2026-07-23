use crate::plan::execution::{
    CustomLocal, ParamLocal, TupleLocalId, UtfCodepointFunctionId, UtfCodepointFunctionLocalId,
    UtfCodepointListLocalId,
};

pub(crate) enum UtfCodepointInstruction {
    Call {
        function: UtfCodepointFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: UtfCodepointFunctionLocalId,
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
        list: UtfCodepointListLocalId,
        index: usize,
    },
}
