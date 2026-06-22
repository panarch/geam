use crate::plan::LocalId;
use ecow::EcoString;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(super) struct LocalTable {
    next: usize,
    locals: HashMap<EcoString, LocalId>,
}

impl LocalTable {
    pub(super) fn define(&mut self, name: EcoString) -> LocalId {
        let local = LocalId(self.next);
        self.next += 1;
        self.locals.insert(name, local);
        local
    }

    pub(super) fn lookup(&self, name: &EcoString) -> LocalId {
        self.locals
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("unknown local `{name}` in planner DSL"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_table_define() {
        let mut locals = LocalTable::default();

        assert_eq!(locals.define("a".into()), LocalId(0));
        assert_eq!(locals.define("b".into()), LocalId(1));
        assert_eq!(locals.lookup(&"a".into()), LocalId(0));
        assert_eq!(locals.lookup(&"b".into()), LocalId(1));
    }

    #[test]
    fn local_table_define_shadow() {
        let mut locals = LocalTable::default();

        assert_eq!(locals.define("x".into()), LocalId(0));
        assert_eq!(locals.define("x".into()), LocalId(1));
        assert_eq!(locals.lookup(&"x".into()), LocalId(1));
    }

    #[test]
    #[should_panic(expected = "unknown local `missing` in planner DSL")]
    fn local_table_lookup_missing() {
        LocalTable::default().lookup(&"missing".into());
    }
}
