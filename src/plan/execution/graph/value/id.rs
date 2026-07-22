#[cfg(test)]
use crate::plan::execution::ListTypeId;
use crate::plan::execution::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    FunctionType, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
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
    shape: crate::plan::execution::CustomValueShape,
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
    type_: crate::plan::execution::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericFunctionLocal {
    id: GenericFunctionLocalId,
    type_: crate::plan::execution::GenericFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionLocal {
    id: CustomFunctionLocalId,
    type_: crate::plan::execution::CustomFunctionType,
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
    type_: crate::plan::execution::FunctionFunctionType,
}

impl CustomFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomFunctionLocalId,
        type_: crate::plan::execution::CustomFunctionType,
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
        type_: crate::plan::execution::GenericFunctionType,
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
        type_: crate::plan::execution::GenericFunctionType,
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
        shape: crate::plan::execution::CustomValueShape,
    ) -> Self {
        Self { id, shape }
    }

    pub(crate) fn id(self) -> CustomLocalId {
        self.id
    }
}

impl FunctionFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: FunctionFunctionLocalId,
        type_: crate::plan::execution::FunctionFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> FunctionFunctionLocalId {
        self.id
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
mod tests {
    use super::{
        BitArrayListFunctionLocalId, BitArrayListLocalId, BoolListFunctionLocalId, BoolListLocalId,
        CustomListFunctionLocalId, CustomListLocalId, FloatListFunctionLocalId, FloatListLocalId,
        FunctionListFunctionLocalId, FunctionListLocalId, IntListFunctionLocalId, IntListLocalId,
        ListFunctionLocal, ListListFunctionLocalId, ListListLocalId, ListLocal,
        NilListFunctionLocalId, NilListLocalId, ParameterListFunctionLocalId,
        ParameterListListFunctionLocalId, ParameterListListLocalId, ParameterListLocalId,
        StringListFunctionLocalId, StringListLocalId, TupleListFunctionLocalId, TupleListLocalId,
        UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    };
    use crate::plan::{TypeParameterId, ValueType};

    #[test]
    fn parameter_list_locals_preserve_symbolic_and_nested_storage_types() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let function_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Nil,
        );
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
        let function_locals = [
            ListFunctionLocal::Parameter {
                local: ParameterListFunctionLocalId(0),
                type_: function_type.clone(),
                list_type: parameter,
            },
            ListFunctionLocal::ParameterList {
                local: ParameterListListFunctionLocalId(0),
                type_: function_type.clone(),
                list_type: nested,
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
        assert_eq!(
            locals.map(|local| local.list_type()),
            function_locals.clone().map(|local| local.list_type()),
        );
        assert_eq!(
            function_locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| function_type.clone()),
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
        let type_ = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Nil,
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
        let function_locals = [
            ListFunctionLocal::Int {
                local: IntListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.int_list_function_id(0).type_id(),
            },
            ListFunctionLocal::String {
                local: StringListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.string_list_function_id(0).type_id(),
            },
            ListFunctionLocal::BitArray {
                local: BitArrayListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bit_array_list_function_id(0).type_id(),
            },
            ListFunctionLocal::UtfCodepoint {
                local: UtfCodepointListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Custom {
                local: CustomListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.custom_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Float {
                local: FloatListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.float_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Bool {
                local: BoolListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bool_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Nil {
                local: NilListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.nil_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Tuple {
                local: TupleListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.tuple_list_function_id(0).type_id(),
            },
            ListFunctionLocal::List {
                local: ListListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.list_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Function {
                local: FunctionListFunctionLocalId(0),
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
