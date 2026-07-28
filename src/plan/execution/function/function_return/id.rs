use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    FunctionType, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

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
    type_: crate::plan::execution::type_::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NeverFunctionFunctionId {
    index: usize,
    type_: crate::plan::execution::type_::GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFunctionFunctionId {
    index: usize,
    type_: crate::plan::execution::type_::CustomFunctionType,
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
    type_: crate::plan::execution::type_::FunctionFunctionType,
}

impl CustomFunctionFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        type_: crate::plan::execution::type_::CustomFunctionType,
    ) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn with_index(&self, index: usize) -> Self {
        Self {
            index,
            type_: self.type_.clone(),
        }
    }
}

impl GenericFunctionFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        type_: crate::plan::execution::type_::GenericFunctionType,
    ) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl NeverFunctionFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        type_: crate::plan::execution::type_::GenericFunctionType,
    ) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl FunctionFunctionFunctionId {
    pub(in crate::plan::execution) fn new(
        index: usize,
        type_: crate::plan::execution::type_::FunctionFunctionType,
    ) -> Self {
        Self { index, type_ }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn with_index(&self, index: usize) -> Self {
        Self {
            index,
            type_: self.type_.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &crate::plan::execution::type_::FunctionFunctionType {
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

impl FunctionLabelSource for FunctionFunctionId {
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Generic(id) => FunctionLabel::new("function.generic", id.index()),
            Self::Never(id) => FunctionLabel::new("function.never", id.index()),
            Self::Int(id) => FunctionLabel::new("function.int", id.0),
            Self::Float(id) => FunctionLabel::new("function.float", id.0),
            Self::String(id) => FunctionLabel::new("function.string", id.0),
            Self::BitArray(id) => FunctionLabel::new("function.bit_array", id.0),
            Self::UtfCodepoint(id) => FunctionLabel::new("function.utf_codepoint", id.0),
            Self::Custom(id) => FunctionLabel::new("function.custom", id.index()),
            Self::Bool(id) => FunctionLabel::new("function.bool", id.0),
            Self::Nil(id) => FunctionLabel::new("function.nil", id.0),
            Self::Tuple(id) => FunctionLabel::new("function.tuple", id.0),
            Self::List(id) => id.function_label(),
            Self::Function(id) => FunctionLabel::new("function.function", id.index()),
        }
    }
}

impl FunctionLabelSource for ListFunctionFunctionId {
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Parameter { id, .. } => FunctionLabel::new("function.list.parameter", id.0),
            Self::ParameterList { id, .. } => {
                FunctionLabel::new("function.list.parameter_list", id.0)
            }
            Self::Int { id, .. } => FunctionLabel::new("function.list.int", id.0),
            Self::String { id, .. } => FunctionLabel::new("function.list.string", id.0),
            Self::BitArray { id, .. } => FunctionLabel::new("function.list.bit_array", id.0),
            Self::UtfCodepoint { id, .. } => {
                FunctionLabel::new("function.list.utf_codepoint", id.0)
            }
            Self::Custom { id, .. } => FunctionLabel::new("function.list.custom", id.0),
            Self::Float { id, .. } => FunctionLabel::new("function.list.float", id.0),
            Self::Bool { id, .. } => FunctionLabel::new("function.list.bool", id.0),
            Self::Nil { id, .. } => FunctionLabel::new("function.list.nil", id.0),
            Self::Tuple { id, .. } => FunctionLabel::new("function.list.tuple", id.0),
            Self::List { id, .. } => FunctionLabel::new("function.list.list", id.0),
            Self::Function { id, .. } => FunctionLabel::new("function.list.function", id.0),
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::FunctionFunctionId;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{FunctionLabelSource, RuntimeFunctionId};

    #[test]
    fn labels_function_return_families() {
        let cases = [
            (
                "pub fn main() -> fn(value) -> value { fn(value) { value } }",
                "function.generic#0",
            ),
            (
                "pub fn main() -> fn() -> value { fn() { panic } }",
                "function.never#0",
            ),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                "function.int#0",
            ),
            (
                "pub fn main() -> fn() -> Float { fn() { 1.0 } }",
                "function.float#0",
            ),
            (
                "pub fn main() -> fn() -> String { fn() { \"one\" } }",
                "function.string#0",
            ),
            (
                "pub fn main() -> fn() -> BitArray { fn() { <<1>> } }",
                "function.bit_array#0",
            ),
            (
                "pub fn main() -> fn() -> UtfCodepoint { fn() { panic } }",
                "function.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> Boxed { fn() { Boxed(1) } }",
                "function.custom#0",
            ),
            (
                "pub fn main() -> fn() -> Bool { fn() { True } }",
                "function.bool#0",
            ),
            (
                "pub fn main() -> fn() -> Nil { fn() { Nil } }",
                "function.nil#0",
            ),
            (
                "pub fn main() -> fn() -> #(Int) { fn() { #(1) } }",
                "function.tuple#0",
            ),
            (
                "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
                "function.list.int#0",
            ),
            (
                "pub fn main() -> fn() -> fn() -> Int { fn() { fn() { 1 } } }",
                "function.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    #[test]
    fn labels_list_returning_function_families() {
        let cases = [
            (
                "pub fn main() -> fn() -> List(value) { fn() { [] } }",
                "function.list.parameter#0",
            ),
            (
                "pub fn main() -> fn() -> List(List(value)) { fn() { [[]] } }",
                "function.list.parameter_list#0",
            ),
            (
                "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
                "function.list.int#0",
            ),
            (
                "pub fn main() -> fn() -> List(String) { fn() { [] } }",
                "function.list.string#0",
            ),
            (
                "pub fn main() -> fn() -> List(BitArray) { fn() { [] } }",
                "function.list.bit_array#0",
            ),
            (
                "pub fn main() -> fn() -> List(UtfCodepoint) { fn() { [] } }",
                "function.list.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> List(Boxed) { fn() { [] } }",
                "function.list.custom#0",
            ),
            (
                "pub fn main() -> fn() -> List(Float) { fn() { [] } }",
                "function.list.float#0",
            ),
            (
                "pub fn main() -> fn() -> List(Bool) { fn() { [] } }",
                "function.list.bool#0",
            ),
            (
                "pub fn main() -> fn() -> List(Nil) { fn() { [] } }",
                "function.list.nil#0",
            ),
            (
                "pub fn main() -> fn() -> List(#(Int)) { fn() { [] } }",
                "function.list.tuple#0",
            ),
            (
                "pub fn main() -> fn() -> List(List(Int)) { fn() { [] } }",
                "function.list.list#0",
            ),
            (
                "pub fn main() -> fn() -> List(fn() -> Int) { fn() { [] } }",
                "function.list.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    #[test]
    #[should_panic(expected = "source should lower a function-returning main function")]
    fn function_return_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", main_function_id);
    }

    fn main_function_id(plan: &ExecutionPlan) -> FunctionFunctionId {
        let RuntimeFunctionId::Function { id, .. } = plan.main_runtime() else {
            panic!("source should lower a function-returning main function");
        };
        id
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            main_function_id(plan).function_label().write(output);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayFunctionFunctionId, BoolListFunctionFunctionId, CustomFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionFunctionId, FunctionListFunctionFunctionId,
        IntFunctionFunctionId, IntListFunctionFunctionId, ListFunctionFunctionId,
        ListListFunctionFunctionId, NeverFunctionFunctionId, NilListFunctionFunctionId,
        ParameterListFunctionFunctionId, ParameterListListFunctionFunctionId,
        StringListFunctionFunctionId, TupleListFunctionFunctionId, UtfCodepointFunctionFunctionId,
    };
    use crate::plan::execution::function::RuntimeFunctionId;
    use crate::plan::{CustomType, CustomTypeName, ValueType};

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn parameter_list_function_ids_preserve_symbolic_and_nested_storage_types() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let function_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Nil,
        );
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

        assert_eq!(parameter_function.type_(), &function_type);
        assert_eq!(nested_function.type_(), &function_type);
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)).generic(),
            None,
        );
    }

    #[test]
    fn never_function_function_id_projection_is_typed() {
        let function_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let type_ = crate::plan::execution::type_::GenericFunctionType::from_shapes(
            function_type.clone(),
            crate::plan::execution::type_::FunctionShape::new(
                crate::plan::execution::type_::ValueShapeId::new(0),
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
        let function_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let type_ = crate::plan::execution::type_::GenericFunctionType::from_shapes(
            function_type.clone(),
            crate::plan::execution::type_::FunctionShape::new(
                crate::plan::execution::type_::ValueShapeId::new(0),
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
        let return_type = crate::plan::execution::type_::CustomTypeId::new(0);
        let function = CustomFunctionFunctionId::new(
            2,
            crate::plan::execution::type_::CustomFunctionType::from_shapes(
                crate::plan::execution::type_::FunctionType::new(
                    Vec::new(),
                    crate::plan::execution::type_::ValueType::Custom(return_type),
                ),
                Vec::new(),
                crate::plan::execution::type_::CustomValueShape::new(
                    return_type,
                    crate::plan::execution::type_::CustomValueShapeId::new(0),
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
        let return_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::List(list_type.list_type()),
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
        let return_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::List(list_type.list_type()),
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
        let return_type = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::List(list_type.list_type()),
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
        let type_ = crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Nil,
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
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
