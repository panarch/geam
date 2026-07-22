use super::super::FunctionLocal;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListLocalId, CustomConstructorId, CustomFunctionId,
    CustomFunctionLocal, CustomListLocalId, CustomLocal, FloatListLocalId, FunctionFunctionId,
    FunctionFunctionLocal, FunctionListLocalId, FunctionReturnFamily, GenericCallableId,
    GenericFunctionLocal, IntFunctionId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, NeverFunctionLocal, NilFunctionId, NilListLocalId, ParamLocal,
    ParameterListListLocalId, ParameterListLocalId, StringFunctionId, StringListLocalId,
    StringLocalId, TupleFunctionId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};

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
    Bool(crate::plan::execution::BoolFunctionId),
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
        target: crate::plan::execution::FloatLocalId,
        source: crate::plan::execution::FloatLocalId,
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
        target: crate::plan::execution::BoolListLocalId,
        source: crate::plan::execution::BoolListLocalId,
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
    Constant(crate::plan::execution::ConstantId<FunctionLocal>),
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
