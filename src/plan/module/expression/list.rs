mod case;
mod elements;
mod item;
mod local;
mod typed;

pub(crate) use self::{
    case::{BoolListCaseBranches, ListCaseBranches},
    elements::{
        ListElementTypeMismatch, ListElements, ListSpreadConstructionError, ListSpreadElements,
    },
    item::{
        BitArrayListItem, BoolListItem, CustomListItem, ExternalListItem, FloatListItem,
        FunctionListItem, GenericListItem, IntListItem, ListItem, ListListItem, NilListItem,
        ParameterListListItem, StringListItem, TupleListItem, UtfCodepointListItem,
    },
    local::ListLocalExpr,
    typed::{ListIndexSource, TypedListExpr, TypedListExprKind, TypedListReturnKind},
};
use super::{
    BoolExpr, CallArg, CustomFieldAccess, Expr, FloatExpr, IntExpr, ListFunctionExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::{ConstantListInstantiation, ListLocal, Step, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListExpr {
    Generic(GenericListExpr),
    ParameterList(ParameterListListExpr),
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Custom(CustomListExpr),
    External(ExternalListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StoredListExpr {
    ParameterList(ParameterListListExpr),
    Int(IntListExpr),
    String(StringListExpr),
    BitArray(BitArrayListExpr),
    UtfCodepoint(UtfCodepointListExpr),
    Custom(CustomListExpr),
    External(ExternalListExpr),
    Float(FloatListExpr),
    Bool(BoolListExpr),
    Nil(NilListExpr),
    Tuple(TupleListExpr),
    List(ListListExpr),
    Function(FunctionListExpr),
}

pub(crate) type GenericListExpr = TypedListExpr<GenericListItem>;
pub(crate) type ParameterListListExpr = TypedListExpr<ParameterListListItem>;

pub(crate) type IntListExpr = TypedListExpr<IntListItem>;
pub(crate) type StringListExpr = TypedListExpr<StringListItem>;
pub(crate) type BitArrayListExpr = TypedListExpr<BitArrayListItem>;
pub(crate) type UtfCodepointListExpr = TypedListExpr<UtfCodepointListItem>;
pub(crate) type CustomListExpr = TypedListExpr<CustomListItem>;
pub(crate) type ExternalListExpr = TypedListExpr<ExternalListItem>;
pub(crate) type FloatListExpr = TypedListExpr<FloatListItem>;
pub(crate) type BoolListExpr = TypedListExpr<BoolListItem>;
pub(crate) type NilListExpr = TypedListExpr<NilListItem>;
pub(crate) type TupleListExpr = TypedListExpr<TupleListItem>;
pub(crate) type ListListExpr = TypedListExpr<ListListItem>;
pub(crate) type FunctionListExpr = TypedListExpr<FunctionListItem>;

impl ListExpr {
    pub(in crate::plan::module) fn constant(reference: ConstantListInstantiation) -> Self {
        match reference {
            ConstantListInstantiation::Generic(reference) => {
                let parameter = *reference.item_shape();
                Self::Generic(GenericListExpr::constant(
                    crate::plan::ValueShape::Parameter(parameter),
                    GenericListItem::new(parameter),
                    reference,
                ))
            }
            ConstantListInstantiation::Int(reference) => Self::Int(IntListExpr::constant(
                crate::plan::ValueShape::Int,
                IntListItem,
                reference,
            )),
            ConstantListInstantiation::String(reference) => Self::String(StringListExpr::constant(
                crate::plan::ValueShape::String,
                StringListItem,
                reference,
            )),
            ConstantListInstantiation::BitArray(reference) => {
                Self::BitArray(BitArrayListExpr::constant(
                    crate::plan::ValueShape::BitArray,
                    BitArrayListItem,
                    reference,
                ))
            }
            ConstantListInstantiation::UtfCodepoint(reference) => {
                Self::UtfCodepoint(UtfCodepointListExpr::constant(
                    crate::plan::ValueShape::UtfCodepoint,
                    UtfCodepointListItem,
                    reference,
                ))
            }
            ConstantListInstantiation::Custom(reference) => {
                let shape = reference.item_shape().clone();
                Self::Custom(CustomListExpr::constant(
                    crate::plan::ValueShape::Custom(shape.clone()),
                    CustomListItem::new(shape.type_().clone()),
                    reference,
                ))
            }
            ConstantListInstantiation::External(reference) => {
                let shape = reference.item_shape().clone();
                Self::External(ExternalListExpr::constant(
                    crate::plan::ValueShape::External(shape.clone()),
                    ExternalListItem::new(shape.type_().clone()),
                    reference,
                ))
            }
            ConstantListInstantiation::Float(reference) => Self::Float(FloatListExpr::constant(
                crate::plan::ValueShape::Float,
                FloatListItem,
                reference,
            )),
            ConstantListInstantiation::Bool(reference) => Self::Bool(BoolListExpr::constant(
                crate::plan::ValueShape::Bool,
                BoolListItem,
                reference,
            )),
            ConstantListInstantiation::Nil(reference) => Self::Nil(NilListExpr::constant(
                crate::plan::ValueShape::Nil,
                NilListItem,
                reference,
            )),
            ConstantListInstantiation::Tuple(reference) => {
                let shape = reference.item_shape().clone();
                Self::Tuple(TupleListExpr::constant(
                    crate::plan::ValueShape::Tuple(shape.clone()),
                    TupleListItem::new(
                        shape
                            .iter()
                            .map(crate::plan::ValueShape::value_type)
                            .collect(),
                    ),
                    reference,
                ))
            }
            ConstantListInstantiation::ParameterList(reference) => {
                let parameter = *reference.item_shape();
                Self::ParameterList(ParameterListListExpr::constant(
                    crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Parameter(
                        parameter,
                    ))),
                    ParameterListListItem::new(parameter),
                    reference,
                ))
            }
            ConstantListInstantiation::List(reference) => {
                let shape = reference.item_shape().clone();
                Self::List(ListListExpr::constant(
                    crate::plan::ValueShape::List(Box::new(shape.to_value_shape())),
                    ListListItem::new(shape),
                    reference,
                ))
            }
            ConstantListInstantiation::Function(reference) => {
                let shape = reference.item_shape().clone();
                Self::Function(FunctionListExpr::constant(
                    crate::plan::ValueShape::Function(Box::new(shape.clone())),
                    FunctionListItem::new(shape.type_()),
                    reference,
                ))
            }
        }
    }

    pub(crate) fn try_value(
        elements: Vec<Expr>,
        element_type: ValueType,
    ) -> Result<Self, ListElementTypeMismatch> {
        let item_shape = list_item_shape(&elements, &element_type)?;
        ListElements::from_exprs(element_type, elements)
            .map(Self::from_elements)
            .map(|value| value.with_item_shape(item_shape))
    }

    pub(crate) fn from_elements(elements: ListElements) -> Self {
        match elements {
            ListElements::Generic { parameter, values } => Self::Generic(GenericListExpr::value(
                GenericListItem::new(parameter),
                values,
            )),
            ListElements::ParameterList { parameter, values } => Self::ParameterList(
                ParameterListListExpr::value(ParameterListListItem::new(parameter), values),
            ),
            ListElements::Int(values) => Self::Int(IntListExpr::value(IntListItem, values)),
            ListElements::String(values) => {
                Self::String(StringListExpr::value(StringListItem, values))
            }
            ListElements::BitArray(values) => {
                Self::BitArray(BitArrayListExpr::value(BitArrayListItem, values))
            }
            ListElements::UtfCodepoint(values) => {
                Self::UtfCodepoint(UtfCodepointListExpr::value(UtfCodepointListItem, values))
            }
            ListElements::Custom { item_type, values } => {
                Self::Custom(CustomListExpr::value(CustomListItem { item_type }, values))
            }
            ListElements::External { item_type, values } => Self::External(
                ExternalListExpr::value(ExternalListItem { item_type }, values),
            ),
            ListElements::Float(values) => Self::Float(FloatListExpr::value(FloatListItem, values)),
            ListElements::Bool(values) => Self::Bool(BoolListExpr::value(BoolListItem, values)),
            ListElements::Nil(values) => Self::Nil(NilListExpr::value(NilListItem, values)),
            ListElements::Tuple { item_type, values } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::value(item, values))
            }
            ListElements::List { item_shape, values } => {
                let item = ListListItem::new(item_shape);
                Self::List(ListListExpr::value(item, values))
            }
            ListElements::Function { item_type, values } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::value(item, values))
            }
        }
    }

    pub(crate) fn from_spread_elements(elements: ListSpreadElements) -> Self {
        match elements {
            ListSpreadElements::Generic { values, tail } => {
                Self::Generic(GenericListExpr::spread(values, tail))
            }
            ListSpreadElements::ParameterList { values, tail } => {
                Self::ParameterList(ParameterListListExpr::spread(values, tail))
            }
            ListSpreadElements::Int { values, tail } => {
                Self::Int(IntListExpr::spread(values, tail))
            }
            ListSpreadElements::String { values, tail } => {
                Self::String(StringListExpr::spread(values, tail))
            }
            ListSpreadElements::BitArray { values, tail } => {
                Self::BitArray(BitArrayListExpr::spread(values, tail))
            }
            ListSpreadElements::UtfCodepoint { values, tail } => {
                Self::UtfCodepoint(UtfCodepointListExpr::spread(values, tail))
            }
            ListSpreadElements::Custom { values, tail } => {
                Self::Custom(CustomListExpr::spread(values, tail))
            }
            ListSpreadElements::External { values, tail } => {
                Self::External(ExternalListExpr::spread(values, tail))
            }
            ListSpreadElements::Float { values, tail } => {
                Self::Float(FloatListExpr::spread(values, tail))
            }
            ListSpreadElements::Bool { values, tail } => {
                Self::Bool(BoolListExpr::spread(values, tail))
            }
            ListSpreadElements::Nil { values, tail } => {
                Self::Nil(NilListExpr::spread(values, tail))
            }
            ListSpreadElements::Tuple { values, tail } => {
                Self::Tuple(TupleListExpr::spread(values, tail))
            }
            ListSpreadElements::List { values, tail } => {
                Self::List(ListListExpr::spread(values, tail))
            }
            ListSpreadElements::Function { values, tail } => {
                Self::Function(FunctionListExpr::spread(values, tail))
            }
        }
    }

    pub(crate) fn local_get(local: ListLocal, name: EcoString) -> Self {
        match local {
            ListLocal::Generic { local, parameter } => Self::Generic(GenericListExpr::local_get(
                GenericListItem::new(parameter),
                local,
                name,
            )),
            ListLocal::Int(local) => Self::Int(IntListExpr::local_get(IntListItem, local, name)),
            ListLocal::String(local) => {
                Self::String(StringListExpr::local_get(StringListItem, local, name))
            }
            ListLocal::BitArray(local) => {
                Self::BitArray(BitArrayListExpr::local_get(BitArrayListItem, local, name))
            }
            ListLocal::UtfCodepoint(local) => Self::UtfCodepoint(UtfCodepointListExpr::local_get(
                UtfCodepointListItem,
                local,
                name,
            )),
            ListLocal::Custom { local, item_type } => Self::Custom(CustomListExpr::local_get(
                CustomListItem { item_type },
                local,
                name,
            )),
            ListLocal::External { local, item_type } => Self::External(
                ExternalListExpr::local_get(ExternalListItem { item_type }, local, name),
            ),
            ListLocal::Float(local) => {
                Self::Float(FloatListExpr::local_get(FloatListItem, local, name))
            }
            ListLocal::Bool(local) => {
                Self::Bool(BoolListExpr::local_get(BoolListItem, local, name))
            }
            ListLocal::Nil(local) => Self::Nil(NilListExpr::local_get(NilListItem, local, name)),
            ListLocal::Tuple { local, item_type } => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::local_get(item, local, name))
            }
            ListLocal::List { local, item_type } => {
                match crate::plan::ValueShape::from_value_type(*item_type).representation() {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(ParameterListListExpr::local_get(
                            ParameterListListItem::new(parameter),
                            local,
                            name,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(item_shape) => {
                        let item = ListListItem::new(item_shape);
                        Self::List(ListListExpr::local_get(item, local, name))
                    }
                }
            }
            ListLocal::Function { local, item_type } => {
                let item = FunctionListItem { item_type };
                Self::Function(FunctionListExpr::local_get(item, local, name))
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn call(
        function: crate::plan::FunctionInstantiation,
        args: Vec<CallArg>,
        item_shape: crate::plan::ValueShape,
    ) -> Self {
        Self::call_at(
            function,
            args,
            item_shape,
            crate::plan::HostCallSite::unknown(),
        )
    }

    pub(crate) fn call_at(
        function: crate::plan::FunctionInstantiation,
        args: Vec<CallArg>,
        item_shape: crate::plan::ValueShape,
        site: crate::plan::HostCallSite,
    ) -> Self {
        let expression = match item_shape.clone() {
            crate::plan::ValueShape::Parameter(parameter) => Self::Generic(
                GenericListExpr::call_at(GenericListItem::new(parameter), function, args, site),
            ),
            crate::plan::ValueShape::Int => {
                Self::Int(IntListExpr::call_at(IntListItem, function, args, site))
            }
            crate::plan::ValueShape::String => Self::String(StringListExpr::call_at(
                StringListItem,
                function,
                args,
                site,
            )),
            crate::plan::ValueShape::BitArray => Self::BitArray(BitArrayListExpr::call_at(
                BitArrayListItem,
                function,
                args,
                site,
            )),
            crate::plan::ValueShape::UtfCodepoint => Self::UtfCodepoint(
                UtfCodepointListExpr::call_at(UtfCodepointListItem, function, args, site),
            ),
            crate::plan::ValueShape::Custom(shape) => Self::Custom(CustomListExpr::call_at(
                CustomListItem {
                    item_type: shape.type_().clone(),
                },
                function,
                args,
                site,
            )),
            crate::plan::ValueShape::External(shape) => Self::External(ExternalListExpr::call_at(
                ExternalListItem {
                    item_type: shape.type_().clone(),
                },
                function,
                args,
                site,
            )),
            crate::plan::ValueShape::Float => {
                Self::Float(FloatListExpr::call_at(FloatListItem, function, args, site))
            }
            crate::plan::ValueShape::Bool => {
                Self::Bool(BoolListExpr::call_at(BoolListItem, function, args, site))
            }
            crate::plan::ValueShape::Nil => {
                Self::Nil(NilListExpr::call_at(NilListItem, function, args, site))
            }
            crate::plan::ValueShape::Tuple(shape) => Self::Tuple(TupleListExpr::call_at(
                TupleListItem {
                    item_type: shape
                        .iter()
                        .map(crate::plan::ValueShape::value_type)
                        .collect(),
                },
                function,
                args,
                site,
            )),
            crate::plan::ValueShape::List(shape) => match shape.representation() {
                crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                    Self::ParameterList(ParameterListListExpr::call_at(
                        ParameterListListItem::new(parameter),
                        function,
                        args,
                        site,
                    ))
                }
                crate::plan::ValueRepresentation::Stored(item_shape) => Self::List(
                    ListListExpr::call_at(ListListItem::new(item_shape), function, args, site),
                ),
            },
            crate::plan::ValueShape::Function(shape) => Self::Function(FunctionListExpr::call_at(
                FunctionListItem {
                    item_type: shape.type_(),
                },
                function,
                args,
                site,
            )),
        };
        expression.with_item_shape(item_shape)
    }

    #[cfg(test)]
    pub(crate) fn function_call(function: ListFunctionExpr, args: Vec<CallArg>) -> Self {
        Self::function_call_at(function, args, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn function_call_at(
        function: ListFunctionExpr,
        args: Vec<CallArg>,
        site: crate::plan::HostCallSite,
    ) -> Self {
        match function.return_item_type() {
            ValueType::Parameter(parameter) => Self::Generic(GenericListExpr::function_call_at(
                GenericListItem::new(parameter),
                function,
                args,
                site,
            )),
            ValueType::Int => Self::Int(IntListExpr::function_call_at(
                IntListItem,
                function,
                args,
                site,
            )),
            ValueType::String => Self::String(StringListExpr::function_call_at(
                StringListItem,
                function,
                args,
                site,
            )),
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::function_call_at(
                BitArrayListItem,
                function,
                args,
                site,
            )),
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::function_call_at(
                UtfCodepointListItem,
                function,
                args,
                site,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::function_call_at(
                CustomListItem { item_type },
                function,
                args,
                site,
            )),
            ValueType::External(item_type) => Self::External(ExternalListExpr::function_call_at(
                ExternalListItem { item_type },
                function,
                args,
                site,
            )),
            ValueType::Float => Self::Float(FloatListExpr::function_call_at(
                FloatListItem,
                function,
                args,
                site,
            )),
            ValueType::Bool => Self::Bool(BoolListExpr::function_call_at(
                BoolListItem,
                function,
                args,
                site,
            )),
            ValueType::Nil => Self::Nil(NilListExpr::function_call_at(
                NilListItem,
                function,
                args,
                site,
            )),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::function_call_at(item, function, args, site))
            }
            ValueType::List(item_type) => {
                match crate::plan::ValueShape::from_value_type(*item_type).representation() {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(ParameterListListExpr::function_call_at(
                            ParameterListListItem::new(parameter),
                            function,
                            args,
                            site,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(item_shape) => {
                        let item = ListListItem::new(item_shape);
                        Self::List(ListListExpr::function_call_at(item, function, args, site))
                    }
                }
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::function_call_at(
                    item, function, args, site,
                ))
            }
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Parameter(parameter) => Self::Generic(GenericListExpr::tuple_index(
                GenericListItem::new(parameter),
                tuple,
                index,
            )),
            ValueType::Int => Self::Int(IntListExpr::tuple_index(IntListItem, tuple, index)),
            ValueType::String => {
                Self::String(StringListExpr::tuple_index(StringListItem, tuple, index))
            }
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::tuple_index(
                BitArrayListItem,
                tuple,
                index,
            )),
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::tuple_index(
                UtfCodepointListItem,
                tuple,
                index,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::tuple_index(
                CustomListItem { item_type },
                tuple,
                index,
            )),
            ValueType::External(item_type) => Self::External(ExternalListExpr::tuple_index(
                ExternalListItem { item_type },
                tuple,
                index,
            )),
            ValueType::Float => {
                Self::Float(FloatListExpr::tuple_index(FloatListItem, tuple, index))
            }
            ValueType::Bool => Self::Bool(BoolListExpr::tuple_index(BoolListItem, tuple, index)),
            ValueType::Nil => Self::Nil(NilListExpr::tuple_index(NilListItem, tuple, index)),
            ValueType::Tuple(item_type) => {
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::tuple_index(item, tuple, index))
            }
            ValueType::List(item_type) => {
                match crate::plan::ValueShape::from_value_type(*item_type).representation() {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(ParameterListListExpr::tuple_index(
                            ParameterListListItem::new(parameter),
                            tuple,
                            index,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(item_shape) => {
                        let item = ListListItem::new(item_shape);
                        Self::List(ListListExpr::tuple_index(item, tuple, index))
                    }
                }
            }
            ValueType::Function(item_type) => {
                let item = FunctionListItem {
                    item_type: *item_type,
                };
                Self::Function(FunctionListExpr::tuple_index(item, tuple, index))
            }
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Parameter(parameter) => Self::Generic(GenericListExpr::custom_field(
                GenericListItem::new(parameter),
                access,
            )),
            ValueType::Int => Self::Int(IntListExpr::custom_field(IntListItem, access)),
            ValueType::String => Self::String(StringListExpr::custom_field(StringListItem, access)),
            ValueType::BitArray => {
                Self::BitArray(BitArrayListExpr::custom_field(BitArrayListItem, access))
            }
            ValueType::UtfCodepoint => Self::UtfCodepoint(UtfCodepointListExpr::custom_field(
                UtfCodepointListItem,
                access,
            )),
            ValueType::Custom(item_type) => Self::Custom(CustomListExpr::custom_field(
                CustomListItem { item_type },
                access,
            )),
            ValueType::External(item_type) => Self::External(ExternalListExpr::custom_field(
                ExternalListItem { item_type },
                access,
            )),
            ValueType::Float => Self::Float(FloatListExpr::custom_field(FloatListItem, access)),
            ValueType::Bool => Self::Bool(BoolListExpr::custom_field(BoolListItem, access)),
            ValueType::Nil => Self::Nil(NilListExpr::custom_field(NilListItem, access)),
            ValueType::Tuple(item_type) => Self::Tuple(TupleListExpr::custom_field(
                TupleListItem { item_type },
                access,
            )),
            ValueType::List(item_type) => {
                match crate::plan::ValueShape::from_value_type(*item_type).representation() {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(ParameterListListExpr::custom_field(
                            ParameterListListItem::new(parameter),
                            access,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(item_shape) => Self::List(
                        ListListExpr::custom_field(ListListItem::new(item_shape), access),
                    ),
                }
            }
            ValueType::Function(item_type) => Self::Function(FunctionListExpr::custom_field(
                FunctionListItem {
                    item_type: *item_type,
                },
                access,
            )),
        }
    }

    pub(crate) fn parameter_list_index(list: ParameterListListExpr, index: usize) -> Self {
        let parameter = list.item().parameter();
        Self::Generic(GenericListExpr::from_list_index(
            GenericListItem::new(parameter),
            ListIndexSource::new(list, index),
        ))
    }

    pub(crate) fn list_index(list: ListListExpr, index: usize) -> Self {
        match list.item().item_shape().clone() {
            crate::plan::ValueStorageShape::Int => Self::Int(IntListExpr::from_list_index(
                IntListItem,
                ListIndexSource::new(list, index),
            )),
            crate::plan::ValueStorageShape::String => Self::String(
                StringListExpr::from_list_index(StringListItem, ListIndexSource::new(list, index)),
            ),
            crate::plan::ValueStorageShape::BitArray => {
                Self::BitArray(BitArrayListExpr::from_list_index(
                    BitArrayListItem,
                    ListIndexSource::new(list, index),
                ))
            }
            crate::plan::ValueStorageShape::UtfCodepoint => {
                Self::UtfCodepoint(UtfCodepointListExpr::from_list_index(
                    UtfCodepointListItem,
                    ListIndexSource::new(list, index),
                ))
            }
            crate::plan::ValueStorageShape::Custom(shape) => {
                let item = CustomListItem {
                    item_type: shape.type_().clone(),
                };
                Self::Custom(CustomListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            crate::plan::ValueStorageShape::External(shape) => {
                let item = ExternalListItem {
                    item_type: shape.type_().clone(),
                };
                Self::External(ExternalListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            crate::plan::ValueStorageShape::Float => Self::Float(FloatListExpr::from_list_index(
                FloatListItem,
                ListIndexSource::new(list, index),
            )),
            crate::plan::ValueStorageShape::Bool => Self::Bool(BoolListExpr::from_list_index(
                BoolListItem,
                ListIndexSource::new(list, index),
            )),
            crate::plan::ValueStorageShape::Nil => Self::Nil(NilListExpr::from_list_index(
                NilListItem,
                ListIndexSource::new(list, index),
            )),
            crate::plan::ValueStorageShape::Tuple(item_shape) => {
                let item_type = item_shape
                    .iter()
                    .map(crate::plan::ValueShape::value_type)
                    .collect();
                let item = TupleListItem { item_type };
                Self::Tuple(TupleListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
            crate::plan::ValueStorageShape::List(item_shape) => match item_shape.representation() {
                crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                    Self::ParameterList(ParameterListListExpr::from_list_index(
                        ParameterListListItem::new(parameter),
                        ListIndexSource::new(list, index),
                    ))
                }
                crate::plan::ValueRepresentation::Stored(item_shape) => {
                    Self::List(ListListExpr::from_list_index(
                        ListListItem::new(item_shape),
                        ListIndexSource::new(list, index),
                    ))
                }
            },
            crate::plan::ValueStorageShape::Function(shape) => {
                let item = FunctionListItem {
                    item_type: shape.type_(),
                };
                Self::Function(FunctionListExpr::from_list_index(
                    item,
                    ListIndexSource::new(list, index),
                ))
            }
        }
    }

    pub(crate) fn drop_first(list: ListExpr, count: usize) -> Self {
        match list {
            Self::Generic(list) => Self::Generic(GenericListExpr::drop_first(list, count)),
            Self::ParameterList(list) => {
                Self::ParameterList(ParameterListListExpr::drop_first(list, count))
            }
            Self::Int(list) => Self::Int(IntListExpr::drop_first(list, count)),
            Self::String(list) => Self::String(StringListExpr::drop_first(list, count)),
            Self::BitArray(list) => Self::BitArray(BitArrayListExpr::drop_first(list, count)),
            Self::UtfCodepoint(list) => {
                Self::UtfCodepoint(UtfCodepointListExpr::drop_first(list, count))
            }
            Self::Custom(list) => Self::Custom(CustomListExpr::drop_first(list, count)),
            Self::External(list) => Self::External(ExternalListExpr::drop_first(list, count)),
            Self::Float(list) => Self::Float(FloatListExpr::drop_first(list, count)),
            Self::Bool(list) => Self::Bool(BoolListExpr::drop_first(list, count)),
            Self::Nil(list) => Self::Nil(NilListExpr::drop_first(list, count)),
            Self::Tuple(list) => Self::Tuple(TupleListExpr::drop_first(list, count)),
            Self::List(list) => Self::List(ListListExpr::drop_first(list, count)),
            Self::Function(list) => Self::Function(FunctionListExpr::drop_first(list, count)),
        }
    }

    pub(crate) fn panic(panic: PanicExpr, element_type: ValueType) -> Self {
        match element_type {
            ValueType::Parameter(parameter) => Self::Generic(GenericListExpr::panic(
                GenericListItem::new(parameter),
                panic,
            )),
            ValueType::Int => Self::Int(IntListExpr::panic(IntListItem, panic)),
            ValueType::String => Self::String(StringListExpr::panic(StringListItem, panic)),
            ValueType::BitArray => Self::BitArray(BitArrayListExpr::panic(BitArrayListItem, panic)),
            ValueType::UtfCodepoint => {
                Self::UtfCodepoint(UtfCodepointListExpr::panic(UtfCodepointListItem, panic))
            }
            ValueType::Custom(item_type) => {
                Self::Custom(CustomListExpr::panic(CustomListItem { item_type }, panic))
            }
            ValueType::External(item_type) => Self::External(ExternalListExpr::panic(
                ExternalListItem { item_type },
                panic,
            )),
            ValueType::Float => Self::Float(FloatListExpr::panic(FloatListItem, panic)),
            ValueType::Bool => Self::Bool(BoolListExpr::panic(BoolListItem, panic)),
            ValueType::Nil => Self::Nil(NilListExpr::panic(NilListItem, panic)),
            ValueType::Tuple(item_type) => {
                Self::Tuple(TupleListExpr::panic(TupleListItem { item_type }, panic))
            }
            ValueType::List(item_type) => {
                match crate::plan::ValueShape::from_value_type(*item_type).representation() {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        Self::ParameterList(ParameterListListExpr::panic(
                            ParameterListListItem::new(parameter),
                            panic,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(item_shape) => {
                        Self::List(ListListExpr::panic(ListListItem::new(item_shape), panic))
                    }
                }
            }
            ValueType::Function(item_type) => Self::Function(FunctionListExpr::panic(
                FunctionListItem {
                    item_type: *item_type,
                },
                panic,
            )),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, branches: BoolListCaseBranches) -> Self {
        match branches {
            BoolListCaseBranches::Generic { true_, false_ } => {
                Self::Generic(GenericListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::ParameterList { true_, false_ } => {
                Self::ParameterList(ParameterListListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Int { true_, false_ } => {
                Self::Int(IntListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::String { true_, false_ } => {
                Self::String(StringListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::BitArray { true_, false_ } => {
                Self::BitArray(BitArrayListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::UtfCodepoint { true_, false_ } => {
                Self::UtfCodepoint(UtfCodepointListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Custom { true_, false_ } => {
                Self::Custom(CustomListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::External { true_, false_ } => {
                Self::External(ExternalListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Float { true_, false_ } => {
                Self::Float(FloatListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Bool { true_, false_ } => {
                Self::Bool(BoolListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Nil { true_, false_ } => {
                Self::Nil(NilListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Tuple { true_, false_ } => {
                Self::Tuple(TupleListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::List { true_, false_ } => {
                Self::List(ListListExpr::bool_case(subject, true_, false_))
            }
            BoolListCaseBranches::Function { true_, false_ } => {
                Self::Function(FunctionListExpr::bool_case(subject, true_, false_))
            }
        }
    }

    pub(crate) fn int_case(subject: IntExpr, branches: ListCaseBranches<BigInt>) -> Self {
        match branches {
            ListCaseBranches::Generic { clauses, fallback } => {
                Self::Generic(GenericListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::ParameterList { clauses, fallback } => {
                Self::ParameterList(ParameterListListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::UtfCodepoint(UtfCodepointListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::External { clauses, fallback } => {
                Self::External(ExternalListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::int_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::int_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn string_case(subject: StringExpr, branches: ListCaseBranches<EcoString>) -> Self {
        match branches {
            ListCaseBranches::Generic { clauses, fallback } => {
                Self::Generic(GenericListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::ParameterList { clauses, fallback } => Self::ParameterList(
                ParameterListListExpr::string_case(subject, clauses, fallback),
            ),
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => Self::UtfCodepoint(
                UtfCodepointListExpr::string_case(subject, clauses, fallback),
            ),
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::External { clauses, fallback } => {
                Self::External(ExternalListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::string_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::string_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: ListCaseBranches<f64>) -> Self {
        match branches {
            ListCaseBranches::Generic { clauses, fallback } => {
                Self::Generic(GenericListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::ParameterList { clauses, fallback } => Self::ParameterList(
                ParameterListListExpr::float_case(subject, clauses, fallback),
            ),
            ListCaseBranches::Int { clauses, fallback } => {
                Self::Int(IntListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::String { clauses, fallback } => {
                Self::String(StringListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::BitArray { clauses, fallback } => {
                Self::BitArray(BitArrayListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::UtfCodepoint(UtfCodepointListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Custom { clauses, fallback } => {
                Self::Custom(CustomListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::External { clauses, fallback } => {
                Self::External(ExternalListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Float { clauses, fallback } => {
                Self::Float(FloatListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Bool { clauses, fallback } => {
                Self::Bool(BoolListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Nil { clauses, fallback } => {
                Self::Nil(NilListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Tuple { clauses, fallback } => {
                Self::Tuple(TupleListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::List { clauses, fallback } => {
                Self::List(ListListExpr::float_case(subject, clauses, fallback))
            }
            ListCaseBranches::Function { clauses, fallback } => {
                Self::Function(FunctionListExpr::float_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: ListExpr) -> Self {
        match return_ {
            Self::Generic(return_) => Self::Generic(GenericListExpr::block(steps, return_)),
            Self::ParameterList(return_) => {
                Self::ParameterList(ParameterListListExpr::block(steps, return_))
            }
            Self::Int(return_) => Self::Int(IntListExpr::block(steps, return_)),
            Self::String(return_) => Self::String(StringListExpr::block(steps, return_)),
            Self::BitArray(return_) => Self::BitArray(BitArrayListExpr::block(steps, return_)),
            Self::UtfCodepoint(return_) => {
                Self::UtfCodepoint(UtfCodepointListExpr::block(steps, return_))
            }
            Self::Custom(return_) => Self::Custom(CustomListExpr::block(steps, return_)),
            Self::External(return_) => Self::External(ExternalListExpr::block(steps, return_)),
            Self::Float(return_) => Self::Float(FloatListExpr::block(steps, return_)),
            Self::Bool(return_) => Self::Bool(BoolListExpr::block(steps, return_)),
            Self::Nil(return_) => Self::Nil(NilListExpr::block(steps, return_)),
            Self::Tuple(return_) => Self::Tuple(TupleListExpr::block(steps, return_)),
            Self::List(return_) => Self::List(ListListExpr::block(steps, return_)),
            Self::Function(return_) => Self::Function(FunctionListExpr::block(steps, return_)),
        }
    }

    pub fn element_type(&self) -> ValueType {
        match self {
            Self::Generic(expression) => expression.element_type(),
            Self::ParameterList(expression) => expression.element_type(),
            Self::Int(expression) => expression.element_type(),
            Self::String(expression) => expression.element_type(),
            Self::BitArray(expression) => expression.element_type(),
            Self::UtfCodepoint(expression) => expression.element_type(),
            Self::Custom(expression) => expression.element_type(),
            Self::External(expression) => expression.element_type(),
            Self::Float(expression) => expression.element_type(),
            Self::Bool(expression) => expression.element_type(),
            Self::Nil(expression) => expression.element_type(),
            Self::Tuple(expression) => expression.element_type(),
            Self::List(expression) => expression.element_type(),
            Self::Function(expression) => expression.element_type(),
        }
    }

    pub(crate) fn item_shape(&self) -> &crate::plan::ValueShape {
        match self {
            Self::Generic(expression) => expression.item_shape(),
            Self::ParameterList(expression) => expression.item_shape(),
            Self::Int(expression) => expression.item_shape(),
            Self::String(expression) => expression.item_shape(),
            Self::BitArray(expression) => expression.item_shape(),
            Self::UtfCodepoint(expression) => expression.item_shape(),
            Self::Custom(expression) => expression.item_shape(),
            Self::External(expression) => expression.item_shape(),
            Self::Float(expression) => expression.item_shape(),
            Self::Bool(expression) => expression.item_shape(),
            Self::Nil(expression) => expression.item_shape(),
            Self::Tuple(expression) => expression.item_shape(),
            Self::List(expression) => expression.item_shape(),
            Self::Function(expression) => expression.item_shape(),
        }
    }

    pub(crate) fn with_item_shape(self, item_shape: crate::plan::ValueShape) -> Self {
        match self {
            Self::Generic(expression) => Self::Generic(expression.with_item_shape(item_shape)),
            Self::ParameterList(expression) => {
                Self::ParameterList(expression.with_item_shape(item_shape))
            }
            Self::Int(expression) => Self::Int(expression.with_item_shape(item_shape)),
            Self::String(expression) => Self::String(expression.with_item_shape(item_shape)),
            Self::BitArray(expression) => Self::BitArray(expression.with_item_shape(item_shape)),
            Self::UtfCodepoint(expression) => {
                Self::UtfCodepoint(expression.with_item_shape(item_shape))
            }
            Self::Custom(expression) => Self::Custom(expression.with_item_shape(item_shape)),
            Self::External(expression) => Self::External(expression.with_item_shape(item_shape)),
            Self::Float(expression) => Self::Float(expression.with_item_shape(item_shape)),
            Self::Bool(expression) => Self::Bool(expression.with_item_shape(item_shape)),
            Self::Nil(expression) => Self::Nil(expression.with_item_shape(item_shape)),
            Self::Tuple(expression) => Self::Tuple(expression.with_item_shape(item_shape)),
            Self::List(expression) => Self::List(expression.with_item_shape(item_shape)),
            Self::Function(expression) => Self::Function(expression.with_item_shape(item_shape)),
        }
    }

    pub(crate) fn into_int(self) -> Option<IntListExpr> {
        match self {
            Self::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_generic(self) -> Option<GenericListExpr> {
        match self {
            Self::Generic(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_parameter_list(self) -> Option<ParameterListListExpr> {
        match self {
            Self::ParameterList(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringListExpr> {
        match self {
            Self::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bit_array(self) -> Option<BitArrayListExpr> {
        match self {
            Self::BitArray(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_utf_codepoint(self) -> Option<UtfCodepointListExpr> {
        match self {
            Self::UtfCodepoint(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_custom(self) -> Option<CustomListExpr> {
        match self {
            Self::Custom(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_external(self) -> Option<ExternalListExpr> {
        match self {
            Self::External(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatListExpr> {
        match self {
            Self::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolListExpr> {
        match self {
            Self::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_nil(self) -> Option<NilListExpr> {
        match self {
            Self::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleListExpr> {
        match self {
            Self::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_list(self) -> Option<ListListExpr> {
        match self {
            Self::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionListExpr> {
        match self {
            Self::Function(expression) => Some(expression),
            _ => None,
        }
    }
}

impl StoredListExpr {
    pub(super) fn try_from_facade(expression: ListExpr) -> Option<Self> {
        match expression {
            ListExpr::Generic(_) => None,
            ListExpr::ParameterList(expression) => Some(Self::ParameterList(expression)),
            ListExpr::Int(expression) => Some(Self::Int(expression)),
            ListExpr::String(expression) => Some(Self::String(expression)),
            ListExpr::BitArray(expression) => Some(Self::BitArray(expression)),
            ListExpr::UtfCodepoint(expression) => Some(Self::UtfCodepoint(expression)),
            ListExpr::Custom(expression) => Some(Self::Custom(expression)),
            ListExpr::External(expression) => Some(Self::External(expression)),
            ListExpr::Float(expression) => Some(Self::Float(expression)),
            ListExpr::Bool(expression) => Some(Self::Bool(expression)),
            ListExpr::Nil(expression) => Some(Self::Nil(expression)),
            ListExpr::Tuple(expression) => Some(Self::Tuple(expression)),
            ListExpr::List(expression) => Some(Self::List(expression)),
            ListExpr::Function(expression) => Some(Self::Function(expression)),
        }
    }
}

fn list_item_shape(
    elements: &[Expr],
    element_type: &ValueType,
) -> Result<crate::plan::ValueShape, ListElementTypeMismatch> {
    let Some((first, rest)) = elements.split_first() else {
        return Ok(crate::plan::ValueShape::from_value_type(
            element_type.clone(),
        ));
    };
    if first.value_type() != *element_type {
        return Err(ListElementTypeMismatch {
            expected: element_type.clone(),
            actual: first.value_type(),
        });
    }

    let mut shape = first.value_shape().clone();
    for element in rest {
        if element.value_type() != *element_type {
            return Err(ListElementTypeMismatch {
                expected: element_type.clone(),
                actual: element.value_type(),
            });
        }
        let Some(merged) = shape.merge(element.value_shape()) else {
            return Err(ListElementTypeMismatch {
                expected: element_type.clone(),
                actual: element.value_type(),
            });
        };
        shape = merged;
    }
    Ok(shape)
}

#[cfg(test)]
impl ListExpr {
    pub(crate) fn value(elements: Vec<Expr>, element_type: ValueType) -> Self {
        Self::try_value(elements, element_type)
            .expect("list expression elements must match declared item type")
    }

    pub(crate) fn spread(elements: Vec<Expr>, tail: ListExpr, element_type: ValueType) -> Self {
        let elements = ListElements::from_exprs(element_type, elements)
            .expect("list spread elements must match declared item type");
        Self::try_spread(elements, tail).expect("list spread tail must match prefix item type")
    }

    pub(crate) fn try_spread(
        elements: ListElements,
        tail: ListExpr,
    ) -> Result<Self, ListSpreadConstructionError> {
        ListSpreadElements::from_parts(elements, tail).map(Self::from_spread_elements)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
        CustomListExpr, CustomListItem, ExternalListExpr, ExternalListItem, FloatListExpr,
        FloatListItem, FunctionListExpr, FunctionListItem, GenericListExpr, GenericListItem,
        IntListExpr, IntListItem, ListCaseBranches, ListElementTypeMismatch, ListElements,
        ListExpr, ListIndexSource, ListListExpr, ListListItem, ListSpreadConstructionError,
        NilListExpr, NilListItem, ParameterListListExpr, ParameterListListItem, StoredListExpr,
        StringListExpr, StringListItem, TupleListExpr, TupleListItem, UtfCodepointListExpr,
        UtfCodepointListItem,
    };
    use crate::plan::{
        BitArrayExpr, BoolExpr, CustomConstructorRefinement, CustomExpr, CustomFieldAccess,
        CustomLocalId, CustomType, CustomTypeName, CustomValueShape, Expr, ExternalTypeName,
        ExternalValueShape, FloatExpr, FunctionExpr, FunctionReference, FunctionShape,
        FunctionType, GenericExpr, GenericLocal, GenericLocalId, IntExpr, IntListLocalId,
        ListFunctionExpr, ListFunctionReference, ListLocal, NilExpr, PanicExpr, PanicSite, Step,
        StringExpr, TupleExpr, TypeParameterId, UtfCodepointExpr, UtfCodepointLocalId, ValueShape,
        ValueStorageShape, ValueType, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    fn list_function_instantiation(
        template: usize,
        item_shape: ValueShape,
    ) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(
            template,
            FunctionShape::new(Vec::new(), ValueShape::List(Box::new(item_shape))),
        )
    }

    #[test]
    fn value_constructor_preserves_typed_item_family() {
        assert_eq!(
            ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int),
            ListExpr::Int(IntListExpr::value(
                IntListItem,
                vec![IntExpr::value(1.into())],
            )),
        );
        assert_eq!(
            ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            ),
            ListExpr::String(StringListExpr::value(
                StringListItem,
                vec![StringExpr::value("one".into())],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float),
            ListExpr::Float(FloatListExpr::value(
                FloatListItem,
                vec![FloatExpr::value(1.5)],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool),
            ListExpr::Bool(BoolListExpr::value(
                BoolListItem,
                vec![BoolExpr::value(true)],
            )),
        );
        let codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        assert_eq!(
            ListExpr::value(
                vec![Expr::utf_codepoint(codepoint.clone())],
                ValueType::UtfCodepoint,
            ),
            ListExpr::UtfCodepoint(UtfCodepointListExpr::value(
                UtfCodepointListItem,
                vec![codepoint],
            )),
        );
        assert_eq!(
            ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
            ListExpr::Nil(NilListExpr::value(NilListItem, vec![NilExpr::value()])),
        );

        let tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(2.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            ListExpr::value(
                vec![Expr::tuple(tuple.clone())],
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            ListExpr::Tuple(TupleListExpr::value(
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
                vec![tuple],
            )),
        );

        let nested = IntListExpr::value(IntListItem, vec![IntExpr::value(3.into())]);
        assert_eq!(
            ListExpr::value(
                vec![Expr::list(ListExpr::Int(nested.clone()))],
                ValueType::List(Box::new(ValueType::Int)),
            ),
            ListExpr::List(ListListExpr::value(
                ListListItem::new(ValueStorageShape::Int),
                vec![StoredListExpr::Int(nested)],
            )),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function =
            FunctionExpr::reference(FunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::from_function_type(function_type.clone()),
            )));
        assert_eq!(
            ListExpr::value(
                vec![Expr::function(function.clone())],
                ValueType::Function(Box::new(function_type.clone())),
            ),
            ListExpr::Function(FunctionListExpr::value(
                FunctionListItem {
                    item_type: function_type,
                },
                vec![function],
            )),
        );
    }

    #[test]
    fn value_constructor_rejects_later_nominal_and_refinement_mismatches() {
        assert_eq!(
            ListExpr::try_value(
                vec![
                    Expr::int(IntExpr::value(1.into())),
                    Expr::string(StringExpr::value("wrong".into())),
                ],
                ValueType::Int,
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );

        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let function_type =
            FunctionType::new(vec![ValueType::Custom(type_.clone())], ValueType::Int);
        let function = |id, constructor| {
            let custom_shape = CustomValueShape::new(
                type_.type_name().clone(),
                Vec::new(),
                CustomConstructorRefinement::Exact(constructor),
            );
            let shape = FunctionShape::new(
                vec![ValueShape::Custom(custom_shape.clone())],
                ValueShape::Int,
            );
            FunctionExpr::reference(FunctionReference::new(monomorphic_function_instantiation(
                id,
                shape.clone(),
            )))
            .with_resolved_shape(shape)
            .expect("function shape has the same nominal type")
        };

        assert_eq!(
            ListExpr::try_value(
                vec![
                    Expr::function(function(0, 0)),
                    Expr::function(function(1, 1)),
                ],
                ValueType::Function(Box::new(function_type.clone())),
            ),
            Err(ListElementTypeMismatch {
                expected: ValueType::Function(Box::new(function_type.clone())),
                actual: ValueType::Function(Box::new(function_type)),
            }),
        );
    }

    #[test]
    fn spread_constructor_preserves_typed_tail_family() {
        let parameter = TypeParameterId(0);
        let generic_value = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "value".into(),
        );
        let generic_tail = ListExpr::value(Vec::new(), ValueType::Parameter(parameter));
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Generic {
                    parameter,
                    values: vec![generic_value.clone()],
                },
                generic_tail.clone(),
            ),
            Ok(ListExpr::Generic(GenericListExpr::spread(
                vec1::vec1![generic_value],
                generic_tail.into_generic().expect("generic list"),
            ))),
        );

        let int_tail = ListExpr::value(vec![Expr::int(IntExpr::value(2.into()))], ValueType::Int);
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Int(vec![IntExpr::value(1.into())]),
                int_tail.clone(),
            ),
            Ok(ListExpr::Int(IntListExpr::spread(
                vec1::vec1![IntExpr::value(1.into())],
                int_tail.into_int().expect("int list"),
            ))),
        );

        let tuple_tail = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(2.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::try_spread(
                ListElements::Tuple {
                    item_type: vec![ValueType::Int],
                    values: vec![TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    )],
                },
                tuple_tail.clone(),
            ),
            Ok(ListExpr::Tuple(TupleListExpr::spread(
                vec1::vec1![TupleExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                )],
                tuple_tail.into_tuple().expect("tuple list"),
            ))),
        );

        assert_eq!(
            ListExpr::try_spread(
                ListElements::Int(vec![IntExpr::value(1.into())]),
                ListExpr::value(
                    vec![Expr::string(StringExpr::value("wrong".into()))],
                    ValueType::String,
                ),
            ),
            Err(ListSpreadConstructionError::ElementTypeMismatch(
                ListElementTypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );
    }

    #[test]
    fn facade_typed_projection_rejects_wrong_item_family() {
        assert_eq!(
            ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            )
            .into_int(),
            None,
        );

        let int_list = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        assert_eq!(int_list.clone().into_string(), None);
        assert_eq!(int_list.clone().into_bit_array(), None);
        assert_eq!(int_list.clone().into_utf_codepoint(), None);
        assert_eq!(int_list.clone().into_custom(), None);
        assert_eq!(int_list.clone().into_float(), None);
        assert_eq!(int_list.clone().into_bool(), None);
        assert_eq!(int_list.clone().into_nil(), None);
        assert_eq!(int_list.clone().into_tuple(), None);
        assert_eq!(int_list.clone().into_list(), None);
        assert_eq!(int_list.into_function(), None);
    }

    #[test]
    fn facade_constructors_dispatch_to_typed_shape() {
        assert_eq!(
            ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into()),
            ListExpr::Int(IntListExpr::local_get(
                IntListItem,
                IntListLocalId(0),
                "values".into(),
            )),
        );
        let int_list_function = list_function_instantiation(0, ValueShape::Int);
        assert_eq!(
            ListExpr::call(int_list_function.clone(), Vec::new(), ValueShape::Int),
            ListExpr::Int(IntListExpr::call(
                IntListItem,
                int_list_function,
                Vec::new(),
            )),
        );

        let list_function = ListFunctionExpr::reference(
            ListFunctionReference::new(list_function_instantiation(0, ValueShape::Int)),
            ValueType::Int,
        );
        assert_eq!(
            ListExpr::function_call(list_function.clone(), Vec::new()),
            ListExpr::Int(IntListExpr::function_call(
                IntListItem,
                list_function,
                Vec::new(),
            )),
        );

        let tuple = TupleExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                ValueType::Int,
            ))],
            vec![ValueType::List(Box::new(ValueType::Int))],
        );
        assert_eq!(
            ListExpr::tuple_index(tuple.clone(), 0, ValueType::List(Box::new(ValueType::Int))),
            ListExpr::List(ListListExpr::tuple_index(
                ListListItem::new(ValueStorageShape::Int),
                tuple,
                0,
            )),
        );

        let nested = ListExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::string(StringExpr::value("one".into()))],
                ValueType::String,
            ))],
            ValueType::List(Box::new(ValueType::String)),
        )
        .into_list()
        .expect("nested list");
        assert_eq!(
            ListExpr::list_index(nested.clone(), 0),
            ListExpr::String(StringListExpr::from_list_index(
                StringListItem,
                ListIndexSource::new(nested, 0),
            )),
        );
    }

    #[test]
    fn custom_field_constructor_dispatches_every_item_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let access = CustomFieldAccess::new(
            CustomExpr::local_get(
                crate::plan::CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                "boxed".into(),
            ),
            0,
            Some("value".into()),
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let external_type = crate::plan::ExternalType::new(
            crate::plan::ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );

        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Parameter(TypeParameterId(0)),),
            ListExpr::Generic(GenericListExpr::custom_field(
                GenericListItem::new(TypeParameterId(0)),
                access.clone(),
            )),
        );

        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Int),
            ListExpr::Int(IntListExpr::custom_field(IntListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::String),
            ListExpr::String(StringListExpr::custom_field(StringListItem, access.clone(),)),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::BitArray),
            ListExpr::BitArray(BitArrayListExpr::custom_field(
                BitArrayListItem,
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::UtfCodepoint),
            ListExpr::UtfCodepoint(UtfCodepointListExpr::custom_field(
                UtfCodepointListItem,
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Custom(custom_type.clone())),
            ListExpr::Custom(CustomListExpr::custom_field(
                CustomListItem {
                    item_type: custom_type,
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::External(external_type.clone())),
            ListExpr::External(ExternalListExpr::custom_field(
                ExternalListItem {
                    item_type: external_type,
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Float),
            ListExpr::Float(FloatListExpr::custom_field(FloatListItem, access.clone(),)),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Bool),
            ListExpr::Bool(BoolListExpr::custom_field(BoolListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Nil),
            ListExpr::Nil(NilListExpr::custom_field(NilListItem, access.clone())),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::Tuple(vec![ValueType::Int]),),
            ListExpr::Tuple(TupleListExpr::custom_field(
                TupleListItem {
                    item_type: vec![ValueType::Int],
                },
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(access.clone(), ValueType::List(Box::new(ValueType::String)),),
            ListExpr::List(ListListExpr::custom_field(
                ListListItem::new(ValueStorageShape::String),
                access.clone(),
            )),
        );
        let parameter = TypeParameterId(1);
        assert_eq!(
            ListExpr::custom_field(
                access.clone(),
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
            ),
            ListExpr::ParameterList(ParameterListListExpr::custom_field(
                ParameterListListItem::new(parameter),
                access.clone(),
            )),
        );
        assert_eq!(
            ListExpr::custom_field(
                access.clone(),
                ValueType::Function(Box::new(function_type.clone())),
            ),
            ListExpr::Function(FunctionListExpr::custom_field(
                FunctionListItem {
                    item_type: function_type,
                },
                access,
            )),
        );
    }

    #[test]
    fn case_and_block_constructors_preserve_typed_family() {
        let subject = BoolExpr::value(true);
        let parameter = TypeParameterId(0);
        let generic_true = ListExpr::value(Vec::new(), ValueType::Parameter(parameter))
            .into_generic()
            .expect("generic list");
        let generic_false = ListExpr::value(Vec::new(), ValueType::Parameter(parameter))
            .into_generic()
            .expect("generic list");
        assert_eq!(
            ListExpr::bool_case(
                subject.clone(),
                BoolListCaseBranches::Generic {
                    true_: generic_true.clone(),
                    false_: generic_false.clone(),
                },
            ),
            ListExpr::Generic(GenericListExpr::bool_case(
                subject.clone(),
                generic_true,
                generic_false,
            )),
        );

        let true_ = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
            .into_int()
            .expect("int list");
        let false_ = ListExpr::value(vec![Expr::int(IntExpr::value(2.into()))], ValueType::Int)
            .into_int()
            .expect("int list");

        assert_eq!(
            ListExpr::bool_case(
                subject.clone(),
                BoolListCaseBranches::Int {
                    true_: true_.clone(),
                    false_: false_.clone(),
                },
            ),
            ListExpr::Int(IntListExpr::bool_case(subject, true_, false_)),
        );

        let fallback = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(3.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        let branch = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(4.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::int_case(
                IntExpr::value(1.into()),
                ListCaseBranches::from_exprs(
                    vec![(BigInt::from(1), branch.clone())],
                    fallback.clone()
                )
                .expect("list case branches"),
            ),
            ListExpr::Tuple(TupleListExpr::int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), branch.into_tuple().expect("tuple list"))],
                fallback.into_tuple().expect("tuple list"),
            )),
        );

        let string_list = ListExpr::value(
            vec![Expr::string(StringExpr::value("done".into()))],
            ValueType::String,
        );
        assert_eq!(
            ListExpr::block(Vec::<Step>::new(), string_list.clone()),
            ListExpr::String(StringListExpr::block(
                Vec::<Step>::new(),
                string_list.into_string().expect("string list"),
            )),
        );
    }

    #[test]
    fn facade_dispatch_preserves_every_item_type() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let item_types = vec![
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                Vec::new(),
            )),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(1)))),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(function_type.clone())),
        ];

        for item_type in item_types {
            let item_shape = ValueShape::from_value_type(item_type.clone());
            assert_eq!(
                ListExpr::call(
                    list_function_instantiation(0, item_shape.clone()),
                    Vec::new(),
                    item_shape.clone(),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::function_call(
                    ListFunctionExpr::reference(
                        ListFunctionReference::new(list_function_instantiation(0, item_shape)),
                        item_type.clone()
                    ),
                    Vec::new(),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::tuple_index(
                    TupleExpr::value(
                        vec![Expr::list(ListExpr::value(Vec::new(), item_type.clone()))],
                        vec![ValueType::List(Box::new(item_type.clone()))],
                    ),
                    0,
                    item_type.clone(),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::drop_first(ListExpr::value(Vec::new(), item_type.clone()), 1)
                    .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::panic(
                    PanicExpr::todo_at(None, PanicSite::unknown()),
                    item_type.clone()
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::int_case(
                    IntExpr::value(1.into()),
                    ListCaseBranches::from_exprs(
                        vec![(
                            BigInt::from(1),
                            ListExpr::value(Vec::new(), item_type.clone()),
                        )],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::string_case(
                    StringExpr::value("one".into()),
                    ListCaseBranches::from_exprs(
                        vec![("one".into(), ListExpr::value(Vec::new(), item_type.clone()))],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::float_case(
                    FloatExpr::value(1.5),
                    ListCaseBranches::from_exprs(
                        vec![(1.5, ListExpr::value(Vec::new(), item_type.clone()))],
                        ListExpr::value(Vec::new(), item_type.clone()),
                    )
                    .expect("list case branches"),
                )
                .element_type(),
                item_type,
            );
            assert_eq!(
                ListExpr::block(
                    Vec::<Step>::new(),
                    ListExpr::value(Vec::new(), item_type.clone())
                )
                .element_type(),
                item_type,
            );
        }
    }

    #[test]
    fn nested_list_index_dispatch_preserves_every_item_type() {
        let parameter = crate::plan::TypeParameterId(0);
        let parameter_source = ListExpr::value(
            vec![Expr::list(ListExpr::value(
                Vec::new(),
                ValueType::Parameter(parameter),
            ))],
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("parameter-list item list");
        assert_eq!(
            ListExpr::parameter_list_index(parameter_source.clone(), 3),
            ListExpr::Generic(super::GenericListExpr::from_list_index(
                super::GenericListItem::new(parameter),
                ListIndexSource::new(parameter_source, 3),
            )),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let custom_type = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let external_shape = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        let item_shapes = vec![
            ValueStorageShape::Int,
            ValueStorageShape::String,
            ValueStorageShape::BitArray,
            ValueStorageShape::UtfCodepoint,
            ValueStorageShape::Custom(CustomValueShape::any(custom_type)),
            ValueStorageShape::External(external_shape),
            ValueStorageShape::Float,
            ValueStorageShape::Bool,
            ValueStorageShape::Nil,
            ValueStorageShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
            ValueStorageShape::List(Box::new(ValueShape::String)),
            ValueStorageShape::Function(Box::new(FunctionShape::from_function_type(
                function_type.clone(),
            ))),
        ];

        for item_shape in item_shapes {
            let item_type = item_shape.value_type();
            let list = ListExpr::value(
                vec![Expr::list(ListExpr::value(Vec::new(), item_type.clone()))],
                ValueType::List(Box::new(item_type.clone())),
            )
            .into_list()
            .expect("list item list");
            let expected = match item_shape {
                ValueStorageShape::Int => ListExpr::Int(IntListExpr::from_list_index(
                    IntListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::String => ListExpr::String(StringListExpr::from_list_index(
                    StringListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::BitArray => {
                    ListExpr::BitArray(BitArrayListExpr::from_list_index(
                        BitArrayListItem,
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueStorageShape::UtfCodepoint => {
                    ListExpr::UtfCodepoint(UtfCodepointListExpr::from_list_index(
                        UtfCodepointListItem,
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueStorageShape::Custom(shape) => {
                    ListExpr::Custom(CustomListExpr::from_list_index(
                        CustomListItem::new(shape.type_().clone()),
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueStorageShape::External(shape) => {
                    ListExpr::External(ExternalListExpr::from_list_index(
                        ExternalListItem::new(shape.type_().clone()),
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueStorageShape::Float => ListExpr::Float(FloatListExpr::from_list_index(
                    FloatListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::Bool => ListExpr::Bool(BoolListExpr::from_list_index(
                    BoolListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::Nil => ListExpr::Nil(NilListExpr::from_list_index(
                    NilListItem,
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::Tuple(elements) => {
                    ListExpr::Tuple(TupleListExpr::from_list_index(
                        TupleListItem::new(elements.iter().map(ValueShape::value_type).collect()),
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
                ValueStorageShape::List(_item) => ListExpr::List(ListListExpr::from_list_index(
                    ListListItem::new(ValueStorageShape::String),
                    ListIndexSource::new(list.clone(), 3),
                )),
                ValueStorageShape::Function(shape) => {
                    ListExpr::Function(FunctionListExpr::from_list_index(
                        FunctionListItem::new(shape.type_()),
                        ListIndexSource::new(list.clone(), 3),
                    ))
                }
            };

            assert_eq!(ListExpr::list_index(list, 3), expected);
            assert_eq!(expected.element_type(), item_type);
        }
    }

    #[test]
    fn generic_list_conversion_rejects_other_item_family() {
        assert_eq!(
            ListExpr::value(Vec::new(), ValueType::Int).into_generic(),
            None,
        );
        assert_eq!(
            ListExpr::value(Vec::new(), ValueType::Int).into_parameter_list(),
            None,
        );
        assert_eq!(
            ListExpr::value(Vec::new(), ValueType::Int).into_external(),
            None,
        );
        assert_eq!(
            StoredListExpr::try_from_facade(ListExpr::value(
                Vec::new(),
                ValueType::Parameter(TypeParameterId(0)),
            )),
            None,
        );
    }

    #[test]
    fn spread_dispatch_preserves_remaining_item_families() {
        assert_eq!(
            ListExpr::spread(
                vec![Expr::string(StringExpr::value("head".into()))],
                ListExpr::value(Vec::new(), ValueType::String),
                ValueType::String,
            )
            .element_type(),
            ValueType::String,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::bit_array(BitArrayExpr::value(Vec::new()))],
                ListExpr::value(Vec::new(), ValueType::BitArray),
                ValueType::BitArray,
            )
            .element_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::utf_codepoint(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                ))],
                ListExpr::value(Vec::new(), ValueType::UtfCodepoint),
                ValueType::UtfCodepoint,
            )
            .element_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::float(FloatExpr::value(1.5))],
                ListExpr::value(Vec::new(), ValueType::Float),
                ValueType::Float,
            )
            .element_type(),
            ValueType::Float,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::bool(BoolExpr::value(true))],
                ListExpr::value(Vec::new(), ValueType::Bool),
                ValueType::Bool,
            )
            .element_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::nil(NilExpr::value())],
                ListExpr::value(Vec::new(), ValueType::Nil),
                ValueType::Nil,
            )
            .element_type(),
            ValueType::Nil,
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            ListExpr::spread(
                vec![Expr::list(ListExpr::value(Vec::new(), ValueType::String))],
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
                ValueType::List(Box::new(ValueType::String)),
            )
            .element_type(),
            ValueType::List(Box::new(ValueType::String)),
        );
        assert_eq!(
            ListExpr::spread(
                vec![Expr::function(FunctionExpr::reference(
                    FunctionReference::new(monomorphic_function_instantiation(
                        0,
                        FunctionShape::from_function_type(function_type.clone()),
                    ))
                ))],
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(function_type.clone()))
                ),
                ValueType::Function(Box::new(function_type.clone())),
            )
            .element_type(),
            ValueType::Function(Box::new(function_type)),
        );
    }
}
