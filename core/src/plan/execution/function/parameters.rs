use super::{ListFunctionId, ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId};
use crate::plan::execution::graph::{
    ExternalFunctionTarget, FunctionTarget, ParamLocal, ParamSlot,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::ValueShapeId;
use std::convert::Infallible;
use std::ops::Range;

pub struct FunctionCatalog {
    pub families: [Range<usize>; FunctionTableFamily::COUNT],
    pub functions: Table<FunctionContract>,
    pub parameters: Table<ParamLocal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionContract {
    pub parameters: Range<usize>,
    pub parameter_shapes: Table<ValueShapeId>,
    pub return_: ValueShapeId,
    pub captures: Table<ParamSlot>,
}

pub(in crate::plan::execution) struct FunctionCatalogEntry {
    pub family: FunctionTableFamily,
    pub index: usize,
    pub parameters: Vec<ParamSlot>,
    pub return_: ValueShapeId,
    pub captures: Vec<ParamSlot>,
}

#[derive(Clone, Copy)]
pub(crate) struct FunctionParameterView<'parameters> {
    families: &'parameters [Range<usize>; FunctionTableFamily::COUNT],
    functions: &'parameters [FunctionContract],
    parameters: &'parameters [ParamLocal],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum FunctionTableFamily {
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Bool,
    Nil,
    Tuple,
    ParameterList,
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
    ParameterListList,
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
    FunctionFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FunctionLocation {
    family: FunctionTableFamily,
    index: usize,
}

impl FunctionCatalog {
    pub(in crate::plan::execution) fn new(mut entries: Vec<FunctionCatalogEntry>) -> Self {
        entries.sort_by_key(|entry| (entry.family, entry.index));
        let mut families = std::array::from_fn(|_| 0..0);
        let mut functions = Vec::with_capacity(entries.len());
        let mut parameters = Vec::new();
        for FunctionCatalogEntry {
            family,
            index: _,
            parameters: inputs,
            return_,
            captures,
        } in entries
        {
            let range = &mut families[family as usize];
            if range.start == range.end {
                range.start = functions.len();
            }
            range.end = functions.len() + 1;
            let start = parameters.len();
            let mut parameter_shapes = Vec::with_capacity(inputs.len());
            for ParamSlot { local, shape } in inputs {
                parameters.push(local);
                parameter_shapes.push(shape);
            }
            functions.push(FunctionContract {
                parameters: start..parameters.len(),
                parameter_shapes: parameter_shapes.into(),
                return_,
                captures: captures.into(),
            });
        }
        Self {
            families,
            functions: functions.into(),
            parameters: parameters.into(),
        }
    }

    pub(crate) fn as_view(&self) -> FunctionParameterView<'_> {
        FunctionParameterView {
            families: &self.families,
            functions: &self.functions,
            parameters: &self.parameters,
        }
    }
}

impl<'parameters> FunctionParameterView<'parameters> {
    pub(crate) fn function(self, target: &FunctionTarget) -> &'parameters [ParamLocal] {
        let Some(location) = function_target_location(target) else {
            return &[];
        };
        self.at(location)
    }

    pub(crate) fn external_function(
        self,
        target: &ExternalFunctionTarget,
    ) -> &'parameters [ParamLocal] {
        let location = match target {
            ExternalFunctionTarget::Value(function) => {
                FunctionLocation::new(FunctionTableFamily::External, function.index())
            }
            ExternalFunctionTarget::List(function) => {
                FunctionLocation::new(FunctionTableFamily::ExternalList, function.index())
            }
            ExternalFunctionTarget::Function(function) => {
                FunctionLocation::new(FunctionTableFamily::ExternalFunction, function.index())
            }
            ExternalFunctionTarget::ListFunction { id, .. } => {
                FunctionLocation::new(FunctionTableFamily::ExternalListFunction, id.0)
            }
        };
        self.at(location)
    }

    fn at(self, location: FunctionLocation) -> &'parameters [ParamLocal] {
        let functions = &self.functions[self.families[location.family as usize].clone()];
        &self.parameters[functions[location.index].parameters.clone()]
    }
}

