use crate::plan::{
    BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
    BoolLocalId, CaptureArg, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId,
    FloatFunctionId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionExpr,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionId,
    FunctionPlan, FunctionType, FunctionValue, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
    IntFunctionId, IntFunctionLocalId, IntLocalId, LocalId, NilExpr, NilFunctionExpr,
    NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilLocalId, ParamBinding, ParamLocal,
    RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringLocalId, ValueType,
};
use crate::planner::error::{InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_core::type_::Type;
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) id: FunctionId,
    pub(super) runtime_id: RuntimeFunctionId,
    pub(super) return_type: ValueType,
    pub(super) params: Vec<FunctionParam>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    pub(super) local: ParamLocal,
    pub(super) binding: ParamBinding,
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    functions: &'a HashMap<EcoString, FunctionInfo>,
    anonymous_functions: &'a mut AnonymousFunctions,
    bindings: HashMap<EcoString, LocalBinding>,
    next_int_local: usize,
    next_float_local: usize,
    next_string_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
    next_int_function_local: usize,
    next_float_function_local: usize,
    next_string_function_local: usize,
    next_bool_function_local: usize,
    next_nil_function_local: usize,
    next_function_function_local: usize,
}

#[derive(Clone)]
enum LocalBinding {
    Primitive(LocalId),
    Function(FunctionLocalBinding),
}

