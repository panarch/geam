mod bit_array;
mod bool_;
mod custom;
mod external;
mod float;
mod function;
mod generic;
mod int;
mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

use crate::plan::{
    BoolExpr, BoolLocalId, Expr, FloatExpr, FloatLocalId, IntExpr, IntLocalId, Step, StringExpr,
    StringLocalId, ValueShape,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, SrcSpan, TypedClause, TypedClauseGuard, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

use super::coverage::{CaseBranchRequirement, CaseCoverage, require_branch};

#[cfg(test)]
use crate::plan::ValueType;

#[cfg(test)]
fn pattern_kind_mismatch(expected: ValueType, actual: crate::planner::PatternKind) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: crate::planner::InvalidTypedAstReason::PatternShape {
            reason: crate::planner::InvalidPatternShapeReason::KindMismatch { expected, actual },
        },
    }
}

#[cfg(test)]
fn pattern_type_mismatch(expected: ValueType, actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: crate::planner::InvalidTypedAstReason::PatternShape {
            reason: crate::planner::InvalidPatternShapeReason::TypeMismatch { expected, actual },
        },
    }
}

#[cfg(test)]
fn expression_type_mismatch(
    expected: crate::planner::InvalidExpressionType,
    actual: crate::planner::InvalidExpressionType,
) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: crate::planner::InvalidTypedAstReason::ExpressionType { expected, actual },
    }
}

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    coverage: CaseCoverage,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let clauses = case_clauses(clauses, &coverage)?;
    let source_type = subject.type_();
    let subject_shape = context.value_shape(source_type.as_ref());
    match subject_shape {
        ValueShape::Parameter(parameter) => {
            generic::plan(type_, subject, parameter, clauses, context)
        }
        ValueShape::Bool => bool_::plan(type_, subject, clauses, context),
        ValueShape::Int => int::plan(type_, subject, clauses, context),
        ValueShape::String => string::plan(type_, subject, clauses, context),
        ValueShape::BitArray => bit_array::plan(type_, subject, clauses, context),
        ValueShape::UtfCodepoint => utf_codepoint::plan(type_, subject, clauses, context),
        ValueShape::Custom(shape) => {
            custom::plan(type_, subject, ValueShape::Custom(shape), clauses, context)
        }
        ValueShape::External(shape) => external::plan(type_, subject, shape, clauses, context),
        ValueShape::Float => float::plan(type_, subject, clauses, context),
        ValueShape::Nil => nil::plan(type_, subject, clauses, context),
        ValueShape::Tuple(subject_shape) => {
            let subject_type = subject_shape.iter().map(ValueShape::value_type).collect();
            tuple::plan(
                type_,
                subject,
                subject_type,
                ValueShape::Tuple(subject_shape),
                clauses,
                context,
            )
        }
        ValueShape::List(subject_shape) => {
            let subject_type = subject_shape.value_type();
            list::plan(
                type_,
                subject,
                subject_type,
                ValueShape::List(subject_shape),
                clauses,
                context,
            )
        }
        ValueShape::Function(subject_shape) => {
            let subject_type = subject_shape.type_();
            function::plan(
                type_,
                subject,
                subject_type,
                ValueShape::Function(subject_shape),
                clauses,
                context,
            )
        }
    }
}

pub(super) fn plan_multi(
    type_: Arc<Type>,
    subjects: Vec<TypedExpr>,
    clauses: Vec<TypedClause>,
    coverage: CaseCoverage,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut subject_types = Vec::with_capacity(subjects.len());
    let mut subject_shapes = Vec::with_capacity(subjects.len());
    for subject in &subjects {
        let source_type = subject.type_();
        let shape = context.value_shape(source_type.as_ref());
        subject_types.push(shape.value_type());
        subject_shapes.push(shape);
    }
    let gleam_subject_type = gleam_core::type_::tuple(
        subjects
            .iter()
            .map(|subject| subject.type_().clone())
            .collect(),
    );
    let subject = TypedExpr::Tuple {
        location: subject_group_location(&subjects),
        type_: gleam_subject_type,
        elements: subjects,
    };
    let clauses = multi_subject_case_clauses(clauses, subject_types.len(), &coverage)?;
    let subject_shape = ValueShape::Tuple(subject_shapes.into_boxed_slice());

    tuple::plan(
        type_,
        subject,
        subject_types,
        subject_shape,
        clauses,
        context,
    )
}

struct CaseClause {
    pattern: Pattern<Arc<Type>>,
    alternative_patterns: Vec<Pattern<Arc<Type>>>,
    guard: Option<TypedClauseGuard>,
    then: TypedExpr,
    reachable: bool,
    exhaustive_remainder: bool,
}

