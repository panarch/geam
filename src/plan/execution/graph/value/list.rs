#[cfg(test)]
use crate::plan::execution::type_::ListTypeId;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId,
    StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

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

pub(crate) enum StoredListLocal {
    ParameterList(ParameterListListLocalId),
    Int(IntListLocalId),
    String(StringListLocalId),
    BitArray(BitArrayListLocalId),
    UtfCodepoint(UtfCodepointListLocalId),
    Custom(CustomListLocalId),
    Float(FloatListLocalId),
    Bool(BoolListLocalId),
    Nil(NilListLocalId),
    Tuple(TupleListLocalId),
    List(ListListLocalId),
    Function(FunctionListLocalId),
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
mod tests {
    use crate::plan::execution::graph::{
        BitArrayListLocalId, BoolListLocalId, CustomListLocalId, FloatListLocalId,
        FunctionListLocalId, IntListLocalId, ListListLocalId, ListLocal, NilListLocalId,
        ParameterListListLocalId, ParameterListLocalId, StringListLocalId, TupleListLocalId,
        UtfCodepointListLocalId,
    };
    use crate::plan::{TypeParameterId, ValueType};

    #[test]
    fn parameter_list_locals_preserve_symbolic_and_nested_storage_types() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let locals = [
            ListLocal::Parameter {
                local: ParameterListLocalId(0),
                type_id: parameter,
            },
            ListLocal::ParameterList {
                local: ParameterListListLocalId(0),
                type_id: nested,
            },
        ];
        assert_eq!(
            parameter_plan.list_value_type(locals[0].list_type()),
            ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
        );
        assert_eq!(
            nested_plan.list_value_type(locals[1].list_type()),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                TypeParameterId(0)
            ),)))),
        );
    }

    #[test]
    fn concrete_list_locals_preserve_every_storage_type() {
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
    ints, strings, bit_arrays, utf_codepoints, customs, floats,
    bools, nils, tuples, lists, functions,
  )
  Nil
}
"#,
        );
        let locals = [
            ListLocal::Int {
                local: IntListLocalId(0),
                type_id: plan.int_list_function_id(0).type_id(),
            },
            ListLocal::String {
                local: StringListLocalId(0),
                type_id: plan.string_list_function_id(0).type_id(),
            },
            ListLocal::BitArray {
                local: BitArrayListLocalId(0),
                type_id: plan.bit_array_list_function_id(0).type_id(),
            },
            ListLocal::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                type_id: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListLocal::Custom {
                local: CustomListLocalId(0),
                type_id: plan.custom_list_function_id(0).type_id(),
            },
            ListLocal::Float {
                local: FloatListLocalId(0),
                type_id: plan.float_list_function_id(0).type_id(),
            },
            ListLocal::Bool {
                local: BoolListLocalId(0),
                type_id: plan.bool_list_function_id(0).type_id(),
            },
            ListLocal::Nil {
                local: NilListLocalId(0),
                type_id: plan.nil_list_function_id(0).type_id(),
            },
            ListLocal::Tuple {
                local: TupleListLocalId(0),
                type_id: plan.tuple_list_function_id(0).type_id(),
            },
            ListLocal::List {
                local: ListListLocalId(0),
                type_id: plan.list_list_function_id(0).type_id(),
            },
            ListLocal::Function {
                local: FunctionListLocalId(0),
                type_id: plan.function_list_function_id(0).type_id(),
            },
        ];
        assert_eq!(
            locals.map(|local| local.list_type()),
            [
                plan.int_list_function_id(0).type_id().list_type(),
                plan.string_list_function_id(0).type_id().list_type(),
                plan.bit_array_list_function_id(0).type_id().list_type(),
                plan.utf_codepoint_list_function_id(0).type_id().list_type(),
                plan.custom_list_function_id(0).type_id().list_type(),
                plan.float_list_function_id(0).type_id().list_type(),
                plan.bool_list_function_id(0).type_id().list_type(),
                plan.nil_list_function_id(0).type_id().list_type(),
                plan.tuple_list_function_id(0).type_id().list_type(),
                plan.list_list_function_id(0).type_id().list_type(),
                plan.function_list_function_id(0).type_id().list_type(),
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
