use crate::plan::execution::{
    ConstantId, CustomLocal, IntFunctionId, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ParamLocal, TupleLocalId,
};
use num_bigint::BigInt;

pub(crate) enum IntInstruction {
    Value(BigInt),
    Constant(ConstantId<IntLocalId>),
    Call {
        function: IntFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: IntFunctionLocalId,
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
        list: IntListLocalId,
        index: usize,
    },
    Add {
        left: IntLocalId,
        right: IntLocalId,
    },
    Sub {
        left: IntLocalId,
        right: IntLocalId,
    },
    Mult {
        left: IntLocalId,
        right: IntLocalId,
    },
    Div {
        left: IntLocalId,
        right: IntLocalId,
    },
    Remainder {
        left: IntLocalId,
        right: IntLocalId,
    },
    Negate(IntLocalId),
}
