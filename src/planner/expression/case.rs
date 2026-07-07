mod bool_subject;
mod float_subject;
mod int_subject;
mod string_subject;

use crate::plan::{
    BoolExpr, BoolLocalId, Expr, FloatExpr, FloatLocalId, IntExpr, IntLocalId, Step, StringExpr,
    StringLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

#[cfg(test)]
use gleam_core::ast::{Statement, TypedModule, TypedStatement};

pub(super) fn plan_case(
    type_: Arc<Type>,
    subjects: Vec<TypedExpr>,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = single_case_subject(subjects)?;
    if clauses.is_empty() {
        return Err(invalid_case_shape(InvalidCaseShapeReason::EmptyClauses));
    }

    if subject.type_().is_bool() {
        return bool_subject::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_int() {
        return int_subject::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_string() {
        return string_subject::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_float() {
        return float_subject::plan(type_, subject, clauses, context);
    }

    Err(unsupported_case(
        UnsupportedCaseReason::UnsupportedSubjectType,
    ))
}

pub(super) fn validate_clause_shape(clause: &TypedClause) -> Result<(), PlanError> {
    if !clause.alternative_patterns.is_empty() {
        return Err(unsupported_case(UnsupportedCaseReason::AlternativePatterns));
    }
    if clause.guard.is_some() {
        return Err(unsupported_case(UnsupportedCaseReason::Guard));
    }

    Ok(())
}

fn single_case_subject(subjects: Vec<TypedExpr>) -> Result<TypedExpr, PlanError> {
    let mut subjects = subjects.into_iter();
    let subject = subjects
        .next()
        .ok_or(invalid_case_shape(InvalidCaseShapeReason::EmptySubjects))?;
    if subjects.next().is_some() {
        return Err(unsupported_case(UnsupportedCaseReason::MultipleSubjects));
    }

    Ok(subject)
}

pub(super) fn single_case_pattern(
    patterns: Vec<Pattern<Arc<Type>>>,
) -> Result<Pattern<Arc<Type>>, PlanError> {
    let mut patterns = patterns.into_iter();
    let pattern = patterns.next().ok_or(invalid_case_shape(
        InvalidCaseShapeReason::PatternSubjectCountMismatch,
    ))?;
    if patterns.next().is_some() {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternSubjectCountMismatch,
        ));
    }

    Ok(pattern)
}

pub(super) fn validate_case_branch_type(case_type: &Type, branch: &Expr) -> Result<(), PlanError> {
    if ValueType::from_gleam(case_type) == Some(branch.value_type()) {
        return Ok(());
    }

    Err(invalid_case_shape(
        InvalidCaseShapeReason::BranchReturnTypeMismatch,
    ))
}

pub(super) fn case_return_type(case_type: &Type) -> Result<ValueType, PlanError> {
    ValueType::from_gleam(case_type)
        .ok_or_else(|| invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch))
}

pub(super) fn plan_case_branch(
    case_type: &Type,
    return_type: &ValueType,
    then: TypedExpr,
    variable_binding: Option<(EcoString, Expr)>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let steps = variable_binding
            .map(|(name, value)| vec![plan_variable_runtime_step(name, value, context)])
            .unwrap_or_default();
        let branch =
            super::plan_expr_with_expected_source_stop_type(then, return_type.clone(), context)?;
        validate_case_branch_type(case_type, &branch)?;

        if steps.is_empty() {
            Ok(branch)
        } else {
            Ok(super::block::block_expr(steps, branch))
        }
    })
}

pub(super) fn bind_int_case_subject(
    subject: IntExpr,
    context: &mut PlanContext<'_>,
) -> (Step, IntExpr) {
    let local = context.define_internal_int_local();
    let name = internal_int_case_subject_name(local);
    (
        Step::let_int(local, name.clone(), subject),
        IntExpr::local_get(local, name),
    )
}

pub(super) fn bind_string_case_subject(
    subject: StringExpr,
    context: &mut PlanContext<'_>,
) -> (Step, StringExpr) {
    let local = context.define_internal_string_local();
    let name = internal_string_case_subject_name(local);
    (
        Step::let_string(local, name.clone(), subject),
        StringExpr::local_get(local, name),
    )
}

