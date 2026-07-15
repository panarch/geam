use super::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
    FunctionListTypeId, FunctionType, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
    StringListTypeId, TupleListTypeId, UtfCodepointListTypeId, ValueType,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListLocalId(pub(crate) usize);

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
pub struct FunctionListLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListLocal {
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
list_function_id!(CustomListFunctionId, CustomListTypeId);
list_function_id!(FloatListFunctionId, FloatListTypeId);
list_function_id!(BoolListFunctionId, BoolListTypeId);
list_function_id!(NilListFunctionId, NilListTypeId);
list_function_id!(TupleListFunctionId, TupleListTypeId);
list_function_id!(ListListFunctionId, ListListTypeId);
list_function_id!(FunctionListFunctionId, FunctionListTypeId);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFunctionId {
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

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &super::CustomFunctionType {
        &self.type_
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

    pub(crate) fn type_id(self) -> CustomTypeId {
        self.shape.type_id()
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

    #[cfg(test)]
    pub(crate) fn return_type(self) -> CustomTypeId {
        self.return_shape.type_id()
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

    pub(crate) fn type_(&self) -> &super::CustomFunctionType {
        &self.type_
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

    pub(crate) fn type_(&self) -> &super::FunctionFunctionType {
        &self.type_
    }
}

impl FunctionFunctionId {
    pub(crate) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Custom(_) => FunctionReturnFamily::Custom,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::List(_) => FunctionReturnFamily::List,
            Self::Function(_) => FunctionReturnFamily::Function,
        }
    }

    pub(crate) fn int(&self) -> Option<IntFunctionFunctionId> {
        match self {
            Self::Int(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn string(&self) -> Option<StringFunctionFunctionId> {
        match self {
            Self::String(id) => Some(*id),
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

    pub(crate) fn float(&self) -> Option<FloatFunctionFunctionId> {
        match self {
            Self::Float(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn bool(&self) -> Option<BoolFunctionFunctionId> {
        match self {
            Self::Bool(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn nil(&self) -> Option<NilFunctionFunctionId> {
        match self {
            Self::Nil(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn tuple(&self) -> Option<TupleFunctionFunctionId> {
        match self {
            Self::Tuple(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn list(&self) -> Option<ListFunctionFunctionId> {
        match self {
            Self::List(id) => Some(id.clone()),
            _ => None,
        }
    }

    pub(crate) fn function(&self) -> Option<FunctionFunctionFunctionId> {
        match self {
            Self::Function(id) => Some(id.clone()),
            _ => None,
        }
    }
}

impl ListFunctionLocal {
    #[cfg(test)]
    pub(crate) fn type_(&self) -> &FunctionType {
        match self {
            Self::Int { type_, .. }
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

impl ListFunctionId {
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int(id) => id.type_id().list_type(),
            Self::String(id) => id.type_id().list_type(),
            Self::BitArray(id) => id.type_id().list_type(),
            Self::UtfCodepoint(id) => id.type_id().list_type(),
            Self::Custom(id) => id.type_id().list_type(),
            Self::Float(id) => id.type_id().list_type(),
            Self::Bool(id) => id.type_id().list_type(),
            Self::Nil(id) => id.type_id().list_type(),
            Self::Tuple(id) => id.type_id().list_type(),
            Self::List(id) => id.type_id().list_type(),
            Self::Function(id) => id.type_id().list_type(),
        }
    }
}

impl ListFunctionFunctionId {
    pub(crate) fn type_(&self) -> &FunctionType {
        match self {
            Self::Int { type_, .. }
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
        BitArrayFunctionFunctionId, BitArrayListLocalId, BoolListLocalId, CustomFunctionFunctionId,
        CustomListLocalId, FloatListLocalId, FunctionFunctionId, FunctionListLocalId,
        FunctionReturnFamily, IntFunctionFunctionId, IntListLocalId, ListFunctionFunctionId,
        ListListLocalId, ListLocal, NilListLocalId, RuntimeFunctionId, StringListLocalId,
        TupleListLocalId, UtfCodepointFunctionFunctionId, UtfCodepointListLocalId,
    };
    use crate::plan::{CustomType, CustomTypeName, ValueType};

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn list_locals_preserve_every_lowered_list_type() {
        let plan = execution_plan(
            r#"
type Boxed { Boxed(Int) }

fn int_values(values: List(Int)) { values }
fn string_values(values: List(String)) { values }
fn bit_array_values(values: List(BitArray)) { values }
fn utf_codepoint_values(values: List(UtfCodepoint)) { values }
fn custom_values(values: List(Boxed)) { values }
fn float_values(values: List(Float)) { values }
fn bool_values(values: List(Bool)) { values }
fn nil_values(values: List(Nil)) { values }
fn tuple_values(values: List(#(Int))) { values }
fn list_values(values: List(List(Int))) { values }
fn function_values(values: List(fn() -> Int)) { values }
fn bit_array_function_value(function: fn() -> List(BitArray)) { function() }
fn utf_codepoint_function_value(function: fn() -> List(UtfCodepoint)) { function() }
fn custom_function_value(function: fn() -> List(Boxed)) { function() }

pub fn main() { Nil }
"#,
        );
        let int = plan
            .int_list_function(plan.int_list_function_id(0))
            .frame_layout()
            .int_lists()[0];
        let string = plan
            .string_list_function(plan.string_list_function_id(0))
            .frame_layout()
            .string_lists()[0];
        let bit_array = plan
            .bit_array_list_function(plan.bit_array_list_function_id(0))
            .frame_layout()
            .bit_array_lists()[0];
        let utf_codepoint = plan
            .utf_codepoint_list_function(plan.utf_codepoint_list_function_id(0))
            .frame_layout()
            .utf_codepoint_lists()[0];
        let custom = plan
            .custom_list_function(plan.custom_list_function_id(0))
            .frame_layout()
            .custom_lists()[0];
        let float = plan
            .float_list_function(plan.float_list_function_id(0))
            .frame_layout()
            .float_lists()[0];
        let bool_ = plan
            .bool_list_function(plan.bool_list_function_id(0))
            .frame_layout()
            .bool_lists()[0];
        let nil = plan
            .nil_list_function(plan.nil_list_function_id(0))
            .frame_layout()
            .nil_lists()[0];
        let tuple = plan
            .tuple_list_function(plan.tuple_list_function_id(0))
            .frame_layout()
            .tuple_lists()[0];
        let list = plan
            .list_list_function(plan.list_list_function_id(0))
            .frame_layout()
            .list_lists()[0];
        let function = plan
            .function_list_function(plan.function_list_function_id(0))
            .frame_layout()
            .function_lists()[0];
        let locals = [
            ListLocal::Int {
                local: IntListLocalId(0),
                type_id: int,
            },
            ListLocal::String {
                local: StringListLocalId(0),
                type_id: string,
            },
            ListLocal::BitArray {
                local: BitArrayListLocalId(0),
                type_id: bit_array,
            },
            ListLocal::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                type_id: utf_codepoint,
            },
            ListLocal::Custom {
                local: CustomListLocalId(0),
                type_id: custom,
            },
            ListLocal::Float {
                local: FloatListLocalId(0),
                type_id: float,
            },
            ListLocal::Bool {
                local: BoolListLocalId(0),
                type_id: bool_,
            },
            ListLocal::Nil {
                local: NilListLocalId(0),
                type_id: nil,
            },
            ListLocal::Tuple {
                local: TupleListLocalId(0),
                type_id: tuple,
            },
            ListLocal::List {
                local: ListListLocalId(0),
                type_id: list,
            },
            ListLocal::Function {
                local: FunctionListLocalId(0),
                type_id: function,
            },
        ];

        assert_eq!(
            locals
                .iter()
                .map(|local| plan.list_value_type(local.list_type()))
                .collect::<Vec<_>>(),
            vec![
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::List(Box::new(ValueType::String)),
                ValueType::List(Box::new(ValueType::BitArray)),
                ValueType::List(Box::new(ValueType::UtfCodepoint)),
                ValueType::List(Box::new(ValueType::Custom(custom_type()))),
                ValueType::List(Box::new(ValueType::Float)),
                ValueType::List(Box::new(ValueType::Bool)),
                ValueType::List(Box::new(ValueType::Nil)),
                ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                ValueType::List(Box::new(ValueType::Function(Box::new(
                    crate::plan::FunctionType::new(Vec::new(), ValueType::Int),
                )))),
            ],
        );

        let bit_array_function_local = plan
            .bit_array_list_function(plan.bit_array_list_function_id(1))
            .frame_layout()
            .list_functions()[0]
            .clone();
        assert_eq!(
            plan.list_value_type(bit_array_function_local.list_type()),
            ValueType::List(Box::new(ValueType::BitArray)),
        );

        let utf_codepoint_function_local = plan
            .utf_codepoint_list_function(plan.utf_codepoint_list_function_id(1))
            .frame_layout()
            .list_functions()[0]
            .clone();
        assert_eq!(
            plan.list_value_type(utf_codepoint_function_local.list_type()),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
        );
        assert_eq!(
            plan.function_type(utf_codepoint_function_local.type_()),
            crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ),
        );

        let custom_function_local = plan
            .custom_list_function(plan.custom_list_function_id(1))
            .frame_layout()
            .list_functions()[0]
            .clone();
        assert_eq!(
            plan.list_value_type(custom_function_local.list_type()),
            ValueType::List(Box::new(ValueType::Custom(custom_type()))),
        );
        assert_eq!(
            plan.function_type(custom_function_local.type_()),
            crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Custom(custom_type()))),
            ),
        );
    }

    #[test]
    fn function_return_family_display_names_every_family() {
        assert_eq!(
            [
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

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
