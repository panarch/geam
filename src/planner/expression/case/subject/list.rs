use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::super::{list_index_expr, tuple_index_expr};
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{
    BoolExpr, Expr, ExprKind, FloatExpr, IntExpr, ListExpr, ListLocal, Step, StringExpr, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedExpressionKind};
use ecow::EcoString;
use gleam_core::ast::{AssignName, Pattern, SrcSpan, TailPattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    element_type: ValueType,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject_value_type = ValueType::List(Box::new(element_type.clone()));
    let subject =
        plan_expr_with_expected_source_stop_type(subject, subject_value_type.clone(), context)?;
    let return_type = case_return_type(type_.as_ref())?;

    let ExprKind::List(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_list_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let pattern =
                plan_list_case_pattern(pattern, subject.clone(), subject_value_type.clone())?;
            let is_total = pattern.is_total() && clause.guard.is_none();
            let match_condition = pattern.match_condition();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_type: &return_type,
                    then: clause.then.clone(),
                    branch_bindings: pattern.branch_bindings,
                    guard: clause.guard.clone(),
                    match_condition,
                    is_total,
                },
                context,
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ListCasePattern {
    match_condition: Option<BoolExpr>,
    branch_bindings: Vec<(EcoString, Expr)>,
    is_total: bool,
}

impl ListCasePattern {
    fn any() -> Self {
        Self {
            match_condition: None,
            branch_bindings: Vec::new(),
            is_total: true,
        }
    }

    fn literal(value: Expr, literal: Expr) -> Self {
        Self {
            match_condition: Some(BoolExpr::equal(value, literal)),
            branch_bindings: Vec::new(),
            is_total: false,
        }
    }

    fn string_prefix(
        value: Expr,
        prefix: EcoString,
        left_side_assignment: Option<(EcoString, SrcSpan)>,
        right_side_assignment: AssignName,
    ) -> Result<Self, PlanError> {
        let Some(value) = value.into_string() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::PatternTypeMismatch,
            ));
        };
        let mut pattern = Self {
            match_condition: Some(BoolExpr::string_starts_with(value.clone(), prefix.clone())),
            branch_bindings: Vec::new(),
            is_total: false,
        };
        let prefix_binding = left_side_assignment
            .map(|(name, _)| (name, Expr::string(StringExpr::value(prefix.clone()))));
        let suffix_binding = match right_side_assignment {
            AssignName::Variable(name) => {
                Some((name, Expr::string(StringExpr::drop_prefix(value, prefix))))
            }
            AssignName::Discard(_) => None,
        };
        pattern.branch_bindings.extend(prefix_binding);
        pattern.branch_bindings.extend(suffix_binding);

        Ok(pattern)
    }

    fn with_binding(mut self, name: EcoString, value: Expr) -> Self {
        self.branch_bindings.push((name, value));
        self
    }

    fn is_total(&self) -> bool {
        self.is_total
    }

    fn match_condition(&self) -> BoolExpr {
        self.match_condition
            .clone()
            .unwrap_or_else(|| BoolExpr::value(true))
    }

    pub(super) fn into_parts(self) -> (Option<BoolExpr>, Vec<(EcoString, Expr)>, bool) {
        (self.match_condition, self.branch_bindings, self.is_total)
    }
}

