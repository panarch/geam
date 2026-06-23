use crate::plan::{
    BoolFunctionId, BoolLocalId, FunctionId, IntFunctionId, IntLocalId, LocalId, NilFunctionId,
    NilLocalId, RuntimeFunctionId, StringFunctionId, StringLocalId, ValueType,
};
use ecow::EcoString;
use gleam_core::type_::Type;
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) id: FunctionId,
    pub(super) runtime_id: Option<RuntimeFunctionId>,
    pub(super) arity: usize,
    pub(super) params: Vec<FunctionParam>,
    pub(super) return_type: Option<ValueType>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    pub(super) local: LocalId,
    pub(super) name: EcoString,
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

    pub(super) fn define_existing_local(&mut self, name: EcoString, local: LocalId) {
        match local {
            LocalId::Int(local) => {
                self.next_int_local = self.next_int_local.max(local.0 + 1);
            }
            LocalId::String(local) => {
                self.next_string_local = self.next_string_local.max(local.0 + 1);
            }
            LocalId::Bool(local) => {
                self.next_bool_local = self.next_bool_local.max(local.0 + 1);
            }
            LocalId::Nil(local) => {
                self.next_nil_local = self.next_nil_local.max(local.0 + 1);
            }
        }
        self.locals.insert(name, local);
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
        self.functions.get(name).cloned()
    }

    pub(super) fn with_local_scope<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let locals = self.locals.clone();
        let result = f(self);
        self.locals = locals;
        result
    }
}

#[derive(Debug, Default)]
pub(super) struct FunctionRuntimeIds {
    next_int: usize,
    next_string: usize,
    next_bool: usize,
    next_nil: usize,
}

impl FunctionRuntimeIds {
    pub(super) fn next(&mut self, return_type: ValueType) -> RuntimeFunctionId {
        match return_type {
            ValueType::Int => {
                let id = IntFunctionId(self.next_int);
                self.next_int += 1;
                RuntimeFunctionId::Int(id)
            }
            ValueType::String => {
                let id = StringFunctionId(self.next_string);
                self.next_string += 1;
                RuntimeFunctionId::String(id)
            }
            ValueType::Bool => {
                let id = BoolFunctionId(self.next_bool);
                self.next_bool += 1;
                RuntimeFunctionId::Bool(id)
            }
            ValueType::Nil => {
                let id = NilFunctionId(self.next_nil);
                self.next_nil += 1;
                RuntimeFunctionId::Nil(id)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::{FunctionInfo, PlanContext};
    use crate::plan::{IntLocalId, LocalId};
    use ecow::EcoString;
    use std::collections::HashMap;

    #[test]
    fn local_scope_restores_names_after_error_without_reusing_ids() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut context = PlanContext::new(&module, &functions);

        assert_eq!(context.define_int_local("x".into()), IntLocalId(0));
        let result = context.with_local_scope(|context| {
            assert_eq!(context.define_int_local("x".into()), IntLocalId(1));
            Err::<(), _>(())
        });

        assert_eq!(result, Err(()));
        assert_eq!(
            context.lookup_local(&"x".into()),
            Some(LocalId::Int(IntLocalId(0)))
        );
        assert_eq!(context.define_int_local("y".into()), IntLocalId(2));
    }
}
