use crate::ast::{AssignName, Function, Pattern, Statement, TypeAlias, TypeAst, UntypedExpr};
use ecow::EcoString;
use std::collections::{HashMap, HashSet};

use super::{AnalyseError, AnalyseErrorType, error};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Unvisited,
    Visiting,
    Visited,
}

pub(super) fn sort_type_aliases(
    aliases: &[TypeAlias<()>],
) -> Result<Vec<TypeAlias<()>>, AnalyseError> {
    let alias_names = aliases
        .iter()
        .enumerate()
        .map(|(index, alias)| (alias.name.1.clone(), index))
        .collect::<HashMap<_, _>>();
    let mut states = vec![VisitState::Unvisited; aliases.len()];
    let mut order = Vec::with_capacity(aliases.len());

    for index in 0..aliases.len() {
        visit_type_alias(index, aliases, &alias_names, &mut states, &mut order)?;
    }

    Ok(order
        .into_iter()
        .map(|index| aliases[index].clone())
        .collect())
}

fn visit_type_alias(
    index: usize,
    aliases: &[TypeAlias<()>],
    alias_names: &HashMap<EcoString, usize>,
    states: &mut [VisitState],
    order: &mut Vec<usize>,
) -> Result<(), AnalyseError> {
    match states[index] {
        VisitState::Visited => return Ok(()),
        VisitState::Visiting => {
            return Err(error(
                AnalyseErrorType::RecursiveTypeAlias {
                    name: aliases[index].name.1.clone(),
                },
                aliases[index].location,
            ));
        }
        VisitState::Unvisited => {}
    }

    states[index] = VisitState::Visiting;
    let mut dependencies = Vec::new();
    collect_type_alias_dependencies(&aliases[index].alias, alias_names, &mut dependencies);
    for dependency in dependencies {
        visit_type_alias(dependency, aliases, alias_names, states, order)?;
    }
    states[index] = VisitState::Visited;
    order.push(index);
    Ok(())
}

fn collect_type_alias_dependencies(
    type_ast: &TypeAst,
    alias_names: &HashMap<EcoString, usize>,
    dependencies: &mut Vec<usize>,
) {
    match type_ast {
        TypeAst::Constructor {
            module,
            name,
            arguments,
            ..
        } => {
            if module.is_none()
                && let Some(index) = alias_names.get(&name.1)
                && !dependencies.contains(index)
            {
                dependencies.push(*index);
            }
            for argument in arguments {
                collect_type_alias_dependencies(argument, alias_names, dependencies);
            }
        }
        TypeAst::Fn {
            arguments, return_, ..
        } => {
            for argument in arguments {
                collect_type_alias_dependencies(argument, alias_names, dependencies);
            }
            collect_type_alias_dependencies(return_, alias_names, dependencies);
        }
        TypeAst::Tuple { elements, .. } => {
            for element in elements {
                collect_type_alias_dependencies(element, alias_names, dependencies);
            }
        }
        TypeAst::Var { .. } | TypeAst::Hole { .. } => {}
    }
}

pub(super) fn collect_type_ast_variables(type_ast: &TypeAst, variables: &mut HashSet<EcoString>) {
    match type_ast {
        TypeAst::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_type_ast_variables(argument, variables);
            }
        }
        TypeAst::Fn {
            arguments, return_, ..
        } => {
            for argument in arguments {
                collect_type_ast_variables(argument, variables);
            }
            collect_type_ast_variables(return_, variables);
        }
        TypeAst::Tuple { elements, .. } => {
            for element in elements {
                collect_type_ast_variables(element, variables);
            }
        }
        TypeAst::Var { name, .. } => {
            variables.insert(name.clone());
        }
        TypeAst::Hole { .. } => {}
    }
}

pub(super) fn sort_function_groups(functions: &[Function<(), UntypedExpr>]) -> Vec<Vec<usize>> {
    let function_names = functions
        .iter()
        .enumerate()
        .filter_map(|(index, function)| {
            function
                .name
                .as_ref()
                .map(|(_, name)| (name.clone(), index))
        })
        .collect::<HashMap<_, _>>();
    let dependencies = functions
        .iter()
        .map(|function| function_dependencies(function, &function_names))
        .collect::<Vec<_>>();
    let mut sorter = FunctionGroupSorter::new(dependencies);

    for index in 0..functions.len() {
        sorter.visit(index);
    }

    sorter.groups
}

struct FunctionGroupSorter {
    dependencies: Vec<Vec<usize>>,
    next_index: usize,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    indices: Vec<Option<usize>>,
    lowlinks: Vec<usize>,
    groups: Vec<Vec<usize>>,
}

impl FunctionGroupSorter {
    fn new(dependencies: Vec<Vec<usize>>) -> Self {
        let len = dependencies.len();
        Self {
            dependencies,
            next_index: 0,
            stack: Vec::new(),
            on_stack: vec![false; len],
            indices: vec![None; len],
            lowlinks: vec![0; len],
            groups: Vec::with_capacity(len),
        }
    }

