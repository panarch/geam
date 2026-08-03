mod coverage;
mod guard;
mod subject;

use crate::plan::Expr;
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
use gleam_core::ast::{TypedClause, TypedExpr};
use gleam_core::exhaustiveness::CompiledCase;
use gleam_core::type_::Type;
use std::sync::Arc;

#[cfg(test)]
use gleam_core::ast::{Statement, TypedModule, TypedStatement};

pub(super) fn plan_case(
    type_: Arc<Type>,
    subjects: Vec<TypedExpr>,
    clauses: Vec<TypedClause>,
    compiled_case: CompiledCase,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if clauses.is_empty() {
        return Err(invalid_case_shape(InvalidCaseShapeReason::EmptyClauses));
    }

    let subject_count = subjects.len();
    let mut subjects = subjects.into_iter();
    let Some(subject) = subjects.next() else {
        return Err(invalid_case_shape(InvalidCaseShapeReason::EmptySubjects));
    };
    let coverage = coverage::CaseCoverage::new(&compiled_case, subject_count, &clauses)?;
    if subjects.len() == 0 {
        return subject::plan(type_, subject, clauses, coverage, context);
    }

    let mut all_subjects = Vec::with_capacity(1 + subjects.len());
    all_subjects.push(subject);
    all_subjects.extend(subjects);
    subject::plan_multi(type_, all_subjects, clauses, coverage, context)
}

pub(super) fn invalid_case_shape(reason: InvalidCaseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CaseShape { reason },
    }
}

#[cfg(test)]
pub(super) fn compile_bool_case_module() -> TypedModule {
    crate::planner::support::compile(
        r#"
pub fn main() {
  case True {
    True -> 1
    False -> 0
  }
}
"#,
    )
}

#[cfg(test)]
pub(super) fn expect_case_statement_mut(
    statement: &mut TypedStatement,
) -> (
    &mut std::sync::Arc<Type>,
    &mut Vec<TypedExpr>,
    &mut Vec<TypedClause>,
) {
    let Statement::Expression(TypedExpr::Case {
        type_,
        subjects,
        clauses,
        ..
    }) = statement
    else {
        panic!("expected case expression statement");
    };
    (type_, subjects, clauses)
}

#[cfg(test)]
pub(super) fn expect_assignment_case_statement_mut(
    statement: &mut TypedStatement,
) -> (
    &mut std::sync::Arc<Type>,
    &mut Vec<TypedExpr>,
    &mut Vec<TypedClause>,
    &mut CompiledCase,
) {
    let Statement::Assignment(assignment) = statement else {
        panic!("expected case assignment statement");
    };
    let TypedExpr::Case {
        type_,
        subjects,
        clauses,
        compiled_case,
        ..
    } = &mut assignment.value
    else {
        panic!("expected case assignment value");
    };
    (type_, subjects, clauses, compiled_case)
}

#[cfg(test)]
pub(super) fn expect_expression_statement(statement: &TypedStatement) -> &TypedExpr {
    let Statement::Expression(expression) = statement else {
        panic!("expected expression statement");
    };
    expression
}

#[cfg(test)]
mod tests {
    use crate::planner::plan_module;
    use crate::planner::support::compile_minimal_module;
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};

    #[test]
    fn reject_margin_case_shapes() {
        let mut empty_subjects = super::compile_bool_case_module();
        let (_, subjects, _) =
            super::expect_case_statement_mut(&mut empty_subjects.definitions.functions[0].body[0]);
        subjects.clear();
        assert_eq!(
            plan_module(empty_subjects),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::EmptySubjects,
                },
            }),
        );

        let mut empty_clauses = super::compile_bool_case_module();
        let (_, _, clauses) =
            super::expect_case_statement_mut(&mut empty_clauses.definitions.functions[0].body[0]);
        clauses.clear();
        assert_eq!(
            plan_module(empty_clauses),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::EmptyClauses,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected case expression statement")]
    fn expect_case_statement_mut_panics_on_int() {
        let mut module = compile_minimal_module();

        super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected case assignment statement")]
    fn expect_assignment_case_statement_mut_panics_on_expression() {
        let mut module = compile_minimal_module();

        super::expect_assignment_case_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected case assignment value")]
    fn expect_assignment_case_statement_mut_panics_on_int_assignment() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        super::expect_assignment_case_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn expect_expression_statement_panics_on_assignment() {
        let module = crate::planner::support::compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        super::expect_expression_statement(&module.definitions.functions[0].body[0]);
    }
}