pub(super) struct CaptureBinding {
    name: EcoString,
    binding: LocalBinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionLocalBinding {
    Int {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    String {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    Float {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    Bool {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    Nil {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    Function {
        local: FunctionFunctionLocalId,
        type_: FunctionType,
    },
}

impl<'a> PlanContext<'a> {
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self {
            module_name,
            functions,
            anonymous_functions,
            bindings: HashMap::new(),
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_function_function_local: 0,
        }
    }

    pub(super) fn define_existing_local(&mut self, name: EcoString, local: LocalId) {
        match local {
            LocalId::Int(local) => {
                self.next_int_local = self.next_int_local.max(local.0 + 1);
            }
            LocalId::Float(local) => {
                self.next_float_local = self.next_float_local.max(local.0 + 1);
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
        self.bindings.insert(name, LocalBinding::Primitive(local));
    }

    pub(super) fn define_existing_param(&mut self, name: EcoString, local: &ParamLocal) {
        match local {
            ParamLocal::Int(local) => {
                self.define_existing_local(name, LocalId::Int(*local));
            }
            ParamLocal::Float(local) => {
                self.define_existing_local(name, LocalId::Float(*local));
            }
            ParamLocal::String(local) => {
                self.define_existing_local(name, LocalId::String(*local));
            }
            ParamLocal::Bool(local) => {
                self.define_existing_local(name, LocalId::Bool(*local));
            }
            ParamLocal::Nil(local) => {
                self.define_existing_local(name, LocalId::Nil(*local));
            }
            ParamLocal::IntFunction { local, type_ } => {
                self.next_int_function_local = self.next_int_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Int {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::FloatFunction { local, type_ } => {
                self.next_float_function_local = self.next_float_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Float {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::StringFunction { local, type_ } => {
                self.next_string_function_local = self.next_string_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::String {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::BoolFunction { local, type_ } => {
                self.next_bool_function_local = self.next_bool_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Bool {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::NilFunction { local, type_ } => {
                self.next_nil_function_local = self.next_nil_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Nil {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::FunctionFunction { local, type_ } => {
                self.next_function_function_local =
                    self.next_function_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Function {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
        }
    }

    pub(super) fn define_int_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Int { local, type_ }),
        );
        local
    }

    pub(super) fn define_string_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::String { local, type_ }),
        );
        local
    }

    pub(super) fn define_float_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Float { local, type_ }),
        );
        local
    }

    pub(super) fn define_bool_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Bool { local, type_ }),
        );
        local
    }

    pub(super) fn define_nil_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Nil { local, type_ }),
        );
        local
    }

    pub(super) fn define_function_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FunctionFunctionLocalId {
        let local = FunctionFunctionLocalId(self.next_function_function_local);
        self.next_function_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Function { local, type_ }),
        );
        local
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Int(local)));
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::String(local)));
        local
    }

    pub(super) fn define_float_local(&mut self, name: EcoString) -> FloatLocalId {
        let local = FloatLocalId(self.next_float_local);
        self.next_float_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Float(local)));
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Bool(local)));
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Nil(local)));
        local
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        match self.bindings.get(name)? {
            LocalBinding::Primitive(local) => Some((*local, local.value_type())),
            LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub(super) fn lookup_function_local(&self, name: &EcoString) -> Option<FunctionLocalBinding> {
        match self.bindings.get(name)? {
            LocalBinding::Function(binding) => Some(binding.clone()),
            LocalBinding::Primitive(_) => None,
        }
    }

    pub(super) fn anonymous_function_error_name(&self) -> EcoString {
        self.anonymous_functions.next_name()
    }

    pub(super) fn allocate_anonymous_function(
        &mut self,
        return_type: ValueType,
        params: Vec<FunctionParam>,
        runtime_id: RuntimeFunctionId,
    ) -> (EcoString, FunctionInfo) {
        self.anonymous_functions
            .allocate(return_type, params, runtime_id)
    }

    pub(super) fn allocate_anonymous_runtime_id(
        &mut self,
        return_type: &ValueType,
    ) -> RuntimeFunctionId {
        self.anonymous_functions.allocate_runtime_id(return_type)
    }

    pub(super) fn push_anonymous_function(&mut self, function: FunctionPlan) {
        self.anonymous_functions.push(function);
    }

    pub(super) fn anonymous_function_context(&mut self) -> PlanContext<'_> {
        PlanContext {
            module_name: self.module_name,
            functions: self.functions,
            anonymous_functions: self.anonymous_functions,
            bindings: HashMap::new(),
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_function_function_local: 0,
        }
    }

    pub(super) fn capture_bindings(
        &self,
        names: &[EcoString],
    ) -> Result<Vec<CaptureBinding>, PlanError> {
        names
            .iter()
            .map(|name| {
                let binding =
                    self.bindings
                        .get(name)
                        .cloned()
                        .ok_or_else(|| PlanError::InvalidTypedAst {
                            reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                        })?;

                Ok(CaptureBinding {
                    name: name.clone(),
                    binding,
                })
            })
            .collect()
    }

    pub(super) fn define_captures(
        &mut self,
        captures: Vec<CaptureBinding>,
    ) -> Result<Vec<CaptureArg>, PlanError> {
        captures
            .into_iter()
            .map(|capture| self.define_capture(capture))
            .collect()
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

    fn define_capture(&mut self, capture: CaptureBinding) -> Result<CaptureArg, PlanError> {
        match capture.binding {
            LocalBinding::Primitive(LocalId::Int(local)) => {
                let target = self.define_int_local(capture.name.clone());
                Ok(CaptureArg::int(
                    target,
                    IntExpr::local_get(local, capture.name),
                ))
            }
            LocalBinding::Primitive(LocalId::Float(local)) => {
                let target = self.define_float_local(capture.name.clone());
                Ok(CaptureArg::float(
                    target,
                    FloatExpr::local_get(local, capture.name),
                ))
            }
            LocalBinding::Primitive(LocalId::String(local)) => {
                let target = self.define_string_local(capture.name.clone());
                Ok(CaptureArg::string(
                    target,
                    StringExpr::local_get(local, capture.name),
                ))
            }
            LocalBinding::Primitive(LocalId::Bool(local)) => {
                let target = self.define_bool_local(capture.name.clone());
                Ok(CaptureArg::bool(
                    target,
                    BoolExpr::local_get(local, capture.name),
                ))
            }
            LocalBinding::Primitive(LocalId::Nil(local)) => {
                let target = self.define_nil_local(capture.name.clone());
                Ok(CaptureArg::nil(
                    target,
                    NilExpr::local_get(local, capture.name),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::Int { local, type_ }) => {
                let target = self.define_int_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::int_function(
                    target,
                    IntFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::Float { local, type_ }) => {
                let target = self.define_float_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::float_function(
                    target,
                    FloatFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::String { local, type_ }) => {
                let target = self.define_string_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::string_function(
                    target,
                    StringFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::Bool { local, type_ }) => {
                let target = self.define_bool_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::bool_function(
                    target,
                    BoolFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::Nil { local, type_ }) => {
                let target = self.define_nil_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::nil_function(
                    target,
                    NilFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
            LocalBinding::Function(FunctionLocalBinding::Function { local, type_ }) => {
                let target =
                    self.define_function_function_local(capture.name.clone(), type_.clone());
                Ok(CaptureArg::function_function(
                    target,
                    FunctionFunctionExpr::local_get(local, capture.name, type_),
                ))
            }
        }
    }
}

pub(in crate::planner) struct AnonymousFunctions {
    next_function_index: usize,
    next_anonymous_index: usize,
    runtime_ids: FunctionRuntimeIds,
    functions: Vec<FunctionPlan>,
}

impl AnonymousFunctions {
    pub(in crate::planner) fn new(
        next_function_index: usize,
        runtime_ids: FunctionRuntimeIds,
    ) -> Self {
        Self {
            next_function_index,
            next_anonymous_index: 0,
            runtime_ids,
            functions: Vec::new(),
        }
    }

    pub(in crate::planner) fn into_functions(self) -> Vec<FunctionPlan> {
        self.functions
    }

    fn next_name(&self) -> EcoString {
        format!("<anonymous:{}>", self.next_anonymous_index).into()
    }

    fn allocate(
        &mut self,
        return_type: ValueType,
        params: Vec<FunctionParam>,
        runtime_id: RuntimeFunctionId,
    ) -> (EcoString, FunctionInfo) {
        let name = self.next_name();
        let info = FunctionInfo {
            id: FunctionId::new(self.next_function_index),
            runtime_id,
            return_type,
            params,
        };
        self.next_function_index += 1;
        self.next_anonymous_index += 1;
        (name, info)
    }

    fn allocate_runtime_id(&mut self, return_type: &ValueType) -> RuntimeFunctionId {
        self.runtime_ids.next(return_type)
    }

    fn push(&mut self, function: FunctionPlan) {
        self.functions.push(function);
    }
}

impl Default for AnonymousFunctions {
    fn default() -> Self {
        Self::new(0, FunctionRuntimeIds::default())
    }
}

impl FunctionInfo {
    pub(super) fn arity(&self) -> usize {
        self.params.len()
    }

    pub(super) fn return_type(&self) -> ValueType {
        self.return_type.clone()
    }

    pub(super) fn value(&self) -> FunctionValue {
        FunctionValue::new(self.runtime_id.clone(), self.param_locals())
    }

    pub(super) fn param_locals(&self) -> Vec<ParamLocal> {
        self.params
            .iter()
            .map(|param| param.local.clone())
            .collect()
    }
}

#[derive(Debug, Default)]
pub(in crate::planner) struct FunctionRuntimeIds {
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bool: usize,
    next_nil: usize,
    next_int_function: usize,
    next_float_function: usize,
    next_string_function: usize,
    next_bool_function: usize,
    next_nil_function: usize,
    next_function_function: usize,
}

impl FunctionRuntimeIds {
    pub(in crate::planner) fn next(&mut self, return_type: &ValueType) -> RuntimeFunctionId {
        match return_type {
            ValueType::Int => RuntimeFunctionId::Int(self.next_int_id()),
            ValueType::Float => RuntimeFunctionId::Float(self.next_float_id()),
            ValueType::String => RuntimeFunctionId::String(self.next_string_id()),
            ValueType::Bool => RuntimeFunctionId::Bool(self.next_bool_id()),
            ValueType::Nil => RuntimeFunctionId::Nil(self.next_nil_id()),
            ValueType::Function(return_type) => self.next_function(return_type.as_ref().clone()),
        }
    }

    pub(super) fn next_function(&mut self, return_type: FunctionType) -> RuntimeFunctionId {
        let id = match return_type.return_() {
            ValueType::Int => FunctionFunctionId::Int(self.next_int_function_id()),
            ValueType::Float => FunctionFunctionId::Float(self.next_float_function_id()),
            ValueType::String => FunctionFunctionId::String(self.next_string_function_id()),
            ValueType::Bool => FunctionFunctionId::Bool(self.next_bool_function_id()),
            ValueType::Nil => FunctionFunctionId::Nil(self.next_nil_function_id()),
            ValueType::Function(_) => {
                FunctionFunctionId::Function(self.next_function_function_id())
            }
        };

        RuntimeFunctionId::Function { id, return_type }
    }

    pub(in crate::planner) fn next_int_id(&mut self) -> IntFunctionId {
        let id = IntFunctionId(self.next_int);
        self.next_int += 1;
        id
    }

    pub(in crate::planner) fn next_string_id(&mut self) -> StringFunctionId {
        let id = StringFunctionId(self.next_string);
        self.next_string += 1;
        id
    }

    pub(in crate::planner) fn next_float_id(&mut self) -> FloatFunctionId {
        let id = FloatFunctionId(self.next_float);
        self.next_float += 1;
        id
    }

    pub(in crate::planner) fn next_bool_id(&mut self) -> BoolFunctionId {
        let id = BoolFunctionId(self.next_bool);
        self.next_bool += 1;
        id
    }

    pub(in crate::planner) fn next_nil_id(&mut self) -> NilFunctionId {
        let id = NilFunctionId(self.next_nil);
        self.next_nil += 1;
        id
    }

    pub(in crate::planner) fn next_int_function_id(&mut self) -> IntFunctionFunctionId {
        let id = IntFunctionFunctionId(self.next_int_function);
        self.next_int_function += 1;
        id
    }

    pub(in crate::planner) fn next_string_function_id(&mut self) -> StringFunctionFunctionId {
        let id = StringFunctionFunctionId(self.next_string_function);
        self.next_string_function += 1;
        id
    }

    pub(in crate::planner) fn next_float_function_id(&mut self) -> FloatFunctionFunctionId {
        let id = FloatFunctionFunctionId(self.next_float_function);
        self.next_float_function += 1;
        id
    }

    pub(in crate::planner) fn next_bool_function_id(&mut self) -> BoolFunctionFunctionId {
        let id = BoolFunctionFunctionId(self.next_bool_function);
        self.next_bool_function += 1;
        id
    }

    pub(in crate::planner) fn next_nil_function_id(&mut self) -> NilFunctionFunctionId {
        let id = NilFunctionFunctionId(self.next_nil_function);
        self.next_nil_function += 1;
        id
    }

    pub(in crate::planner) fn next_function_function_id(&mut self) -> FunctionFunctionFunctionId {
        let id = FunctionFunctionFunctionId(self.next_function_function);
        self.next_function_function += 1;
        id
    }
}

impl ValueType {
    pub(super) fn from_gleam(type_: &Type) -> Option<Self> {
        if type_.is_int() {
            Some(Self::Int)
        } else if type_.is_float() {
            Some(Self::Float)
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
    use super::FunctionLocalBinding;
    use super::{AnonymousFunctions, FunctionInfo, FunctionRuntimeIds, PlanContext};
    use crate::plan::{
        CaptureArgKind, FloatFunctionId, FloatFunctionLocalId, FunctionType, FunctionValue,
        IntFunctionId, IntFunctionLocalId, IntLocalId, LocalId, RuntimeFunctionId, ValueType,
    };
    use ecow::EcoString;
    use std::collections::HashMap;

    #[test]
    fn local_scope_restores_names_after_error_without_reusing_ids() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

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
    fn define_function_local_records_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let value = function_value();

        let local = context.define_int_function_local("f".into(), value.type_());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some(FunctionLocalBinding::Int {
                local,
                type_: value.type_(),
            })
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn function_local_shadows_primitive_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let value = function_value();

        context.define_int_local("f".into());
        context.define_int_function_local("f".into(), value.type_());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some(FunctionLocalBinding::Int {
                local: IntFunctionLocalId(0),
                type_: value.type_(),
            })
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn primitive_binding_shadows_function_local() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        context.define_int_function_local("f".into(), function_value().type_());
        let local = context.define_int_local("f".into());

        assert_eq!(context.lookup_function_local(&"f".into()), None);
        assert_eq!(
            context.lookup_local(&"f".into()),
            Some((LocalId::Int(local), ValueType::Int))
        );
    }

    #[test]
    fn define_captures_records_float_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Float], ValueType::Float);

        context.define_float_function_local("f".into(), type_);
        let captures = context.capture_bindings(&[EcoString::from("f")]).unwrap();
        let captures = context.define_captures(captures).unwrap();

        assert!(matches!(
            captures[0].kind(),
            CaptureArgKind::FloatFunction {
                local: FloatFunctionLocalId(1),
                ..
            },
        ));
    }

    #[test]
    fn function_runtime_ids_allocate_by_return_type() {
        let mut ids = FunctionRuntimeIds::default();

        assert_eq!(
            ids.next(&ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(1))
        );
        assert_eq!(
            ids.next(&ValueType::Float),
            RuntimeFunctionId::Float(FloatFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::String),
            RuntimeFunctionId::String(crate::plan::StringFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Bool),
            RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Nil),
            RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0))
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new())
    }
}