pub(super) fn bind_float_case_subject(
    subject: FloatExpr,
    context: &mut PlanContext<'_>,
) -> (Step, FloatExpr) {
    let local = context.define_internal_float_local();
    let name = internal_float_case_subject_name(local);
    (
        Step::let_float(local, name.clone(), subject),
        FloatExpr::local_get(local, name),
    )
}

pub(super) fn bind_bool_case_subject(
    subject: BoolExpr,
    context: &mut PlanContext<'_>,
) -> (Step, BoolExpr) {
    let local = context.define_internal_bool_local();
    let name = internal_bool_case_subject_name(local);
    (
        Step::let_bool(local, name.clone(), subject),
        BoolExpr::local_get(local, name),
    )
}

pub(super) fn case_subject_block(step: Step, case: Expr) -> Expr {
    super::block::block_expr(vec![step], case)
}

pub(super) fn unsupported_case(reason: UnsupportedCaseReason) -> PlanError {
    PlanError::UnsupportedCase { reason }
}

pub(super) fn invalid_case_shape(reason: InvalidCaseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CaseShape { reason },
    }
}

fn internal_int_case_subject_name(local: IntLocalId) -> EcoString {
    format!("<case:int:{}>", local.0).into()
}

fn internal_string_case_subject_name(local: StringLocalId) -> EcoString {
    format!("<case:string:{}>", local.0).into()
}

fn internal_float_case_subject_name(local: FloatLocalId) -> EcoString {
    format!("<case:float:{}>", local.0).into()
}

fn internal_bool_case_subject_name(local: BoolLocalId) -> EcoString {
    format!("<case:bool:{}>", local.0).into()
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
) {
    let Statement::Assignment(assignment) = statement else {
        panic!("expected case assignment statement");
    };
    let TypedExpr::Case {
        type_,
        subjects,
        clauses,
        ..
    } = &mut assignment.value
    else {
        panic!("expected case assignment value");
    };
    (type_, subjects, clauses)
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
    use crate::planner::support::{compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;

    #[test]
    fn reject_profile_case_expressions() {
        let cases = [
            (
                r#"
pub fn main() {
  case True, False {
    True, False -> 1
    _, _ -> 0
  }
}
"#,
                UnsupportedCaseReason::MultipleSubjects,
            ),
            (
                r#"
pub fn main() {
  case True {
    True if True -> 1
    False -> 0
    _ -> 2
  }
}
"#,
                UnsupportedCaseReason::Guard,
            ),
            (
                r#"pub fn main() { case True { True | False -> 1 } }"#,
                UnsupportedCaseReason::AlternativePatterns,
            ),
            (
                r#"pub fn main() { case #(1, 2) { _ -> 1 } }"#,
                UnsupportedCaseReason::UnsupportedSubjectType,
            ),
            (
                r#"pub fn main() { case [1, 2] { _ -> 1 } }"#,
                UnsupportedCaseReason::UnsupportedSubjectType,
            ),
        ];

        for (src, reason) in cases {
            assert_eq!(
                expect_plan_error(src),
                PlanError::UnsupportedCase { reason },
            );
        }
    }

    #[test]
    fn reject_profile_unreachable_case_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case True {
    _ -> 1
    True -> echo 2
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    _ -> 1
    1 -> echo 2
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

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

        let mut empty_pattern = super::compile_bool_case_module();
        let (_, _, clauses) =
            super::expect_case_statement_mut(&mut empty_pattern.definitions.functions[0].body[0]);
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut extra_pattern = super::compile_bool_case_module();
        let (_, _, clauses) =
            super::expect_case_statement_mut(&mut extra_pattern.definitions.functions[0].body[0]);
        let pattern = clauses[0].pattern[0].clone();
        clauses[0].pattern.push(pattern);
        assert_eq!(
            plan_module(extra_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut case_type_mismatch = super::compile_bool_case_module();
        let (case_type, _, _) = super::expect_case_statement_mut(
            &mut case_type_mismatch.definitions.functions[0].body[0],
        );
        *case_type = type_::bool();
        assert_eq!(
            plan_module(case_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut branch_type_mismatch = super::compile_bool_case_module();
        let (case_type, _, clauses) = super::expect_case_statement_mut(
            &mut branch_type_mismatch.definitions.functions[0].body[0],
        );
        *case_type = type_::string();
        clauses[0].then = TypedExpr::String {
            location: dummy_span(),
            type_: type_::string(),
            value: "bad".into(),
        };
        assert_eq!(
            plan_module(branch_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
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
