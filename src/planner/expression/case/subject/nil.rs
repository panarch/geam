use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{BoolExpr, Expr, ExprKind, NilExpr, NilLocalId, Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_type(subject, ValueType::Nil, context)?;
    let return_type = case_return_type(type_.as_ref())?;

    let ExprKind::Nil(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_nil_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let pattern = plan_nil_case_pattern(pattern)?;
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
struct NilCasePattern {
    bound_names: Vec<EcoString>,
}

impl NilCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_nil_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<NilCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if type_.is_nil() => Ok(NilCasePattern {
            bound_names: vec![name],
        }),
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_nil() => Ok(NilCasePattern {
            bound_names: Vec::new(),
        }),
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_nil_case_pattern(*pattern)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if name == "Nil" && arguments.is_empty() && spread.is_none() && type_.is_nil() => {
            Ok(NilCasePattern {
                bound_names: Vec::new(),
            })
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

fn bind_nil_case_subject(subject: NilExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_nil_local();
    let name = internal_nil_case_subject_name(local);
    (
        Step::let_nil(local, name.clone(), subject),
        Expr::nil(NilExpr::local_get(local, name)),
    )
}

fn internal_nil_case_subject_name(local: NilLocalId) -> EcoString {
    format!("<case:nil:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{
        function, int, int_return_block, int_return_expr, let_nil_step, local_nil, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
    use gleam_core::type_::error::VariableOrigin;

    #[test]
    fn plan_nil_subject_constructor_pattern_binds_internal_subject_once() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case Nil {
    Nil -> 1
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_nil_step(0, "<case:nil:0>", nil())],
                    int_return_expr(int(1)),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nil_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case Nil {
    value as alias -> 1
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_nil_step(0, "<case:nil:0>", nil())],
                    int_return_block(
                        [
                            let_nil_step(1, "value", local_nil(0, "<case:nil:0>")),
                            let_nil_step(2, "alias", local_nil(0, "<case:nil:0>")),
                        ],
                        int_return_expr(int(1)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_nil_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case echo Nil {
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
    fn reject_profile_nil_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case Nil {
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
    fn reject_margin_nil_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case Nil {
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
  case Nil {
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

        let mut pattern_type_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case Nil {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut subject_expression_family_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case Nil {
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
            type_: gleam_core::type_::nil(),
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
    fn reject_margin_nil_case_pattern_mismatched_and_invalid_shapes() {
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Other".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: gleam_core::type_::nil(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_: gleam_core::type_::int(),
                origin: VariableOrigin::generated(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                }),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_core::type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Invalid {
                location: dummy_span(),
                type_: gleam_core::type_::nil(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: num_bigint::BigInt::from(1),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_core::ast::Pattern::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }
}
