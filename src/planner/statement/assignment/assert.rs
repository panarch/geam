use super::{
    BindingPattern, PlannedAssignment, plan_assignment_steps, plan_bound_assignment,
    plan_ordinary_assignment_value, value_type_expression_type,
};
use crate::plan::{
    BitArrayAssertPattern, BitArrayExpr, Expr, ListExpr, ListLocal, Step, StringExpr, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedPatternKind,
};
use crate::planner::expression::plan_expr;
use ecow::EcoString;
use gleam_core::ast::{Pattern, SrcSpan, TypedExpr, TypedPattern};

pub(super) fn plan_assert_assignment(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    if let Some(pattern_type) = assert_custom_pattern_type(&pattern) {
        return plan_assert_custom_assignment(
            location,
            pattern,
            pattern_type,
            value,
            message,
            context,
        );
    }
    if assert_bit_array_pattern(&pattern) {
        return plan_assert_bit_array_assignment(location, pattern, value, message, context);
    }
    let Some(pattern_type) = assert_list_pattern_type(&pattern) else {
        let pattern = plan_assert_exhaustive_pattern(pattern, context)?;
        let value = plan_ordinary_assignment_value(&pattern, value, context)?;
        return plan_bound_assignment(pattern, value, context);
    };

    let value = plan_expr(value, context)?;
    let actual = value.value_type();
    let value = value
        .into_list()
        .ok_or_else(|| list_assert_value_must_be_list(actual))?;
    let element_type = value.element_type().clone();
    if ValueType::from_gleam(pattern_type.as_ref())
        != Some(ValueType::List(Box::new(element_type.clone())))
    {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    }
    let message = message
        .map(|message| plan_assert_message(message, context))
        .transpose()?;

    let (local, list_value) = context.define_internal_list_value(value);
    let name = internal_list_name(&local);
    let site = context.panic_site(location);
    let pattern_span = pattern.location().into();
    let pattern = crate::planner::pattern::plan_runtime_pattern(pattern, context)?.pattern;
    let list_local = ListExpr::local_get(local.clone(), name.clone());
    let steps = vec![
        Step::let_list_expr(name, list_value),
        Step::assert_list_at(local, pattern, message, site, pattern_span),
    ];

    Ok(PlannedAssignment {
        steps,
        value: Expr::list(list_local),
    })
}

fn plan_bit_array_assert_pattern(
    pattern: TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayAssertPattern, PlanError> {
    match pattern {
        Pattern::BitArray { segments, .. } => {
            let (pattern, _) = crate::planner::pattern::plan_bit_array_pattern(segments, context)?;
            Ok(BitArrayAssertPattern::pattern(pattern))
        }
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_bit_array_assert_pattern(*pattern, context)?;
            let local = context.define_bit_array_local(name.clone());
            Ok(BitArrayAssertPattern::alias(pattern, local, name))
        }
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        }),
    }
}

pub(super) fn plan_assert_assignment_steps(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    if assert_custom_pattern_type(&pattern).is_some()
        || assert_list_pattern_type(&pattern).is_some()
        || assert_bit_array_pattern(&pattern)
    {
        return Ok(plan_assert_assignment(location, pattern, value, message, context)?.steps);
    }

    let pattern = plan_assert_exhaustive_pattern(pattern, context)?;
    let value = plan_ordinary_assignment_value(&pattern, value, context)?;
    plan_assignment_steps(pattern, value, context)
}

fn plan_assert_bit_array_assignment(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let value = plan_expr(value, context)?;
    let actual = value.value_type();
    let value = value
        .into_bit_array()
        .ok_or_else(|| bit_array_assert_value_must_be_bit_array(actual))?;
    let message = message
        .map(|message| plan_assert_message(message, context))
        .transpose()?;
    let local = context.define_internal_bit_array_local();
    let name = internal_bit_array_name(local);
    let local_value = BitArrayExpr::local_get(local, name.clone());
    let site = context.panic_site(location);
    let pattern_span = pattern.location().into();
    let pattern = plan_bit_array_assert_pattern(pattern, context)?;

    Ok(PlannedAssignment {
        steps: vec![
            Step::let_bit_array(local, name, value),
            Step::assert_bit_array_at(local, pattern, message, site, pattern_span),
        ],
        value: Expr::bit_array(local_value),
    })
}

