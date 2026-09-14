use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntListFunctionId {
    pub index: usize,
    pub type_id: IntListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListFunctionId {
    pub index: usize,
    pub type_id: StringListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayListFunctionId {
    pub index: usize,
    pub type_id: BitArrayListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointListFunctionId {
    pub index: usize,
    pub type_id: UtfCodepointListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListFunctionId {
    pub index: usize,
    pub type_id: ParameterListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterListListFunctionId {
    pub index: usize,
    pub type_id: ParameterListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomListFunctionId {
    pub index: usize,
    pub type_id: CustomListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalListFunctionId {
    pub index: usize,
    pub type_id: ExternalListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatListFunctionId {
    pub index: usize,
    pub type_id: FloatListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolListFunctionId {
    pub index: usize,
    pub type_id: BoolListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilListFunctionId {
    pub index: usize,
    pub type_id: NilListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleListFunctionId {
    pub index: usize,
    pub type_id: TupleListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListListFunctionId {
    pub index: usize,
    pub type_id: ListListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionListFunctionId {
    pub index: usize,
    pub type_id: FunctionListTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFunctionId {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryListFunctionId<Graph: ExecutionGraphProfile = HostedExecutionGraph> {
    Int(IntListFunctionId),
    String(StringListFunctionId),
    BitArray(BitArrayListFunctionId),
    UtfCodepoint(UtfCodepointListFunctionId),
    Custom(CustomListFunctionId),
    External(Graph::ExternalListFunctionId),
    Float(FloatListFunctionId),
    Bool(BoolListFunctionId),
    Nil(NilListFunctionId),
    Tuple(TupleListFunctionId),
    List(ListListFunctionId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfiledListFunctionId<Graph: ExecutionGraphProfile> {
    Core(ListFunctionId),
    External(Graph::ExternalListFunctionId),
}

pub(crate) type RuntimeListFunctionId = ProfiledListFunctionId<HostedExecutionGraph>;

impl<Graph: ExecutionGraphProfile> LibraryListFunctionId<Graph> {
    pub(crate) fn profiled_runtime_id(&self) -> ProfiledListFunctionId<Graph> {
        match self {
            Self::Int(id) => ProfiledListFunctionId::Core(ListFunctionId::Int(*id)),
            Self::String(id) => ProfiledListFunctionId::Core(ListFunctionId::String(*id)),
            Self::BitArray(id) => ProfiledListFunctionId::Core(ListFunctionId::BitArray(*id)),
            Self::UtfCodepoint(id) => {
                ProfiledListFunctionId::Core(ListFunctionId::UtfCodepoint(*id))
            }
            Self::Custom(id) => ProfiledListFunctionId::Core(ListFunctionId::Custom(*id)),
            Self::External(id) => ProfiledListFunctionId::External(id.clone()),
            Self::Float(id) => ProfiledListFunctionId::Core(ListFunctionId::Float(*id)),
            Self::Bool(id) => ProfiledListFunctionId::Core(ListFunctionId::Bool(*id)),
            Self::Nil(id) => ProfiledListFunctionId::Core(ListFunctionId::Nil(*id)),
            Self::Tuple(id) => ProfiledListFunctionId::Core(ListFunctionId::Tuple(*id)),
            Self::List(id) => ProfiledListFunctionId::Core(ListFunctionId::List(*id)),
        }
    }
}

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

impl Emit for IntListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::IntListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for StringListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::StringListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for BitArrayListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::BitArrayListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for UtfCodepointListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::UtfCodepointListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for ParameterListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::ParameterListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for ParameterListListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::ParameterListListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for CustomListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::CustomListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for ExternalListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::ExternalListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for FloatListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::FloatListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for BoolListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::BoolListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for NilListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::NilListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for TupleListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::TupleListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for ListListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::ListListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for FunctionListFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, type_id } = self;
        output.structure(
            "function::FunctionListFunctionId",
            &[("index", index), ("type_id", type_id)],
        );
    }
}

impl Emit for ListFunctionId {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(field_0) => {
                output.call("function::ListFunctionId::Parameter", &[field_0])
            }
            Self::ParameterList(field_0) => {
                output.call("function::ListFunctionId::ParameterList", &[field_0])
            }
            Self::Int(field_0) => output.call("function::ListFunctionId::Int", &[field_0]),
            Self::String(field_0) => output.call("function::ListFunctionId::String", &[field_0]),
            Self::BitArray(field_0) => {
                output.call("function::ListFunctionId::BitArray", &[field_0])
            }
            Self::UtfCodepoint(field_0) => {
                output.call("function::ListFunctionId::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => output.call("function::ListFunctionId::Custom", &[field_0]),
            Self::Float(field_0) => output.call("function::ListFunctionId::Float", &[field_0]),
            Self::Bool(field_0) => output.call("function::ListFunctionId::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("function::ListFunctionId::Nil", &[field_0]),
            Self::Tuple(field_0) => output.call("function::ListFunctionId::Tuple", &[field_0]),
            Self::List(field_0) => output.call("function::ListFunctionId::List", &[field_0]),
            Self::Function(field_0) => {
                output.call("function::ListFunctionId::Function", &[field_0])
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> Emit for LibraryListFunctionId<Graph>
where
    Graph::ExternalListFunctionId: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(field_0) => output.call("function::LibraryListFunctionId::Int", &[field_0]),
            Self::String(field_0) => {
                output.call("function::LibraryListFunctionId::String", &[field_0])
            }
            Self::BitArray(field_0) => {
                output.call("function::LibraryListFunctionId::BitArray", &[field_0])
            }
            Self::UtfCodepoint(field_0) => {
                output.call("function::LibraryListFunctionId::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => {
                output.call("function::LibraryListFunctionId::Custom", &[field_0])
            }
            Self::External(field_0) => {
                output.call("function::LibraryListFunctionId::External", &[field_0])
            }
            Self::Float(field_0) => {
                output.call("function::LibraryListFunctionId::Float", &[field_0])
            }
            Self::Bool(field_0) => output.call("function::LibraryListFunctionId::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("function::LibraryListFunctionId::Nil", &[field_0]),
            Self::Tuple(field_0) => {
                output.call("function::LibraryListFunctionId::Tuple", &[field_0])
            }
            Self::List(field_0) => output.call("function::LibraryListFunctionId::List", &[field_0]),
        }
    }
}

impl<Graph: ExecutionGraphProfile> Emit for ProfiledListFunctionId<Graph>
where
    Graph::ExternalListFunctionId: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Core(field_0) => {
                output.call("function::ProfiledListFunctionId::Core", &[field_0])
            }
            Self::External(field_0) => {
                output.call("function::ProfiledListFunctionId::External", &[field_0])
            }
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
        FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, LibraryListFunctionId,
        ListFunctionId, ListListFunctionId, NilListFunctionId, ParameterListFunctionId,
        ParameterListListFunctionId, ProfiledListFunctionId, StringListFunctionId,
        TupleListFunctionId, UtfCodepointListFunctionId,
    };
    use crate::plan::execution::function::HostedExecutionGraph;
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::list::{FunctionItemTypeId, TupleItemTypeId};
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId,
        ListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
        TupleListTypeId, UtfCodepointListTypeId,
    };

    #[test]
    fn emits_list_function_ids_with_nested_item_types() {
        let cases = [
            (
                ListFunctionId::Int(IntListFunctionId::new(2, IntListTypeId::new(ListTypeId(3)))),
                r#"
data::function::ListFunctionId::Int(data::function::IntListFunctionId {
    index: 2,
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::String(StringListFunctionId::new(
                    2,
                    StringListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::ListFunctionId::String(data::function::StringListFunctionId {
    index: 2,
    type_id: data::type_::StringListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::BitArray(BitArrayListFunctionId::new(
                    2,
                    BitArrayListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::ListFunctionId::BitArray(data::function::BitArrayListFunctionId {
    index: 2,
    type_id: data::type_::BitArrayListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::UtfCodepoint(UtfCodepointListFunctionId::new(
                    2,
                    UtfCodepointListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::ListFunctionId::UtfCodepoint(data::function::UtfCodepointListFunctionId {
    index: 2,
    type_id: data::type_::UtfCodepointListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Float(FloatListFunctionId::new(
                    2,
                    FloatListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::ListFunctionId::Float(data::function::FloatListFunctionId {
    index: 2,
    type_id: data::type_::FloatListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Bool(BoolListFunctionId::new(
                    2,
                    BoolListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::ListFunctionId::Bool(data::function::BoolListFunctionId {
    index: 2,
    type_id: data::type_::BoolListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Nil(NilListFunctionId::new(2, NilListTypeId::new(ListTypeId(3)))),
                r#"
data::function::ListFunctionId::Nil(data::function::NilListFunctionId {
    index: 2,
    type_id: data::type_::NilListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Parameter(ParameterListFunctionId::new(
                    2,
                    ParameterListTypeId::new(ListTypeId(3), crate::plan::TypeParameterId(4)),
                )),
                r#"
data::function::ListFunctionId::Parameter(data::function::ParameterListFunctionId {
    index: 2,
    type_id: data::type_::ParameterListTypeId {
        list_type: data::type_::ListTypeId(3),
        item: data::type_::parameter_id(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::ParameterList(ParameterListListFunctionId::new(
                    2,
                    ParameterListListTypeId::new(
                        ListTypeId(3),
                        ParameterListTypeId::new(ListTypeId(4), crate::plan::TypeParameterId(5)),
                    ),
                )),
                r#"
data::function::ListFunctionId::ParameterList(data::function::ParameterListListFunctionId {
    index: 2,
    type_id: data::type_::ParameterListListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ParameterListTypeId {
            list_type: data::type_::ListTypeId(4),
            item: data::type_::parameter_id(5),
        },
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Custom(CustomListFunctionId::new(
                    2,
                    CustomListTypeId::new(ListTypeId(3), CustomTypeId(4)),
                )),
                r#"
data::function::ListFunctionId::Custom(data::function::CustomListFunctionId {
    index: 2,
    type_id: data::type_::CustomListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::CustomTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Tuple(TupleListFunctionId::new(
                    2,
                    TupleListTypeId {
                        list_type: ListTypeId(3),
                        item_type: TupleItemTypeId(4),
                    },
                )),
                r#"
data::function::ListFunctionId::Tuple(data::function::TupleListFunctionId {
    index: 2,
    type_id: data::type_::TupleListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::TupleItemTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::List(ListListFunctionId::new(
                    2,
                    ListListTypeId::new(ListTypeId(3), ListTypeId(4)),
                )),
                r#"
data::function::ListFunctionId::List(data::function::ListListFunctionId {
    index: 2,
    type_id: data::type_::ListListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ListTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                ListFunctionId::Function(FunctionListFunctionId::new(
                    2,
                    FunctionListTypeId {
                        list_type: ListTypeId(3),
                        item_type: FunctionItemTypeId(4),
                    },
                )),
                r#"
data::function::ListFunctionId::Function(data::function::FunctionListFunctionId {
    index: 2,
    type_id: data::type_::FunctionListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::FunctionItemTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
        ];
        for (id, expected) in cases {
            assert_eq!(Rust::expression(&id), expected);
        }
        let library: [(LibraryListFunctionId<HostedExecutionGraph>, &str); 11] = [
            (
                LibraryListFunctionId::Int(IntListFunctionId::new(
                    2,
                    IntListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::Int(data::function::IntListFunctionId {
    index: 2,
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::String(StringListFunctionId::new(
                    2,
                    StringListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::String(data::function::StringListFunctionId {
    index: 2,
    type_id: data::type_::StringListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::BitArray(BitArrayListFunctionId::new(
                    2,
                    BitArrayListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::BitArray(data::function::BitArrayListFunctionId {
    index: 2,
    type_id: data::type_::BitArrayListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::UtfCodepoint(UtfCodepointListFunctionId::new(
                    2,
                    UtfCodepointListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::UtfCodepoint(data::function::UtfCodepointListFunctionId {
    index: 2,
    type_id: data::type_::UtfCodepointListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::Float(FloatListFunctionId::new(
                    2,
                    FloatListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::Float(data::function::FloatListFunctionId {
    index: 2,
    type_id: data::type_::FloatListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::Bool(BoolListFunctionId::new(
                    2,
                    BoolListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::Bool(data::function::BoolListFunctionId {
    index: 2,
    type_id: data::type_::BoolListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::Nil(NilListFunctionId::new(
                    2,
                    NilListTypeId::new(ListTypeId(3)),
                )),
                r#"
data::function::LibraryListFunctionId::Nil(data::function::NilListFunctionId {
    index: 2,
    type_id: data::type_::NilListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::Custom(CustomListFunctionId::new(
                    2,
                    CustomListTypeId::new(ListTypeId(3), CustomTypeId(4)),
                )),
                r#"
data::function::LibraryListFunctionId::Custom(data::function::CustomListFunctionId {
    index: 2,
    type_id: data::type_::CustomListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::CustomTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::External(ExternalListFunctionId::new(
                    2,
                    ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                )),
                r#"
data::function::LibraryListFunctionId::External(data::function::ExternalListFunctionId {
    index: 2,
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::Tuple(TupleListFunctionId::new(
                    2,
                    TupleListTypeId {
                        list_type: ListTypeId(3),
                        item_type: TupleItemTypeId(4),
                    },
                )),
                r#"
data::function::LibraryListFunctionId::Tuple(data::function::TupleListFunctionId {
    index: 2,
    type_id: data::type_::TupleListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::TupleItemTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
            (
                LibraryListFunctionId::List(ListListFunctionId::new(
                    2,
                    ListListTypeId::new(ListTypeId(3), ListTypeId(4)),
                )),
                r#"
data::function::LibraryListFunctionId::List(data::function::ListListFunctionId {
    index: 2,
    type_id: data::type_::ListListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ListTypeId(4),
    },
})"#
                .trim_start_matches('\n'),
            ),
        ];
        for (id, expected) in library {
            assert_eq!(Rust::expression(&id), expected);
        }
        let profiled: [(ProfiledListFunctionId<HostedExecutionGraph>, &str); 2] = [
            (
                ProfiledListFunctionId::Core(ListFunctionId::Int(IntListFunctionId::new(
                    2,
                    IntListTypeId::new(ListTypeId(3)),
                ))),
                r#"
data::function::ProfiledListFunctionId::Core(data::function::ListFunctionId::Int(data::function::IntListFunctionId {
    index: 2,
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}))"#.trim_start_matches('\n'),
            ),
            (
                ProfiledListFunctionId::External(ExternalListFunctionId::new(
                    2,
                    ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                )),
                r#"
data::function::ProfiledListFunctionId::External(data::function::ExternalListFunctionId {
    index: 2,
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
})"#.trim_start_matches('\n'),
            ),
        ];
        for (id, expected) in profiled {
            assert_eq!(Rust::expression(&id), expected);
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
mod tests {
    use super::{
        BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
        FloatListFunctionId, IntListFunctionId, LibraryListFunctionId, ListFunctionId,
        ListListFunctionId, NilListFunctionId, ProfiledListFunctionId, StringListFunctionId,
        TupleListFunctionId, UtfCodepointListFunctionId,
    };
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
        StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
    };

    #[test]
    fn library_list_routing_preserves_the_function_and_item_type_ids() {
        macro_rules! core {
            ($variant:ident, $id:expr) => {{
                let id = $id;
                (
                    LibraryListFunctionId::$variant(id),
                    ProfiledListFunctionId::Core(ListFunctionId::$variant(id)),
                )
            }};
        }
        for (library, expected) in [
            core!(
                Int,
                IntListFunctionId::new(11, IntListTypeId::new(ListTypeId::new(1)))
            ),
            core!(
                String,
                StringListFunctionId::new(12, StringListTypeId::new(ListTypeId::new(2)))
            ),
            core!(
                BitArray,
                BitArrayListFunctionId::new(13, BitArrayListTypeId::new(ListTypeId::new(3)))
            ),
            core!(
                UtfCodepoint,
                UtfCodepointListFunctionId::new(
                    14,
                    UtfCodepointListTypeId::new(ListTypeId::new(4))
                )
            ),
            core!(
                Custom,
                CustomListFunctionId::new(
                    15,
                    CustomListTypeId::new(ListTypeId::new(5), CustomTypeId::new(2))
                )
            ),
            core!(
                Float,
                FloatListFunctionId::new(16, FloatListTypeId::new(ListTypeId::new(6)))
            ),
            core!(
                Bool,
                BoolListFunctionId::new(17, BoolListTypeId::new(ListTypeId::new(7)))
            ),
            core!(
                Nil,
                NilListFunctionId::new(18, NilListTypeId::new(ListTypeId::new(8)))
            ),
            core!(
                Tuple,
                TupleListFunctionId::new(19, TupleListTypeId::new(ListTypeId::new(9), 3))
            ),
            core!(
                List,
                ListListFunctionId::new(
                    20,
                    ListListTypeId::new(ListTypeId::new(10), ListTypeId::new(1))
                )
            ),
            {
                let id = ExternalListFunctionId::new(
                    21,
                    ExternalListTypeId::new(ListTypeId::new(11), ExternalTypeId::new(4)),
                );
                (
                    LibraryListFunctionId::<super::HostedExecutionGraph>::External(id),
                    ProfiledListFunctionId::External(id),
                )
            },
        ] {
            assert_eq!(library.profiled_runtime_id(), expected);
        }
    }

    #[test]
    fn external_list_function_id_preserves_index_and_type() {
        let type_id = ExternalListTypeId::new(ListTypeId::new(1), ExternalTypeId::new(2));
        let function = ExternalListFunctionId::new(3, type_id);

        assert_eq!(function.index(), 3);
        assert_eq!(function.type_id(), type_id);
    }
}
