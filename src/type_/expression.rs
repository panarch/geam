use crate::analyse::{AnalyseError, AnalyseErrorType, error};
use crate::ast::{
    AssignName, Assignment, BinOp, CallArg, Clause, ClauseGuard, HasLocation, Pattern, SrcSpan,
    Statement, TypedExpr, TypedStatement, UntypedExpr, UntypedStatement,
};
use crate::type_::{
    Environment, LocalEnv, PatternTyper, Type, bool, float, int, list, nil, reorder_call_args,
    string, tuple, var,
};
use ecow::EcoString;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

type AnalysedStatement = (TypedStatement, Arc<Type>);
type AnalysedBody = (Vec<TypedStatement>, Arc<Type>);

enum CallArgInput {
    Untyped(UntypedExpr),
    Typed(TypedExpr),
}

pub(crate) struct ExprTyper<'a, 'b> {
    environment: &'a Environment<'b>,
    type_variables: RefCell<HashMap<EcoString, Arc<Type>>>,
}

impl<'a, 'b> ExprTyper<'a, 'b> {
    pub(crate) fn new(
        environment: &'a Environment<'b>,
        type_variables: HashMap<EcoString, Arc<Type>>,
    ) -> Self {
        Self {
            environment,
            type_variables: RefCell::new(type_variables),
        }
    }

    pub(crate) fn analyse_function_body(
        &self,
        statements: Vec<UntypedStatement>,
        env: &mut LocalEnv,
        expected_return: Option<Arc<Type>>,
    ) -> Result<AnalysedBody, AnalyseError> {
        if statements.is_empty() {
            return Err(error(
                AnalyseErrorType::EmptyFunctionBody,
                SrcSpan::default(),
            ));
        }

        let mut typed = Vec::with_capacity(statements.len());
        let mut last_type = nil();
        let last_index = statements.len() - 1;
        for (index, statement) in statements.into_iter().enumerate() {
            let expected = (index == last_index)
                .then(|| expected_return.clone())
                .flatten();
            let (statement, type_) = self.analyse_statement(statement, env, expected)?;
            typed.push(statement);
            last_type = type_;
        }
        Ok((typed, last_type))
    }

    fn analyse_statement(
        &self,
        statement: UntypedStatement,
        env: &mut LocalEnv,
        expected: Option<Arc<Type>>,
    ) -> Result<AnalysedStatement, AnalyseError> {
        match statement {
            Statement::Expression(expression) => {
                let expression =
                    self.analyse_expression_with_expected(expression, env, expected)?;
                let type_ = expression.type_();
                Ok((Statement::Expression(expression), type_))
            }
            Statement::Assignment(assignment) => {
                let Assignment {
                    location,
                    pattern,
                    annotation,
                    value,
                } = *assignment;
                collect_pattern_bindings(std::slice::from_ref(&pattern))?;
                let expected = annotation
                    .as_ref()
                    .map(|annotation| self.resolve_annotation(annotation))
                    .transpose()?;
                let value = self.analyse_expression_with_expected(value, env, expected.clone())?;
                let expected = expected.unwrap_or_else(|| value.type_());
                self.environment
                    .expect_type(expected.clone(), value.type_(), value.location())?;
                let pattern =
                    PatternTyper::new(self.environment).analyse_pattern(pattern, expected, env)?;
                let typed = Statement::Assignment(Box::new(Assignment {
                    location,
                    pattern,
                    annotation,
                    value,
                }));
                Ok((typed, nil()))
            }
        }
    }

    fn resolve_annotation(
        &self,
        annotation: &crate::ast::TypeAst,
    ) -> Result<Arc<Type>, AnalyseError> {
        self.environment
            .resolve_inferred_annotation(annotation, &mut self.type_variables.borrow_mut())
    }

    fn analyse_expression(
        &self,
        expression: UntypedExpr,
        env: &mut LocalEnv,
    ) -> Result<TypedExpr, AnalyseError> {
        self.analyse_expression_with_expected(expression, env, None)
    }

