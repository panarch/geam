mod output;

pub(super) use output::Output;

use super::pattern::BindingValue;
use super::type_::{ListSlot, Slot, TupleSlot, TypeError, Types};
use crate::plan::execution::graph::{self, ParamLocal, ParamSlot};
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct Locals<'data> {
    definitions: HashMap<Family, Vec<Slot<'data, 'data>>>,
    tuples: Vec<TupleSlot<'data>>,
    lists: HashMap<Family, Vec<ListSlot<'data>>>,
    constructors: HashMap<Address, Vec<usize>>,
    exact_constructors: HashMap<Address, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Family {
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    GenericFunction,
    NeverFunction,
    CustomFunction,
    ExternalFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    ParameterListFunction,
    ParameterListListFunction,
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
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    ParameterList,
    CustomList,
    ExternalList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ListList,
    ParameterListList,
    FunctionList,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct Address {
    family: Family,
    index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum LocalError {
    Type(TypeError),
    DefinitionOrder { address: Address, expected: usize },
    Missing(Address),
    Metadata(Address),
}

impl<'data> Locals<'data> {
    pub(super) fn set_constructor(&mut self, local: &ParamLocal, constructor: usize) {
        self.exact_constructors
            .insert(Address::of(local), constructor);
    }

    pub(super) fn value(&self, slot: Slot<'_, '_>) -> BindingValue {
        self.projected_value(&slot, slot.shape)
    }

    pub(super) fn projected_value(
        &self,
        slot: &ParamSlot,
        shape: crate::plan::execution::type_::ValueShapeId,
    ) -> BindingValue {
        BindingValue::Source {
            shape,
            constructor: self
                .exact_constructors
                .get(&Address::of(&slot.local))
                .copied(),
        }
    }

    pub(super) fn restrict_constructors(&mut self, local: &ParamLocal, constructors: Vec<usize>) {
        self.constructors.insert(Address::of(local), constructors);
    }

    pub(super) fn allows_constructor(&self, local: &ParamLocal, index: usize) -> bool {
        self.constructors
            .get(&Address::of(local))
            .is_none_or(|constructors| constructors.contains(&index))
    }

    pub(super) fn define(
        &mut self,
        slot: &'data ParamSlot,
        types: &Types<'data>,
    ) -> Result<(), LocalError> {
        if let ParamLocal::Tuple { local, .. } = slot.local {
            let slot = types.tuple_slot(slot).map_err(LocalError::Type)?;
            if local.0 != self.tuples.len() {
                return Err(LocalError::DefinitionOrder {
                    address: local.into(),
                    expected: self.tuples.len(),
                });
            }
            self.tuples.push(slot);
            return Ok(());
        }
        if let ParamLocal::List(_) = slot.local {
            let slot = types.list_slot(slot).map_err(LocalError::Type)?;
            let address = Address::of(&slot.slot.local);
            let family = self.lists.entry(address.family).or_default();
            if address.index != family.len() {
                return Err(LocalError::DefinitionOrder {
                    address,
                    expected: family.len(),
                });
            }
            family.push(slot);
            return Ok(());
        }
        let slot = types.slot(slot).map_err(LocalError::Type)?;
        let address = Address::of(&slot.local);
        let family = self.definitions.entry(address.family).or_default();
        if address.index != family.len() {
            return Err(LocalError::DefinitionOrder {
                address,
                expected: family.len(),
            });
        }
        family.push(slot);
        Ok(())
    }

    pub(super) fn get(
        &self,
        address: impl Into<Address>,
    ) -> Result<Slot<'data, 'data>, LocalError> {
        let address = address.into();
        if address.family == Family::Tuple {
            return self
                .tuple(graph::TupleLocalId(address.index))
                .map(|tuple| tuple.slot);
        }
        if matches!(
            address.family,
            Family::IntList
                | Family::StringList
                | Family::BitArrayList
                | Family::UtfCodepointList
                | Family::ParameterList
                | Family::CustomList
                | Family::ExternalList
                | Family::FloatList
                | Family::BoolList
                | Family::NilList
                | Family::TupleList
                | Family::ListList
                | Family::ParameterListList
                | Family::FunctionList
        ) {
            return self.list(address).map(|list| list.slot);
        }
        self.definitions
            .get(&address.family)
            .and_then(|family| family.get(address.index))
            .copied()
            .ok_or(LocalError::Missing(address))
    }

    pub(super) fn tuple(&self, id: graph::TupleLocalId) -> Result<TupleSlot<'data>, LocalError> {
        self.tuples
            .get(id.0)
            .copied()
            .ok_or(LocalError::Missing(id.into()))
    }

    pub(super) fn list(&self, address: impl Into<Address>) -> Result<ListSlot<'data>, LocalError> {
        let address = address.into();
        self.lists
            .get(&address.family)
            .and_then(|family| family.get(address.index))
            .copied()
            .ok_or(LocalError::Missing(address))
    }

    pub(super) fn parameter(&self, local: &ParamLocal) -> Result<Slot<'data, 'data>, LocalError> {
        let address = Address::of(local);
        let slot = self.get(address)?;
        if &slot.local != local {
            return Err(LocalError::Metadata(address));
        }
        Ok(slot)
    }
}

