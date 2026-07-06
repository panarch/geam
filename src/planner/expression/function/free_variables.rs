use ecow::EcoString;
use gleam_core::ast::{
    AssignmentKind, Pattern, Statement, TypedArg, TypedExpr, TypedPipelineAssignment,
    TypedStatement,
};
use gleam_core::type_::{Type, ValueConstructorVariant};
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
    collect_statements(body.as_slice(), &mut bound, &mut free);
    free.names
}

struct FreeVariables {
    names: Vec<EcoString>,
    seen: HashSet<EcoString>,
}

impl FreeVariables {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            seen: HashSet::new(),
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

fn collect_statements(
    statements: &[TypedStatement],
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    for statement in statements {
        collect_statement(statement, bound, free);
    }
}

fn collect_statement(
    statement: &TypedStatement,
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    match statement {
        Statement::Expression(expression) => collect_expr(expression, bound, free),
        Statement::Assignment(assignment) => {
            collect_expr(&assignment.value, bound, free);
            if let AssignmentKind::Assert {
                message: Some(message),
                ..
            } = &assignment.kind
            {
                collect_expr(message, bound, free);
            }
            collect_variable_pattern_bound_name(&assignment.pattern, bound);
        }
        Statement::Use(use_) => {
            collect_expr(&use_.call, bound, free);
        }
        Statement::Assert(assert) => {
            collect_expr(&assert.value, bound, free);
            if let Some(message) = &assert.message {
                collect_expr(message, bound, free);
            }
        }
    }
}

fn collect_expr(expression: &TypedExpr, bound: &mut HashSet<EcoString>, free: &mut FreeVariables) {
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
                free.record(name, bound);
            }
        }
        TypedExpr::Block { statements, .. } => {
            let mut block_bound = bound.clone();
            collect_statements(statements.as_slice(), &mut block_bound, free);
        }
        TypedExpr::Pipeline {
            first_value,
            assignments,
            finally,
            ..
        } => {
            let mut pipeline_bound = bound.clone();
            collect_pipeline_assignment(first_value, &mut pipeline_bound, free);
            for (assignment, _) in assignments {
                collect_pipeline_assignment(assignment, &mut pipeline_bound, free);
            }
            collect_expr(finally, &mut pipeline_bound, free);
        }
        TypedExpr::Fn {
            arguments, body, ..
        } => {
            for name in anonymous_free_variables(arguments, body) {
                free.record(&name, bound);
            }
        }
        TypedExpr::Call { fun, arguments, .. } => {
            collect_expr(fun, bound, free);
            for argument in arguments {
                collect_expr(&argument.value, bound, free);
            }
        }
        TypedExpr::BinOp { left, right, .. } => {
            collect_expr(left, bound, free);
            collect_expr(right, bound, free);
        }
        TypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                collect_expr(subject, bound, free);
            }
            for clause in clauses {
                let mut branch_bound = bound.clone();
                for pattern in &clause.pattern {
                    collect_variable_pattern_bound_name(pattern, &mut branch_bound);
                }
                collect_expr(&clause.then, &mut branch_bound, free);
            }
        }
        TypedExpr::NegateBool { value, .. } | TypedExpr::NegateInt { value, .. } => {
            collect_expr(value, bound, free);
        }
        TypedExpr::Tuple { elements, .. } => {
            for element in elements {
                collect_expr(element, bound, free);
            }
        }
        TypedExpr::TupleIndex { tuple, .. } => {
            collect_expr(tuple, bound, free);
        }
        TypedExpr::List { elements, tail, .. } => {
            for element in elements {
                collect_expr(element, bound, free);
            }
            if let Some(tail) = tail {
                collect_expr(tail, bound, free);
            }
        }
        TypedExpr::Todo { message, .. } | TypedExpr::Panic { message, .. } => {
            if let Some(message) = message {
                collect_expr(message, bound, free);
            }
        }
        TypedExpr::RecordAccess { .. }
        | TypedExpr::PositionalAccess { .. }
        | TypedExpr::Echo { .. }
        | TypedExpr::BitArray { .. }
        | TypedExpr::RecordUpdate { .. }
        | TypedExpr::ModuleSelect { .. } => {}
    }
}

fn collect_pipeline_assignment(
    assignment: &TypedPipelineAssignment,
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    collect_expr(&assignment.value, bound, free);
    bound.insert(assignment.name.clone());
}

fn collect_variable_pattern_bound_name(
    pattern: &Pattern<Arc<Type>>,
    bound: &mut HashSet<EcoString>,
) {
    match pattern {
        Pattern::Variable { name, .. } => {
            bound.insert(name.clone());
        }
        Pattern::Assign { name, pattern, .. } => {
            collect_variable_pattern_bound_name(pattern, bound);
            bound.insert(name.clone());
        }
        Pattern::Tuple { elements, .. } => {
            for element in elements {
                collect_variable_pattern_bound_name(element, bound);
            }
        }
        Pattern::List { elements, tail, .. } => {
            for element in elements {
                collect_variable_pattern_bound_name(element, bound);
            }
            if let Some(tail) = tail {
                collect_variable_pattern_bound_name(&tail.pattern, bound);
            }
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Constructor { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Discard { .. }
        | Pattern::Invalid { .. } => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::support::{compile, dummy_span};
    use gleam_core::ast::{Publicity, Statement, TypedExpr};
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{self, Deprecation, ValueConstructor, ValueConstructorVariant};
    use vec1::Vec1;

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