    fn analyse_expression_with_expected(
        &self,
        expression: UntypedExpr,
        env: &mut LocalEnv,
        expected: Option<Arc<Type>>,
    ) -> Result<TypedExpr, AnalyseError> {
        match expression {
            UntypedExpr::Int {
                location,
                value,
                int_value,
            } => Ok(TypedExpr::Int {
                location,
                type_: int(),
                value,
                int_value,
            }),
            UntypedExpr::Float { location, value } => Ok(TypedExpr::Float {
                location,
                type_: float(),
                value,
            }),
            UntypedExpr::String { location, value } => Ok(TypedExpr::String {
                location,
                type_: string(),
                value,
            }),
            UntypedExpr::Block {
                location,
                statements,
            } => {
                if statements.is_empty() {
                    return Err(error(AnalyseErrorType::EmptyBlock, location));
                }
                let mut block_env = env.clone();
                let mut typed = Vec::with_capacity(statements.len());
                let mut last_type = nil();
                let last_index = statements.len() - 1;
                for (index, statement) in statements.into_iter().enumerate() {
                    let statement_expected =
                        (index == last_index).then(|| expected.clone()).flatten();
                    let (statement, type_) =
                        self.analyse_statement(statement, &mut block_env, statement_expected)?;
                    typed.push(statement);
                    last_type = type_;
                }
                Ok(TypedExpr::Block {
                    location,
                    type_: last_type,
                    statements: typed,
                })
            }
            UntypedExpr::Var { location, name } => self.analyse_var(location, name, env),
            UntypedExpr::List { location, elements } => {
                let expected_element_type = expected
                    .as_ref()
                    .map(|expected| self.expected_list_element_type(expected.clone(), location))
                    .transpose()?;
                if elements.is_empty() {
                    let element_type =
                        expected_element_type.unwrap_or_else(|| self.environment.fresh_infer_var());
                    return Ok(TypedExpr::List {
                        location,
                        type_: list(element_type),
                        elements: Vec::new(),
                    });
                }
                let mut typed = Vec::with_capacity(elements.len());
                let mut elements = elements.into_iter();
                let first = self.analyse_expression_with_expected(
                    elements.next().expect("checked non-empty"),
                    env,
                    expected_element_type.clone(),
                )?;
                let element_type = first.type_();
                typed.push(first);

                for element in elements {
                    let element = self.analyse_expression_with_expected(
                        element,
                        env,
                        Some(element_type.clone()),
                    )?;
                    self.environment.expect_type(
                        element_type.clone(),
                        element.type_(),
                        element.location(),
                    )?;
                    typed.push(element);
                }

                Ok(TypedExpr::List {
                    location,
                    type_: list(element_type),
                    elements: typed,
                })
            }
            UntypedExpr::Call {
                location,
                fun,
                arguments,
                open_parenthesis,
            } => {
                let fun = self.analyse_expression(*fun, env)?;
                let (arguments, type_) = self.analyse_call(&fun, arguments, env, location)?;
                Ok(TypedExpr::Call {
                    location,
                    type_,
                    fun: Box::new(fun),
                    arguments,
                    open_parenthesis,
                })
            }
            UntypedExpr::BinOp {
                location,
                operator,
                operator_start,
                left,
                right,
            } => {
                let left = self.analyse_expression(*left, env)?;
                let right = self.analyse_expression(*right, env)?;
                let type_ = self.analyse_bin_op(operator, &left, &right, location)?;
                Ok(TypedExpr::BinOp {
                    location,
                    type_,
                    operator,
                    operator_start,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            UntypedExpr::PipeLine { expressions } => {
                self.analyse_pipeline(expressions, env, expected)
            }
            UntypedExpr::Case {
                location,
                subjects,
                clauses,
            } => self.analyse_case(location, subjects, clauses, env, expected),
            UntypedExpr::FieldAccess {
                location,
                label_location,
                label,
                container,
            } => {
                if let UntypedExpr::Var {
                    name: module_alias, ..
                } = container.as_ref()
                    && let Some(module_name) = self.environment.imported_modules.get(module_alias)
                {
                    let module = (module_alias.clone(), label_location);
                    let value = self.environment.resolve_value_constructor(
                        Some(&module),
                        &label,
                        label_location,
                    )?;
                    return Ok(TypedExpr::ModuleSelect {
                        location,
                        type_: value.type_.clone(),
                        module_name: module_name.clone(),
                        module_alias: module_alias.clone(),
                        label,
                    });
                }

                Err(error(
                    AnalyseErrorType::UnsupportedFieldAccess { label },
                    location,
                ))
            }
            UntypedExpr::Tuple { location, elements } => {
                let expected_elements =
                    self.expected_tuple_element_types(expected, elements.len(), location)?;
                let elements = elements
                    .into_iter()
                    .enumerate()
                    .map(|(index, element)| {
                        let expected = expected_elements
                            .as_ref()
                            .and_then(|types| types.get(index).cloned());
                        self.analyse_expression_with_expected(element, env, expected)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let type_ = tuple(elements.iter().map(TypedExpr::type_).collect());
                Ok(TypedExpr::Tuple {
                    location,
                    type_,
                    elements,
                })
            }
            UntypedExpr::TupleIndex {
                location,
                index,
                tuple: tuple_expr,
            } => {
                let tuple_expr = self.analyse_expression(*tuple_expr, env)?;
                let tuple_type = self.environment.resolve_inferred_type(&tuple_expr.type_());
                let Type::Tuple { elements } = tuple_type.as_ref() else {
                    return Err(error(
                        AnalyseErrorType::NotTuple { type_: tuple_type },
                        tuple_expr.location(),
                    ));
                };
                let type_ = elements.get(index as usize).cloned().ok_or_else(|| {
                    error(
                        AnalyseErrorType::TupleIndexOutOfBounds {
                            index,
                            size: elements.len(),
                        },
                        location,
                    )
                })?;
                Ok(TypedExpr::TupleIndex {
                    location,
                    type_,
                    index,
                    tuple: Box::new(tuple_expr),
                })
            }
            UntypedExpr::NegateBool { location, value } => {
                let value = self.analyse_expression(*value, env)?;
                self.environment
                    .expect_type(bool(), value.type_(), value.location())?;
                Ok(TypedExpr::NegateBool {
                    location,
                    type_: bool(),
                    value: Box::new(value),
                })
            }
            UntypedExpr::NegateInt { location, value } => {
                let value = self.analyse_expression(*value, env)?;
                self.environment
                    .expect_type(int(), value.type_(), value.location())?;
                Ok(TypedExpr::NegateInt {
                    location,
                    type_: int(),
                    value: Box::new(value),
                })
            }
        }
    }

    fn analyse_var(
        &self,
        location: SrcSpan,
        name: EcoString,
        env: &LocalEnv,
    ) -> Result<TypedExpr, AnalyseError> {
        if let Some(type_) = env.get(&name) {
            return Ok(TypedExpr::Var {
                location,
                type_,
                name,
            });
        }
        if self.environment.values.contains_key(&name) {
            let value = self
                .environment
                .resolve_value_constructor(None, &name, location)?;
            return Ok(TypedExpr::Var {
                location,
                type_: value.type_.clone(),
                name,
            });
        }
        Err(error(AnalyseErrorType::UnknownVariable { name }, location))
    }

    fn expected_list_element_type(
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

    fn expected_tuple_element_types(
        &self,
        expected_type: Option<Arc<Type>>,
        arity: usize,
        location: SrcSpan,
    ) -> Result<Option<Vec<Arc<Type>>>, AnalyseError> {
        let Some(expected_type) = expected_type else {
            return Ok(None);
        };
        match self
            .environment
            .resolve_inferred_type(&expected_type)
            .as_ref()
        {
            Type::Tuple { elements } if elements.len() == arity => Ok(Some(elements.clone())),
            Type::Infer(_) => {
                let elements = (0..arity)
                    .map(|_| self.environment.fresh_infer_var())
                    .collect::<Vec<_>>();
                self.environment
                    .expect_type(tuple(elements.clone()), expected_type, location)?;
                Ok(Some(elements))
            }
            _ => Ok(None),
        }
    }

    fn analyse_bin_op(
        &self,
        operator: BinOp,
        left: &TypedExpr,
        right: &TypedExpr,
        _location: SrcSpan,
    ) -> Result<Arc<Type>, AnalyseError> {
        let (argument_type, return_type) = match operator {
            BinOp::And | BinOp::Or => (bool(), bool()),
            BinOp::Eq | BinOp::NotEq => {
                self.environment
                    .expect_type(left.type_(), right.type_(), right.location())?;
                return Ok(bool());
            }
            BinOp::LtInt
            | BinOp::LtEqInt
            | BinOp::GtEqInt
            | BinOp::GtInt
            | BinOp::AddInt
            | BinOp::SubInt
            | BinOp::MultInt
            | BinOp::DivInt
            | BinOp::RemainderInt => (int(), int()),
            BinOp::LtFloat
            | BinOp::LtEqFloat
            | BinOp::GtEqFloat
            | BinOp::GtFloat
            | BinOp::AddFloat
            | BinOp::SubFloat
            | BinOp::MultFloat
            | BinOp::DivFloat => (float(), float()),
            BinOp::Concatenate => (string(), string()),
        };
        self.environment
            .expect_type(argument_type.clone(), left.type_(), left.location())?;
        self.environment
            .expect_type(argument_type, right.type_(), right.location())?;

        match operator {
            BinOp::LtInt
            | BinOp::LtEqInt
            | BinOp::LtFloat
            | BinOp::LtEqFloat
            | BinOp::GtEqInt
            | BinOp::GtInt
            | BinOp::GtEqFloat
            | BinOp::GtFloat => Ok(bool()),
            BinOp::And
            | BinOp::Or
            | BinOp::AddInt
            | BinOp::AddFloat
            | BinOp::SubInt
            | BinOp::SubFloat
            | BinOp::MultInt
            | BinOp::MultFloat
            | BinOp::DivInt
            | BinOp::DivFloat
            | BinOp::RemainderInt
            | BinOp::Concatenate => Ok(return_type),
            BinOp::Eq | BinOp::NotEq => Ok(bool()),
        }
    }

    fn analyse_pipeline(
        &self,
        expressions: Vec<UntypedExpr>,
        env: &mut LocalEnv,
        expected: Option<Arc<Type>>,
    ) -> Result<TypedExpr, AnalyseError> {
        let Some((first, rest)) = expressions.split_first() else {
            return Err(error(AnalyseErrorType::EmptyBlock, SrcSpan::default()));
        };

        let mut current = self.analyse_expression(first.clone(), env)?;
        for expression in rest {
            current = self.analyse_pipeline_step(current, expression.clone(), env)?;
        }

        if let Some(expected) = expected {
            self.environment
                .expect_type(expected, current.type_(), current.location())?;
        }

        Ok(current)
    }

    fn analyse_pipeline_step(
        &self,
        previous: TypedExpr,
        expression: UntypedExpr,
        env: &mut LocalEnv,
    ) -> Result<TypedExpr, AnalyseError> {
        let location = previous.location().merge(&expression.location());
        match expression {
            UntypedExpr::Call {
                location: call_location,
                fun,
                arguments,
                open_parenthesis,
            } => {
                let fun = self.analyse_expression(*fun, env)?;
                if self.pipeline_step_calls_returned_function(&fun, arguments.len()) {
                    let (arguments, type_) =
                        self.analyse_call(&fun, arguments, env, call_location)?;
                    let call = TypedExpr::Call {
                        location: call_location,
                        type_,
                        fun: Box::new(fun),
                        arguments,
                        open_parenthesis,
                    };
                    return self.apply_pipeline_value_to_function(previous, call, env, location);
                }

                self.analyse_pipeline_insert_call(
                    previous,
                    fun,
                    arguments,
                    env,
                    location,
                    open_parenthesis,
                )
            }
            expression => {
                let fun = self.analyse_expression(expression, env)?;
                self.apply_pipeline_value_to_function(previous, fun, env, location)
            }
        }
    }

    fn pipeline_step_calls_returned_function(
        &self,
        fun: &TypedExpr,
        explicit_argument_count: usize,
    ) -> bool {
        matches!(
            self.environment.resolve_inferred_type(&fun.type_()).as_ref(),
            Type::Fn { arguments, .. } if arguments.len() == explicit_argument_count
        )
    }

    fn analyse_pipeline_insert_call(
        &self,
        previous: TypedExpr,
        fun: TypedExpr,
        arguments: Vec<CallArg<UntypedExpr>>,
        env: &mut LocalEnv,
        location: SrcSpan,
        open_parenthesis: u32,
    ) -> Result<TypedExpr, AnalyseError> {
        let mut arguments = arguments
            .into_iter()
            .map(|argument| CallArg {
                location: argument.location,
                label: argument.label,
                value: CallArgInput::Untyped(argument.value),
            })
            .collect::<Vec<_>>();
        arguments.insert(
            0,
            CallArg {
                location: previous.location(),
                label: None,
                value: CallArgInput::Typed(previous),
            },
        );
        let parameter_labels = self.parameter_labels_for_fun(&fun, env);
        let (arguments, type_) =
            self.analyse_call_inputs(&fun, arguments, &parameter_labels, env, location)?;

        Ok(TypedExpr::Call {
            location,
            type_,
            fun: Box::new(fun),
            arguments,
            open_parenthesis,
        })
    }

    fn apply_pipeline_value_to_function(
        &self,
        previous: TypedExpr,
        fun: TypedExpr,
        env: &mut LocalEnv,
        location: SrcSpan,
    ) -> Result<TypedExpr, AnalyseError> {
        let open_parenthesis = fun.location().end;
        let arguments = vec![CallArg {
            location: previous.location(),
            label: None,
            value: CallArgInput::Typed(previous),
        }];
        let parameter_labels = self.parameter_labels_for_fun(&fun, env);
        let (arguments, type_) =
            self.analyse_call_inputs(&fun, arguments, &parameter_labels, env, location)?;

        Ok(TypedExpr::Call {
            location,
            type_,
            fun: Box::new(fun),
            arguments,
            open_parenthesis,
        })
    }

    fn analyse_case(
        &self,
        location: SrcSpan,
        subjects: Vec<UntypedExpr>,
        clauses: Vec<Clause<UntypedExpr, ()>>,
        env: &mut LocalEnv,
        expected: Option<Arc<Type>>,
    ) -> Result<TypedExpr, AnalyseError> {
        if clauses.is_empty() {
            return Err(error(AnalyseErrorType::EmptyCase, location));
        }

        let subjects = subjects
            .into_iter()
            .map(|subject| self.analyse_expression(subject, env))
            .collect::<Result<Vec<_>, _>>()?;
        let subject_types = subjects.iter().map(TypedExpr::type_).collect::<Vec<_>>();
        let result_type = expected.unwrap_or_else(|| self.environment.fresh_infer_var());
        let mut typed_clauses = Vec::with_capacity(clauses.len());

        for clause in clauses {
            let typed_clause =
                self.analyse_clause(clause, &subject_types, env, result_type.clone())?;
            self.environment.expect_type(
                result_type.clone(),
                typed_clause.then.type_(),
                typed_clause.then.location(),
            )?;
            typed_clauses.push(typed_clause);
        }

        Ok(TypedExpr::Case {
            location,
            type_: result_type,
            subjects,
            clauses: typed_clauses,
        })
    }

    fn analyse_clause(
        &self,
        clause: Clause<UntypedExpr, ()>,
        subject_types: &[Arc<Type>],
        env: &LocalEnv,
        expected_result: Arc<Type>,
    ) -> Result<Clause<TypedExpr, Arc<Type>>, AnalyseError> {
        if clause.pattern.len() != subject_types.len() {
            return Err(error(
                AnalyseErrorType::CasePatternArityMismatch {
                    expected: subject_types.len(),
                    actual: clause.pattern.len(),
                },
                clause.location,
            ));
        }

        let expected_bindings = collect_pattern_bindings(&clause.pattern)?;
        let mut clause_env = env.clone();
        let pattern = clause
            .pattern
            .into_iter()
            .zip(subject_types.iter().cloned())
            .map(|(pattern, type_)| {
                PatternTyper::new(self.environment).analyse_pattern(pattern, type_, &mut clause_env)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let expected_binding_types = expected_bindings
            .keys()
            .map(|name| {
                (
                    name.clone(),
                    clause_env
                        .get(name)
                        .expect("pattern binding should be present after analysis"),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let alternative_patterns = clause
            .alternative_patterns
            .into_iter()
            .map(|alternative| {
                if alternative.len() != subject_types.len() {
                    return Err(error(
                        AnalyseErrorType::CasePatternArityMismatch {
                            expected: subject_types.len(),
                            actual: alternative.len(),
                        },
                        clause.location,
                    ));
                }
                let alternative_bindings = collect_pattern_bindings(&alternative)?;
                check_alternative_pattern_bindings(&expected_bindings, &alternative_bindings)?;
                let mut alternative_env = env.clone();
                let typed = alternative
                    .into_iter()
                    .zip(subject_types.iter().cloned())
                    .map(|(pattern, type_)| {
                        PatternTyper::new(self.environment).analyse_pattern(
                            pattern,
                            type_,
                            &mut alternative_env,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                for (name, expected_type) in &expected_binding_types {
                    let actual_type = alternative_env
                        .get(name)
                        .expect("checked alternative pattern binding should exist");
                    self.environment.expect_type(
                        expected_type.clone(),
                        actual_type,
                        clause.location,
                    )?;
                }

                Ok(typed)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let guard = if let Some(guard) = clause.guard {
            let expression = self.analyse_expression(guard.expression, &mut clause_env)?;
            self.environment
                .expect_type(bool(), expression.type_(), expression.location())?;
            Some(ClauseGuard {
                location: guard.location,
                expression,
            })
        } else {
            None
        };
        let then = self.analyse_expression_with_expected(
            clause.then,
            &mut clause_env,
            Some(expected_result),
        )?;

        Ok(Clause {
            location: clause.location,
            pattern,
            alternative_patterns,
            guard,
            then,
        })
    }

    fn analyse_call(
        &self,
        fun: &TypedExpr,
        arguments: Vec<CallArg<UntypedExpr>>,
        env: &mut LocalEnv,
        location: SrcSpan,
    ) -> Result<(Vec<CallArg<TypedExpr>>, Arc<Type>), AnalyseError> {
        let arguments = arguments
            .into_iter()
            .map(|argument| CallArg {
                location: argument.location,
                label: argument.label,
                value: CallArgInput::Untyped(argument.value),
            })
            .collect::<Vec<_>>();
        let parameter_labels = self.parameter_labels_for_fun(fun, env);
        self.analyse_call_inputs(fun, arguments, &parameter_labels, env, location)
    }

    fn analyse_call_inputs(
        &self,
        fun: &TypedExpr,
        arguments: Vec<CallArg<CallArgInput>>,
        parameter_labels: &[Option<EcoString>],
        env: &mut LocalEnv,
        location: SrcSpan,
    ) -> Result<(Vec<CallArg<TypedExpr>>, Arc<Type>), AnalyseError> {
        let (parameter_types, return_) =
            self.environment
                .match_fun_type(fun.type_(), arguments.len(), location)?;
        let arguments =
            reorder_call_args(arguments, parameter_labels, parameter_types.len(), location)?;

        let mut substitutions = HashMap::new();
        let mut typed_arguments = Vec::with_capacity(arguments.len());
        for (parameter, argument) in parameter_types.iter().zip(arguments) {
            let value = match argument.value {
                CallArgInput::Untyped(value) => {
                    self.analyse_expression_with_expected(value, env, Some(parameter.clone()))?
                }
                CallArgInput::Typed(value) => value,
            };
            if !self
                .environment
                .unify(parameter, &value.type_(), &mut substitutions)
            {
                return Err(error(
                    AnalyseErrorType::TypeMismatch {
                        expected: parameter.clone(),
                        actual: value.type_(),
                    },
                    argument.location,
                ));
            }
            typed_arguments.push(CallArg {
                location: argument.location,
                label: argument.label,
                value,
            });
        }

        Ok((
            typed_arguments,
            self.environment.substitute_type(&return_, &substitutions),
        ))
    }

    fn parameter_labels_for_fun(&self, fun: &TypedExpr, env: &LocalEnv) -> Vec<Option<EcoString>> {
        match fun {
            TypedExpr::Var { name, .. } if env.get(name).is_none() => self
                .environment
                .values
                .get(name)
                .map(|constructor| constructor.parameter_labels.clone())
                .unwrap_or_default(),
            TypedExpr::ModuleSelect {
                module_name, label, ..
            } => self
                .environment
                .importable_modules
                .get(module_name)
                .and_then(|interface| interface.values.get(label))
                .map(|constructor| constructor.parameter_labels.clone())
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }
}

fn collect_pattern_bindings(
    patterns: &[Pattern<()>],
) -> Result<BTreeMap<EcoString, SrcSpan>, AnalyseError> {
    let mut bindings = BTreeMap::new();
    for pattern in patterns {
        collect_pattern_binding(pattern, &mut bindings)?;
    }
    Ok(bindings)
}

fn collect_pattern_binding(
    pattern: &Pattern<()>,
    bindings: &mut BTreeMap<EcoString, SrcSpan>,
) -> Result<(), AnalyseError> {
    match pattern {
        Pattern::Variable { location, name, .. } => insert_binding(bindings, name, *location),
        Pattern::Assign {
            name,
            location,
            pattern,
        } => {
            insert_binding(bindings, name, *location)?;
            collect_pattern_binding(pattern, bindings)
        }
        Pattern::List { elements, .. } | Pattern::Tuple { elements, .. } => {
            for element in elements {
                collect_pattern_binding(element, bindings)?;
            }
            Ok(())
        }
        Pattern::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_pattern_binding(&argument.value, bindings)?;
            }
            Ok(())
        }
        Pattern::StringPrefix {
            left_side_assignment,
            right_side_assignment,
            ..
        } => {
            if let Some((location, name)) = left_side_assignment {
                insert_binding(bindings, name, *location)?;
            }
            if let AssignName::Variable((location, name)) = right_side_assignment {
                insert_binding(bindings, name, *location)?;
            }
            Ok(())
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Discard { .. } => Ok(()),
    }
}

fn insert_binding(
    bindings: &mut BTreeMap<EcoString, SrcSpan>,
    name: &EcoString,
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    if bindings.insert(name.clone(), location).is_some() {
        return Err(error(
            AnalyseErrorType::DuplicateVarInPattern { name: name.clone() },
            location,
        ));
    }
    Ok(())
}

fn check_alternative_pattern_bindings(
    expected: &BTreeMap<EcoString, SrcSpan>,
    actual: &BTreeMap<EcoString, SrcSpan>,
) -> Result<(), AnalyseError> {
    for (name, location) in actual {
        if !expected.contains_key(name) {
            return Err(error(
                AnalyseErrorType::ExtraVarInAlternativePattern { name: name.clone() },
                *location,
            ));
        }
    }
    for (name, location) in expected {
        if !actual.contains_key(name) {
            return Err(error(
                AnalyseErrorType::MissingVarInAlternativePattern { name: name.clone() },
                *location,
            ));
        }
    }
    Ok(())
}
