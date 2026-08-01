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
    BoolCaseBranches, BoolExpr, BoolListCaseBranches, BoolLocalId, Expr, ExprKind, FloatExpr,
    FloatLocalId, FunctionExprKind, IntExpr, IntLocalId, ListExpr, Step, StringExpr, StringLocalId,
    ValueShape,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
use crate::planner::statement::plan_variable_runtime_step;
use ecow::EcoString;
use gleam_core::ast::{Pattern, SrcSpan, TypedClause, TypedClauseGuard, TypedExpr};
use gleam_core::type_::{Type, TypeVar};
use std::sync::Arc;

#[cfg(test)]
use crate::plan::ValueType;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CaseSubjectVariants {
    Other,
    Custom,
    Tuple(Vec<CaseSubjectVariants>),
    List(Box<CaseSubjectVariants>),
}

impl CaseSubjectVariants {
    fn from_gleam(type_: &Type) -> Self {
        match type_ {
            Type::Var { type_ } => match &*type_.borrow() {
                TypeVar::Link { type_ } => Self::from_gleam(type_.as_ref()),
                TypeVar::Unbound { .. } | TypeVar::Generic { .. } => Self::Other,
            },
            Type::Tuple { elements } => Self::Tuple(
                elements
                    .iter()
                    .map(|element| Self::from_gleam(element.as_ref()))
                    .collect(),
            ),
            Type::Named { .. } => {
                if let Some(element) = type_.list_type() {
                    Self::List(Box::new(Self::from_gleam(element.as_ref())))
                } else if type_.is_int()
                    || type_.is_float()
                    || type_.is_string()
                    || type_.is_bit_array()
                    || type_.is_utf_codepoint()
                    || type_.is_bool()
                    || type_.is_nil()
                {
                    Self::Other
                } else {
                    Self::Custom
                }
            }
            Type::Fn { .. } => Self::Other,
        }
    }

    #[cfg(test)]
    fn from_value_type(value_type: &ValueType) -> Self {
        match value_type {
            ValueType::Parameter(_) => Self::Other,
            ValueType::Custom(_) => Self::Custom,
            ValueType::External(_) => Self::Other,
            ValueType::Tuple(elements) => {
                Self::Tuple(elements.iter().map(Self::from_value_type).collect())
            }
            ValueType::List(element) => Self::List(Box::new(Self::from_value_type(element))),
            ValueType::Int
            | ValueType::Float
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Bool
            | ValueType::Nil
            | ValueType::Function(_) => Self::Other,
        }
    }

    fn into_tuple(self) -> Option<Vec<Self>> {
        match self {
            Self::Tuple(elements) => Some(elements),
            Self::Other | Self::Custom | Self::List(_) => None,
        }
    }

