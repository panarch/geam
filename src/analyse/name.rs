use crate::ast::{
    AssignName, CustomType, Function, Import, Pattern, RecordConstructor, SrcSpan, Statement,
    TypeAlias, TypeAst, UntypedExpr,
};
use ecow::EcoString;
use std::collections::HashMap;

use super::{AnalyseError, AnalyseErrorType, ModuleAnalyzer, NameKind, error};

impl ModuleAnalyzer<'_> {
    pub(super) fn check_unique_top_level_names(
        &self,
        custom_types: &[CustomType<()>],
        type_aliases: &[TypeAlias<()>],
        functions: &[Function<(), UntypedExpr>],
    ) -> Result<(), AnalyseError> {
        let mut type_names = HashMap::new();
        let mut value_names = HashMap::new();

        for custom_type in custom_types {
            insert_unique_name(&mut type_names, &custom_type.name.1, custom_type.name.0)?;
            check_unique_spanned_names(&custom_type.parameters)?;
            for constructor in &custom_type.constructors {
                insert_unique_name(&mut value_names, &constructor.name.1, constructor.name.0)?;
                check_record_constructor_field_order(constructor)?;
                let labels = constructor
                    .arguments
                    .iter()
                    .filter_map(|argument| argument.label.clone())
                    .collect::<Vec<_>>();
                check_unique_spanned_names(&labels)?;
            }
        }

        for type_alias in type_aliases {
            insert_unique_name(&mut type_names, &type_alias.name.1, type_alias.name.0)?;
            check_unique_spanned_names(&type_alias.parameters)?;
        }

        for function in functions {
            if let Some((location, name)) = &function.name {
                insert_unique_name(&mut value_names, name, *location)?;
            }
            let arguments = function
                .arguments
                .iter()
                .map(|argument| argument.name.clone())
                .collect::<Vec<_>>();
            check_unique_spanned_names(&arguments)?;
        }

        Ok(())
    }

    pub(super) fn check_unique_import_names(&self, imports: &[Import]) -> Result<(), AnalyseError> {
        let mut names = HashMap::new();
        for import in imports {
            let local_name = import_local_name(import);
            insert_unique_name(&mut names, &local_name.1, local_name.0)?;
        }
        Ok(())
    }

    pub(super) fn check_name_cases(
        &self,
        imports: &[Import],
        custom_types: &[CustomType<()>],
        type_aliases: &[TypeAlias<()>],
        functions: &[Function<(), UntypedExpr>],
    ) -> Result<(), AnalyseError> {
        for import in imports {
            let (location, name) = import_local_name(import);
            check_name_case(location, &name, NameKind::Variable)?;
        }

        for custom_type in custom_types {
            check_name_case(custom_type.name.0, &custom_type.name.1, NameKind::Type)?;
            for (location, parameter) in &custom_type.parameters {
                check_name_case(*location, parameter, NameKind::TypeVariable)?;
            }
            for constructor in &custom_type.constructors {
                check_name_case(
                    constructor.name.0,
                    &constructor.name.1,
                    NameKind::CustomTypeVariant,
                )?;
                for argument in &constructor.arguments {
                    if let Some((location, label)) = &argument.label {
                        check_name_case(*location, label, NameKind::Label)?;
                    }
                    check_type_ast_names(&argument.annotation)?;
                }
            }
        }

        for type_alias in type_aliases {
            check_name_case(type_alias.name.0, &type_alias.name.1, NameKind::TypeAlias)?;
            for (location, parameter) in &type_alias.parameters {
                check_name_case(*location, parameter, NameKind::TypeVariable)?;
            }
            check_type_ast_names(&type_alias.alias)?;
        }

        for function in functions {
            if let Some((location, name)) = &function.name {
                check_name_case(*location, name, NameKind::Function)?;
            }
            for argument in &function.arguments {
                check_name_case(argument.name.0, &argument.name.1, NameKind::Argument)?;
                if let Some(annotation) = &argument.annotation {
                    check_type_ast_names(annotation)?;
                }
            }
            if let Some(annotation) = &function.return_annotation {
                check_type_ast_names(annotation)?;
            }
            for statement in &function.body {
                check_statement_names(statement)?;
            }
        }

        Ok(())
    }
}