impl Address {
    pub(super) fn same_family(self, other: Self) -> bool {
        self.family == other.family
    }

    pub(super) fn of(local: &ParamLocal) -> Self {
        match local {
            ParamLocal::Int(id) => (*id).into(),
            ParamLocal::Float(id) => (*id).into(),
            ParamLocal::String(id) => (*id).into(),
            ParamLocal::BitArray(id) => (*id).into(),
            ParamLocal::UtfCodepoint(id) => (*id).into(),
            ParamLocal::Bool(id) => (*id).into(),
            ParamLocal::Nil(id) => (*id).into(),
            ParamLocal::Custom(local) => local.id.into(),
            ParamLocal::External(local) => local.id.into(),
            ParamLocal::Tuple { local, .. } => (*local).into(),
            ParamLocal::List(local) => match local {
                graph::ListLocal::Parameter { local, .. } => (*local).into(),
                graph::ListLocal::ParameterList { local, .. } => (*local).into(),
                graph::ListLocal::Int { local, .. } => (*local).into(),
                graph::ListLocal::String { local, .. } => (*local).into(),
                graph::ListLocal::BitArray { local, .. } => (*local).into(),
                graph::ListLocal::UtfCodepoint { local, .. } => (*local).into(),
                graph::ListLocal::Custom { local, .. } => (*local).into(),
                graph::ListLocal::External { local, .. } => (*local).into(),
                graph::ListLocal::Float { local, .. } => (*local).into(),
                graph::ListLocal::Bool { local, .. } => (*local).into(),
                graph::ListLocal::Nil { local, .. } => (*local).into(),
                graph::ListLocal::Tuple { local, .. } => (*local).into(),
                graph::ListLocal::List { local, .. } => (*local).into(),
                graph::ListLocal::Function { local, .. } => (*local).into(),
            },
            ParamLocal::IntFunction { local, .. } => (*local).into(),
            ParamLocal::FloatFunction { local, .. } => (*local).into(),
            ParamLocal::StringFunction { local, .. } => (*local).into(),
            ParamLocal::BitArrayFunction { local, .. } => (*local).into(),
            ParamLocal::UtfCodepointFunction { local, .. } => (*local).into(),
            ParamLocal::BoolFunction { local, .. } => (*local).into(),
            ParamLocal::NilFunction { local, .. } => (*local).into(),
            ParamLocal::TupleFunction { local, .. } => (*local).into(),
            ParamLocal::CustomFunction(local) => local.id.into(),
            ParamLocal::ExternalFunction(local) => local.id.into(),
            ParamLocal::GenericFunction(local) => local.id.into(),
            ParamLocal::NeverFunction(local) => local.id.into(),
            ParamLocal::ListFunction(local) => match local {
                graph::ListFunctionLocal::Parameter { local, .. } => (*local).into(),
                graph::ListFunctionLocal::ParameterList { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Int { local, .. } => (*local).into(),
                graph::ListFunctionLocal::String { local, .. } => (*local).into(),
                graph::ListFunctionLocal::BitArray { local, .. } => (*local).into(),
                graph::ListFunctionLocal::UtfCodepoint { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Custom { local, .. } => (*local).into(),
                graph::ListFunctionLocal::External { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Float { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Bool { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Nil { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Tuple { local, .. } => (*local).into(),
                graph::ListFunctionLocal::List { local, .. } => (*local).into(),
                graph::ListFunctionLocal::Function { local, .. } => (*local).into(),
            },
            ParamLocal::FunctionFunction(local) => match local {
                graph::FunctionFunctionLocal::Core(local) => local.id.into(),
                graph::FunctionFunctionLocal::External(local) => local.id.into(),
            },
        }
    }
}

impl From<graph::IntFunctionLocalId> for Address {
    fn from(id: graph::IntFunctionLocalId) -> Self {
        Self {
            family: Family::IntFunction,
            index: id.0,
        }
    }
}

impl From<graph::FloatFunctionLocalId> for Address {
    fn from(id: graph::FloatFunctionLocalId) -> Self {
        Self {
            family: Family::FloatFunction,
            index: id.0,
        }
    }
}

impl From<graph::StringFunctionLocalId> for Address {
    fn from(id: graph::StringFunctionLocalId) -> Self {
        Self {
            family: Family::StringFunction,
            index: id.0,
        }
    }
}

impl From<graph::BitArrayFunctionLocalId> for Address {
    fn from(id: graph::BitArrayFunctionLocalId) -> Self {
        Self {
            family: Family::BitArrayFunction,
            index: id.0,
        }
    }
}

impl From<graph::UtfCodepointFunctionLocalId> for Address {
    fn from(id: graph::UtfCodepointFunctionLocalId) -> Self {
        Self {
            family: Family::UtfCodepointFunction,
            index: id.0,
        }
    }
}

impl From<graph::GenericFunctionLocalId> for Address {
    fn from(id: graph::GenericFunctionLocalId) -> Self {
        Self {
            family: Family::GenericFunction,
            index: id.0,
        }
    }
}

impl From<graph::NeverFunctionLocalId> for Address {
    fn from(id: graph::NeverFunctionLocalId) -> Self {
        Self {
            family: Family::NeverFunction,
            index: id.0,
        }
    }
}

impl From<graph::CustomFunctionLocalId> for Address {
    fn from(id: graph::CustomFunctionLocalId) -> Self {
        Self {
            family: Family::CustomFunction,
            index: id.0,
        }
    }
}

impl From<graph::ExternalFunctionLocalId> for Address {
    fn from(id: graph::ExternalFunctionLocalId) -> Self {
        Self {
            family: Family::ExternalFunction,
            index: id.0,
        }
    }
}

impl From<graph::BoolFunctionLocalId> for Address {
    fn from(id: graph::BoolFunctionLocalId) -> Self {
        Self {
            family: Family::BoolFunction,
            index: id.0,
        }
    }
}

impl From<graph::NilFunctionLocalId> for Address {
    fn from(id: graph::NilFunctionLocalId) -> Self {
        Self {
            family: Family::NilFunction,
            index: id.0,
        }
    }
}

impl From<graph::TupleFunctionLocalId> for Address {
    fn from(id: graph::TupleFunctionLocalId) -> Self {
        Self {
            family: Family::TupleFunction,
            index: id.0,
        }
    }
}

impl From<graph::IntListFunctionLocalId> for Address {
    fn from(id: graph::IntListFunctionLocalId) -> Self {
        Self {
            family: Family::IntListFunction,
            index: id.0,
        }
    }
}

impl From<graph::StringListFunctionLocalId> for Address {
    fn from(id: graph::StringListFunctionLocalId) -> Self {
        Self {
            family: Family::StringListFunction,
            index: id.0,
        }
    }
}

impl From<graph::BitArrayListFunctionLocalId> for Address {
    fn from(id: graph::BitArrayListFunctionLocalId) -> Self {
        Self {
            family: Family::BitArrayListFunction,
            index: id.0,
        }
    }
}

impl From<graph::UtfCodepointListFunctionLocalId> for Address {
    fn from(id: graph::UtfCodepointListFunctionLocalId) -> Self {
        Self {
            family: Family::UtfCodepointListFunction,
            index: id.0,
        }
    }
}

impl From<graph::ParameterListFunctionLocalId> for Address {
    fn from(id: graph::ParameterListFunctionLocalId) -> Self {
        Self {
            family: Family::ParameterListFunction,
            index: id.0,
        }
    }
}

impl From<graph::ParameterListListFunctionLocalId> for Address {
    fn from(id: graph::ParameterListListFunctionLocalId) -> Self {
        Self {
            family: Family::ParameterListListFunction,
            index: id.0,
        }
    }
}

impl From<graph::CustomListFunctionLocalId> for Address {
    fn from(id: graph::CustomListFunctionLocalId) -> Self {
        Self {
            family: Family::CustomListFunction,
            index: id.0,
        }
    }
}

impl From<graph::ExternalListFunctionLocalId> for Address {
    fn from(id: graph::ExternalListFunctionLocalId) -> Self {
        Self {
            family: Family::ExternalListFunction,
            index: id.0,
        }
    }
}

impl From<graph::FloatListFunctionLocalId> for Address {
    fn from(id: graph::FloatListFunctionLocalId) -> Self {
        Self {
            family: Family::FloatListFunction,
            index: id.0,
        }
    }
}

impl From<graph::BoolListFunctionLocalId> for Address {
    fn from(id: graph::BoolListFunctionLocalId) -> Self {
        Self {
            family: Family::BoolListFunction,
            index: id.0,
        }
    }
}

impl From<graph::NilListFunctionLocalId> for Address {
    fn from(id: graph::NilListFunctionLocalId) -> Self {
        Self {
            family: Family::NilListFunction,
            index: id.0,
        }
    }
}

impl From<graph::TupleListFunctionLocalId> for Address {
    fn from(id: graph::TupleListFunctionLocalId) -> Self {
        Self {
            family: Family::TupleListFunction,
            index: id.0,
        }
    }
}

impl From<graph::ListListFunctionLocalId> for Address {
    fn from(id: graph::ListListFunctionLocalId) -> Self {
        Self {
            family: Family::ListListFunction,
            index: id.0,
        }
    }
}

impl From<graph::FunctionListFunctionLocalId> for Address {
    fn from(id: graph::FunctionListFunctionLocalId) -> Self {
        Self {
            family: Family::FunctionListFunction,
            index: id.0,
        }
    }
}

impl From<graph::CoreFunctionFunctionLocalId> for Address {
    fn from(id: graph::CoreFunctionFunctionLocalId) -> Self {
        Self {
            family: Family::CoreFunctionFunction,
            index: id.0,
        }
    }
}

impl From<graph::ExternalFunctionFunctionLocalId> for Address {
    fn from(id: graph::ExternalFunctionFunctionLocalId) -> Self {
        Self {
            family: Family::ExternalFunctionFunction,
            index: id.0,
        }
    }
}

impl From<graph::IntListLocalId> for Address {
    fn from(id: graph::IntListLocalId) -> Self {
        Self {
            family: Family::IntList,
            index: id.0,
        }
    }
}

impl From<graph::StringListLocalId> for Address {
    fn from(id: graph::StringListLocalId) -> Self {
        Self {
            family: Family::StringList,
            index: id.0,
        }
    }
}

impl From<graph::BitArrayListLocalId> for Address {
    fn from(id: graph::BitArrayListLocalId) -> Self {
        Self {
            family: Family::BitArrayList,
            index: id.0,
        }
    }
}

impl From<graph::UtfCodepointListLocalId> for Address {
    fn from(id: graph::UtfCodepointListLocalId) -> Self {
        Self {
            family: Family::UtfCodepointList,
            index: id.0,
        }
    }
}

impl From<graph::ParameterListLocalId> for Address {
    fn from(id: graph::ParameterListLocalId) -> Self {
        Self {
            family: Family::ParameterList,
            index: id.0,
        }
    }
}

impl From<graph::CustomListLocalId> for Address {
    fn from(id: graph::CustomListLocalId) -> Self {
        Self {
            family: Family::CustomList,
            index: id.0,
        }
    }
}

impl From<graph::ExternalListLocalId> for Address {
    fn from(id: graph::ExternalListLocalId) -> Self {
        Self {
            family: Family::ExternalList,
            index: id.0,
        }
    }
}

impl From<graph::FloatListLocalId> for Address {
    fn from(id: graph::FloatListLocalId) -> Self {
        Self {
            family: Family::FloatList,
            index: id.0,
        }
    }
}

impl From<graph::BoolListLocalId> for Address {
    fn from(id: graph::BoolListLocalId) -> Self {
        Self {
            family: Family::BoolList,
            index: id.0,
        }
    }
}

impl From<graph::NilListLocalId> for Address {
    fn from(id: graph::NilListLocalId) -> Self {
        Self {
            family: Family::NilList,
            index: id.0,
        }
    }
}

impl From<graph::TupleListLocalId> for Address {
    fn from(id: graph::TupleListLocalId) -> Self {
        Self {
            family: Family::TupleList,
            index: id.0,
        }
    }
}

impl From<graph::ListListLocalId> for Address {
    fn from(id: graph::ListListLocalId) -> Self {
        Self {
            family: Family::ListList,
            index: id.0,
        }
    }
}

impl From<graph::ParameterListListLocalId> for Address {
    fn from(id: graph::ParameterListListLocalId) -> Self {
        Self {
            family: Family::ParameterListList,
            index: id.0,
        }
    }
}

impl From<graph::FunctionListLocalId> for Address {
    fn from(id: graph::FunctionListLocalId) -> Self {
        Self {
            family: Family::FunctionList,
            index: id.0,
        }
    }
}

impl From<graph::IntLocalId> for Address {
    fn from(id: graph::IntLocalId) -> Self {
        Self {
            family: Family::Int,
            index: id.0,
        }
    }
}

impl From<graph::FloatLocalId> for Address {
    fn from(id: graph::FloatLocalId) -> Self {
        Self {
            family: Family::Float,
            index: id.0,
        }
    }
}

impl From<graph::StringLocalId> for Address {
    fn from(id: graph::StringLocalId) -> Self {
        Self {
            family: Family::String,
            index: id.0,
        }
    }
}

impl From<graph::BitArrayLocalId> for Address {
    fn from(id: graph::BitArrayLocalId) -> Self {
        Self {
            family: Family::BitArray,
            index: id.0,
        }
    }
}

impl From<graph::UtfCodepointLocalId> for Address {
    fn from(id: graph::UtfCodepointLocalId) -> Self {
        Self {
            family: Family::UtfCodepoint,
            index: id.0,
        }
    }
}

impl From<graph::CustomLocalId> for Address {
    fn from(id: graph::CustomLocalId) -> Self {
        Self {
            family: Family::Custom,
            index: id.0,
        }
    }
}

impl From<graph::ExternalLocalId> for Address {
    fn from(id: graph::ExternalLocalId) -> Self {
        Self {
            family: Family::External,
            index: id.0,
        }
    }
}

impl From<graph::BoolLocalId> for Address {
    fn from(id: graph::BoolLocalId) -> Self {
        Self {
            family: Family::Bool,
            index: id.0,
        }
    }
}

impl From<graph::NilLocalId> for Address {
    fn from(id: graph::NilLocalId) -> Self {
        Self {
            family: Family::Nil,
            index: id.0,
        }
    }
}

impl From<graph::TupleLocalId> for Address {
    fn from(id: graph::TupleLocalId) -> Self {
        Self {
            family: Family::Tuple,
            index: id.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, LocalError, Locals, ParamLocal, ParamSlot, TypeError, Types, graph};
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        CustomTypeTable, ExternalTypeTable, ListTypeTable, ValueShapeDescriptor, ValueShapeId,
        ValueShapeTable, ValueType,
    };

    #[test]
    fn concrete_ids_address_distinct_families() {
        let addresses: Vec<Address> = vec![
            graph::IntFunctionLocalId(0).into(),
            graph::FloatFunctionLocalId(0).into(),
            graph::StringFunctionLocalId(0).into(),
            graph::BitArrayFunctionLocalId(0).into(),
            graph::UtfCodepointFunctionLocalId(0).into(),
            graph::GenericFunctionLocalId(0).into(),
            graph::NeverFunctionLocalId(0).into(),
            graph::CustomFunctionLocalId(0).into(),
            graph::ExternalFunctionLocalId(0).into(),
            graph::BoolFunctionLocalId(0).into(),
            graph::NilFunctionLocalId(0).into(),
            graph::TupleFunctionLocalId(0).into(),
            graph::IntListFunctionLocalId(0).into(),
            graph::StringListFunctionLocalId(0).into(),
            graph::BitArrayListFunctionLocalId(0).into(),
            graph::UtfCodepointListFunctionLocalId(0).into(),
            graph::ParameterListFunctionLocalId(0).into(),
            graph::ParameterListListFunctionLocalId(0).into(),
            graph::CustomListFunctionLocalId(0).into(),
            graph::ExternalListFunctionLocalId(0).into(),
            graph::FloatListFunctionLocalId(0).into(),
            graph::BoolListFunctionLocalId(0).into(),
            graph::NilListFunctionLocalId(0).into(),
            graph::TupleListFunctionLocalId(0).into(),
            graph::ListListFunctionLocalId(0).into(),
            graph::FunctionListFunctionLocalId(0).into(),
            graph::CoreFunctionFunctionLocalId(0).into(),
            graph::ExternalFunctionFunctionLocalId(0).into(),
            graph::IntListLocalId(0).into(),
            graph::StringListLocalId(0).into(),
            graph::BitArrayListLocalId(0).into(),
            graph::UtfCodepointListLocalId(0).into(),
            graph::ParameterListLocalId(0).into(),
            graph::CustomListLocalId(0).into(),
            graph::ExternalListLocalId(0).into(),
            graph::FloatListLocalId(0).into(),
            graph::BoolListLocalId(0).into(),
            graph::NilListLocalId(0).into(),
            graph::TupleListLocalId(0).into(),
            graph::ListListLocalId(0).into(),
            graph::ParameterListListLocalId(0).into(),
            graph::FunctionListLocalId(0).into(),
            graph::IntLocalId(0).into(),
            graph::FloatLocalId(0).into(),
            graph::StringLocalId(0).into(),
            graph::BitArrayLocalId(0).into(),
            graph::UtfCodepointLocalId(0).into(),
            graph::CustomLocalId(0).into(),
            graph::ExternalLocalId(0).into(),
            graph::BoolLocalId(0).into(),
            graph::NilLocalId(0).into(),
            graph::TupleLocalId(0).into(),
        ];
        for (index, address) in addresses.iter().enumerate() {
            assert_eq!(address.index, 0);
            assert!(!addresses[..index].contains(address));
        }
    }

    #[test]
    fn checks_definition_order_operand_ownership_and_descriptor_agreement() {
        let lists = ListTypeTable {
            types: Table::Static(&[]),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable {
            definitions: Table::Static(&[]),
            types: Table::Static(&[]),
        };
        let externals = ExternalTypeTable {
            types: Table::Static(&[]),
        };
        let shapes = ValueShapeTable {
            shapes: Table::Static(&[
                ValueShapeDescriptor::Int,
                ValueShapeDescriptor::Tuple(Table::Static(&[ValueShapeId(0)])),
            ]),
            shape_types: Table::Static(&[
                ValueType::Int,
                ValueType::Tuple(Table::Static(&[ValueType::Int])),
            ]),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let first = ParamSlot {
            local: ParamLocal::Int(graph::IntLocalId(0)),
            shape: ValueShapeId(0),
        };
        let second = ParamSlot {
            local: ParamLocal::Int(graph::IntLocalId(1)),
            shape: ValueShapeId(0),
        };
        let tuple = ParamSlot {
            local: ParamLocal::Tuple {
                local: graph::TupleLocalId(0),
                type_: Table::Static(&[ValueType::Int]),
            },
            shape: ValueShapeId(1),
        };
        let mut locals = Locals::default();
        assert_eq!(
            locals.get(graph::TupleLocalId(0)),
            Err(LocalError::Missing(graph::TupleLocalId(0).into()))
        );
        let out_of_order = ParamSlot {
            local: ParamLocal::Tuple {
                local: graph::TupleLocalId(1),
                type_: Table::Static(&[ValueType::Int]),
            },
            shape: ValueShapeId(1),
        };
        assert_eq!(
            locals.define(&out_of_order, &types),
            Err(LocalError::DefinitionOrder {
                address: graph::TupleLocalId(1).into(),
                expected: 0,
            })
        );
        let invalid_tuples = [
            (
                ParamSlot {
                    local: tuple.local.clone(),
                    shape: ValueShapeId(99),
                },
                LocalError::Type(TypeError::MissingShape { index: 99 }),
            ),
            (
                ParamSlot {
                    local: tuple.local.clone(),
                    shape: ValueShapeId(0),
                },
                LocalError::Type(TypeError::LocalTypeMismatch),
            ),
            (
                ParamSlot {
                    local: ParamLocal::Tuple {
                        local: graph::TupleLocalId(0),
                        type_: Table::Static(&[ValueType::Bool]),
                    },
                    shape: ValueShapeId(1),
                },
                LocalError::Type(TypeError::LocalTypeMismatch),
            ),
        ];
        for (invalid, expected) in &invalid_tuples {
            assert_eq!(locals.define(invalid, &types).as_ref(), Err(expected));
        }
        assert_eq!(
            locals.get(graph::IntLocalId(0)),
            Err(LocalError::Missing(graph::IntLocalId(0).into()))
        );
        assert_eq!(
            locals.define(&second, &types),
            Err(LocalError::DefinitionOrder {
                address: graph::IntLocalId(1).into(),
                expected: 0
            })
        );
        assert_eq!(locals.define(&first, &types), Ok(()));
        assert_eq!(locals.define(&second, &types), Ok(()));
        assert_eq!(
            locals.define(&first, &types),
            Err(LocalError::DefinitionOrder {
                address: graph::IntLocalId(0).into(),
                expected: 2
            })
        );
        assert!(std::ptr::eq(
            &*locals.get(graph::IntLocalId(1)).unwrap(),
            &second
        ));
        assert!(std::ptr::eq(
            &*locals.parameter(&first.local).unwrap(),
            &first
        ));
        assert_eq!(
            locals.get(graph::StringLocalId(0)),
            Err(LocalError::Missing(graph::StringLocalId(0).into()))
        );
        assert_eq!(
            locals.get(graph::IntLocalId(2)),
            Err(LocalError::Missing(graph::IntLocalId(2).into()))
        );
        assert_eq!(locals.define(&tuple, &types), Ok(()));
        let stored_tuple = locals.tuple(graph::TupleLocalId(0)).unwrap();
        assert!(std::ptr::eq(&*stored_tuple.slot, &tuple));
        assert_eq!(stored_tuple.elements, &[ValueShapeId(0)]);
        let wrong = ParamLocal::Tuple {
            local: graph::TupleLocalId(0),
            type_: Table::Static(&[ValueType::String]),
        };
        assert_eq!(
            locals.parameter(&wrong),
            Err(LocalError::Metadata(graph::TupleLocalId(0).into()))
        );
        let invalid = ParamSlot {
            local: ParamLocal::String(graph::StringLocalId(0)),
            shape: ValueShapeId(0),
        };
        assert_eq!(
            locals.define(&invalid, &types),
            Err(LocalError::Type(TypeError::LocalTypeMismatch))
        );
        assert_eq!(
            locals.get(graph::StringLocalId(0)),
            Err(LocalError::Missing(graph::StringLocalId(0).into()))
        );
    }

    #[test]
    fn list_definitions_retain_item_shapes_and_reject_invalid_slots_before_reads() {
        use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
        let source = "pub fn main() { [42] }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let graph = plan.program.functions.list_returns.int_list_functions[0]
            .1
            .body()
            .block_graph();
        let slots = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .map(|instruction| instruction.output())
            .collect::<Vec<_>>();
        assert_eq!(slots.len(), 2);
        let integer = slots[0];
        let list = slots[1];
        assert_eq!(integer.local, ParamLocal::Int(graph::IntLocalId(0)));
        assert_eq!(
            list.local,
            ParamLocal::List(graph::ListLocal::Int {
                local: graph::IntListLocalId(0),
                type_id: IntListTypeId {
                    list_type: ListTypeId(0)
                },
            })
        );
        let mut locals = Locals::default();
        assert_eq!(
            locals.list(graph::IntListLocalId(0)),
            Err(LocalError::Missing(graph::IntListLocalId(0).into()))
        );
        let bad_slots = [
            (
                ParamSlot {
                    local: list.local.clone(),
                    shape: ValueShapeId(999),
                },
                TypeError::MissingShape { index: 999 },
            ),
            (
                ParamSlot {
                    local: list.local.clone(),
                    shape: integer.shape,
                },
                TypeError::LocalTypeMismatch,
            ),
            (
                ParamSlot {
                    local: ParamLocal::List(graph::ListLocal::Int {
                        local: graph::IntListLocalId(0),
                        type_id: IntListTypeId {
                            list_type: ListTypeId(999),
                        },
                    }),
                    shape: list.shape,
                },
                TypeError::MissingList { index: 999 },
            ),
        ]
        .map(|(slot, error)| (slot, LocalError::Type(error)));
        for (slot, error) in &bad_slots {
            assert_eq!(locals.define(slot, &types).as_ref(), Err(error));
        }
        assert_eq!(locals.define(list, &types), Ok(()));
        let admitted = locals.list(graph::IntListLocalId(0)).unwrap();
        assert!(std::ptr::eq(&*admitted.slot, list));
        assert_eq!(admitted.item, integer.shape);
        assert_eq!(locals.get(graph::IntListLocalId(0)), Ok(admitted.slot));
        assert_eq!(
            locals.define(list, &types),
            Err(LocalError::DefinitionOrder {
                address: graph::IntListLocalId(0).into(),
                expected: 1,
            })
        );
        assert_eq!(
            locals.list(graph::IntListLocalId(1)),
            Err(LocalError::Missing(graph::IntListLocalId(1).into()))
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::List(crate::ListValue::int(vec![42.into()]))
        );
    }
}
