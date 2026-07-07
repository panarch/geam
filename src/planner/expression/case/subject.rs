mod bool_;
mod float;
mod int;
mod string;

use crate::plan::{
    BoolCaseBranches, BoolExpr, BoolLocalId, Expr, ExprKind, FloatExpr, FloatLocalId,
    FunctionExprKind, IntExpr, IntLocalId, Step, StringExpr, StringLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedClause, TypedClauseGuard, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if subject.type_().is_bool() {
        return bool_::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_int() {
        return int::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_string() {
        return string::plan(type_, subject, clauses, context);
    }
    if subject.type_().is_float() {
        return float::plan(type_, subject, clauses, context);
    }

    Err(super::unsupported_case(
        UnsupportedCaseReason::UnsupportedSubjectType,
    ))
}

fn validate_clause_shape(clause: &TypedClause) -> Result<(), PlanError> {
    if !clause.alternative_patterns.is_empty() {
        return Err(super::unsupported_case(
            UnsupportedCaseReason::AlternativePatterns,
        ));
    }

    Ok(())
}

fn single_case_pattern(patterns: Vec<Pattern<Arc<Type>>>) -> Result<Pattern<Arc<Type>>, PlanError> {
    let mut patterns = patterns.into_iter();
    let pattern = patterns.next().ok_or(super::invalid_case_shape(
        InvalidCaseShapeReason::PatternSubjectCountMismatch,
    ))?;
    if patterns.next().is_some() {
        return Err(super::invalid_case_shape(
            InvalidCaseShapeReason::PatternSubjectCountMismatch,
        ));
    }

    Ok(pattern)
}

fn validate_case_branch_type(case_type: &Type, branch: &Expr) -> Result<(), PlanError> {
    if ValueType::from_gleam(case_type) == Some(branch.value_type()) {
        return Ok(());
    }

    Err(super::invalid_case_shape(
        InvalidCaseShapeReason::BranchReturnTypeMismatch,
    ))
}

fn bool_case_expr(subject: BoolExpr, true_: Expr, false_: Expr) -> Result<Expr, PlanError> {
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
            return Err(super::invalid_case_shape(
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
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        }
    })
}

fn case_return_type(case_type: &Type) -> Result<ValueType, PlanError> {
    ValueType::from_gleam(case_type)
        .ok_or_else(|| super::invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch))
}

fn plan_case_branch(
    case_type: &Type,
    return_type: &ValueType,
    then: TypedExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let steps = plan_branch_binding_steps(branch_bindings, context);
        let branch = super::super::plan_expr_with_expected_source_stop_type(
            then,
            return_type.clone(),
            context,
        )?;
        validate_case_branch_type(case_type, &branch)?;

        if steps.is_empty() {
            Ok(branch)
        } else {
            Ok(super::super::block::block_expr(steps, branch))
        }
    })
}

#[derive(Clone)]
struct OrderedCaseClause {
    condition: BoolExpr,
    branch: Expr,
    is_total: bool,
}

struct OrderedCaseClauseInput<'a> {
    case_type: &'a Type,
    return_type: &'a ValueType,
    then: TypedExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    guard: Option<TypedClauseGuard>,
    match_condition: BoolExpr,
    is_total: bool,
}

fn plan_ordered_case_clause(
    input: OrderedCaseClauseInput<'_>,
    context: &mut PlanContext<'_>,
) -> Result<OrderedCaseClause, PlanError> {
    let OrderedCaseClauseInput {
        case_type,
        return_type,
        then,
        branch_bindings,
        guard,
        match_condition,
        is_total,
    } = input;

    context.with_local_scope(|context| {
        let binding_steps = plan_branch_binding_steps(branch_bindings, context);
        let guard_condition = guard
            .map(|guard| super::guard::plan_bool(guard, context))
            .transpose()?;
        let condition = match guard_condition {
            Some(guard_condition) => BoolExpr::and(match_condition, guard_condition),
            None => match_condition,
        };
        let condition = if binding_steps.is_empty() {
            condition
        } else {
            BoolExpr::block(binding_steps.clone(), condition)
        };

        let branch = super::super::plan_expr_with_expected_source_stop_type(
            then,
            return_type.clone(),
            context,
        )?;
        validate_case_branch_type(case_type, &branch)?;
        let branch = if binding_steps.is_empty() {
            branch
        } else {
            super::super::block::block_expr(binding_steps, branch)
        };

        Ok(OrderedCaseClause {
            condition,
            branch,
            is_total,
        })
    })
}

