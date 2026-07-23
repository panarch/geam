use crate::plan::execution::{
    BoolFunctionId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId, ConstantId, CustomLocal,
    FloatLocalId, IntLocalId, ListLocal, ParamLocal, StringLocalId, TupleLocalId,
};
use ecow::EcoString;

pub(crate) enum BoolInstruction {
    Value(bool),
    Constant(ConstantId<BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: BoolFunctionLocalId,
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
        list: BoolListLocalId,
        index: usize,
    },
    Not(BoolLocalId),
    LtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    LtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Equal {
        left: ParamLocal,
        right: ParamLocal,
    },
    NotEqual {
        left: ParamLocal,
        right: ParamLocal,
    },
    StringStartsWith {
        value: StringLocalId,
        prefix: EcoString,
    },
    ListLengthEquals {
        value: ListLocal,
        length: usize,
    },
    ListLengthAtLeast {
        value: ListLocal,
        length: usize,
    },
}
