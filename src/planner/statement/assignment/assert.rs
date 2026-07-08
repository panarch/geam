use super::{
    BindingPattern, PlannedAssignment, plan_assignment_steps, plan_binding_pattern,
    plan_bound_assignment, plan_ordinary_assignment_value, value_type_expression_type,
};
use crate::plan::{
    AssertBinding, AssertPattern, Expr, ListAssertPattern, ListAssertTail, ListExpr, ListLocal,
    ParamLocal, Step, StringExpr, ValueType,
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
    let Some(pattern_type) = assert_list_pattern_type(&pattern) else {
        let pattern = plan_assert_exhaustive_pattern(pattern)?;
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

    let local = context.define_internal_list_local(element_type.clone());
    let name = internal_list_name(&local);
    let site = context.panic_site(location);
    let pattern_span = pattern.location().into();
    let pattern = plan_assert_pattern(pattern, context)?;
    let list_local = ListExpr::local_get(local.clone(), name.clone());
    let steps = vec![
        Step::let_list(local.clone(), name, value),
        Step::assert_list_at(local, pattern, message, site, pattern_span),
    ];

    Ok(PlannedAssignment {
        steps,
        value: Expr::list(list_local),
    })
}

pub(super) fn plan_assert_assignment_steps(
    location: SrcSpan,
    pattern: TypedPattern,
    value: TypedExpr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    if assert_list_pattern_type(&pattern).is_some() {
        return Ok(plan_assert_assignment(location, pattern, value, message, context)?.steps);
    }

    let pattern = plan_assert_exhaustive_pattern(pattern)?;
    let value = plan_ordinary_assignment_value(&pattern, value, context)?;
    plan_assignment_steps(pattern, value, context)
}

fn plan_assert_exhaustive_pattern(pattern: TypedPattern) -> Result<BindingPattern, PlanError> {
    match plan_binding_pattern(pattern.clone()) {
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

fn plan_assert_list_pattern(
    pattern: TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<ListAssertPattern, PlanError> {
    let Pattern::List {
        elements,
        tail,
        type_,
        ..
    } = pattern
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    };
    let ValueType::List(element_type) =
        ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::UnsupportedExpression {
            kind: crate::planner::UnsupportedExpressionKind::UnsupportedListElementType,
        })?
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    };
    let element_type = *element_type;
    let elements = elements
        .into_iter()
        .map(|element| plan_assert_pattern(element, context))
        .collect::<Result<Vec<_>, _>>()?;
    let tail = tail
        .map(|tail| plan_assert_list_tail(*tail, element_type.clone(), context))
        .transpose()?;

    Ok(ListAssertPattern::new(element_type, elements, tail))
}

fn plan_assert_pattern(
    pattern: TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<AssertPattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } => Ok(AssertPattern::Bind(define_assert_binding(
            name, &type_, context,
        )?)),
        Pattern::Discard { .. } => Ok(AssertPattern::Discard),
        Pattern::Tuple { elements, .. } => elements
            .into_iter()
            .map(|element| plan_assert_pattern(element, context))
            .collect::<Result<Vec<_>, _>>()
            .map(AssertPattern::Tuple),
        Pattern::List { .. } => plan_assert_list_pattern(pattern, context).map(AssertPattern::list),
        Pattern::Assign { name, pattern, .. } => {
            let type_ = pattern_type(&pattern)?;
            let pattern = plan_assert_pattern(*pattern, context)?;
            let binding = define_assert_binding(name, &type_, context)?;
            Ok(AssertPattern::alias(pattern, binding))
        }
        pattern => Err(unsupported_assert_pattern_error(&pattern)),
    }
}

fn plan_assert_list_tail(
    tail: gleam_core::ast::TailPattern<std::sync::Arc<gleam_core::type_::Type>>,
    element_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<ListAssertTail, PlanError> {
    match tail.pattern {
        Pattern::Variable { name, type_, .. } => {
            assert_list_tail_type_matches(type_.as_ref(), &element_type)?;
            Ok(ListAssertTail::bind(
                context.define_list_local(name.clone(), element_type),
                name,
            ))
        }
        Pattern::Discard { type_, .. } => {
            assert_list_tail_type_matches(type_.as_ref(), &element_type)?;
            Ok(ListAssertTail::Ignore)
        }
        pattern => Err(unsupported_assert_pattern_error(&pattern)),
    }
}

fn assert_list_tail_type_matches(
    type_: &gleam_core::type_::Type,
    element_type: &ValueType,
) -> Result<(), PlanError> {
    if ValueType::from_gleam(type_) == Some(ValueType::List(Box::new(element_type.clone()))) {
        Ok(())
    } else {
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        })
    }
}

