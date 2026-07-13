mod bit_array;
mod bool_;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;

use crate::plan::{
    BoolCaseBranches, BoolExpr, BoolListCaseBranches, BoolLocalId, Expr, ExprKind, FloatExpr,
    FloatLocalId, FunctionExprKind, IntExpr, IntLocalId, ListExpr, Step, StringExpr, StringLocalId,
    ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCaseShapeReason, PlanError, UnsupportedCaseReason, UnsupportedPatternKind,
};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, SrcSpan, TypedClause, TypedClauseGuard, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let clauses = case_clauses(clauses)?;
    match ValueType::from_gleam(subject.type_().as_ref()) {
        Some(ValueType::Bool) => bool_::plan(type_, subject, clauses, context),
        Some(ValueType::Int) => int::plan(type_, subject, clauses, context),
        Some(ValueType::String) => string::plan(type_, subject, clauses, context),
        Some(ValueType::BitArray) => bit_array::plan(type_, subject, clauses, context),
        Some(ValueType::Float) => float::plan(type_, subject, clauses, context),
        Some(ValueType::Nil) => nil::plan(type_, subject, clauses, context),
        Some(ValueType::Tuple(subject_type)) => {
            tuple::plan(type_, subject, subject_type, clauses, context)
        }
        Some(ValueType::List(subject_type)) => {
            list::plan(type_, subject, *subject_type, clauses, context)
        }
        Some(ValueType::Function(subject_type)) => {
            function::plan(type_, subject, *subject_type, clauses, context)
        }
        _ => Err(super::unsupported_case(
            UnsupportedCaseReason::UnsupportedSubjectType,
        )),
    }
}

pub(super) fn plan_multi(
    type_: Arc<Type>,
    subjects: Vec<TypedExpr>,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut subject_types = Vec::with_capacity(subjects.len());
    for subject in &subjects {
        let Some(type_) = ValueType::from_gleam(subject.type_().as_ref()) else {
            return Err(super::unsupported_case(
                UnsupportedCaseReason::UnsupportedSubjectType,
            ));
        };
        subject_types.push(type_);
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
    let clauses = multi_subject_case_clauses(clauses, subject_types.len())?;

    tuple::plan(type_, subject, subject_types, clauses, context)
}

struct CaseClause {
    pattern: Pattern<Arc<Type>>,
    alternative_patterns: Vec<Pattern<Arc<Type>>>,
    guard: Option<TypedClauseGuard>,
    then: TypedExpr,
}

impl CaseClause {
    fn from_single_subject_typed(clause: TypedClause) -> Result<Self, PlanError> {
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
        })
    }

    fn from_multi_subject_typed(
        clause: TypedClause,
        subject_count: usize,
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
        })
    }

    fn has_alternative_patterns(&self) -> bool {
        !self.alternative_patterns.is_empty()
    }

    fn patterns(&self) -> impl Iterator<Item = Pattern<Arc<Type>>> + '_ {
        std::iter::once(self.pattern.clone()).chain(self.alternative_patterns.iter().cloned())
    }
}

fn case_clauses(clauses: Vec<TypedClause>) -> Result<Vec<CaseClause>, PlanError> {
    clauses
        .into_iter()
        .map(CaseClause::from_single_subject_typed)
        .collect()
}