pub(super) fn plan_list_case_pattern(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<ListCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if matches_type(type_.as_ref(), &subject_type) => {
            Ok(ListCasePattern::any().with_binding(name, value))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), &subject_type) => {
            Ok(ListCasePattern::any())
        }
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_list_case_pattern(*pattern, value.clone(), subject_type)?;
            Ok(pattern.with_binding(name, value))
        }
        Pattern::List {
            elements,
            tail,
            type_,
            ..
        } => plan_list_structural_case_pattern(
            elements,
            tail.map(|tail| *tail),
            type_,
            value,
            subject_type,
        ),
        Pattern::Tuple { elements, .. } => plan_tuple_case_pattern(elements, value, subject_type),
        Pattern::Int { int_value, .. } if subject_type == ValueType::Int => Ok(
            ListCasePattern::literal(value, Expr::int(IntExpr::value(int_value))),
        ),
        Pattern::Float { float_value, .. } if subject_type == ValueType::Float => Ok(
            ListCasePattern::literal(value, Expr::float(FloatExpr::value(float_value.value()))),
        ),
        Pattern::String { value: literal, .. } if subject_type == ValueType::String => Ok(
            ListCasePattern::literal(value, Expr::string(StringExpr::value(literal))),
        ),
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => {
            match (name.as_str(), subject_type) {
                ("True", ValueType::Bool) => Ok(ListCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(true)),
                )),
                ("False", ValueType::Bool) => Ok(ListCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(false)),
                )),
                _ => Err(invalid_case_shape(
                    InvalidCaseShapeReason::PatternTypeMismatch,
                )),
            }
        }
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if name == "Nil" && arguments.is_empty() && spread.is_none() && type_.is_nil() => {
            if subject_type == ValueType::Nil {
                Ok(ListCasePattern::any())
            } else {
                Err(invalid_case_shape(
                    InvalidCaseShapeReason::PatternTypeMismatch,
                ))
            }
        }
        Pattern::StringPrefix {
            left_side_string,
            left_side_assignment,
            right_side_assignment,
            ..
        } if subject_type == ValueType::String => ListCasePattern::string_prefix(
            value,
            left_side_string,
            left_side_assignment,
            right_side_assignment,
        ),
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::BitArraySize(_) | Pattern::BitArray { .. } => {
            super::unsupported_bit_array_pattern()
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Constructor { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn plan_list_structural_case_pattern(
    elements: Vec<Pattern<Arc<Type>>>,
    tail: Option<TailPattern<Arc<Type>>>,
    type_: Arc<Type>,
    value: Expr,
    subject_type: ValueType,
) -> Result<ListCasePattern, PlanError> {
    let pattern_type =
        ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::UnsupportedListElementType,
        })?;
    if pattern_type != subject_type {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    }
    let ValueType::List(element_type) = subject_type else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let element_type = *element_type;
    let Some(list) = value.into_list() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };

    let element_count = elements.len();
    let mut patterns = Vec::with_capacity(elements.len());
    for (index, pattern) in elements.into_iter().enumerate() {
        let value = list_index_expr(list.clone(), index, element_type.clone())?;
        patterns.push(plan_list_case_pattern(
            pattern,
            value,
            element_type.clone(),
        )?);
    }

    let mut pattern = combine_list_case_patterns(patterns);
    pattern.match_condition = combine_conditions(
        list_length_condition(list.clone(), element_count, tail.is_some()),
        pattern.match_condition,
    );
    pattern.is_total = tail.is_some() && element_count == 0 && pattern.is_total;
    if let Some(tail) = tail
        && let Some(binding) = plan_list_tail_binding(tail, &element_type, list, element_count)?
    {
        pattern.branch_bindings.push(binding);
    }

    Ok(pattern)
}

fn plan_tuple_case_pattern(
    elements: Vec<Pattern<Arc<Type>>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<ListCasePattern, PlanError> {
    let ValueType::Tuple(element_types) = subject_type else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    if elements.len() != element_types.len() {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    }
    let Some(tuple) = value.into_tuple() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };

    let mut patterns = Vec::with_capacity(elements.len());
    for (index, (pattern, type_)) in elements.into_iter().zip(element_types).enumerate() {
        let value = tuple_index_expr(tuple.clone(), index, type_.clone());
        patterns.push(plan_list_case_pattern(pattern, value, type_)?);
    }

    Ok(combine_list_case_patterns(patterns))
}

