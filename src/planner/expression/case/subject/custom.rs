use super::super::super::plan_expr_with_expected_source_stop_shape;
use super::{CaseClause, OrderedCaseCandidateInput, OrderedCasePattern};
use crate::plan::{BoolExpr, CustomExpr, CustomLocalId, Expr, Step, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
use crate::planner::pattern::{PlannedCustomBinding, plan_custom_subject_pattern};
use ecow::EcoString;
use gleam_core::ast::{TypedExpr, TypedPattern};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    subject_shape: ValueShape,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_shape(subject, subject_shape, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    let actual = InvalidExpressionType::from_value_type(subject.value_type());
    let Some(subject) = subject.into_custom() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Custom,
                actual,
            },
        });
    };
    let (subject_step, subject_local, subject) = bind_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            ordered_clauses.push(super::plan_ordered_case_candidate(
                OrderedCaseCandidateInput {
                    case_type: type_.as_ref(),
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    guard: clause.guard.clone(),
                    reachable,
                    exhaustive_remainder,
                },
                context,
                |context| {
                    let mut planned = plan_pattern(pattern, subject.clone(), context)?;
                    if let Some(binding) = planned.custom_binding.take() {
                        let binding = binding
                            .clone()
                            .into_intrinsic_binding()
                            .unwrap_or_else(|| binding.into_exhaustive_remainder_binding());
                        planned
                            .pattern
                            .total_branch_steps
                            .push(Step::bind_custom_fields(subject_local, binding));
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
    context: &mut PlanContext<'_>,
) -> Result<PlannedCustomPattern, PlanError> {
    crate::planner::pattern::validate_pattern(
        &pattern,
        &ValueShape::Custom(subject.shape().clone()),
        context,
    )?;
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
            let pattern = plan_custom_subject_pattern(pattern, subject.shape().clone(), context)?;
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
    context: &mut PlanContext<'_>,
) -> (Step, CustomLocalId, CustomExpr) {
    let local = context.define_internal_custom_local();
    let typed_local = crate::plan::CustomLocal::from_shape(local, subject.shape().clone());
    let name = internal_subject_name(local);
    let step = Step::let_custom(local, name.clone(), subject);
    (step, local, CustomExpr::local_get(typed_local, name))
}

fn internal_subject_name(local: CustomLocalId) -> EcoString {
    format!("<case:custom:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertBinding, AssertPattern, BoolExpr, CustomBindingPattern, CustomConstructor,
        CustomConstructorField, CustomExpr, CustomLocal, CustomLocalId, CustomPattern, CustomType,
        CustomTypeName, CustomValueShape, Expr, IntLocalId, ParamLocal, ReturnExpr, Step,
        TotalBindingPattern, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::dsl::{int, int_return_block, int_return_expr, local_int};
    use crate::planner::plan_module;
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
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
  case { <<1:native>> value } { Choice(inner) -> inner }
}
pub fn main() { 0 }
"#,
            r#"
pub type Choice { Choice(Int) }
fn identity(value: Choice) -> Int {
  case value { Choice(inner) -> { <<1:native>> inner } }
}
pub fn main() { 0 }
"#,
        ] {
            assert_eq!(
                plan_module(crate::planner::support::compile(source)),
                Err(PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
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

        assert_eq!(
            plan_module(module),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Custom,
                InvalidExpressionType::Int,
            )),
        );
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

        assert_eq!(
            plan_module(module),
            Err(super::super::pattern_type_mismatch(
                custom_value_type("Choice"),
                custom_value_type("Missing"),
            )),
        );
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
                &mut context,
            )
            .map(|_| ()),
            Err(super::super::pattern_type_mismatch(
                custom_value_type("Choice"),
                custom_value_type("Other"),
            )),
        );
        assert_eq!(
            super::plan_pattern(
                gleam_core::ast::Pattern::BitArraySize(gleam_core::ast::BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                subject,
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
                },
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
        *type_ = super::super::mismatched_generic_case_return_type();
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        assert_eq!(
            plan_module(crate::planner::support::compile(
                r#"
pub type Choice { Choice(BitArray) }
fn identity(value: Choice) -> Int {
  case value {
    Choice(<<rest:native>>) -> 1
    Choice(_) -> 0
  }
}
pub fn main() { 0 }
"#,
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

    #[test]
    fn total_custom_wildcard_remains_intrinsically_total() {
        let plan = plan_module(crate::planner::support::compile(
            r#"
pub type Choice { First Second }
fn pick(value: Choice) -> Int {
  case value { _ -> 1 }
}
pub fn main() { 0 }
"#,
        ))
        .expect("an intrinsically total custom case should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let shape = CustomValueShape::any(type_);
        let expected = ReturnExpr::int_body(int_return_block(
            [Step::let_custom(
                CustomLocalId(1),
                "<case:custom:1>".into(),
                CustomExpr::local_get(
                    CustomLocal::from_shape(CustomLocalId(0), shape),
                    "value".into(),
                ),
            )],
            int_return_expr(int(1)),
        ));

        assert_eq!(plan.functions()[0].return_(), &expected);
    }

    #[test]
    fn guarded_custom_case_preserves_the_final_remainder_binding() {
        let plan = plan_module(crate::planner::support::compile(
            r#"
pub type Choice { First(Int) Second(Int) }
fn pick(value: Choice) -> Int {
  case value {
    First(inner) if inner > 0 -> inner
    First(inner) -> inner
    Second(inner) -> inner
  }
}
pub fn main() { 0 }
"#,
        ))
        .expect("an exhaustive custom case should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let shape = CustomValueShape::any(type_.clone());
        let first = CustomConstructor::new(
            type_.clone(),
            "First".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let second = CustomConstructor::new(
            type_,
            "Second".into(),
            1,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let first_binding = AssertBinding::new(
            ParamLocal::int(IntLocalId(0)),
            "inner".into(),
            ValueShape::Int,
        );
        let second_binding = AssertBinding::new(
            ParamLocal::int(IntLocalId(1)),
            "inner".into(),
            ValueShape::Int,
        );
        let third_binding = AssertBinding::new(
            ParamLocal::int(IntLocalId(2)),
            "inner".into(),
            ValueShape::Int,
        );
        let subject_local = CustomLocal::from_shape(CustomLocalId(1), shape.clone());
        let subject_name = "<case:custom:1>";
        let subject = CustomExpr::local_get(subject_local, subject_name.into());
        let expected = ReturnExpr::int_body(int_return_block(
            [Step::let_custom(
                CustomLocalId(1),
                subject_name.into(),
                CustomExpr::local_get(
                    CustomLocal::from_shape(CustomLocalId(0), shape.clone()),
                    "value".into(),
                ),
            )],
            crate::plan::IntReturn::bool_case(
                BoolExpr::and(
                    BoolExpr::custom_matches(
                        subject.clone(),
                        CustomPattern::new(
                            first.clone(),
                            vec![AssertPattern::Bind(first_binding.clone())],
                            Some(vec![TotalBindingPattern::bind(first_binding)]),
                        ),
                    ),
                    BoolExpr::gt_int(
                        local_int(0, "inner").into(),
                        crate::plan::IntExpr::value(0.into()),
                    ),
                ),
                int_return_expr(local_int(0, "inner")),
                crate::plan::IntReturn::bool_case(
                    BoolExpr::custom_matches(
                        subject,
                        CustomPattern::new(
                            first,
                            vec![AssertPattern::Bind(second_binding.clone())],
                            Some(vec![TotalBindingPattern::bind(second_binding)]),
                        ),
                    ),
                    int_return_expr(local_int(1, "inner")),
                    int_return_block(
                        [Step::bind_custom_fields(
                            CustomLocalId(1),
                            CustomBindingPattern::exhaustive_remainder(
                                shape,
                                vec![0],
                                second,
                                vec![TotalBindingPattern::bind(third_binding)],
                            ),
                        )],
                        int_return_expr(local_int(2, "inner")),
                    ),
                ),
            ),
        ));

        assert_eq!(plan.functions()[0].return_(), &expected);
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

    fn custom_value_type(name: &str) -> ValueType {
        ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        ))
    }
}
