use super::FunctionLocal;
use crate::plan::PanicSite;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListFunctionId, BitArrayListLocalId, BitArrayListTypeId,
    BoolFunctionId, BoolListFunctionId, BoolListLocalId, BoolListTypeId, ConstantId,
    CustomConstructorId, CustomFunctionId, CustomFunctionLocal, CustomListFunctionId,
    CustomListLocalId, CustomListTypeId, CustomLocal, FloatListFunctionId, FloatListLocalId,
    FloatListTypeId, FloatLocalId, FunctionFunctionId, FunctionFunctionLocal,
    FunctionListFunctionId, FunctionListLocalId, FunctionListTypeId, FunctionReturnFamily,
    GenericCallableId, GenericFunctionLocal, IntFunctionId, IntListFunctionId, IntListLocalId,
    IntListTypeId, IntLocalId, ListFunctionLocal, ListListFunctionId, ListListLocalId,
    ListListTypeId, ListLocal, NeverFunctionLocal, NilFunctionId, NilListFunctionId,
    NilListLocalId, NilListTypeId, ParamLocal, ParamSlot, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListLocalId, ParameterListListTypeId,
    ParameterListLocalId, ParameterListTypeId, StringFunctionId, StringListFunctionId,
    StringListLocalId, StringListTypeId, StringLocalId, TupleFunctionId, TupleListFunctionId,
    TupleListLocalId, TupleListTypeId, TupleLocalId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId, UtfCodepointListLocalId, UtfCodepointListTypeId,
    UtfCodepointLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringEncoding {
    Utf8,
    Utf16(Endianness),
    Utf32(Endianness),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

pub(crate) struct Instruction {
    output: ParamSlot,
    kind: InstructionKind,
}

pub(crate) enum InstructionKind {
    Int(IntInstruction),
    Float(FloatInstruction),
    String(StringInstruction),
    BitArray(BitArrayInstruction),
    UtfCodepoint(UtfCodepointInstruction),
    Custom(CustomInstruction),
    Bool(BoolInstruction),
    Nil(NilInstruction),
    Tuple(TupleInstruction),
    List(ListInstruction),
    Function(FunctionInstruction),
}

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

pub(crate) struct BitArrayEvaluatedSize {
    value: IntLocalId,
    unit: u8,
}

pub(crate) enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

pub(crate) enum BitArraySegment {
    Int {
        value: IntLocalId,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: IntLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    Float {
        value: FloatLocalId,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: FloatLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    String {
        value: StringLocalId,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: UtfCodepointLocalId,
        encoding: StringEncoding,
    },
    Bits(crate::plan::execution::BitArrayLocalId),
    SizedBits {
        value: crate::plan::execution::BitArrayLocalId,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

pub(crate) enum BitArrayInstruction {
    Value(Box<[BitArraySegment]>),
    Constant(ConstantId<crate::plan::execution::BitArrayLocalId>),
    Call {
        function: BitArrayFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::BitArrayFunctionLocalId,
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
        list: BitArrayListLocalId,
        index: usize,
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

pub(crate) enum ParameterListInstruction {
    Empty,
    Constant(ConstantId<ParameterListLocalId>),
    Call {
        function: ParameterListFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: ListFunctionLocal,
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
        list: ParameterListListLocalId,
        index: usize,
    },
}

pub(crate) enum TypedListInstruction<Element, Local, Function> {
    Value(Box<[Element]>),
    Constant(ConstantId<Local>),
    Spread {
        elements: Box<[Element]>,
        tail: Local,
    },
    Call {
        function: Function,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: ListFunctionLocal,
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
        list: ListListLocalId,
        index: usize,
    },
    DropFirst {
        list: Local,
        count: usize,
    },
}

pub(crate) enum ListInstruction {
    Parameter(ParameterListTypeId, ParameterListInstruction),
    ParameterList(
        ParameterListListTypeId,
        TypedListInstruction<
            ParameterListLocalId,
            ParameterListListLocalId,
            ParameterListListFunctionId,
        >,
    ),
    Int(
        IntListTypeId,
        TypedListInstruction<IntLocalId, IntListLocalId, IntListFunctionId>,
    ),
    String(
        StringListTypeId,
        TypedListInstruction<StringLocalId, StringListLocalId, StringListFunctionId>,
    ),
    BitArray(
        BitArrayListTypeId,
        TypedListInstruction<
            crate::plan::execution::BitArrayLocalId,
            BitArrayListLocalId,
            BitArrayListFunctionId,
        >,
    ),
    UtfCodepoint(
        UtfCodepointListTypeId,
        TypedListInstruction<
            UtfCodepointLocalId,
            UtfCodepointListLocalId,
            UtfCodepointListFunctionId,
        >,
    ),
    Custom(
        CustomListTypeId,
        TypedListInstruction<CustomLocal, CustomListLocalId, CustomListFunctionId>,
    ),
    Float(
        FloatListTypeId,
        TypedListInstruction<FloatLocalId, FloatListLocalId, FloatListFunctionId>,
    ),
    Bool(
        BoolListTypeId,
        TypedListInstruction<
            crate::plan::execution::BoolLocalId,
            BoolListLocalId,
            BoolListFunctionId,
        >,
    ),
    Nil(
        NilListTypeId,
        TypedListInstruction<crate::plan::execution::NilLocalId, NilListLocalId, NilListFunctionId>,
    ),
    Tuple(
        TupleListTypeId,
        TypedListInstruction<TupleLocalId, TupleListLocalId, TupleListFunctionId>,
    ),
    List(
        ListListTypeId,
        TypedListInstruction<super::StoredListLocal, ListListLocalId, ListListFunctionId>,
    ),
    Function(
        FunctionListTypeId,
        TypedListInstruction<FunctionLocal, FunctionListLocalId, FunctionListFunctionId>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionTarget {
    Generic(GenericCallableId),
    Never(crate::plan::execution::NeverFunctionId),
    Int(IntFunctionId),
    Float(crate::plan::execution::FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple(TupleFunctionId),
    List(crate::plan::execution::ListFunctionId),
    Function(FunctionFunctionId),
}

pub(crate) struct FunctionInstruction {
    type_: crate::plan::execution::FunctionType,
    family: FunctionReturnFamily,
    kind: FunctionInstructionKind,
}

pub(crate) enum FunctionCapture {
    Int {
        target: IntLocalId,
        source: IntLocalId,
    },
    Float {
        target: FloatLocalId,
        source: FloatLocalId,
    },
    String {
        target: StringLocalId,
        source: StringLocalId,
    },
    BitArray {
        target: crate::plan::execution::BitArrayLocalId,
        source: crate::plan::execution::BitArrayLocalId,
    },
    UtfCodepoint {
        target: UtfCodepointLocalId,
        source: UtfCodepointLocalId,
    },
    Custom {
        target: CustomLocal,
        source: CustomLocal,
    },
    Bool {
        target: crate::plan::execution::BoolLocalId,
        source: crate::plan::execution::BoolLocalId,
    },
    Nil {
        target: crate::plan::execution::NilLocalId,
        source: crate::plan::execution::NilLocalId,
    },
    Tuple {
        target: TupleLocalId,
        source: TupleLocalId,
    },
    ParameterList {
        target: ParameterListLocalId,
        source: ParameterListLocalId,
    },
    ParameterListList {
        target: ParameterListListLocalId,
        source: ParameterListListLocalId,
    },
    IntList {
        target: IntListLocalId,
        source: IntListLocalId,
    },
    StringList {
        target: StringListLocalId,
        source: StringListLocalId,
    },
    BitArrayList {
        target: BitArrayListLocalId,
        source: BitArrayListLocalId,
    },
    UtfCodepointList {
        target: UtfCodepointListLocalId,
        source: UtfCodepointListLocalId,
    },
    CustomList {
        target: CustomListLocalId,
        source: CustomListLocalId,
    },
    FloatList {
        target: FloatListLocalId,
        source: FloatListLocalId,
    },
    BoolList {
        target: BoolListLocalId,
        source: BoolListLocalId,
    },
    NilList {
        target: NilListLocalId,
        source: NilListLocalId,
    },
    TupleList {
        target: TupleListLocalId,
        source: TupleListLocalId,
    },
    ListList {
        target: ListListLocalId,
        source: ListListLocalId,
    },
    FunctionList {
        target: FunctionListLocalId,
        source: FunctionListLocalId,
    },
    IntFunction {
        target: crate::plan::execution::IntFunctionLocalId,
        source: crate::plan::execution::IntFunctionLocalId,
    },
    FloatFunction {
        target: crate::plan::execution::FloatFunctionLocalId,
        source: crate::plan::execution::FloatFunctionLocalId,
    },
    StringFunction {
        target: crate::plan::execution::StringFunctionLocalId,
        source: crate::plan::execution::StringFunctionLocalId,
    },
    BitArrayFunction {
        target: crate::plan::execution::BitArrayFunctionLocalId,
        source: crate::plan::execution::BitArrayFunctionLocalId,
    },
    UtfCodepointFunction {
        target: crate::plan::execution::UtfCodepointFunctionLocalId,
        source: crate::plan::execution::UtfCodepointFunctionLocalId,
    },
    GenericFunction {
        target: GenericFunctionLocal,
        source: GenericFunctionLocal,
    },
    NeverFunction {
        target: NeverFunctionLocal,
        source: NeverFunctionLocal,
    },
    CustomFunction {
        target: CustomFunctionLocal,
        source: CustomFunctionLocal,
    },
    BoolFunction {
        target: crate::plan::execution::BoolFunctionLocalId,
        source: crate::plan::execution::BoolFunctionLocalId,
    },
    NilFunction {
        target: crate::plan::execution::NilFunctionLocalId,
        source: crate::plan::execution::NilFunctionLocalId,
    },
    TupleFunction {
        target: crate::plan::execution::TupleFunctionLocalId,
        source: crate::plan::execution::TupleFunctionLocalId,
    },
    ListFunction {
        target: ListFunctionLocal,
        source: ListFunctionLocal,
    },
    FunctionFunction {
        target: FunctionFunctionLocal,
        source: FunctionFunctionLocal,
    },
}

pub(crate) enum FunctionInstructionKind {
    Constant(ConstantId<FunctionLocal>),
    Reference(FunctionTarget),
    Closure {
        target: FunctionTarget,
        captures: Box<[FunctionCapture]>,
    },
    Constructor(CustomConstructorId),
    Call {
        function: FunctionFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: FunctionFunctionLocal,
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
        list: FunctionListLocalId,
        index: usize,
    },
}

impl Instruction {
    pub(in crate::plan::execution) fn new(output: ParamSlot, kind: InstructionKind) -> Self {
        Self { output, kind }
    }

    pub(crate) fn output(&self) -> &ParamSlot {
        &self.output
    }

    pub(crate) fn kind(&self) -> &InstructionKind {
        &self.kind
    }
}

impl FunctionInstruction {
    pub(in crate::plan::execution) fn new(
        type_: crate::plan::execution::FunctionType,
        family: FunctionReturnFamily,
        kind: FunctionInstructionKind,
    ) -> Self {
        Self {
            type_,
            family,
            kind,
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::execution::FunctionType {
        &self.type_
    }

    pub(crate) fn family(&self) -> FunctionReturnFamily {
        self.family
    }

    pub(crate) fn kind(&self) -> &FunctionInstructionKind {
        &self.kind
    }
}

impl BitArrayEvaluatedSize {
    pub(in crate::plan::execution) fn new(value: IntLocalId, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> IntLocalId {
        self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}