fn plan_list_tail_binding(
    tail: TailPattern<Arc<Type>>,
    element_type: &ValueType,
    list: ListExpr,
    element_count: usize,
) -> Result<Option<(EcoString, Expr)>, PlanError> {
    match tail.pattern {
        Pattern::Variable { name, type_, .. } => {
            assert_list_tail_type_matches(type_.as_ref(), element_type)?;
            Ok(Some((
                name,
                Expr::list(ListExpr::drop_first(list, element_count)),
            )))
        }
        Pattern::Discard { type_, .. } => {
            assert_list_tail_type_matches(type_.as_ref(), element_type)?;
            Ok(None)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        _ => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn assert_list_tail_type_matches(type_: &Type, element_type: &ValueType) -> Result<(), PlanError> {
    if ValueType::from_gleam(type_) == Some(ValueType::List(Box::new(element_type.clone()))) {
        Ok(())
    } else {
        Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ))
    }
}

fn combine_list_case_patterns(patterns: Vec<ListCasePattern>) -> ListCasePattern {
    let mut combined = ListCasePattern::any();
    for pattern in patterns {
        combined.match_condition =
            combine_conditions(combined.match_condition, pattern.match_condition);
        combined.branch_bindings.extend(pattern.branch_bindings);
        combined.is_total &= pattern.is_total;
    }

    combined
}

fn combine_conditions(left: Option<BoolExpr>, right: Option<BoolExpr>) -> Option<BoolExpr> {
    match (left, right) {
        (Some(left), Some(right)) => Some(BoolExpr::and(left, right)),
        (Some(condition), None) | (None, Some(condition)) => Some(condition),
        (None, None) => None,
    }
}

fn list_length_condition(list: ListExpr, length: usize, has_tail: bool) -> Option<BoolExpr> {
    match (has_tail, length) {
        (true, 0) => None,
        (true, _) => Some(BoolExpr::list_length_at_least(list, length)),
        (false, _) => Some(BoolExpr::list_length_equals(list, length)),
    }
}

fn matches_type(type_: &Type, subject_type: &ValueType) -> bool {
    ValueType::from_gleam(type_) == Some(subject_type.clone())
}

fn bind_list_case_subject(subject: ListExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let (local, value) = context.define_internal_list_value(subject);
    let name = internal_list_case_subject_name(&local);
    (
        Step::let_list_expr(name.clone(), value),
        Expr::list(ListExpr::local_get(local, name)),
    )
}

fn internal_list_case_subject_name(local: &ListLocal) -> EcoString {
    format!("<case:list:{}:{}>", local.family_name(), local.index()).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, IntListLocalId, IntLocalId, ListExpr, ListLocal, Step, StringExpr,
        ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_block, bool_return_expr, equal, function, int, int_return_block,
        int_return_expr, let_list_step, list, local_int, local_list, module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::type_::error::VariableOrigin;

    #[test]
    fn plan_list_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    value as alias -> value == alias
  }
}
"#,
        ))
        .expect("source should plan");
        let value = local_list(1, "value", ValueType::Int);
        let alias = local_list(2, "alias", ValueType::Int);
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    bool_return_block(
                        [
                            let_list_step(
                                1,
                                "value",
                                local_list(0, "<case:list:int:0>", ValueType::Int),
                            ),
                            let_list_step(
                                2,
                                "alias",
                                local_list(0, "<case:list:int:0>", ValueType::Int),
                            ),
                        ],
                        bool_return_expr(equal(value, alias)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_subject_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    value if value == [] -> False
    value as alias if alias == [1, 2] -> value == alias
    _ -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let second_value = local_list(2, "value", ValueType::Int);
        let second_alias = local_list(3, "alias", ValueType::Int);
        let first_binding = let_list_step(
            1,
            "value",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let second_value_binding = let_list_step(
            2,
            "value",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let second_alias_binding = let_list_step(
            3,
            "alias",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let first_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![first_binding.clone()],
                BoolExpr::equal(
                    Expr::from(local_list(1, "value", ValueType::Int)),
                    Expr::from(list(Vec::<Expr>::new(), ValueType::Int)),
                ),
            ),
        );
        let second_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![second_value_binding.clone(), second_alias_binding.clone()],
                BoolExpr::equal(
                    Expr::from(local_list(3, "alias", ValueType::Int)),
                    Expr::from(list([int(1), int(2)], ValueType::Int)),
                ),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    crate::plan::BoolReturn::bool_case(
                        first_condition,
                        bool_return_block([first_binding], bool_return_expr(bool_(false))),
                        crate::plan::BoolReturn::bool_case(
                            second_condition,
                            bool_return_block(
                                [second_value_binding, second_alias_binding],
                                bool_return_expr(equal(second_value, second_alias)),
                            ),
                            bool_return_expr(bool_(false)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_subject_structural_pattern_binds_element_after_length_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    [first, ..] -> first
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "<case:list:int:0>".into(),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::list_length_at_least(subject.clone(), 1),
                        int_return_block(
                            [Step::let_int(
                                IntLocalId(0),
                                "first".into(),
                                crate::plan::IntExpr::list_index(subject, 0),
                            )],
                            int_return_expr(local_int(0, "first")),
                        ),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_subject_tail_binding_uses_drop_first_after_length_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2, 3] {
    [first, ..rest] -> rest == [2, 3]
    _ -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "<case:list:int:0>".into(),
        );
        let rest = local_list(1, "rest", ValueType::Int);
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2), int(3)], ValueType::Int),
                    )],
                    crate::plan::BoolReturn::bool_case(
                        BoolExpr::list_length_at_least(subject.clone(), 1),
                        bool_return_block(
                            [
                                Step::let_int(
                                    IntLocalId(0),
                                    "first".into(),
                                    crate::plan::IntExpr::list_index(subject.clone(), 0),
                                ),
                                Step::let_list_expr(
                                    "rest".into(),
                                    crate::plan::ListLocalExpr::Int {
                                        local: IntListLocalId(1),
                                        value: ListExpr::drop_first(subject.clone(), 1)
                                            .into_int()
                                            .expect("expected int list"),
                                    },
                                ),
                            ],
                            bool_return_expr(equal(rest, list([int(2), int(3)], ValueType::Int))),
                        ),
                        bool_return_expr(bool_(false)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_list_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case echo [1] {
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: crate::planner::UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_profile_list_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case [1] {
    _ -> echo 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: crate::planner::UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_list_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
    _ -> 1
  }
}
"#,
        );
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
            &mut unsupported_case_type.definitions.functions[0].body[0],
        );
        *case_type = gleam_core::type_::bit_array();
        assert_eq!(
            plan_module(unsupported_case_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut empty_pattern = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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

        let mut subject_expression_family_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
    _ -> 1
  }
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut subject_expression_family_mismatch.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_core::type_::list(gleam_core::type_::int()),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(subject_expression_family_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut invalid_pattern_during_case_lowering = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern_during_case_lowering.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(invalid_pattern_during_case_lowering),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_case_pattern_mismatched_and_invalid_shapes() {
        let list_type = ValueType::List(Box::new(ValueType::Int));
        let subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(gleam_core::ast::Pattern::Variable {
                        location: dummy_span(),
                        name: "value".into(),
                        type_: gleam_core::type_::int(),
                        origin: VariableOrigin::generated(),
                    }),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Invalid {
                    location: dummy_span(),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                subject,
                list_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_structural_element_projection_type_mismatch() {
        let int_list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::string(),
                    }],
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::string()),
                },
                int_list_subject,
                ValueType::List(Box::new(ValueType::String)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn list_case_pattern_binds_string_prefix_parts() {
        let value = Expr::string(StringExpr::value("Hello, Geam".into()));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: Some(("prefix".into(), dummy_span())),
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Variable("name".into()),
                },
                value.clone(),
                ValueType::String,
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::string_starts_with(
                    StringExpr::value("Hello, Geam".into()),
                    "Hello, ".into(),
                )),
                branch_bindings: vec![
                    (
                        "prefix".into(),
                        Expr::string(StringExpr::value("Hello, ".into()))
                    ),
                    (
                        "name".into(),
                        Expr::string(StringExpr::drop_prefix(
                            value.into_string().expect("value should be string"),
                            "Hello, ".into(),
                        )),
                    ),
                ],
                is_total: false,
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Discard("_rest".into()),
                },
                Expr::string(StringExpr::value("Hello, Geam".into())),
                ValueType::String,
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::string_starts_with(
                    StringExpr::value("Hello, Geam".into()),
                    "Hello, ".into(),
                )),
                branch_bindings: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn list_case_pattern_literal_builds_match_condition() {
        assert_eq!(
            super::ListCasePattern::literal(int(1).into(), int(1).into()),
            super::ListCasePattern {
                match_condition: Some(BoolExpr::equal(int(1).into(), int(1).into())),
                branch_bindings: Vec::new(),
                is_total: false,
            },
        );
    }

    #[test]
    fn list_case_pattern_supports_literal_leaf_families() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: gleam_core::parse::LiteralFloatValue::ONE,
                },
                crate::planner::dsl::float(1.0).into(),
                ValueType::Float,
            ),
            Ok(super::ListCasePattern::literal(
                crate::planner::dsl::float(1.0).into(),
                crate::planner::dsl::float(1.0).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::String {
                    location: dummy_span(),
                    value: "one".into(),
                },
                crate::planner::dsl::string("one").into(),
                ValueType::String,
            ),
            Ok(super::ListCasePattern::literal(
                crate::planner::dsl::string("one").into(),
                crate::planner::dsl::string("one").into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "True".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                bool_(true).into(),
                ValueType::Bool,
            ),
            Ok(super::ListCasePattern::literal(
                bool_(true).into(),
                bool_(true).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "False".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                bool_(false).into(),
                ValueType::Bool,
            ),
            Ok(super::ListCasePattern::literal(
                bool_(false).into(),
                bool_(false).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Nil".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::nil(),
                },
                crate::planner::dsl::nil().into(),
                ValueType::Nil,
            ),
            Ok(super::ListCasePattern::any()),
        );
    }

    #[test]
    fn list_structural_case_pattern_builds_fixed_length_condition() {
        let list_subject = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::list(list_subject.clone()),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::list_length_equals(list_subject, 1)),
                branch_bindings: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn reject_margin_list_structural_pattern_shapes() {
        let list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        let list_type = ValueType::List(Box::new(ValueType::Int));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::result(
                        gleam_core::type_::int(),
                        gleam_core::type_::nil(),
                    )),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::UnsupportedExpression {
                kind: crate::planner::UnsupportedExpressionKind::UnsupportedListElementType,
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::string()),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::int(),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn list_case_pattern_binds_tail_shapes() {
        let list_subject = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Variable {
                            location: dummy_span(),
                            name: "rest".into(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                            origin: VariableOrigin::generated(),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::list(list_subject.clone()),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: None,
                branch_bindings: vec![(
                    "rest".into(),
                    Expr::list(ListExpr::drop_first(list_subject.clone(), 0)),
                )],
                is_total: true,
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::list(list_subject),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: None,
                branch_bindings: Vec::new(),
                is_total: true,
            }),
        );
    }

    #[test]
    fn reject_margin_list_structural_tail_shapes() {
        let list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        let list_type = ValueType::List(Box::new(ValueType::Int));

        for tail_pattern in [
            gleam_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "rest".into(),
                type_: gleam_core::type_::list(gleam_core::type_::string()),
                origin: VariableOrigin::generated(),
            },
            gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_core::type_::list(gleam_core::type_::string()),
            },
        ] {
            assert_eq!(
                super::plan_list_case_pattern(
                    gleam_core::ast::Pattern::List {
                        location: dummy_span(),
                        elements: Vec::new(),
                        tail: Some(Box::new(gleam_core::ast::TailPattern {
                            location: dummy_span(),
                            pattern: tail_pattern,
                        })),
                        type_: gleam_core::type_::list(gleam_core::type_::int()),
                    },
                    list_subject.clone(),
                    list_type.clone(),
                ),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CaseShape {
                        reason: InvalidCaseShapeReason::PatternTypeMismatch,
                    },
                }),
            );
        }

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Invalid {
                            location: dummy_span(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: num_bigint::BigInt::from(1),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                list_subject,
                list_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn list_tuple_case_pattern_combines_literal_match_conditions() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: num_bigint::BigInt::from(1),
                        },
                        gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "2".into(),
                            int_value: num_bigint::BigInt::from(2),
                        },
                    ],
                },
                crate::planner::dsl::tuple([int(1), int(2)]).into(),
                ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::and(
                    BoolExpr::equal(
                        crate::planner::dsl::tuple([int(1), int(2)])
                            .index_int(0)
                            .into(),
                        int(1).into(),
                    ),
                    BoolExpr::equal(
                        crate::planner::dsl::tuple([int(1), int(2)])
                            .index_int(1)
                            .into(),
                        int(2).into(),
                    ),
                )),
                branch_bindings: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn reject_margin_list_nested_tuple_pattern_shapes() {
        let tuple_type = ValueType::Tuple(vec![ValueType::Int]);
        let tuple_value = Expr::tuple(crate::plan::TupleExpr::value(
            vec![Expr::int(crate::plan::IntExpr::value(1.into()))],
            vec![ValueType::Int],
        ));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: Vec::new(),
                },
                tuple_value.clone(),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::int(),
                        },
                        gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::int(),
                        },
                    ],
                },
                tuple_value.clone(),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Tuple {
                        location: dummy_span(),
                        elements: Vec::new(),
                    }],
                },
                tuple_value,
                tuple_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn list_case_pattern_helpers_preserve_condition_shapes() {
        let left = BoolExpr::local_get(crate::plan::BoolLocalId(0), "left".into());
        let right = BoolExpr::local_get(crate::plan::BoolLocalId(1), "right".into());
        assert_eq!(
            super::combine_conditions(Some(left.clone()), Some(right.clone())),
            Some(BoolExpr::and(left.clone(), right.clone())),
        );
        assert_eq!(
            super::combine_conditions(Some(left.clone()), None),
            Some(left),
        );
        assert_eq!(
            super::combine_conditions(None, Some(right.clone())),
            Some(right),
        );
        assert_eq!(super::combine_conditions(None, None), None);

        let list = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());
        assert_eq!(super::list_length_condition(list.clone(), 0, true), None);
        assert_eq!(
            super::list_length_condition(list.clone(), 1, true),
            Some(BoolExpr::list_length_at_least(list.clone(), 1)),
        );
        assert_eq!(
            super::list_length_condition(list.clone(), 2, false),
            Some(BoolExpr::list_length_equals(list, 2)),
        );
    }

    #[test]
    fn reject_margin_list_element_special_patterns_with_wrong_subject_shape() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: Some(("prefix".into(), dummy_span())),
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Variable("name".into()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::String,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "True".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Nil".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::nil(),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }
}
