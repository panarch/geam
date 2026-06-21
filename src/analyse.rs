mod dependency;
mod finalize;
mod name;

#[cfg(test)]
use crate::analyse::dependency::function_dependencies;
use crate::analyse::dependency::{
    collect_type_ast_variables, sort_function_groups, sort_type_aliases,
};
use crate::ast::{
    Arg, CustomType, Definition, Function, Import, Publicity, RecordConstructor,
    RecordConstructorArg, SrcSpan, TypeAlias, TypedDefinitions, TypedExpr, TypedModule,
    UntypedExpr, UntypedModule,
};
use crate::type_::{
    Environment, ExprTyper, ImportableModules, LocalEnv, Type, TypeConstructor, ValueConstructor,
    fn_, var,
};
use ecow::EcoString;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;

type TypeVariables = HashMap<EcoString, Arc<Type>>;
type FunctionSignature = (Vec<Arc<Type>>, Arc<Type>, TypeVariables);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{error:?} at {location:?}")]
pub struct AnalyseError {
    pub error: AnalyseErrorType,
    pub location: SrcSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyseErrorType {
    UnknownModule {
        module: EcoString,
    },
    UnknownModuleValue {
        module: EcoString,
        value: EcoString,
    },
    InvalidModuleInterface {
        module: EcoString,
    },
    UnknownType {
        name: EcoString,
    },
    UnknownVariable {
        name: EcoString,
    },
    UnknownArgumentLabel {
        label: EcoString,
    },
    DuplicateArgumentLabel {
        label: EcoString,
    },
    UnlabelledArgumentAfterLabelled,
    UnsupportedTypeHole {
        name: EcoString,
    },
    UnsupportedFieldAccess {
        label: EcoString,
    },
    TypeMismatch {
        expected: Arc<Type>,
        actual: Arc<Type>,
    },
    WrongArity {
        expected: usize,
        actual: usize,
    },
    NotCallable {
        type_: Arc<Type>,
    },
    NotConstructor {
        name: EcoString,
    },
    NotTuple {
        type_: Arc<Type>,
    },
    TupleIndexOutOfBounds {
        index: u64,
        size: usize,
    },
    EmptyFunctionBody,
    EmptyBlock,
    EmptyCase,
    CasePatternArityMismatch {
        expected: usize,
        actual: usize,
    },
    DuplicateName {
        name: EcoString,
        previous_location: SrcSpan,
    },
    DuplicateVarInPattern {
        name: EcoString,
    },
    ExtraVarInAlternativePattern {
        name: EcoString,
    },
    MissingVarInAlternativePattern {
        name: EcoString,
    },
    UnresolvedType {
        type_: Arc<Type>,
    },
    RecursiveTypeAlias {
        name: EcoString,
    },
    UnusedTypeAliasParameter {
        name: EcoString,
    },
    PrivateTypeLeak {
        type_: Arc<Type>,
    },
    InvalidName {
        kind: NameKind,
        name: EcoString,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameKind {
    Type,
    TypeAlias,
    CustomTypeVariant,
    Variable,
    TypeVariable,
    Argument,
    Label,
    Function,
    Discard,
}

pub fn analyse_module(
    module: UntypedModule,
    importable_modules: &ImportableModules,
) -> Result<TypedModule, AnalyseError> {
    ModuleAnalyzer::new(module.name.clone(), importable_modules).analyse_module(module)
}

struct ModuleAnalyzer<'a> {
    environment: Environment<'a>,
    function_type_variables: HashMap<EcoString, TypeVariables>,
}

impl<'a> ModuleAnalyzer<'a> {
    fn new(module_name: EcoString, importable_modules: &'a ImportableModules) -> Self {
        Self {
            environment: Environment::new(module_name, importable_modules),
            function_type_variables: HashMap::new(),
        }
    }

    fn analyse_module(mut self, module: UntypedModule) -> Result<TypedModule, AnalyseError> {
        let mut imports = Vec::new();
        let mut custom_types = Vec::new();
        let mut type_aliases = Vec::new();
        let mut functions = Vec::new();

        for definition in module.definitions {
            match definition {
                Definition::Import(import) => imports.push(import),
                Definition::CustomType(custom_type) => custom_types.push(custom_type),
                Definition::TypeAlias(type_alias) => type_aliases.push(type_alias),
                Definition::Function(function) => functions.push(function),
            }
        }

        self.check_unique_top_level_names(&custom_types, &type_aliases, &functions)?;
        self.check_unique_import_names(&imports)?;
        self.check_name_cases(&imports, &custom_types, &type_aliases, &functions)?;

        let typed_imports = imports
            .into_iter()
            .map(|import| self.analyse_import(import))
            .collect::<Result<Vec<_>, _>>()?;

        for custom_type in &custom_types {
            self.environment.register_custom_type_header(custom_type);
        }

        let mut typed_type_aliases_by_name = HashMap::new();
        for type_alias in sort_type_aliases(&type_aliases)? {
            let name = type_alias.name.1.clone();
            let typed = self.analyse_type_alias(type_alias)?;
            typed_type_aliases_by_name.insert(name, typed);
        }
        let typed_type_aliases = type_aliases
            .into_iter()
            .map(|type_alias| {
                typed_type_aliases_by_name
                    .remove(&type_alias.name.1)
                    .expect("type alias should have been analysed")
            })
            .collect::<Vec<_>>();

        let typed_custom_types = custom_types
            .into_iter()
            .map(|custom_type| self.analyse_custom_type(custom_type))
            .collect::<Result<Vec<_>, _>>()?;

        for function in &functions {
            let (value, type_variables) = self.function_value_constructor(function)?;
            if let Some((_, name)) = &function.name {
                self.function_type_variables
                    .insert(name.clone(), type_variables);
                self.environment.insert_local_value(name.clone(), value);
            }
        }

        let mut typed_functions_by_index = HashMap::new();
        for group in sort_function_groups(&functions) {
            let mut typed_group = Vec::with_capacity(group.len());
            for index in group {
                let typed = self.analyse_function(functions[index].clone())?;
                typed_group.push((index, typed));
            }
            for (index, typed) in typed_group {
                let typed = self.finalize_function(typed)?;
                self.register_function_signature(&typed);
                typed_functions_by_index.insert(index, typed);
            }
        }
        let typed_functions = (0..functions.len())
            .map(|index| {
                typed_functions_by_index
                    .remove(&index)
                    .expect("function should have been analysed")
            })
            .collect::<Vec<_>>();

        let typed_custom_types = typed_custom_types
            .into_iter()
            .map(|custom_type| self.finalize_custom_type(custom_type))
            .collect::<Result<Vec<_>, _>>()?;
        let typed_type_aliases = typed_type_aliases
            .into_iter()
            .map(|type_alias| self.finalize_type_alias(type_alias))
            .collect::<Result<Vec<_>, _>>()?;
        self.check_for_type_leaks(&typed_custom_types, &typed_functions)?;
        Ok(TypedModule {
            name: module.name,
            path: module.path,
            documentation: module.documentation,
            type_info: self.environment.module_interface(),
            definitions: TypedDefinitions {
                imports: typed_imports,
                custom_types: typed_custom_types,
                type_aliases: typed_type_aliases,
                functions: typed_functions,
            },
        })
    }

    fn analyse_import(&mut self, import: Import) -> Result<Import, AnalyseError> {
        self.environment.register_import(&import)?;
        Ok(import)
    }

    fn analyse_type_alias(
        &mut self,
        alias: TypeAlias<()>,
    ) -> Result<TypeAlias<Arc<Type>>, AnalyseError> {
        let parameters = alias
            .parameters
            .iter()
            .map(|(_, name)| name.clone())
            .collect::<Vec<_>>();
        let type_ = self
            .environment
            .resolve_declared_type(&alias.alias, &parameters)?;
        self.check_type_alias_parameters_used(&alias)?;

        self.environment.local_types.insert(
            alias.name.1.clone(),
            TypeConstructor {
                publicity: alias.publicity,
                parameters,
                type_: type_.clone(),
            },
        );

        Ok(TypeAlias {
            location: alias.location,
            publicity: alias.publicity,
            name: alias.name,
            parameters: alias.parameters,
            alias: alias.alias,
            type_,
        })
    }

    fn check_type_alias_parameters_used(&self, alias: &TypeAlias<()>) -> Result<(), AnalyseError> {
        let mut used = HashSet::new();
        collect_type_ast_variables(&alias.alias, &mut used);
        for (location, parameter) in &alias.parameters {
            if !used.contains(parameter) {
                return Err(error(
                    AnalyseErrorType::UnusedTypeAliasParameter {
                        name: parameter.clone(),
                    },
                    *location,
                ));
            }
        }
        Ok(())
    }

    fn analyse_custom_type(
        &mut self,
        custom_type: CustomType<()>,
    ) -> Result<CustomType<Arc<Type>>, AnalyseError> {
        let parameters = custom_type
            .parameters
            .iter()
            .map(|(_, name)| name.clone())
            .collect::<Vec<_>>();
        let type_ = self.environment.resolve_type_constructor(
            None,
            &custom_type.name.1,
            custom_type
                .parameters
                .iter()
                .map(|(_, name)| var(name.clone()))
                .collect(),
            custom_type.name.0,
        )?;

        let constructors = custom_type
            .constructors
            .into_iter()
            .map(|constructor| {
                self.analyse_record_constructor(
                    constructor,
                    custom_type.publicity,
                    &parameters,
                    type_.clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CustomType {
            location: custom_type.location,
            publicity: custom_type.publicity,
            name: custom_type.name,
            parameters: custom_type.parameters,
            constructors,
            type_,
        })
    }

    fn analyse_record_constructor(
        &mut self,
        constructor: RecordConstructor<()>,
        publicity: Publicity,
        parameters: &[EcoString],
        return_type: Arc<Type>,
    ) -> Result<RecordConstructor<Arc<Type>>, AnalyseError> {
        let arguments = constructor
            .arguments
            .into_iter()
            .map(|argument| self.analyse_record_constructor_arg(argument, parameters))
            .collect::<Result<Vec<_>, _>>()?;
        let argument_types = arguments
            .iter()
            .map(|argument| argument.type_.clone())
            .collect::<Vec<_>>();
        let parameter_labels = arguments
            .iter()
            .map(|argument| argument.label.as_ref().map(|(_, label)| label.clone()))
            .collect::<Vec<_>>();
        let constructor_type = if argument_types.is_empty() {
            return_type
        } else {
            fn_(argument_types, return_type)
        };

        self.environment.insert_local_value(
            constructor.name.1.clone(),
            ValueConstructor::record(publicity, constructor_type)
                .with_parameter_labels(parameter_labels),
        );

        Ok(RecordConstructor {
            location: constructor.location,
            name: constructor.name,
            arguments,
        })
    }

    fn analyse_record_constructor_arg(
        &self,
        argument: RecordConstructorArg<()>,
        parameters: &[EcoString],
    ) -> Result<RecordConstructorArg<Arc<Type>>, AnalyseError> {
        let type_ = self
            .environment
            .resolve_declared_type(&argument.annotation, parameters)?;
        Ok(RecordConstructorArg {
            location: argument.location,
            label: argument.label,
            annotation: argument.annotation,
            type_,
        })
    }

    fn function_value_constructor(
        &mut self,
        function: &Function<(), UntypedExpr>,
    ) -> Result<(ValueConstructor, TypeVariables), AnalyseError> {
        let mut type_variables = HashMap::new();
        let argument_types = function
            .arguments
            .iter()
            .map(|argument| self.resolve_arg_type(argument, &mut type_variables))
            .collect::<Result<Vec<_>, _>>()?;
        let return_type = function
            .return_annotation
            .as_ref()
            .map(|annotation| {
                self.environment
                    .resolve_inferred_annotation(annotation, &mut type_variables)
            })
            .transpose()?
            .unwrap_or_else(|| self.environment.fresh_infer_var());

        Ok((
            ValueConstructor::module_fn(function.publicity, fn_(argument_types, return_type)),
            type_variables,
        ))
    }

    fn analyse_function(
        &mut self,
        function: Function<(), UntypedExpr>,
    ) -> Result<Function<Arc<Type>, TypedExpr>, AnalyseError> {
        let (argument_types, expected_return_type, type_variables) =
            self.function_signature(&function)?;
        let arguments = function
            .arguments
            .into_iter()
            .zip(argument_types)
            .map(|(argument, type_)| Arg {
                location: argument.location,
                name: argument.name,
                annotation: argument.annotation,
                type_,
            })
            .collect::<Vec<_>>();
        let mut local_env = LocalEnv::new();
        for argument in &arguments {
            local_env.insert(argument.name.1.clone(), argument.type_.clone());
        }

        let expr_typer = ExprTyper::new(&self.environment, type_variables);
        let expected_body_return = function
            .return_annotation
            .as_ref()
            .map(|_| expected_return_type.clone());
        let (body, inferred_return) = expr_typer.analyse_function_body(
            function.body,
            &mut local_env,
            expected_body_return,
        )?;
        self.environment.expect_type(
            expected_return_type.clone(),
            inferred_return,
            function.location,
        )?;
        let return_type = self
            .environment
            .resolve_inferred_type(&expected_return_type);
        Ok(Function {
            location: function.location,
            body_start: function.body_start,
            end_position: function.end_position,
            name: function.name,
            arguments,
            body,
            publicity: function.publicity,
            return_annotation: function.return_annotation,
            return_type,
        })
    }

    fn register_function_signature(&mut self, function: &Function<Arc<Type>, TypedExpr>) {
        let Some((_, name)) = &function.name else {
            return;
        };
        let function_type = fn_(
            function
                .arguments
                .iter()
                .map(|argument| argument.type_.clone())
                .collect(),
            function.return_type.clone(),
        );
        self.environment.insert_local_value(
            name.clone(),
            ValueConstructor::module_fn(function.publicity, function_type),
        );
    }

    fn function_signature(
        &self,
        function: &Function<(), UntypedExpr>,
    ) -> Result<FunctionSignature, AnalyseError> {
        if let Some((location, name)) = &function.name {
            let value = self
                .environment
                .resolve_value_constructor(None, name, *location)?;
            let Type::Fn { arguments, return_ } = value.type_.as_ref() else {
                return Err(error(
                    AnalyseErrorType::NotCallable { type_: value.type_ },
                    *location,
                ));
            };
            let type_variables = self
                .function_type_variables
                .get(name)
                .cloned()
                .unwrap_or_default();
            return Ok((arguments.clone(), return_.clone(), type_variables));
        }

        let mut type_variables = HashMap::new();
        let argument_types = function
            .arguments
            .iter()
            .map(|argument| self.resolve_arg_type(argument, &mut type_variables))
            .collect::<Result<Vec<_>, _>>()?;
        let return_type = function
            .return_annotation
            .as_ref()
            .map(|annotation| {
                self.environment
                    .resolve_inferred_annotation(annotation, &mut type_variables)
            })
            .transpose()?
            .unwrap_or_else(|| self.environment.fresh_infer_var());
        Ok((argument_types, return_type, type_variables))
    }

    fn resolve_arg_type(
        &self,
        argument: &Arg<()>,
        type_variables: &mut HashMap<EcoString, Arc<Type>>,
    ) -> Result<Arc<Type>, AnalyseError> {
        let Some(annotation) = &argument.annotation else {
            return Ok(self.environment.fresh_infer_var());
        };
        self.environment
            .resolve_inferred_annotation(annotation, type_variables)
    }
}

pub(crate) fn error(error: AnalyseErrorType, location: SrcSpan) -> AnalyseError {
    AnalyseError { error, location }
}

#[cfg(test)]
mod tests;
