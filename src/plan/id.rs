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
pub struct ListFunctionLocalId(pub(crate) usize);

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
    List {
        id: ListFunctionId,
        return_type: Box<crate::plan::ValueType>,
    },
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
pub struct ListFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct ListFunctionFunctionId(pub(crate) usize);

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
    pub(crate) fn family(self) -> FunctionReturnFamily {
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

    pub(crate) fn int(self) -> Option<IntFunctionFunctionId> {
        match self {
            Self::Int(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn string(self) -> Option<StringFunctionFunctionId> {
        match self {
            Self::String(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn float(self) -> Option<FloatFunctionFunctionId> {
        match self {
            Self::Float(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn bool(self) -> Option<BoolFunctionFunctionId> {
        match self {
            Self::Bool(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn nil(self) -> Option<NilFunctionFunctionId> {
        match self {
            Self::Nil(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn tuple(self) -> Option<TupleFunctionFunctionId> {
        match self {
            Self::Tuple(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn list(self) -> Option<ListFunctionFunctionId> {
        match self {
            Self::List(id) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn function(self) -> Option<FunctionFunctionFunctionId> {
        match self {
            Self::Function(id) => Some(id),
            _ => None,
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
        BoolFunctionFunctionId, BoolFunctionLocalId, FloatFunctionFunctionId, FloatFunctionLocalId,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionId,
        IntFunctionFunctionId, IntFunctionLocalId, ListFunctionFunctionId, ListFunctionLocalId,
        NilFunctionFunctionId, NilFunctionLocalId, StringFunctionFunctionId, StringFunctionLocalId,
        TupleFunctionFunctionId, TupleFunctionLocalId,
    };

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
            format!("{:?}", ListFunctionLocalId(3)),
            "ListFunctionLocalId(3)"
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
            FunctionFunctionId::List(ListFunctionFunctionId(6)).list(),
            Some(ListFunctionFunctionId(6)),
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
            FunctionFunctionId::List(ListFunctionFunctionId(1)).family(),
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
}
