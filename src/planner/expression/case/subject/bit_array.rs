use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{BitArrayExpr, BitArrayLocalId, BoolExpr, Expr, ExprKind, Step, ValueType};
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
    let subject = plan_expr_with_expected_source_stop_type(subject, ValueType::BitArray, context)?;
    let return_type = case_return_type(type_.as_ref())?;
    let ExprKind::BitArray(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let pattern = plan_pattern(pattern)?;
            let bindings = super::branch_bindings(&pattern.bound_names, subject.clone());
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

struct BitArrayCasePattern {
    bound_names: Vec<EcoString>,
}

fn plan_pattern(pattern: Pattern<Arc<Type>>) -> Result<BitArrayCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if type_.is_bit_array() => Ok(BitArrayCasePattern {
            bound_names: vec![name],
        }),
        Pattern::Discard { type_, .. } if type_.is_bit_array() => Ok(BitArrayCasePattern {
            bound_names: Vec::new(),
        }),
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_pattern(*pattern)?;
            pattern.bound_names.push(name);
            Ok(pattern)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::BitArraySize(_) | Pattern::BitArray { .. } => {
            super::unsupported_bit_array_pattern()
        }
        Pattern::Variable { .. }
        | Pattern::Discard { .. }
        | Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn bind_subject(subject: BitArrayExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_bit_array_local();
    let name = internal_subject_name(local);
    (
        Step::let_bit_array(local, name.clone(), subject),
        Expr::bit_array(BitArrayExpr::local_get(local, name)),
    )
}

fn internal_subject_name(local: BitArrayLocalId) -> EcoString {
    format!("<case:bit_array:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{BitArrayExpr, BitArrayLocalId, BitArraySegment, Endianness, IntExpr, Step};
    use crate::planner::dsl::{function, int, int_return_block, int_return_expr, module};
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use gleam_core::ast::Pattern;

    #[test]
    fn plan_bit_array_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    value as alias -> 1
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    int_return_block(
                        [
                            Step::let_bit_array(
                                BitArrayLocalId(1),
                                "value".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
                            Step::let_bit_array(
                                BitArrayLocalId(2),
                                "alias".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
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
    fn reject_margin_bit_array_subject_invalid_and_mismatched_patterns() {
        let mut invalid = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: gleam_core::type_::bit_array(),
        };
        assert_eq!(
            plan_module(invalid),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        };
        assert_eq!(
            plan_module(mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut subject_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut subject_mismatch.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_core::type_::bit_array(),
            value: "1".into(),
            int_value: 1.into(),
        };
        assert_eq!(
            plan_module(subject_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_bit_array_subject_clause_error_during_ordered_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case <<1>> {
    _ -> echo 1
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }
}
