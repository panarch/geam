use crate::plan::ValueType;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::TypeMetadata;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ConstructionIndex<Id: 'static> {
    pub entries: Table<(TypeMetadata, Id)>,
}

impl<Id: Copy> ConstructionIndex<Id> {
    pub(super) fn new(types: HashMap<ValueType, Id>) -> Self {
        let mut entries = types
            .into_iter()
            .map(|(type_, id)| (TypeMetadata::from_public(&type_), id))
            .collect::<Vec<_>>();
        entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        Self {
            entries: entries.into(),
        }
    }

    pub(super) fn get(&self, type_: &ValueType) -> Id {
        // Sealing or prepared admission has checked this key in the sorted table.
        let index = self
            .entries
            .partition_point(|(candidate, _)| candidate.compare(type_).is_lt());
        self.entries[index].1
    }
}

impl<Id: 'static> Emit for ConstructionIndex<Id>
where
    Table<(TypeMetadata, Id)>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self { entries } = self;
        output.structure("host::ConstructionIndex", &[("entries", entries)]);
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstructionIndex, HashMap, Table, TypeMetadata, ValueType};

    #[test]
    fn owned_indexes_freeze_independently_of_map_insertion_order() {
        let index = ConstructionIndex::new(HashMap::from([
            (ValueType::String, 7),
            (ValueType::Int, 3),
            (ValueType::Bool, 11),
        ]));
        assert_eq!(index.get(&ValueType::Int), 3);
        assert_eq!(index.get(&ValueType::String), 7);
        assert_eq!(index.get(&ValueType::Bool), 11);
        assert_eq!(
            index.entries.as_ref(),
            &[
                (TypeMetadata::Int, 3),
                (TypeMetadata::String, 7),
                (TypeMetadata::Bool, 11),
            ]
        );
    }

    #[test]
    fn static_indexes_search_nested_types_without_rebuilding_the_table() {
        static ELEMENT: TypeMetadata = TypeMetadata::String;
        static ENTRIES: [(TypeMetadata, usize); 2] = [
            (TypeMetadata::Int, 2),
            (
                TypeMetadata::List(crate::plan::execution::storage::Node::Static(&ELEMENT)),
                13,
            ),
        ];
        let index = ConstructionIndex {
            entries: Table::Static(&ENTRIES),
        };
        let cloned = index.clone();
        assert!(std::ptr::eq(index.entries.as_ptr(), ENTRIES.as_ptr()));
        assert!(std::ptr::eq(cloned.entries.as_ptr(), ENTRIES.as_ptr()));
        assert_eq!(index.get(&ValueType::List(Box::new(ValueType::String))), 13);
        assert_eq!(cloned.get(&ValueType::Int), 2);
    }
}