    fn visit(&mut self, index: usize) {
        if self.indices[index].is_some() {
            return;
        }

        let current_index = self.next_index;
        self.indices[index] = Some(current_index);
        self.lowlinks[index] = current_index;
        self.next_index += 1;
        self.stack.push(index);
        self.on_stack[index] = true;

        for dependency in self.dependencies[index].clone() {
            if self.indices[dependency].is_none() {
                self.visit(dependency);
                self.lowlinks[index] = self.lowlinks[index].min(self.lowlinks[dependency]);
            } else if self.on_stack[dependency] {
                let dependency_index = self.indices[dependency]
                    .expect("visited dependency should have a dependency index");
                self.lowlinks[index] = self.lowlinks[index].min(dependency_index);
            }
        }

        if self.lowlinks[index] == current_index {
            let mut group = Vec::new();
            loop {
                let function = self.stack.pop().expect("root should be on stack");
                self.on_stack[function] = false;
                group.push(function);
                if function == index {
                    break;
                }
            }
            self.groups.push(group);
        }
    }
}

pub(super) fn function_dependencies(
    function: &Function<(), UntypedExpr>,
    function_names: &HashMap<EcoString, usize>,
) -> Vec<usize> {
    let mut local_names = function
        .arguments
        .iter()
        .map(|argument| argument.name.1.clone())
        .collect::<HashSet<_>>();
    let mut dependencies = Vec::new();

    for statement in &function.body {
        collect_statement_dependencies(
            statement,
            &mut local_names,
            function_names,
            &mut dependencies,
        );
    }

    dependencies
}

fn collect_statement_dependencies(
    statement: &Statement<(), UntypedExpr>,
    local_names: &mut HashSet<EcoString>,
    function_names: &HashMap<EcoString, usize>,
    dependencies: &mut Vec<usize>,
) {
    match statement {
        Statement::Expression(expression) => {
            collect_expression_dependencies(expression, local_names, function_names, dependencies)
        }
        Statement::Assignment(assignment) => {
            collect_expression_dependencies(
                &assignment.value,
                local_names,
                function_names,
                dependencies,
            );
            collect_pattern_names(&assignment.pattern, local_names);
        }
    }
}

fn collect_expression_dependencies(
    expression: &UntypedExpr,
    local_names: &mut HashSet<EcoString>,
    function_names: &HashMap<EcoString, usize>,
    dependencies: &mut Vec<usize>,
) {
    match expression {
        UntypedExpr::Var { name, .. } => {
            if !local_names.contains(name)
                && let Some(index) = function_names.get(name)
                && !dependencies.contains(index)
            {
                dependencies.push(*index);
            }
        }
        UntypedExpr::Block { statements, .. } => {
            let mut block_names = local_names.clone();
            for statement in statements {
                collect_statement_dependencies(
                    statement,
                    &mut block_names,
                    function_names,
                    dependencies,
                );
            }
        }
        UntypedExpr::List { elements, .. }
        | UntypedExpr::Tuple { elements, .. }
        | UntypedExpr::PipeLine {
            expressions: elements,
        } => {
            for element in elements {
                collect_expression_dependencies(element, local_names, function_names, dependencies);
            }
        }
        UntypedExpr::Call { fun, arguments, .. } => {
            collect_expression_dependencies(fun, local_names, function_names, dependencies);
            for argument in arguments {
                collect_expression_dependencies(
                    &argument.value,
                    local_names,
                    function_names,
                    dependencies,
                );
            }
        }
        UntypedExpr::BinOp { left, right, .. } => {
            collect_expression_dependencies(left, local_names, function_names, dependencies);
            collect_expression_dependencies(right, local_names, function_names, dependencies);
        }
        UntypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                collect_expression_dependencies(subject, local_names, function_names, dependencies);
            }
            for clause in clauses {
                let mut clause_names = local_names.clone();
                for pattern in &clause.pattern {
                    collect_pattern_names(pattern, &mut clause_names);
                }
                for alternative in &clause.alternative_patterns {
                    for pattern in alternative {
                        collect_pattern_names(pattern, &mut clause_names);
                    }
                }
                if let Some(guard) = &clause.guard {
                    collect_expression_dependencies(
                        &guard.expression,
                        &mut clause_names,
                        function_names,
                        dependencies,
                    );
                }
                collect_expression_dependencies(
                    &clause.then,
                    &mut clause_names,
                    function_names,
                    dependencies,
                );
            }
        }
        UntypedExpr::FieldAccess { .. } => {}
        UntypedExpr::TupleIndex {
            tuple: container, ..
        }
        | UntypedExpr::NegateBool {
            value: container, ..
        }
        | UntypedExpr::NegateInt {
            value: container, ..
        } => collect_expression_dependencies(container, local_names, function_names, dependencies),
        UntypedExpr::Int { .. } | UntypedExpr::Float { .. } | UntypedExpr::String { .. } => {}
    }
}

fn collect_pattern_names(pattern: &Pattern<()>, names: &mut HashSet<EcoString>) {
    match pattern {
        Pattern::Variable { name, .. } | Pattern::Assign { name, .. } => {
            names.insert(name.clone());
            if let Pattern::Assign { pattern, .. } = pattern {
                collect_pattern_names(pattern, names);
            }
        }
        Pattern::List { elements, .. } | Pattern::Tuple { elements, .. } => {
            for element in elements {
                collect_pattern_names(element, names);
            }
        }
        Pattern::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_pattern_names(&argument.value, names);
            }
        }
        Pattern::StringPrefix {
            left_side_assignment,
            right_side_assignment,
            ..
        } => {
            if let Some((_, name)) = left_side_assignment {
                names.insert(name.clone());
            }
            if let AssignName::Variable((_, name)) = right_side_assignment {
                names.insert(name.clone());
            }
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Discard { .. } => {}
    }
}
