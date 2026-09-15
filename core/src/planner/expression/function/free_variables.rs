use ecow::EcoString;
use gleam_compiler_core::ast::{
    AssignmentKind, BitArraySize, ClauseGuard, Pattern, Statement, TypedArg, TypedClause,
    TypedClauseGuard, TypedExpr, TypedPipelineAssignment, TypedStatement,
};
use gleam_compiler_core::type_::{Type, ValueConstructorVariant};
use std::collections::HashSet;
use std::sync::Arc;
use vec1::Vec1;

pub(super) fn anonymous_free_variables(
    arguments: &[TypedArg],
    body: &Vec1<TypedStatement>,
) -> Vec<EcoString> {
    let mut bound = HashSet::new();
    for argument in arguments {
        bound.extend(argument.get_variable_name().cloned());
    }

    let mut free = FreeVariables::new();
    free.collect(Visit::Statements(body.as_slice()), &mut bound);
    free.names
}

struct FreeVariables {
    names: Vec<EcoString>,
    seen: HashSet<EcoString>,
}

enum Visit<'a> {
    Statements(&'a [TypedStatement]),
    Statement(&'a TypedStatement),
    Expression(&'a TypedExpr),
    PipelineAssignment(&'a TypedPipelineAssignment),
    Clause(&'a TypedClause),
    Alternative {
        patterns: &'a [Pattern<Arc<Type>>],
        outer_bound: HashSet<EcoString>,
    },
    Guard(&'a TypedClauseGuard),
    Pattern(&'a Pattern<Arc<Type>>),
    BitArraySize(&'a BitArraySize<Arc<Type>>),
    Bind(&'a EcoString),
    RestoreBound(HashSet<EcoString>),
}

impl FreeVariables {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            seen: HashSet::new(),
        }
    }

    fn collect(&mut self, first: Visit<'_>, bound: &mut HashSet<EcoString>) {
        let mut pending = vec![first];
        // Push children in reverse source order; scope exits restore lexical bindings.
        while let Some(visit) = pending.pop() {
            match visit {
                Visit::Statements(statements) => {
                    pending.extend(statements.iter().rev().map(Visit::Statement));
                }
                Visit::Statement(statement) => schedule_statement(statement, &mut pending),
                Visit::Expression(expression) => {
                    self.collect_expression(expression, bound, &mut pending);
                }
                Visit::PipelineAssignment(assignment) => {
                    pending.push(Visit::Bind(&assignment.name));
                    pending.push(Visit::Expression(&assignment.value));
                }
                Visit::Clause(clause) => {
                    pending.push(Visit::RestoreBound(bound.clone()));
                    pending.push(Visit::Expression(&clause.then));
                    if let Some(guard) = &clause.guard {
                        pending.push(Visit::Guard(guard));
                    }
                    for patterns in clause.alternative_patterns.iter().rev() {
                        pending.push(Visit::Alternative {
                            patterns,
                            outer_bound: bound.clone(),
                        });
                    }
                    pending.extend(clause.pattern.iter().rev().map(Visit::Pattern));
                }
                Visit::Alternative {
                    patterns,
                    outer_bound,
                } => {
                    pending.push(Visit::RestoreBound(std::mem::replace(bound, outer_bound)));
                    pending.extend(patterns.iter().rev().map(Visit::Pattern));
                }
                Visit::Guard(guard) => self.collect_guard(guard, bound, &mut pending),
                Visit::Pattern(pattern) => schedule_pattern(pattern, bound, &mut pending),
                Visit::BitArraySize(size) => self.collect_bit_array_size(size, bound, &mut pending),
                Visit::Bind(name) => {
                    bound.insert(name.clone());
                }
                Visit::RestoreBound(previous) => *bound = previous,
            }
        }
    }

    fn collect_expression<'a>(
        &mut self,
        expression: &'a TypedExpr,
        bound: &mut HashSet<EcoString>,
        pending: &mut Vec<Visit<'a>>,
    ) {
        match expression {
            TypedExpr::Int { .. }
            | TypedExpr::Float { .. }
            | TypedExpr::String { .. }
            | TypedExpr::Invalid { .. } => {}
            TypedExpr::Var {
                name, constructor, ..
            } => {
                if matches!(
                    constructor.variant,
                    ValueConstructorVariant::LocalVariable { .. }
                ) {
                    self.record(name, bound);
                }
            }
            TypedExpr::Block { statements, .. } => {
                pending.push(Visit::RestoreBound(bound.clone()));
                pending.push(Visit::Statements(statements.as_slice()));
            }
            TypedExpr::Pipeline {
                first_value,
                assignments,
                finally,
                ..
            } => {
                pending.push(Visit::RestoreBound(bound.clone()));
                pending.push(Visit::Expression(finally));
                pending.extend(
                    assignments
                        .iter()
                        .rev()
                        .map(|(assignment, _)| Visit::PipelineAssignment(assignment)),
                );
                pending.push(Visit::PipelineAssignment(first_value));
            }
            TypedExpr::Fn {
                arguments, body, ..
            } => {
                pending.push(Visit::RestoreBound(bound.clone()));
                for argument in arguments {
                    bound.extend(argument.get_variable_name().cloned());
                }
                pending.push(Visit::Statements(body.as_slice()));
            }
            TypedExpr::Call { fun, arguments, .. } => {
                pending.extend(
                    arguments
                        .iter()
                        .rev()
                        .map(|argument| Visit::Expression(&argument.value)),
                );
                pending.push(Visit::Expression(fun));
            }
            TypedExpr::BinOp { left, right, .. } => {
                pending.push(Visit::Expression(right));
                pending.push(Visit::Expression(left));
            }
            TypedExpr::Case {
                subjects, clauses, ..
            } => {
                pending.extend(clauses.iter().rev().map(Visit::Clause));
                pending.extend(subjects.iter().rev().map(Visit::Expression));
            }
            TypedExpr::NegateBool { value, .. } | TypedExpr::NegateInt { value, .. } => {
                pending.push(Visit::Expression(value));
            }
            TypedExpr::Tuple { elements, .. } => {
                pending.extend(elements.iter().rev().map(Visit::Expression));
            }
            TypedExpr::TupleIndex { tuple, .. } => {
                pending.push(Visit::Expression(tuple));
            }
            TypedExpr::List { elements, tail, .. } => {
                if let Some(tail) = tail {
                    pending.push(Visit::Expression(tail));
                }
                pending.extend(elements.iter().rev().map(Visit::Expression));
            }
            TypedExpr::Todo { message, .. } | TypedExpr::Panic { message, .. } => {
                if let Some(message) = message {
                    pending.push(Visit::Expression(message));
                }
            }
            TypedExpr::BitArray { segments, .. } => {
                for segment in segments.iter().rev() {
                    for option in segment.options.iter().rev() {
                        if let gleam_compiler_core::ast::BitArrayOption::Size { value, .. } = option
                        {
                            pending.push(Visit::Expression(value));
                        }
                    }
                    pending.push(Visit::Expression(&segment.value));
                }
            }
            TypedExpr::RecordAccess { record, .. } => pending.push(Visit::Expression(record)),
            TypedExpr::RecordUpdate {
                updated_record,
                arguments,
                ..
            } => {
                for argument in arguments.iter().rev() {
                    if argument.implicit.is_none() {
                        pending.push(Visit::Expression(&argument.value));
                    }
                }
                pending.push(Visit::Expression(updated_record));
            }
            TypedExpr::Echo {
                expression,
                message,
                ..
            } => {
                if let Some(message) = message {
                    pending.push(Visit::Expression(message));
                }
                if let Some(expression) = expression {
                    pending.push(Visit::Expression(expression));
                }
            }
            TypedExpr::PositionalAccess { .. } | TypedExpr::ModuleSelect { .. } => {}
        }
    }

    fn collect_guard<'a>(
        &mut self,
        guard: &'a TypedClauseGuard,
        bound: &HashSet<EcoString>,
        pending: &mut Vec<Visit<'a>>,
    ) {
        match guard {
            ClauseGuard::Var { name, .. } => self.record(name, bound),
            ClauseGuard::Block { value, .. } => pending.push(Visit::Guard(value)),
            ClauseGuard::BinaryOperator { left, right, .. } => {
                pending.push(Visit::Guard(right));
                pending.push(Visit::Guard(left));
            }
            ClauseGuard::Not { expression, .. } => pending.push(Visit::Guard(expression)),
            ClauseGuard::TupleIndex { tuple, .. } => pending.push(Visit::Guard(tuple)),
            ClauseGuard::FieldAccess { container, .. } => pending.push(Visit::Guard(container)),
            ClauseGuard::Constant(_)
            | ClauseGuard::ModuleSelect { .. }
            | ClauseGuard::Invalid { .. } => {}
        }
    }

    fn collect_bit_array_size<'a>(
        &mut self,
        size: &'a BitArraySize<Arc<Type>>,
        bound: &HashSet<EcoString>,
        pending: &mut Vec<Visit<'a>>,
    ) {
        match size {
            BitArraySize::Variable { name, .. } => self.record(name, bound),
            BitArraySize::BinaryOperator { left, right, .. } => {
                pending.push(Visit::BitArraySize(right));
                pending.push(Visit::BitArraySize(left));
            }
            BitArraySize::Block { inner, .. } => pending.push(Visit::BitArraySize(inner)),
            BitArraySize::Int { .. } => {}
        }
    }

    fn record(&mut self, name: &EcoString, bound: &HashSet<EcoString>) {
        if bound.contains(name) {
            return;
        }
        if self.seen.insert(name.clone()) {
            self.names.push(name.clone());
        }
    }
}

fn schedule_statement<'a>(statement: &'a TypedStatement, pending: &mut Vec<Visit<'a>>) {
    match statement {
        Statement::Expression(expression) => pending.push(Visit::Expression(expression)),
        Statement::Assignment(assignment) => {
            pending.push(Visit::Pattern(&assignment.pattern));
            if let AssignmentKind::Assert {
                message: Some(message),
                ..
            } = &assignment.kind
            {
                pending.push(Visit::Expression(message));
            }
            pending.push(Visit::Expression(&assignment.value));
        }
        Statement::Use(use_) => pending.push(Visit::Expression(&use_.call)),
        Statement::Assert(assert) => {
            if let Some(message) = &assert.message {
                pending.push(Visit::Expression(message));
            }
            pending.push(Visit::Expression(&assert.value));
        }
    }
}

fn schedule_pattern<'a>(
    pattern: &'a Pattern<Arc<Type>>,
    bound: &mut HashSet<EcoString>,
    pending: &mut Vec<Visit<'a>>,
) {
    match pattern {
        Pattern::Variable { name, .. } => {
            bound.insert(name.clone());
        }
        Pattern::Assign { name, pattern, .. } => {
            pending.push(Visit::Bind(name));
            pending.push(Visit::Pattern(pattern));
        }
        Pattern::Tuple { elements, .. } => {
            pending.extend(elements.iter().rev().map(Visit::Pattern));
        }
        Pattern::List { elements, tail, .. } => {
            if let Some(tail) = tail {
                pending.push(Visit::Pattern(&tail.pattern));
            }
            pending.extend(elements.iter().rev().map(Visit::Pattern));
        }
        Pattern::BitArray { segments, .. } => {
            for segment in segments.iter().rev() {
                pending.push(Visit::Pattern(&segment.value));
                if let Some(Pattern::BitArraySize(size)) = segment.size() {
                    pending.push(Visit::BitArraySize(size));
                }
            }
        }
        Pattern::BitArraySize(size) => pending.push(Visit::BitArraySize(size)),
        Pattern::StringPrefix {
            left_side_assignment,
            right_side_assignment,
            ..
        } => {
            if let Some((name, _)) = left_side_assignment {
                bound.insert(name.clone());
            }
            if let gleam_compiler_core::ast::AssignName::Variable(name) = right_side_assignment {
                bound.insert(name.clone());
            }
        }
        Pattern::Constructor { arguments, .. } => {
            pending.extend(
                arguments
                    .iter()
                    .rev()
                    .map(|argument| Visit::Pattern(&argument.value)),
            );
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Discard { .. }
        | Pattern::Invalid { .. } => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::support::{compile, dummy_span};
    use gleam_compiler_core::ast::{
        AssignName, BitArraySize, ClauseGuard, Constant, Pattern, Publicity, Statement, TypedExpr,
    };
    use gleam_compiler_core::type_::error::VariableOrigin;
    use gleam_compiler_core::type_::{
        self, Deprecation, ValueConstructor, ValueConstructorVariant,
    };
    use vec1::Vec1;

    #[test]
    fn anonymous_free_variables_preserve_descendant_order_and_sibling_scopes() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let first = 1
  let second = 2
  let third = 3
  fn(argument) {
    let nested = fn(second) { fn() { third + second + first + argument } }
    let local = {
      let third = 0
      fn() { third }
    }
    #(nested(0)(), local(), second, first)
  }
  Nil
}
"#
            ),
            ["third", "first", "second"],
        );
    }

    #[test]
    fn anonymous_free_variables_include_use_callback_call() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
