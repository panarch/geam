use crate::plan::{
    BoolFunctionId, BoolLocalId, FunctionFunctionId, FunctionId, FunctionLocalId, FunctionPlan,
    FunctionType, IntFunctionId, IntLocalId, LocalId, NilFunctionId, NilLocalId, RuntimeFunctionId,
    StringFunctionId, StringLocalId, ValueType,
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
    pub(super) type_: Option<FunctionType>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    pub(super) local: LocalId,
    pub(super) name: EcoString,
    pub(super) type_: ValueType,
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    functions: &'a HashMap<EcoString, FunctionInfo>,
    state: &'a mut FunctionPlanState,
    locals: HashMap<EcoString, (LocalId, ValueType)>,
    outer_locals: Option<std::collections::HashSet<EcoString>>,
    next_int_local: usize,
    next_string_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
    next_function_local: usize,
}

impl<'a> PlanContext<'a> {
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        state: &'a mut FunctionPlanState,
    ) -> Self {
        Self {
            module_name,
            functions,
            state,
            locals: HashMap::new(),
            outer_locals: None,
            next_int_local: 0,
            next_string_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_function_local: 0,
        }
    }

    pub(super) fn define_existing_local(
        &mut self,
        name: EcoString,
        local: LocalId,
        type_: ValueType,
    ) {
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
            LocalId::Function(local) => {
                self.next_function_local = self.next_function_local.max(local.0 + 1);
            }
        }
        self.locals.insert(name, (local, type_));
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.locals
            .insert(name, (LocalId::Int(local), ValueType::Int));
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.locals
            .insert(name, (LocalId::String(local), ValueType::String));
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.locals
            .insert(name, (LocalId::Bool(local), ValueType::Bool));
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.locals
            .insert(name, (LocalId::Nil(local), ValueType::Nil));
        local
    }

    pub(super) fn define_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FunctionLocalId {
        let local = FunctionLocalId(self.next_function_local);
        self.next_function_local += 1;
        self.locals.insert(
            name,
            (
                LocalId::Function(local),
                ValueType::Function(Box::new(type_)),
            ),
        );
        local
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        self.locals.get(name).cloned()
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub(super) fn is_outer_local(&self, name: &EcoString) -> bool {
        self.outer_locals
            .as_ref()
            .is_some_and(|outer_locals| outer_locals.contains(name))
    }

    pub(super) fn push_anonymous_function(
        &mut self,
        return_type: ValueType,
        function: impl FnOnce(
            FunctionId,
            RuntimeFunctionId,
            &mut Self,
        ) -> Result<FunctionPlan, crate::planner::PlanError>,
    ) -> Result<RuntimeFunctionId, crate::planner::PlanError> {
        let (id, runtime_id) = self.state.next_function(return_type);
        let function = function(id, runtime_id, self)?;
        self.state.push_function(function);
        Ok(runtime_id)
    }

    pub(super) fn with_anonymous_function_scope<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let locals = self.locals.clone();
        let outer_locals = Some(locals.keys().cloned().collect());
        let current_locals = std::mem::take(&mut self.locals);
        let previous_outer = std::mem::replace(&mut self.outer_locals, outer_locals);
        let next_int = std::mem::replace(&mut self.next_int_local, 0);
        let next_string = std::mem::replace(&mut self.next_string_local, 0);
        let next_bool = std::mem::replace(&mut self.next_bool_local, 0);
        let next_nil = std::mem::replace(&mut self.next_nil_local, 0);
        let next_function = std::mem::replace(&mut self.next_function_local, 0);

        let result = f(self);

        self.locals = current_locals;
        self.outer_locals = previous_outer;
        self.next_int_local = next_int;
        self.next_string_local = next_string;
        self.next_bool_local = next_bool;
        self.next_nil_local = next_nil;
        self.next_function_local = next_function;

        result
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

pub(super) struct FunctionPlanState {
    runtime_ids: FunctionRuntimeIds,
    next_function_index: usize,
    anonymous_functions: Vec<FunctionPlan>,
}

impl FunctionPlanState {
    pub(super) fn new(runtime_ids: FunctionRuntimeIds, next_function_index: usize) -> Self {
        Self {
            runtime_ids,
            next_function_index,
            anonymous_functions: Vec::new(),
        }
    }

    pub(super) fn next_function(
        &mut self,
        return_type: ValueType,
    ) -> (FunctionId, RuntimeFunctionId) {
        let id = FunctionId::new(self.next_function_index);
        self.next_function_index += 1;
        let runtime_id = self.runtime_ids.next(return_type);

        (id, runtime_id)
    }

    fn push_function(&mut self, function: FunctionPlan) {
        self.anonymous_functions.push(function);
    }

    pub(super) fn into_anonymous_functions(self) -> Vec<FunctionPlan> {
        self.anonymous_functions
    }
}

#[derive(Debug, Default)]
pub(super) struct FunctionRuntimeIds {
    next_int: usize,
    next_string: usize,
    next_bool: usize,
    next_nil: usize,
    next_function: usize,
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
            ValueType::Function(_) => {
                let id = FunctionFunctionId(self.next_function);
                self.next_function += 1;
                RuntimeFunctionId::Function(id)
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
        } else if let Some((arguments, return_)) = type_.fn_types() {
            let arguments = arguments
                .iter()
                .map(|argument| Self::from_gleam(argument.as_ref()))
                .collect::<Option<Vec<_>>>()?;
            let return_ = Self::from_gleam(return_.as_ref())?;
            Some(Self::Function(Box::new(FunctionType::new(
                arguments, return_,
            ))))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionInfo, PlanContext};
    use crate::plan::{IntLocalId, LocalId, ValueType};
    use ecow::EcoString;
    use std::collections::HashMap;

    #[test]
    fn local_scope_restores_names_after_error_without_reusing_ids() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut state = super::FunctionPlanState::new(Default::default(), 0);
        let mut context = PlanContext::new(&module, &functions, &mut state);

        assert_eq!(context.define_int_local("x".into()), IntLocalId(0));
        let result = context.with_local_scope(|context| {
            assert_eq!(context.define_int_local("x".into()), IntLocalId(1));
            Err::<(), _>(())
        });

        assert_eq!(result, Err(()));
        assert_eq!(
            context.lookup_local(&"x".into()),
            Some((LocalId::Int(IntLocalId(0)), ValueType::Int))
        );
        assert_eq!(context.define_int_local("y".into()), IntLocalId(2));
    }
}
