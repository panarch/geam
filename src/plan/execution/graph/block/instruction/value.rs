use crate::plan::execution::{
    BoolFunctionId, BoolListLocalId, ConstantId, CustomConstructorId, CustomFunctionId,
    CustomFunctionLocal, CustomListLocalId, CustomLocal, FloatListLocalId, FloatLocalId,
    IntFunctionId, IntListLocalId, IntLocalId, ListLocal, NilFunctionId, NilListLocalId,
    ParamLocal, StringFunctionId, StringListLocalId, StringLocalId, TupleFunctionId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionId, UtfCodepointListLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) enum IntInstruction {
    Value(BigInt),
    Constant(ConstantId<IntLocalId>),
    Call {
        function: IntFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::IntFunctionLocalId,
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

pub(crate) enum FloatInstruction {
    Value(f64),
    Constant(ConstantId<FloatLocalId>),
    Call {
        function: crate::plan::execution::FloatFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::FloatFunctionLocalId,
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

pub(crate) enum StringInstruction {
    Value(EcoString),
    Constant(ConstantId<StringLocalId>),
    Call {
        function: StringFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::StringFunctionLocalId,
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

pub(crate) enum UtfCodepointInstruction {
    Call {
        function: UtfCodepointFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::UtfCodepointFunctionLocalId,
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

pub(crate) enum BoolInstruction {
    Value(bool),
    Constant(ConstantId<crate::plan::execution::BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::BoolFunctionLocalId,
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
    Not(crate::plan::execution::BoolLocalId),
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

pub(crate) enum NilInstruction {
    Value,
    Constant(ConstantId<crate::plan::execution::NilLocalId>),
    Call {
        function: NilFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::NilFunctionLocalId,
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

pub(crate) enum TupleInstruction {
    Value(Box<[ParamLocal]>),
    Constant(ConstantId<TupleLocalId>),
    Call {
        function: TupleFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::TupleFunctionLocalId,
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
