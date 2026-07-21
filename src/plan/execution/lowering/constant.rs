use std::collections::HashMap;

use crate::plan::execution::{ConstantId, ConstantProgram, ConstantTable, ConstantValue};
use crate::plan::module::ConstantInstantiation;

#[derive(Default)]
pub(super) struct ConstantLowering {
    indices: HashMap<ConstantInstantiation, usize>,
    table: ConstantTable,
}

impl ConstantLowering {
    pub(super) fn get<Value>(&self, key: &ConstantInstantiation) -> Option<ConstantId<Value>> {
        self.indices.get(key).copied().map(ConstantId::new)
    }

    pub(super) fn insert<Value: ConstantValue>(
        &mut self,
        key: ConstantInstantiation,
        program: ConstantProgram<Value>,
    ) -> ConstantId<Value> {
        let id = self.table.push(program);
        self.indices.insert(key, id.index());
        id
    }

    pub(super) fn finish(self) -> ConstantTable {
        self.table
    }
}
