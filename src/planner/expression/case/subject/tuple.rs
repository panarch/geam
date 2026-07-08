use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::super::tuple_index_expr;
use super::super::invalid_case_shape;
use super::{case_return_type, single_case_pattern, validate_clause_shape};
use crate::plan::{
    BoolExpr, Expr, ExprKind, FloatExpr, IntExpr, Step, StringExpr, TupleExpr, TupleLocalId,
    ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    subject_type: Vec<ValueType>,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject_value_type = ValueType::Tuple(subject_type.clone());
    let subject =
        plan_expr_with_expected_source_stop_type(subject, subject_value_type.clone(), context)?;
    let return_type = case_return_type(type_.as_ref())?;
    for clause in &clauses {
        validate_clause_shape(clause)?;
    }

    let ExprKind::Tuple(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_tuple_case_subject(subject, context);
    let mut ordered_clauses = Vec::with_capacity(clauses.len());
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern =
            plan_tuple_case_pattern(pattern, subject.clone(), subject_value_type.clone())?;
        let is_total = pattern.is_total() && clause.guard.is_none();
        let match_condition = pattern.match_condition();
        ordered_clauses.push(super::plan_ordered_case_clause(
            super::OrderedCaseClauseInput {
                case_type: type_.as_ref(),
                return_type: &return_type,
                then: clause.then,
                branch_bindings: pattern.branch_bindings,
                guard: clause.guard,
                match_condition,
                is_total,
            },
            context,
        )?);
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq)]
struct TupleCasePattern {
    match_condition: Option<BoolExpr>,
    branch_bindings: Vec<(EcoString, Expr)>,
    is_total: bool,
}

impl TupleCasePattern {
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
}