impl FunctionTableFamily {
    pub(in crate::plan::execution) const COUNT: usize = Self::FunctionFunction as usize + 1;
}

impl FunctionLocation {
    fn new(family: FunctionTableFamily, index: usize) -> Self {
        Self { family, index }
    }
}

fn function_target_location(target: &FunctionTarget) -> Option<FunctionLocation> {
    use FunctionTableFamily as F;

    let (family, index) = match target {
        FunctionTarget::Generic(_) => return None,
        FunctionTarget::Never(function) => (F::Never, function.0),
        FunctionTarget::Int(function) => (F::Int, function.0),
        FunctionTarget::Float(function) => (F::Float, function.0),
        FunctionTarget::String(function) => (F::String, function.0),
        FunctionTarget::BitArray(function) => (F::BitArray, function.0),
        FunctionTarget::UtfCodepoint(function) => (F::UtfCodepoint, function.0),
        FunctionTarget::Custom(function) => (F::Custom, function.index()),
        FunctionTarget::Bool(function) => (F::Bool, function.0),
        FunctionTarget::Nil(function) => (F::Nil, function.0),
        FunctionTarget::Tuple(function) => (F::Tuple, function.0),
        FunctionTarget::List(function) => list_function_location(function),
        FunctionTarget::Function(function) => function_function_location(function),
    };
    Some(FunctionLocation::new(family, index))
}

fn list_function_location(function: &ListFunctionId) -> (FunctionTableFamily, usize) {
    use FunctionTableFamily as F;

    match function {
        ListFunctionId::Parameter(function) => (F::ParameterList, function.index()),
        ListFunctionId::ParameterList(function) => (F::ParameterListList, function.index()),
        ListFunctionId::Int(function) => (F::IntList, function.index()),
        ListFunctionId::String(function) => (F::StringList, function.index()),
        ListFunctionId::BitArray(function) => (F::BitArrayList, function.index()),
        ListFunctionId::UtfCodepoint(function) => (F::UtfCodepointList, function.index()),
        ListFunctionId::Custom(function) => (F::CustomList, function.index()),
        ListFunctionId::Float(function) => (F::FloatList, function.index()),
        ListFunctionId::Bool(function) => (F::BoolList, function.index()),
        ListFunctionId::Nil(function) => (F::NilList, function.index()),
        ListFunctionId::Tuple(function) => (F::TupleList, function.index()),
        ListFunctionId::List(function) => (F::ListList, function.index()),
        ListFunctionId::Function(function) => (F::FunctionList, function.index()),
    }
}

fn function_function_location(
    function: &ProfiledFunctionFunctionId<Infallible>,
) -> (FunctionTableFamily, usize) {
    use FunctionTableFamily as F;
    use ProfiledFunctionFunctionId as R;

    match function {
        R::Generic(function) => (F::GenericFunction, function.index()),
        R::Never(function) => (F::NeverFunction, function.index()),
        R::Int(function) => (F::IntFunction, function.0),
        R::Float(function) => (F::FloatFunction, function.0),
        R::String(function) => (F::StringFunction, function.0),
        R::BitArray(function) => (F::BitArrayFunction, function.0),
        R::UtfCodepoint(function) => (F::UtfCodepointFunction, function.0),
        R::Custom(function) => (F::CustomFunction, function.index()),
        R::External(function) => match *function {},
        R::Bool(function) => (F::BoolFunction, function.0),
        R::Nil(function) => (F::NilFunction, function.0),
        R::Tuple(function) => (F::TupleFunction, function.0),
        R::List(function) => list_function_function_location(function),
        R::Function(function) => (F::FunctionFunction, function.index()),
    }
}