fn plan_assert_custom_assignment(
    location: SrcSpan,
    pattern: TypedPattern,
    pattern_type: std::sync::Arc<gleam_core::type_::Type>,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let Some(ValueType::Custom(pattern_type)) = ValueType::from_gleam(pattern_type.as_ref()) else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    };
    let value = plan_expr(value, context)?;
    let actual = value.value_type();
    let value = value
        .into_custom()
        .ok_or_else(|| custom_assert_value_must_be_custom(actual))?;
    if value.type_() != &pattern_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    }
    let message = message
        .map(|message| plan_assert_message(message, context))
        .transpose()?;
    let local = context.define_internal_custom_local();
    let name = internal_custom_name(local);
    let local_value = crate::plan::CustomExpr::local_get(local, name.clone(), pattern_type);
    let site = context.panic_site(location);
    let pattern_span = pattern.location().into();
    let pattern = crate::planner::pattern::plan_runtime_pattern(pattern, context)?.pattern;

    Ok(PlannedAssignment {
        steps: vec![
            Step::let_custom(local, name, value),
            Step::assert_custom_at(local, pattern, message, site, pattern_span),
        ],
        value: Expr::custom(local_value),
    })
}

fn plan_assert_exhaustive_pattern(
    pattern: TypedPattern,
    context: &PlanContext<'_>,
) -> Result<BindingPattern, PlanError> {
    match super::plan_binding_pattern_in_context(pattern.clone(), context) {
        Ok(pattern) => Ok(pattern),
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        }) => Err(unsupported_assert_exhaustive_pattern_error(&pattern)),
        Err(error) => Err(error),
    }
}

fn plan_assert_message(
    message: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let value = plan_expr(message, context)?;
    let actual = value.value_type();
    value
        .into_string()
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::String,
                actual: value_type_expression_type(actual),
            },
        })
}

fn assert_bit_array_pattern(pattern: &TypedPattern) -> bool {
    match pattern {
        Pattern::BitArray { .. } => true,
        Pattern::Assign { pattern, .. } => assert_bit_array_pattern(pattern),
        _ => false,
    }
}

fn assert_custom_pattern_type(
    pattern: &TypedPattern,
) -> Option<std::sync::Arc<gleam_core::type_::Type>> {
    match pattern {
        Pattern::Constructor { type_, .. }
            if matches!(
                ValueType::from_gleam(type_.as_ref()),
                Some(ValueType::Custom(_))
            ) =>
        {
            Some(type_.clone())
        }
        Pattern::Assign { pattern, .. } => assert_custom_pattern_type(pattern),
        _ => None,
    }
}

fn assert_list_pattern_type(
    pattern: &TypedPattern,
) -> Option<std::sync::Arc<gleam_core::type_::Type>> {
    match pattern {
        Pattern::List { type_, .. } => Some(type_.clone()),
        Pattern::Assign { pattern, .. } => assert_list_pattern_type(pattern),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Variable { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Discard { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. } => None,
    }
}

fn unsupported_assert_pattern_error(pattern: &TypedPattern) -> PlanError {
    match pattern {
        Pattern::List { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::List,
        },
        Pattern::Int { .. } | Pattern::Float { .. } | Pattern::String { .. } => {
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Literal,
            }
        }
        Pattern::Constructor { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Constructor,
        },
        Pattern::StringPrefix { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::StringPrefix,
        },
        Pattern::Assign { pattern, .. } => unsupported_assert_pattern_error(pattern),
        Pattern::Invalid { .. }
        | Pattern::BitArray { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Discard { .. }
        | Pattern::Variable { .. }
        | Pattern::Tuple { .. } => PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        },
    }
}

