use crate::analyse::{AnalyseError, AnalyseErrorType, error};
use crate::ast::{AssignName, CallArg, Pattern, SrcSpan};
use crate::type_::{
    Environment, LocalEnv, Type, float, int, list, reorder_call_args, string, tuple, var,
};
use ecow::EcoString;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct PatternTyper<'a, 'b> {
    environment: &'a Environment<'b>,
}

impl<'a, 'b> PatternTyper<'a, 'b> {
    pub(crate) fn new(environment: &'a Environment<'b>) -> Self {
        Self { environment }
    }

    pub(crate) fn analyse_pattern(
        &self,
        pattern: Pattern<()>,
        expected_type: Arc<Type>,
        env: &mut LocalEnv,
    ) -> Result<Pattern<Arc<Type>>, AnalyseError> {
        match pattern {
            Pattern::Int {
                location,
                value,
                int_value,
            } => {
                self.environment
                    .expect_type(int(), expected_type, location)?;
                Ok(Pattern::Int {
                    location,
                    value,
                    int_value,
                })
            }
            Pattern::Float { location, value } => {
                self.environment
                    .expect_type(float(), expected_type, location)?;
                Ok(Pattern::Float { location, value })
            }
            Pattern::String { location, value } => {
                self.environment
                    .expect_type(string(), expected_type, location)?;
                Ok(Pattern::String { location, value })
            }
            Pattern::Variable { location, name, .. } => {
                env.insert(name.clone(), expected_type.clone());
                Ok(Pattern::Variable {
                    location,
                    name,
                    type_: expected_type,
                })
            }
            Pattern::Assign {
                name,
                location,
                pattern,
            } => {
                env.insert(name.clone(), expected_type.clone());
                let pattern = self.analyse_pattern(*pattern, expected_type, env)?;
                Ok(Pattern::Assign {
                    name,
                    location,
                    pattern: Box::new(pattern),
                })
            }
            Pattern::Discard { name, location, .. } => Ok(Pattern::Discard {
                name,
                location,
                type_: expected_type,
            }),
            Pattern::List {
                location, elements, ..
            } => {
                let element_type = self.list_element_type(expected_type.clone(), location)?;
                let elements = elements
                    .into_iter()
                    .map(|element| self.analyse_pattern(element, element_type.clone(), env))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Pattern::List {
                    location,
                    elements,
                    type_: list(element_type),
                })
            }
            Pattern::Constructor {
                location,
                name_location,
                name,
                arguments,
                module,
                ..
            } => self.analyse_constructor_pattern(
                location,
                name_location,
                name,
                arguments,
                module,
                expected_type,
                env,
            ),
            Pattern::Tuple { location, elements } => {
                let type_elements =
                    self.tuple_element_types(expected_type, elements.len(), location)?;
                let elements = elements
                    .into_iter()
                    .zip(type_elements)
                    .map(|(pattern, type_)| self.analyse_pattern(pattern, type_, env))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Pattern::Tuple { location, elements })
            }
            Pattern::StringPrefix {
                location,
                left_location,
                left_side_assignment,
                right_location,
                left_side_string,
                right_side_assignment,
            } => {
                self.environment
                    .expect_type(string(), expected_type, location)?;
                if let Some((_, name)) = &left_side_assignment {
                    env.insert(name.clone(), string());
                }
                self.bind_assign_name(&right_side_assignment, string(), env);
                Ok(Pattern::StringPrefix {
                    location,
                    left_location,
                    left_side_assignment,
                    right_location,
                    left_side_string,
                    right_side_assignment,
                })
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn analyse_constructor_pattern(
        &self,
        location: SrcSpan,
        name_location: SrcSpan,
        name: EcoString,
        arguments: Vec<CallArg<Pattern<()>>>,
        module: Option<(EcoString, SrcSpan)>,
        expected_type: Arc<Type>,
        env: &mut LocalEnv,
    ) -> Result<Pattern<Arc<Type>>, AnalyseError> {
        let constructor =
            self.environment
                .resolve_value_constructor(module.as_ref(), &name, name_location)?;
        if !constructor.is_record() {
            return Err(error(
                AnalyseErrorType::NotConstructor { name },
                name_location,
            ));
        }
        let (argument_types, return_type) = match constructor.type_.as_ref() {
            Type::Fn { arguments, return_ } => (arguments.clone(), return_.clone()),
            Type::Named { .. } | Type::Tuple { .. } | Type::Var { .. } | Type::Infer(_) => {
                (Vec::new(), constructor.type_.clone())
            }
        };
        if argument_types.len() != arguments.len() {
            return Err(error(
                AnalyseErrorType::WrongArity {
                    expected: argument_types.len(),
                    actual: arguments.len(),
                },
                location,
            ));
        }
        let arguments = reorder_call_args(
            arguments,
            &constructor.parameter_labels,
            argument_types.len(),
            location,
        )?;
        let mut substitutions = HashMap::new();
        if !self
            .environment
            .unify(&return_type, &expected_type, &mut substitutions)
        {
            return Err(error(
                AnalyseErrorType::TypeMismatch {
                    expected: return_type,
                    actual: expected_type,
                },
                location,
            ));
        }

        let arguments = arguments
            .into_iter()
            .zip(argument_types)
            .map(|(argument, type_)| {
                let type_ = self.environment.substitute_type(&type_, &substitutions);
                let value = self.analyse_pattern(argument.value, type_, env)?;
                Ok(CallArg {
                    location: argument.location,
                    label: argument.label,
                    value,
                })
            })
            .collect::<Result<Vec<_>, AnalyseError>>()?;

        Ok(Pattern::Constructor {
            location,
            name_location,
            name,
            arguments,
            module,
            type_: expected_type,
        })
    }

    fn list_element_type(
        &self,
        expected_type: Arc<Type>,
        location: SrcSpan,
    ) -> Result<Arc<Type>, AnalyseError> {
        match self
            .environment
            .resolve_inferred_type(&expected_type)
            .as_ref()
        {
            Type::Named {
                module,
                name,
                arguments,
            } if module.as_str() == "gleam" && name.as_str() == "List" && arguments.len() == 1 => {
                Ok(arguments[0].clone())
            }
            Type::Infer(_) => {
                let element_type = self.environment.fresh_infer_var();
                self.environment.expect_type(
                    list(element_type.clone()),
                    expected_type,
                    location,
                )?;
                Ok(element_type)
            }
            _ => Err(error(
                AnalyseErrorType::TypeMismatch {
                    expected: list(var("_")),
                    actual: expected_type,
                },
                location,
            )),
        }
    }

    fn tuple_element_types(
        &self,
        expected_type: Arc<Type>,
        arity: usize,
        location: SrcSpan,
    ) -> Result<Vec<Arc<Type>>, AnalyseError> {
        match self
            .environment
            .resolve_inferred_type(&expected_type)
            .as_ref()
        {
            Type::Tuple { elements } => {
                if elements.len() != arity {
                    return Err(error(
                        AnalyseErrorType::WrongArity {
                            expected: elements.len(),
                            actual: arity,
                        },
                        location,
                    ));
                }
                Ok(elements.clone())
            }
            Type::Infer(_) => {
                let elements = (0..arity)
                    .map(|_| self.environment.fresh_infer_var())
                    .collect::<Vec<_>>();
                self.environment
                    .expect_type(tuple(elements.clone()), expected_type, location)?;
                Ok(elements)
            }
            _ => Err(error(
                AnalyseErrorType::TypeMismatch {
                    expected: tuple(vec![]),
                    actual: expected_type,
                },
                location,
            )),
        }
    }

    fn bind_assign_name(&self, name: &AssignName, type_: Arc<Type>, env: &mut LocalEnv) {
        if let AssignName::Variable((_, name)) = name {
            env.insert(name.clone(), type_);
        }
    }
}