struct CasePattern {
    pattern: Pattern<Arc<Type>>,
    reachable: bool,
    exhaustive_remainder: bool,
}

impl CasePattern {
    fn into_parts(self) -> (Pattern<Arc<Type>>, bool, bool) {
        (self.pattern, self.reachable, self.exhaustive_remainder)
    }
}

impl CaseClause {
    fn from_single_subject_typed(
        clause: TypedClause,
        reachable: bool,
        exhaustive_remainder: bool,
    ) -> Result<Self, PlanError> {
        let TypedClause {
            pattern,
            alternative_patterns,
            guard,
            then,
            ..
        } = clause;
        Ok(Self {
            pattern: single_case_pattern(pattern)?,
            alternative_patterns: alternative_patterns
                .into_iter()
                .map(single_case_pattern)
                .collect::<Result<_, _>>()?,
            guard,
            then,
            reachable,
            exhaustive_remainder,
        })
    }

    fn from_multi_subject_typed(
        clause: TypedClause,
        subject_count: usize,
        reachable: bool,
        exhaustive_remainder: bool,
    ) -> Result<Self, PlanError> {
        let TypedClause {
            pattern,
            alternative_patterns,
            guard,
            then,
            ..
        } = clause;
        Ok(Self {
            pattern: tuple_case_pattern(pattern, subject_count)?,
            alternative_patterns: alternative_patterns
                .into_iter()
                .map(|pattern| tuple_case_pattern(pattern, subject_count))
                .collect::<Result<_, _>>()?,
            guard,
            then,
            reachable,
            exhaustive_remainder,
        })
    }

    fn has_alternative_patterns(&self) -> bool {
        !self.alternative_patterns.is_empty()
    }

    fn patterns(&self) -> impl Iterator<Item = CasePattern> + '_ {
        let pattern_count = 1 + self.alternative_patterns.len();
        std::iter::once(self.pattern.clone())
            .chain(self.alternative_patterns.iter().cloned())
            .enumerate()
            .map(move |(index, pattern)| CasePattern {
                pattern,
                reachable: self.reachable,
                exhaustive_remainder: self.exhaustive_remainder && index + 1 == pattern_count,
            })
    }
}

fn case_clauses(
    clauses: Vec<TypedClause>,
    coverage: &CaseCoverage,
) -> Result<Vec<CaseClause>, PlanError> {
    clauses
        .into_iter()
        .enumerate()
        .map(|(index, clause)| {
            CaseClause::from_single_subject_typed(
                clause,
                coverage.is_reachable(index),
                coverage.is_exhaustive_remainder(index),
            )
        })
        .collect()
}

fn multi_subject_case_clauses(
    clauses: Vec<TypedClause>,
    subject_count: usize,
    coverage: &CaseCoverage,
) -> Result<Vec<CaseClause>, PlanError> {
    clauses
        .into_iter()
        .enumerate()
        .map(|(index, clause)| {
            CaseClause::from_multi_subject_typed(
                clause,
                subject_count,
                coverage.is_reachable(index),
                coverage.is_exhaustive_remainder(index),
            )
        })
        .collect()
}

fn single_case_pattern(patterns: Vec<Pattern<Arc<Type>>>) -> Result<Pattern<Arc<Type>>, PlanError> {
    validate_case_pattern_count(&patterns, 1)?;
    let mut patterns = patterns;
    Ok(patterns.remove(0))
}

fn tuple_case_pattern(
    patterns: Vec<Pattern<Arc<Type>>>,
    subject_count: usize,
) -> Result<Pattern<Arc<Type>>, PlanError> {
    validate_case_pattern_count(&patterns, subject_count)?;

    Ok(Pattern::Tuple {
        location: pattern_group_location(&patterns),
        elements: patterns,
    })
}

fn validate_case_pattern_count(
    patterns: &[Pattern<Arc<Type>>],
    expected: usize,
) -> Result<(), PlanError> {
    let actual = patterns.len();
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::PatternSubjectCountMismatch { expected, actual },
            },
        });
    }

    Ok(())
}

fn subject_group_location(subjects: &[TypedExpr]) -> SrcSpan {
    let start = subjects
        .first()
        .map(|subject| subject.location().start)
        .unwrap_or_default();
    let end = subjects
        .last()
        .map(|subject| subject.location().end)
        .unwrap_or_default();
    SrcSpan::new(start, end)
}

fn pattern_group_location(patterns: &[Pattern<Arc<Type>>]) -> SrcSpan {
    let start = patterns
        .first()
        .map(|pattern| pattern.location().start)
        .unwrap_or_default();
    let end = patterns
        .last()
        .map(|pattern| pattern.location().end)
        .unwrap_or_default();
    SrcSpan::new(start, end)
}