fn multi_subject_case_clauses(
    clauses: Vec<TypedClause>,
    subject_count: usize,
) -> Result<Vec<CaseClause>, PlanError> {
    clauses
        .into_iter()
        .map(|clause| CaseClause::from_multi_subject_typed(clause, subject_count))
        .collect()
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

fn tuple_case_pattern(
    patterns: Vec<Pattern<Arc<Type>>>,
    subject_count: usize,
) -> Result<Pattern<Arc<Type>>, PlanError> {
    if patterns.len() != subject_count {
        return Err(super::invalid_case_shape(
            InvalidCaseShapeReason::PatternSubjectCountMismatch,
        ));
    }

    Ok(Pattern::Tuple {
        location: pattern_group_location(&patterns),
        elements: patterns,
    })
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

fn validate_case_branch_type(case_type: &Type, branch: &Expr) -> Result<(), PlanError> {
    if ValueType::from_gleam(case_type) == Some(branch.value_type()) {
        return Ok(());
    }

    Err(super::invalid_case_shape(
        InvalidCaseShapeReason::BranchReturnTypeMismatch,
    ))
}

fn unsupported_bit_array_pattern<T>() -> Result<T, PlanError> {
    Err(PlanError::UnsupportedPattern {
        kind: UnsupportedPatternKind::BitArray,
    })
}

fn bool_case_expr(subject: BoolExpr, true_: Expr, false_: Expr) -> Result<Expr, PlanError> {
    let branches = match (true_.into_kind(), false_.into_kind()) {
        (ExprKind::Int(true_), ExprKind::Int(false_)) => BoolCaseBranches::Int { true_, false_ },
        (ExprKind::String(true_), ExprKind::String(false_)) => {
            BoolCaseBranches::String { true_, false_ }
        }
        (ExprKind::BitArray(true_), ExprKind::BitArray(false_)) => {
            BoolCaseBranches::BitArray { true_, false_ }
        }
        (ExprKind::Float(true_), ExprKind::Float(false_)) => {
            BoolCaseBranches::Float { true_, false_ }
        }
        (ExprKind::Bool(true_), ExprKind::Bool(false_)) => BoolCaseBranches::Bool { true_, false_ },
        (ExprKind::Nil(true_), ExprKind::Nil(false_)) => BoolCaseBranches::Nil { true_, false_ },
        (ExprKind::Tuple(true_), ExprKind::Tuple(false_)) => {
            BoolCaseBranches::Tuple { true_, false_ }
        }
        (ExprKind::List(true_), ExprKind::List(false_)) => {
            BoolCaseBranches::List(bool_list_case_branches(true_, false_)?)
        }
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

fn bool_list_case_branches(
    true_: ListExpr,
    false_: ListExpr,
) -> Result<BoolListCaseBranches, PlanError> {
    Ok(match (true_, false_) {
        (ListExpr::Int(true_), ListExpr::Int(false_)) => {
            BoolListCaseBranches::Int { true_, false_ }
        }
        (ListExpr::String(true_), ListExpr::String(false_)) => {
            BoolListCaseBranches::String { true_, false_ }
        }
        (ListExpr::BitArray(true_), ListExpr::BitArray(false_)) => {
            BoolListCaseBranches::BitArray { true_, false_ }
        }
        (ListExpr::Float(true_), ListExpr::Float(false_)) => {
            BoolListCaseBranches::Float { true_, false_ }
        }
        (ListExpr::Bool(true_), ListExpr::Bool(false_)) => {
            BoolListCaseBranches::Bool { true_, false_ }
        }
        (ListExpr::Nil(true_), ListExpr::Nil(false_)) => {
            BoolListCaseBranches::Nil { true_, false_ }
        }
        (ListExpr::Tuple(true_), ListExpr::Tuple(false_)) if true_.item() == false_.item() => {
            BoolListCaseBranches::Tuple { true_, false_ }
        }
        (ListExpr::List(true_), ListExpr::List(false_)) if true_.item() == false_.item() => {
            BoolListCaseBranches::List { true_, false_ }
        }
        (ListExpr::Function(true_), ListExpr::Function(false_))
            if true_.item() == false_.item() =>
        {
            BoolListCaseBranches::Function { true_, false_ }
        }
        _ => {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        }
    })
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
        (FunctionExprKind::BitArray(true_), FunctionExprKind::BitArray(false_)) => {
            BoolCaseBranches::BitArrayFunction { true_, false_ }
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
            Some(guard_condition) => {
                let guard_condition = if binding_steps.is_empty() {
                    guard_condition
                } else {
                    BoolExpr::block(binding_steps.clone(), guard_condition)
                };
                BoolExpr::and(match_condition, guard_condition)
            }
            None => match_condition,
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
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, Expr, FunctionType, IntExpr, ListExpr, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        function, int, int_return_block, int_return_expr, let_int_step, let_tuple_step, local_int,
        local_tuple, module, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;
    use std::collections::HashMap;

    #[test]
    fn bool_list_case_branches_preserves_typed_item_family() {
        let true_ = ListExpr::value(Vec::new(), ValueType::String)
            .into_string()
            .expect("string list should build StringListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::String)
            .into_string()
            .expect("string list should build StringListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::String(true_.clone()),
                ListExpr::String(false_.clone()),
            ),
            Ok(BoolListCaseBranches::String { true_, false_ }),
        );
        let true_ = ListExpr::value(Vec::new(), ValueType::String)
            .into_string()
            .expect("string list should build StringListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::String)
            .into_string()
            .expect("string list should build StringListExpr");
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::list(ListExpr::String(true_.clone())),
                Expr::list(ListExpr::String(false_.clone())),
            ),
            Ok(Expr::list(ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::String { true_, false_ },
            ))),
        );

        let true_ = ListExpr::value(Vec::new(), ValueType::Float)
            .into_float()
            .expect("float list should build FloatListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::Float)
            .into_float()
            .expect("float list should build FloatListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Float(true_.clone()),
                ListExpr::Float(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Float { true_, false_ }),
        );

        let true_ = ListExpr::value(Vec::new(), ValueType::Bool)
            .into_bool()
            .expect("bool list should build BoolListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::Bool)
            .into_bool()
            .expect("bool list should build BoolListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Bool(true_.clone()),
                ListExpr::Bool(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Bool { true_, false_ }),
        );

        let true_ = ListExpr::value(Vec::new(), ValueType::Nil)
            .into_nil()
            .expect("nil list should build NilListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::Nil)
            .into_nil()
            .expect("nil list should build NilListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Nil(true_.clone()),
                ListExpr::Nil(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Nil { true_, false_ }),
        );

        let true_ = ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
            .into_tuple()
            .expect("tuple list should build TupleListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
            .into_tuple()
            .expect("tuple list should build TupleListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Tuple(true_.clone()),
                ListExpr::Tuple(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Tuple { true_, false_ }),
        );

        let true_ = ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String)))
            .into_list()
            .expect("nested list should build ListListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String)))
            .into_list()
            .expect("nested list should build ListListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::List(true_.clone()),
                ListExpr::List(false_.clone()),
            ),
            Ok(BoolListCaseBranches::List { true_, false_ }),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let true_ = ListExpr::value(
            Vec::new(),
            ValueType::Function(Box::new(function_type.clone())),
        )
        .into_function()
        .expect("function list should build FunctionListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::Function(Box::new(function_type)))
            .into_function()
            .expect("function list should build FunctionListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Function(true_.clone()),
                ListExpr::Function(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Function { true_, false_ }),
        );

        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::String])),
            ),
            Err(super::super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            )),
        );
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::list(ListExpr::value(
                    Vec::new(),
                    ValueType::Tuple(vec![ValueType::Int]),
                )),
                Expr::list(ListExpr::value(
                    Vec::new(),
                    ValueType::Tuple(vec![ValueType::String]),
                )),
            ),
            Err(super::super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            )),
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

        let mut empty_alternative_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut empty_alternative_pattern.definitions.functions[0].body[0],
        );
        clauses[0].alternative_patterns.push(Vec::new());
        assert_eq!(
            plan_module(empty_alternative_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_multi_subject_with_unsupported_value_family() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case Ok(1), 2 {
    _, _ -> 0
  }
}
"#,
            ),
            super::super::unsupported_case(UnsupportedCaseReason::UnsupportedSubjectType),
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
