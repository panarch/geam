use super::{
    BindingPattern, PlannedAssignment, plan_assignment_steps, plan_bound_assignment,
    plan_ordinary_assignment_value,
};
use crate::plan::{
    AssertSubject, BitArrayExpr, BoolExpr, CustomExpr, CustomLocal, Expr, ExprKind, FloatExpr,
    IntExpr, ListExpr, ListLocal, NilExpr, Step, StringExpr, TupleExpr, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidTypedAstReason, PlanError};
use crate::planner::expression::{conversion::expect_expression, plan_expr};
use ecow::EcoString;
use gleam_compiler_core::ast::{SrcSpan, TypedExpr, TypedPattern};

pub(super) fn plan_assert_assignment(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let source_shape = context.value_shape_in_scope(value.type_().as_ref());
    if let Some(binding) = plan_total_assert_pattern(pattern.clone(), context)? {
        if binding_pattern_depends_on_source_shape(&binding) {
            let value = plan_expr(value, context)?;
            crate::planner::pattern::validate_pattern(&pattern, &source_shape, context)?;
            if binding_pattern_accepts_shape(&binding, value.shape()) {
                return plan_bound_assignment(binding, value, context);
            }
            return plan_refutable_assert_assignment_from_expr(
                location, pattern, value, message, context,
            );
        }
        let value = plan_ordinary_assignment_value(&binding, value, context)?;
        crate::planner::pattern::validate_pattern(&pattern, &source_shape, context)?;
        return plan_bound_assignment(binding, value, context);
    }

    plan_refutable_assert_assignment(location, pattern, value, source_shape, message, context)
}

pub(super) fn plan_assert_assignment_steps(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    let source_shape = context.value_shape_in_scope(value.type_().as_ref());
    if let Some(binding) = plan_total_assert_pattern(pattern.clone(), context)? {
        if binding_pattern_depends_on_source_shape(&binding) {
            let value = plan_expr(value, context)?;
            crate::planner::pattern::validate_pattern(&pattern, &source_shape, context)?;
            if binding_pattern_accepts_shape(&binding, value.shape()) {
                return plan_assignment_steps(binding, value, context);
            }
            return plan_refutable_assert_assignment_from_expr(
                location, pattern, value, message, context,
            )
            .map(|planned| planned.steps);
        }
        let value = plan_ordinary_assignment_value(&binding, value, context)?;
        crate::planner::pattern::validate_pattern(&pattern, &source_shape, context)?;
        return plan_assignment_steps(binding, value, context);
    }

    Ok(
        plan_refutable_assert_assignment(location, pattern, value, source_shape, message, context)?
            .steps,
    )
}

fn plan_total_assert_pattern(
    pattern: TypedPattern,
    context: &PlanContext<'_>,
) -> Result<Option<BindingPattern>, PlanError> {
    if super::is_total_binding_pattern(&pattern, context)? {
        super::plan_binding_pattern_in_context(pattern, context).map(Some)
    } else {
        Ok(None)
    }
}

fn binding_pattern_depends_on_source_shape(pattern: &BindingPattern) -> bool {
    match pattern {
        BindingPattern::Custom { .. } => true,
        BindingPattern::Tuple(elements) => {
            elements.iter().any(binding_pattern_depends_on_source_shape)
        }
        BindingPattern::Alias { pattern, .. } => binding_pattern_depends_on_source_shape(pattern),
        BindingPattern::Named(_) | BindingPattern::Discard | BindingPattern::ListTail { .. } => {
            false
        }
    }
}

fn binding_pattern_accepts_shape(
    pattern: &BindingPattern,
    shape: &crate::plan::ValueShape,
) -> bool {
    match pattern {
        BindingPattern::Named(_) | BindingPattern::Discard => true,
        BindingPattern::Tuple(elements) => {
            let crate::plan::ValueShape::Tuple(shapes) = shape else {
                return false;
            };
            elements.len() == shapes.len()
                && elements
                    .iter()
                    .zip(shapes)
                    .all(|(pattern, shape)| binding_pattern_accepts_shape(pattern, shape))
        }
        BindingPattern::ListTail { element_type, .. } => {
            shape.value_type() == ValueType::List(Box::new(element_type.clone()))
        }
        BindingPattern::Custom {
            source_shape,
            constructor_count,
            constructor,
            fields,
        } => {
            let crate::plan::ValueShape::Custom(actual) = shape else {
                return false;
            };
            actual.type_() == source_shape.type_()
                && (*constructor_count == 1
                    || actual.constructor()
                        == crate::plan::CustomConstructorRefinement::Exact(constructor.index()))
                && fields.len() == constructor.fields().len()
                && fields
                    .iter()
                    .zip(constructor.fields())
                    .all(|(pattern, field)| {
                        binding_pattern_accepts_shape(
                            pattern,
                            &crate::plan::ValueShape::from_value_type(field.type_().clone()),
                        )
                    })
        }
        BindingPattern::Alias { pattern, .. } => binding_pattern_accepts_shape(pattern, shape),
    }
}

