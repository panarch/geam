use super::{
    DraftBitArray, DraftBool, DraftCustom, DraftFloat, DraftFunction, DraftGraphValue, DraftInt,
    DraftList, DraftNil, DraftStoredList, DraftString, DraftTuple, DraftUtfCodepoint,
    DraftValueRef,
};
use crate::plan::execution::graph::FunctionLocal;
use crate::plan::execution::graph::FunctionTarget;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListTypeId, BoolFunctionId, BoolListTypeId, ConstantId,
    CustomConstructorId, CustomFunctionId, CustomListTypeId, Endianness, FloatBitSize,
    FloatListTypeId, FunctionFunctionId, FunctionListTypeId, IntFunctionId, IntListTypeId,
    ListListTypeId, NilFunctionId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId,
    StringEncoding, StringFunctionId, StringListTypeId, TupleFunctionId, TupleListTypeId,
    UtfCodepointFunctionId, UtfCodepointListTypeId,
};

pub(in crate::plan::execution::lowering) enum DraftInstructionKind {
    Int(DraftIntInstruction),
    Float(DraftFloatInstruction),
    String(DraftStringInstruction),
    BitArray(DraftBitArrayInstruction),
    UtfCodepoint(DraftUtfCodepointInstruction),
    Custom(DraftCustomInstruction),
    Bool(DraftBoolInstruction),
    Nil(DraftNilInstruction),
    Tuple(DraftTupleInstruction),
    List(DraftListInstruction),
    Function {
        shape: super::SpecializedFunctionShape,
        kind: DraftFunctionInstruction,
    },
}

