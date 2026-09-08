use super::{ListFunctionId, ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId};
use crate::plan::execution::graph::{ExternalFunctionTarget, FunctionTarget, ParamLocal};
use std::collections::HashMap;
use std::convert::Infallible;

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

impl FunctionLocation {
    pub(crate) fn new(family: FunctionTableFamily, index: usize) -> Self {
        Self { family, index }
    }
}

pub(crate) struct FunctionParameterCatalog {
    tables: HashMap<FunctionTableFamily, Box<[Box<[ParamLocal]>]>>,
}

impl FunctionParameterCatalog {
    pub(in crate::plan::execution) fn new(
        entries: Vec<(FunctionTableFamily, usize, Box<[ParamLocal]>)>,
    ) -> Self {
        let mut grouped = HashMap::<_, Vec<_>>::new();
        for (family, index, parameters) in entries {
            grouped.entry(family).or_default().push((index, parameters));
        }
        let tables = grouped
            .into_iter()
            .map(|(family, mut entries)| {
                entries.sort_by_key(|(index, _)| *index);
                let parameters = entries
                    .into_iter()
                    .map(|(_, parameters)| parameters)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                (family, parameters)
            })
            .collect();
        Self { tables }
    }

    pub(crate) fn function(&self, target: &FunctionTarget) -> &[ParamLocal] {
        let Some(location) = function_target_location(target) else {
            return &[];
        };
        &self.tables[&location.family][location.index]
    }

    pub(crate) fn external_function(&self, target: &ExternalFunctionTarget) -> &[ParamLocal] {
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
        &self.tables[&location.family][location.index]
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

#[cfg(test)]
mod tests {
    use super::{
        FunctionTableFamily, ProfiledListFunctionFunctionId, list_function_function_location,
    };
    use crate::plan::execution::function::{
        BitArrayListFunctionFunctionId, BoolListFunctionFunctionId, CustomListFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionListFunctionFunctionId, ListListFunctionFunctionId,
        NilListFunctionFunctionId, ParameterListFunctionFunctionId,
        ParameterListListFunctionFunctionId, StringListFunctionFunctionId,
        TupleListFunctionFunctionId, UtfCodepointListFunctionFunctionId,
    };
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
        FunctionListTypeId, FunctionType, ListListTypeId, ListTypeId, NilListTypeId,
        ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
        UtfCodepointListTypeId, ValueType,
    };
    use std::convert::Infallible;

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
