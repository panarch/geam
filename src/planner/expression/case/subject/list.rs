use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::{invalid_case_shape, unsupported_case};
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{BoolExpr, Expr, ExprKind, ListExpr, ListLocalId, Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
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
            let pattern = plan_list_case_pattern(pattern, &subject_value_type)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_type: &return_type,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
                },
                context,
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListCasePattern {
    bound_names: Vec<EcoString>,
}

impl ListCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_list_case_pattern(
    pattern: Pattern<Arc<Type>>,
    subject_type: &ValueType,
) -> Result<ListCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(ListCasePattern {
                bound_names: vec![name],
            })
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(ListCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_list_case_pattern(*pattern, subject_type)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::List { .. } => Err(unsupported_case(UnsupportedCaseReason::ListPattern)),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
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

fn bind_list_case_subject(subject: ListExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_list_local();
    let name = internal_list_case_subject_name(local);
    let element_type = subject.element_type().clone();
    (
        Step::let_list(local, name.clone(), subject),
        Expr::list(ListExpr::local_get(local, name, element_type)),
    )
}

fn internal_list_case_subject_name(local: ListLocalId) -> EcoString {
    format!("<case:list:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, Expr, ValueType};
    use crate::planner::dsl::{
        bool_, bool_return_block, bool_return_expr, equal, function, int, let_list_step, list,
        local_list, module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
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
                        "<case:list:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    bool_return_block(
                        [
                            let_list_step(
                                1,
                                "value",
                                local_list(0, "<case:list:0>", ValueType::Int),
                            ),
                            let_list_step(
                                2,
                                "alias",
                                local_list(0, "<case:list:0>", ValueType::Int),
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
        let first_binding =
            let_list_step(1, "value", local_list(0, "<case:list:0>", ValueType::Int));
        let second_value_binding =
            let_list_step(2, "value", local_list(0, "<case:list:0>", ValueType::Int));
        let second_alias_binding =
            let_list_step(3, "alias", local_list(0, "<case:list:0>", ValueType::Int));
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
                        "<case:list:0>",
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
    fn reject_profile_list_subject_structural_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case [1, 2] {
    [first, ..] -> first
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
    }

    #[test]
    fn reject_margin_list_case_pattern_mismatched_and_invalid_shapes() {
        let list_type = ValueType::List(Box::new(ValueType::Int));
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                &list_type,
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
                &list_type,
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
                &list_type,
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
                &list_type,
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
                &list_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
    }
}
