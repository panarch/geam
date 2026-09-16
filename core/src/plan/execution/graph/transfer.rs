use super::{FunctionFunctionLocal, ListFunctionLocal, ListLocal, ParamLocal};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

/// A complete, bounded routing of the consumed block's initialized typed storage.
#[derive(Clone)]
pub struct Transfer {
    pub families: Table<FamilyTransfer>,
}

#[derive(Clone)]
pub struct FamilyTransfer {
    pub family: StorageFamily,
    /// One source position per output, interpreted after earlier outputs are placed.
    pub positions: Table<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Bool,
    Tuple,
    ParameterList,
    ParameterListList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    ExternalList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ListList,
    FunctionList,
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    ExternalFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    GenericFunction,
    NeverFunction,
    ParameterListFunction,
    ParameterListListFunction,
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    CustomListFunction,
    ExternalListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ListListFunction,
    FunctionListFunction,
    CoreFunctionFunction,
    ExternalFunctionFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::plan::execution) struct StorageSlot {
    pub family: StorageFamily,
    pub index: usize,
}

impl ParamLocal {
    pub(in crate::plan::execution) fn storage_slot(&self) -> Option<StorageSlot> {
        use StorageFamily as F;
        let (family, index) = match self {
            Self::Int(local) => (F::Int, local.0),
            Self::Float(local) => (F::Float, local.0),
            Self::String(local) => (F::String, local.0),
            Self::BitArray(local) => (F::BitArray, local.0),
            Self::UtfCodepoint(local) => (F::UtfCodepoint, local.0),
            Self::Custom(local) => (F::Custom, local.id().0),
            Self::External(local) => (F::External, local.id().0),
            Self::Bool(local) => (F::Bool, local.0),
            Self::Tuple { local, .. } => (F::Tuple, local.0),
            Self::Nil(_) => return None,
            Self::List(list) => match list {
                ListLocal::Parameter { local, .. } => (F::ParameterList, local.0),
                ListLocal::ParameterList { local, .. } => (F::ParameterListList, local.0),
                ListLocal::Int { local, .. } => (F::IntList, local.0),
                ListLocal::String { local, .. } => (F::StringList, local.0),
                ListLocal::BitArray { local, .. } => (F::BitArrayList, local.0),
                ListLocal::UtfCodepoint { local, .. } => (F::UtfCodepointList, local.0),
                ListLocal::Custom { local, .. } => (F::CustomList, local.0),
                ListLocal::External { local, .. } => (F::ExternalList, local.0),
                ListLocal::Float { local, .. } => (F::FloatList, local.0),
                ListLocal::Bool { local, .. } => (F::BoolList, local.0),
                ListLocal::Nil { local, .. } => (F::NilList, local.0),
                ListLocal::Tuple { local, .. } => (F::TupleList, local.0),
                ListLocal::List { local, .. } => (F::ListList, local.0),
                ListLocal::Function { local, .. } => (F::FunctionList, local.0),
            },
            Self::IntFunction { local, .. } => (F::IntFunction, local.0),
            Self::FloatFunction { local, .. } => (F::FloatFunction, local.0),
            Self::StringFunction { local, .. } => (F::StringFunction, local.0),
            Self::BitArrayFunction { local, .. } => (F::BitArrayFunction, local.0),
            Self::UtfCodepointFunction { local, .. } => (F::UtfCodepointFunction, local.0),
            Self::CustomFunction(local) => (F::CustomFunction, local.id().0),
            Self::ExternalFunction(local) => (F::ExternalFunction, local.id().0),
            Self::BoolFunction { local, .. } => (F::BoolFunction, local.0),
            Self::NilFunction { local, .. } => (F::NilFunction, local.0),
            Self::TupleFunction { local, .. } => (F::TupleFunction, local.0),
            Self::GenericFunction(local) => (F::GenericFunction, local.id().0),
            Self::NeverFunction(local) => (F::NeverFunction, local.id().0),
            Self::ListFunction(function) => match function {
                ListFunctionLocal::Parameter { local, .. } => (F::ParameterListFunction, local.0),
                ListFunctionLocal::ParameterList { local, .. } => {
                    (F::ParameterListListFunction, local.0)
                }
                ListFunctionLocal::Int { local, .. } => (F::IntListFunction, local.0),
                ListFunctionLocal::String { local, .. } => (F::StringListFunction, local.0),
                ListFunctionLocal::BitArray { local, .. } => (F::BitArrayListFunction, local.0),
                ListFunctionLocal::UtfCodepoint { local, .. } => {
                    (F::UtfCodepointListFunction, local.0)
                }
                ListFunctionLocal::Custom { local, .. } => (F::CustomListFunction, local.0),
                ListFunctionLocal::External { local, .. } => (F::ExternalListFunction, local.0),
                ListFunctionLocal::Float { local, .. } => (F::FloatListFunction, local.0),
                ListFunctionLocal::Bool { local, .. } => (F::BoolListFunction, local.0),
                ListFunctionLocal::Nil { local, .. } => (F::NilListFunction, local.0),
                ListFunctionLocal::Tuple { local, .. } => (F::TupleListFunction, local.0),
                ListFunctionLocal::List { local, .. } => (F::ListListFunction, local.0),
                ListFunctionLocal::Function { local, .. } => (F::FunctionListFunction, local.0),
            },
            Self::FunctionFunction(function) => match function {
                FunctionFunctionLocal::Core(local) => (F::CoreFunctionFunction, local.id().0),
                FunctionFunctionLocal::External(local) => {
                    (F::ExternalFunctionFunction, local.id().0)
                }
            },
        };
        Some(StorageSlot { family, index })
    }
}

