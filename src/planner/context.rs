use crate::plan::LocalId;
use ecow::EcoString;
use std::collections::{HashMap, HashSet};

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    pub(super) function_names: &'a HashSet<EcoString>,
    locals: HashMap<EcoString, LocalId>,
    next_local: usize,
}

impl<'a> PlanContext<'a> {
    pub(super) fn new(module_name: &'a EcoString, function_names: &'a HashSet<EcoString>) -> Self {
        Self {
            module_name,
            function_names,
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
}
