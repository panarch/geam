mod bool_subject;
mod float_subject;
mod guard;
mod int_subject;
mod string_subject;

use crate::plan::{
    BoolCaseBranches, BoolExpr, BoolLocalId, Expr, ExprKind, FloatExpr, FloatLocalId,
    FunctionExprKind, IntExpr, IntLocalId, Step, StringExpr, StringLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedClause, TypedClauseGuard, TypedExpr};
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

pub(super) fn bool_case_expr(
    subject: BoolExpr,
    true_: Expr,
    false_: Expr,
) -> Result<Expr, PlanError> {
    let branches = match (true_.into_kind(), false_.into_kind()) {
        (ExprKind::Int(true_), ExprKind::Int(false_)) => BoolCaseBranches::Int { true_, false_ },
        (ExprKind::String(true_), ExprKind::String(false_)) => {
            BoolCaseBranches::String { true_, false_ }
        }
        (ExprKind::Float(true_), ExprKind::Float(false_)) => {
            BoolCaseBranches::Float { true_, false_ }
        }
        (ExprKind::Bool(true_), ExprKind::Bool(false_)) => BoolCaseBranches::Bool { true_, false_ },
        (ExprKind::Nil(true_), ExprKind::Nil(false_)) => BoolCaseBranches::Nil { true_, false_ },
        (ExprKind::Tuple(true_), ExprKind::Tuple(false_)) => {
            BoolCaseBranches::Tuple { true_, false_ }
        }
        (ExprKind::List(true_), ExprKind::List(false_)) => BoolCaseBranches::List { true_, false_ },
        (ExprKind::Function(true_), ExprKind::Function(false_)) => {
            bool_function_case_branches(true_, false_)?
        }
        _ => {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        }
    };

    Ok(Expr::bool_case(subject, branches))
}

fn bool_function_case_branches(
    true_: crate::plan::FunctionExpr,
    false_: crate::plan::FunctionExpr,
) -> Result<BoolCaseBranches, PlanError> {
    Ok(match (true_.into_kind(), false_.into_kind()) {
        (FunctionExprKind::Int(true_), FunctionExprKind::Int(false_)) => {
            BoolCaseBranches::IntFunction { true_, false_ }
        }
        (FunctionExprKind::String(true_), FunctionExprKind::String(false_)) => {
            BoolCaseBranches::StringFunction { true_, false_ }
        }
        (FunctionExprKind::Float(true_), FunctionExprKind::Float(false_)) => {
            BoolCaseBranches::FloatFunction { true_, false_ }
        }
        (FunctionExprKind::Bool(true_), FunctionExprKind::Bool(false_)) => {
            BoolCaseBranches::BoolFunction { true_, false_ }
        }
        (FunctionExprKind::Nil(true_), FunctionExprKind::Nil(false_)) => {
            BoolCaseBranches::NilFunction { true_, false_ }
        }
        (FunctionExprKind::Tuple(true_), FunctionExprKind::Tuple(false_)) => {
            BoolCaseBranches::TupleFunction { true_, false_ }
        }
        (FunctionExprKind::List(true_), FunctionExprKind::List(false_)) => {
            BoolCaseBranches::ListFunction { true_, false_ }
        }
        (FunctionExprKind::Function(true_), FunctionExprKind::Function(false_)) => {
            BoolCaseBranches::FunctionFunction { true_, false_ }
        }
        _ => {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        }
    })
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

#[derive(Clone)]
pub(super) struct OrderedCaseClause {
    pub(super) condition: BoolExpr,
    pub(super) branch: Expr,
    pub(super) is_total: bool,
}

pub(super) struct OrderedCaseClauseInput<'a> {
    pub(super) case_type: &'a Type,
    pub(super) return_type: &'a ValueType,
    pub(super) then: TypedExpr,
    pub(super) variable_binding: Option<(EcoString, Expr)>,
    pub(super) guard: Option<TypedClauseGuard>,
    pub(super) match_condition: BoolExpr,
    pub(super) is_total: bool,
}

