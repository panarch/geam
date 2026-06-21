use crate::ast::{
    Arg, Assignment, CallArg, Clause, ClauseGuard, CustomType, Function, Pattern,
    RecordConstructor, RecordConstructorArg, SrcSpan, Statement, TypeAlias, TypedExpr,
};
use crate::type_::{Type, TypeGeneralizer};
use std::sync::Arc;

use super::{AnalyseError, AnalyseErrorType, ModuleAnalyzer, error};

impl ModuleAnalyzer<'_> {
    pub(super) fn finalize_type(
        &self,
        type_: Arc<Type>,
        location: SrcSpan,
    ) -> Result<Arc<Type>, AnalyseError> {
        let type_ = self.environment.generalise_inferred_type(&type_);
        if self.environment.has_unresolved_infer(&type_) {
            return Err(error(AnalyseErrorType::UnresolvedType { type_ }, location));
        }
        Ok(type_)
    }

    fn finalize_type_with(
        &self,
        type_: Arc<Type>,
        location: SrcSpan,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<Arc<Type>, AnalyseError> {
        let type_ = generalizer.generalise(&type_);
        if self.environment.has_unresolved_infer(&type_) {
            return Err(error(AnalyseErrorType::UnresolvedType { type_ }, location));
        }
        Ok(type_)
    }

    pub(super) fn finalize_type_alias(
        &self,
        alias: TypeAlias<Arc<Type>>,
    ) -> Result<TypeAlias<Arc<Type>>, AnalyseError> {
        Ok(TypeAlias {
            location: alias.location,
            publicity: alias.publicity,
            name: alias.name,
            parameters: alias.parameters,
            alias: alias.alias,
            type_: self.finalize_type(alias.type_, alias.location)?,
        })
    }

    pub(super) fn finalize_custom_type(
        &self,
        custom_type: CustomType<Arc<Type>>,
    ) -> Result<CustomType<Arc<Type>>, AnalyseError> {
        let constructors = custom_type
            .constructors
            .into_iter()
            .map(|constructor| self.finalize_record_constructor(constructor))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CustomType {
            location: custom_type.location,
            publicity: custom_type.publicity,
            name: custom_type.name,
            parameters: custom_type.parameters,
            constructors,
            type_: self.finalize_type(custom_type.type_, custom_type.location)?,
        })
    }

    fn finalize_record_constructor(
        &self,
        constructor: RecordConstructor<Arc<Type>>,
    ) -> Result<RecordConstructor<Arc<Type>>, AnalyseError> {
        let arguments = constructor
            .arguments
            .into_iter()
            .map(|argument| {
                Ok(RecordConstructorArg {
                    location: argument.location,
                    label: argument.label,
                    annotation: argument.annotation,
                    type_: self.finalize_type(argument.type_, argument.location)?,
                })
            })
            .collect::<Result<Vec<_>, AnalyseError>>()?;
        Ok(RecordConstructor {
            location: constructor.location,
            name: constructor.name,
            arguments,
        })
    }

    pub(super) fn finalize_function(
        &self,
        function: Function<Arc<Type>, TypedExpr>,
    ) -> Result<Function<Arc<Type>, TypedExpr>, AnalyseError> {
        let mut generalizer =
            TypeGeneralizer::new(&self.environment, &collect_function_types(&function));
        let location = function.location;
        let arguments = function
            .arguments
            .into_iter()
            .map(|argument| {
                let type_ =
                    self.finalize_type_with(argument.type_, argument.location, &mut generalizer)?;
                Ok(Arg {
                    location: argument.location,
                    name: argument.name,
                    annotation: argument.annotation,
                    type_,
                })
            })
            .collect::<Result<Vec<_>, AnalyseError>>()?;
        let return_type =
            self.finalize_type_with(function.return_type, location, &mut generalizer)?;
        let body = function
            .body
            .into_iter()
            .map(|statement| self.finalize_statement(statement, &mut generalizer))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Function {
            location,
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

    fn finalize_statement(
        &self,
        statement: Statement<Arc<Type>, TypedExpr>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<Statement<Arc<Type>, TypedExpr>, AnalyseError> {
        match statement {
            Statement::Expression(expression) => Ok(Statement::Expression(
                self.finalize_expr(expression, generalizer)?,
            )),
            Statement::Assignment(assignment) => {
                let Assignment {
                    location,
                    pattern,
                    annotation,
                    value,
                } = *assignment;
                Ok(Statement::Assignment(Box::new(Assignment {
                    location,
                    pattern: self.finalize_pattern(pattern, generalizer)?,
                    annotation,
                    value: self.finalize_expr(value, generalizer)?,
                })))
            }
        }
    }

    fn finalize_pattern(
        &self,
        pattern: Pattern<Arc<Type>>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<Pattern<Arc<Type>>, AnalyseError> {
        Ok(match pattern {
            Pattern::Variable {
                location,
                name,
                type_,
            } => Pattern::Variable {
                location,
                name,
                type_: self.finalize_type_with(type_, location, generalizer)?,
            },
            Pattern::Assign {
                name,
                location,
                pattern,
            } => Pattern::Assign {
                name,
                location,
                pattern: Box::new(self.finalize_pattern(*pattern, generalizer)?),
            },
            Pattern::Discard {
                name,
                location,
                type_,
            } => Pattern::Discard {
                name,
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
            },
            Pattern::List {
                location,
                elements,
                type_,
            } => Pattern::List {
                location,
                elements: elements
                    .into_iter()
                    .map(|element| self.finalize_pattern(element, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
                type_: self.finalize_type_with(type_, location, generalizer)?,
            },
            Pattern::Constructor {
                location,
                name_location,
                name,
                arguments,
                module,
                type_,
            } => Pattern::Constructor {
                location,
                name_location,
                name,
                arguments: arguments
                    .into_iter()
                    .map(|argument| self.finalize_pattern_call_arg(argument, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
                module,
                type_: self.finalize_type_with(type_, location, generalizer)?,
            },
            Pattern::Tuple { location, elements } => Pattern::Tuple {
                location,
                elements: elements
                    .into_iter()
                    .map(|element| self.finalize_pattern(element, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Pattern::Int {
                location,
                value,
                int_value,
            } => Pattern::Int {
                location,
                value,
                int_value,
            },
            Pattern::Float { location, value } => Pattern::Float { location, value },
            Pattern::String { location, value } => Pattern::String { location, value },
            Pattern::StringPrefix {
                location,
                left_location,
                left_side_assignment,
                right_location,
                left_side_string,
                right_side_assignment,
            } => Pattern::StringPrefix {
                location,
                left_location,
                left_side_assignment,
                right_location,
                left_side_string,
                right_side_assignment,
            },
        })
    }

    fn finalize_pattern_call_arg(
        &self,
        argument: CallArg<Pattern<Arc<Type>>>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<CallArg<Pattern<Arc<Type>>>, AnalyseError> {
        Ok(CallArg {
            location: argument.location,
            label: argument.label,
            value: self.finalize_pattern(argument.value, generalizer)?,
        })
    }

    fn finalize_expr(
        &self,
        expression: TypedExpr,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<TypedExpr, AnalyseError> {
        Ok(match expression {
            TypedExpr::Int {
                location,
                type_,
                value,
                int_value,
            } => TypedExpr::Int {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                value,
                int_value,
            },
            TypedExpr::Float {
                location,
                type_,
                value,
            } => TypedExpr::Float {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                value,
            },
            TypedExpr::String {
                location,
                type_,
                value,
            } => TypedExpr::String {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                value,
            },
            TypedExpr::Block {
                location,
                type_,
                statements,
            } => TypedExpr::Block {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                statements: statements
                    .into_iter()
                    .map(|statement| self.finalize_statement(statement, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            TypedExpr::Var {
                location,
                type_,
                name,
            } => TypedExpr::Var {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                name,
            },
            TypedExpr::List {
                location,
                type_,
                elements,
            } => TypedExpr::List {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                elements: elements
                    .into_iter()
                    .map(|element| self.finalize_expr(element, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            TypedExpr::Call {
                location,
                type_,
                fun,
                arguments,
                open_parenthesis,
            } => TypedExpr::Call {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                fun: Box::new(self.finalize_expr(*fun, generalizer)?),
                arguments: arguments
                    .into_iter()
                    .map(|argument| self.finalize_expr_call_arg(argument, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
                open_parenthesis,
            },
            TypedExpr::BinOp {
                location,
                type_,
                operator,
                operator_start,
                left,
                right,
            } => TypedExpr::BinOp {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                operator,
                operator_start,
                left: Box::new(self.finalize_expr(*left, generalizer)?),
                right: Box::new(self.finalize_expr(*right, generalizer)?),
            },
            TypedExpr::Case {
                location,
                type_,
                subjects,
                clauses,
            } => TypedExpr::Case {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                subjects: subjects
                    .into_iter()
                    .map(|subject| self.finalize_expr(subject, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
                clauses: clauses
                    .into_iter()
                    .map(|clause| self.finalize_clause(clause, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            TypedExpr::FieldAccess {
                location,
                type_,
                label_location,
                label,
                container,
            } => TypedExpr::FieldAccess {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                label_location,
                label,
                container: Box::new(self.finalize_expr(*container, generalizer)?),
            },
            TypedExpr::ModuleSelect {
                location,
                type_,
                module_name,
                module_alias,
                label,
            } => TypedExpr::ModuleSelect {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                module_name,
                module_alias,
                label,
            },
            TypedExpr::Tuple {
                location,
                type_,
                elements,
            } => TypedExpr::Tuple {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                elements: elements
                    .into_iter()
                    .map(|element| self.finalize_expr(element, generalizer))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            TypedExpr::TupleIndex {
                location,
                type_,
                index,
                tuple,
            } => TypedExpr::TupleIndex {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                index,
                tuple: Box::new(self.finalize_expr(*tuple, generalizer)?),
            },
            TypedExpr::NegateBool {
                location,
                type_,
                value,
            } => TypedExpr::NegateBool {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                value: Box::new(self.finalize_expr(*value, generalizer)?),
            },
            TypedExpr::NegateInt {
                location,
                type_,
                value,
            } => TypedExpr::NegateInt {
                location,
                type_: self.finalize_type_with(type_, location, generalizer)?,
                value: Box::new(self.finalize_expr(*value, generalizer)?),
            },
        })
    }

    fn finalize_expr_call_arg(
        &self,
        argument: CallArg<TypedExpr>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<CallArg<TypedExpr>, AnalyseError> {
        Ok(CallArg {
            location: argument.location,
            label: argument.label,
            value: self.finalize_expr(argument.value, generalizer)?,
        })
    }

    fn finalize_clause(
        &self,
        clause: Clause<TypedExpr, Arc<Type>>,
        generalizer: &mut TypeGeneralizer<'_, '_>,
    ) -> Result<Clause<TypedExpr, Arc<Type>>, AnalyseError> {
        Ok(Clause {
            location: clause.location,
            pattern: clause
                .pattern
                .into_iter()
                .map(|pattern| self.finalize_pattern(pattern, generalizer))
                .collect::<Result<Vec<_>, _>>()?,
            alternative_patterns: clause
                .alternative_patterns
                .into_iter()
                .map(|patterns| {
                    patterns
                        .into_iter()
                        .map(|pattern| self.finalize_pattern(pattern, generalizer))
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
            guard: clause
                .guard
                .map(|guard| {
                    Ok(ClauseGuard {
                        location: guard.location,
                        expression: self.finalize_expr(guard.expression, generalizer)?,
                    })
                })
                .transpose()?,
            then: self.finalize_expr(clause.then, generalizer)?,
        })
    }

    pub(super) fn check_for_type_leaks(
        &self,
        custom_types: &[CustomType<Arc<Type>>],
        functions: &[Function<Arc<Type>, TypedExpr>],
    ) -> Result<(), AnalyseError> {
        for custom_type in custom_types {
            if !custom_type.publicity.is_public() {
                continue;
            }
            for constructor in &custom_type.constructors {
                for argument in &constructor.arguments {
                    self.check_public_type(&argument.type_, argument.location)?;
                }
            }
        }

        for function in functions {
            if !function.publicity.is_public() {
                continue;
            }
            for argument in &function.arguments {
                self.check_public_type(&argument.type_, argument.location)?;
            }
            self.check_public_type(&function.return_type, function.location)?;
        }

        Ok(())
    }

    fn check_public_type(&self, type_: &Arc<Type>, location: SrcSpan) -> Result<(), AnalyseError> {
        if let Some(type_) = self.environment.find_private_type(type_) {
            return Err(error(AnalyseErrorType::PrivateTypeLeak { type_ }, location));
        }
        Ok(())
    }
}

fn collect_function_types(function: &Function<Arc<Type>, TypedExpr>) -> Vec<Arc<Type>> {
    let mut types = function
        .arguments
        .iter()
        .map(|argument| argument.type_.clone())
        .collect::<Vec<_>>();
    types.push(function.return_type.clone());
    for statement in &function.body {
        collect_statement_types(statement, &mut types);
    }
    types
}

fn collect_statement_types(
    statement: &Statement<Arc<Type>, TypedExpr>,
    types: &mut Vec<Arc<Type>>,
) {
    match statement {
        Statement::Expression(expression) => collect_expr_types(expression, types),
        Statement::Assignment(assignment) => {
            collect_pattern_types(&assignment.pattern, types);
            collect_expr_types(&assignment.value, types);
        }
    }
}

fn collect_pattern_types(pattern: &Pattern<Arc<Type>>, types: &mut Vec<Arc<Type>>) {
    match pattern {
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } => {
            types.push(type_.clone());
        }
        Pattern::Assign { pattern, .. } => collect_pattern_types(pattern, types),
        Pattern::List {
            elements, type_, ..
        } => {
            types.push(type_.clone());
            for element in elements {
                collect_pattern_types(element, types);
            }
        }
        Pattern::Constructor {
            arguments, type_, ..
        } => {
            types.push(type_.clone());
            for argument in arguments {
                collect_pattern_types(&argument.value, types);
            }
        }
        Pattern::Tuple { elements, .. } => {
            for element in elements {
                collect_pattern_types(element, types);
            }
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::StringPrefix { .. } => {}
    }
}

fn collect_expr_types(expression: &TypedExpr, types: &mut Vec<Arc<Type>>) {
    types.push(expression.type_());
    match expression {
        TypedExpr::Block { statements, .. } => {
            for statement in statements {
                collect_statement_types(statement, types);
            }
        }
        TypedExpr::List { elements, .. } | TypedExpr::Tuple { elements, .. } => {
            for element in elements {
                collect_expr_types(element, types);
            }
        }
        TypedExpr::Call { fun, arguments, .. } => {
            collect_expr_types(fun, types);
            for argument in arguments {
                collect_expr_types(&argument.value, types);
            }
        }
        TypedExpr::BinOp { left, right, .. } => {
            collect_expr_types(left, types);
            collect_expr_types(right, types);
        }
        TypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                collect_expr_types(subject, types);
            }
            for clause in clauses {
                for pattern in &clause.pattern {
                    collect_pattern_types(pattern, types);
                }
                for patterns in &clause.alternative_patterns {
                    for pattern in patterns {
                        collect_pattern_types(pattern, types);
                    }
                }
                if let Some(guard) = &clause.guard {
                    collect_expr_types(&guard.expression, types);
                }
                collect_expr_types(&clause.then, types);
            }
        }
        TypedExpr::FieldAccess { container, .. } => collect_expr_types(container, types),
        TypedExpr::TupleIndex { tuple, .. } => collect_expr_types(tuple, types),
        TypedExpr::NegateBool { value, .. } | TypedExpr::NegateInt { value, .. } => {
            collect_expr_types(value, types);
        }
        TypedExpr::Int { .. }
        | TypedExpr::Float { .. }
        | TypedExpr::String { .. }
        | TypedExpr::Var { .. }
        | TypedExpr::ModuleSelect { .. } => {}
    }
}