fn plan_tuple_case_pattern(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<TupleCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if matches_type(type_.as_ref(), &subject_type) => {
            Ok(TupleCasePattern::any().with_binding(name, value))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), &subject_type) => {
            Ok(TupleCasePattern::any())
        }
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_tuple_case_pattern(*pattern, value.clone(), subject_type)?;
            Ok(pattern.with_binding(name, value))
        }
        Pattern::Tuple { elements, .. } => {
            plan_tuple_structural_case_pattern(elements, value, subject_type)
        }
        Pattern::Int { int_value, .. } if subject_type == ValueType::Int => Ok(
            TupleCasePattern::literal(value, Expr::int(IntExpr::value(int_value))),
        ),
        Pattern::Float { float_value, .. } if subject_type == ValueType::Float => Ok(
            TupleCasePattern::literal(value, Expr::float(FloatExpr::value(float_value.value()))),
        ),
        Pattern::String { value: literal, .. } if subject_type == ValueType::String => Ok(
            TupleCasePattern::literal(value, Expr::string(StringExpr::value(literal))),
        ),
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => {
            match (name.as_str(), subject_type) {
                ("True", ValueType::Bool) => Ok(TupleCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(true)),
                )),
                ("False", ValueType::Bool) => Ok(TupleCasePattern::literal(
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
                Ok(TupleCasePattern::any())
            } else {
                Err(invalid_case_shape(
                    InvalidCaseShapeReason::PatternTypeMismatch,
                ))
            }
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::List { .. } if matches!(subject_type, ValueType::List(_)) => Err(
            super::super::unsupported_case(UnsupportedCaseReason::ListPattern),
        ),
        Pattern::StringPrefix { .. } if subject_type == ValueType::String => Err(
            super::super::unsupported_case(UnsupportedCaseReason::StringPrefixPattern),
        ),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Constructor { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn plan_tuple_structural_case_pattern(
    elements: Vec<Pattern<Arc<Type>>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<TupleCasePattern, PlanError> {
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
        patterns.push(plan_tuple_case_pattern(pattern, value, type_)?);
    }

    Ok(combine_tuple_case_patterns(patterns))
}

fn combine_tuple_case_patterns(patterns: Vec<TupleCasePattern>) -> TupleCasePattern {
    let mut combined = TupleCasePattern::any();
    for pattern in patterns {
        combined.match_condition = match (combined.match_condition, pattern.match_condition) {
            (Some(left), Some(right)) => Some(BoolExpr::and(left, right)),
            (Some(condition), None) | (None, Some(condition)) => Some(condition),
            (None, None) => None,
        };
        combined.branch_bindings.extend(pattern.branch_bindings);
        combined.is_total &= pattern.is_total;
    }

    combined
}

fn matches_type(type_: &Type, subject_type: &ValueType) -> bool {
    ValueType::from_gleam(type_) == Some(subject_type.clone())
}

fn bind_tuple_case_subject(subject: TupleExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_tuple_local();
    let name = internal_tuple_case_subject_name(local);
    let type_ = subject.type_().to_vec();
    (
        Step::let_tuple(local, name.clone(), subject),
        Expr::tuple(TupleExpr::local_get(local, name, type_)),
    )
}

fn internal_tuple_case_subject_name(local: TupleLocalId) -> EcoString {
    format!("<case:tuple:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, Expr, ValueType};
    use crate::planner::dsl::{
        bool_, float, function, int, int_return_block, int_return_expr, let_int_step,
        let_tuple_step, local_int, local_tuple, module, nil, string, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
    };
    use gleam_core::ast::{AssignName, Pattern};
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::error::VariableOrigin;

    #[test]
    fn plan_tuple_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    value as alias -> value.0 + alias.1
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let value = local_tuple(1, "value", tuple_type.clone());
        let alias = local_tuple(2, "alias", tuple_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    int_return_block(
                        [
                            let_tuple_step(
                                1,
                                "value",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
                            ),
                            let_tuple_step(
                                2,
                                "alias",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
                            ),
                        ],
                        int_return_expr(value.index_int(0).add_int(alias.index_int(1))),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    value if value.0 > 10 -> 0
    value as alias if alias.1 == 2 -> value.0 + alias.1
    _ -> 999
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let second_value = local_tuple(2, "value", tuple_type.clone());
        let second_alias = local_tuple(3, "alias", tuple_type.clone());
        let first_binding = let_tuple_step(
            1,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let second_value_binding = let_tuple_step(
            2,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let second_alias_binding = let_tuple_step(
            3,
            "alias",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let first_condition = BoolExpr::block(
            vec![first_binding.clone()],
            BoolExpr::and(
                BoolExpr::value(true),
                BoolExpr::gt_int(
                    local_tuple(1, "value", tuple_type.clone())
                        .index_int(0)
                        .into(),
                    int(10).into(),
                ),
            ),
        );
        let second_condition = BoolExpr::block(
            vec![second_value_binding.clone(), second_alias_binding.clone()],
            BoolExpr::and(
                BoolExpr::value(true),
                BoolExpr::equal(
                    Expr::from(local_tuple(3, "alias", tuple_type.clone()).index_int(1)),
                    Expr::from(int(2)),
                ),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(int(0))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_value_binding, second_alias_binding],
                                int_return_expr(
                                    second_value.index_int(0).add_int(second_alias.index_int(1)),
                                ),
                            ),
                            int_return_expr(int(999)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_structural_pattern_binds_element_projections() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    #(left, right) -> left + right
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
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
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
    fn plan_tuple_subject_literal_pattern_preserves_ordered_fallthrough_shape() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(2, 5) {
    #(1, value) -> value
    #(2, value) -> value + 10
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let first_binding = let_int_step(
            0,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let second_binding = let_int_step(
            1,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let first_condition = BoolExpr::block(
            vec![first_binding.clone()],
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
                Expr::from(int(1)),
            ),
        );
        let second_condition = BoolExpr::block(
            vec![second_binding.clone()],
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
                Expr::from(int(2)),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(2), int(5)]))],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(local_int(0, "value"))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_binding],
                                int_return_expr(local_int(1, "value").add_int(int(10))),
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
    fn plan_tuple_subject_structural_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    #(left, right) if left > 0 -> right
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let left_binding = let_int_step(
            0,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
        );
        let right_binding = let_int_step(
            1,
            "right",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let condition = BoolExpr::block(
            vec![left_binding.clone(), right_binding.clone()],
            BoolExpr::and(
                BoolExpr::value(true),
                BoolExpr::gt_int(local_int(0, "left").into(), int(0).into()),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    crate::plan::IntReturn::bool_case(
                        condition,
                        int_return_block(
                            [left_binding, right_binding],
                            int_return_expr(local_int(1, "right")),
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
    fn tuple_case_pattern_supports_literal_leaf_families() {
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                float(1.0).into(),
                ValueType::Float,
            ),
            Ok(super::TupleCasePattern::literal(
                float(1.0).into(),
                float(1.0).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::String {
                    location: dummy_span(),
                    value: "one".into(),
                },
                string("one").into(),
                ValueType::String,
            ),
            Ok(super::TupleCasePattern::literal(
                string("one").into(),
                string("one").into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
            Ok(super::TupleCasePattern::literal(
                bool_(true).into(),
                bool_(true).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
            Ok(super::TupleCasePattern::literal(
                bool_(false).into(),
                bool_(false).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
                nil().into(),
                ValueType::Nil,
            ),
            Ok(super::TupleCasePattern::any()),
        );
    }

    #[test]
    fn tuple_case_pattern_combines_literal_match_conditions() {
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let actual = super::plan_tuple_case_pattern(
            Pattern::Tuple {
                location: dummy_span(),
                elements: vec![
                    Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: num_bigint::BigInt::from(1),
                    },
                    Pattern::Int {
                        location: dummy_span(),
                        value: "2".into(),
                        int_value: num_bigint::BigInt::from(2),
                    },
                ],
            },
            tuple([int(1), int(2)]).into(),
            ValueType::Tuple(tuple_type.clone()),
        );

        assert_eq!(
            actual,
            Ok(super::TupleCasePattern {
                match_condition: Some(BoolExpr::and(
                    BoolExpr::equal(tuple([int(1), int(2)]).index_int(0).into(), int(1).into()),
                    BoolExpr::equal(tuple([int(1), int(2)]).index_int(1).into(), int(2).into()),
                )),
                branch_bindings: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn reject_profile_tuple_subject_inner_string_prefix_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #("Hello, Geam", 1) {
    #("Hello, " <> name, value) -> value
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::StringPrefixPattern,
            },
        );
    }

    #[test]
    fn reject_profile_tuple_subject_inner_list_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #([1, 2], 3) {
    #([first, ..], value) -> first + value
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::ListPattern,
            },
        );
    }

    #[test]
    fn reject_margin_tuple_subject_invalid_pattern_during_case_lowering() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: gleam_core::type_::tuple(vec![
                gleam_core::type_::int(),
                gleam_core::type_::int(),
            ]),
        };
        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_tuple_subject_alternative_patterns() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #(1, 2) {
    _ | _ -> 1
  }
}
"#,
            ),
            PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::AlternativePatterns,
            },
        );
    }

    #[test]
    fn reject_profile_tuple_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case echo #(1, 2) {
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
    fn reject_profile_tuple_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #(1, 2) {
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
    fn reject_margin_tuple_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
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
  case #(1, 2) {
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
  case #(1, 2) {
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
            type_: gleam_core::type_::tuple(vec![
                gleam_core::type_::int(),
                gleam_core::type_::int(),
            ]),
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
    }

    #[test]
    fn reject_margin_tuple_case_pattern_mismatched_and_invalid_shapes() {
        let tuple_type = ValueType::Tuple(vec![ValueType::Int]);
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "prefix".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Invalid {
                    location: dummy_span(),
                    type_: gleam_core::type_::tuple(vec![gleam_core::type_::int()]),
                },
                Expr::from(tuple([int(1)])),
                tuple_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
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
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: Vec::new(),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::from(tuple([int(1), int(2)])),
                ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::from(int(1)),
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }
}