fn unsupported_assert_exhaustive_pattern_error(pattern: &TypedPattern) -> PlanError {
    match pattern {
        Pattern::Tuple { elements, .. } => elements
            .iter()
            .map(unsupported_assert_exhaustive_pattern_error)
            .find(|error| matches!(error, PlanError::UnsupportedPattern { .. }))
            .unwrap_or(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        Pattern::Assign { pattern, .. } => unsupported_assert_exhaustive_pattern_error(pattern),
        pattern => unsupported_assert_pattern_error(pattern),
    }
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

fn list_assert_value_must_be_list(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::List,
            actual: value_type_expression_type(actual),
        },
    }
}

fn bit_array_assert_value_must_be_bit_array(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::BitArray,
            actual: value_type_expression_type(actual),
        },
    }
}

fn custom_assert_value_must_be_custom(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Custom,
            actual: value_type_expression_type(actual),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertBinding, AssertPattern, BitArrayAssertPattern, BitArrayExpr, BitArrayListLocalId,
        BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArraySegment, Endianness, IntExpr,
        IntListLocalId, IntLocalId, ListAssertPattern, ListAssertTail, ListLocal, PanicSite,
        ParamLocal, Signedness, SourceSpan, Step, StringExpr, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bit_array, function, int, let_list_step, let_tuple_step, list, local_int, local_tuple,
        module, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind, UnsupportedPatternKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArraySegment as BitArrayPatternSegmentAst, Pattern,
        Statement, TailPattern, TypedAssignment, TypedExpr,
    };
    use gleam_core::exhaustiveness::CompiledCase;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

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
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Custom,
                    actual: InvalidExpressionType::Int,
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
            gleam_core::ast::Publicity::Public,
            Vec::new(),
        );
        assert_eq!(
            plan_module(nominal_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );

        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        assert_eq!(
            super::plan_assert_custom_assignment(
                dummy_span(),
                Pattern::Discard {
                    name: "_".into(),
                    location: dummy_span(),
                    type_: type_::int(),
                },
                type_::int(),
                typed_int_expr(1),
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
    fn custom_let_assert_propagates_value_message_and_pattern_errors() {
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
        arguments[0].value = Pattern::BitArraySize(gleam_core::ast::BitArraySize::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        });

        for invalid in [invalid_value, invalid_message] {
            assert_eq!(
                plan_module(invalid),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                }),
            );
        }
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
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
                .step(Step::assert_list_at(
                    ListLocal::int(IntListLocalId(0)),
                    AssertPattern::list(ListAssertPattern::new(
                        ValueType::Int,
                        vec![AssertPattern::Bind(AssertBinding::new(
                            ParamLocal::int(IntLocalId(0)),
                            "first".into(),
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
                .step(Step::assert_list_at(
                    ListLocal::int(IntListLocalId(0)),
                    AssertPattern::list(ListAssertPattern::new(
                        ValueType::Int,
                        vec![AssertPattern::Bind(AssertBinding::new(
                            ParamLocal::int(IntLocalId(0)),
                            "first".into(),
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
    fn reject_profile_let_assert_tuple_containing_list_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert #([first]) = #([1])
  first
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            },
        );
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
                .step(Step::assert_bit_array_at(
                    BitArrayLocalId(0),
                    BitArrayAssertPattern::pattern(pattern),
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
                .step(Step::assert_list_at(
                    ListLocal::bit_array(BitArrayListLocalId(0)),
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
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::BitArray,
                    actual: InvalidExpressionType::Int,
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
            options: vec![gleam_core::ast::BitArrayOption::Bits {
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
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                }),
            );
        }
        for invalid in [invalid_pattern, invalid_nested_pattern] {
            assert_eq!(
                plan_module(invalid),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::InvalidPattern,
                }),
            );
        }
    }

    #[test]
    fn reject_margin_bit_array_assert_pattern_must_end_in_a_bit_array_pattern() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            super::plan_bit_array_assert_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "whole".into(),
                    pattern: Box::new(Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    }),
                },
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    #[test]
    fn reject_profile_let_assert_string_prefix_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert "pre" <> rest = "prefix"
  rest
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            },
        );
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
                reason: InvalidTypedAstReason::InvalidPattern,
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
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_tuple_pattern_rejects_nested_list_pattern() {
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
            Some(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
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
                typed_echo_expr(type_::nil()),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            }),
        );
    }

    #[test]
    fn reject_margin_let_assert_exhaustive_pattern_propagates_binding_error() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_exhaustive_pattern(
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
                &context
            ),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
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
                typed_echo_expr(type_::list(type_::int())),
                None,
                &mut context,
            )
            .err(),
            Some(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
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
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
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
                reason: InvalidTypedAstReason::InvalidPattern,
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
                typed_echo_expr(type_::nil()),
                None,
                &mut context,
            ),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
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
                Some(typed_echo_expr(type_::string())),
                &mut context,
            )
            .err(),
            Some(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            }),
        );
    }

    #[test]
    fn assert_list_pattern_type_only_accepts_list_wrappers() {
        let list = int_list_pattern();
        let alias = Pattern::Assign {
            location: dummy_span(),
            name: "values".into(),
            pattern: Box::new(list.clone()),
        };

        assert_eq!(
            super::assert_list_pattern_type(&list),
            Some(type_::list(type_::int())),
        );
        assert_eq!(
            super::assert_list_pattern_type(&alias),
            Some(type_::list(type_::int())),
        );
        assert_eq!(
            super::assert_list_pattern_type(&Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_: type_::int(),
                origin: VariableOrigin::generated(),
            }),
            None,
        );
    }

    #[test]
    fn unsupported_assert_pattern_error_reports_profile_boundaries() {
        let list = Pattern::List {
            location: dummy_span(),
            elements: Vec::new(),
            tail: None,
            type_: type_::list(type_::int()),
        };
        let string_prefix = Pattern::StringPrefix {
            location: dummy_span(),
            left_location: dummy_span(),
            left_side_assignment: None,
            right_location: dummy_span(),
            left_side_string: "pre".into(),
            right_side_assignment: AssignName::Variable("rest".into()),
        };
        let invalid = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::int(),
        };
        let alias = Pattern::Assign {
            location: dummy_span(),
            name: "alias".into(),
            pattern: Box::new(string_prefix.clone()),
        };

        assert_eq!(
            super::unsupported_assert_pattern_error(&list),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            },
        );
        assert_eq!(
            super::unsupported_assert_pattern_error(&string_prefix),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            },
        );
        assert_eq!(
            super::unsupported_assert_pattern_error(&alias),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            },
        );
        assert_eq!(
            super::unsupported_assert_pattern_error(&invalid),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            },
        );
    }

    #[test]
    fn list_assert_value_must_be_list_reports_actual_family() {
        assert_eq!(
            super::list_assert_value_must_be_list(ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            },
        );
    }

    fn int_list_pattern() -> Pattern<std::sync::Arc<gleam_core::type_::Type>> {
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

    fn typed_echo_expr(type_: std::sync::Arc<gleam_core::type_::Type>) -> TypedExpr {
        TypedExpr::Echo {
            location: dummy_span(),
            type_,
            expression: None,
            message: None,
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
        statement: &mut gleam_core::ast::TypedStatement,
    ) -> &mut TypedAssignment {
        let Statement::Assignment(assignment) = statement else {
            panic!("expected assignment statement");
        };
        assignment
    }

    fn expect_constructor_pattern_type_mut(
        pattern: &mut Pattern<std::sync::Arc<gleam_core::type_::Type>>,
    ) -> &mut std::sync::Arc<gleam_core::type_::Type> {
        let Pattern::Constructor { type_, .. } = pattern else {
            panic!("expected constructor pattern");
        };
        type_
    }

    fn expect_constructor_pattern_arguments_mut(
        pattern: &mut Pattern<std::sync::Arc<gleam_core::type_::Type>>,
    ) -> &mut Vec<gleam_core::ast::CallArg<Pattern<std::sync::Arc<gleam_core::type_::Type>>>> {
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
