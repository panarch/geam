use super::NominalTypeMetadata;
use crate::plan;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct ExternalTypeTable {
    pub types: Table<NominalTypeMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalTypeId(pub usize);

impl ExternalTypeTable {
    pub(in crate::plan::execution) fn new(types: Vec<plan::ExternalType>) -> Self {
        Self {
            types: types
                .iter()
                .map(NominalTypeMetadata::from_external)
                .collect(),
        }
    }

    pub(crate) fn value_type(&self, id: ExternalTypeId) -> plan::ExternalType {
        self.types[id.index()].external_type()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.types.len()
    }
}

impl Default for ExternalTypeTable {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl ExternalTypeId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl Emit for ExternalTypeTable {
    fn emit(&self, output: &mut Rust) {
        let Self { types } = self;
        output.structure("type_::ExternalTypeTable", &[("types", types)]);
    }
}

impl Emit for ExternalTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::ExternalTypeId", &[field_0]);
    }
}

#[cfg(test)]
mod tests {
    use super::{ExternalTypeId, ExternalTypeTable};
    use crate::plan::{ExternalType, ExternalTypeName, ValueType};

    #[test]
    fn external_type_table_preserves_nominal_type_arguments() {
        let type_ = ExternalType::new(
            ExternalTypeName::new("domain".into(), "domain/box".into(), "Box".into()),
            vec![ValueType::Int],
        );
        let table = ExternalTypeTable::new(vec![type_.clone()]);

        assert_eq!(table.len(), 1);
        assert_eq!(table.value_type(ExternalTypeId::new(0)), type_);
        assert_eq!(ExternalTypeTable::default().len(), 0);
    }
}
