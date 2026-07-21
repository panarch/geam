#[cfg(test)]
use super::ListTypeId;
use super::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    FunctionType, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId, ValueType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomLocal {
    id: CustomLocalId,
    shape: super::CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenericCallableId {
    Function {
        template: usize,
        substitution: Box<[super::ValueShapeId]>,
    },
    Constructor(super::CustomConstructorId),
}

impl GenericCallableId {
    pub(in crate::plan::execution) fn function(
        template: usize,
        substitution: Vec<super::ValueShapeId>,
    ) -> Self {
        Self::Function {
            template,
            substitution: substitution.into_boxed_slice(),
        }
    }

    pub(in crate::plan::execution) fn constructor(constructor: super::CustomConstructorId) -> Self {
        Self::Constructor(constructor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListLocal {
    Parameter {
        local: ParameterListLocalId,
        type_id: ParameterListTypeId,
    },
    ParameterList {
        local: ParameterListListLocalId,
        type_id: ParameterListListTypeId,
    },
    Int {
        local: IntListLocalId,
        type_id: IntListTypeId,
    },
    String {
        local: StringListLocalId,
        type_id: StringListTypeId,
    },
    BitArray {
        local: BitArrayListLocalId,
        type_id: BitArrayListTypeId,
    },
    UtfCodepoint {
        local: UtfCodepointListLocalId,
        type_id: UtfCodepointListTypeId,
    },
    Custom {
        local: CustomListLocalId,
        type_id: CustomListTypeId,
    },
    Float {
        local: FloatListLocalId,
        type_id: FloatListTypeId,
    },
    Bool {
        local: BoolListLocalId,
        type_id: BoolListTypeId,
    },
    Nil {
        local: NilListLocalId,
        type_id: NilListTypeId,
    },
    Tuple {
        local: TupleListLocalId,
        type_id: TupleListTypeId,
    },
    List {
        local: ListListLocalId,
        type_id: ListListTypeId,
    },
    Function {
        local: FunctionListLocalId,
        type_id: FunctionListTypeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitArrayFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtfCodepointFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NeverFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NeverFunctionLocal {
    id: NeverFunctionLocalId,
    type_: super::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericFunctionLocal {
    id: GenericFunctionLocalId,
    type_: super::GenericFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionLocal {
    id: CustomFunctionLocalId,
    type_: super::CustomFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitArrayListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtfCodepointListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionListFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFunctionLocal {
    Parameter {
        local: ParameterListFunctionLocalId,
        type_: FunctionType,
        list_type: ParameterListTypeId,
    },
    ParameterList {
        local: ParameterListListFunctionLocalId,
        type_: FunctionType,
        list_type: ParameterListListTypeId,
    },
    Int {
        local: IntListFunctionLocalId,
        type_: FunctionType,
        list_type: IntListTypeId,
    },
    String {
        local: StringListFunctionLocalId,
        type_: FunctionType,
        list_type: StringListTypeId,
    },
    BitArray {
        local: BitArrayListFunctionLocalId,
        type_: FunctionType,
        list_type: BitArrayListTypeId,
    },
    UtfCodepoint {
        local: UtfCodepointListFunctionLocalId,
        type_: FunctionType,
        list_type: UtfCodepointListTypeId,
    },
    Custom {
        local: CustomListFunctionLocalId,
        type_: FunctionType,
        list_type: CustomListTypeId,
    },
    Float {
        local: FloatListFunctionLocalId,
        type_: FunctionType,
        list_type: FloatListTypeId,
    },
    Bool {
        local: BoolListFunctionLocalId,
        type_: FunctionType,
        list_type: BoolListTypeId,
    },
    Nil {
        local: NilListFunctionLocalId,
        type_: FunctionType,
        list_type: NilListTypeId,
    },
    Tuple {
        local: TupleListFunctionLocalId,
        type_: FunctionType,
        list_type: TupleListTypeId,
    },
    List {
        local: ListListFunctionLocalId,
        type_: FunctionType,
        list_type: ListListTypeId,
    },
    Function {
        local: FunctionListFunctionLocalId,
        type_: FunctionType,
        list_type: FunctionListTypeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionFunctionLocal {
    id: FunctionFunctionLocalId,
    type_: super::FunctionFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Never(NeverFunctionId),
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple {
        id: TupleFunctionId,
        return_type: Vec<ValueType>,
    },
    List(ListFunctionId),
    Function {
        id: FunctionFunctionId,
        return_type: FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NeverFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomFunctionId {
    index: usize,
    return_shape: super::CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionId(pub(crate) usize);

macro_rules! list_function_id {
    ($name:ident, $type_id:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            index: usize,
            type_id: $type_id,
        }

        impl $name {
            pub(in crate::plan::execution) fn new(index: usize, type_id: $type_id) -> Self {
                Self { index, type_id }
            }

            pub(crate) fn index(self) -> usize {
                self.index
            }

            pub(crate) fn type_id(self) -> $type_id {
                self.type_id
            }
        }
    };
}

list_function_id!(IntListFunctionId, IntListTypeId);
list_function_id!(StringListFunctionId, StringListTypeId);
list_function_id!(BitArrayListFunctionId, BitArrayListTypeId);
list_function_id!(UtfCodepointListFunctionId, UtfCodepointListTypeId);
list_function_id!(ParameterListFunctionId, ParameterListTypeId);
list_function_id!(ParameterListListFunctionId, ParameterListListTypeId);
list_function_id!(CustomListFunctionId, CustomListTypeId);
list_function_id!(FloatListFunctionId, FloatListTypeId);
list_function_id!(BoolListFunctionId, BoolListTypeId);
list_function_id!(NilListFunctionId, NilListTypeId);
list_function_id!(TupleListFunctionId, TupleListTypeId);
list_function_id!(ListListFunctionId, ListListTypeId);
list_function_id!(FunctionListFunctionId, FunctionListTypeId);

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionFunctionId {
    Generic(GenericFunctionFunctionId),
    Never(NeverFunctionFunctionId),
    Int(IntFunctionFunctionId),
    Float(FloatFunctionFunctionId),
    String(StringFunctionFunctionId),
    BitArray(BitArrayFunctionFunctionId),
    UtfCodepoint(UtfCodepointFunctionFunctionId),
    Custom(CustomFunctionFunctionId),
    Bool(BoolFunctionFunctionId),
    Nil(NilFunctionFunctionId),
    Tuple(TupleFunctionFunctionId),
    List(ListFunctionFunctionId),
    Function(FunctionFunctionFunctionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionReturnFamily {
    Generic,
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericFunctionFunctionId {
    index: usize,
    type_: super::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NeverFunctionFunctionId {
    index: usize,
    type_: super::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFunctionFunctionId {
    index: usize,
    type_: super::CustomFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFunctionFunctionId {
    Parameter {
        id: ParameterListFunctionFunctionId,
        type_: FunctionType,
        list_type: ParameterListTypeId,
    },
    ParameterList {
        id: ParameterListListFunctionFunctionId,
        type_: FunctionType,
        list_type: ParameterListListTypeId,
    },
    Int {
        id: IntListFunctionFunctionId,
        type_: FunctionType,
        list_type: IntListTypeId,
    },
    String {
        id: StringListFunctionFunctionId,
        type_: FunctionType,
        list_type: StringListTypeId,
    },
    BitArray {
        id: BitArrayListFunctionFunctionId,
        type_: FunctionType,
        list_type: BitArrayListTypeId,
    },
    UtfCodepoint {
        id: UtfCodepointListFunctionFunctionId,
        type_: FunctionType,
        list_type: UtfCodepointListTypeId,
    },
    Custom {
        id: CustomListFunctionFunctionId,
        type_: FunctionType,
        list_type: CustomListTypeId,
    },
    Float {
        id: FloatListFunctionFunctionId,
        type_: FunctionType,
        list_type: FloatListTypeId,
    },
    Bool {
        id: BoolListFunctionFunctionId,
        type_: FunctionType,
        list_type: BoolListTypeId,
    },
    Nil {
        id: NilListFunctionFunctionId,
        type_: FunctionType,
        list_type: NilListTypeId,
    },
    Tuple {
        id: TupleListFunctionFunctionId,
        type_: FunctionType,
        list_type: TupleListTypeId,
    },
    List {
        id: ListListFunctionFunctionId,
        type_: FunctionType,
        list_type: ListListTypeId,
    },
    Function {
        id: FunctionListFunctionFunctionId,
        type_: FunctionType,
        list_type: FunctionListTypeId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionFunctionFunctionId {
    index: usize,
    type_: super::FunctionFunctionType,
}

impl CustomFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomFunctionLocalId,
        type_: super::CustomFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> CustomFunctionLocalId {
        self.id
    }
}

impl GenericFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: GenericFunctionLocalId,
        type_: super::GenericFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> GenericFunctionLocalId {
        self.id
    }
}

impl NeverFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: NeverFunctionLocalId,
        type_: super::GenericFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> NeverFunctionLocalId {
        self.id
    }
}

impl CustomLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomLocalId,
        shape: super::CustomValueShape,
    ) -> Self {
        Self { id, shape }
    }

    pub(crate) fn id(self) -> CustomLocalId {
        self.id
    }
}

impl CustomFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        return_shape: super::CustomValueShape,
    ) -> Self {
        Self {
            index,
            return_shape,
        }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl FunctionFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: FunctionFunctionLocalId,
        type_: super::FunctionFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> FunctionFunctionLocalId {
        self.id
    }
}

impl CustomFunctionFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_: super::CustomFunctionType) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl GenericFunctionFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_: super::GenericFunctionType) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl NeverFunctionFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_: super::GenericFunctionType) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl FunctionFunctionFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        type_: super::FunctionFunctionType,
    ) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &super::FunctionFunctionType {
        &self.type_
    }
}

#[cfg(test)]
impl FunctionFunctionId {
    pub(crate) fn generic(&self) -> Option<GenericFunctionFunctionId> {
        match self {
            Self::Generic(id) => Some(id.clone()),
            _ => None,
        }
    }

    pub(crate) fn never(&self) -> Option<NeverFunctionFunctionId> {
        match self {
            Self::Never(id) => Some(id.clone()),
            _ => None,
        }
    }

    pub(crate) fn bit_array(&self) -> Option<BitArrayFunctionFunctionId> {
        match self {
            Self::BitArray(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn utf_codepoint(&self) -> Option<UtfCodepointFunctionFunctionId> {
        match self {
            Self::UtfCodepoint(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn custom(&self) -> Option<CustomFunctionFunctionId> {
        match self {
            Self::Custom(id) => Some(id.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
impl ListFunctionLocal {
    pub(crate) fn type_(&self) -> &FunctionType {
        match self {
            Self::Parameter { type_, .. }
            | Self::ParameterList { type_, .. }
            | Self::Int { type_, .. }
            | Self::String { type_, .. }
            | Self::BitArray { type_, .. }
            | Self::UtfCodepoint { type_, .. }
            | Self::Custom { type_, .. }
            | Self::Float { type_, .. }
            | Self::Bool { type_, .. }
            | Self::Nil { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::List { type_, .. }
            | Self::Function { type_, .. } => type_,
        }
    }

    #[cfg(test)]
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Parameter { list_type, .. } => list_type.list_type(),
            Self::ParameterList { list_type, .. } => list_type.list_type(),
            Self::Int { list_type, .. } => list_type.list_type(),
            Self::String { list_type, .. } => list_type.list_type(),
            Self::BitArray { list_type, .. } => list_type.list_type(),
            Self::UtfCodepoint { list_type, .. } => list_type.list_type(),
            Self::Custom { list_type, .. } => list_type.list_type(),
            Self::Float { list_type, .. } => list_type.list_type(),
            Self::Bool { list_type, .. } => list_type.list_type(),
            Self::Nil { list_type, .. } => list_type.list_type(),
            Self::Tuple { list_type, .. } => list_type.list_type(),
            Self::List { list_type, .. } => list_type.list_type(),
            Self::Function { list_type, .. } => list_type.list_type(),
        }
    }
}

#[cfg(test)]
impl ListLocal {
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Parameter { type_id, .. } => type_id.list_type(),
            Self::ParameterList { type_id, .. } => type_id.list_type(),
            Self::Int { type_id, .. } => type_id.list_type(),
            Self::String { type_id, .. } => type_id.list_type(),
            Self::BitArray { type_id, .. } => type_id.list_type(),
            Self::UtfCodepoint { type_id, .. } => type_id.list_type(),
            Self::Custom { type_id, .. } => type_id.list_type(),
            Self::Float { type_id, .. } => type_id.list_type(),
            Self::Bool { type_id, .. } => type_id.list_type(),
            Self::Nil { type_id, .. } => type_id.list_type(),
            Self::Tuple { type_id, .. } => type_id.list_type(),
            Self::List { type_id, .. } => type_id.list_type(),
            Self::Function { type_id, .. } => type_id.list_type(),
        }
    }
}

#[cfg(test)]
impl ListFunctionFunctionId {
    pub(crate) fn type_(&self) -> &FunctionType {
        match self {
            Self::Parameter { type_, .. }
            | Self::ParameterList { type_, .. }
            | Self::Int { type_, .. }
            | Self::String { type_, .. }
            | Self::BitArray { type_, .. }
            | Self::UtfCodepoint { type_, .. }
            | Self::Custom { type_, .. }
            | Self::Float { type_, .. }
            | Self::Bool { type_, .. }
            | Self::Nil { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::List { type_, .. }
            | Self::Function { type_, .. } => type_,
        }
    }
}

impl std::fmt::Display for FunctionReturnFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => f.write_str("Generic"),
            Self::Never => f.write_str("Never"),
            Self::Int => f.write_str("Int"),
            Self::Float => f.write_str("Float"),
            Self::String => f.write_str("String"),
            Self::BitArray => f.write_str("BitArray"),
            Self::UtfCodepoint => f.write_str("UtfCodepoint"),
            Self::Custom => f.write_str("Custom"),
            Self::Bool => f.write_str("Bool"),
            Self::Nil => f.write_str("Nil"),
            Self::Tuple => f.write_str("Tuple"),
            Self::List => f.write_str("List"),
            Self::Function => f.write_str("Function"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayFunctionFunctionId, BoolListFunctionFunctionId, CustomFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionFunctionId, FunctionListFunctionFunctionId,
        FunctionReturnFamily, IntFunctionFunctionId, IntListFunctionFunctionId,
        ListFunctionFunctionId, ListFunctionLocal, ListListFunctionFunctionId, ListLocal,
        NeverFunctionFunctionId, NilListFunctionFunctionId, ParameterListFunctionFunctionId,
        ParameterListFunctionLocalId, ParameterListListFunctionFunctionId,
        ParameterListListFunctionLocalId, ParameterListListLocalId, ParameterListLocalId,
        RuntimeFunctionId, StringListFunctionFunctionId, TupleListFunctionFunctionId,
        UtfCodepointFunctionFunctionId,
    };
    use crate::plan::{CustomType, CustomTypeName, ValueType};

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn function_return_family_display_names_every_family() {
        assert_eq!(
            [
                FunctionReturnFamily::Generic,
                FunctionReturnFamily::Never,
                FunctionReturnFamily::Int,
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
                FunctionReturnFamily::BitArray,
                FunctionReturnFamily::UtfCodepoint,
                FunctionReturnFamily::Custom,
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::List,
                FunctionReturnFamily::Function,
            ]
            .map(|family| family.to_string()),
            [
                "Generic",
                "Never",
                "Int",
                "Float",
                "String",
                "BitArray",
                "UtfCodepoint",
                "Custom",
                "Bool",
                "Nil",
                "Tuple",
                "List",
                "Function",
            ],
        );
    }

    #[test]
    fn parameter_list_ids_preserve_symbolic_and_nested_storage_types() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let function_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Nil,
        );
        let parameter_local = ListLocal::Parameter {
            local: ParameterListLocalId(0),
            type_id: parameter,
        };
        let nested_local = ListLocal::ParameterList {
            local: ParameterListListLocalId(0),
            type_id: nested,
        };
        let parameter_function_local = ListFunctionLocal::Parameter {
            local: ParameterListFunctionLocalId(0),
            type_: function_type.clone(),
            list_type: parameter,
        };
        let nested_function_local = ListFunctionLocal::ParameterList {
            local: ParameterListListFunctionLocalId(0),
            type_: function_type.clone(),
            list_type: nested,
        };
        let parameter_function = ListFunctionFunctionId::Parameter {
            id: ParameterListFunctionFunctionId(0),
            type_: function_type.clone(),
            list_type: parameter,
        };
        let nested_function = ListFunctionFunctionId::ParameterList {
            id: ParameterListListFunctionFunctionId(0),
            type_: function_type.clone(),
            list_type: nested,
        };

        assert_eq!(
            parameter_plan.list_value_type(parameter_local.list_type()),
            ValueType::List(Box::new(ValueType::Parameter(
                crate::plan::TypeParameterId(0),
            ))),
        );
        assert_eq!(
            nested_plan.list_value_type(nested_local.list_type()),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                crate::plan::TypeParameterId(0),
            ))))),
        );
        assert_eq!(parameter_function_local.type_(), &function_type);
        assert_eq!(nested_function_local.type_(), &function_type);
        assert_eq!(
            parameter_plan.list_value_type(parameter_function_local.list_type()),
            ValueType::List(Box::new(ValueType::Parameter(
                crate::plan::TypeParameterId(0),
            ))),
        );
        assert_eq!(
            nested_plan.list_value_type(nested_function_local.list_type()),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                crate::plan::TypeParameterId(0),
            ))))),
        );
        assert_eq!(parameter_function.type_(), &function_type);
        assert_eq!(nested_function.type_(), &function_type);
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).generic(),
            None,
        );
    }

    #[test]
    fn never_function_function_id_projection_is_typed() {
        let function_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let type_ = crate::plan::execution::GenericFunctionType::from_shapes(
            function_type.clone(),
            crate::plan::execution::FunctionShape::new(
                crate::plan::execution::ValueShapeId::new(0),
                function_type,
            ),
        );
        let never = NeverFunctionFunctionId::new(2, type_);
        let id = FunctionFunctionId::Never(never.clone());

        assert_eq!(id.never(), Some(never));
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).never(),
            None
        );
    }

    #[test]
    fn generic_function_function_id_projection_is_typed() {
        let function_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let type_ = crate::plan::execution::GenericFunctionType::from_shapes(
            function_type.clone(),
            crate::plan::execution::FunctionShape::new(
                crate::plan::execution::ValueShapeId::new(0),
                function_type,
            ),
        );
        let generic = super::GenericFunctionFunctionId::new(2, type_);
        let id = FunctionFunctionId::Generic(generic.clone());

        assert_eq!(id.generic(), Some(generic));
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).generic(),
            None,
        );
    }

    #[test]
    fn bit_array_function_function_id_projection_is_typed() {
        let id = FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(2));

        assert_eq!(id.bit_array(), Some(BitArrayFunctionFunctionId(2)));
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).bit_array(),
            None,
        );
    }

    #[test]
    fn utf_codepoint_function_function_id_projection_is_typed() {
        let id = FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(2));

        assert_eq!(id.utf_codepoint(), Some(UtfCodepointFunctionFunctionId(2)),);
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).utf_codepoint(),
            None,
        );
    }

    #[test]
    fn custom_function_function_id_projection_is_typed() {
        let return_type = super::super::CustomTypeId::new(0);
        let function = CustomFunctionFunctionId::new(
            2,
            super::super::CustomFunctionType::from_shapes(
                super::super::FunctionType::new(
                    Vec::new(),
                    super::super::ValueType::Custom(return_type),
                ),
                Vec::new(),
                super::super::CustomValueShape::new(
                    return_type,
                    super::super::CustomValueShapeId::new(0),
                ),
            ),
        );
        let id = FunctionFunctionId::Custom(function.clone());

        assert_eq!(id.custom(), Some(function));
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).custom(),
            None,
        );
    }

    #[test]
    fn bit_array_list_function_function_id_preserves_exact_return_type() {
        let plan = execution_plan("pub fn main() -> fn() -> List(BitArray) { fn() { [] } }");
        let list_type = plan.bit_array_list_function_id(0).type_id();
        let return_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::List(list_type.list_type()),
        );
        let id = ListFunctionFunctionId::BitArray {
            id: super::BitArrayListFunctionFunctionId(0),
            type_: return_type.clone(),
            list_type,
        };

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(id.clone()),
                return_type: return_type.clone(),
            },
        );

        assert_eq!(
            plan.function_type(id.type_()),
            crate::plan::FunctionType::new(
                Vec::new(),
                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::BitArray)),
            ),
        );
    }

    #[test]
    fn utf_codepoint_list_function_function_id_preserves_exact_return_type() {
        let plan = execution_plan("pub fn main() -> fn() -> List(UtfCodepoint) { fn() { [] } }");
        let list_type = plan.utf_codepoint_list_function_id(0).type_id();
        let return_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::List(list_type.list_type()),
        );
        let id = ListFunctionFunctionId::UtfCodepoint {
            id: super::UtfCodepointListFunctionFunctionId(0),
            type_: return_type.clone(),
            list_type,
        };

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(id.clone()),
                return_type: return_type.clone(),
            },
        );
        assert_eq!(
            plan.function_type(id.type_()),
            crate::plan::FunctionType::new(
                Vec::new(),
                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::UtfCodepoint)),
            ),
        );
    }

    #[test]
    fn custom_list_function_function_id_preserves_exact_return_type() {
        let plan = execution_plan(
            "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> List(Boxed) { fn() { [] } }",
        );
        let list_type = plan.custom_list_function_id(0).type_id();
        let return_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::List(list_type.list_type()),
        );
        let id = ListFunctionFunctionId::Custom {
            id: super::CustomListFunctionFunctionId(0),
            type_: return_type.clone(),
            list_type,
        };

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(id.clone()),
                return_type: return_type.clone(),
            },
        );
        assert_eq!(
            plan.function_type(id.type_()),
            crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Custom(custom_type()))),
            ),
        );
    }

    #[test]
    fn list_function_function_ids_preserve_every_exact_return_type() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