fn check_statement_names(statement: &Statement<(), UntypedExpr>) -> Result<(), AnalyseError> {
    match statement {
        Statement::Expression(expression) => check_expr_names(expression),
        Statement::Assignment(assignment) => {
            check_pattern_names(&assignment.pattern)?;
            if let Some(annotation) = &assignment.annotation {
                check_type_ast_names(annotation)?;
            }
            check_expr_names(&assignment.value)
        }
    }
}

fn check_expr_names(expression: &UntypedExpr) -> Result<(), AnalyseError> {
    match expression {
        UntypedExpr::Var { location, name } => {
            if name.chars().next().is_some_and(char::is_uppercase) {
                check_name_case(*location, name, NameKind::CustomTypeVariant)
            } else {
                check_name_case(*location, name, NameKind::Variable)
            }
        }
        UntypedExpr::Block { statements, .. } => {
            for statement in statements {
                check_statement_names(statement)?;
            }
            Ok(())
        }
        UntypedExpr::List { elements, .. } | UntypedExpr::Tuple { elements, .. } => {
            for element in elements {
                check_expr_names(element)?;
            }
            Ok(())
        }
        UntypedExpr::Call { fun, arguments, .. } => {
            check_expr_names(fun)?;
            for argument in arguments {
                if let Some((location, label)) = &argument.label {
                    check_name_case(*location, label, NameKind::Label)?;
                }
                check_expr_names(&argument.value)?;
            }
            Ok(())
        }
        UntypedExpr::BinOp { left, right, .. } => {
            check_expr_names(left)?;
            check_expr_names(right)
        }
        UntypedExpr::PipeLine { expressions } => {
            for expression in expressions {
                check_expr_names(expression)?;
            }
            Ok(())
        }
        UntypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                check_expr_names(subject)?;
            }
            for clause in clauses {
                for pattern in &clause.pattern {
                    check_pattern_names(pattern)?;
                }
                for alternative in &clause.alternative_patterns {
                    for pattern in alternative {
                        check_pattern_names(pattern)?;
                    }
                }
                if let Some(guard) = &clause.guard {
                    check_expr_names(&guard.expression)?;
                }
                check_expr_names(&clause.then)?;
            }
            Ok(())
        }
        UntypedExpr::FieldAccess {
            container,
            label_location,
            label,
            ..
        } => {
            check_expr_names(container)?;
            if label.chars().next().is_some_and(char::is_uppercase) {
                check_name_case(*label_location, label, NameKind::CustomTypeVariant)
            } else {
                check_name_case(*label_location, label, NameKind::Label)
            }
        }
        UntypedExpr::TupleIndex { tuple, .. }
        | UntypedExpr::NegateBool { value: tuple, .. }
        | UntypedExpr::NegateInt { value: tuple, .. } => check_expr_names(tuple),
        UntypedExpr::Int { .. } | UntypedExpr::Float { .. } | UntypedExpr::String { .. } => Ok(()),
    }
}

fn check_pattern_names(pattern: &Pattern<()>) -> Result<(), AnalyseError> {
    match pattern {
        Pattern::Variable { location, name, .. } => {
            check_name_case(*location, name, NameKind::Variable)
        }
        Pattern::Assign {
            name,
            location,
            pattern,
        } => {
            check_name_case(*location, name, NameKind::Variable)?;
            check_pattern_names(pattern)
        }
        Pattern::Discard { location, name, .. } => {
            check_name_case(*location, name, NameKind::Discard)
        }
        Pattern::List { elements, .. } | Pattern::Tuple { elements, .. } => {
            for element in elements {
                check_pattern_names(element)?;
            }
            Ok(())
        }
        Pattern::Constructor {
            name_location,
            name,
            arguments,
            ..
        } => {
            check_name_case(*name_location, name, NameKind::CustomTypeVariant)?;
            for argument in arguments {
                if let Some((location, label)) = &argument.label {
                    check_name_case(*location, label, NameKind::Label)?;
                }
                check_pattern_names(&argument.value)?;
            }
            Ok(())
        }
        Pattern::StringPrefix {
            left_side_assignment,
            right_side_assignment,
            ..
        } => {
            if let Some((location, name)) = left_side_assignment {
                check_name_case(*location, name, NameKind::Variable)?;
            }
            check_assign_name_case(right_side_assignment)
        }
        Pattern::Int { .. } | Pattern::Float { .. } | Pattern::String { .. } => Ok(()),
    }
}

