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
}

impl ListFunctionId {
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
}

impl ListLocal {
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
        BoolFunctionFunctionId, BoolListFunctionFunctionId, BoolListFunctionId,
        BoolListFunctionLocalId, BoolListLocalId, FloatFunctionFunctionId,
        FloatListFunctionFunctionId, FloatListFunctionId, FloatListFunctionLocalId,
        FloatListLocalId, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionListFunctionFunctionId, FunctionListFunctionId, FunctionListFunctionLocalId,
        FunctionListLocalId, FunctionReturnFamily, IntFunctionFunctionId,
        IntListFunctionFunctionId, IntListFunctionId, IntListFunctionLocalId, IntListLocalId,
        ListFunctionFunctionId, ListFunctionId, ListFunctionLocal, ListListFunctionFunctionId,
        ListListFunctionId, ListListFunctionLocalId, ListListLocalId, ListLocal,
        NilFunctionFunctionId, NilListFunctionFunctionId, NilListFunctionId,
        NilListFunctionLocalId, NilListLocalId, StringFunctionFunctionId,
        StringListFunctionFunctionId, StringListFunctionId, StringListFunctionLocalId,
        StringListLocalId, TupleFunctionFunctionId, TupleListFunctionFunctionId,
        TupleListFunctionId, TupleListFunctionLocalId, TupleListLocalId,
    };
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn function_function_ids_expose_exact_return_families() {
        let ids = [
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            FunctionFunctionId::Float(FloatFunctionFunctionId(1)),
            FunctionFunctionId::String(StringFunctionFunctionId(2)),
            FunctionFunctionId::Bool(BoolFunctionFunctionId(3)),
            FunctionFunctionId::Nil(NilFunctionFunctionId(4)),
            FunctionFunctionId::Tuple(TupleFunctionFunctionId(5)),
            FunctionFunctionId::List(ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(6),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
            }),
            FunctionFunctionId::Function(FunctionFunctionFunctionId(7)),
        ];

        assert_eq!(ids[0].family(), FunctionReturnFamily::Int);
        assert_eq!(ids[0].int(), Some(IntFunctionFunctionId(0)));
        assert_eq!(ids[0].float(), None);
        assert_eq!(ids[1].family(), FunctionReturnFamily::Float);
        assert_eq!(ids[1].float(), Some(FloatFunctionFunctionId(1)));
        assert_eq!(ids[1].string(), None);
        assert_eq!(ids[2].family(), FunctionReturnFamily::String);
        assert_eq!(ids[2].string(), Some(StringFunctionFunctionId(2)));
        assert_eq!(ids[2].bool(), None);
        assert_eq!(ids[3].family(), FunctionReturnFamily::Bool);
        assert_eq!(ids[3].bool(), Some(BoolFunctionFunctionId(3)));
        assert_eq!(ids[3].nil(), None);
        assert_eq!(ids[4].family(), FunctionReturnFamily::Nil);
        assert_eq!(ids[4].nil(), Some(NilFunctionFunctionId(4)));
        assert_eq!(ids[4].tuple(), None);
        assert_eq!(ids[5].family(), FunctionReturnFamily::Tuple);
        assert_eq!(ids[5].tuple(), Some(TupleFunctionFunctionId(5)));
        assert_eq!(ids[5].list(), None);
        assert_eq!(ids[6].family(), FunctionReturnFamily::List);
        assert_eq!(
            ids[6].list(),
            Some(ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(6),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
            })
        );
        assert_eq!(ids[6].function(), None);
        assert_eq!(ids[7].family(), FunctionReturnFamily::Function);
        assert_eq!(ids[7].function(), Some(FunctionFunctionFunctionId(7)));
        assert_eq!(ids[7].int(), None);
    }

    #[test]
    fn list_function_ids_preserve_every_item_family() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Bool);
        let cases = [
            (ListFunctionId::Int(IntListFunctionId(0)), ValueType::Int),
            (
                ListFunctionId::String(StringListFunctionId(1)),
                ValueType::String,
            ),
            (
                ListFunctionId::Float(FloatListFunctionId(2)),
                ValueType::Float,
            ),
            (ListFunctionId::Bool(BoolListFunctionId(3)), ValueType::Bool),
            (ListFunctionId::Nil(NilListFunctionId(4)), ValueType::Nil),
            (
                ListFunctionId::Tuple {
                    id: TupleListFunctionId(5),
                    item_type: vec![ValueType::Int],
                },
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            (
                ListFunctionId::List {
                    id: ListListFunctionId(6),
                    item_type: Box::new(ValueType::String),
                },
                ValueType::List(Box::new(ValueType::String)),
            ),
            (
                ListFunctionId::Function {
                    id: FunctionListFunctionId(7),
                    item_type: function_type.clone(),
                },
                ValueType::Function(Box::new(function_type)),
            ),
        ];

        for (id, item_type) in cases {
            assert_eq!(id.item_type(), item_type);
        }
    }

    #[test]
    fn list_function_local_and_function_ids_preserve_function_types() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let locals = [
            ListFunctionLocal::Int {
                local: IntListFunctionLocalId(0),
                type_: function_type.clone(),
            },
            ListFunctionLocal::String {
                local: StringListFunctionLocalId(1),
                type_: function_type.clone(),
            },
            ListFunctionLocal::Float {
                local: FloatListFunctionLocalId(2),
                type_: function_type.clone(),
            },
            ListFunctionLocal::Bool {
                local: BoolListFunctionLocalId(3),
                type_: function_type.clone(),
            },
            ListFunctionLocal::Nil {
                local: NilListFunctionLocalId(4),
                type_: function_type.clone(),
            },
            ListFunctionLocal::Tuple {
                local: TupleListFunctionLocalId(5),
                type_: function_type.clone(),
                item_type: vec![ValueType::Int],
            },
            ListFunctionLocal::List {
                local: ListListFunctionLocalId(6),
                type_: function_type.clone(),
                item_type: Box::new(ValueType::String),
            },
            ListFunctionLocal::Function {
                local: FunctionListFunctionLocalId(7),
                type_: function_type.clone(),
                item_type: Box::new(function_type.clone()),
            },
        ];
        for local in &locals {
            assert_eq!(local.type_(), &function_type);
        }

        let ids = [
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(0),
                type_: function_type.clone(),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(1),
                type_: function_type.clone(),
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(2),
                type_: function_type.clone(),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(3),
                type_: function_type.clone(),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(4),
                type_: function_type.clone(),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(5),
                type_: function_type.clone(),
                item_type: vec![ValueType::Int],
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(6),
                type_: function_type.clone(),
                item_type: Box::new(ValueType::String),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(7),
                type_: function_type.clone(),
                item_type: Box::new(function_type.clone()),
            },
        ];
        for id in &ids {
            assert_eq!(id.type_(), &function_type);
        }
    }

    #[test]
    fn list_locals_preserve_every_item_family() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let locals = [
            ListLocal::Int(IntListLocalId(0)),
            ListLocal::String(StringListLocalId(1)),
            ListLocal::Float(FloatListLocalId(2)),
            ListLocal::Bool(BoolListLocalId(3)),
            ListLocal::Nil(NilListLocalId(4)),
            ListLocal::Tuple {
                local: TupleListLocalId(5),
                item_type: vec![ValueType::Int],
            },
            ListLocal::List {
                local: ListListLocalId(6),
                item_type: Box::new(ValueType::String),
            },
            ListLocal::Function {
                local: FunctionListLocalId(7),
                item_type: function_type.clone(),
            },
        ];
        let expected = [
            ValueType::Int,
            ValueType::String,
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(function_type)),
        ];

        for (local, item_type) in locals.iter().zip(expected) {
            assert_eq!(local.item_type(), item_type);
        }
    }

    #[test]
    fn function_return_family_display_names_every_family() {
        let families = [
            FunctionReturnFamily::Int,
            FunctionReturnFamily::Float,
            FunctionReturnFamily::String,
            FunctionReturnFamily::Bool,
            FunctionReturnFamily::Nil,
            FunctionReturnFamily::Tuple,
            FunctionReturnFamily::List,
            FunctionReturnFamily::Function,
        ];
        let expected = [
            "Int", "Float", "String", "Bool", "Nil", "Tuple", "List", "Function",
        ];

        for (family, expected) in families.iter().zip(expected) {
            assert_eq!(family.to_string(), expected);
        }
    }
}