#[cfg(test)]
fn mismatched_generic_case_return_type() -> Arc<Type> {
    gleam_core::type_::generic_var(0)
}

fn plan_case_branch(
    return_shape: &ValueShape,
    then: TypedExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let steps = plan_branch_binding_steps(branch_bindings, context);
        let branch = super::super::plan_expr_with_expected_source_stop_shape(
            then,
            return_shape.clone(),
            context,
        )?;
        super::result::validate_branch_type(return_shape, &branch)?;

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
    reachable: bool,
}

struct OrderedCaseClauseInput<'a> {
    return_shape: &'a ValueShape,
    then: TypedExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    guard: Option<TypedClauseGuard>,
    match_condition: BoolExpr,
    is_total: bool,
    reachable: bool,
    exhaustive_remainder: bool,
}

struct OrderedCasePattern {
    match_condition: BoolExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    total_branch_steps: Vec<Step>,
    is_total: bool,
}

struct OrderedCaseCandidateInput<'a> {
    return_shape: &'a ValueShape,
    then: TypedExpr,
    guard: Option<TypedClauseGuard>,
    reachable: bool,
    exhaustive_remainder: bool,
}

fn plan_ordered_case_clause(
    input: OrderedCaseClauseInput<'_>,
    context: &mut PlanContext<'_>,
) -> Result<OrderedCaseClause, PlanError> {
    let OrderedCaseClauseInput {
        return_shape,
        then,
        branch_bindings,
        guard,
        match_condition,
        is_total,
        reachable,
        exhaustive_remainder,
    } = input;

    plan_ordered_case_candidate(
        OrderedCaseCandidateInput {
            return_shape,
            then,
            guard,
            reachable,
            exhaustive_remainder,
        },
        context,
        |_| {
            Ok(OrderedCasePattern {
                match_condition,
                branch_bindings,
                total_branch_steps: Vec::new(),
                is_total,
            })
        },
    )
}