fn check_assign_name_case(name: &AssignName) -> Result<(), AnalyseError> {
    match name {
        AssignName::Variable((location, name)) => {
            check_name_case(*location, name, NameKind::Variable)
        }
        AssignName::Discard((location, name)) => {
            check_name_case(*location, name, NameKind::Discard)
        }
    }
}

fn check_type_ast_names(type_ast: &TypeAst) -> Result<(), AnalyseError> {
    match type_ast {
        TypeAst::Constructor {
            module,
            name,
            arguments,
            ..
        } => {
            if let Some((module_alias, location)) = module {
                check_name_case(*location, module_alias, NameKind::Variable)?;
            }
            check_name_case(name.0, &name.1, NameKind::Type)?;
            for argument in arguments {
                check_type_ast_names(argument)?;
            }
            Ok(())
        }
        TypeAst::Fn {
            arguments, return_, ..
        } => {
            for argument in arguments {
                check_type_ast_names(argument)?;
            }
            check_type_ast_names(return_)
        }
        TypeAst::Var { location, name } => check_name_case(*location, name, NameKind::TypeVariable),
        TypeAst::Tuple { elements, .. } => {
            for element in elements {
                check_type_ast_names(element)?;
            }
            Ok(())
        }
        TypeAst::Hole { .. } => Ok(()),
    }
}

fn check_name_case(
    location: SrcSpan,
    name: &EcoString,
    kind: NameKind,
) -> Result<(), AnalyseError> {
    let valid = match kind {
        NameKind::Type | NameKind::TypeAlias | NameKind::CustomTypeVariant => valid_upname(name),
        NameKind::Variable
        | NameKind::TypeVariable
        | NameKind::Argument
        | NameKind::Label
        | NameKind::Function => valid_name(name),
        NameKind::Discard => valid_discard_name(name),
    };

    if valid {
        Ok(())
    } else {
        Err(error(
            AnalyseErrorType::InvalidName {
                kind,
                name: name.clone(),
            },
            location,
        ))
    }
}

fn valid_name(name: &EcoString) -> bool {
    let rest = name.strip_prefix('_').unwrap_or(name.as_str());
    let mut chars = rest.chars();
    chars.next().is_some_and(|char| char.is_ascii_lowercase())
        && chars.all(|char| char.is_ascii_lowercase() || char.is_ascii_digit() || char == '_')
}

fn valid_discard_name(name: &EcoString) -> bool {
    let Some(rest) = name.strip_prefix('_') else {
        return false;
    };
    rest.chars()
        .all(|char| char.is_ascii_lowercase() || char.is_ascii_digit() || char == '_')
}

fn valid_upname(name: &EcoString) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|char| char.is_ascii_uppercase())
        && chars.all(|char| char.is_ascii_alphanumeric())
}

fn check_record_constructor_field_order(
    constructor: &RecordConstructor<()>,
) -> Result<(), AnalyseError> {
    let mut seen_labelled = false;
    for argument in &constructor.arguments {
        if argument.label.is_some() {
            seen_labelled = true;
        } else if seen_labelled {
            return Err(error(
                AnalyseErrorType::UnlabelledArgumentAfterLabelled,
                argument.location,
            ));
        }
    }
    Ok(())
}

fn insert_unique_name(
    names: &mut HashMap<EcoString, SrcSpan>,
    name: &EcoString,
    location: SrcSpan,
) -> Result<(), AnalyseError> {
    match names.insert(name.clone(), location) {
        Some(previous_location) => Err(error(
            AnalyseErrorType::DuplicateName {
                name: name.clone(),
                previous_location,
            },
            location,
        )),
        None => Ok(()),
    }
}

fn check_unique_spanned_names(names: &[(SrcSpan, EcoString)]) -> Result<(), AnalyseError> {
    let mut seen = HashMap::new();
    for (location, name) in names {
        insert_unique_name(&mut seen, name, *location)?;
    }
    Ok(())
}

fn import_local_name(import: &Import) -> (SrcSpan, EcoString) {
    import.alias.clone().unwrap_or_else(|| {
        (
            import.location,
            import
                .module
                .rsplit('/')
                .next()
                .unwrap_or(&import.module)
                .into(),
        )
    })
}