fn branch_bindings(names: &[EcoString], value: Expr) -> Vec<(EcoString, Expr)> {
    names
        .iter()
        .cloned()
        .map(|name| (name, value.clone()))
        .collect()
}

fn plan_branch_binding_steps(
    bindings: Vec<(EcoString, Expr)>,
    context: &mut PlanContext<'_>,
) -> Vec<Step> {
    bindings
        .into_iter()
        .map(|(name, value)| plan_variable_runtime_step(name, value, context))
        .collect()
}

fn ordered_case_expr(clauses: Vec<OrderedCaseClause>) -> Result<Expr, PlanError> {
    let mut reachable_clauses = Vec::new();
    for clause in clauses {
        let is_total = clause.is_total;
        reachable_clauses.push(clause);
        if is_total {
            break;
        }
    }

    let Some(last_clause) = reachable_clauses.pop() else {
        return Err(super::invalid_case_shape(
            InvalidCaseShapeReason::MissingFallbackPattern,
        ));
    };
    if !last_clause.is_total {
        return Err(super::invalid_case_shape(
            InvalidCaseShapeReason::MissingFallbackPattern,
        ));
    }

    let mut next = last_clause.branch;
    for clause in reachable_clauses.into_iter().rev() {
        next = bool_case_expr(clause.condition, clause.branch, next)?;
    }

    Ok(next)
}

fn bind_int_case_subject(subject: IntExpr, context: &mut PlanContext<'_>) -> (Step, IntExpr) {
    let local = context.define_internal_int_local();
    let name = internal_int_case_subject_name(local);
    (
        Step::let_int(local, name.clone(), subject),
        IntExpr::local_get(local, name),
    )
}

fn bind_string_case_subject(
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

fn bind_float_case_subject(subject: FloatExpr, context: &mut PlanContext<'_>) -> (Step, FloatExpr) {
    let local = context.define_internal_float_local();
    let name = internal_float_case_subject_name(local);
    (
        Step::let_float(local, name.clone(), subject),
        FloatExpr::local_get(local, name),
    )
}

fn bind_bool_case_subject(subject: BoolExpr, context: &mut PlanContext<'_>) -> (Step, BoolExpr) {
    let local = context.define_internal_bool_local();
    let name = internal_bool_case_subject_name(local);
    (
        Step::let_bool(local, name.clone(), subject),
        BoolExpr::local_get(local, name),
    )
}

fn case_subject_block(step: Step, case: Expr) -> Expr {
    super::super::block::block_expr(vec![step], case)
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
mod tests {
    use crate::plan::{BoolExpr, Expr, IntExpr};
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;
    use std::collections::HashMap;

    #[test]
    fn reject_profile_subject_clause_shapes() {
        assert_eq!(
            expect_plan_error(r#"pub fn main() { case True { True | False -> 1 } }"#),
            PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::AlternativePatterns,
            },
        );
    }

    #[test]
    fn reject_profile_unreachable_subject_clause_body() {
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
    fn reject_margin_subject_clause_shapes() {
        let mut empty_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut extra_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut extra_pattern.definitions.functions[0].body[0],
        );
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

        let mut case_type_mismatch = super::super::compile_bool_case_module();
        let (case_type, _, _) = super::super::expect_case_statement_mut(
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

        let mut branch_type_mismatch = super::super::compile_bool_case_module();
        let (case_type, _, clauses) = super::super::expect_case_statement_mut(
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
                then: super::super::super::typed_int_expr(1),
                branch_bindings: Vec::new(),
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
}
