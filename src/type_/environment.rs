use crate::analyse::{AnalyseError, AnalyseErrorType, error};
use crate::ast::{CustomType, Import, Publicity, SrcSpan, TypeAst};
use crate::type_::{
    ImportableModules, ModuleInterface, Type, TypeConstructor, ValueConstructor, bool,
    builtin_types, contains_infer, fn_, infer, named, named_infer, nil, substitute, tuple, var,
};
use ecow::EcoString;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct LocalEnv {
    values: HashMap<EcoString, Arc<Type>>,
}

impl LocalEnv {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, name: EcoString, type_: Arc<Type>) {
        self.values.insert(name, type_);
    }

    pub(crate) fn get(&self, name: &EcoString) -> Option<Arc<Type>> {
        self.values.get(name).cloned()
    }
}

pub(crate) struct Environment<'a> {
    pub(crate) module_name: EcoString,
    pub(crate) importable_modules: &'a ImportableModules,
    pub(crate) imported_modules: HashMap<EcoString, EcoString>,
    pub(crate) local_types: BTreeMap<EcoString, TypeConstructor>,
    pub(crate) local_values: BTreeMap<EcoString, ValueConstructor>,
    pub(crate) values: BTreeMap<EcoString, ValueConstructor>,
    next_infer_id: Cell<u64>,
    infer_substitutions: RefCell<HashMap<u64, Arc<Type>>>,
}

#[derive(Clone, Copy)]
enum TypeVariableScope<'a> {
    Declared(&'a [EcoString]),
}

