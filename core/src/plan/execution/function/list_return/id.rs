use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListFunctionId {
    index: usize,
    type_id: IntListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListFunctionId {
    index: usize,
    type_id: StringListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListFunctionId {
    index: usize,
    type_id: BitArrayListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListFunctionId {
    index: usize,
    type_id: UtfCodepointListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListFunctionId {
    index: usize,
    type_id: ParameterListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListFunctionId {
    index: usize,
    type_id: ParameterListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListFunctionId {
    index: usize,
    type_id: CustomListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalListFunctionId {
    index: usize,
    type_id: ExternalListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListFunctionId {
    index: usize,
    type_id: FloatListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListFunctionId {
    index: usize,
    type_id: BoolListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListFunctionId {
    index: usize,
    type_id: NilListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListFunctionId {
    index: usize,
    type_id: TupleListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListFunctionId {
    index: usize,
    type_id: ListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListFunctionId {
    index: usize,
    type_id: FunctionListTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListFunctionId {
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
pub(crate) enum ProfiledListFunctionId<Graph: ExecutionGraphProfile> {
    Core(ListFunctionId),
    External(Graph::ExternalListFunctionId),
}

pub(crate) type RuntimeListFunctionId = ProfiledListFunctionId<HostedExecutionGraph>;

impl IntListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: IntListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> IntListTypeId {
        self.type_id
    }
}

impl StringListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: StringListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> StringListTypeId {
        self.type_id
    }
}

impl BitArrayListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: BitArrayListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> BitArrayListTypeId {
        self.type_id
    }
}

impl UtfCodepointListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: UtfCodepointListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> UtfCodepointListTypeId {
        self.type_id
    }
}

impl ParameterListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ParameterListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ParameterListTypeId {
        self.type_id
    }
}

impl ParameterListListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ParameterListListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ParameterListListTypeId {
        self.type_id
    }
}

impl CustomListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: CustomListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> CustomListTypeId {
        self.type_id
    }
}

impl ExternalListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ExternalListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ExternalListTypeId {
        self.type_id
    }
}

impl FloatListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: FloatListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> FloatListTypeId {
        self.type_id
    }
}

impl BoolListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: BoolListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> BoolListTypeId {
        self.type_id
    }
}

impl NilListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: NilListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> NilListTypeId {
        self.type_id
    }
}

impl TupleListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: TupleListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> TupleListTypeId {
        self.type_id
    }
}

impl ListListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: ListListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> ListListTypeId {
        self.type_id
    }
}

impl FunctionListFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, type_id: FunctionListTypeId) -> Self {
        Self { index, type_id }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(crate) fn type_id(self) -> FunctionListTypeId {
        self.type_id
    }
}

impl FunctionLabelSource for ParameterListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.parameter", self.index())
    }
}

impl FunctionLabelSource for ParameterListListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.parameter_list", self.index())
    }
}

impl FunctionLabelSource for IntListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.int", self.index())
    }
}

impl FunctionLabelSource for StringListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.string", self.index())
    }
}

impl FunctionLabelSource for BitArrayListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.bit_array", self.index())
    }
}

impl FunctionLabelSource for UtfCodepointListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.utf_codepoint", self.index())
    }
}

impl FunctionLabelSource for CustomListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.custom", self.index())
    }
}

impl FunctionLabelSource for ExternalListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.external", self.index())
    }
}

impl FunctionLabelSource for FloatListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.float", self.index())
    }
}

impl FunctionLabelSource for BoolListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.bool", self.index())
    }
}

impl FunctionLabelSource for NilListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.nil", self.index())
    }
}

impl FunctionLabelSource for TupleListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.tuple", self.index())
    }
}

impl FunctionLabelSource for ListListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.list", self.index())
    }
}

impl FunctionLabelSource for FunctionListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("list.function", self.index())
    }
}

impl<Graph: ExecutionGraphProfile> FunctionLabelSource for ProfiledListFunctionId<Graph>
where
    Graph::ExternalListFunctionId: FunctionLabelSource,
{
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Core(id) => id.function_label(),
            Self::External(id) => id.function_label(),
        }
    }
}

impl FunctionLabelSource for ListFunctionId {
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Parameter(id) => id.function_label(),
            Self::ParameterList(id) => id.function_label(),
            Self::Int(id) => id.function_label(),
            Self::String(id) => id.function_label(),
            Self::BitArray(id) => id.function_label(),
            Self::UtfCodepoint(id) => id.function_label(),
            Self::Custom(id) => id.function_label(),
            Self::Float(id) => id.function_label(),
            Self::Bool(id) => id.function_label(),
            Self::Nil(id) => id.function_label(),
            Self::Tuple(id) => id.function_label(),
            Self::List(id) => id.function_label(),
            Self::Function(id) => id.function_label(),
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{ExternalListFunctionId, RuntimeListFunctionId};
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, FunctionLabelSource, RuntimeFunctionId,
    };
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn labels_list_function_families() {
        let cases = [
            ("pub fn main() -> List(value) { [] }", "list.parameter#0"),
            (
                "pub fn main() -> List(List(value)) { [[]] }",
                "list.parameter_list#0",
            ),
            ("pub fn main() -> List(Int) { [] }", "list.int#0"),
            ("pub fn main() -> List(String) { [] }", "list.string#0"),
            ("pub fn main() -> List(BitArray) { [] }", "list.bit_array#0"),
            (
                "pub fn main() -> List(UtfCodepoint) { [] }",
                "list.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> List(Boxed) { [] }",
                "list.custom#0",
            ),
            ("pub fn main() -> List(Float) { [] }", "list.float#0"),
            ("pub fn main() -> List(Bool) { [] }", "list.bool#0"),
            ("pub fn main() -> List(Nil) { [] }", "list.nil#0"),
            ("pub fn main() -> List(#(Int)) { [] }", "list.tuple#0"),
            ("pub fn main() -> List(List(Int)) { [] }", "list.list#0"),
            (
                "pub fn main() -> List(fn() -> Int) { [] }",
                "list.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }

        let external = ExternalListFunctionId::new(
            13,
            ExternalListTypeId::new(ListTypeId::new(0), ExternalTypeId::new(0)),
        );
        explain::assert_written("list.external#13", |output| {
            external.function_label().write(output);
        });
        explain::assert_written("list.external#13", |output| {
            RuntimeListFunctionId::External(external)
                .function_label()
                .write(output);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower a list-returning main function")]
    fn list_function_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", main_list_function_id);
    }

    fn main_list_function_id(plan: &ExecutionPlan) -> RuntimeListFunctionId {
        let RuntimeFunctionId::Core(CoreRuntimeFunctionId::List(function)) = plan.main_runtime()
        else {
            panic!("source should lower a list-returning main function");
        };
        function
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            main_list_function_id(plan).function_label().write(output);
        });
    }
}

#[cfg(test)]
mod external_tests {
    use super::ExternalListFunctionId;
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn external_list_function_id_preserves_index_and_type() {
        let type_id = ExternalListTypeId::new(ListTypeId::new(1), ExternalTypeId::new(2));
        let function = ExternalListFunctionId::new(3, type_id);

        assert_eq!(function.index(), 3);
        assert_eq!(function.type_id(), type_id);
    }
}