    fn into_list(self) -> Option<Self> {
        match self {
            Self::List(element) => Some(*element),
            Self::Other | Self::Custom | Self::Tuple(_) => None,
        }
    }
}

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let clauses = case_clauses(clauses)?;
    let source_type = subject.type_();
    let subject_shape = context.value_shape(source_type.as_ref());
    let subject_variants = CaseSubjectVariants::from_gleam(source_type.as_ref());
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
                subject_variants,
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
                subject_variants,
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
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut subject_types = Vec::with_capacity(subjects.len());
    let mut subject_shapes = Vec::with_capacity(subjects.len());
    let mut subject_variants = Vec::with_capacity(subjects.len());
    for subject in &subjects {
        let source_type = subject.type_();
        let shape = context.value_shape(source_type.as_ref());
        subject_variants.push(CaseSubjectVariants::from_gleam(source_type.as_ref()));
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
    let clauses = multi_subject_case_clauses(clauses, subject_types.len())?;

    tuple::plan(
        type_,
        subject,
        subject_types,
        ValueShape::Tuple(subject_shapes.into_boxed_slice()),
        CaseSubjectVariants::Tuple(subject_variants),
        clauses,
        context,
    )
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

fn validate_case_branch_type(
    _case_type: &Type,
    expected: &ValueShape,
    branch: &Expr,
) -> Result<(), PlanError> {
    if expected.value_type() == branch.value_type() {
        return Ok(());
    }

    Err(super::invalid_case_shape(
        InvalidCaseShapeReason::BranchReturnTypeMismatch,
    ))
}

fn bool_case_expr(subject: BoolExpr, true_: Expr, false_: Expr) -> Result<Expr, PlanError> {
    if let (ExprKind::Generic(true_), ExprKind::Generic(false_)) = (true_.kind(), false_.kind()) {
        let Some(expression) =
            crate::plan::GenericExpr::bool_case(subject, true_.clone(), false_.clone())
        else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        return Ok(Expr::generic(expression));
    }
    if let (ExprKind::Function(true_), ExprKind::Function(false_)) = (true_.kind(), false_.kind())
        && let (FunctionExprKind::Generic(true_), FunctionExprKind::Generic(false_)) =
            (true_.kind(), false_.kind())
    {
        let Some(expression) =
            crate::plan::GenericFunctionExpr::bool_case(subject, true_.clone(), false_.clone())
        else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        return Ok(Expr::function(crate::plan::FunctionExpr::generic(
            expression,
        )));
    }

    let true_shape = true_.value_shape().clone();
    let false_shape = false_.value_shape().clone();
    let branches = match (true_.into_kind(), false_.into_kind()) {
        (ExprKind::Int(true_), ExprKind::Int(false_)) => BoolCaseBranches::Int { true_, false_ },
        (ExprKind::String(true_), ExprKind::String(false_)) => {
            BoolCaseBranches::String { true_, false_ }
        }
        (ExprKind::BitArray(true_), ExprKind::BitArray(false_)) => {
            BoolCaseBranches::BitArray { true_, false_ }
        }
        (ExprKind::UtfCodepoint(true_), ExprKind::UtfCodepoint(false_)) => {
            BoolCaseBranches::UtfCodepoint { true_, false_ }
        }
        (ExprKind::Custom(true_), ExprKind::Custom(false_)) => {
            let Some(branches) = crate::plan::CustomBoolCaseBranches::try_new(true_, false_) else {
                return Err(super::invalid_case_shape(
                    InvalidCaseShapeReason::BranchReturnTypeMismatch,
                ));
            };
            BoolCaseBranches::Custom(branches)
        }
        (ExprKind::External(true_), ExprKind::External(false_))
            if true_.shape() == false_.shape() =>
        {
            BoolCaseBranches::External { true_, false_ }
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
    let shape = case_result_shape(std::slice::from_ref(&true_shape), &false_shape)?;
    match Expr::bool_case(subject, branches).with_resolved_shape(shape) {
        Some(expression) => Ok(expression),
        None => Err(super::invalid_case_shape(
            InvalidCaseShapeReason::BranchReturnTypeMismatch,
        )),
    }
}

fn generic_case_clauses<Pattern>(
    clauses: Vec<(Pattern, Expr)>,
) -> Result<Vec<(Pattern, crate::plan::GenericExpr)>, PlanError> {
    let (patterns, clauses): (Vec<_>, Vec<_>) = clauses.into_iter().unzip();
    generic_case_bodies(clauses).map(|clauses| patterns.into_iter().zip(clauses).collect())
}

fn generic_case_bodies(clauses: Vec<Expr>) -> Result<Vec<crate::plan::GenericExpr>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for clause in clauses {
        let ExprKind::Generic(clause) = clause.into_kind() else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push(clause);
    }
    Ok(typed_clauses)
}

fn generic_function_case_clauses<Pattern>(
    clauses: Vec<(Pattern, Expr)>,
) -> Result<Vec<(Pattern, crate::plan::GenericFunctionExpr)>, PlanError> {
    let (patterns, clauses): (Vec<_>, Vec<_>) = clauses.into_iter().unzip();
    generic_function_case_bodies(clauses).map(|clauses| patterns.into_iter().zip(clauses).collect())
}

fn generic_function_case_bodies(
    clauses: Vec<Expr>,
) -> Result<Vec<crate::plan::GenericFunctionExpr>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for clause in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_generic() else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push(clause);
    }
    Ok(typed_clauses)
}