pub(super) fn plan_ordered_case_clause(
    input: OrderedCaseClauseInput<'_>,
    context: &mut PlanContext<'_>,
) -> Result<OrderedCaseClause, PlanError> {
    let OrderedCaseClauseInput {
        case_type,
        return_type,
        then,
        variable_binding,
        guard,
        match_condition,
        is_total,
    } = input;

    context.with_local_scope(|context| {
        let binding_step =
            variable_binding.map(|(name, value)| plan_variable_runtime_step(name, value, context));
        let guard_condition = guard
            .map(|guard| guard::plan_bool(guard, context))
            .transpose()?;
        let condition = match guard_condition {
            Some(guard_condition) => BoolExpr::and(match_condition, guard_condition),
            None => match_condition,
        };
        let condition = match &binding_step {
            Some(step) => BoolExpr::block(vec![step.clone()], condition),
            None => condition,
        };

        let branch =
            super::plan_expr_with_expected_source_stop_type(then, return_type.clone(), context)?;
        validate_case_branch_type(case_type, &branch)?;
        let branch = if let Some(step) = binding_step {
            super::block::block_expr(vec![step], branch)
        } else {
            branch
        };

        Ok(OrderedCaseClause {
            condition,
            branch,
            is_total,
        })
    })
}

pub(super) fn ordered_case_expr(clauses: Vec<OrderedCaseClause>) -> Result<Expr, PlanError> {
    let mut reachable_clauses = Vec::new();
    for clause in clauses {
        let is_total = clause.is_total;
        reachable_clauses.push(clause);
        if is_total {
            break;
        }
    }

    let Some(last_clause) = reachable_clauses.pop() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::MissingFallbackPattern,
        ));
    };
    if !last_clause.is_total {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::MissingFallbackPattern,
        ));
    }

    let mut next = last_clause.branch;
    for clause in reachable_clauses.into_iter().rev() {
        next = bool_case_expr(clause.condition, clause.branch, next)?;
    }

    Ok(next)
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
    use crate::plan::{BoolExpr, Expr, IntExpr};
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::plan_module;
    use crate::planner::support::{compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;
    use std::collections::HashMap;

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

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    value if value > 0 -> echo value
    _ -> 0
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
    fn reject_margin_ordered_case_expr_requires_total_fallback() {
        assert_eq!(
            super::ordered_case_expr(Vec::new()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
        assert_eq!(
            super::ordered_case_expr(vec![super::OrderedCaseClause {
                condition: BoolExpr::value(false),
                branch: Expr::int(IntExpr::value(1.into())),
                is_total: false,
            }]),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
    }

    #[test]
    fn ordered_case_expr_preserves_source_ordered_fallthrough_shape() {
        assert_eq!(
            super::ordered_case_expr(vec![super::OrderedCaseClause {
                condition: BoolExpr::value(true),
                branch: Expr::int(IntExpr::value(1.into())),
                is_total: true,
            }]),
            Ok(Expr::int(IntExpr::value(1.into()))),
        );
        assert_eq!(
            super::ordered_case_expr(vec![
                super::OrderedCaseClause {
                    condition: BoolExpr::value(false),
                    branch: Expr::int(IntExpr::value(1.into())),
                    is_total: false,
                },
                super::OrderedCaseClause {
                    condition: BoolExpr::value(true),
                    branch: Expr::int(IntExpr::value(0.into())),
                    is_total: true,
                }
            ]),
            Ok(Expr::int(IntExpr::bool_case(
                BoolExpr::value(false),
                IntExpr::value(1.into()),
                IntExpr::value(0.into()),
            ))),
        );
        assert_eq!(
            super::ordered_case_expr(vec![
                super::OrderedCaseClause {
                    condition: BoolExpr::value(true),
                    branch: Expr::int(IntExpr::value(10.into())),
                    is_total: true,
                },
                super::OrderedCaseClause {
                    condition: BoolExpr::value(true),
                    branch: Expr::int(IntExpr::value(999.into())),
                    is_total: false,
                },
            ]),
            Ok(Expr::int(IntExpr::value(10.into()))),
        );
    }

    #[test]
    fn reject_margin_ordered_case_clause_branch_type_mismatch() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let case_type = type_::string();

        let actual = super::plan_ordered_case_clause(
            super::OrderedCaseClauseInput {
                case_type: case_type.as_ref(),
                return_type: &crate::plan::ValueType::String,
                then: super::super::typed_int_expr(1),
                variable_binding: None,
                guard: None,
                match_condition: BoolExpr::value(true),
                is_total: true,
            },
            &mut context,
        );
        let error = actual
            .map(|_| ())
            .expect_err("branch type mismatch should reject ordered case clause");
        assert_eq!(
            error,
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            },
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