fn plan_ordered_case_candidate(
    input: OrderedCaseCandidateInput<'_>,
    context: &mut PlanContext<'_>,
    plan_pattern: impl FnOnce(&mut PlanContext<'_>) -> Result<OrderedCasePattern, PlanError>,
) -> Result<OrderedCaseClause, PlanError> {
    let OrderedCaseCandidateInput {
        return_shape,
        then,
        guard,
        reachable,
        exhaustive_remainder,
    } = input;

    context.with_local_scope(|context| {
        let OrderedCasePattern {
            match_condition,
            branch_bindings,
            total_branch_steps,
            is_total,
        } = plan_pattern(context)?;
        let is_guarded = guard.is_some();
        let is_total = !is_guarded && (is_total || exhaustive_remainder);
        let branch_binding_steps = plan_branch_binding_steps(branch_bindings, context);
        let guard_condition = guard
            .map(|guard| super::guard::plan_bool(guard, context))
            .transpose()?;
        let condition = match guard_condition {
            Some(guard_condition) => {
                let guard_condition = if branch_binding_steps.is_empty() {
                    guard_condition
                } else {
                    BoolExpr::block(branch_binding_steps.clone(), guard_condition)
                };
                BoolExpr::and(match_condition, guard_condition)
            }
            None => match_condition,
        };

        let branch = super::super::plan_expr_with_expected_source_stop_shape(
            then,
            return_shape.clone(),
            context,
        )?;
        super::result::validate_branch_type(return_shape, &branch)?;
        let mut binding_steps = if is_total {
            total_branch_steps
        } else {
            Vec::new()
        };
        binding_steps.extend(branch_binding_steps);
        let branch = if binding_steps.is_empty() {
            branch
        } else {
            super::super::block::block_expr(binding_steps, branch)
        };

        Ok(OrderedCaseClause {
            condition,
            branch,
            is_total,
            reachable,
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
    ordered_case_expr_for(clauses, CaseBranchRequirement::Fallback)
}

fn ordered_case_expr_for(
    clauses: Vec<OrderedCaseClause>,
    requirement: CaseBranchRequirement,
) -> Result<Expr, PlanError> {
    let mut reachable_clauses = Vec::new();
    for clause in clauses {
        if !clause.reachable {
            continue;
        }
        let is_total = clause.is_total;
        reachable_clauses.push(clause);
        if is_total {
            break;
        }
    }

    let last_clause = require_branch(
        reachable_clauses.pop().filter(|clause| clause.is_total),
        requirement,
    )?;

    let mut next = last_clause.branch;
    for clause in reachable_clauses.into_iter().rev() {
        next = super::result::bool_case_expr(clause.condition, clause.branch, next)?;
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
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use crate::plan::{BoolExpr, Expr, IntExpr, ValueType};
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        function, int, int_return_block, int_return_expr, let_int_step, let_tuple_step, local_int,
        local_tuple, module, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;
    use std::collections::HashMap;

    #[test]
    fn reject_profile_unreachable_subject_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case True {
    _ -> 1
    True -> { <<1:native>> 2 }
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    _ -> 1
    1 -> { <<1:native>> 2 }
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    value if value > 0 -> { <<1:native>> value }
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 0,
                    },
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 2,
                    },
                },
            }),
        );

        let mut empty_alternative_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut empty_alternative_pattern.definitions.functions[0].body[0],
        );
        clauses[0].alternative_patterns.push(Vec::new());
        assert_eq!(
            plan_module(empty_alternative_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 0,
                    },
                },
            }),
        );

        let mut extra_alternative_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut extra_alternative_pattern.definitions.functions[0].body[0],
        );
        let pattern = clauses[0].pattern[0].clone();
        clauses[0]
            .alternative_patterns
            .push(vec![pattern.clone(), pattern]);
        assert_eq!(
            plan_module(extra_alternative_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 2,
                    },
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
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Bool,
                        actual: ValueType::Int,
                    },
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
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::String,
                        actual: ValueType::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_multi_subject_pattern_count_mismatch() {
        let mut primary_mismatch = compile(
            r#"
pub fn main() {
  case True, False {
    True, False -> 1
    _, _ -> 0
  }
}
"#,
        );
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut primary_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern.pop();
        assert_eq!(
            plan_module(primary_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 2,
                        actual: 1,
                    },
                },
            }),
        );

        let mut alternative_mismatch = compile(
            r#"
pub fn main() {
  case True, False {
    True, False | False, True -> 1
    _, _ -> 0
  }
}
"#,
        );
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut alternative_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].alternative_patterns[0].pop();
        assert_eq!(
            plan_module(alternative_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 2,
                        actual: 1,
                    },
                },
            }),
        );
    }

    #[test]
    fn plan_multi_subject_binds_tuple_subject_once_and_projects_branch_bindings() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  case 11, 37 {
    left, right -> left + right
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple([int(11), int(37)]),
                    )],
                    int_return_block(
                        [
                            let_int_step(
                                0,
                                "left",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
                            ),
                            let_int_step(
                                1,
                                "right",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
                            ),
                        ],
                        int_return_expr(local_int(0, "left").add_int(local_int(1, "right"))),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_multi_subject_alternative_guard_wraps_each_pattern_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  case 11, 37 {
    left, 0 | 11, left if left > 20 -> left
    _, _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let first_binding = let_int_step(
            0,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
        );
        let second_binding = let_int_step(
            1,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let first_condition = BoolExpr::and(
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1)),
                Expr::from(int(0)),
            ),
            BoolExpr::block(
                vec![first_binding.clone()],
                BoolExpr::gt_int(local_int(0, "left").into(), int(20).into()),
            ),
        );
        let second_condition = BoolExpr::and(
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
                Expr::from(int(11)),
            ),
            BoolExpr::block(
                vec![second_binding.clone()],
                BoolExpr::gt_int(local_int(1, "left").into(), int(20).into()),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple([int(11), int(37)]),
                    )],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(local_int(0, "left"))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_binding],
                                int_return_expr(local_int(1, "left")),
                            ),
                            int_return_expr(int(0)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
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
                reachable: true,
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
                reachable: true,
            }]),
            Ok(Expr::int(IntExpr::value(1.into()))),
        );
        assert_eq!(
            super::ordered_case_expr(vec![
                super::OrderedCaseClause {
                    condition: BoolExpr::value(false),
                    branch: Expr::int(IntExpr::value(1.into())),
                    is_total: false,
                    reachable: true,
                },
                super::OrderedCaseClause {
                    condition: BoolExpr::value(true),
                    branch: Expr::int(IntExpr::value(0.into())),
                    is_total: true,
                    reachable: true,
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
                    reachable: true,
                },
                super::OrderedCaseClause {
                    condition: BoolExpr::value(true),
                    branch: Expr::int(IntExpr::value(999.into())),
                    is_total: false,
                    reachable: true,
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
        let actual = super::plan_ordered_case_clause(
            super::OrderedCaseClauseInput {
                return_shape: &crate::plan::ValueShape::String,
                then: super::super::super::typed_int_expr(1),
                branch_bindings: Vec::new(),
                guard: None,
                match_condition: BoolExpr::value(true),
                is_total: true,
                reachable: true,
                exhaustive_remainder: false,
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
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::String,
                        actual: ValueType::Int,
                    },
                },
            },
        );
    }
}