impl Emit for Transfer {
    fn emit(&self, output: &mut Rust) {
        output.structure("graph::Transfer", &[("families", &self.families)]);
    }
}

impl Emit for FamilyTransfer {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "graph::FamilyTransfer",
            &[("family", &self.family), ("positions", &self.positions)],
        );
    }
}

impl Emit for StorageFamily {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::Int => "graph::StorageFamily::Int",
            Self::Float => "graph::StorageFamily::Float",
            Self::String => "graph::StorageFamily::String",
            Self::BitArray => "graph::StorageFamily::BitArray",
            Self::UtfCodepoint => "graph::StorageFamily::UtfCodepoint",
            Self::Custom => "graph::StorageFamily::Custom",
            Self::External => "graph::StorageFamily::External",
            Self::Bool => "graph::StorageFamily::Bool",
            Self::Tuple => "graph::StorageFamily::Tuple",
            Self::ParameterList => "graph::StorageFamily::ParameterList",
            Self::ParameterListList => "graph::StorageFamily::ParameterListList",
            Self::IntList => "graph::StorageFamily::IntList",
            Self::StringList => "graph::StorageFamily::StringList",
            Self::BitArrayList => "graph::StorageFamily::BitArrayList",
            Self::UtfCodepointList => "graph::StorageFamily::UtfCodepointList",
            Self::CustomList => "graph::StorageFamily::CustomList",
            Self::ExternalList => "graph::StorageFamily::ExternalList",
            Self::FloatList => "graph::StorageFamily::FloatList",
            Self::BoolList => "graph::StorageFamily::BoolList",
            Self::NilList => "graph::StorageFamily::NilList",
            Self::TupleList => "graph::StorageFamily::TupleList",
            Self::ListList => "graph::StorageFamily::ListList",
            Self::FunctionList => "graph::StorageFamily::FunctionList",
            Self::IntFunction => "graph::StorageFamily::IntFunction",
            Self::FloatFunction => "graph::StorageFamily::FloatFunction",
            Self::StringFunction => "graph::StorageFamily::StringFunction",
            Self::BitArrayFunction => "graph::StorageFamily::BitArrayFunction",
            Self::UtfCodepointFunction => "graph::StorageFamily::UtfCodepointFunction",
            Self::CustomFunction => "graph::StorageFamily::CustomFunction",
            Self::ExternalFunction => "graph::StorageFamily::ExternalFunction",
            Self::BoolFunction => "graph::StorageFamily::BoolFunction",
            Self::NilFunction => "graph::StorageFamily::NilFunction",
            Self::TupleFunction => "graph::StorageFamily::TupleFunction",
            Self::GenericFunction => "graph::StorageFamily::GenericFunction",
            Self::NeverFunction => "graph::StorageFamily::NeverFunction",
            Self::ParameterListFunction => "graph::StorageFamily::ParameterListFunction",
            Self::ParameterListListFunction => "graph::StorageFamily::ParameterListListFunction",
            Self::IntListFunction => "graph::StorageFamily::IntListFunction",
            Self::StringListFunction => "graph::StorageFamily::StringListFunction",
            Self::BitArrayListFunction => "graph::StorageFamily::BitArrayListFunction",
            Self::UtfCodepointListFunction => "graph::StorageFamily::UtfCodepointListFunction",
            Self::CustomListFunction => "graph::StorageFamily::CustomListFunction",
            Self::ExternalListFunction => "graph::StorageFamily::ExternalListFunction",
            Self::FloatListFunction => "graph::StorageFamily::FloatListFunction",
            Self::BoolListFunction => "graph::StorageFamily::BoolListFunction",
            Self::NilListFunction => "graph::StorageFamily::NilListFunction",
            Self::TupleListFunction => "graph::StorageFamily::TupleListFunction",
            Self::ListListFunction => "graph::StorageFamily::ListListFunction",
            Self::FunctionListFunction => "graph::StorageFamily::FunctionListFunction",
            Self::CoreFunctionFunction => "graph::StorageFamily::CoreFunctionFunction",
            Self::ExternalFunctionFunction => "graph::StorageFamily::ExternalFunctionFunction",
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{FamilyTransfer, StorageFamily, Transfer};
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_each_concrete_storage_family_without_erasing_its_identity() {
        let cases = [
            (StorageFamily::Int, "data::graph::StorageFamily::Int"),
            (StorageFamily::Float, "data::graph::StorageFamily::Float"),
            (StorageFamily::String, "data::graph::StorageFamily::String"),
            (
                StorageFamily::BitArray,
                "data::graph::StorageFamily::BitArray",
            ),
            (
                StorageFamily::UtfCodepoint,
                "data::graph::StorageFamily::UtfCodepoint",
            ),
            (StorageFamily::Custom, "data::graph::StorageFamily::Custom"),
            (
                StorageFamily::External,
                "data::graph::StorageFamily::External",
            ),
            (StorageFamily::Bool, "data::graph::StorageFamily::Bool"),
            (StorageFamily::Tuple, "data::graph::StorageFamily::Tuple"),
            (
                StorageFamily::ParameterList,
                "data::graph::StorageFamily::ParameterList",
            ),
            (
                StorageFamily::ParameterListList,
                "data::graph::StorageFamily::ParameterListList",
            ),
            (
                StorageFamily::IntList,
                "data::graph::StorageFamily::IntList",
            ),
            (
                StorageFamily::StringList,
                "data::graph::StorageFamily::StringList",
            ),
            (
                StorageFamily::BitArrayList,
                "data::graph::StorageFamily::BitArrayList",
            ),
            (
                StorageFamily::UtfCodepointList,
                "data::graph::StorageFamily::UtfCodepointList",
            ),
            (
                StorageFamily::CustomList,
                "data::graph::StorageFamily::CustomList",
            ),
            (
                StorageFamily::ExternalList,
                "data::graph::StorageFamily::ExternalList",
            ),
            (
                StorageFamily::FloatList,
                "data::graph::StorageFamily::FloatList",
            ),
            (
                StorageFamily::BoolList,
                "data::graph::StorageFamily::BoolList",
            ),
            (
                StorageFamily::NilList,
                "data::graph::StorageFamily::NilList",
            ),
            (
                StorageFamily::TupleList,
                "data::graph::StorageFamily::TupleList",
            ),
            (
                StorageFamily::ListList,
                "data::graph::StorageFamily::ListList",
            ),
            (
                StorageFamily::FunctionList,
                "data::graph::StorageFamily::FunctionList",
            ),
            (
                StorageFamily::IntFunction,
                "data::graph::StorageFamily::IntFunction",
            ),
            (
                StorageFamily::FloatFunction,
                "data::graph::StorageFamily::FloatFunction",
            ),
            (
                StorageFamily::StringFunction,
                "data::graph::StorageFamily::StringFunction",
            ),
            (
                StorageFamily::BitArrayFunction,
                "data::graph::StorageFamily::BitArrayFunction",
            ),
            (
                StorageFamily::UtfCodepointFunction,
                "data::graph::StorageFamily::UtfCodepointFunction",
            ),
            (
                StorageFamily::CustomFunction,
                "data::graph::StorageFamily::CustomFunction",
            ),
            (
                StorageFamily::ExternalFunction,
                "data::graph::StorageFamily::ExternalFunction",
            ),
            (
                StorageFamily::BoolFunction,
                "data::graph::StorageFamily::BoolFunction",
            ),
            (
                StorageFamily::NilFunction,
                "data::graph::StorageFamily::NilFunction",
            ),
            (
                StorageFamily::TupleFunction,
                "data::graph::StorageFamily::TupleFunction",
            ),
            (
                StorageFamily::GenericFunction,
                "data::graph::StorageFamily::GenericFunction",
            ),
            (
                StorageFamily::NeverFunction,
                "data::graph::StorageFamily::NeverFunction",
            ),
            (
                StorageFamily::ParameterListFunction,
                "data::graph::StorageFamily::ParameterListFunction",
            ),
            (
                StorageFamily::ParameterListListFunction,
                "data::graph::StorageFamily::ParameterListListFunction",
            ),
            (
                StorageFamily::IntListFunction,
                "data::graph::StorageFamily::IntListFunction",
            ),
            (
                StorageFamily::StringListFunction,
                "data::graph::StorageFamily::StringListFunction",
            ),
            (
                StorageFamily::BitArrayListFunction,
                "data::graph::StorageFamily::BitArrayListFunction",
            ),
            (
                StorageFamily::UtfCodepointListFunction,
                "data::graph::StorageFamily::UtfCodepointListFunction",
            ),
            (
                StorageFamily::CustomListFunction,
                "data::graph::StorageFamily::CustomListFunction",
            ),
            (
                StorageFamily::ExternalListFunction,
                "data::graph::StorageFamily::ExternalListFunction",
            ),
            (
                StorageFamily::FloatListFunction,
                "data::graph::StorageFamily::FloatListFunction",
            ),
            (
                StorageFamily::BoolListFunction,
                "data::graph::StorageFamily::BoolListFunction",
            ),
            (
                StorageFamily::NilListFunction,
                "data::graph::StorageFamily::NilListFunction",
            ),
            (
                StorageFamily::TupleListFunction,
                "data::graph::StorageFamily::TupleListFunction",
            ),
            (
                StorageFamily::ListListFunction,
                "data::graph::StorageFamily::ListListFunction",
            ),
            (
                StorageFamily::FunctionListFunction,
                "data::graph::StorageFamily::FunctionListFunction",
            ),
            (
                StorageFamily::CoreFunctionFunction,
                "data::graph::StorageFamily::CoreFunctionFunction",
            ),
            (
                StorageFamily::ExternalFunctionFunction,
                "data::graph::StorageFamily::ExternalFunctionFunction",
            ),
        ];
        for (family, expected) in cases {
            assert_eq!(Rust::expression(&family), expected);
        }
    }

    #[test]
    fn emits_the_bounded_positions_attached_to_a_complete_transfer() {
        let transfer = Transfer {
            families: vec![FamilyTransfer {
                family: StorageFamily::Int,
                positions: vec![2, 2, 0].into(),
            }]
            .into(),
        };
        assert_eq!(
            Rust::expression(&transfer),
            r#"
data::graph::Transfer {
    families: data::Storage::Static(&[
        data::graph::FamilyTransfer {
            family: data::graph::StorageFamily::Int,
            positions: data::Storage::Static(&[
                2,
                2,
                0,
            ]),
        },
    ]),
}"#
            .trim_start_matches('\n')
        );
    }
}
