use crate::plan::{
    BoolFunctionId, BoolLocalId, FunctionArgumentType, FunctionId, FunctionType, FunctionValue,
    IntFunctionId, IntLocalId, LocalId, NilFunctionId, NilLocalId, RuntimeFunctionId,
    StringFunctionId, StringLocalId, ValueType,
};
use ecow::EcoString;
use gleam_core::type_::Type;
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) id: FunctionId,
    pub(super) runtime_id: RuntimeFunctionId,
    pub(super) params: Vec<FunctionParam>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    pub(super) local: LocalId,
    pub(super) name: EcoString,
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    functions: &'a HashMap<EcoString, FunctionInfo>,
    bindings: HashMap<EcoString, LocalBinding>,
    next_int_local: usize,
    next_string_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
}

#[derive(Clone)]
enum LocalBinding {
    Primitive { local: LocalId, type_: ValueType },
    Function(FunctionValue),
}

impl<'a> PlanContext<'a> {
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
    ) -> Self {
        Self {
            module_name,
            functions,
            bindings: HashMap::new(),
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
        self.bindings
            .insert(name, LocalBinding::Primitive { local, type_ });
    }

    pub(super) fn define_function_alias(&mut self, name: EcoString, value: FunctionValue) {
        self.bindings.insert(name, LocalBinding::Function(value));
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Primitive {
                local: LocalId::Int(local),
                type_: ValueType::Int,
            },
        );
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Primitive {
                local: LocalId::String(local),
                type_: ValueType::String,
            },
        );
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Primitive {
                local: LocalId::Bool(local),
                type_: ValueType::Bool,
            },
        );
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Primitive {
                local: LocalId::Nil(local),
                type_: ValueType::Nil,
            },
        );
        local
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        match self.bindings.get(name)? {
            LocalBinding::Primitive { local, type_ } => Some((*local, type_.clone())),
            LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub(super) fn lookup_function_value(&self, name: &EcoString) -> Option<FunctionValue> {
        match self.bindings.get(name)? {
            LocalBinding::Function(value) => Some(value.clone()),
            LocalBinding::Primitive { .. } => None,
        }
    }

    pub(super) fn with_local_scope<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let bindings = self.bindings.clone();
        let result = f(self);
        self.bindings = bindings;
        result
    }
}

impl FunctionInfo {
    pub(super) fn arity(&self) -> usize {
        self.params.len()
    }

    pub(super) fn return_type(&self) -> ValueType {
        self.runtime_id.value_type()
    }

    pub(super) fn value(&self) -> FunctionValue {
        FunctionValue::new(
            self.runtime_id,
            self.params.iter().map(|param| param.local).collect(),
        )
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
    pub(super) fn next_int(&mut self) -> RuntimeFunctionId {
        let id = IntFunctionId(self.next_int);
        self.next_int += 1;
        RuntimeFunctionId::Int(id)
    }

    pub(super) fn next_string(&mut self) -> RuntimeFunctionId {
        let id = StringFunctionId(self.next_string);
        self.next_string += 1;
        RuntimeFunctionId::String(id)
    }

    pub(super) fn next_bool(&mut self) -> RuntimeFunctionId {
        let id = BoolFunctionId(self.next_bool);
        self.next_bool += 1;
        RuntimeFunctionId::Bool(id)
    }

    pub(super) fn next_nil(&mut self) -> RuntimeFunctionId {
        let id = NilFunctionId(self.next_nil);
        self.next_nil += 1;
        RuntimeFunctionId::Nil(id)
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
                .map(|argument| {
                    Self::from_gleam(argument.as_ref())
                        .and_then(|type_| FunctionArgumentType::from_value_type(&type_))
                })
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
        FunctionValue, IntFunctionId, IntLocalId, LocalId, RuntimeFunctionId, ValueType,
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
        let value = function_value();

        context.define_function_alias("f".into(), value.clone());

        assert_eq!(context.lookup_function_value(&"f".into()), Some(value));
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn function_alias_shadows_primitive_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut context = PlanContext::new(&module, &functions);
        let value = function_value();

        context.define_int_local("f".into());
        context.define_function_alias("f".into(), value.clone());

        assert_eq!(context.lookup_function_value(&"f".into()), Some(value));
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn primitive_binding_shadows_function_alias() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut context = PlanContext::new(&module, &functions);

        context.define_function_alias("f".into(), function_value());
        let local = context.define_int_local("f".into());

        assert_eq!(context.lookup_function_value(&"f".into()), None);
        assert_eq!(
            context.lookup_local(&"f".into()),
            Some((LocalId::Int(local), ValueType::Int))
        );
    }

    #[test]
    fn function_runtime_ids_allocate_by_return_type() {
        let mut ids = FunctionRuntimeIds::default();

        assert_eq!(ids.next_int(), RuntimeFunctionId::Int(IntFunctionId(0)));
        assert_eq!(ids.next_int(), RuntimeFunctionId::Int(IntFunctionId(1)));
        assert_eq!(
            ids.next_string(),
            RuntimeFunctionId::String(crate::plan::StringFunctionId(0))
        );
        assert_eq!(
            ids.next_bool(),
            RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0))
        );
        assert_eq!(
            ids.next_nil(),
            RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0))
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new())
    }
}