fn list_function_function_location(
    function: &ProfiledListFunctionFunctionId<Infallible>,
) -> (FunctionTableFamily, usize) {
    use FunctionTableFamily as F;
    use ProfiledListFunctionFunctionId as R;

    match function {
        R::Parameter { id, .. } => (F::ParameterListFunction, id.0),
        R::ParameterList { id, .. } => (F::ParameterListListFunction, id.0),
        R::Int { id, .. } => (F::IntListFunction, id.0),
        R::String { id, .. } => (F::StringListFunction, id.0),
        R::BitArray { id, .. } => (F::BitArrayListFunction, id.0),
        R::UtfCodepoint { id, .. } => (F::UtfCodepointListFunction, id.0),
        R::Custom { id, .. } => (F::CustomListFunction, id.0),
        R::External { id, .. } => match *id {},
        R::Float { id, .. } => (F::FloatListFunction, id.0),
        R::Bool { id, .. } => (F::BoolListFunction, id.0),
        R::Nil { id, .. } => (F::NilListFunction, id.0),
        R::Tuple { id, .. } => (F::TupleListFunction, id.0),
        R::List { id, .. } => (F::ListListFunction, id.0),
        R::Function { id, .. } => (F::FunctionListFunction, id.0),
    }
}

impl Emit for FunctionContract {
    fn emit(&self, output: &mut Rust) {
        let Self {
            parameters,
            parameter_shapes,
            return_,
            captures,
        } = self;
        output.structure(
            "function::FunctionContract",
            &[
                ("parameters", parameters),
                ("parameter_shapes", parameter_shapes),
                ("return_", return_),
                ("captures", captures),
            ],
        );
    }
}

