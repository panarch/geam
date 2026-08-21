use super::super::super::{
    conversion::expect_expression, plan_expr_with_expected_source_stop_type,
};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, NilExpr, NilLocalId, Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_compiler_core::ast::{Pattern, TypedExpr};
use gleam_compiler_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_type(subject, ValueType::Nil, context)?;
    let return_shape = context.value_shape(type_.as_ref());

    let subject: NilExpr = expect_expression(subject)?;
    let (subject_step, subject) = bind_nil_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_nil_case_pattern_with_context(pattern, context)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
                    reachable,
                    exhaustive_remainder,
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

fn plan_nil_case_pattern_with_context(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<NilCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::Nil,
                context,
            )?;
            Ok(NilCasePattern {
                bound_names: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::Nil,
                context,
            )?;
            Ok(NilCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_nil_case_pattern_with_context(*pattern, context)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        ref pattern @ Pattern::Constructor {
            ref name,
            ref arguments,
            ref spread,
            ref type_,
            ..
        } if name == "Nil" && arguments.is_empty() && spread.is_none() && type_.is_nil() => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::Nil,
                context,
            )?;
            Ok(NilCasePattern {
                bound_names: Vec::new(),
            })
        }
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &crate::plan::ValueShape::Nil,
            context,
        )),
    }
}

#[cfg(test)]
fn plan_nil_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<NilCasePattern, PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module_name, &functions, &mut anonymous);
    plan_nil_case_pattern_with_context(pattern, &context)
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
    use crate::plan::ValueType;
    use crate::planner::dsl::{
        function, int, int_return_block, int_return_expr, let_nil_step, local_nil, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_compiler_core::type_::error::VariableOrigin;

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
  case { <<1:native>> Nil } {
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
    fn reject_profile_nil_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case Nil {
    _ -> { <<1:native>> 0 }
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
        *case_type = super::super::mismatched_generic_case_return_type();
        assert_eq!(
            plan_module(unsupported_case_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Parameter(crate::plan::TypeParameterId(0)),
                        actual: ValueType::Int,
                    },
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 0,
                    },
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
        clauses[0].pattern[0] = gleam_compiler_core::ast::Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Int,
            )),
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
        subjects[0] = gleam_compiler_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_compiler_core::type_::nil(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(subject_expression_family_mismatch),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Nil,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn reject_margin_nil_case_pattern_mismatched_and_invalid_shapes() {
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Nil".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: gleam_compiler_core::type_::nil(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Other".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: gleam_compiler_core::type_::nil(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_: gleam_compiler_core::type_::int(),
                origin: VariableOrigin::generated(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(gleam_compiler_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_compiler_core::type_::int(),
                    origin: VariableOrigin::generated(),
                }),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_compiler_core::type_::int(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Invalid {
                location: dummy_span(),
                type_: gleam_compiler_core::type_::nil(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: num_bigint::BigInt::from(1),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_nil_case_pattern(gleam_compiler_core::ast::Pattern::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::Nil,
                ValueType::Tuple(Vec::new()),
            )),
        );
    }
}
