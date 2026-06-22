use crate::plan::{
    BoolLocalId, FunctionId, IntLocalId, LocalId, NilLocalId, StringLocalId, ValueType,
};
use ecow::EcoString;
use gleam_core::type_::Type;
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
    next_int_local: usize,
    next_string_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
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
            next_int_local: 0,
            next_string_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
        }
    }

    pub(super) fn define_local(&mut self, name: EcoString, type_: ValueType) -> LocalId {
        match type_ {
            ValueType::Int => LocalId::Int(self.define_int_local(name.clone())),
            ValueType::String => LocalId::String(self.define_string_local(name.clone())),
            ValueType::Bool => LocalId::Bool(self.define_bool_local(name.clone())),
            ValueType::Nil => LocalId::Nil(self.define_nil_local(name.clone())),
        }
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.locals.insert(name, LocalId::Int(local));
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.locals.insert(name, LocalId::String(local));
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.locals.insert(name, LocalId::Bool(local));
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.locals.insert(name, LocalId::Nil(local));
        local
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<LocalId> {
        self.locals.get(name).copied()
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).copied()
    }
}

impl ValueType {
    pub(super) fn from_gleam(type_: &Type) -> Option<Self> {
        if type_.is_int() {
            Some(Self::Int)
        } else if type_.is_string() {
            Some(Self::String)
        } else if type_.is_bool() {
            Some(Self::Bool)
        } else if type_.is_nil() {
            Some(Self::Nil)
        } else {
            None
        }
    }
}