fn plan_refutable_assert_assignment(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    source_shape: crate::plan::ValueShape,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let value = plan_expr(value, context)?;
    crate::planner::pattern::validate_pattern(&pattern, &source_shape, context)?;
    plan_refutable_assert_assignment_from_expr(location, pattern, value, message, context)
}

fn plan_refutable_assert_assignment_from_expr(
    location: SrcSpan,
    pattern: TypedPattern,
    value: Expr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let source_shape = value.shape().clone();
    let message = message
        .map(|message| plan_assert_message(message, context))
        .transpose()?;
    let (let_step, subject, local_value) = plan_assert_subject(value, context)?;
    let site = context.panic_site(location);
    let pattern_span = pattern.location().into();
    let pattern = crate::planner::pattern::plan_runtime_pattern_with_source_shape(
        pattern,
        source_shape,
        context,
    )?
    .pattern;

    Ok(PlannedAssignment {
        steps: vec![
            let_step,
            Step::assert_pattern_at(subject, pattern, message, site, pattern_span),
        ],
        value: local_value,
    })
}

fn plan_assert_subject(
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<(Step, AssertSubject, Expr), PlanError> {
    let actual = value.value_type();
    match value.into_kind() {
        ExprKind::Int(value) => {
            let local = context.define_internal_int_local();
            let name = internal_assert_name("int", local.0);
            Ok((
                Step::let_int(local, name.clone(), value),
                AssertSubject::Int(local),
                Expr::int(IntExpr::local_get(local, name)),
            ))
        }
        ExprKind::Float(value) => {
            let local = context.define_internal_float_local();
            let name = internal_assert_name("float", local.0);
            Ok((
                Step::let_float(local, name.clone(), value),
                AssertSubject::Float(local),
                Expr::float(FloatExpr::local_get(local, name)),
            ))
        }
        ExprKind::String(value) => {
            let local = context.define_internal_string_local();
            let name = internal_assert_name("string", local.0);
            Ok((
                Step::let_string(local, name.clone(), value),
                AssertSubject::String(local),
                Expr::string(StringExpr::local_get(local, name)),
            ))
        }
        ExprKind::BitArray(value) => {
            let local = context.define_internal_bit_array_local();
            let name = internal_bit_array_name(local);
            Ok((
                Step::let_bit_array(local, name.clone(), value),
                AssertSubject::BitArray(local),
                Expr::bit_array(BitArrayExpr::local_get(local, name)),
            ))
        }
        ExprKind::Custom(value) => {
            let local = context.define_internal_custom_local();
            let local = CustomLocal::from_shape(local, value.shape().clone());
            let name = internal_custom_name(local.id());
            Ok((
                Step::let_custom(local.id(), name.clone(), value),
                AssertSubject::Custom(local.clone()),
                Expr::custom(CustomExpr::local_get(local, name)),
            ))
        }
        ExprKind::Bool(value) => {
            let local = context.define_internal_bool_local();
            let name = internal_assert_name("bool", local.0);
            Ok((
                Step::let_bool(local, name.clone(), value),
                AssertSubject::Bool(local),
                Expr::bool(BoolExpr::local_get(local, name)),
            ))
        }
        ExprKind::Nil(value) => {
            let local = context.define_internal_nil_local();
            let name = internal_assert_name("nil", local.0);
            Ok((
                Step::let_nil(local, name.clone(), value),
                AssertSubject::Nil(local),
                Expr::nil(NilExpr::local_get(local, name)),
            ))
        }
        ExprKind::Tuple(value) => {
            let local = context.define_internal_tuple_local();
            let name = internal_assert_name("tuple", local.0);
            let type_ = value.type_().to_vec();
            let shape = value.shape().to_vec().into_boxed_slice();
            Ok((
                Step::let_tuple(local, name.clone(), value),
                AssertSubject::Tuple(local),
                Expr::tuple(TupleExpr::local_get(local, name, type_).with_shape(shape)),
            ))
        }
        ExprKind::List(value) => {
            let item_shape = value.item_shape().clone();
            let (local, value) = context.define_internal_list_value(value);
            let name = internal_list_name(&local);
            let local_value =
                ListExpr::local_get(local.clone(), name.clone()).with_item_shape(item_shape);
            Ok((
                Step::let_list_expr(name, value),
                AssertSubject::List(local),
                Expr::list(local_value),
            ))
        }
        ExprKind::Generic(_)
        | ExprKind::UtfCodepoint(_)
        | ExprKind::External(_)
        | ExprKind::Function(_) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::AssertSubject { actual },
            },
        }),
    }
}

fn plan_assert_message(
    message: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    expect_expression(plan_expr(message, context)?)
}