impl Emit for FunctionCatalog {
    fn emit(&self, output: &mut Rust) {
        let Self {
            families,
            functions,
            parameters,
        } = self;
        output.structure(
            "function::FunctionCatalog",
            &[
                ("families", families),
                ("functions", functions),
                ("parameters", parameters),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionCatalogEntry;
    use super::{
        FunctionCatalog, FunctionContract, FunctionParameterView, FunctionTableFamily,
        ProfiledListFunctionFunctionId, list_function_function_location,
    };
    use crate::plan::execution::function::{
        BitArrayListFunctionFunctionId, BoolListFunctionFunctionId, CustomListFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionListFunctionFunctionId, ListListFunctionFunctionId,
        NilListFunctionFunctionId, ParameterListFunctionFunctionId,
        ParameterListListFunctionFunctionId, StringListFunctionFunctionId,
        TupleListFunctionFunctionId, UtfCodepointListFunctionFunctionId,
    };
    use crate::plan::execution::graph::ParamSlot;
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::ValueShapeId;
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
        FunctionListTypeId, FunctionType, ListListTypeId, ListTypeId, NilListTypeId,
        ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
        UtfCodepointListTypeId, ValueType,
    };
    use std::convert::Infallible;

    #[test]
    fn catalog_groups_and_orders_parameters_without_rebuilding_on_read() {
        use crate::plan::execution::function::{
            ExternalFunctionId, FloatFunctionId, IntFunctionId,
        };
        use crate::plan::execution::graph::{
            ExternalFunctionTarget, FloatLocalId, FunctionTarget, IntLocalId, ParamLocal,
        };
        use crate::plan::execution::type_::ExternalTypeId;

        let catalog = FunctionCatalog::new(vec![
            FunctionCatalogEntry {
                family: FunctionTableFamily::Int,
                index: 1,
                parameters: vec![],
                return_: ValueShapeId(0),
                captures: vec![ParamSlot::new(
                    ParamLocal::Int(IntLocalId(1)),
                    ValueShapeId(1),
                )],
            },
            FunctionCatalogEntry {
                family: FunctionTableFamily::External,
                index: 0,
                parameters: vec![ParamSlot::new(
                    ParamLocal::Int(IntLocalId(3)),
                    ValueShapeId(0),
                )],
                return_: ValueShapeId(3),
                captures: Vec::new(),
            },
            FunctionCatalogEntry {
                family: FunctionTableFamily::Float,
                index: 0,
                parameters: vec![ParamSlot::new(
                    ParamLocal::Float(FloatLocalId(1)),
                    ValueShapeId(2),
                )],
                return_: ValueShapeId(2),
                captures: Vec::new(),
            },
            FunctionCatalogEntry {
                family: FunctionTableFamily::Int,
                index: 0,
                parameters: vec![ParamSlot::new(
                    ParamLocal::Int(IntLocalId(2)),
                    ValueShapeId(0),
                )],
                return_: ValueShapeId(0),
                captures: Vec::new(),
            },
        ]);

        assert_eq!(catalog.families[FunctionTableFamily::Int as usize], 0..2);
        assert_eq!(catalog.families[FunctionTableFamily::Float as usize], 2..3);
        assert_eq!(
            catalog.families[FunctionTableFamily::External as usize],
            3..4
        );
        assert_eq!(catalog.families[FunctionTableFamily::Never as usize], 0..0);
        assert_eq!(
            &*catalog.functions,
            &[
                FunctionContract {
                    parameters: 0..1,
                    parameter_shapes: Table::Static(&[ValueShapeId(0)]),
                    return_: ValueShapeId(0),
                    captures: Table::Static(&[])
                },
                FunctionContract {
                    parameters: 1..1,
                    parameter_shapes: Table::Static(&[]),
                    return_: ValueShapeId(0),
                    captures: Table::Static(&[ParamSlot {
                        local: ParamLocal::Int(IntLocalId(1)),
                        shape: ValueShapeId(1)
                    }])
                },
                FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: Table::Static(&[ValueShapeId(2)]),
                    return_: ValueShapeId(2),
                    captures: Table::Static(&[])
                },
                FunctionContract {
                    parameters: 2..3,
                    parameter_shapes: Table::Static(&[ValueShapeId(0)]),
                    return_: ValueShapeId(3),
                    captures: Table::Static(&[])
                },
            ]
        );
        assert_eq!(
            &*catalog.parameters,
            &[
                ParamLocal::Int(IntLocalId(2)),
                ParamLocal::Float(FloatLocalId(1)),
                ParamLocal::Int(IntLocalId(3)),
            ]
        );

        let view = catalog.as_view();
        let parameters = view.function(&FunctionTarget::Int(IntFunctionId(0)));
        assert_eq!(parameters, &[ParamLocal::Int(IntLocalId(2))]);
        assert!(std::ptr::eq(
            parameters.as_ptr(),
            catalog.parameters.as_ptr()
        ));
        assert_eq!(view.function(&FunctionTarget::Int(IntFunctionId(1))), &[]);
        assert_eq!(
            view.function(&FunctionTarget::Float(FloatFunctionId(0))),
            &[ParamLocal::Float(FloatLocalId(1))]
        );
        assert_eq!(
            view.external_function(&ExternalFunctionTarget::Value(ExternalFunctionId::new(
                0,
                ExternalTypeId::new(0),
            ))),
            &[ParamLocal::Int(IntLocalId(3))]
        );
    }

    #[test]
    fn parameter_view_reads_static_arrays_without_owned_catalog() {
        use crate::plan::execution::function::{IntFunctionId, NilFunctionId};
        use crate::plan::execution::graph::{BoolLocalId, FunctionTarget, IntLocalId, ParamLocal};
        use std::ops::Range;

        static PARAMETERS: [ParamLocal; 2] = [
            ParamLocal::Int(IntLocalId(0)),
            ParamLocal::Bool(BoolLocalId(0)),
        ];
        static FUNCTIONS: [FunctionContract; 2] = [
            FunctionContract {
                parameters: 0..2,
                parameter_shapes: Table::Static(&[ValueShapeId(0), ValueShapeId(1)]),
                return_: ValueShapeId(0),
                captures: Table::Static(&[]),
            },
            FunctionContract {
                parameters: 2..2,
                parameter_shapes: Table::Static(&[]),
                return_: ValueShapeId(3),
                captures: Table::Static(&[]),
            },
        ];
        static FAMILIES: [Range<usize>; FunctionTableFamily::COUNT] = {
            let mut families = [const { 0..0 }; FunctionTableFamily::COUNT];
            families[FunctionTableFamily::Int as usize] = 0..1;
            families[FunctionTableFamily::Nil as usize] = 1..2;
            families
        };

        let view = FunctionParameterView {
            families: &FAMILIES,
            functions: &FUNCTIONS,
            parameters: &PARAMETERS,
        };
        let copied = Clone::clone(&view);
        let inputs = copied.function(&FunctionTarget::Int(IntFunctionId(0)));
        assert_eq!(
            inputs,
            &[
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Bool(BoolLocalId(0)),
            ]
        );
        assert!(std::ptr::eq(inputs.as_ptr(), PARAMETERS.as_ptr()));
        assert_eq!(copied.function(&FunctionTarget::Nil(NilFunctionId(0))), &[]);
    }

    #[test]
    fn empty_catalog_keeps_symbolic_function_parameters_absent() {
        use crate::plan::execution::function::GenericCallableId;
        use crate::plan::execution::graph::FunctionTarget;

        let catalog = FunctionCatalog::new(Vec::new());
        let symbolic = FunctionTarget::Generic(GenericCallableId::function(0, Vec::new()));
        assert_eq!(catalog.as_view().function(&symbolic), &[]);
        assert!(catalog.functions.is_empty());
        assert!(catalog.parameters.is_empty());
        assert!(catalog.families.iter().all(|range| range == &(0..0)));
    }

    #[test]
    fn list_function_locations_preserve_parameter_and_concrete_tail_families() {
        let list_type = ListTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::List(list_type));
        let cases = [
            (
                ProfiledListFunctionFunctionId::<Infallible>::BitArray {
                    id: BitArrayListFunctionFunctionId(13),
                    type_: function_type.clone(),
                    list_type: BitArrayListTypeId::new(list_type),
                },
                (FunctionTableFamily::BitArrayListFunction, 13),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::UtfCodepoint {
                    id: UtfCodepointListFunctionFunctionId(14),
                    type_: function_type.clone(),
                    list_type: UtfCodepointListTypeId::new(list_type),
                },
                (FunctionTableFamily::UtfCodepointListFunction, 14),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Parameter {
                    id: ParameterListFunctionFunctionId(10),
                    type_: function_type.clone(),
                    list_type: ParameterListTypeId::new(list_type, crate::plan::TypeParameterId(0)),
                },
                (FunctionTableFamily::ParameterListFunction, 10),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::ParameterList {
                    id: ParameterListListFunctionFunctionId(11),
                    type_: function_type.clone(),
                    list_type: ParameterListListTypeId::new(
                        list_type,
                        ParameterListTypeId::new(
                            ListTypeId::new(1),
                            crate::plan::TypeParameterId(0),
                        ),
                    ),
                },
                (FunctionTableFamily::ParameterListListFunction, 11),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::String {
                    id: StringListFunctionFunctionId(12),
                    type_: function_type.clone(),
                    list_type: StringListTypeId::new(list_type),
                },
                (FunctionTableFamily::StringListFunction, 12),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Custom {
                    id: CustomListFunctionFunctionId(2),
                    type_: function_type.clone(),
                    list_type: CustomListTypeId::new(list_type, CustomTypeId::new(0)),
                },
                (FunctionTableFamily::CustomListFunction, 2),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Float {
                    id: FloatListFunctionFunctionId(3),
                    type_: function_type.clone(),
                    list_type: FloatListTypeId::new(list_type),
                },
                (FunctionTableFamily::FloatListFunction, 3),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Bool {
                    id: BoolListFunctionFunctionId(4),
                    type_: function_type.clone(),
                    list_type: BoolListTypeId::new(list_type),
                },
                (FunctionTableFamily::BoolListFunction, 4),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Nil {
                    id: NilListFunctionFunctionId(5),
                    type_: function_type.clone(),
                    list_type: NilListTypeId::new(list_type),
                },
                (FunctionTableFamily::NilListFunction, 5),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Tuple {
                    id: TupleListFunctionFunctionId(6),
                    type_: function_type.clone(),
                    list_type: TupleListTypeId::new(list_type, 0),
                },
                (FunctionTableFamily::TupleListFunction, 6),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::List {
                    id: ListListFunctionFunctionId(7),
                    type_: function_type.clone(),
                    list_type: ListListTypeId::new(list_type, ListTypeId::new(1)),
                },
                (FunctionTableFamily::ListListFunction, 7),
            ),
            (
                ProfiledListFunctionFunctionId::<Infallible>::Function {
                    id: FunctionListFunctionFunctionId(8),
                    type_: function_type,
                    list_type: FunctionListTypeId::new(list_type, 0),
                },
                (FunctionTableFamily::FunctionListFunction, 8),
            ),
        ];

        for (target, expected) in cases {
            assert_eq!(list_function_function_location(&target), expected);
        }
    }
}
