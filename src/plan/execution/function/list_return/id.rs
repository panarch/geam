use crate::plan::execution::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId,
    StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListFunctionId {
    index: usize,
    type_id: IntListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListFunctionId {
    index: usize,
    type_id: StringListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListFunctionId {
    index: usize,
    type_id: BitArrayListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListFunctionId {
    index: usize,
    type_id: UtfCodepointListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListFunctionId {
    index: usize,
    type_id: ParameterListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListFunctionId {
    index: usize,
    type_id: ParameterListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListFunctionId {
    index: usize,
    type_id: CustomListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListFunctionId {
    index: usize,
    type_id: FloatListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListFunctionId {
    index: usize,
    type_id: BoolListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListFunctionId {
    index: usize,
    type_id: NilListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListFunctionId {
    index: usize,
    type_id: TupleListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListFunctionId {
    index: usize,
    type_id: ListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListFunctionId {
    index: usize,
    type_id: FunctionListTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFunctionId {
    Parameter(ParameterListFunctionId),
    ParameterList(ParameterListListFunctionId),
    Int(IntListFunctionId),
    String(StringListFunctionId),
    BitArray(BitArrayListFunctionId),
    UtfCodepoint(UtfCodepointListFunctionId),
    Custom(CustomListFunctionId),
    Float(FloatListFunctionId),
    Bool(BoolListFunctionId),
    Nil(NilListFunctionId),
    Tuple(TupleListFunctionId),
    List(ListListFunctionId),
    Function(FunctionListFunctionId),
}

impl IntListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: IntListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> IntListTypeId {
        self.type_id
    }
}

impl StringListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: StringListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> StringListTypeId {
        self.type_id
    }
}

impl BitArrayListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: BitArrayListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> BitArrayListTypeId {
        self.type_id
    }
}

impl UtfCodepointListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: UtfCodepointListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> UtfCodepointListTypeId {
        self.type_id
    }
}

impl ParameterListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ParameterListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ParameterListTypeId {
        self.type_id
    }
}

impl ParameterListListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ParameterListListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ParameterListListTypeId {
        self.type_id
    }
}

impl CustomListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: CustomListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> CustomListTypeId {
        self.type_id
    }
}

impl FloatListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: FloatListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> FloatListTypeId {
        self.type_id
    }
}

impl BoolListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: BoolListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> BoolListTypeId {
        self.type_id
    }
}

impl NilListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: NilListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> NilListTypeId {
        self.type_id
    }
}

impl TupleListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: TupleListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> TupleListTypeId {
        self.type_id
    }
}

impl ListListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ListListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ListListTypeId {
        self.type_id
    }
}

impl FunctionListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: FunctionListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> FunctionListTypeId {
        self.type_id
    }
}
