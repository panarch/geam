use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{case_return_type, single_case_pattern, validate_clause_shape};
use crate::plan::{BoolExpr, Expr, ExprKind, Step, TupleExpr, TupleLocalId, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
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
        let pattern = plan_tuple_case_pattern(pattern, &subject_value_type)?;
        let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
        let is_total = clause.guard.is_none();
        ordered_clauses.push(super::plan_ordered_case_clause(
            super::OrderedCaseClauseInput {
                case_type: type_.as_ref(),
                return_type: &return_type,
                then: clause.then,
                branch_bindings: bindings,
                guard: clause.guard,
                match_condition: BoolExpr::value(true),
                is_total,
            },
            context,
        )?);
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TupleCasePattern {
    bound_names: Vec<EcoString>,
}

impl TupleCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_tuple_case_pattern(
    pattern: Pattern<Arc<Type>>,
    subject_type: &ValueType,
) -> Result<TupleCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(TupleCasePattern {
                bound_names: vec![name],
            })
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(TupleCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_tuple_case_pattern(*pattern, subject_type)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
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
        function, int, int_return_block, int_return_expr, let_tuple_step, local_tuple, module,
        tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
    };
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
    fn reject_profile_tuple_subject_structural_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #(1, 2) {
    #(left, right) -> left + right
  }
}
"#,
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            },
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
                &tuple_type,
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
                &tuple_type,
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
                &tuple_type,
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
                &tuple_type,
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
                &tuple_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
    }
}
