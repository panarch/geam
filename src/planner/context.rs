use crate::plan::{
    BoolFunctionId, BoolLocalId, FunctionId, FunctionType, FunctionValue, IntFunctionId,
    IntLocalId, LocalId, NilFunctionId, NilLocalId, RuntimeFunctionId, StringFunctionId,
    StringLocalId, ValueType,
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
    locals: HashMap<EcoString, (LocalId, ValueType)>,
    function_values: HashMap<EcoString, FunctionValue>,
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
            function_values: HashMap::new(),
            next_int_local: 0,
            next_string_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
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
        }
        self.locals.insert(name, (local, type_));
    }

    pub(super) fn define_function_alias(&mut self, name: EcoString, value: FunctionValue) {
        self.function_values.insert(name, value);
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

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        self.locals.get(name).cloned()
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub(super) fn lookup_function_value(&self, name: &EcoString) -> Option<FunctionValue> {
        self.function_values.get(name).cloned()
    }

    pub(super) fn with_local_scope<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let locals = self.locals.clone();
        let function_values = self.function_values.clone();
        let result = f(self);
        self.locals = locals;
        self.function_values = function_values;
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
    pub(super) fn next(&mut self, return_type: ValueType) -> Option<RuntimeFunctionId> {
        match return_type {
            ValueType::Int => {
                let id = IntFunctionId(self.next_int);
                self.next_int += 1;
                Some(RuntimeFunctionId::Int(id))
            }
            ValueType::String => {
                let id = StringFunctionId(self.next_string);
                self.next_string += 1;
                Some(RuntimeFunctionId::String(id))
            }
            ValueType::Bool => {
                let id = BoolFunctionId(self.next_bool);
                self.next_bool += 1;
                Some(RuntimeFunctionId::Bool(id))
            }
            ValueType::Nil => {
                let id = NilFunctionId(self.next_nil);
                self.next_nil += 1;
                Some(RuntimeFunctionId::Nil(id))
            }
            ValueType::Function(_) => None,
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
    use super::{FunctionInfo, FunctionRuntimeIds, PlanContext};
    use crate::plan::{
        FunctionType, FunctionValue, IntFunctionId, IntLocalId, LocalId, RuntimeFunctionId,
        ValueType,
    };
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
            Some((LocalId::Int(IntLocalId(0)), ValueType::Int))
        );
        assert_eq!(context.define_int_local("y".into()), IntLocalId(2));
    }

    #[test]
    fn define_function_alias_records_value() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut context = PlanContext::new(&module, &functions);
        let value = FunctionValue::new(
            FunctionType::new(Vec::new(), ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
        );

        context.define_function_alias("f".into(), value.clone());

        assert_eq!(context.lookup_function_value(&"f".into()), Some(value));
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn function_runtime_ids_skip_function_return_ids() {
        let mut ids = FunctionRuntimeIds::default();

        assert_eq!(
            ids.next(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            )))),
            None,
        );
    }
}