fn with_value(value: Int, continue: fn(Int) -> Int) {
  continue(value)
}

pub fn main() {
  let use_value = 1
  fn() {
    use captured <- with_value(use_value)
    captured
  }
  1
}
"#,
            ),
            vec!["use_value".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_supported_expression_shapes() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let block_value = 1
  let case_subject = True
  let pattern_source = 2
  let case_value = 3
  let repeated_value = 4
  let negate_int_value = 4
  let negate_bool_value = True
  let tuple_value = #(5, 6)
  let list_element_value = 7
  let list_tail_value = [7]
  let panic_message = "boom"
  let todo_message = "later"
  let assert_condition = True
  let assert_message = "failed"
  let plain_assert_condition = True
  fn() {
    {
      block_value
    }
    repeated_value
    repeated_value
    case case_subject {
      True -> {
        let branch_local = pattern_source
        branch_local
      }
      False -> case_value
    }
    case !negate_bool_value {
      True -> 0
      False -> 1
    }
    -negate_int_value
    tuple_value.0
    [list_element_value]
    [0, ..list_tail_value]
    panic as panic_message
    todo as todo_message
    assert assert_condition as assert_message
    assert plain_assert_condition
  }
  1
}
"#,
            ),
            vec![
                "block_value".to_string(),
                "repeated_value".to_string(),
                "case_subject".to_string(),
                "pattern_source".to_string(),
                "case_value".to_string(),
                "negate_bool_value".to_string(),
                "negate_int_value".to_string(),
                "tuple_value".to_string(),
                "list_element_value".to_string(),
                "list_tail_value".to_string(),
                "panic_message".to_string(),
                "todo_message".to_string(),
                "assert_condition".to_string(),
                "assert_message".to_string(),
                "plain_assert_condition".to_string(),
            ],
        );
    }

    #[test]
    fn anonymous_free_variables_treat_string_prefix_pattern_names_as_bound() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  fn() {
    case "Hello, Geam" {
      "Hello, " as prefix <> name -> prefix <> name
      _ -> ""
    }
  }
  1
}
"#,
            ),
            Vec::<String>::new(),
        );
    }

    #[test]
    fn collect_pattern_records_string_prefix_alias_and_suffix_names() {
        let mut bound = std::collections::HashSet::new();
        let mut free = super::FreeVariables::new();

        free.collect(
            super::Visit::Pattern(&gleam_compiler_core::ast::Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: Some(("prefix".into(), dummy_span())),
                right_location: dummy_span(),
                left_side_string: "Hello, ".into(),
                right_side_assignment: AssignName::Variable("name".into()),
            }),
            &mut bound,
        );

        assert_eq!(
            bound,
            std::collections::HashSet::from(["prefix".into(), "name".into()]),
        );
        assert_eq!(free.names, Vec::<String>::new());
    }

    #[test]
    fn collect_pattern_ignores_discard_string_prefix_names() {
        let mut bound = std::collections::HashSet::new();
        let mut free = super::FreeVariables::new();

        free.collect(
            super::Visit::Pattern(&gleam_compiler_core::ast::Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: None,
                right_location: dummy_span(),
                left_side_string: "Hello, ".into(),
                right_side_assignment: AssignName::Discard("_rest".into()),
            }),
            &mut bound,
        );

        assert_eq!(bound, std::collections::HashSet::new());
        assert_eq!(free.names, Vec::<String>::new());
    }

    #[test]
    fn anonymous_free_variables_include_case_guard_only_outer_local() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let threshold = 40
  fn() {
    case 41 {
      value if value > threshold -> value
      _ -> 0
    }
  }
  1
}
"#,
            ),
            vec!["threshold".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_bit_array_segment_value_and_size() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let value = 1
  let size = 8
  fn() { <<value:int, value:size(size)>> }
  1
}
"#,
            ),
            vec!["value".to_string(), "size".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_record_access_source() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
type Boxed {
  Boxed(value: Int)
}

pub fn main() {
  let boxed = Boxed(1)
  fn() { boxed.value }
  1
}
"#,
            ),
            vec!["boxed".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_record_update_base_and_explicit_arguments_once() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  let age = 31
  fn() { Person(..person, age: age) }
  1
}
"#,
            ),
            vec!["person".to_string(), "age".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_echo_value_then_message() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let value = 1
  let message = "selected"
  fn() { echo value as message }
  1
}
"#,
            ),
            vec!["value".to_string(), "message".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_pipeline_echo_value_then_message() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let value = 1
  let message = "selected"
  fn() { value |> echo as message }
  1
}
"#,
            ),
            vec!["value".to_string(), "message".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_accept_missing_echo_operands() {
        let body = Vec1::new(Statement::Expression(TypedExpr::Echo {
            location: dummy_span(),
            expression: None,
            message: None,
            type_: type_::int(),
        }));

        assert!(super::anonymous_free_variables(&[], &body).is_empty());
    }

    #[test]
    fn anonymous_free_variables_distinguish_outer_and_prior_bit_array_pattern_sizes() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let outer_size = 8
  fn(bits) {
    case bits {
      <<size, outer:size(outer_size), inner:size(size)>> -> outer + inner
      _ -> 0
    }
  }
  1
}
"#,
            ),
            vec!["outer_size".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_visit_bit_array_alternatives_and_size_arithmetic() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let outer_size = 8
  fn(bits) {
    case bits {
      <<size, value:size(size)>> | <<size, value:size({ outer_size + 0 })>> -> value
      _ -> 0
    }
  }
  1
}
"#,
            ),
            vec!["outer_size".to_string()],
        );
    }

    #[test]
    fn direct_bit_array_size_pattern_collects_its_variable() {
        let pattern = Pattern::BitArraySize(BitArraySize::Variable {
            location: dummy_span(),
            name: "size".into(),
            constructor: None,
            type_: type_::int(),
        });
        let mut free = super::FreeVariables::new();

        free.collect(
            super::Visit::Pattern(&pattern),
            &mut std::collections::HashSet::new(),
        );

        assert_eq!(free.names, vec!["size".to_string()]);
    }

    #[test]
    fn collect_clause_guard_records_supported_guard_shapes() {
        let mut bound = std::collections::HashSet::new();
        bound.insert("bound".into());
        let mut free = super::FreeVariables::new();
        let guard = ClauseGuard::Block {
            location: dummy_span(),
            value: Box::new(ClauseGuard::BinaryOperator {
                location: dummy_span(),
                operator: gleam_compiler_core::ast::BinOp::And,
                operator_start: 0,
                left: Box::new(ClauseGuard::Not {
                    location: dummy_span(),
                    expression: Box::new(guard_var("outer_bool", type_::bool())),
                }),
                right: Box::new(ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::int(),
                    tuple: Box::new(guard_var("outer_tuple", type_::tuple(vec![type_::int()]))),
                }),
            }),
        };
        free.collect(super::Visit::Guard(&guard), &mut bound);
        let field_guard = ClauseGuard::FieldAccess {
            label_location: dummy_span(),
            index: Some(0),
            label: "field".into(),
            type_: type_::int(),
            container: Box::new(guard_var("outer_record", type_::int())),
        };
        free.collect(super::Visit::Guard(&field_guard), &mut bound);
        free.collect(
            super::Visit::Guard(&guard_var("bound", type_::int())),
            &mut bound,
        );
        free.collect(
            super::Visit::Guard(&ClauseGuard::Constant(Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: 1.into(),
            })),
            &mut bound,
        );
        free.collect(
            super::Visit::Guard(&ClauseGuard::ModuleSelect {
                location: dummy_span(),
                field_start: 0,
                definition_location: dummy_span(),
                type_: type_::int(),
                label: "answer".into(),
                module_name: "main".into(),
                module_alias: "main".into(),
                literal: guard_constant_literal(),
            }),
            &mut bound,
        );
        free.collect(
            super::Visit::Guard(&ClauseGuard::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            }),
            &mut bound,
        );

        assert_eq!(
            free.names,
            vec![
                "outer_bool".to_string(),
                "outer_tuple".to_string(),
                "outer_record".to_string(),
            ],
        );
    }

    #[test]
    fn anonymous_free_variables_treat_tuple_alias_assignment_names_as_bound() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let captured = 1
  fn() {
    let #(one, #(two, _) as inner) as pair = #(captured, #(2, 3))
    one + two + inner.0 + pair.0
  }
  1
}
"#,
            ),
            vec!["captured".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_treat_let_assert_list_names_as_bound() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let captured = [1, 2]
  fn() {
    let assert [first, ..rest] = captured
    first == 1 && rest == [2]
  }
  1
}
"#,
            ),
            vec!["captured".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_treat_fixed_let_assert_list_names_as_bound() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let captured = [1]
  fn() {
    let assert [first] = captured
    first
  }
  1
}
"#,
            ),
            vec!["captured".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_let_assert_message_expression() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub fn main() {
  let values = [1]
  let message = "missing"
  fn() {
    let assert [first] = values as message
    first
  }
  1
}
"#,
            ),
            vec!["values".to_string(), "message".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_treat_nested_let_assert_pattern_names_as_bound() {
        assert_eq!(
            anonymous_function_free_variables(
                r#"
pub type Payload {
  Payload(Int)
}

pub fn main() {
  let subject = #([Payload(1)], <<2>>, "prefix")
  let message = "missing"
  fn() {
    let assert #([Payload(value)], <<bit>>, "pre" <> suffix) = subject as message
    #(value, bit, suffix)
  }
  1
}
"#,
            ),
            vec!["subject".to_string(), "message".to_string()],
        );
    }

    #[test]
    fn anonymous_free_variables_include_synthetic_negate_int() {
        let body = Vec1::new(Statement::Expression(TypedExpr::NegateInt {
            location: dummy_span(),
            value: Box::new(typed_local_int_variable("negate_int_value")),
        }));

        let mut names = Vec::new();
        for name in super::anonymous_free_variables(&[], &body) {
            names.push(name.to_string());
        }

        assert_eq!(names, vec!["negate_int_value".to_string()]);
    }

    fn guard_var(
        name: impl Into<ecow::EcoString>,
        type_: std::sync::Arc<gleam_compiler_core::type_::Type>,
    ) -> ClauseGuard<std::sync::Arc<gleam_compiler_core::type_::Type>> {
        ClauseGuard::Var {
            location: dummy_span(),
            type_,
            name: name.into(),
            definition_location: dummy_span(),
            origin: VariableOrigin::generated(),
        }
    }

    fn guard_constant_literal() -> Constant<std::sync::Arc<gleam_compiler_core::type_::Type>> {
        Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        }
    }

    fn anonymous_function_free_variables(src: &str) -> Vec<String> {
        let module = compile(src);
        let function = module
            .definitions
            .functions
            .last()
            .expect("main function should exist");
        let (arguments, body) = function
            .body
            .iter()
            .find_map(|statement| match statement {
                Statement::Expression(TypedExpr::Fn {
                    arguments, body, ..
                }) => Some((arguments, body)),
                _ => None,
            })
            .expect("expected anonymous function expression statement");

        let mut names = Vec::new();
        for name in super::anonymous_free_variables(arguments, body) {
            names.push(name.to_string());
        }
        names
    }

    fn typed_local_int_variable(name: &str) -> TypedExpr {
        TypedExpr::Var {
            location: dummy_span(),
            name: name.into(),
            constructor: ValueConstructor {
                publicity: Publicity::Private,
                deprecation: Deprecation::NotDeprecated,
                type_: type_::int(),
                variant: ValueConstructorVariant::LocalVariable {
                    location: dummy_span(),
                    origin: VariableOrigin::generated(),
                },
            },
        }
    }
}
