use crate::analyse::{AnalyseError, AnalyseErrorType, error};
use crate::ast::{CallArg, Publicity, SrcSpan};
use ecow::EcoString;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub(crate) mod environment;
mod expression;
mod pattern;
#[cfg(test)]
mod tests;

pub(crate) use environment::{Environment, LocalEnv, TypeGeneralizer};
pub(crate) use expression::ExprTyper;
pub(crate) use pattern::PatternTyper;

/// Caller-supplied module interfaces available to `analyse_module`.
///
/// Geam does not load or compile imported source files in this milestone. The
/// caller provides interfaces directly, and analyse validates their basic
/// compiler invariants before using them.
pub type ImportableModules = HashMap<EcoString, ModuleInterface>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Named {
        module: EcoString,
        name: EcoString,
        arguments: Vec<Arc<Type>>,
    },
    Fn {
        arguments: Vec<Arc<Type>>,
        return_: Arc<Type>,
    },
    Tuple {
        elements: Vec<Arc<Type>>,
    },
    Var {
        name: EcoString,
    },
    #[doc(hidden)]
    Infer(InferType),
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferType {
    pub(crate) id: u64,
    pub(crate) name: Option<EcoString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInterface {
    /// The canonical module name, such as `gleam/io`.
    pub name: EcoString,
    /// Public type constructors exported by the module.
    pub types: BTreeMap<EcoString, TypeConstructor>,
    /// Public value constructors exported by the module.
    pub values: BTreeMap<EcoString, ValueConstructor>,
}

impl ModuleInterface {
    pub fn new(name: impl Into<EcoString>) -> Self {
        Self {
            name: name.into(),
            types: BTreeMap::new(),
            values: BTreeMap::new(),
        }
    }

    pub fn with_builtins(name: impl Into<EcoString>) -> Self {
        let mut interface = Self::new(name);
        insert_builtin_types(&mut interface.types);
        interface
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeConstructor {
    /// Whether this type can be used from other modules.
    pub publicity: Publicity,
    /// Type parameter names in declaration order.
    pub parameters: Vec<EcoString>,
    /// The exported type shape. This may represent a custom type or a type
    /// alias; record constructors validate the custom-type-shaped case.
    pub type_: Arc<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueConstructor {
    /// Whether this value can be used from other modules.
    pub publicity: Publicity,
    /// The kind of value constructor this represents.
    pub variant: ValueConstructorVariant,
    /// The value's type.
    pub type_: Arc<Type>,
    /// Function or record-constructor argument labels in positional order.
    pub parameter_labels: Vec<Option<EcoString>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueConstructorVariant {
    ModuleFn,
    Record,
}

impl ValueConstructor {
    /// Construct an imported or local module function value.
    pub fn module_fn(publicity: Publicity, type_: Arc<Type>) -> Self {
        Self {
            publicity,
            variant: ValueConstructorVariant::ModuleFn,
            type_,
            parameter_labels: Vec::new(),
        }
    }

    /// Construct a record constructor value.
    pub fn record(publicity: Publicity, type_: Arc<Type>) -> Self {
        Self {
            publicity,
            variant: ValueConstructorVariant::Record,
            type_,
            parameter_labels: Vec::new(),
        }
    }

    /// Attach argument labels in the same order as the value's arguments.
    ///
    /// Analyse rejects caller-supplied interfaces whose labels do not match the
    /// value arity or record constructor shape.
    pub fn with_parameter_labels(mut self, labels: Vec<Option<EcoString>>) -> Self {
        self.parameter_labels = labels;
        self
    }

    pub(crate) fn is_record(&self) -> bool {
        matches!(self.variant, ValueConstructorVariant::Record)
    }
}

pub fn int() -> Arc<Type> {
    named_builtin("Int", vec![])
}

pub fn float() -> Arc<Type> {
    named_builtin("Float", vec![])
}

pub fn string() -> Arc<Type> {
    named_builtin("String", vec![])
}

pub fn bool() -> Arc<Type> {
    named_builtin("Bool", vec![])
}

pub fn nil() -> Arc<Type> {
    named_builtin("Nil", vec![])
}

pub fn list(element: Arc<Type>) -> Arc<Type> {
    named_builtin("List", vec![element])
}

pub fn fn_(arguments: Vec<Arc<Type>>, return_: Arc<Type>) -> Arc<Type> {
    Arc::new(Type::Fn { arguments, return_ })
}

pub fn tuple(elements: Vec<Arc<Type>>) -> Arc<Type> {
    Arc::new(Type::Tuple { elements })
}

pub fn var(name: impl Into<EcoString>) -> Arc<Type> {
    Arc::new(Type::Var { name: name.into() })
}

pub(crate) fn infer(id: u64) -> Arc<Type> {
    Arc::new(Type::Infer(InferType { id, name: None }))
}

pub(crate) fn named_infer(id: u64, name: impl Into<EcoString>) -> Arc<Type> {
    Arc::new(Type::Infer(InferType {
        id,
        name: Some(name.into()),
    }))
}

pub fn named(
    module: impl Into<EcoString>,
    name: impl Into<EcoString>,
    arguments: Vec<Arc<Type>>,
) -> Arc<Type> {
    Arc::new(Type::Named {
        module: module.into(),
        name: name.into(),
        arguments,
    })
}

pub fn builtin_types() -> BTreeMap<EcoString, TypeConstructor> {
    let mut types = BTreeMap::new();
    insert_builtin_types(&mut types);
    types
}

fn named_builtin(name: impl Into<EcoString>, arguments: Vec<Arc<Type>>) -> Arc<Type> {
    named("gleam", name, arguments)
}

fn insert_builtin_types(types: &mut BTreeMap<EcoString, TypeConstructor>) {
    for (name, parameters, type_) in [
        ("Int", vec![], int()),
        ("Float", vec![], float()),
        ("String", vec![], string()),
        ("Bool", vec![], bool()),
        ("Nil", vec![], nil()),
        ("List", vec!["a".into()], list(var("a"))),
    ] {
        types.insert(
            name.into(),
            TypeConstructor {
                publicity: Publicity::Public,
                parameters,
                type_,
            },
        );
    }
}

pub(crate) fn substitute(
    type_: &Arc<Type>,
    substitutions: &HashMap<EcoString, Arc<Type>>,
) -> Arc<Type> {
    match type_.as_ref() {
        Type::Var { name } => substitutions
            .get(name)
            .cloned()
            .unwrap_or_else(|| type_.clone()),
        Type::Named {
            module,
            name,
            arguments,
        } => named(
            module.clone(),
            name.clone(),
            arguments
                .iter()
                .map(|argument| substitute(argument, substitutions))
                .collect(),
        ),
        Type::Fn { arguments, return_ } => fn_(
            arguments
                .iter()
                .map(|argument| substitute(argument, substitutions))
                .collect(),
            substitute(return_, substitutions),
        ),
        Type::Tuple { elements } => tuple(
            elements
                .iter()
                .map(|element| substitute(element, substitutions))
                .collect(),
        ),
        Type::Infer(_) => type_.clone(),
    }
}

pub(crate) fn contains_infer(type_: &Arc<Type>) -> bool {
    match type_.as_ref() {
        Type::Infer(_) => true,
        Type::Named { arguments, .. } => arguments.iter().any(contains_infer),
        Type::Fn { arguments, return_ } => {
            arguments.iter().any(contains_infer) || contains_infer(return_)
        }
        Type::Tuple { elements } => elements.iter().any(contains_infer),
        Type::Var { .. } => false,
    }
}

pub(crate) fn reorder_call_args<T>(
    arguments: Vec<CallArg<T>>,
    parameter_labels: &[Option<EcoString>],
    arity: usize,
    location: SrcSpan,
) -> Result<Vec<CallArg<T>>, AnalyseError> {
    if arguments.len() != arity {
        return Err(error(
            AnalyseErrorType::WrongArity {
                expected: arity,
                actual: arguments.len(),
            },
            location,
        ));
    }

    let mut ordered = std::iter::repeat_with(|| None)
        .take(arity)
        .collect::<Vec<Option<CallArg<T>>>>();
    let mut next_positional = 0;
    let mut seen_labelled = false;

    for argument in arguments {
        if let Some((label_location, label)) = &argument.label {
            seen_labelled = true;
            let Some(index) = parameter_labels
                .iter()
                .position(|parameter| parameter.as_ref() == Some(label))
                .filter(|index| *index < arity)
            else {
                return Err(error(
                    AnalyseErrorType::UnknownArgumentLabel {
                        label: label.clone(),
                    },
                    *label_location,
                ));
            };

            if ordered[index].is_some() {
                return Err(error(
                    AnalyseErrorType::DuplicateArgumentLabel {
                        label: label.clone(),
                    },
                    *label_location,
                ));
            }

            ordered[index] = Some(argument);
            continue;
        }
        if seen_labelled {
            return Err(error(
                AnalyseErrorType::UnlabelledArgumentAfterLabelled,
                argument.location,
            ));
        }

        while next_positional < arity && ordered[next_positional].is_some() {
            next_positional += 1;
        }

        if next_positional >= arity {
            return Err(error(
                AnalyseErrorType::WrongArity {
                    expected: arity,
                    actual: arity + 1,
                },
                location,
            ));
        }

        ordered[next_positional] = Some(argument);
        next_positional += 1;
    }

    Ok(ordered
        .into_iter()
        .map(|argument| argument.expect("arity check should fill every argument slot"))
        .collect())
}
