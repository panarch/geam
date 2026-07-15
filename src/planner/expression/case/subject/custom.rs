use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseCandidateInput, OrderedCasePattern, case_return_type};
use crate::plan::{BoolExpr, CustomExpr, CustomLocalId, CustomType, Expr, Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
use crate::planner::pattern::{
    PlannedCustomBinding, pattern_value_type, plan_custom_subject_pattern,
};
use ecow::EcoString;
use gleam_core::ast::{TypedExpr, TypedPattern};
use gleam_core::type_::Type;
use std::collections::HashSet;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    subject_type: CustomType,
    inferred_variant: Option<usize>,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_type(
        subject,
        ValueType::Custom(subject_type.clone()),
        context,
    )?;
    let return_type = case_return_type(type_.as_ref())?;
    let Some(subject) = subject.into_custom() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject_local, subject) = bind_subject(subject, subject_type, context);
    let mut covered_constructors = HashSet::new();
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        let is_guarded = clause.guard.is_some();
        for pattern in clause.patterns() {
            ordered_clauses.push(super::plan_ordered_case_candidate(
                OrderedCaseCandidateInput {
                    case_type: type_.as_ref(),
                    return_type: &return_type,
                    then: clause.then.clone(),
                    guard: clause.guard.clone(),
                },
                context,
                |context| {
                    let mut planned =
                        plan_pattern(pattern, subject.clone(), inferred_variant, context)?;
                    if !is_guarded && let Some(binding) = planned.custom_binding.take() {
                        covered_constructors.insert(binding.binding().constructor().index());
                        if planned.pattern.is_total
                            || covered_constructors.len() == binding.constructor_count()
                        {
                            planned.pattern.is_total = true;
                            planned
                                .pattern
                                .total_branch_steps
                                .push(Step::bind_custom_fields(
                                    subject_local,
                                    binding.into_binding(),
                                ));
                        }
                    }
                    Ok(planned.pattern)
                },
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

fn plan_pattern(
    pattern: TypedPattern,
    subject: CustomExpr,
    inferred_variant: Option<usize>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedCustomPattern, PlanError> {
    if pattern_value_type(&pattern)? != ValueType::Custom(subject.type_().clone()) {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    }
    let (pattern, mut whole_bindings) = strip_whole_aliases(pattern);
    match pattern {
        gleam_core::ast::Pattern::Variable { name, .. } => {
            whole_bindings.insert(0, name);
            Ok(PlannedCustomPattern {
                pattern: OrderedCasePattern {
                    match_condition: BoolExpr::value(true),
                    branch_bindings: super::branch_bindings(&whole_bindings, Expr::custom(subject)),
                    total_branch_steps: Vec::new(),
                    is_total: true,
                },
                custom_binding: None,
            })
        }
        gleam_core::ast::Pattern::Discard { .. } => Ok(PlannedCustomPattern {
            pattern: OrderedCasePattern {
                match_condition: BoolExpr::value(true),
                branch_bindings: super::branch_bindings(&whole_bindings, Expr::custom(subject)),
                total_branch_steps: Vec::new(),
                is_total: true,
            },
            custom_binding: None,
        }),
        pattern => {
            let pattern = plan_custom_subject_pattern(pattern, inferred_variant, context)?;
            Ok(PlannedCustomPattern {
                pattern: OrderedCasePattern {
                    match_condition: BoolExpr::custom_matches(subject.clone(), pattern.pattern),
                    branch_bindings: super::branch_bindings(&whole_bindings, Expr::custom(subject)),
                    total_branch_steps: Vec::new(),
                    is_total: pattern.is_total,
                },
                custom_binding: pattern.custom_binding,
            })
        }
    }
}

struct PlannedCustomPattern {
    pattern: OrderedCasePattern,
    custom_binding: Option<PlannedCustomBinding>,
}

fn strip_whole_aliases(pattern: TypedPattern) -> (TypedPattern, Vec<EcoString>) {
    match pattern {
        gleam_core::ast::Pattern::Assign { name, pattern, .. } => {
            let (pattern, mut bindings) = strip_whole_aliases(*pattern);
            bindings.push(name);
            (pattern, bindings)
        }
        pattern => (pattern, Vec::new()),
    }
}

fn bind_subject(
    subject: CustomExpr,
    subject_type: CustomType,
    context: &mut PlanContext<'_>,
) -> (Step, CustomLocalId, CustomExpr) {
    let local = context.define_internal_custom_local();
    let typed_local = crate::plan::CustomLocal::new(local, subject_type);
    let name = internal_subject_name(local);
    let step = Step::let_custom(local, name.clone(), subject);
    (step, local, CustomExpr::local_get(typed_local, name))
}

fn internal_subject_name(local: CustomLocalId) -> EcoString {
    format!("<case:custom:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, CustomExpr, CustomLocalId, CustomType, CustomTypeName, Expr};
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::plan_module;
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use gleam_core::type_::error::VariableOrigin;
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn whole_custom_pattern_aliases_preserve_inner_to_outer_binding_order() {
        let custom_type = custom_type();
        let variable = gleam_core::ast::Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: custom_type,
            origin: VariableOrigin::generated(),
        };
        let pattern = gleam_core::ast::Pattern::Assign {
            location: dummy_span(),
            name: "outer".into(),
            pattern: Box::new(gleam_core::ast::Pattern::Assign {
                location: dummy_span(),
                name: "inner".into(),
                pattern: Box::new(variable.clone()),
            }),
        };

        assert_eq!(
            super::strip_whole_aliases(pattern),
            (variable, vec!["inner".into(), "outer".into()]),
        );
    }

    #[test]
    fn custom_case_subject_and_branch_errors_propagate() {
        for source in [
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case echo value { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> echo inner }
}
pub fn main() { 0 }
"#,
        ] {
            assert_eq!(
                plan_module(crate::planner::support::compile(source)),
                Err(PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Echo,
                }),
            );
        }
    }

    #[test]
    fn custom_case_subject_family_mismatch_is_exact() {
        let mut module = crate::planner::support::compile(
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: custom_type(),
            value: "1".into(),
            int_value: 1.into(),
        };

        assert_eq!(plan_module(module), Err(pattern_type_mismatch()));
    }

    #[test]
    fn custom_case_rejects_a_pattern_for_a_different_subject_type() {
        let mut module = crate::planner::support::compile(
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: gleam_core::type_::named(
                "geam",
                "main",
                "Missing",
                gleam_core::ast::Publicity::Public,
                Vec::new(),
            ),
        };

        assert_eq!(plan_module(module), Err(pattern_type_mismatch()));
    }

    #[test]
    fn custom_case_pattern_owner_rejects_malformed_typed_ast_shapes() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let subject_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let subject = CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), subject_type.clone()),
            "subject".into(),
        );

        let planned = super::plan_pattern(
            gleam_core::ast::Pattern::Variable {
                name: "bound".into(),
                location: dummy_span(),
                type_: custom_type(),
                origin: VariableOrigin::generated(),
            },
            subject.clone(),
            None,
            &mut context,
        )
        .expect("a variable custom pattern should plan");
        assert_eq!(planned.pattern.match_condition, BoolExpr::value(true));
        assert_eq!(
            planned.pattern.branch_bindings,
            vec![("bound".into(), Expr::custom(subject.clone()))],
        );
        assert_eq!(planned.pattern.total_branch_steps, Vec::new());
        assert!(planned.pattern.is_total);
        assert!(planned.custom_binding.is_none());

        assert_eq!(
            super::plan_pattern(
                gleam_core::ast::Pattern::Discard {
                    name: "_".into(),
                    location: dummy_span(),
                    type_: gleam_core::type_::named(
                        "geam",
                        "main",
                        "Other",
                        gleam_core::ast::Publicity::Public,
                        Vec::new(),
                    ),
                },
                subject.clone(),
                None,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_type_mismatch()),
        );
        assert_eq!(
            super::plan_pattern(
                gleam_core::ast::Pattern::BitArraySize(gleam_core::ast::BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                subject,
                None,
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    #[test]
    fn custom_case_return_type_and_nested_pattern_errors_propagate() {
        let mut invalid_return_type = crate::planner::support::compile(
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
        );
        let (type_, _, _) = super::super::super::expect_case_statement_mut(
            &mut invalid_return_type.definitions.functions[0].body[0],
        );
        *type_ = super::super::invalid_case_return_type();
        assert_eq!(
            plan_module(invalid_return_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut invalid_pattern = crate::planner::support::compile(
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Invalid {
            location: dummy_span(),
            type_: custom_type(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    fn custom_type() -> std::sync::Arc<gleam_core::type_::Type> {
        crate::planner::support::compile(
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Choice { value }
pub fn main() { 0 }
"#,
        )
        .definitions
        .functions[0]
            .return_type
            .clone()
    }

    fn pattern_type_mismatch() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::PatternTypeMismatch,
            },
        }
    }
}
