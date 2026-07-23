use crate::plan::execution::{
    ConstantId, CustomLocal, ParamLocal, StringFunctionId, StringFunctionLocalId,
    StringListLocalId, StringLocalId, TupleLocalId,
};
use ecow::EcoString;

pub(crate) enum StringInstruction {
    Value(EcoString),
    Constant(ConstantId<StringLocalId>),
    Call {
        function: StringFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: StringFunctionLocalId,
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
        list: StringListLocalId,
        index: usize,
    },
    Concatenate {
        left: StringLocalId,
        right: StringLocalId,
    },
    DropPrefix {
        value: StringLocalId,
        prefix: EcoString,
    },
}