fn customs() -> List(Boxed) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }

pub fn main() {
  let _ = #(
    ints,
    strings,
    bit_arrays,
    utf_codepoints,
    customs,
    floats,
    bools,
    nils,
    tuples,
    lists,
    functions,
  )
  Nil
}
"#,
        );
        let type_ = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Nil,
        );
        let ids = [
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.int_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.string_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::BitArray {
                id: super::BitArrayListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.bit_array_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::UtfCodepoint {
                id: super::UtfCodepointListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Custom {
                id: super::CustomListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.custom_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.float_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.bool_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.nil_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.tuple_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.list_list_function_id(0).type_id(),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(0),
                type_: type_.clone(),
                list_type: plan.function_list_function_id(0).type_id(),
            },
        ];

        assert_eq!(
            ids.map(|id| id.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );

        let locals = [
            ListLocal::Int {
                local: super::IntListLocalId(0),
                type_id: plan.int_list_function_id(0).type_id(),
            },
            ListLocal::String {
                local: super::StringListLocalId(0),
                type_id: plan.string_list_function_id(0).type_id(),
            },
            ListLocal::BitArray {
                local: super::BitArrayListLocalId(0),
                type_id: plan.bit_array_list_function_id(0).type_id(),
            },
            ListLocal::UtfCodepoint {
                local: super::UtfCodepointListLocalId(0),
                type_id: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListLocal::Custom {
                local: super::CustomListLocalId(0),
                type_id: plan.custom_list_function_id(0).type_id(),
            },
            ListLocal::Float {
                local: super::FloatListLocalId(0),
                type_id: plan.float_list_function_id(0).type_id(),
            },
            ListLocal::Bool {
                local: super::BoolListLocalId(0),
                type_id: plan.bool_list_function_id(0).type_id(),
            },
            ListLocal::Nil {
                local: super::NilListLocalId(0),
                type_id: plan.nil_list_function_id(0).type_id(),
            },
            ListLocal::Tuple {
                local: super::TupleListLocalId(0),
                type_id: plan.tuple_list_function_id(0).type_id(),
            },
            ListLocal::List {
                local: super::ListListLocalId(0),
                type_id: plan.list_list_function_id(0).type_id(),
            },
            ListLocal::Function {
                local: super::FunctionListLocalId(0),
                type_id: plan.function_list_function_id(0).type_id(),
            },
        ];
        let function_locals = [
            ListFunctionLocal::Int {
                local: super::IntListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.int_list_function_id(0).type_id(),
            },
            ListFunctionLocal::String {
                local: super::StringListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.string_list_function_id(0).type_id(),
            },
            ListFunctionLocal::BitArray {
                local: super::BitArrayListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bit_array_list_function_id(0).type_id(),
            },
            ListFunctionLocal::UtfCodepoint {
                local: super::UtfCodepointListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Custom {
                local: super::CustomListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.custom_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Float {
                local: super::FloatListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.float_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Bool {
                local: super::BoolListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bool_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Nil {
                local: super::NilListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.nil_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Tuple {
                local: super::TupleListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.tuple_list_function_id(0).type_id(),
            },
            ListFunctionLocal::List {
                local: super::ListListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.list_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Function {
                local: super::FunctionListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.function_list_function_id(0).type_id(),
            },
        ];

        assert_eq!(
            locals.map(|local| local.list_type()),
            function_locals.clone().map(|local| local.list_type()),
        );
        assert_eq!(
            function_locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