fn define_assert_binding(
    name: EcoString,
    type_: &gleam_core::type_::Type,
    context: &mut PlanContext<'_>,
) -> Result<AssertBinding, PlanError> {
    let type_ = ValueType::from_gleam(type_).ok_or(PlanError::UnsupportedExpression {
        kind: crate::planner::UnsupportedExpressionKind::UnsupportedListElementType,
    })?;
    let local = define_assert_local(name.clone(), type_, context);
    Ok(AssertBinding::new(local, name))
}

fn define_assert_local(
    name: EcoString,
    type_: ValueType,
    context: &mut PlanContext<'_>,
) -> ParamLocal {
    match type_ {
        ValueType::Int => ParamLocal::int(context.define_int_local(name)),
        ValueType::Float => ParamLocal::float(context.define_float_local(name)),
        ValueType::String => ParamLocal::string(context.define_string_local(name)),
        ValueType::Bool => ParamLocal::bool(context.define_bool_local(name)),
        ValueType::Nil => ParamLocal::nil(context.define_nil_local(name)),
        ValueType::Tuple(type_) => {
            ParamLocal::tuple(context.define_tuple_local(name, type_.clone()), type_)
        }
        ValueType::List(element_type) => {
            let element_type = *element_type;
            ParamLocal::list(context.define_list_local(name, element_type))
        }
        ValueType::Function(type_) => {
            let type_ = *type_;
            match type_.return_() {
                ValueType::Int => ParamLocal::int_function(
                    context.define_int_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::Float => ParamLocal::float_function(
                    context.define_float_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::String => ParamLocal::string_function(
                    context.define_string_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::Bool => ParamLocal::bool_function(
                    context.define_bool_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::Nil => ParamLocal::nil_function(
                    context.define_nil_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::Tuple(_) => ParamLocal::tuple_function(
                    context.define_tuple_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::List(_) => ParamLocal::list_function(
                    context.define_list_function_local(name, type_.clone()),
                    type_,
                ),
                ValueType::Function(_) => ParamLocal::function_function(
                    context.define_function_function_local(name, type_.clone()),
                    type_,
                ),
            }
        }
    }
}

fn pattern_type(
    pattern: &TypedPattern,
) -> Result<std::sync::Arc<gleam_core::type_::Type>, PlanError> {
    match pattern {
        Pattern::Variable { type_, .. }
        | Pattern::Discard { type_, .. }
        | Pattern::List { type_, .. }
        | Pattern::Constructor { type_, .. } => Ok(type_.clone()),
        Pattern::Tuple { elements, .. } => {
            let elements = elements
                .iter()
                .map(pattern_type)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(gleam_core::type_::tuple(elements))
        }
        Pattern::Assign { pattern, .. } => pattern_type(pattern),
        pattern => Err(unsupported_assert_pattern_error(pattern)),
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
        Pattern::BitArray { .. } | Pattern::BitArraySize(_) => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::BitArray,
        },
        Pattern::Constructor { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Constructor,
        },
        Pattern::StringPrefix { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::StringPrefix,
        },
        Pattern::Assign { pattern, .. } => unsupported_assert_pattern_error(pattern),
        Pattern::Invalid { .. }
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

fn list_assert_value_must_be_list(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::List,
            actual: value_type_expression_type(actual),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertBinding, AssertPattern, BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId,
        FloatLocalId, FunctionFunctionLocalId, FunctionType, IntFunctionLocalId, IntListLocalId,
        IntLocalId, ListAssertPattern, ListAssertTail, ListFunctionLocalId, ListLocal,
        NilFunctionLocalId, NilLocalId, PanicSite, ParamLocal, SourceSpan, Step, StringExpr,
        StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        function, int, let_list_step, let_tuple_step, list, local_int, local_tuple, module, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
        UnsupportedPatternKind,
    };
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, Pattern, TailPattern, TypedAssignment, TypedExpr,
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
        assert_eq!(
            super::plan_assert_exhaustive_pattern(Pattern::Tuple {
                location: dummy_span(),
                elements: vec![Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(TailPattern {
                        location: dummy_span(),
                        pattern: Pattern::Variable {
                            location: dummy_span(),
                            name: "rest".into(),
                            type_: type_::list(type_::bit_array()),
                            origin: VariableOrigin::generated(),
                        },
                    })),
                    type_: type_::list(type_::bit_array()),
                }],
            }),
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
    fn reject_margin_assert_list_pattern_shapes() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_list_pattern(
                Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
        assert_eq!(
            super::plan_assert_list_pattern(
                Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: type_::int(),
                },
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
        assert_eq!(
            super::plan_assert_list_pattern(
                Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: type_::list(type_::bit_array()),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            }),
        );
        assert_eq!(
            super::plan_assert_list_pattern(
                Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(TailPattern {
                        location: dummy_span(),
                        pattern: Pattern::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: BigInt::from(1),
                        },
                    })),
                    type_: type_::list(type_::int()),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Literal,
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
    fn reject_margin_assert_list_tail_pattern_shapes() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_list_tail(
                TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                },
                ValueType::Int,
                &mut context,
            ),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Literal,
            }),
        );
        assert_eq!(
            super::plan_assert_list_tail(
                TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Variable {
                        location: dummy_span(),
                        name: "rest".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    },
                },
                ValueType::Int,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
        assert_eq!(
            super::plan_assert_list_tail(
                TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::int(),
                    },
                },
                ValueType::Int,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    #[test]
    fn plan_assert_pattern_builds_bind_and_tuple_shapes() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                &mut context,
            ),
            Ok(AssertPattern::Bind(AssertBinding::new(
                ParamLocal::int(IntLocalId(0)),
                "value".into(),
            ))),
        );
        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: type_::int(),
                        },
                        Pattern::Variable {
                            location: dummy_span(),
                            name: "other".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        },
                    ],
                },
                &mut context,
            ),
            Ok(AssertPattern::Tuple(vec![
                AssertPattern::Discard,
                AssertPattern::Bind(AssertBinding::new(
                    ParamLocal::int(IntLocalId(1)),
                    "other".into(),
                )),
            ])),
        );
        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Variable {
                        location: dummy_span(),
                        name: "inner".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    }),
                },
                &mut context,
            ),
            Ok(AssertPattern::alias(
                AssertPattern::Bind(AssertBinding::new(
                    ParamLocal::int(IntLocalId(2)),
                    "inner".into(),
                )),
                AssertBinding::new(ParamLocal::int(IntLocalId(3)), "alias".into()),
            )),
        );
    }

    #[test]
    fn reject_margin_assert_pattern_unsupported_binding_type() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Variable {
                    location: dummy_span(),
                    name: "bits".into(),
                    type_: type_::bit_array(),
                    origin: VariableOrigin::generated(),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            }),
        );
        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::StringPrefix {
                        location: dummy_span(),
                        left_location: dummy_span(),
                        left_side_assignment: None,
                        right_location: dummy_span(),
                        left_side_string: "pre".into(),
                        right_side_assignment: AssignName::Variable("rest".into()),
                    }),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            }),
        );
        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Constructor {
                        location: dummy_span(),
                        name_location: dummy_span(),
                        name: "Boxed".into(),
                        arguments: Vec::new(),
                        module: None,
                        constructor: Inferred::Unknown,
                        spread: None,
                        type_: type_::int(),
                    }),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Constructor,
            }),
        );
        assert_eq!(
            super::plan_assert_pattern(
                Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::bit_array(),
                    }),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            }),
        );
    }

    #[test]
    fn define_assert_local_allocates_each_supported_family() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let list_type = ValueType::Int;
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let float_function_type = FunctionType::new(Vec::new(), ValueType::Float);
        let string_function_type = FunctionType::new(Vec::new(), ValueType::String);
        let bool_function_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_function_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let tuple_function_type =
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_function_type =
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );

        assert_eq!(
            super::define_assert_local("int".into(), ValueType::Int, &mut context),
            ParamLocal::int(IntLocalId(0)),
        );
        assert_eq!(
            super::define_assert_local("float".into(), ValueType::Float, &mut context),
            ParamLocal::float(FloatLocalId(0)),
        );
        assert_eq!(
            super::define_assert_local("string".into(), ValueType::String, &mut context),
            ParamLocal::string(StringLocalId(0)),
        );
        assert_eq!(
            super::define_assert_local("bool".into(), ValueType::Bool, &mut context),
            ParamLocal::bool(BoolLocalId(0)),
        );
        assert_eq!(
            super::define_assert_local("nil".into(), ValueType::Nil, &mut context),
            ParamLocal::nil(NilLocalId(0)),
        );
        assert_eq!(
            super::define_assert_local(
                "tuple".into(),
                ValueType::Tuple(tuple_type.clone()),
                &mut context,
            ),
            ParamLocal::tuple(TupleLocalId(0), tuple_type),
        );
        assert_eq!(
            super::define_assert_local(
                "list".into(),
                ValueType::List(Box::new(list_type.clone())),
                &mut context,
            ),
            ParamLocal::list(ListLocal::int(IntListLocalId(0))),
        );
        assert_eq!(
            super::define_assert_local(
                "int_function".into(),
                ValueType::Function(Box::new(int_function_type.clone())),
                &mut context,
            ),
            ParamLocal::int_function(IntFunctionLocalId(0), int_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "float_function".into(),
                ValueType::Function(Box::new(float_function_type.clone())),
                &mut context,
            ),
            ParamLocal::float_function(FloatFunctionLocalId(0), float_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "string_function".into(),
                ValueType::Function(Box::new(string_function_type.clone())),
                &mut context,
            ),
            ParamLocal::string_function(StringFunctionLocalId(0), string_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "bool_function".into(),
                ValueType::Function(Box::new(bool_function_type.clone())),
                &mut context,
            ),
            ParamLocal::bool_function(BoolFunctionLocalId(0), bool_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "nil_function".into(),
                ValueType::Function(Box::new(nil_function_type.clone())),
                &mut context,
            ),
            ParamLocal::nil_function(NilFunctionLocalId(0), nil_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "tuple_function".into(),
                ValueType::Function(Box::new(tuple_function_type.clone())),
                &mut context,
            ),
            ParamLocal::tuple_function(TupleFunctionLocalId(0), tuple_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "list_function".into(),
                ValueType::Function(Box::new(list_function_type.clone())),
                &mut context,
            ),
            ParamLocal::list_function(ListFunctionLocalId(0), list_function_type),
        );
        assert_eq!(
            super::define_assert_local(
                "function_function".into(),
                ValueType::Function(Box::new(function_function_type.clone())),
                &mut context,
            ),
            ParamLocal::function_function(FunctionFunctionLocalId(0), function_function_type),
        );
    }

    #[test]
    fn pattern_type_returns_supported_pattern_shapes() {
        let variable = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };
        let discard = Pattern::Discard {
            location: dummy_span(),
            name: "_".into(),
            type_: type_::string(),
        };
        let list = Pattern::List {
            location: dummy_span(),
            elements: Vec::new(),
            tail: None,
            type_: type_::list(type_::int()),
        };
        let constructor = Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Boxed".into(),
            arguments: Vec::new(),
            module: None,
            constructor: Inferred::Unknown,
            spread: None,
            type_: type_::bool(),
        };
        let tuple = Pattern::Tuple {
            location: dummy_span(),
            elements: vec![variable.clone(), discard.clone()],
        };
        let alias = Pattern::Assign {
            location: dummy_span(),
            name: "alias".into(),
            pattern: Box::new(list.clone()),
        };

        assert_eq!(super::pattern_type(&variable), Ok(type_::int()));
        assert_eq!(super::pattern_type(&discard), Ok(type_::string()));
        assert_eq!(super::pattern_type(&list), Ok(type_::list(type_::int())),);
        assert_eq!(super::pattern_type(&constructor), Ok(type_::bool()));
        assert_eq!(
            super::pattern_type(&tuple),
            Ok(type_::tuple(vec![type_::int(), type_::string()])),
        );
        assert_eq!(super::pattern_type(&alias), Ok(type_::list(type_::int())),);
        assert_eq!(
            super::pattern_type(&Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: None,
                right_location: dummy_span(),
                left_side_string: "pre".into(),
                right_side_assignment: AssignName::Variable("rest".into()),
            }),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            }),
        );
        assert_eq!(
            super::pattern_type(&Pattern::Tuple {
                location: dummy_span(),
                elements: vec![Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "pre".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                }],
            }),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::StringPrefix,
            }),
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
        let bit_array = Pattern::BitArray {
            location: dummy_span(),
            segments: Vec::new(),
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
            super::unsupported_assert_pattern_error(&bit_array),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::BitArray,
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
}