fn case_result_shape(
    branches: &[crate::plan::ValueShape],
    fallback: &crate::plan::ValueShape,
) -> Result<crate::plan::ValueShape, PlanError> {
    let mut shape = fallback.clone();
    for branch in branches {
        let Some(merged) = branch.merge(&shape) else {
            return Err(super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        shape = merged;
    }
    Ok(shape)
}

fn bool_list_case_branches(
    true_: ListExpr,
    false_: ListExpr,
) -> Result<BoolListCaseBranches, PlanError> {
    Ok(match (true_, false_) {
        (ListExpr::Generic(true_), ListExpr::Generic(false_)) if true_.item() == false_.item() => {
            BoolListCaseBranches::Generic { true_, false_ }
        }
        (ListExpr::ParameterList(true_), ListExpr::ParameterList(false_))
            if true_.item() == false_.item() =>
        {
            BoolListCaseBranches::ParameterList { true_, false_ }
        }
        (ListExpr::Int(true_), ListExpr::Int(false_)) => {
            BoolListCaseBranches::Int { true_, false_ }
        }
        (ListExpr::String(true_), ListExpr::String(false_)) => {
            BoolListCaseBranches::String { true_, false_ }
        }
        (ListExpr::BitArray(true_), ListExpr::BitArray(false_)) => {
            BoolListCaseBranches::BitArray { true_, false_ }
        }
        (ListExpr::UtfCodepoint(true_), ListExpr::UtfCodepoint(false_)) => {
            BoolListCaseBranches::UtfCodepoint { true_, false_ }
        }
        (ListExpr::Custom(true_), ListExpr::Custom(false_)) if true_.item() == false_.item() => {
            BoolListCaseBranches::Custom { true_, false_ }
        }
        (ListExpr::External(true_), ListExpr::External(false_))
            if true_.item() == false_.item() =>
        {
            BoolListCaseBranches::External { true_, false_ }
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
        (FunctionExprKind::UtfCodepoint(true_), FunctionExprKind::UtfCodepoint(false_)) => {
            BoolCaseBranches::UtfCodepointFunction { true_, false_ }
        }
        (FunctionExprKind::Custom(true_), FunctionExprKind::Custom(false_))
            if true_.type_() == false_.type_() =>
        {
            BoolCaseBranches::CustomFunction { true_, false_ }
        }
        (FunctionExprKind::External(true_), FunctionExprKind::External(false_))
            if true_.type_() == false_.type_() =>
        {
            BoolCaseBranches::ExternalFunction { true_, false_ }
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

#[cfg(test)]
fn mismatched_generic_case_return_type() -> Arc<Type> {
    gleam_core::type_::generic_var(0)
}

fn plan_case_branch(
    case_type: &Type,
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
        validate_case_branch_type(case_type, return_shape, &branch)?;

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
    return_shape: &'a ValueShape,
    then: TypedExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    guard: Option<TypedClauseGuard>,
    match_condition: BoolExpr,
    is_total: bool,
}

struct OrderedCasePattern {
    match_condition: BoolExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    total_branch_steps: Vec<Step>,
    is_total: bool,
}

struct OrderedCaseCandidateInput<'a> {
    case_type: &'a Type,
    return_shape: &'a ValueShape,
    then: TypedExpr,
    guard: Option<TypedClauseGuard>,
}

fn plan_ordered_case_clause(
    input: OrderedCaseClauseInput<'_>,
    context: &mut PlanContext<'_>,
) -> Result<OrderedCaseClause, PlanError> {
    let OrderedCaseClauseInput {
        case_type,
        return_shape,
        then,
        branch_bindings,
        guard,
        match_condition,
        is_total,
    } = input;

    plan_ordered_case_candidate(
        OrderedCaseCandidateInput {
            case_type,
            return_shape,
            then,
            guard,
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
        case_type,
        return_shape,
        then,
        guard,
    } = input;

    context.with_local_scope(|context| {
        let OrderedCasePattern {
            match_condition,
            branch_bindings,
            total_branch_steps,
            is_total,
        } = plan_pattern(context)?;
        let is_guarded = guard.is_some();
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
        validate_case_branch_type(case_type, return_shape, &branch)?;
        let mut binding_steps = if is_total && !is_guarded {
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
            is_total: is_total && !is_guarded,
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
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use crate::plan::{
        BoolCaseBranches, BoolExpr, BoolListCaseBranches, Expr, ExternalExpr, ExternalFunctionExpr,
        ExternalFunctionLocal, ExternalFunctionLocalId, ExternalFunctionType, ExternalLocal,
        ExternalLocalId, ExternalTypeName, ExternalValueShape, FunctionExpr, FunctionType,
        GenericExpr, GenericFunctionExpr, GenericFunctionLocal, GenericFunctionLocalId,
        GenericFunctionType, GenericLocal, GenericLocalId, IntExpr, ListExpr, TypeParameterId,
        ValueShape, ValueType,
    };
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
    use gleam_core::type_::{self, Type, TypeVar};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn reject_margin_generic_bool_case_handoffs() {
        let generic = |parameter, local| {
            Expr::generic(crate::plan::GenericExpr::local_get(
                crate::plan::GenericLocal::new(
                    crate::plan::GenericLocalId(local),
                    crate::plan::TypeParameterId(parameter),
                ),
                "generic".into(),
            ))
        };
        let generic_function = |parameter, local| {
            let type_ = crate::plan::GenericFunctionType::new(
                vec![crate::plan::ValueShape::Int],
                crate::plan::TypeParameterId(parameter),
            );
            Expr::function(crate::plan::FunctionExpr::generic(
                crate::plan::GenericFunctionExpr::local_get(
                    crate::plan::GenericFunctionLocal::new(
                        crate::plan::GenericFunctionLocalId(local),
                        type_,
                    ),
                    "generic_function".into(),
                ),
            ))
        };
        let branch_mismatch =
            || super::super::invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch);

        assert_eq!(
            super::bool_case_expr(BoolExpr::value(true), generic(0, 0), generic(1, 1),),
            Err(branch_mismatch()),
        );
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                generic_function(0, 0),
                generic_function(1, 1),
            ),
            Err(branch_mismatch()),
        );
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                generic(0, 0),
                Expr::int(IntExpr::value(0.into()))
            ),
            Err(branch_mismatch()),
        );
        assert_eq!(
            super::generic_case_bodies(vec![Expr::int(IntExpr::value(0.into()))]),
            Err(branch_mismatch()),
        );
        assert_eq!(
            super::generic_function_case_bodies(vec![Expr::int(IntExpr::value(0.into()))]),
            Err(branch_mismatch()),
        );

        let int_function = crate::plan::FunctionExpr::int(crate::plan::IntFunctionExpr::local_get(
            crate::plan::IntFunctionLocalId(0),
            "int_function".into(),
            crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Int),
        ));
        assert_eq!(
            super::generic_function_case_bodies(vec![Expr::function(int_function.clone())]),
            Err(branch_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_branches(
                generic_function(0, 0)
                    .into_function()
                    .expect("generic function expression"),
                generic_function(0, 1)
                    .into_function()
                    .expect("generic function expression"),
            ),
            Err(branch_mismatch()),
        );

        let custom = |name: &str, local| {
            Expr::custom(crate::plan::CustomExpr::local_get(
                crate::plan::CustomLocal::new(
                    crate::plan::CustomLocalId(local),
                    crate::plan::CustomType::new(
                        crate::plan::CustomTypeName::new("geam".into(), "main".into(), name.into()),
                        Vec::new(),
                    ),
                ),
                name.into(),
            ))
        };
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                custom("First", 0),
                custom("Second", 1),
            ),
            Err(branch_mismatch()),
        );
    }

    #[test]
    fn generic_bool_case_handoffs_preserve_parameter_owned_shapes() {
        let parameter = TypeParameterId(0);
        let subject = BoolExpr::value(true);
        let true_ = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "true_value".into(),
        );
        let false_ = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(1), parameter),
            "false_value".into(),
        );
        assert_eq!(
            super::bool_case_expr(
                subject.clone(),
                Expr::generic(true_.clone()),
                Expr::generic(false_.clone()),
            ),
            Ok(Expr::generic(
                GenericExpr::bool_case(subject.clone(), true_, false_)
                    .expect("matching parameter branches have one generic result shape"),
            )),
        );

        let function_type = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let true_ = GenericFunctionExpr::local_get(
            GenericFunctionLocal::new(GenericFunctionLocalId(0), function_type.clone()),
            "true_function".into(),
        );
        let false_ = GenericFunctionExpr::local_get(
            GenericFunctionLocal::new(GenericFunctionLocalId(1), function_type),
            "false_function".into(),
        );
        assert_eq!(
            super::bool_case_expr(
                subject.clone(),
                Expr::function(FunctionExpr::generic(true_.clone())),
                Expr::function(FunctionExpr::generic(false_.clone())),
            ),
            Ok(Expr::function(FunctionExpr::generic(
                GenericFunctionExpr::bool_case(subject.clone(), true_, false_)
                    .expect("matching generic callables have one function shape"),
            ))),
        );

        let true_ = ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty parameter list has generic list storage")
            .into_generic()
            .expect("a parameter item list has generic list storage");
        let false_ = ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty parameter list has generic list storage")
            .into_generic()
            .expect("a parameter item list has generic list storage");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::Generic(true_.clone()),
                ListExpr::Generic(false_.clone()),
            ),
            Ok(BoolListCaseBranches::Generic { true_, false_ }),
        );

        let true_ = ListExpr::try_value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .expect("empty nested parameter list")
        .into_parameter_list()
        .expect("parameter-list item family");
        let false_ = ListExpr::try_value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .expect("empty nested parameter list")
        .into_parameter_list()
        .expect("parameter-list item family");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::ParameterList(true_.clone()),
                ListExpr::ParameterList(false_.clone()),
            ),
            Ok(BoolListCaseBranches::ParameterList { true_, false_ }),
        );
    }

    #[test]
    fn bool_case_branches_preserve_external_nominal_shapes() {
        let first_shape = ExternalValueShape::new(
            ExternalTypeName::new("application".into(), "main".into(), "First".into()),
            Vec::new(),
        );
        let second_shape = ExternalValueShape::new(
            ExternalTypeName::new("application".into(), "main".into(), "Second".into()),
            Vec::new(),
        );
        let true_value = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(0), first_shape.clone()),
            "true_value".into(),
        );
        let false_value = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(1), first_shape.clone()),
            "false_value".into(),
        );
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::external(true_value.clone()),
                Expr::external(false_value.clone()),
            ),
            Ok(Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::External {
                    true_: true_value,
                    false_: false_value,
                },
            )),
        );
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::external(ExternalExpr::local_get(
                    ExternalLocal::from_shape(ExternalLocalId(0), first_shape.clone()),
                    "first".into(),
                )),
                Expr::external(ExternalExpr::local_get(
                    ExternalLocal::from_shape(ExternalLocalId(1), second_shape.clone()),
                    "second".into(),
                )),
            ),
            Err(super::super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            )),
        );

        let first_type = first_shape.type_().clone();
        let second_type = second_shape.type_().clone();
        let true_list = ListExpr::value(Vec::new(), ValueType::External(first_type.clone()))
            .into_external()
            .expect("an external item list should preserve its nominal item");
        let false_list = ListExpr::value(Vec::new(), ValueType::External(first_type.clone()))
            .into_external()
            .expect("an external item list should preserve its nominal item");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::External(true_list.clone()),
                ListExpr::External(false_list.clone()),
            ),
            Ok(BoolListCaseBranches::External {
                true_: true_list,
                false_: false_list,
            }),
        );
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::value(Vec::new(), ValueType::External(first_type)),
                ListExpr::value(Vec::new(), ValueType::External(second_type)),
            ),
            Err(super::super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            )),
        );

        let first_function_type = ExternalFunctionType::from_shapes(Vec::new(), first_shape);
        let second_function_type = ExternalFunctionType::from_shapes(Vec::new(), second_shape);
        let true_function = ExternalFunctionExpr::local_get(
            ExternalFunctionLocal::new(ExternalFunctionLocalId(0), first_function_type.clone()),
            "true_function".into(),
        );
        let false_function = ExternalFunctionExpr::local_get(
            ExternalFunctionLocal::new(ExternalFunctionLocalId(1), first_function_type.clone()),
            "false_function".into(),
        );
        assert_eq!(
            super::bool_function_case_branches(
                FunctionExpr::external(true_function.clone()),
                FunctionExpr::external(false_function.clone()),
            ),
            Ok(BoolCaseBranches::ExternalFunction {
                true_: true_function,
                false_: false_function,
            }),
        );
        assert_eq!(
            super::bool_function_case_branches(
                FunctionExpr::external(ExternalFunctionExpr::local_get(
                    ExternalFunctionLocal::new(ExternalFunctionLocalId(0), first_function_type),
                    "first_function".into(),
                )),
                FunctionExpr::external(ExternalFunctionExpr::local_get(
                    ExternalFunctionLocal::new(ExternalFunctionLocalId(1), second_function_type),
                    "second_function".into(),
                )),
            ),
            Err(super::super::invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            )),
        );
    }

    #[test]
    fn case_subject_variants_follow_the_gleam_type_shape() {
        assert_eq!(
            super::CaseSubjectVariants::from_value_type(&ValueType::Parameter(
                crate::plan::TypeParameterId(0),
            )),
            super::CaseSubjectVariants::Other,
        );
        assert_eq!(
            super::CaseSubjectVariants::from_value_type(&ValueType::External(
                crate::plan::ExternalType::new(
                    crate::plan::ExternalTypeName::new(
                        "application".into(),
                        "main".into(),
                        "Counter".into(),
                    ),
                    Vec::new(),
                ),
            )),
            super::CaseSubjectVariants::Other,
        );
        assert_eq!(
            super::CaseSubjectVariants::from_gleam(type_::int().as_ref()),
            super::CaseSubjectVariants::Other,
        );
        assert_eq!(
            super::CaseSubjectVariants::from_gleam(
                type_::tuple(vec![
                    type_::int(),
                    type_::list(type_::result(type_::int(), type_::string())),
                ])
                .as_ref(),
            ),
            super::CaseSubjectVariants::Tuple(vec![
                super::CaseSubjectVariants::Other,
                super::CaseSubjectVariants::List(Box::new(super::CaseSubjectVariants::Custom)),
            ]),
        );
        assert_eq!(
            super::CaseSubjectVariants::from_gleam(type_::generic_var(0).as_ref()),
            super::CaseSubjectVariants::Other,
        );
        assert_eq!(
            super::CaseSubjectVariants::from_gleam(type_::unbound_var(0).as_ref()),
            super::CaseSubjectVariants::Other,
        );

        let linked = Arc::new(Type::Var {
            type_: Arc::new(RefCell::new(TypeVar::Link {
                type_: type_::list(type_::result(type_::int(), type_::string())),
            })),
        });
        assert_eq!(
            super::CaseSubjectVariants::from_gleam(linked.as_ref()),
            super::CaseSubjectVariants::List(Box::new(super::CaseSubjectVariants::Custom)),
        );
    }

    #[test]
    fn case_subject_variant_accessors_reject_other_shapes() {
        use super::CaseSubjectVariants as Variants;

        assert_eq!(Variants::Other.into_tuple(), None);
        assert_eq!(Variants::Custom.into_tuple(), None);
        assert_eq!(Variants::List(Box::new(Variants::Other)).into_tuple(), None);

        assert_eq!(Variants::Other.into_list(), None);
        assert_eq!(Variants::Custom.into_list(), None);
        assert_eq!(Variants::Tuple(Vec::new()).into_list(), None);
    }

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

        let true_ = ListExpr::value(Vec::new(), ValueType::BitArray)
            .into_bit_array()
            .expect("bit array list should build BitArrayListExpr");
        let false_ = ListExpr::value(Vec::new(), ValueType::BitArray)
            .into_bit_array()
            .expect("bit array list should build BitArrayListExpr");
        assert_eq!(
            super::bool_list_case_branches(
                ListExpr::BitArray(true_.clone()),
                ListExpr::BitArray(false_.clone()),
            ),
            Ok(BoolListCaseBranches::BitArray { true_, false_ }),
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
                return_shape: &crate::plan::ValueShape::String,
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
