use super::super::super::plan_expr_with_expected_source_stop_shape;
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, ExprKind, GenericExpr, Step, TypeParameterId, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    parameter: TypeParameterId,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_shape(
        subject,
        ValueShape::Parameter(parameter),
        context,
    )?;
    let return_shape = context.value_shape(type_.as_ref());

    let actual = InvalidExpressionType::from_value_type(subject.value_type());
    let ExprKind::Generic(subject) = subject.into_kind() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::TypeParameter,
                actual,
            },
        });
    };
    let (subject_step, subject) = bind_generic_case_subject(subject, parameter, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_generic_case_pattern(pattern, parameter, context)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
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

    let case = super::ordered_case_expr(ordered_clauses)?;
    Ok(super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenericCasePattern {
    bound_names: Vec<EcoString>,
}

impl GenericCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_generic_case_pattern(
    pattern: Pattern<Arc<Type>>,
    parameter: TypeParameterId,
    context: &mut PlanContext<'_>,
) -> Result<GenericCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &ValueShape::Parameter(parameter),
                context,
            )?;
            Ok(GenericCasePattern {
                bound_names: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &ValueShape::Parameter(parameter),
                context,
            )?;
            Ok(GenericCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_generic_case_pattern(*pattern, parameter, context)?;
            pattern.add_bound_name(name);
            Ok(pattern)
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
            &ValueShape::Parameter(parameter),
            context,
        )),
    }
}

fn bind_generic_case_subject(
    subject: GenericExpr,
    parameter: TypeParameterId,
    context: &mut PlanContext<'_>,
) -> (Step, Expr) {
    let local = context.define_internal_generic_local(parameter);
    let name: EcoString = format!("<case:generic:{}>", local.id().0).into();
    (
        Step::let_generic(local, name.clone(), subject),
        Expr::generic(GenericExpr::local_get(local, name)),
    )
}

#[cfg(test)]
mod tests {
    use super::{GenericCasePattern, bind_generic_case_subject, plan, plan_generic_case_pattern};
    use crate::plan::{
        Expr, GenericExpr, GenericLocal, GenericLocalId, GenericReturn, ReturnExpr, Step,
        TypeParameterId, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::expression::typed_int_expr;
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use ecow::EcoString;
    use gleam_core::ast::{BinOp, ClauseGuard, Constant, Pattern};
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn generic_case_patterns_preserve_bindings_and_reject_invalid_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let generic_type = type_::generic_var(0);

        let variable = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: generic_type.clone(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_generic_case_pattern(variable.clone(), parameter, &mut context),
            Ok(GenericCasePattern {
                bound_names: vec!["value".into()],
            }),
        );
        assert_eq!(
            plan_generic_case_pattern(
                Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: generic_type.clone(),
                },
                parameter,
                &mut context,
            ),
            Ok(GenericCasePattern {
                bound_names: Vec::new(),
            }),
        );
        assert_eq!(
            plan_generic_case_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(variable),
                },
                parameter,
                &mut context,
            ),
            Ok(GenericCasePattern {
                bound_names: vec!["value".into(), "alias".into()],
            }),
        );
        assert_eq!(
            plan_generic_case_pattern(
                Pattern::Invalid {
                    location: dummy_span(),
                    type_: generic_type,
                },
                parameter,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
        assert_eq!(
            plan_generic_case_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Invalid {
                        location: dummy_span(),
                        type_: type_::generic_var(0),
                    }),
                },
                parameter,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
        assert_eq!(
            plan_generic_case_pattern(
                Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                },
                parameter,
                &mut context,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Parameter(parameter),
                ValueType::Int,
            )),
        );
        for pattern in [
            Pattern::Variable {
                location: dummy_span(),
                name: "mismatched".into(),
                type_: type_::int(),
                origin: VariableOrigin::generated(),
            },
            Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_::int(),
            },
        ] {
            assert_eq!(
                plan_generic_case_pattern(pattern, parameter, &mut context),
                Err(super::super::pattern_type_mismatch(
                    ValueType::Parameter(parameter),
                    ValueType::Int,
                )),
            );
        }
    }

    #[test]
    fn generic_case_subject_binding_preserves_parameter_and_internal_name() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let source_local = GenericLocal::new(GenericLocalId(7), parameter);
        let subject = GenericExpr::local_get(source_local, "input".into());
        let local = GenericLocal::new(GenericLocalId(0), parameter);

        assert_eq!(
            bind_generic_case_subject(subject.clone(), parameter, &mut context),
            (
                Step::let_generic(local, "<case:generic:0>".into(), subject),
                Expr::generic(GenericExpr::local_get(local, "<case:generic:0>".into())),
            ),
        );
    }

    #[test]
    fn plan_generic_case_preserves_the_exact_successful_return_plan() {
        let plan = plan_module(compile(
            r#"
fn identity(value: value) -> value {
  case value {
    _ -> value
  }
}

pub fn main() { 1 }
"#,
        ))
        .expect("a total generic case should plan");
        let parameter = TypeParameterId(0);
        let input = GenericLocal::new(GenericLocalId(0), parameter);
        let subject = GenericLocal::new(GenericLocalId(1), parameter);

        assert_eq!(
            plan.functions()[0].return_(),
            &ReturnExpr::generic_body(
                parameter,
                GenericReturn::block(
                    vec![Step::let_generic(
                        subject,
                        "<case:generic:1>".into(),
                        GenericExpr::local_get(input, "value".into()),
                    )],
                    GenericReturn::expr(GenericExpr::local_get(input, "value".into())),
                ),
            ),
        );
    }

    #[test]
    fn reject_margin_generic_case_subject_family_mismatch() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan(
                type_::int(),
                typed_int_expr(1),
                TypeParameterId(0),
                Vec::new(),
                &mut context,
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::TypeParameter,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn generic_case_preserves_subject_and_branch_planning_errors() {
        for source in [
            r#"
fn invalid(value: value) -> value {
  case { <<1:native>> value } {
    _ -> value
  }
}

pub fn main() { 1 }
"#,
            r#"
fn invalid(value: value) -> value {
  case value {
    _ -> { <<1:native>> value }
  }
}

pub fn main() { 1 }
"#,
        ] {
            assert_eq!(
                plan_module(compile(source)),
                Err(PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                }),
            );
        }
    }

    #[test]
    fn generic_case_propagates_invalid_pattern_from_owner() {
        let mut module = compile(
            r#"
fn invalid(value: value) -> Int {
  case value {
    _ -> 1
  }
}

pub fn main() { 1 }
"#,
        );
        let (_, subjects, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: subjects[0].type_().clone(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    #[test]
    fn generic_case_requires_an_ordered_total_fallback() {
        let mut module = compile(
            r#"
fn invalid(value: value) -> Int {
  case value {
    _ -> 1
  }
}

pub fn main() { 1 }
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::BinaryOperator {
            location: dummy_span(),
            operator: BinOp::Eq,
            operator_start: 0,
            left: Box::new(ClauseGuard::Constant(Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            })),
            right: Box::new(ClauseGuard::Constant(Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            })),
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
    }
}