fn internal_list_name(local: &ListLocal) -> EcoString {
    format!("<list:{}:{}>", local.family_name(), local.index()).into()
}

fn internal_bit_array_name(local: crate::plan::BitArrayLocalId) -> EcoString {
    format!("<bit_array:{}>", local.0).into()
}

fn internal_custom_name(local: crate::plan::CustomLocalId) -> EcoString {
    format!("<custom:{}>", local.0).into()
}

fn internal_assert_name(family: &str, index: usize) -> EcoString {
    format!("<assert:{family}:{index}>").into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertBinding, AssertPattern, AssertSubject, BitArrayExpr, BitArrayListLocalId,
        BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArraySegment, CustomConstructor,
        CustomConstructorRefinement, CustomLocal, CustomLocalId, CustomType, CustomTypeName,
        CustomValueShape, Endianness, FloatLocalId, FunctionExpr, IntExpr, IntFunctionExpr,
        IntFunctionReference, IntListLocalId, IntLocalId, ListAssertPattern, ListAssertTail,
        ListLocal, NilLocalId, PanicSite, ParamLocal, Signedness, SourceSpan, Step, StepKind,
        StringExpr, StringLocalId, TupleLocalId, TypeParameterId, UtfCodepointExpr,
        UtfCodepointLocalId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bit_array, function, int, let_list_step, let_tuple_step, list, local_int, local_tuple,
        module, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{InvalidTypedAstReason, PlanError};
    use gleam_compiler_core::analyse::Inferred;
    use gleam_compiler_core::ast::{
        AssignmentKind, BitArraySegment as BitArrayPatternSegmentAst, Pattern, Statement,
        TailPattern, TypedAssignment, TypedExpr,
    };
    use gleam_compiler_core::exhaustiveness::CompiledCase;
    use gleam_compiler_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    use super::super::{BindingPattern, ListTailBinding};

    #[test]
    fn plan_let_assert_named_assignment_reuses_exhaustive_binding_semantics() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert x = 1
  x
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x")).let_int(0, "x", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn refutable_let_assert_uses_each_reachable_typed_subject_family() {
        let actual = plan_module(compile(
            r#"
pub type Choice { Empty Full(Int) }
fn choice(value: Bool) -> Choice {
  case value { True -> Full(1) False -> Empty }
}
pub fn main() {
  let assert 1 = 1
  let assert 1.5 = 1.5
  let assert "one" = "one"
  let assert <<1>> = <<1>>
  let assert Full(_) = choice(True)
  let assert True = True
  let assert Nil = Nil
  let assert #(1) = #(1)
  let assert [1] = [1]
  0
}
"#,
        ))
        .expect("source should plan");
        let subjects = actual
            .main_function()
            .steps()
            .iter()
            .filter_map(|step| match step.kind() {
                StepKind::AssertPattern { subject, .. } => Some(subject.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );

        assert_eq!(
            subjects,
            vec![
                AssertSubject::Int(IntLocalId(0)),
                AssertSubject::Float(FloatLocalId(0)),
                AssertSubject::String(StringLocalId(0)),
                AssertSubject::BitArray(BitArrayLocalId(0)),
                AssertSubject::Custom(CustomLocal::from_shape(
                    CustomLocalId(0),
                    CustomValueShape::any(custom_type),
                )),
                AssertSubject::Bool(crate::plan::BoolLocalId(0)),
                AssertSubject::Nil(NilLocalId(0)),
                AssertSubject::Tuple(TupleLocalId(0)),
                AssertSubject::List(ListLocal::int(IntListLocalId(0))),
            ],
        );
    }

    #[test]
    fn final_custom_let_assert_uses_constructor_shape_to_select_binding_or_assertion() {
        let total = plan_module(compile(
            r#"
pub type Choice { Empty Full(Int) }
pub fn main() {
  let assert Full(value) = Full(1)
}
"#,
        ))
        .expect("matching final custom assertion should plan");
        let refutable = plan_module(compile(
            r#"
pub type Choice { Empty Full(Int) }
pub fn main() {
  let assert Full(value) = Empty
}
"#,
        ))
        .expect("mismatched final custom assertion should plan as refutable");
        let plans = [("total", &total), ("refutable", &refutable)];
        let custom_bindings = plans
            .iter()
            .flat_map(|(kind, plan)| {
                plan.main_function()
                    .steps()
                    .iter()
                    .filter_map(move |step| match step.kind() {
                        StepKind::BindCustomFields { .. } => Some(*kind),
                        _ => None,
                    })
            })
            .collect::<Vec<_>>();
        let assertion_subjects = plans
            .iter()
            .flat_map(|(kind, plan)| {
                plan.main_function()
                    .steps()
                    .iter()
                    .filter_map(move |step| match step.kind() {
                        StepKind::AssertPattern { subject, .. } => Some((*kind, subject.clone())),
                        _ => None,
                    })
            })
            .collect::<Vec<_>>();
        let choice_name = CustomTypeName::new("geam".into(), "main".into(), "Choice".into());

        assert_eq!(custom_bindings, vec!["total"]);
        assert_eq!(
            assertion_subjects,
            vec![(
                "refutable",
                AssertSubject::Custom(CustomLocal::from_shape(
                    CustomLocalId(0),
                    CustomValueShape::new(
                        choice_name,
                        Vec::new(),
                        CustomConstructorRefinement::Exact(0),
                    ),
                )),
            )],
        );
    }

    #[test]
    fn total_custom_assertions_validate_the_pattern_against_the_source_type() {
        let source = |final_assignment| {
            if final_assignment {
                r#"
pub type First { First(Int) }
pub type Second { Second(Int) }
pub fn main() {
  let assert First(value) = First(1)
}
"#
            } else {
                r#"
pub type First { First(Int) }
pub type Second { Second(Int) }
pub fn main() {
  let assert First(value) = First(1)
  value
}
"#
            }
        };
        let second_pattern = || Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Second".into(),
            arguments: vec![gleam_compiler_core::ast::CallArg {
                label: None,
                location: dummy_span(),
                value: Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                implicit: None,
            }],
            module: None,
            constructor: Inferred::Known(gleam_compiler_core::type_::PatternConstructor {
                name: "Second".into(),
                field_map: None,
                documentation: None,
                module: "main".into(),
                location: dummy_span(),
                constructor_index: 0,
            }),
            spread: None,
            type_: type_::named(
                "geam",
                "main",
                "Second",
                gleam_compiler_core::ast::Publicity::Public,
                Vec::new(),
            ),
        };

        for final_assignment in [false, true] {
            let mut module = compile(source(final_assignment));
            expect_assignment_mut(&mut module.definitions.functions[0].body[0]).pattern =
                second_pattern();
            assert_eq!(
                plan_module(module),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::PatternShape {
                        reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                            expected: ValueType::Custom(CustomType::new(
                                CustomTypeName::new("geam".into(), "main".into(), "First".into(),),
                                Vec::new(),
                            )),
                            actual: ValueType::Custom(CustomType::new(
                                CustomTypeName::new("geam".into(), "main".into(), "Second".into(),),
                                Vec::new(),
                            )),
                        },
                    },
                }),
            );
        }

        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        assert_eq!(
            super::plan_assert_assignment_steps(
                dummy_span(),
                Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::string(),
                },
                typed_int_expr(1),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::String,
                    },
                },
            }),
        );
    }

    #[test]
    fn total_binding_shape_compatibility_checks_each_recursive_owner() {
        let named = BindingPattern::Named("value".into());
        let discard = BindingPattern::Discard;
        let alias = BindingPattern::Alias {
            pattern: Box::new(BindingPattern::Discard),
            name: "whole".into(),
        };
        let tuple = BindingPattern::Tuple(vec![
            BindingPattern::Named("first".into()),
            BindingPattern::ListTail {
                tail: ListTailBinding::Discard,
                element_type: ValueType::Int,
            },
        ]);
        let list = BindingPattern::ListTail {
            tail: ListTailBinding::Named("rest".into()),
            element_type: ValueType::Int,
        };

        assert!(super::binding_pattern_accepts_shape(
            &named,
            &ValueShape::Int
        ));
        assert!(super::binding_pattern_accepts_shape(
            &discard,
            &ValueShape::String,
        ));
        assert!(super::binding_pattern_accepts_shape(
            &alias,
            &ValueShape::Bool
        ));
        assert!(!super::binding_pattern_accepts_shape(
            &tuple,
            &ValueShape::Int
        ));
        assert!(!super::binding_pattern_accepts_shape(
            &tuple,
            &ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
        ));
        assert!(super::binding_pattern_accepts_shape(
            &tuple,
            &ValueShape::Tuple(
                vec![ValueShape::Int, ValueShape::List(Box::new(ValueShape::Int))]
                    .into_boxed_slice(),
            ),
        ));
        assert!(!super::binding_pattern_accepts_shape(
            &tuple,
            &ValueShape::Tuple(
                vec![
                    ValueShape::Int,
                    ValueShape::List(Box::new(ValueShape::String)),
                ]
                .into_boxed_slice(),
            ),
        ));
        assert!(super::binding_pattern_accepts_shape(
            &list,
            &ValueShape::List(Box::new(ValueShape::Int)),
        ));
        assert!(!super::binding_pattern_accepts_shape(
            &list,
            &ValueShape::List(Box::new(ValueShape::String)),
        ));

        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom = BindingPattern::Custom {
            source_shape: CustomValueShape::any(custom_type.clone()),
            constructor_count: 1,
            constructor: CustomConstructor::new(custom_type, "Boxed".into(), 0, Vec::new()),
            fields: Vec::new(),
        };
        assert!(!super::binding_pattern_accepts_shape(
            &custom,
            &ValueShape::Int,
        ));
    }

    #[test]
    fn assert_subject_rejects_root_families_without_refutable_patterns() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let cases = [
            (
                Pattern::Variable {
                    location: dummy_span(),
                    name: "codepoint".into(),
                    type_: type_::utf_codepoint(),
                    origin: VariableOrigin::generated(),
                },
                crate::plan::Expr::utf_codepoint(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                )),
            ),
            (
                Pattern::Variable {
                    location: dummy_span(),
                    name: "function".into(),
                    type_: type_::fn_(Vec::new(), type_::int()),
                    origin: VariableOrigin::generated(),
                },
                crate::plan::Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(crate::plan::monomorphic_function_instantiation(
                        0,
                        crate::plan::FunctionShape::new(Vec::new(), crate::plan::ValueShape::Int),
                    )),
                ))),
            ),
        ];

        for (pattern, expression) in cases {
            let actual = expression.value_type();
            assert_eq!(
                super::plan_refutable_assert_assignment_from_expr(
                    dummy_span(),
                    pattern,
                    expression,
                    None,
                    &mut context,
                )
                .map(|_| ()),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::PatternShape {
                        reason: crate::planner::InvalidPatternShapeReason::AssertSubject { actual },
                    },
                }),
            );
        }
    }

    #[test]
    fn assertion_type_validation_propagates_malformed_pattern_shape() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        assert_eq!(
            crate::planner::pattern::validate_pattern(
                &Pattern::BitArraySize(gleam_compiler_core::ast::BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                &crate::plan::ValueShape::Int,
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
                },
            }),
        );
        assert_eq!(
            super::plan_refutable_assert_assignment_from_expr(
                dummy_span(),
                Pattern::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                crate::plan::Expr::int(IntExpr::value(1.into())),
                None,
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    #[test]
    fn custom_let_assert_rejects_non_custom_and_nominally_mismatched_values() {
        let mut non_custom_value = compile(
            r#"
pub type First { First(Int) }
pub fn main() {
  let assert First(value) = First(1)
  value
}
"#,
        );
        expect_assignment_mut(&mut non_custom_value.definitions.functions[0].body[0]).value =
            typed_int_expr(1);
        assert_eq!(
            plan_module(non_custom_value),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Custom(CustomType::new(
                            CustomTypeName::new("geam".into(), "main".into(), "First".into(),),
                            Vec::new(),
                        )),
                    },
                },
            }),
        );

        let mut nominal_mismatch = compile(
            r#"
pub type First { First(Int) }
pub type Second { Second(Int) }
pub fn main() {
  let assert First(value) = First(1)
  value
}
"#,
        );
        let assignment =
            expect_assignment_mut(&mut nominal_mismatch.definitions.functions[0].body[0]);
        *expect_constructor_pattern_type_mut(&mut assignment.pattern) = type_::named(
            "geam",
            "main",
            "Second",
            gleam_compiler_core::ast::Publicity::Public,
            Vec::new(),
        );
        assert_eq!(
            plan_module(nominal_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Second".into(),
                    reason: Box::new(crate::planner::InvalidCustomTypeReason::ConstructorName {
                        index: 0,
                        expected: "Second".into(),
                        actual: "First".into(),
                    }),
                },
            }),
        );

        let mut final_nominal_mismatch = compile(
            r#"
pub type First { First(Int) }
pub type Second { Second(Int) }
pub fn main() {
  let assert First(value) = First(1)
}
"#,
        );
        let assignment =
            expect_assignment_mut(&mut final_nominal_mismatch.definitions.functions[0].body[0]);
        *expect_constructor_pattern_type_mut(&mut assignment.pattern) = type_::named(
            "geam",
            "main",
            "Second",
            gleam_compiler_core::ast::Publicity::Public,
            Vec::new(),
        );
        assert_eq!(
            plan_module(final_nominal_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Second".into(),
                    reason: Box::new(crate::planner::InvalidCustomTypeReason::ConstructorName {
                        index: 0,
                        expected: "Second".into(),
                        actual: "First".into(),
                    }),
                },
            }),
        );
    }

    #[test]
    fn total_custom_let_assert_propagates_value_and_pattern_errors_but_not_message() {
        let source = r#"
pub type Boxed { Boxed(Int) }
pub fn main() {
  let assert Boxed(value) = Boxed(1)
  value
}
"#;
        let invalid_expr = |type_| TypedExpr::Invalid {
            location: dummy_span(),
            type_,
            extra_information: None,
        };

        let mut invalid_value = compile(source);
        let assignment = expect_assignment_mut(&mut invalid_value.definitions.functions[0].body[0]);
        assignment.value = invalid_expr(assignment.value.type_());

        let mut invalid_final_value = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() {
  let assert Boxed(value) = Boxed(1)
}
"#,
        );
        let assignment =
            expect_assignment_mut(&mut invalid_final_value.definitions.functions[0].body[0]);
        assignment.value = invalid_expr(assignment.value.type_());

        let mut invalid_message = compile(source);
        expect_assignment_mut(&mut invalid_message.definitions.functions[0].body[0]).kind =
            AssignmentKind::Assert {
                location: dummy_span(),
                assert_keyword_start: 0,
                message: Some(invalid_expr(type_::string())),
            };

        let mut invalid_pattern = compile(source);
        let arguments = expect_constructor_pattern_arguments_mut(
            &mut expect_assignment_mut(&mut invalid_pattern.definitions.functions[0].body[0])
                .pattern,
        );
        arguments[0].value = Pattern::BitArraySize(gleam_compiler_core::ast::BitArraySize::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        });

        assert_eq!(
            plan_module(invalid_value),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
        assert_eq!(
            plan_module(invalid_final_value),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
        assert_eq!(plan_module(invalid_message), plan_module(compile(source)));
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
                },
            }),
        );
    }

    #[test]
    fn plan_ordinary_let_assert_discard_assignment_evaluates_value() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert _ = 1
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(42)).evaluate(int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_let_assert_discard_assignment_returns_value() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert _ = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_let_assert_named_assignment_returns_bound_value() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert value = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "value")).let_int(0, "value", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_let_assert_tuple_assignment_returns_internal_tuple() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert #(one, two) = #(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let expected = module(
            "main",
            function("main", local_tuple(0, "<tuple:0>", type_.clone()))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", type_.clone()).index_int(0),
                )
                .let_int(1, "two", local_tuple(0, "<tuple:0>", type_).index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_let_assert_alias_assignment_returns_alias() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert value as alias = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(1, "alias"))
                .let_int(0, "value", int(1))
                .let_int(1, "alias", local_int(0, "value")),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_let_assert_tuple_assignment_reuses_tuple_destructuring() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert #(one, two) = #(1, 2)
  one + two
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function("main", local_int(0, "one").add_int(local_int(1, "two")))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]).index_int(0),
                )
                .let_int(1, "two", tuple_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_let_assert_list_assignment_checks_internal_list_once() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert [first, ..rest] = [1, 2]
  first
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "first"))
                .step(let_list_step(
                    0,
                    "<list:int:0>",
                    list([int(1), int(2)], ValueType::Int),
                ))
                .step(Step::assert_pattern_at(
                    AssertSubject::List(ListLocal::int(IntListLocalId(0))),
                    AssertPattern::list(ListAssertPattern::new(
                        ValueType::Int,
                        vec![AssertPattern::Bind(AssertBinding::new(
                            ParamLocal::int(IntLocalId(0)),
                            "first".into(),
                            ValueShape::Int,
                        ))],
                        Some(ListAssertTail::bind(
                            ListLocal::int(IntListLocalId(1)),
                            "rest".into(),
                        )),
                    )),
                    None,
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(19, 29)),
                    SourceSpan::new(30, 45),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_let_assert_list_assignment_preserves_message_expression() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert [first, ..] = [1] as "not empty"
  first
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "first"))
                .step(let_list_step(
                    0,
                    "<list:int:0>",
                    list([int(1)], ValueType::Int),
                ))
                .step(Step::assert_pattern_at(
                    AssertSubject::List(ListLocal::int(IntListLocalId(0))),
                    AssertPattern::list(ListAssertPattern::new(
                        ValueType::Int,
                        vec![AssertPattern::Bind(AssertBinding::new(
                            ParamLocal::int(IntLocalId(0)),
                            "first".into(),
                            ValueShape::Int,
                        ))],
                        Some(ListAssertTail::Ignore),
                    )),
                    Some(StringExpr::value("not empty".into())),
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(19, 29)),
                    SourceSpan::new(30, 41),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_let_assert_bit_array_pattern_checks_internal_subject_once() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert <<first>> = <<1>>
  first
}
"#,
        ))
        .expect("source should plan");
        let pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
            pattern: BitArrayPatternValue::Bind(crate::plan::PatternBinding::new(
                IntLocalId(0),
                "first".into(),
            )),
            size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
            endianness: Endianness::Big,
            signedness: Signedness::Unsigned,
        }]);
        let expected = module(
            "main",
            function("main", local_int(0, "first"))
                .step(Step::let_bit_array(
                    BitArrayLocalId(0),
                    "<bit_array:0>".into(),
                    BitArrayExpr::value(vec![BitArraySegment::Int {
                        value: IntExpr::value(1.into()),
                        bit_size: 8,
                        endianness: Endianness::Big,
                    }]),
                ))
                .step(Step::assert_pattern_at(
                    AssertSubject::BitArray(BitArrayLocalId(0)),
                    AssertPattern::BitArray(pattern),
                    None,
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(19, 29)),
                    SourceSpan::new(30, 39),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_assert_bit_array_pattern_preserves_nested_pattern_type() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let assert [<<first>>] = [<<1>>]
  first
}
"#,
        ))
        .expect("source should plan");
        let pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
            pattern: BitArrayPatternValue::Bind(crate::plan::PatternBinding::new(
                IntLocalId(0),
                "first".into(),
            )),
            size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
            endianness: Endianness::Big,
            signedness: Signedness::Unsigned,
        }]);
        let expected = module(
            "main",
            function("main", local_int(0, "first"))
                .step(let_list_step(
                    0,
                    "<list:bit array:0>",
                    list(
                        [bit_array([BitArraySegment::Int {
                            value: IntExpr::value(1.into()),
                            bit_size: 8,
                            endianness: Endianness::Big,
                        }])],
                        ValueType::BitArray,
                    ),
                ))
                .step(Step::assert_pattern_at(
                    AssertSubject::List(ListLocal::bit_array(BitArrayListLocalId(0))),
                    AssertPattern::list(ListAssertPattern::new(
                        ValueType::BitArray,
                        vec![AssertPattern::BitArray(pattern)],
                        None,
                    )),
                    None,
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(19, 29)),
                    SourceSpan::new(30, 41),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_let_assert_bit_array_value_must_be_bit_array() {
        let mut invalid = compile(
            r#"
pub fn main() {
  let assert <<1>> = <<1>>
  1
}
"#,
        );
        let assignment = expect_assignment_mut(&mut invalid.definitions.functions[0].body[0]);
        assignment.value = typed_int_expr(1);

        assert_eq!(
            plan_module(invalid),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::BitArray,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bit_array_assert_propagates_expression_and_pattern_errors() {
        let invalid_expr = || TypedExpr::Invalid {
            location: dummy_span(),
            type_: type_::bit_array(),
            extra_information: None,
        };
        let invalid_segment = || BitArrayPatternSegmentAst {
            location: dummy_span(),
            value: Box::new(Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: 1.into(),
            }),
            options: vec![gleam_compiler_core::ast::BitArrayOption::Bits {
                location: dummy_span(),
            }],
            type_: type_::bit_array(),
        };
        let source = r#"
pub fn main() {
  let assert <<1>> = <<1>>
  1
}
"#;
        let mut invalid_value = compile(source);
        expect_assignment_mut(&mut invalid_value.definitions.functions[0].body[0]).value =
            invalid_expr();

        let mut invalid_message = compile(source);
        expect_assignment_mut(&mut invalid_message.definitions.functions[0].body[0]).kind =
            AssignmentKind::Assert {
                location: dummy_span(),
                assert_keyword_start: 0,
                message: Some(invalid_expr()),
            };

        let mut invalid_pattern = compile(source);
        expect_assignment_mut(&mut invalid_pattern.definitions.functions[0].body[0]).pattern =
            Pattern::BitArray {
                location: dummy_span(),
                segments: vec![invalid_segment()],
            };

        let mut invalid_nested_pattern = compile(
            r#"
pub fn main() {
  let assert [<<1>>] = [<<1>>]
  1
}
"#,
        );
        expect_assignment_mut(&mut invalid_nested_pattern.definitions.functions[0].body[0])
            .pattern = Pattern::List {
            location: dummy_span(),
            elements: vec![Pattern::BitArray {
                location: dummy_span(),
                segments: vec![invalid_segment()],
            }],
            tail: None,
            type_: type_::list(type_::bit_array()),
        };

        for invalid in [invalid_value, invalid_message] {
            assert_eq!(
                plan_module(invalid),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::InvalidExpressionNode,
                }),
            );
        }
        for invalid in [invalid_pattern, invalid_nested_pattern] {
            assert_eq!(
                plan_module(invalid),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::PatternShape {
                        reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                            expected: ValueType::BitArray,
                            actual: ValueType::Int,
                        },
                    },
                }),
            );
        }
    }

    #[test]
    fn reject_margin_let_assert_list_pattern_type_mismatch() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                Pattern::List {
                    location: dummy_span(),
                    elements: vec![Pattern::Variable {
                        location: dummy_span(),
                        name: "first".into(),
                        type_: type_::string(),
                        origin: VariableOrigin::generated(),
                    }],
                    tail: None,
                    type_: type_::list(type_::string()),
                },
                typed_int_list_expr(),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::List(Box::new(ValueType::Int)),
                        actual: ValueType::List(Box::new(ValueType::String)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_message_must_be_string() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                int_list_pattern(),
                typed_int_list_expr(),
                Some(typed_int_expr(1)),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: crate::planner::InvalidExpressionType::String,
                    actual: crate::planner::InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_tuple_pattern_rejects_non_tuple_value() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![int_list_pattern()],
                },
                typed_int_expr(1),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(vec![ValueType::List(Box::new(ValueType::Int,))]),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_exhaustive_pattern_propagates_value_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::int(),
                },
                TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::nil(),
                    extra_information: None,
                },
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_exhaustive_generic_list_propagates_value_type_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::List {
                        location: dummy_span(),
                        elements: Vec::new(),
                        tail: Some(Box::new(TailPattern {
                            location: dummy_span(),
                            pattern: Pattern::Variable {
                                location: dummy_span(),
                                name: "rest".into(),
                                type_: type_::list(type_::generic_var(0)),
                                origin: VariableOrigin::generated(),
                            },
                        })),
                        type_: type_::list(type_::generic_var(0)),
                    }],
                },
                typed_int_expr(1),
                None,
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(vec![ValueType::List(Box::new(
                            ValueType::Parameter(TypeParameterId(0)),
                        ))]),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_list_pattern_propagates_value_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                int_list_pattern(),
                TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    extra_information: None,
                },
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_list_value_must_be_list() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                int_list_pattern(),
                typed_int_expr(1),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::KindMismatch {
                        expected: ValueType::Int,
                        actual: crate::planner::PatternKind::List,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_final_let_assignment_propagates_pattern_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::super::plan_final_assignment(
                TypedAssignment {
                    location: dummy_span(),
                    value: typed_int_list_expr(),
                    pattern: int_list_pattern(),
                    kind: AssignmentKind::Let,
                    compiled_case: CompiledCase::simple_variable_assignment(
                        "first".into(),
                        type_::int(),
                    ),
                    annotation: None,
                },
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::ListBindingElements {
                        actual: 1,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_ordinary_let_assert_propagates_value_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment_steps(
                dummy_span(),
                Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::int(),
                },
                TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::nil(),
                    extra_information: None,
                },
                None,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_propagates_message_expression_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_assignment(
                dummy_span(),
                int_list_pattern(),
                typed_int_list_expr(),
                Some(TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::string(),
                    extra_information: None,
                }),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
    }

    fn int_list_pattern() -> Pattern<std::sync::Arc<gleam_compiler_core::type_::Type>> {
        Pattern::List {
            location: dummy_span(),
            elements: vec![Pattern::Variable {
                location: dummy_span(),
                name: "first".into(),
                type_: type_::int(),
                origin: VariableOrigin::generated(),
            }],
            tail: None,
            type_: type_::list(type_::int()),
        }
    }

    fn typed_int_list_expr() -> TypedExpr {
        TypedExpr::List {
            location: dummy_span(),
            type_: type_::list(type_::int()),
            elements: vec![typed_int_expr(1)],
            tail: None,
        }
    }

    fn typed_int_expr(value: i64) -> TypedExpr {
        TypedExpr::Int {
            location: dummy_span(),
            type_: type_::int(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }

    fn expect_assignment_mut(
        statement: &mut gleam_compiler_core::ast::TypedStatement,
    ) -> &mut TypedAssignment {
        let Statement::Assignment(assignment) = statement else {
            panic!("expected assignment statement");
        };
        assignment
    }

    fn expect_constructor_pattern_type_mut(
        pattern: &mut Pattern<std::sync::Arc<gleam_compiler_core::type_::Type>>,
    ) -> &mut std::sync::Arc<gleam_compiler_core::type_::Type> {
        let Pattern::Constructor { type_, .. } = pattern else {
            panic!("expected constructor pattern");
        };
        type_
    }

    fn expect_constructor_pattern_arguments_mut(
        pattern: &mut Pattern<std::sync::Arc<gleam_compiler_core::type_::Type>>,
    ) -> &mut Vec<
        gleam_compiler_core::ast::CallArg<
            Pattern<std::sync::Arc<gleam_compiler_core::type_::Type>>,
        >,
    > {
        let Pattern::Constructor { arguments, .. } = pattern else {
            panic!("expected constructor pattern");
        };
        arguments
    }

    #[test]
    #[should_panic(expected = "expected assignment statement")]
    fn assignment_shape_guard_rejects_expression_statements() {
        let mut statement = Statement::Expression(typed_int_expr(1));
        let _ = expect_assignment_mut(&mut statement);
    }

    #[test]
    #[should_panic(expected = "expected constructor pattern")]
    fn constructor_pattern_shape_guard_rejects_discard_patterns() {
        let mut pattern = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::int(),
        };
        let _ = expect_constructor_pattern_type_mut(&mut pattern);
    }

    #[test]
    #[should_panic(expected = "expected constructor pattern")]
    fn constructor_argument_shape_guard_rejects_discard_patterns() {
        let mut pattern = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::int(),
        };
        let _ = expect_constructor_pattern_arguments_mut(&mut pattern);
    }
}
