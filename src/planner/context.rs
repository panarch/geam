use crate::plan::{FunctionId, LocalId};
use ecow::EcoString;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FunctionInfo {
    pub(super) id: FunctionId,
    pub(super) arity: usize,
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    functions: &'a HashMap<EcoString, FunctionInfo>,
    locals: HashMap<EcoString, LocalId>,
    next_local: usize,
}

impl<'a> PlanContext<'a> {
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
    ) -> Self {
        Self {
            module_name,
            functions,
            locals: HashMap::new(),
            next_local: 0,
        }
    }

    pub(super) fn define_local(&mut self, name: EcoString) -> LocalId {
        let local = LocalId(self.next_local);
        self.next_local += 1;
        self.locals.insert(name, local);
        local
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<LocalId> {
        self.locals.get(name).copied()
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).copied()
    }
}
