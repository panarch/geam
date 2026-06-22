use crate::plan::{BoolLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId, ValueType};
use ecow::EcoString;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(super) struct LocalTable {
    next_int: usize,
    next_string: usize,
    next_bool: usize,
    next_nil: usize,
    locals: HashMap<EcoString, LocalId>,
}

impl LocalTable {
    pub(super) fn define(&mut self, name: EcoString, type_: ValueType) -> LocalId {
        match type_ {
            ValueType::Int => LocalId::Int(self.define_int(name)),
            ValueType::String => LocalId::String(self.define_string(name)),
            ValueType::Bool => LocalId::Bool(self.define_bool(name)),
            ValueType::Nil => LocalId::Nil(self.define_nil(name)),
        }
    }

    pub(super) fn define_int(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int);
        self.next_int += 1;
        self.locals.insert(name, LocalId::Int(local));
        local
    }

    pub(super) fn define_string(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string);
        self.next_string += 1;
        self.locals.insert(name, LocalId::String(local));
        local
    }

    pub(super) fn define_bool(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool);
        self.next_bool += 1;
        self.locals.insert(name, LocalId::Bool(local));
        local
    }

    pub(super) fn define_nil(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil);
        self.next_nil += 1;
        self.locals.insert(name, LocalId::Nil(local));
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

        assert_eq!(locals.define_int("a".into()), IntLocalId(0));
        assert_eq!(locals.define_string("b".into()), StringLocalId(0));
        assert_eq!(locals.define_bool("c".into()), BoolLocalId(0));
        assert_eq!(locals.define_nil("d".into()), NilLocalId(0));
        assert_eq!(locals.lookup(&"a".into()), LocalId::Int(IntLocalId(0)));
        assert_eq!(
            locals.lookup(&"b".into()),
            LocalId::String(StringLocalId(0))
        );
        assert_eq!(locals.lookup(&"c".into()), LocalId::Bool(BoolLocalId(0)));
        assert_eq!(locals.lookup(&"d".into()), LocalId::Nil(NilLocalId(0)));
    }

    #[test]
    fn local_table_define_shadow() {
        let mut locals = LocalTable::default();

        assert_eq!(locals.define_int("x".into()), IntLocalId(0));
        assert_eq!(locals.define_int("x".into()), IntLocalId(1));
        assert_eq!(locals.lookup(&"x".into()), LocalId::Int(IntLocalId(1)));
    }

    #[test]
    fn local_table_define_by_type() {
        let mut locals = LocalTable::default();

        assert_eq!(
            locals.define("x".into(), ValueType::Bool),
            LocalId::Bool(BoolLocalId(0)),
        );
        assert_eq!(locals.lookup(&"x".into()), LocalId::Bool(BoolLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "unknown local `missing` in planner DSL")]
    fn local_table_lookup_missing() {
        LocalTable::default().lookup(&"missing".into());
    }
}
