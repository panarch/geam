use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomFunctionType, CustomListTypeId, FloatListTypeId,
    FunctionFunctionType, FunctionListTypeId, FunctionType, GenericFunctionType, IntListTypeId,
    ListListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
    TupleListTypeId, UtfCodepointListTypeId,
};

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
    type_: GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericFunctionLocal {
    id: GenericFunctionLocalId,
    type_: GenericFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionLocal {
    id: CustomFunctionLocalId,
    type_: CustomFunctionType,
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
    type_: FunctionFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FunctionLocal {
    Generic(GenericFunctionLocal),
    Never(NeverFunctionLocal),
    Int(IntFunctionLocalId),
    Float(FloatFunctionLocalId),
    String(StringFunctionLocalId),
    BitArray(BitArrayFunctionLocalId),
    UtfCodepoint(UtfCodepointFunctionLocalId),
    Custom(CustomFunctionLocal),
    Bool(BoolFunctionLocalId),
    Nil(NilFunctionLocalId),
    Tuple(TupleFunctionLocalId),
    List(ListFunctionLocal),
    Function(FunctionFunctionLocal),
}

impl CustomFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomFunctionLocalId,
        type_: CustomFunctionType,
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
        type_: GenericFunctionType,
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
        type_: GenericFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> NeverFunctionLocalId {
        self.id
    }
}

impl FunctionFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: FunctionFunctionLocalId,
        type_: FunctionFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> FunctionFunctionLocalId {
        self.id
    }
}

impl ListFunctionLocal {
    #[cfg(test)]
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
    pub(crate) fn list_type(&self) -> crate::plan::execution::type_::ListTypeId {
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
mod tests {
    use super::{
        BitArrayListFunctionLocalId, BoolListFunctionLocalId, CustomListFunctionLocalId,
        FloatListFunctionLocalId, FunctionListFunctionLocalId, IntListFunctionLocalId,
        ListFunctionLocal, ListListFunctionLocalId, NilListFunctionLocalId,
        ParameterListFunctionLocalId, ParameterListListFunctionLocalId, StringListFunctionLocalId,
        TupleListFunctionLocalId, UtfCodepointListFunctionLocalId,
    };

    #[test]
    fn parameter_list_function_locals_preserve_types_and_storage() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let type_ = function_type();
        let locals = [
            ListFunctionLocal::Parameter {
                local: ParameterListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: parameter,
            },
            ListFunctionLocal::ParameterList {
                local: ParameterListListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: nested,
            },
        ];

        assert_eq!(
            locals.clone().map(|local| local.list_type()),
            [parameter.list_type(), nested.list_type()],
        );
        assert_eq!(
            locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );
    }

    #[test]
    fn concrete_list_function_locals_preserve_every_type_and_storage() {
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
        let type_ = function_type();
        let locals = [
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
            locals.clone().map(|local| local.list_type()),
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
        assert_eq!(
            locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );
    }

    fn function_type() -> crate::plan::execution::type_::FunctionType {
        crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Nil,
        )
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