impl<'a> Environment<'a> {
    pub(crate) fn new(module_name: EcoString, importable_modules: &'a ImportableModules) -> Self {
        let mut values = BTreeMap::new();
        values.insert(
            "True".into(),
            ValueConstructor::record(Publicity::Public, bool()),
        );
        values.insert(
            "False".into(),
            ValueConstructor::record(Publicity::Public, bool()),
        );
        values.insert(
            "Nil".into(),
            ValueConstructor::record(Publicity::Public, nil()),
        );

        Self {
            module_name,
            importable_modules,
            imported_modules: HashMap::new(),
            local_types: BTreeMap::new(),
            local_values: BTreeMap::new(),
            values,
            next_infer_id: Cell::new(0),
            infer_substitutions: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn module_interface(&self) -> ModuleInterface {
        ModuleInterface {
            name: self.module_name.clone(),
            types: self
                .local_types
                .iter()
                .filter(|(_, constructor)| constructor.publicity.is_public())
                .map(|(name, constructor)| {
                    (
                        name.clone(),
                        TypeConstructor {
                            publicity: constructor.publicity,
                            parameters: constructor.parameters.clone(),
                            type_: self.generalise_inferred_type(&constructor.type_),
                        },
                    )
                })
                .collect(),
            values: self
                .local_values
                .iter()
                .filter(|(_, constructor)| constructor.publicity.is_public())
                .map(|(name, constructor)| {
                    (
                        name.clone(),
                        ValueConstructor {
                            publicity: constructor.publicity,
                            variant: constructor.variant,
                            type_: self.generalise_inferred_type(&constructor.type_),
                            parameter_labels: constructor.parameter_labels.clone(),
                        },
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn register_import(&mut self, import: &Import) -> Result<(), AnalyseError> {
        let module = import.module.clone();
        let interface = self.importable_modules.get(&module).ok_or_else(|| {
            error(
                AnalyseErrorType::UnknownModule {
                    module: module.clone(),
                },
                import.location,
            )
        })?;
        validate_module_interface(interface, import.location)?;

        let local_name = import
            .alias
            .as_ref()
            .map(|(_, alias)| alias.clone())
            .unwrap_or_else(|| {
                import
                    .module
                    .rsplit('/')
                    .next()
                    .unwrap_or(&import.module)
                    .into()
            });
        self.imported_modules
            .insert(local_name, import.module.clone());

        Ok(())
    }

    pub(crate) fn register_custom_type_header(&mut self, custom_type: &CustomType<()>) {
        let parameters = custom_type
            .parameters
            .iter()
            .map(|(_, name)| name.clone())
            .collect::<Vec<_>>();
        let arguments = parameters.iter().map(|name| var(name.clone())).collect();
        let type_ = crate::type_::named(
            self.module_name.clone(),
            custom_type.name.1.clone(),
            arguments,
        );

        self.local_types.insert(
            custom_type.name.1.clone(),
            TypeConstructor {
                publicity: custom_type.publicity,
                parameters,
                type_,
            },
        );
    }

    pub(crate) fn insert_local_value(&mut self, name: EcoString, value: ValueConstructor) {
        self.values.insert(name.clone(), value.clone());
        self.local_values.insert(name, value);
    }

    pub(crate) fn fresh_infer_var(&self) -> Arc<Type> {
        let id = self.next_infer_id.get();
        self.next_infer_id.set(id + 1);
        infer(id)
    }

    pub(crate) fn fresh_named_infer_var(&self, name: EcoString) -> Arc<Type> {
        let id = self.next_infer_id.get();
        self.next_infer_id.set(id + 1);
        named_infer(id, name)
    }

    pub(crate) fn resolve_declared_type(
        &self,
        type_ast: &TypeAst,
        parameters: &[EcoString],
    ) -> Result<Arc<Type>, AnalyseError> {
        self.resolve_type_inner(type_ast, TypeVariableScope::Declared(parameters))
    }

    pub(crate) fn resolve_inferred_annotation(
        &self,
        type_ast: &TypeAst,
        type_variables: &mut HashMap<EcoString, Arc<Type>>,
    ) -> Result<Arc<Type>, AnalyseError> {
        match type_ast {
            TypeAst::Constructor {
                location,
                module,
                name,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_inferred_annotation(argument, type_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                self.resolve_type_constructor(module.as_ref(), &name.1, arguments, *location)
            }
            TypeAst::Fn {
                arguments, return_, ..
            } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_inferred_annotation(argument, type_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                let return_ = self.resolve_inferred_annotation(return_, type_variables)?;
                Ok(fn_(arguments, return_))
            }
            TypeAst::Var { name, .. } => Ok(type_variables
                .entry(name.clone())
                .or_insert_with(|| self.fresh_named_infer_var(name.clone()))
                .clone()),
            TypeAst::Tuple { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|element| self.resolve_inferred_annotation(element, type_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(tuple(elements))
            }
            TypeAst::Hole { location, name } => Err(error(
                AnalyseErrorType::UnsupportedTypeHole { name: name.clone() },
                *location,
            )),
        }
    }

    fn resolve_type_inner(
        &self,
        type_ast: &TypeAst,
        scope: TypeVariableScope<'_>,
    ) -> Result<Arc<Type>, AnalyseError> {
        match type_ast {
            TypeAst::Constructor {
                location,
                module,
                name,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_type_inner(argument, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                self.resolve_type_constructor(module.as_ref(), &name.1, arguments, *location)
            }
            TypeAst::Fn {
                arguments, return_, ..
            } => {
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_type_inner(argument, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                let return_ = self.resolve_type_inner(return_, scope)?;
                Ok(fn_(arguments, return_))
            }
            TypeAst::Var { location, name } => match scope {
                TypeVariableScope::Declared(parameters) if parameters.contains(name) => {
                    Ok(var(name.clone()))
                }
                TypeVariableScope::Declared(_) => Err(error(
                    AnalyseErrorType::UnknownType { name: name.clone() },
                    *location,
                )),
            },
            TypeAst::Tuple { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|element| self.resolve_type_inner(element, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(tuple(elements))
            }
            TypeAst::Hole { location, name } => Err(error(
                AnalyseErrorType::UnsupportedTypeHole { name: name.clone() },
                *location,
            )),
        }
    }

    pub(crate) fn resolve_type_constructor(
        &self,
        module: Option<&(EcoString, SrcSpan)>,
        name: &EcoString,
        arguments: Vec<Arc<Type>>,
        location: SrcSpan,
    ) -> Result<Arc<Type>, AnalyseError> {
        let constructor = if let Some((module_alias, alias_location)) = module {
            let module_name = self.imported_modules.get(module_alias).ok_or_else(|| {
                error(
                    AnalyseErrorType::UnknownModule {
                        module: module_alias.clone(),
                    },
                    *alias_location,
                )
            })?;
            self.importable_modules
                .get(module_name)
                .and_then(|interface| {
                    interface
                        .types
                        .get(name)
                        .filter(|constructor| constructor.publicity.is_public())
                        .cloned()
                })
                .ok_or_else(|| {
                    error(
                        AnalyseErrorType::UnknownType { name: name.clone() },
                        location,
                    )
                })?
        } else if let Some(constructor) = self.local_types.get(name) {
            constructor.clone()
        } else {
            builtin_types().remove(name).ok_or_else(|| {
                error(
                    AnalyseErrorType::UnknownType { name: name.clone() },
                    location,
                )
            })?
        };

        if constructor.parameters.len() != arguments.len() {
            return Err(error(
                AnalyseErrorType::WrongArity {
                    expected: constructor.parameters.len(),
                    actual: arguments.len(),
                },
                location,
            ));
        }

        let substitutions = constructor
            .parameters
            .iter()
            .cloned()
            .zip(arguments)
            .collect::<HashMap<_, _>>();
        Ok(self.substitute_type(&constructor.type_, &substitutions))
    }

    pub(crate) fn resolve_value_constructor(
        &self,
        module: Option<&(EcoString, SrcSpan)>,
        name: &EcoString,
        location: SrcSpan,
    ) -> Result<ValueConstructor, AnalyseError> {
        if let Some((module_alias, alias_location)) = module {
            let module_name = self.imported_modules.get(module_alias).ok_or_else(|| {
                error(
                    AnalyseErrorType::UnknownModule {
                        module: module_alias.clone(),
                    },
                    *alias_location,
                )
            })?;
            let interface = self.importable_modules.get(module_name).ok_or_else(|| {
                error(
                    AnalyseErrorType::UnknownModule {
                        module: module_name.clone(),
                    },
                    *alias_location,
                )
            })?;
            return interface
                .values
                .get(name)
                .filter(|value| value.publicity.is_public())
                .map(|value| self.instantiate_value_constructor(value))
                .ok_or_else(|| {
                    error(
                        AnalyseErrorType::UnknownModuleValue {
                            module: module_name.clone(),
                            value: name.clone(),
                        },
                        location,
                    )
                });
        }

        self.values
            .get(name)
            .map(|value| self.instantiate_value_constructor(value))
            .ok_or_else(|| {
                error(
                    AnalyseErrorType::UnknownVariable { name: name.clone() },
                    location,
                )
            })
    }

    pub(crate) fn expect_type(
        &self,
        expected: Arc<Type>,
        actual: Arc<Type>,
        location: SrcSpan,
    ) -> Result<(), AnalyseError> {
        let mut substitutions = HashMap::new();
        if self.unify(&expected, &actual, &mut substitutions) {
            Ok(())
        } else {
            let expected = self.resolve_inferred_type(&expected);
            let actual = self.resolve_inferred_type(&actual);
            Err(error(
                AnalyseErrorType::TypeMismatch { expected, actual },
                location,
            ))
        }
    }

    pub(crate) fn match_fun_type(
        &self,
        type_: Arc<Type>,
        arity: usize,
        location: SrcSpan,
    ) -> Result<(Vec<Arc<Type>>, Arc<Type>), AnalyseError> {
        let type_ = self.resolve_inferred_type(&type_);
        match type_.as_ref() {
            Type::Infer(infer) => {
                let arguments = (0..arity)
                    .map(|_| self.fresh_infer_var())
                    .collect::<Vec<_>>();
                let return_ = self.fresh_infer_var();
                let fn_type = fn_(arguments.clone(), return_.clone());
                if self.bind_infer(infer.id, fn_type.clone()) {
                    Ok((arguments, return_))
                } else {
                    Err(error(
                        AnalyseErrorType::TypeMismatch {
                            expected: fn_type,
                            actual: type_,
                        },
                        location,
                    ))
                }
            }
            Type::Fn { arguments, return_ } => {
                if arguments.len() == arity {
                    Ok((arguments.clone(), return_.clone()))
                } else {
                    Err(error(
                        AnalyseErrorType::WrongArity {
                            expected: arguments.len(),
                            actual: arity,
                        },
                        location,
                    ))
                }
            }
            _ => Err(error(AnalyseErrorType::NotCallable { type_ }, location)),
        }
    }

    pub(crate) fn unify(
        &self,
        expected: &Arc<Type>,
        actual: &Arc<Type>,
        substitutions: &mut HashMap<EcoString, Arc<Type>>,
    ) -> bool {
        let expected = self.resolve_inferred_type(expected);
        let actual = self.resolve_inferred_type(actual);

        match (expected.as_ref(), actual.as_ref()) {
            (Type::Infer(infer), _) => self.bind_infer(infer.id, actual),
            (_, Type::Infer(infer)) => self.bind_infer(infer.id, expected),
            (Type::Var { name }, _) => match substitutions.get(name).cloned() {
                Some(existing) => self.unify(&existing, &actual, substitutions),
                None => {
                    substitutions.insert(name.clone(), actual);
                    true
                }
            },
            (_, Type::Var { name }) => match substitutions.get(name).cloned() {
                Some(existing) => self.unify(&expected, &existing, substitutions),
                None => {
                    substitutions.insert(name.clone(), expected);
                    true
                }
            },
            (
                Type::Named {
                    module,
                    name,
                    arguments,
                },
                Type::Named {
                    module: other_module,
                    name: other_name,
                    arguments: other_arguments,
                },
            ) => {
                module == other_module
                    && name == other_name
                    && arguments.len() == other_arguments.len()
                    && arguments
                        .iter()
                        .zip(other_arguments)
                        .all(|(left, right)| self.unify(left, right, substitutions))
            }
            (
                Type::Fn { arguments, return_ },
                Type::Fn {
                    arguments: other_arguments,
                    return_: other_return,
                },
            ) => {
                arguments.len() == other_arguments.len()
                    && arguments
                        .iter()
                        .zip(other_arguments)
                        .all(|(left, right)| self.unify(left, right, substitutions))
                    && self.unify(return_, other_return, substitutions)
            }
            (
                Type::Tuple { elements },
                Type::Tuple {
                    elements: other_elements,
                },
            ) => {
                elements.len() == other_elements.len()
                    && elements
                        .iter()
                        .zip(other_elements)
                        .all(|(left, right)| self.unify(left, right, substitutions))
            }
            _ => false,
        }
    }

    pub(crate) fn substitute_type(
        &self,
        type_: &Arc<Type>,
        substitutions: &HashMap<EcoString, Arc<Type>>,
    ) -> Arc<Type> {
        let type_ = self.resolve_inferred_type(type_);
        let substituted = substitute(&type_, substitutions);
        self.resolve_inferred_type(&substituted)
    }

    pub(crate) fn instantiate_value_constructor(
        &self,
        value: &ValueConstructor,
    ) -> ValueConstructor {
        ValueConstructor {
            publicity: value.publicity,
            variant: value.variant,
            type_: self.instantiate_type(&value.type_, &mut HashMap::new()),
            parameter_labels: value.parameter_labels.clone(),
        }
    }

    fn instantiate_type(
        &self,
        type_: &Arc<Type>,
        variables: &mut HashMap<EcoString, Arc<Type>>,
    ) -> Arc<Type> {
        match self.resolve_inferred_type(type_).as_ref() {
            Type::Var { name } => variables
                .entry(name.clone())
                .or_insert_with(|| self.fresh_infer_var())
                .clone(),
            Type::Named {
                module,
                name,
                arguments,
            } => named(
                module.clone(),
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| self.instantiate_type(argument, variables))
                    .collect(),
            ),
            Type::Fn { arguments, return_ } => fn_(
                arguments
                    .iter()
                    .map(|argument| self.instantiate_type(argument, variables))
                    .collect(),
                self.instantiate_type(return_, variables),
            ),
            Type::Tuple { elements } => tuple(
                elements
                    .iter()
                    .map(|element| self.instantiate_type(element, variables))
                    .collect(),
            ),
            Type::Infer(_) => type_.clone(),
        }
    }

    pub(crate) fn resolve_inferred_type(&self, type_: &Arc<Type>) -> Arc<Type> {
        match type_.as_ref() {
            Type::Infer(infer) => {
                let replacement = self.infer_substitutions.borrow().get(&infer.id).cloned();
                replacement
                    .map(|type_| self.resolve_inferred_type(&type_))
                    .unwrap_or_else(|| type_.clone())
            }
            Type::Named {
                module,
                name,
                arguments,
            } => named(
                module.clone(),
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| self.resolve_inferred_type(argument))
                    .collect(),
            ),
            Type::Fn { arguments, return_ } => fn_(
                arguments
                    .iter()
                    .map(|argument| self.resolve_inferred_type(argument))
                    .collect(),
                self.resolve_inferred_type(return_),
            ),
            Type::Tuple { elements } => tuple(
                elements
                    .iter()
                    .map(|element| self.resolve_inferred_type(element))
                    .collect(),
            ),
            Type::Var { .. } => type_.clone(),
        }
    }

    pub(crate) fn generalise_inferred_type(&self, type_: &Arc<Type>) -> Arc<Type> {
        self.generalise_inferred_types(std::slice::from_ref(type_))
            .remove(0)
    }

    pub(crate) fn generalise_inferred_types(&self, types: &[Arc<Type>]) -> Vec<Arc<Type>> {
        let mut generalizer = TypeGeneralizer::new(self, types);
        types
            .iter()
            .map(|type_| generalizer.generalise(type_))
            .collect()
    }

    fn generalise_inferred_type_with(
        &self,
        type_: &Arc<Type>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Arc<Type> {
        match self.resolve_inferred_type(type_).as_ref() {
            Type::Infer(infer) => var(generalizer.generated_name(infer.id)),
            Type::Named {
                module,
                name,
                arguments,
            } => named(
                module.clone(),
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| self.generalise_inferred_type_with(argument, generalizer))
                    .collect(),
            ),
            Type::Fn { arguments, return_ } => fn_(
                arguments
                    .iter()
                    .map(|argument| self.generalise_inferred_type_with(argument, generalizer))
                    .collect(),
                self.generalise_inferred_type_with(return_, generalizer),
            ),
            Type::Tuple { elements } => tuple(
                elements
                    .iter()
                    .map(|element| self.generalise_inferred_type_with(element, generalizer))
                    .collect(),
            ),
            Type::Var { .. } => type_.clone(),
        }
    }

    pub(crate) fn has_unresolved_infer(&self, type_: &Arc<Type>) -> bool {
        match self.resolve_inferred_type(type_).as_ref() {
            Type::Infer(_) => true,
            Type::Named { arguments, .. } => arguments
                .iter()
                .any(|type_| self.has_unresolved_infer(type_)),
            Type::Fn { arguments, return_ } => {
                arguments
                    .iter()
                    .any(|type_| self.has_unresolved_infer(type_))
                    || self.has_unresolved_infer(return_)
            }
            Type::Tuple { elements } => elements
                .iter()
                .any(|type_| self.has_unresolved_infer(type_)),
            Type::Var { .. } => false,
        }
    }

    pub(crate) fn find_private_type(&self, type_: &Arc<Type>) -> Option<Arc<Type>> {
        let type_ = self.resolve_inferred_type(type_);
        match type_.as_ref() {
            Type::Named {
                module,
                name,
                arguments,
            } => {
                if module == &self.module_name
                    && self
                        .local_types
                        .get(name)
                        .is_some_and(|constructor| !constructor.publicity.is_public())
                {
                    return Some(type_);
                }
                arguments
                    .iter()
                    .find_map(|argument| self.find_private_type(argument))
            }
            Type::Fn { arguments, return_ } => self.find_private_type(return_).or_else(|| {
                arguments
                    .iter()
                    .find_map(|type_| self.find_private_type(type_))
            }),
            Type::Tuple { elements } => elements
                .iter()
                .find_map(|element| self.find_private_type(element)),
            Type::Infer(_) | Type::Var { .. } => None,
        }
    }

    fn bind_infer(&self, id: u64, type_: Arc<Type>) -> bool {
        if matches!(type_.as_ref(), Type::Infer(infer) if infer.id == id) {
            return true;
        }
        if self.contains_infer_id(&type_, id) {
            return false;
        }

        let existing = self.infer_substitutions.borrow().get(&id).cloned();
        if let Some(existing) = existing {
            let mut substitutions = HashMap::new();
            return self.unify(&existing, &type_, &mut substitutions);
        }

        self.infer_substitutions.borrow_mut().insert(id, type_);
        true
    }

    fn contains_infer_id(&self, type_: &Arc<Type>, id: u64) -> bool {
        match self.resolve_inferred_type(type_).as_ref() {
            Type::Infer(infer) => infer.id == id,
            Type::Named { arguments, .. } => arguments
                .iter()
                .any(|type_| self.contains_infer_id(type_, id)),
            Type::Fn { arguments, return_ } => {
                arguments
                    .iter()
                    .any(|type_| self.contains_infer_id(type_, id))
                    || self.contains_infer_id(return_, id)
            }
            Type::Tuple { elements } => elements
                .iter()
                .any(|type_| self.contains_infer_id(type_, id)),
            Type::Var { .. } => false,
        }
    }
}

pub(crate) struct TypeGeneralizer<'a, 'b> {
    environment: &'a Environment<'b>,
    generated_names: HashMap<u64, EcoString>,
    used_names: HashSet<EcoString>,
    next_name: usize,
}

impl<'a, 'b> TypeGeneralizer<'a, 'b> {
    pub(crate) fn new(environment: &'a Environment<'b>, types: &[Arc<Type>]) -> Self {
        let mut generalizer = Self {
            environment,
            generated_names: HashMap::new(),
            used_names: HashSet::new(),
            next_name: 0,
        };
        for type_ in types {
            generalizer.collect_used_names(type_);
        }
        generalizer
    }

    pub(crate) fn generalise(&mut self, type_: &Arc<Type>) -> Arc<Type> {
        self.environment.generalise_inferred_type_with(type_, self)
    }

    fn generated_name(&mut self, id: u64) -> EcoString {
        if let Some(name) = self.generated_names.get(&id) {
            return name.clone();
        }

        loop {
            let name = generated_type_name(self.next_name);
            self.next_name += 1;
            if self.used_names.insert(name.clone()) {
                self.generated_names.insert(id, name.clone());
                return name;
            }
        }
    }

    fn collect_used_names(&mut self, type_: &Arc<Type>) {
        match self.environment.resolve_inferred_type(type_).as_ref() {
            Type::Infer(_) => {}
            Type::Named { arguments, .. } => {
                for argument in arguments {
                    self.collect_used_names(argument);
                }
            }
            Type::Fn { arguments, return_ } => {
                for argument in arguments {
                    self.collect_used_names(argument);
                }
                self.collect_used_names(return_);
            }
            Type::Tuple { elements } => {
                for element in elements {
                    self.collect_used_names(element);
                }
            }
            Type::Var { name } => {
                self.used_names.insert(name.clone());
            }
        }
    }
}

fn generated_type_name(mut index: usize) -> EcoString {
    let mut chars = Vec::new();
    loop {
        chars.push((b'a' + (index % 26) as u8) as char);
        if index < 26 {
            break;
        }
        index = (index / 26) - 1;
    }
    chars.iter().rev().collect::<String>().into()
}

fn validate_module_interface(
    interface: &ModuleInterface,
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    for constructor in interface.types.values() {
        if contains_infer(&constructor.type_) {
            return Err(error(
                AnalyseErrorType::UnresolvedType {
                    type_: constructor.type_.clone(),
                },
                location,
            ));
        }
        validate_module_type(interface, constructor, location)?;
    }
    for (name, constructor) in &interface.values {
        if contains_infer(&constructor.type_) {
            return Err(error(
                AnalyseErrorType::UnresolvedType {
                    type_: constructor.type_.clone(),
                },
                location,
            ));
        }
        validate_module_value(interface, name, constructor, location)?;
    }
    Ok(())
}

fn validate_module_type(
    interface: &ModuleInterface,
    constructor: &TypeConstructor,
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    let mut parameters = HashSet::new();
    for parameter in &constructor.parameters {
        if !parameters.insert(parameter.clone()) {
            return invalid_module_interface(interface, location);
        }
    }

    let mut used_variables = HashSet::new();
    collect_type_variables(&constructor.type_, &mut used_variables);
    if used_variables
        .iter()
        .any(|variable| !parameters.contains(variable))
        || parameters
            .iter()
            .any(|parameter| !used_variables.contains(parameter))
    {
        return invalid_module_interface(interface, location);
    }

    Ok(())
}

fn validate_module_value(
    interface: &ModuleInterface,
    name: &EcoString,
    constructor: &ValueConstructor,
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    let arity = value_arity(&constructor.type_);
    if !constructor.parameter_labels.is_empty() && constructor.parameter_labels.len() != arity {
        return invalid_module_interface(interface, location);
    }
    if constructor.is_record() {
        if constructor.parameter_labels.len() != arity {
            return invalid_module_interface(interface, location);
        }
        let return_type = value_return_type(&constructor.type_);
        let Type::Named {
            module,
            name: type_name,
            arguments: return_arguments,
        } = return_type.as_ref()
        else {
            return invalid_module_interface(interface, location);
        };
        if module != &interface.name {
            return invalid_module_interface(interface, location);
        }
        let Some(type_constructor) = interface
            .types
            .get(type_name)
            .filter(|type_| type_.publicity.is_public())
        else {
            return invalid_module_interface(interface, location);
        };
        validate_record_constructor_type_variables(
            interface,
            constructor,
            type_constructor,
            type_name,
            return_arguments,
            location,
        )?;
    } else if !matches!(constructor.type_.as_ref(), Type::Fn { .. }) {
        return invalid_module_interface(interface, location);
    }
    if name.is_empty() {
        return invalid_module_interface(interface, location);
    }
    Ok(())
}

fn validate_record_constructor_type_variables(
    interface: &ModuleInterface,
    constructor: &ValueConstructor,
    type_constructor: &TypeConstructor,
    type_name: &EcoString,
    return_arguments: &[Arc<Type>],
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    if return_arguments.len() != type_constructor.parameters.len() {
        return invalid_module_interface(interface, location);
    }
    let Type::Named {
        module: constructor_module,
        name: constructor_name,
        arguments: constructor_arguments,
    } = type_constructor.type_.as_ref()
    else {
        return invalid_module_interface(interface, location);
    };
    if constructor_module != &interface.name
        || constructor_name != type_name
        || constructor_arguments.len() != return_arguments.len()
    {
        return invalid_module_interface(interface, location);
    }
    if !type_arguments_match_parameters(constructor_arguments, &type_constructor.parameters)
        || !type_arguments_match_parameters(return_arguments, &type_constructor.parameters)
    {
        return invalid_module_interface(interface, location);
    }

    let mut return_variables = HashSet::new();
    for argument in return_arguments {
        let Type::Var { name } = argument.as_ref() else {
            return invalid_module_interface(interface, location);
        };
        if !return_variables.insert(name.clone()) {
            return invalid_module_interface(interface, location);
        }
    }
    for argument in constructor_arguments {
        let Type::Var { .. } = argument.as_ref() else {
            return invalid_module_interface(interface, location);
        };
    }

    let mut constructor_variables = HashSet::new();
    collect_type_variables(&constructor.type_, &mut constructor_variables);
    if constructor_variables
        .iter()
        .any(|variable| !return_variables.contains(variable))
    {
        return invalid_module_interface(interface, location);
    }

    Ok(())
}

fn type_arguments_match_parameters(arguments: &[Arc<Type>], parameters: &[EcoString]) -> bool {
    arguments.len() == parameters.len()
        && arguments
            .iter()
            .zip(parameters)
            .all(|(argument, parameter)| match argument.as_ref() {
                Type::Var { name } => name == parameter,
                Type::Named { .. } | Type::Fn { .. } | Type::Tuple { .. } | Type::Infer(_) => false,
            })
}

fn collect_type_variables(type_: &Arc<Type>, variables: &mut HashSet<EcoString>) {
    match type_.as_ref() {
        Type::Var { name } => {
            variables.insert(name.clone());
        }
        Type::Named { arguments, .. } => {
            for argument in arguments {
                collect_type_variables(argument, variables);
            }
        }
        Type::Fn { arguments, return_ } => {
            for argument in arguments {
                collect_type_variables(argument, variables);
            }
            collect_type_variables(return_, variables);
        }
        Type::Tuple { elements } => {
            for element in elements {
                collect_type_variables(element, variables);
            }
        }
        Type::Infer(_) => {}
    }
}

fn value_arity(type_: &Arc<Type>) -> usize {
    match type_.as_ref() {
        Type::Fn { arguments, .. } => arguments.len(),
        Type::Named { .. } | Type::Tuple { .. } | Type::Var { .. } | Type::Infer(_) => 0,
    }
}

fn value_return_type(type_: &Arc<Type>) -> Arc<Type> {
    match type_.as_ref() {
        Type::Fn { return_, .. } => return_.clone(),
        Type::Named { .. } | Type::Tuple { .. } | Type::Var { .. } | Type::Infer(_) => {
            type_.clone()
        }
    }
}

fn invalid_module_interface<T>(
    interface: &ModuleInterface,
    location: SrcSpan,
) -> Result<T, AnalyseError> {
    Err(error(
        AnalyseErrorType::InvalidModuleInterface {
            module: interface.name.clone(),
        },
        location,
    ))
}
