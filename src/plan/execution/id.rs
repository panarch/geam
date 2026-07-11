use super::{
    BoolListTypeId, FloatListTypeId, FunctionListTypeId, FunctionType, IntListTypeId,
    ListListTypeId, ListTypeId, NilListTypeId, StringListTypeId, TupleListTypeId, ValueType,
};

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
    Int {
        local: IntListLocalId,
        type_id: IntListTypeId,
    },
    String {
        local: StringListLocalId,
        type_id: StringListTypeId,
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
        type_: FunctionType,
        list_type: IntListTypeId,
    },
    String {
        local: StringListFunctionLocalId,
        type_: FunctionType,
        list_type: StringListTypeId,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
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
        type_: FunctionType,
        list_type: IntListTypeId,
    },
    String {
        id: StringListFunctionFunctionId,
        type_: FunctionType,
        list_type: StringListTypeId,
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
    pub(crate) fn type_(&self) -> &FunctionType {
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

    #[cfg(test)]
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int { list_type, .. } => list_type.list_type(),
            Self::String { list_type, .. } => list_type.list_type(),
            Self::Float { list_type, .. } => list_type.list_type(),
            Self::Bool { list_type, .. } => list_type.list_type(),
            Self::Nil { list_type, .. } => list_type.list_type(),
            Self::Tuple { list_type, .. } => list_type.list_type(),
            Self::List { list_type, .. } => list_type.list_type(),
            Self::Function { list_type, .. } => list_type.list_type(),
        }
    }
}

impl ListFunctionId {
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int(id) => id.type_id().list_type(),
            Self::String(id) => id.type_id().list_type(),
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
    pub(crate) fn list_type(&self) -> ListTypeId {
        match self {
            Self::Int { type_id, .. } => type_id.list_type(),
            Self::String { type_id, .. } => type_id.list_type(),
            Self::Float { type_id, .. } => type_id.list_type(),
            Self::Bool { type_id, .. } => type_id.list_type(),
            Self::Nil { type_id, .. } => type_id.list_type(),
            Self::Tuple { type_id, .. } => type_id.list_type(),
            Self::List { type_id, .. } => type_id.list_type(),
            Self::Function { type_id, .. } => type_id.list_type(),
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
        BoolListLocalId, FloatListLocalId, FunctionListLocalId, FunctionReturnFamily,
        IntListLocalId, ListListLocalId, ListLocal, NilListLocalId, StringListLocalId,
        TupleListLocalId,
    };
    use crate::plan::ValueType;

    #[test]
    fn list_locals_preserve_every_lowered_list_type() {
        let plan = execution_plan(
            r#"
fn int_values(values: List(Int)) { values }
fn string_values(values: List(String)) { values }
fn float_values(values: List(Float)) { values }
fn bool_values(values: List(Bool)) { values }
fn nil_values(values: List(Nil)) { values }
fn tuple_values(values: List(#(Int))) { values }
fn list_values(values: List(List(Int))) { values }
fn function_values(values: List(fn() -> Int)) { values }

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
    }

    #[test]
    fn function_return_family_display_names_every_family() {
        assert_eq!(
            [
                FunctionReturnFamily::Int,
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::List,
                FunctionReturnFamily::Function,
            ]
            .map(|family| family.to_string()),
            [
                "Int", "Float", "String", "Bool", "Nil", "Tuple", "List", "Function",
            ],
        );
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
