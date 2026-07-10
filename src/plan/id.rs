#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalId {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

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
    Int(IntListLocalId),
    String(StringListLocalId),
    Float(FloatListLocalId),
    Bool(BoolListLocalId),
    Nil(NilListLocalId),
    Tuple {
        local: TupleListLocalId,
        item_type: Vec<crate::plan::ValueType>,
    },
    List {
        local: ListListLocalId,
        item_type: Box<crate::plan::ValueType>,
    },
    Function {
        local: FunctionListLocalId,
        item_type: crate::plan::FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFunctionLocalId(pub(crate) usize);

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
        type_: crate::plan::FunctionType,
    },
    String {
        local: StringListFunctionLocalId,
        type_: crate::plan::FunctionType,
    },
    Float {
        local: FloatListFunctionLocalId,
        type_: crate::plan::FunctionType,
    },
    Bool {
        local: BoolListFunctionLocalId,
        type_: crate::plan::FunctionType,
    },
    Nil {
        local: NilListFunctionLocalId,
        type_: crate::plan::FunctionType,
    },
    Tuple {
        local: TupleListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: Vec<crate::plan::ValueType>,
    },
    List {
        local: ListListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: Box<crate::plan::ValueType>,
    },
    Function {
        local: FunctionListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: Box<crate::plan::FunctionType>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple {
        id: TupleFunctionId,
        return_type: Vec<crate::plan::ValueType>,
    },
    List(ListFunctionId),
    Function {
        id: FunctionFunctionId,
        return_type: crate::plan::FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFunctionId {
    Int(IntListFunctionId),
    String(StringListFunctionId),
    Float(FloatListFunctionId),
    Bool(BoolListFunctionId),
    Nil(NilListFunctionId),
    Tuple {
        id: TupleListFunctionId,
        item_type: Vec<crate::plan::ValueType>,
    },
    List {
        id: ListListFunctionId,
        item_type: Box<crate::plan::ValueType>,
    },
    Function {
        id: FunctionListFunctionId,
        item_type: crate::plan::FunctionType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionFunctionId {
    Int(IntFunctionFunctionId),
    Float(FloatFunctionFunctionId),
    String(StringFunctionFunctionId),
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
        type_: crate::plan::FunctionType,
    },
    String {
        id: StringListFunctionFunctionId,
        type_: crate::plan::FunctionType,
    },
    Float {
        id: FloatListFunctionFunctionId,
        type_: crate::plan::FunctionType,
    },
    Bool {
        id: BoolListFunctionFunctionId,
        type_: crate::plan::FunctionType,
    },
    Nil {
        id: NilListFunctionFunctionId,
        type_: crate::plan::FunctionType,
    },
    Tuple {
        id: TupleListFunctionFunctionId,
        type_: crate::plan::FunctionType,
        item_type: Vec<crate::plan::ValueType>,
    },
    List {
        id: ListListFunctionFunctionId,
        type_: crate::plan::FunctionType,
        item_type: Box<crate::plan::ValueType>,
    },
    Function {
        id: FunctionListFunctionFunctionId,
        type_: crate::plan::FunctionType,
        item_type: Box<crate::plan::FunctionType>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionFunctionFunctionId(pub(crate) usize);

impl LocalId {
    pub(crate) fn value_type(self) -> crate::plan::ValueType {
        match self {
            Self::Int(_) => crate::plan::ValueType::Int,
            Self::Float(_) => crate::plan::ValueType::Float,
            Self::String(_) => crate::plan::ValueType::String,
            Self::Bool(_) => crate::plan::ValueType::Bool,
            Self::Nil(_) => crate::plan::ValueType::Nil,
        }
    }
}

impl FunctionId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl FunctionFunctionId {
    pub(crate) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
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
            Self::Function(id) => Some(*id),
            _ => None,
        }
    }
}

impl ListFunctionLocal {
    pub(crate) fn from_item_type(
        index: usize,
        type_: crate::plan::FunctionType,
        item_type: crate::plan::ValueType,
    ) -> Self {
        match item_type {
            crate::plan::ValueType::Int => Self::int(IntListFunctionLocalId(index), type_),
            crate::plan::ValueType::String => Self::string(StringListFunctionLocalId(index), type_),
            crate::plan::ValueType::Float => Self::float(FloatListFunctionLocalId(index), type_),
            crate::plan::ValueType::Bool => Self::bool(BoolListFunctionLocalId(index), type_),
            crate::plan::ValueType::Nil => Self::nil(NilListFunctionLocalId(index), type_),
            crate::plan::ValueType::Tuple(item_type) => {
                Self::tuple(TupleListFunctionLocalId(index), type_, item_type)
            }
            crate::plan::ValueType::List(item_type) => {
                Self::list(ListListFunctionLocalId(index), type_, *item_type)
            }
            crate::plan::ValueType::Function(item_type) => {
                Self::function(FunctionListFunctionLocalId(index), type_, *item_type)
            }
        }
    }

    pub(crate) fn int(local: IntListFunctionLocalId, type_: crate::plan::FunctionType) -> Self {
        Self::Int { local, type_ }
    }

    pub(crate) fn string(
        local: StringListFunctionLocalId,
        type_: crate::plan::FunctionType,
    ) -> Self {
        Self::String { local, type_ }
    }

    pub(crate) fn float(local: FloatListFunctionLocalId, type_: crate::plan::FunctionType) -> Self {
        Self::Float { local, type_ }
    }

    pub(crate) fn bool(local: BoolListFunctionLocalId, type_: crate::plan::FunctionType) -> Self {
        Self::Bool { local, type_ }
    }

    pub(crate) fn nil(local: NilListFunctionLocalId, type_: crate::plan::FunctionType) -> Self {
        Self::Nil { local, type_ }
    }

    pub(crate) fn tuple(
        local: TupleListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: Vec<crate::plan::ValueType>,
    ) -> Self {
        Self::Tuple {
            local,
            type_,
            item_type,
        }
    }

    pub(crate) fn list(
        local: ListListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: crate::plan::ValueType,
    ) -> Self {
        Self::List {
            local,
            type_,
            item_type: Box::new(item_type),
        }
    }

    pub(crate) fn function(
        local: FunctionListFunctionLocalId,
        type_: crate::plan::FunctionType,
        item_type: crate::plan::FunctionType,
    ) -> Self {
        Self::Function {
            local,
            type_,
            item_type: Box::new(item_type),
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::FunctionType {
        match self {
            Self::Int { type_, .. }
            | Self::String { type_, .. }
            | Self::Float { type_, .. }
            | Self::Bool { type_, .. }
            | Self::Nil { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::List { type_, .. }
            | Self::Function { type_, .. } => type_,
        }
    }

    pub(crate) fn value_type(&self) -> crate::plan::ValueType {
        crate::plan::ValueType::Function(Box::new(self.type_().clone()))
    }

    pub(crate) fn item_type(&self) -> crate::plan::ValueType {
        match self {
            Self::Int { .. } => crate::plan::ValueType::Int,
            Self::String { .. } => crate::plan::ValueType::String,
            Self::Float { .. } => crate::plan::ValueType::Float,
            Self::Bool { .. } => crate::plan::ValueType::Bool,
            Self::Nil { .. } => crate::plan::ValueType::Nil,
            Self::Tuple { item_type, .. } => crate::plan::ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => crate::plan::ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => crate::plan::ValueType::Function(item_type.clone()),
        }
    }

    pub(crate) fn index(&self) -> usize {
        match self {
            Self::Int { local, .. } => local.0,
            Self::String { local, .. } => local.0,
            Self::Float { local, .. } => local.0,
            Self::Bool { local, .. } => local.0,
            Self::Nil { local, .. } => local.0,
            Self::Tuple { local, .. } => local.0,
            Self::List { local, .. } => local.0,
            Self::Function { local, .. } => local.0,
        }
    }
}

impl ListFunctionId {
    #[cfg(test)]
    pub(crate) fn from_item_type(index: usize, item_type: crate::plan::ValueType) -> Self {
        match item_type {
            crate::plan::ValueType::Int => Self::Int(IntListFunctionId(index)),
            crate::plan::ValueType::String => Self::String(StringListFunctionId(index)),
            crate::plan::ValueType::Float => Self::Float(FloatListFunctionId(index)),
            crate::plan::ValueType::Bool => Self::Bool(BoolListFunctionId(index)),
            crate::plan::ValueType::Nil => Self::Nil(NilListFunctionId(index)),
            crate::plan::ValueType::Tuple(item_type) => Self::Tuple {
                id: TupleListFunctionId(index),
                item_type,
            },
            crate::plan::ValueType::List(item_type) => Self::List {
                id: ListListFunctionId(index),
                item_type,
            },
            crate::plan::ValueType::Function(item_type) => Self::Function {
                id: FunctionListFunctionId(index),
                item_type: *item_type,
            },
        }
    }

    pub(crate) fn item_type(&self) -> crate::plan::ValueType {
        match self {
            Self::Int(_) => crate::plan::ValueType::Int,
            Self::String(_) => crate::plan::ValueType::String,
            Self::Float(_) => crate::plan::ValueType::Float,
            Self::Bool(_) => crate::plan::ValueType::Bool,
            Self::Nil(_) => crate::plan::ValueType::Nil,
            Self::Tuple { item_type, .. } => crate::plan::ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => crate::plan::ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => {
                crate::plan::ValueType::Function(Box::new(item_type.clone()))
            }
        }
    }
}

impl ListFunctionFunctionId {
    pub(crate) fn from_item_type(
        index: usize,
        type_: crate::plan::FunctionType,
        item_type: crate::plan::ValueType,
    ) -> Self {
        match item_type {
            crate::plan::ValueType::Int => Self::Int {
                id: IntListFunctionFunctionId(index),
                type_,
            },
            crate::plan::ValueType::String => Self::String {
                id: StringListFunctionFunctionId(index),
                type_,
            },
            crate::plan::ValueType::Float => Self::Float {
                id: FloatListFunctionFunctionId(index),
                type_,
            },
            crate::plan::ValueType::Bool => Self::Bool {
                id: BoolListFunctionFunctionId(index),
                type_,
            },
            crate::plan::ValueType::Nil => Self::Nil {
                id: NilListFunctionFunctionId(index),
                type_,
            },
            crate::plan::ValueType::Tuple(item_type) => Self::Tuple {
                id: TupleListFunctionFunctionId(index),
                type_,
                item_type,
            },
            crate::plan::ValueType::List(item_type) => Self::List {
                id: ListListFunctionFunctionId(index),
                type_,
                item_type,
            },
            crate::plan::ValueType::Function(item_type) => Self::Function {
                id: FunctionListFunctionFunctionId(index),
                type_,
                item_type,
            },
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::FunctionType {
        match self {
            Self::Int { type_, .. }
            | Self::String { type_, .. }
            | Self::Float { type_, .. }
            | Self::Bool { type_, .. }
            | Self::Nil { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::List { type_, .. }
            | Self::Function { type_, .. } => type_,
        }
    }

    pub(crate) fn item_type(&self) -> crate::plan::ValueType {
        match self {
            Self::Int { .. } => crate::plan::ValueType::Int,
            Self::String { .. } => crate::plan::ValueType::String,
            Self::Float { .. } => crate::plan::ValueType::Float,
            Self::Bool { .. } => crate::plan::ValueType::Bool,
            Self::Nil { .. } => crate::plan::ValueType::Nil,
            Self::Tuple { item_type, .. } => crate::plan::ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => crate::plan::ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => crate::plan::ValueType::Function(item_type.clone()),
        }
    }
}

impl ListLocal {
    pub(crate) fn int(local: IntListLocalId) -> Self {
        Self::Int(local)
    }

    pub(crate) fn string(local: StringListLocalId) -> Self {
        Self::String(local)
    }

    pub(crate) fn float(local: FloatListLocalId) -> Self {
        Self::Float(local)
    }

    pub(crate) fn bool(local: BoolListLocalId) -> Self {
        Self::Bool(local)
    }

    pub(crate) fn nil(local: NilListLocalId) -> Self {
        Self::Nil(local)
    }

    pub(crate) fn tuple(local: TupleListLocalId, item_type: Vec<crate::plan::ValueType>) -> Self {
        Self::Tuple { local, item_type }
    }

    pub(crate) fn list(local: ListListLocalId, item_type: crate::plan::ValueType) -> Self {
        Self::List {
            local,
            item_type: Box::new(item_type),
        }
    }

    pub(crate) fn function(
        local: FunctionListLocalId,
        item_type: crate::plan::FunctionType,
    ) -> Self {
        Self::Function { local, item_type }
    }

    pub(crate) fn item_type(&self) -> crate::plan::ValueType {
        match self {
            Self::Int(_) => crate::plan::ValueType::Int,
            Self::String(_) => crate::plan::ValueType::String,
            Self::Float(_) => crate::plan::ValueType::Float,
            Self::Bool(_) => crate::plan::ValueType::Bool,
            Self::Nil(_) => crate::plan::ValueType::Nil,
            Self::Tuple { item_type, .. } => crate::plan::ValueType::Tuple(item_type.clone()),
            Self::List { item_type, .. } => crate::plan::ValueType::List(item_type.clone()),
            Self::Function { item_type, .. } => {
                crate::plan::ValueType::Function(Box::new(item_type.clone()))
            }
        }
    }

    pub(crate) fn value_type(&self) -> crate::plan::ValueType {
        crate::plan::ValueType::List(Box::new(self.item_type()))
    }

    pub(crate) fn family_name(&self) -> &'static str {
        match self {
            Self::Int(_) => "int",
            Self::String(_) => "string",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
            Self::Nil(_) => "nil",
            Self::Tuple { .. } => "tuple",
            Self::List { .. } => "list",
            Self::Function { .. } => "function",
        }
    }

    pub(crate) fn index(&self) -> usize {
        match self {
            Self::Int(local) => local.0,
            Self::String(local) => local.0,
            Self::Float(local) => local.0,
            Self::Bool(local) => local.0,
            Self::Nil(local) => local.0,
            Self::Tuple { local, .. } => local.0,
            Self::List { local, .. } => local.0,
            Self::Function { local, .. } => local.0,
        }
    }
}

impl std::fmt::Display for FunctionReturnFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => f.write_str("Int"),
            Self::Float => f.write_str("Float"),
            Self::String => f.write_str("String"),
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
        BoolFunctionFunctionId, BoolFunctionLocalId, BoolListFunctionFunctionId,
        BoolListFunctionLocalId, FloatFunctionFunctionId, FloatFunctionLocalId,
        FloatListFunctionFunctionId, FloatListFunctionLocalId, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionId, FunctionListFunctionFunctionId,
        FunctionListFunctionLocalId, FunctionListLocalId, IntFunctionFunctionId,
        IntFunctionLocalId, IntListFunctionFunctionId, IntListFunctionLocalId, IntListLocalId,
        ListFunctionFunctionId, ListFunctionLocal, ListListFunctionFunctionId,
        ListListFunctionLocalId, ListListLocalId, ListLocal, NilFunctionFunctionId,
        NilFunctionLocalId, NilListFunctionFunctionId, NilListFunctionLocalId, NilListLocalId,
        StringFunctionFunctionId, StringFunctionLocalId, StringListFunctionFunctionId,
        StringListFunctionLocalId, StringListLocalId, TupleFunctionFunctionId,
        TupleFunctionLocalId, TupleListFunctionFunctionId, TupleListFunctionLocalId,
        TupleListLocalId,
    };
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn function_id_index() {
        assert_eq!(FunctionId::new(5).index(), 5);
    }

    #[test]
    fn function_local_id_debug_surface() {
        assert_eq!(
            format!("{:?}", IntFunctionLocalId(3)),
            "IntFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", FloatFunctionLocalId(3)),
            "FloatFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", StringFunctionLocalId(3)),
            "StringFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", BoolFunctionLocalId(3)),
            "BoolFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", NilFunctionLocalId(3)),
            "NilFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", TupleFunctionLocalId(3)),
            "TupleFunctionLocalId(3)"
        );
        assert_eq!(
            format!(
                "{:?}",
                crate::plan::ListFunctionLocal::from_item_type(
                    3,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int,
                )
            ),
            "Int { local: IntListFunctionLocalId(3), type_: FunctionType { arguments: [], return_: List(Int) } }"
        );
        assert_eq!(
            format!("{:?}", FunctionFunctionLocalId(3)),
            "FunctionFunctionLocalId(3)"
        );
    }

    #[test]
    fn function_function_id_typed_projection() {
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).int(),
            Some(IntFunctionFunctionId(1)),
        );
        assert_eq!(
            FunctionFunctionId::Float(FloatFunctionFunctionId(6)).float(),
            Some(FloatFunctionFunctionId(6)),
        );
        assert_eq!(
            FunctionFunctionId::String(StringFunctionFunctionId(2)).string(),
            Some(StringFunctionFunctionId(2)),
        );
        assert_eq!(
            FunctionFunctionId::Bool(BoolFunctionFunctionId(3)).bool(),
            Some(BoolFunctionFunctionId(3)),
        );
        assert_eq!(
            FunctionFunctionId::Nil(NilFunctionFunctionId(4)).nil(),
            Some(NilFunctionFunctionId(4)),
        );
        assert_eq!(
            FunctionFunctionId::Tuple(TupleFunctionFunctionId(5)).tuple(),
            Some(TupleFunctionFunctionId(5)),
        );
        assert_eq!(
            FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                6,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                ),
                crate::plan::ValueType::Int
            ))
            .list(),
            Some(ListFunctionFunctionId::from_item_type(
                6,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                ),
                crate::plan::ValueType::Int
            )),
        );
        assert_eq!(
            FunctionFunctionId::Function(FunctionFunctionFunctionId(6)).function(),
            Some(FunctionFunctionFunctionId(6)),
        );
    }

    #[test]
    fn function_function_id_typed_projection_mismatch() {
        assert_eq!(
            FunctionFunctionId::String(StringFunctionFunctionId(1)).int(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).string(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).float(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).bool(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).nil(),
            None
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).tuple(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).list(),
            None,
        );
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).function(),
            None,
        );
    }

    #[test]
    fn function_function_id_family() {
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Int,
        );
        assert_eq!(
            FunctionFunctionId::Float(FloatFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Float,
        );
        assert_eq!(
            FunctionFunctionId::String(StringFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::String,
        );
        assert_eq!(
            FunctionFunctionId::Bool(BoolFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Bool,
        );
        assert_eq!(
            FunctionFunctionId::Nil(NilFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Nil,
        );
        assert_eq!(
            FunctionFunctionId::Tuple(TupleFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Tuple,
        );
        assert_eq!(
            FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                1,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                ),
                crate::plan::ValueType::Int,
            ))
            .family(),
            super::FunctionReturnFamily::List,
        );
        assert_eq!(
            FunctionFunctionId::Function(FunctionFunctionFunctionId(1)).family(),
            super::FunctionReturnFamily::Function,
        );
    }

    #[test]
    fn function_return_family_display() {
        assert_eq!(super::FunctionReturnFamily::Int.to_string(), "Int");
        assert_eq!(super::FunctionReturnFamily::Float.to_string(), "Float");
        assert_eq!(super::FunctionReturnFamily::String.to_string(), "String");
        assert_eq!(super::FunctionReturnFamily::Bool.to_string(), "Bool");
        assert_eq!(super::FunctionReturnFamily::Nil.to_string(), "Nil");
        assert_eq!(super::FunctionReturnFamily::Tuple.to_string(), "Tuple");
        assert_eq!(super::FunctionReturnFamily::List.to_string(), "List");
        assert_eq!(
            super::FunctionReturnFamily::Function.to_string(),
            "Function"
        );
    }

    #[test]
    fn list_locals_preserve_item_family() {
        let item_types = list_item_types();
        let locals = [
            ListLocal::int(IntListLocalId(3)),
            ListLocal::string(StringListLocalId(3)),
            ListLocal::float(super::FloatListLocalId(3)),
            ListLocal::bool(super::BoolListLocalId(3)),
            ListLocal::nil(NilListLocalId(3)),
            ListLocal::tuple(TupleListLocalId(3), vec![ValueType::Int, ValueType::String]),
            ListLocal::list(ListListLocalId(3), ValueType::Int),
            ListLocal::function(
                FunctionListLocalId(3),
                FunctionType::new(vec![ValueType::Int], ValueType::String),
            ),
        ];

        assert_eq!(
            locals.iter().map(ListLocal::item_type).collect::<Vec<_>>(),
            item_types,
        );
        assert_eq!(
            locals.iter().map(ListLocal::index).collect::<Vec<_>>(),
            vec![3; 8],
        );
    }

    #[test]
    fn list_function_locals_preserve_item_family_and_type() {
        let cases = list_function_type_cases();
        let item_types = list_item_types();
        let locals = [
            ListFunctionLocal::from_item_type(3, cases[0].clone(), item_types[0].clone()),
            ListFunctionLocal::from_item_type(3, cases[1].clone(), item_types[1].clone()),
            ListFunctionLocal::from_item_type(3, cases[2].clone(), item_types[2].clone()),
            ListFunctionLocal::from_item_type(3, cases[3].clone(), item_types[3].clone()),
            ListFunctionLocal::from_item_type(3, cases[4].clone(), item_types[4].clone()),
            ListFunctionLocal::from_item_type(3, cases[5].clone(), item_types[5].clone()),
            ListFunctionLocal::from_item_type(3, cases[6].clone(), item_types[6].clone()),
            ListFunctionLocal::from_item_type(3, cases[7].clone(), item_types[7].clone()),
        ];

        assert_eq!(
            locals,
            [
                ListFunctionLocal::int(IntListFunctionLocalId(3), cases[0].clone()),
                ListFunctionLocal::string(StringListFunctionLocalId(3), cases[1].clone()),
                ListFunctionLocal::float(FloatListFunctionLocalId(3), cases[2].clone()),
                ListFunctionLocal::bool(BoolListFunctionLocalId(3), cases[3].clone()),
                ListFunctionLocal::nil(NilListFunctionLocalId(3), cases[4].clone()),
                ListFunctionLocal::tuple(
                    TupleListFunctionLocalId(3),
                    cases[5].clone(),
                    vec![ValueType::Int, ValueType::String],
                ),
                ListFunctionLocal::list(
                    ListListFunctionLocalId(3),
                    cases[6].clone(),
                    ValueType::Int,
                ),
                ListFunctionLocal::function(
                    FunctionListFunctionLocalId(3),
                    cases[7].clone(),
                    FunctionType::new(vec![ValueType::Int], ValueType::String),
                ),
            ],
        );
        assert_eq!(
            locals
                .iter()
                .map(ListFunctionLocal::value_type)
                .collect::<Vec<_>>(),
            cases
                .iter()
                .cloned()
                .map(|type_| ValueType::Function(Box::new(type_)))
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            locals
                .iter()
                .map(ListFunctionLocal::item_type)
                .collect::<Vec<_>>(),
            item_types,
        );
        assert_eq!(
            locals
                .iter()
                .map(ListFunctionLocal::index)
                .collect::<Vec<_>>(),
            vec![3; 8],
        );
    }

    #[test]
    fn list_function_function_ids_preserve_item_family_and_type() {
        let cases = list_function_type_cases();
        let item_types = list_item_types();
        let ids = [
            ListFunctionFunctionId::from_item_type(7, cases[0].clone(), item_types[0].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[1].clone(), item_types[1].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[2].clone(), item_types[2].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[3].clone(), item_types[3].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[4].clone(), item_types[4].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[5].clone(), item_types[5].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[6].clone(), item_types[6].clone()),
            ListFunctionFunctionId::from_item_type(7, cases[7].clone(), item_types[7].clone()),
        ];

        assert_eq!(
            ids,
            [
                ListFunctionFunctionId::Int {
                    id: IntListFunctionFunctionId(7),
                    type_: cases[0].clone(),
                },
                ListFunctionFunctionId::String {
                    id: StringListFunctionFunctionId(7),
                    type_: cases[1].clone(),
                },
                ListFunctionFunctionId::Float {
                    id: FloatListFunctionFunctionId(7),
                    type_: cases[2].clone(),
                },
                ListFunctionFunctionId::Bool {
                    id: BoolListFunctionFunctionId(7),
                    type_: cases[3].clone(),
                },
                ListFunctionFunctionId::Nil {
                    id: NilListFunctionFunctionId(7),
                    type_: cases[4].clone(),
                },
                ListFunctionFunctionId::Tuple {
                    id: TupleListFunctionFunctionId(7),
                    type_: cases[5].clone(),
                    item_type: vec![ValueType::Int, ValueType::String],
                },
                ListFunctionFunctionId::List {
                    id: ListListFunctionFunctionId(7),
                    type_: cases[6].clone(),
                    item_type: Box::new(ValueType::Int),
                },
                ListFunctionFunctionId::Function {
                    id: FunctionListFunctionFunctionId(7),
                    type_: cases[7].clone(),
                    item_type: Box::new(FunctionType::new(vec![ValueType::Int], ValueType::String)),
                },
            ],
        );
        assert_eq!(
            ids.iter().map(|id| id.type_().clone()).collect::<Vec<_>>(),
            cases,
        );
        assert_eq!(
            ids.iter().map(|id| id.item_type()).collect::<Vec<_>>(),
            item_types,
        );
    }

    fn list_function_type_cases() -> [FunctionType; 8] {
        list_item_types().map(list_function_type)
    }

    fn list_item_types() -> [ValueType; 8] {
        [
            ValueType::Int,
            ValueType::String,
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::String,
            ))),
        ]
    }

    fn list_function_type(item_type: ValueType) -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::List(Box::new(item_type)))
    }
}