pub(in crate::plan::execution::lowering) enum DraftIntInstruction {
    Value(num_bigint::BigInt),
    Constant(ConstantId<crate::plan::execution::IntLocalId>),
    Call {
        function: IntFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
    Add {
        left: DraftInt,
        right: DraftInt,
    },
    Sub {
        left: DraftInt,
        right: DraftInt,
    },
    Mult {
        left: DraftInt,
        right: DraftInt,
    },
    Div {
        left: DraftInt,
        right: DraftInt,
    },
    Remainder {
        left: DraftInt,
        right: DraftInt,
    },
    Negate(DraftInt),
}

pub(in crate::plan::execution::lowering) enum DraftFloatInstruction {
    Value(f64),
    Constant(ConstantId<crate::plan::execution::FloatLocalId>),
    Call {
        function: crate::plan::execution::FloatFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
    Add {
        left: DraftFloat,
        right: DraftFloat,
    },
    Sub {
        left: DraftFloat,
        right: DraftFloat,
    },
    Mult {
        left: DraftFloat,
        right: DraftFloat,
    },
    Div {
        left: DraftFloat,
        right: DraftFloat,
    },
}

pub(in crate::plan::execution::lowering) enum DraftStringInstruction {
    Value(ecow::EcoString),
    Constant(ConstantId<crate::plan::execution::StringLocalId>),
    Call {
        function: StringFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
    Concatenate {
        left: DraftString,
        right: DraftString,
    },
    DropPrefix {
        value: DraftString,
        prefix: ecow::EcoString,
    },
}

pub(in crate::plan::execution::lowering) struct DraftBitArrayEvaluatedSize {
    pub(in crate::plan::execution::lowering::graph) value: DraftInt,
    pub(in crate::plan::execution::lowering::graph) unit: u8,
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayBitsSize {
    Fixed(usize),
    Evaluated(DraftBitArrayEvaluatedSize),
}

pub(in crate::plan::execution::lowering) enum DraftBitArraySegment {
    Int {
        value: DraftInt,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: DraftInt,
        size: DraftBitArrayEvaluatedSize,
        endianness: Endianness,
        site: crate::plan::PanicSite,
    },
    Float {
        value: DraftFloat,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: DraftFloat,
        size: DraftBitArrayEvaluatedSize,
        endianness: Endianness,
        site: crate::plan::PanicSite,
    },
    String {
        value: DraftString,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: DraftUtfCodepoint,
        encoding: StringEncoding,
    },
    Bits(DraftBitArray),
    SizedBits {
        value: DraftBitArray,
        size: DraftBitArrayBitsSize,
        site: crate::plan::PanicSite,
    },
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayInstruction {
    Value(Vec<DraftBitArraySegment>),
    Constant(ConstantId<crate::plan::execution::BitArrayLocalId>),
    Call {
        function: BitArrayFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftUtfCodepointInstruction {
    Call {
        function: UtfCodepointFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftCustomInstruction {
    Construct {
        constructor: CustomConstructorId,
        fields: Vec<DraftValueRef>,
    },
    Constant(ConstantId<crate::plan::execution::CustomLocal>),
    Call {
        function: CustomFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftBoolInstruction {
    Value(bool),
    Constant(ConstantId<crate::plan::execution::BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
    Not(DraftBool),
    LtInt {
        left: DraftInt,
        right: DraftInt,
    },
    LtEqInt {
        left: DraftInt,
        right: DraftInt,
    },
    GtInt {
        left: DraftInt,
        right: DraftInt,
    },
    GtEqInt {
        left: DraftInt,
        right: DraftInt,
    },
    LtFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    LtEqFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    GtFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    GtEqFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    Equal {
        left: DraftValueRef,
        right: DraftValueRef,
    },
    NotEqual {
        left: DraftValueRef,
        right: DraftValueRef,
    },
    StringStartsWith {
        value: DraftString,
        prefix: ecow::EcoString,
    },
    ListLengthEquals {
        value: DraftList,
        length: usize,
    },
    ListLengthAtLeast {
        value: DraftList,
        length: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftNilInstruction {
    Value,
    Constant(ConstantId<crate::plan::execution::NilLocalId>),
    Call {
        function: NilFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftTupleInstruction {
    Value(Vec<DraftValueRef>),
    Constant(ConstantId<crate::plan::execution::TupleLocalId>),
    Call {
        function: TupleFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftParameterListInstruction {
    Empty,
    Constant(ConstantId<crate::plan::execution::ParameterListLocalId>),
    Call {
        function: crate::plan::execution::ParameterListFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftTypedListInstruction<Element, Local, Function> {
    Value(Vec<Element>),
    Constant(ConstantId<Local>),
    Spread {
        elements: Vec<Element>,
        tail: DraftList,
    },
    Call {
        function: Function,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
    DropFirst {
        list: DraftList,
        count: usize,
    },
}

pub(in crate::plan::execution::lowering) enum DraftListInstruction {
    Parameter(ParameterListTypeId, DraftParameterListInstruction),
    ParameterList(
        ParameterListListTypeId,
        DraftTypedListInstruction<
            DraftList,
            crate::plan::execution::ParameterListListLocalId,
            crate::plan::execution::ParameterListListFunctionId,
        >,
    ),
    Int(
        IntListTypeId,
        DraftTypedListInstruction<
            DraftInt,
            crate::plan::execution::IntListLocalId,
            crate::plan::execution::IntListFunctionId,
        >,
    ),
    String(
        StringListTypeId,
        DraftTypedListInstruction<
            DraftString,
            crate::plan::execution::StringListLocalId,
            crate::plan::execution::StringListFunctionId,
        >,
    ),
    BitArray(
        BitArrayListTypeId,
        DraftTypedListInstruction<
            DraftBitArray,
            crate::plan::execution::BitArrayListLocalId,
            crate::plan::execution::BitArrayListFunctionId,
        >,
    ),
    UtfCodepoint(
        UtfCodepointListTypeId,
        DraftTypedListInstruction<
            DraftUtfCodepoint,
            crate::plan::execution::UtfCodepointListLocalId,
            crate::plan::execution::UtfCodepointListFunctionId,
        >,
    ),
    Custom(
        CustomListTypeId,
        DraftTypedListInstruction<
            DraftCustom,
            crate::plan::execution::CustomListLocalId,
            crate::plan::execution::CustomListFunctionId,
        >,
    ),
    Float(
        FloatListTypeId,
        DraftTypedListInstruction<
            DraftFloat,
            crate::plan::execution::FloatListLocalId,
            crate::plan::execution::FloatListFunctionId,
        >,
    ),
    Bool(
        BoolListTypeId,
        DraftTypedListInstruction<
            DraftBool,
            crate::plan::execution::BoolListLocalId,
            crate::plan::execution::BoolListFunctionId,
        >,
    ),
    Nil(
        NilListTypeId,
        DraftTypedListInstruction<
            DraftNil,
            crate::plan::execution::NilListLocalId,
            crate::plan::execution::NilListFunctionId,
        >,
    ),
    Tuple(
        TupleListTypeId,
        DraftTypedListInstruction<
            DraftTuple,
            crate::plan::execution::TupleListLocalId,
            crate::plan::execution::TupleListFunctionId,
        >,
    ),
    List(
        ListListTypeId,
        DraftTypedListInstruction<
            DraftStoredList,
            crate::plan::execution::ListListLocalId,
            crate::plan::execution::ListListFunctionId,
        >,
    ),
    Function(
        FunctionListTypeId,
        DraftTypedListInstruction<
            DraftFunction,
            crate::plan::execution::FunctionListLocalId,
            crate::plan::execution::FunctionListFunctionId,
        >,
    ),
}

pub(in crate::plan::execution::lowering) enum DraftFunctionInstruction {
    Constant(ConstantId<FunctionLocal>),
    Reference(FunctionTarget),
    Closure {
        target: FunctionTarget,
        captures: Vec<DraftFunctionCapture>,
    },
    Constructor(CustomConstructorId),
    Call {
        function: FunctionFunctionId,
        args: Vec<DraftValueRef>,
    },
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        tuple: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        list: DraftList,
        index: usize,
    },
}

pub(in crate::plan::execution::lowering) struct DraftFunctionCapture {
    pub(in crate::plan::execution::lowering::graph) target: crate::plan::execution::ParamLocal,
    pub(in crate::plan::execution::lowering::graph) source: DraftValueRef,
}

pub(in crate::plan::execution::lowering) trait DraftOperand {
    fn push_operand(&self, values: &mut Vec<DraftValueRef>);
}

impl<Family> DraftOperand for super::DraftValue<Family> {
    fn push_operand(&self, values: &mut Vec<DraftValueRef>) {
        values.push(self.erase());
    }
}

impl DraftOperand for DraftValueRef {
    fn push_operand(&self, values: &mut Vec<DraftValueRef>) {
        values.push(self.clone());
    }
}

impl DraftOperand for super::DraftStoredList {
    fn push_operand(&self, values: &mut Vec<DraftValueRef>) {
        values.push(self.erase());
    }
}

fn push_operands<Value: DraftOperand>(operands: &[Value], values: &mut Vec<DraftValueRef>) {
    for operand in operands {
        operand.push_operand(values);
    }
}

impl DraftIntInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(_) | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
            Self::Add { left, right }
            | Self::Sub { left, right }
            | Self::Mult { left, right }
            | Self::Div { left, right }
            | Self::Remainder { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::Negate(value) => value.push_operand(values),
        }
    }
}

impl DraftFloatInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(_) | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
            Self::Add { left, right }
            | Self::Sub { left, right }
            | Self::Mult { left, right }
            | Self::Div { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
        }
    }
}

impl DraftStringInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(_) | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
            Self::Concatenate { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::DropPrefix { value, .. } => value.push_operand(values),
        }
    }
}

impl DraftBitArrayEvaluatedSize {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        self.value.push_operand(values);
    }
}

impl DraftBitArrayBitsSize {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Fixed(_) => {}
            Self::Evaluated(size) => size.uses(values),
        }
    }
}

impl DraftBitArraySegment {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Int { value, .. } => value.push_operand(values),
            Self::Float { value, .. } => value.push_operand(values),
            Self::EvaluatedInt { value, size, .. } => {
                value.push_operand(values);
                size.uses(values);
            }
            Self::EvaluatedFloat { value, size, .. } => {
                value.push_operand(values);
                size.uses(values);
            }
            Self::String { value, .. } => value.push_operand(values),
            Self::UtfCodepoint { value, .. } => value.push_operand(values),
            Self::Bits(value) => value.push_operand(values),
            Self::SizedBits { value, size, .. } => {
                value.push_operand(values);
                size.uses(values);
            }
        }
    }
}

impl DraftBitArrayInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(segments) => {
                for segment in segments {
                    segment.uses(values);
                }
            }
            Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftUtfCodepointInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftCustomInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Construct { fields, .. } => push_operands(fields, values),
            Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftBoolInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(_) | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
            Self::Not(value) => value.push_operand(values),
            Self::LtInt { left, right }
            | Self::LtEqInt { left, right }
            | Self::GtInt { left, right }
            | Self::GtEqInt { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::LtFloat { left, right }
            | Self::LtEqFloat { left, right }
            | Self::GtFloat { left, right }
            | Self::GtEqFloat { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::Equal { left, right } | Self::NotEqual { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::StringStartsWith { value, .. } => value.push_operand(values),
            Self::ListLengthEquals { value, .. } | Self::ListLengthAtLeast { value, .. } => {
                value.push_operand(values);
            }
        }
    }
}

impl DraftNilInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftTupleInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(elements) => push_operands(elements, values),
            Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftParameterListInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Empty | Self::Constant(_) => {}
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}

impl<Element: DraftOperand, Local, Function> DraftTypedListInstruction<Element, Local, Function> {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Value(elements) => push_operands(elements, values),
            Self::Constant(_) => {}
            Self::Spread { elements, tail } => {
                push_operands(elements, values);
                tail.push_operand(values);
            }
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
            Self::DropFirst { list, .. } => list.push_operand(values),
        }
    }
}

impl DraftListInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Parameter(_, instruction) => instruction.uses(values),
            Self::ParameterList(_, instruction) => instruction.uses(values),
            Self::Int(_, instruction) => instruction.uses(values),
            Self::String(_, instruction) => instruction.uses(values),
            Self::BitArray(_, instruction) => instruction.uses(values),
            Self::UtfCodepoint(_, instruction) => instruction.uses(values),
            Self::Custom(_, instruction) => instruction.uses(values),
            Self::Float(_, instruction) => instruction.uses(values),
            Self::Bool(_, instruction) => instruction.uses(values),
            Self::Nil(_, instruction) => instruction.uses(values),
            Self::Tuple(_, instruction) => instruction.uses(values),
            Self::List(_, instruction) => instruction.uses(values),
            Self::Function(_, instruction) => instruction.uses(values),
        }
    }
}

impl DraftFunctionInstruction {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Constant(_) | Self::Reference(_) | Self::Constructor(_) => {}
            Self::Closure { captures, .. } => {
                for capture in captures {
                    capture.source.push_operand(values);
                }
            }
            Self::Call { args, .. } => push_operands(args, values),
            Self::FunctionCall { function, args } => {
                function.push_operand(values);
                push_operands(args, values);
            }
            Self::TupleIndex { tuple, .. } => tuple.push_operand(values),
            Self::CustomField { source, .. } => source.push_operand(values),
            Self::ListIndex { list, .. } => list.push_operand(values),
        }
    }
}
