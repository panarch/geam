mod assert;

use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr,
    NilExpr, NilFunctionExpr, Step, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    TupleLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedPatternKind,
};
use crate::planner::expression::{
    plan_expr, plan_expr_with_expected_source_stop_type, tuple_index_expr,
};
use ecow::EcoString;
use gleam_core::ast::{AssignmentKind, Pattern, TypedAssignment, TypedExpr, TypedPattern};

pub(super) fn plan_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match assignment.kind {
        AssignmentKind::Let => {
            let pattern = plan_binding_pattern(assignment.pattern)?;
            let value = plan_ordinary_assignment_value(&pattern, assignment.value, context)?;
            plan_assignment_steps(pattern, value, context)
        }
        AssignmentKind::Assert { message, .. } => assert::plan_assert_assignment_steps(
            assignment.pattern,
            assignment.value,
            message,
            context,
        ),
        AssignmentKind::Generated => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::GeneratedAssignment,
        }),
    }
}

pub(super) struct PlannedAssignment {
    pub(super) steps: Vec<Step>,
    pub(super) value: Expr,
}

pub(super) fn plan_final_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let (pattern, value) = match assignment.kind {
        AssignmentKind::Let => {
            let pattern = plan_binding_pattern(assignment.pattern)?;
            let value = plan_ordinary_assignment_value(&pattern, assignment.value, context)?;
            (pattern, value)
        }
        AssignmentKind::Assert { message, .. } => {
            return assert::plan_assert_assignment(
                assignment.pattern,
                assignment.value,
                message,
                context,
            );
        }
        AssignmentKind::Generated => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            });
        }
    };
    match pattern {
        BindingPattern::Named(name) => {
            let (step, value) = plan_variable_runtime_step_and_return(name, value, context);
            Ok(PlannedAssignment {
                steps: vec![step],
                value,
            })
        }
        BindingPattern::Discard => Ok(PlannedAssignment {
            steps: Vec::new(),
            value,
        }),
        BindingPattern::Tuple(elements) => plan_tuple_assignment(elements, value, context),
        BindingPattern::Alias { pattern, name } => {
            plan_alias_assignment(*pattern, name, value, context)
        }
    }
}

fn plan_ordinary_assignment_value(
    pattern: &BindingPattern,
    value: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if matches!(pattern, BindingPattern::Discard) {
        plan_expr_with_expected_source_stop_type(value, ValueType::Nil, context)
    } else {
        plan_expr(value, context)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BindingPattern {
    Named(EcoString),
    Discard,
    Tuple(Vec<BindingPattern>),
    Alias {
        pattern: Box<BindingPattern>,
        name: EcoString,
    },
}

fn plan_assignment_steps(
    pattern: BindingPattern,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match pattern {
        BindingPattern::Named(name) => Ok(vec![plan_variable_runtime_step(name, value, context)]),
        BindingPattern::Discard => Ok(vec![Step::evaluate(value)]),
        BindingPattern::Tuple(elements) => {
            Ok(plan_tuple_assignment(elements, value, context)?.steps)
        }
        BindingPattern::Alias { pattern, name } => {
            Ok(plan_alias_assignment(*pattern, name, value, context)?.steps)
        }
    }
}

fn plan_alias_assignment(
    pattern: BindingPattern,
    name: EcoString,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let mut planned = match pattern {
        BindingPattern::Named(name) => {
            let (step, value) = plan_variable_runtime_step_and_return(name, value, context);
            PlannedAssignment {
                steps: vec![step],
                value,
            }
        }
        BindingPattern::Discard => PlannedAssignment {
            steps: Vec::new(),
            value,
        },
        BindingPattern::Tuple(elements) => plan_tuple_assignment(elements, value, context)?,
        BindingPattern::Alias { .. } => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            });
        }
    };
    let (step, value) = plan_variable_runtime_step_and_return(name, planned.value, context);
    planned.steps.push(step);
    Ok(PlannedAssignment {
        steps: planned.steps,
        value,
    })
}

fn plan_tuple_assignment(
    elements: Vec<BindingPattern>,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let actual = value.value_type();
    let value = value
        .into_tuple()
        .ok_or_else(|| tuple_assignment_value_must_be_tuple(actual))?;
    let type_ = value.type_().to_vec();
    if elements.len() != type_.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        });
    }

    let local = context.define_internal_tuple_local();
    let name = internal_tuple_name(local);
    let tuple_local = TupleExpr::local_get(local, name.clone(), type_.clone());
    let mut steps = vec![Step::let_tuple(local, name, value)];

    for (index, (pattern, type_)) in elements.into_iter().zip(type_).enumerate() {
        let element = tuple_index_expr(tuple_local.clone(), index, type_);
        steps.extend(plan_assignment_steps(pattern, element, context)?);
    }

    Ok(PlannedAssignment {
        steps,
        value: Expr::tuple(tuple_local),
    })
}

fn internal_tuple_name(local: TupleLocalId) -> EcoString {
    format!("<tuple:{}>", local.0).into()
}

fn tuple_assignment_value_must_be_tuple(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Tuple,
            actual: value_type_expression_type(actual),
        },
    }
}

fn value_type_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::List(_) => InvalidExpressionType::List,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

pub(in crate::planner) fn plan_variable_runtime_step(
    name: EcoString,
    value: crate::plan::Expr,
    context: &mut PlanContext<'_>,
) -> Step {
    plan_variable_runtime_step_and_return(name, value, context).0
}

fn plan_variable_runtime_step_and_return(
    name: EcoString,
    value: crate::plan::Expr,
    context: &mut PlanContext<'_>,
) -> (Step, Expr) {
    match value.into_kind() {
        ExprKind::Int(value) => {
            let local = context.define_int_local(name.clone());
            (
                Step::let_int(local, name.clone(), value),
                Expr::int(IntExpr::local_get(local, name)),
            )
        }
        ExprKind::String(value) => {
            let local = context.define_string_local(name.clone());
            (
                Step::let_string(local, name.clone(), value),
                Expr::string(StringExpr::local_get(local, name)),
            )
        }
        ExprKind::Float(value) => {
            let local = context.define_float_local(name.clone());
            (
                Step::let_float(local, name.clone(), value),
                Expr::float(FloatExpr::local_get(local, name)),
            )
        }
        ExprKind::Bool(value) => {
            let local = context.define_bool_local(name.clone());
            (
                Step::let_bool(local, name.clone(), value),
                Expr::bool(BoolExpr::local_get(local, name)),
            )
        }
        ExprKind::Nil(value) => {
            let local = context.define_nil_local(name.clone());
            (
                Step::let_nil(local, name.clone(), value),
                Expr::nil(NilExpr::local_get(local, name)),
            )
        }
        ExprKind::Tuple(value) => {
            let local = context.define_tuple_local(name.clone(), value.type_().to_vec());
            let type_ = value.type_().to_vec();
            (
                Step::let_tuple(local, name.clone(), value),
                Expr::tuple(TupleExpr::local_get(local, name, type_)),
            )
        }
        ExprKind::List(value) => {
            let local = context.define_list_local(name.clone(), value.element_type().clone());
            let element_type = value.element_type().clone();
            (
                Step::let_list(local, name.clone(), value),
                Expr::list(ListExpr::local_get(local, name, element_type)),
            )
        }
        ExprKind::Function(value) => match value.into_kind() {
            FunctionExprKind::Int(value) => {
                let local = context.define_int_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_int_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::int(IntFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::String(value) => {
                let local =
                    context.define_string_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_string_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::string(StringFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::Float(value) => {
                let local =
                    context.define_float_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_float_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::float(FloatFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::Bool(value) => {
                let local = context.define_bool_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_bool_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::bool(BoolFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::Nil(value) => {
                let local = context.define_nil_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_nil_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::nil(NilFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::Tuple(value) => {
                let local =
                    context.define_tuple_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_tuple_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::tuple(TupleFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::List(value) => {
                let local = context.define_list_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_list_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::list(ListFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
            FunctionExprKind::Function(value) => {
                let local =
                    context.define_function_function_local(name.clone(), value.type_().clone());
                let type_ = value.type_().clone();
                (
                    Step::let_function_function(local, name.clone(), value),
                    Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                        local, name, type_,
                    ))),
                )
            }
        },
    }
}

pub(super) fn plan_binding_pattern(pattern: TypedPattern) -> Result<BindingPattern, PlanError> {
    match pattern {
        Pattern::Variable { name, .. } => Ok(BindingPattern::Named(name)),
        Pattern::Discard { .. } => Ok(BindingPattern::Discard),
        Pattern::Tuple { elements, .. } => elements
            .into_iter()
            .map(plan_binding_pattern)
            .collect::<Result<Vec<_>, _>>()
            .map(BindingPattern::Tuple),
        Pattern::Assign { name, pattern, .. } => Ok(BindingPattern::Alias {
            pattern: Box::new(plan_binding_pattern(*pattern)?),
            name,
        }),
        pattern => Err(non_variable_pattern_error(&pattern)),
    }
}

pub(super) fn non_variable_pattern_error(pattern: &TypedPattern) -> PlanError {
    match pattern {
        Pattern::List { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::List,
        },
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArray { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Constructor { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }
        | Pattern::Discard { .. }
        | Pattern::Variable { .. }
        | Pattern::Assign { .. }
        | Pattern::Tuple { .. } => PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingPattern, plan_binding_pattern};
    use crate::plan::{
        BoolLocalId, Expr, FunctionType, IntLocalId, LocalId, NilLocalId, StringLocalId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_case_int_function, bool_function_ref, equal, function, int, int_function_ref,
        let_bool_function_step, let_int_function_step, let_nil_function_step,
        let_string_function_step, let_tuple_step, local_bool, local_int, local_nil, local_string,
        local_tuple, module, nil_function_ref, string_function_ref, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
        UnsupportedPatternKind,
    };
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArraySize, Pattern, Statement, TypedAssignment, TypedExpr,
    };
    use gleam_core::exhaustiveness::CompiledCase;
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

    #[test]
    fn plan_let_and_integer_binop() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
  x + 2
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x").add_int(int(2))).let_int(0, "x", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_discard_assignment_evaluates_value() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ = 1
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(42)).evaluate(int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_assignment_binds_projected_elements_from_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, two) = #(1, 2)
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
    fn plan_tuple_assignment_discard_evaluates_projected_element_without_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(_, value) = #(1, 2)
  value
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function("main", local_int(0, "value"))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .evaluate(
                    local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]).index_int(0),
                )
                .let_int(0, "value", tuple_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_variable_alias_assignment_binds_inner_name_then_alias() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value as alias = 1
  value + alias
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "value").add_int(local_int(1, "alias")))
                .let_int(0, "value", int(1))
                .let_int(1, "alias", local_int(0, "value")),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_discard_alias_assignment_binds_alias_without_discard_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ as alias = 1
  alias
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "alias")).let_int(0, "alias", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_nested_alias_pattern_is_invalid() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Assign {
                        location: dummy_span(),
                        name: "inner".into(),
                        pattern: Box::new(Pattern::Variable {
                            location: dummy_span(),
                            name: "value".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }),
                    }),
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment(
                    "value".into(),
                    type_::int(),
                ),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    #[test]
    fn reject_margin_alias_tuple_pattern_requires_tuple_value() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Tuple {
                        location: dummy_span(),
                        elements: vec![Pattern::Variable {
                            location: dummy_span(),
                            name: "value".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }],
                    }),
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment(
                    "value".into(),
                    type_::int(),
                ),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn plan_tuple_alias_assignment_binds_projected_elements_and_alias_from_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, _) as pair = #(1, 2)
  one == pair.0
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let internal_tuple = local_tuple(0, "<tuple:0>", type_.clone());
        let alias_tuple = local_tuple(1, "pair", type_.clone());
        let expected = module(
            "main",
            function("main", equal(local_int(0, "one"), alias_tuple.index_int(0)))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", type_.clone()).index_int(0),
                )
                .evaluate(local_tuple(0, "<tuple:0>", type_.clone()).index_int(1))
                .step(let_tuple_step(1, "pair", internal_tuple)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_tuple_assignment_binds_nested_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, #(two, three)) = #(1, #(2, 3))
  one + two + three
}
"#,
        ))
        .expect("source should plan");
        let outer_type = [
            ValueType::Int,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        ];
        let outer_local = local_tuple(0, "<tuple:0>", outer_type.clone());
        let inner_local = local_tuple(1, "<tuple:1>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "one")
                    .add_int(local_int(1, "two"))
                    .add_int(local_int(2, "three")),
            )
            .step(let_tuple_step(
                0,
                "<tuple:0>",
                tuple([
                    Expr::from(int(1)),
                    Expr::from(tuple([Expr::from(int(2)), Expr::from(int(3))])),
                ]),
            ))
            .let_int(
                0,
                "one",
                local_tuple(0, "<tuple:0>", outer_type).index_int(0),
            )
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                outer_local.index_tuple(1, [ValueType::Int, ValueType::Int]),
            ))
            .let_int(
                1,
                "two",
                local_tuple(1, "<tuple:1>", [ValueType::Int, ValueType::Int]).index_int(0),
            )
            .let_int(2, "three", inner_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_tuple_alias_assignment_binds_nested_aliases_in_step_order() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, #(two, _) as inner) as pair = #(1, #(2, 3))
  one + two + inner.0 + pair.0
}
"#,
        ))
        .expect("source should plan");
        let outer_type = [
            ValueType::Int,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        ];
        let inner_type = [ValueType::Int, ValueType::Int];
        let outer_internal = local_tuple(0, "<tuple:0>", outer_type.clone());
        let inner_internal = local_tuple(1, "<tuple:1>", inner_type.clone());
        let inner_alias = local_tuple(2, "inner", inner_type.clone());
        let pair_alias = local_tuple(3, "pair", outer_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "one")
                    .add_int(local_int(1, "two"))
                    .add_int(inner_alias.index_int(0))
                    .add_int(pair_alias.index_int(0)),
            )
            .step(let_tuple_step(
                0,
                "<tuple:0>",
                tuple([
                    Expr::from(int(1)),
                    Expr::from(tuple([Expr::from(int(2)), Expr::from(int(3))])),
                ]),
            ))
            .let_int(
                0,
                "one",
                local_tuple(0, "<tuple:0>", outer_type.clone()).index_int(0),
            )
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                outer_internal.index_tuple(1, inner_type.clone()),
            ))
            .let_int(
                1,
                "two",
                local_tuple(1, "<tuple:1>", inner_type.clone()).index_int(0),
            )
            .evaluate(local_tuple(1, "<tuple:1>", inner_type.clone()).index_int(1))
            .step(let_tuple_step(2, "inner", inner_internal))
            .step(let_tuple_step(
                3,
                "pair",
                local_tuple(0, "<tuple:0>", outer_type),
            )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_tuple_assignment_arity_mismatch() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![typed_int_expr(1)],
                },
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        Pattern::Variable {
                            location: dummy_span(),
                            name: "one".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        },
                        Pattern::Variable {
                            location: dummy_span(),
                            name: "two".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        },
                    ],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            }),
        );
    }

    #[test]
    fn reject_margin_tuple_assignment_value_must_be_tuple() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Variable {
                        location: dummy_span(),
                        name: "one".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    }],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: crate::planner::InvalidExpressionType::Tuple,
                    actual: crate::planner::InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_nested_tuple_assignment_value_must_be_tuple() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![typed_int_expr(1)],
                },
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Tuple {
                        location: dummy_span(),
                        elements: vec![Pattern::Variable {
                            location: dummy_span(),
                            name: "one".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }],
                    }],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_tuple_assignment_value_type_error_preserves_actual_family() {
        let cases = [
            (ValueType::Int, InvalidExpressionType::Int),
            (ValueType::String, InvalidExpressionType::String),
            (ValueType::Float, InvalidExpressionType::Float),
            (ValueType::Bool, InvalidExpressionType::Bool),
            (ValueType::Nil, InvalidExpressionType::Nil),
            (
                ValueType::Tuple(vec![ValueType::Int]),
                InvalidExpressionType::Tuple,
            ),
            (
                ValueType::List(Box::new(ValueType::Int)),
                InvalidExpressionType::List,
            ),
            (
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                InvalidExpressionType::Function,
            ),
        ];

        for (actual_type, actual) in cases {
            assert_eq!(
                super::tuple_assignment_value_must_be_tuple(actual_type),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual,
                    },
                },
            );
        }
    }

    #[test]
    fn reject_profile_discard_assignment_value_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let _ = echo 1
  42
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn plan_function_valued_assignment() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

pub fn main() {
  let function = case True {
    True -> add_one
    False -> add_one
  }
  let string = string_identity
  let bool = bool_identity
  let nil = nil_identity
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1))
                .step(let_int_function_step(
                    0,
                    "function",
                    bool_case_int_function(
                        bool_(true),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ),
                ))
                .step(let_string_function_step(
                    0,
                    "string",
                    string_function_ref(0, [LocalId::String(StringLocalId(0))]),
                ))
                .step(let_bool_function_step(
                    0,
                    "bool",
                    bool_function_ref(0, [LocalId::Bool(BoolLocalId(0))]),
                ))
                .step(let_nil_function_step(
                    0,
                    "nil",
                    nil_function_ref(0, [LocalId::Nil(NilLocalId(0))]),
                )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function("bool_identity", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_assignment_returns_assigned_value_from_binding_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
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
    fn plan_final_discard_assignment_returns_assigned_value_without_binding_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_tuple_assignment_returns_internal_tuple_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, two) = #(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function(
                "main",
                local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]),
            )
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
    fn plan_final_pattern_alias_assignment_returns_alias_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, _) as pair = #(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let expected = module(
            "main",
            function("main", local_tuple(1, "pair", type_.clone()))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", type_.clone()).index_int(0),
                )
                .evaluate(local_tuple(0, "<tuple:0>", type_.clone()).index_int(1))
                .step(let_tuple_step(
                    1,
                    "pair",
                    local_tuple(0, "<tuple:0>", type_),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_final_assignment_value_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let x = echo 1
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_generated_assignment() {
        let mut generated = compile_minimal_module();
        generated.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "x".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                kind: AssignmentKind::Generated,
                compiled_case: CompiledCase::simple_variable_assignment("x".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];
        assert_eq!(
            plan_module(generated),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            }),
        );

        let mut final_generated = compile_minimal_module();
        final_generated.definitions.functions[0].body =
            vec![Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "x".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                kind: AssignmentKind::Generated,
                compiled_case: CompiledCase::simple_variable_assignment("x".into(), type_::int()),
                annotation: None,
            }))];
        assert_eq!(
            plan_module(final_generated),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            }),
        );
    }

    #[test]
    fn reject_profile_non_variable_pattern_shapes() {
        let cases = [(
            r#"
pub fn main() {
  let [..rest] = [1]
  rest
}
"#,
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            },
        )];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
    }

    #[test]
    fn reject_profile_let_assert_list_literal_element_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert [1] = [1]
  1
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Literal,
            },
        );
    }

    #[test]
    fn reject_profile_let_assert_literal_alias_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert 1 as one = 1
  one
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Literal,
            },
        );
    }

    #[test]
    fn reject_margin_invalid_pattern_shapes() {
        let variable = |name: &str| Pattern::Variable {
            location: dummy_span(),
            name: name.into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };

        assert_eq!(
            plan_binding_pattern(variable("x")),
            Ok(BindingPattern::Named("x".into())),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_::int(),
            }),
            Ok(BindingPattern::Discard),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Tuple {
                location: dummy_span(),
                elements: vec![variable("x"), variable("y")],
            }),
            Ok(BindingPattern::Tuple(vec![
                BindingPattern::Named("x".into()),
                BindingPattern::Named("y".into()),
            ])),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(variable("x")),
            }),
            Ok(BindingPattern::Alias {
                pattern: Box::new(BindingPattern::Named("x".into())),
                name: "alias".into(),
            }),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(Pattern::List {
                    location: dummy_span(),
                    elements: vec![variable("x")],
                    tail: None,
                    type_: type_::list(type_::int()),
                }),
            }),
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            }),
        );

        let patterns = [
            Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            },
            Pattern::Float {
                location: dummy_span(),
                value: "1.0".into(),
                float_value: LiteralFloatValue::ONE,
            },
            Pattern::String {
                location: dummy_span(),
                value: "a".into(),
            },
            Pattern::BitArraySize(BitArraySize::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            }),
            Pattern::BitArray {
                location: dummy_span(),
                segments: Vec::new(),
            },
            Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Boxed".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Inferred::Unknown,
                spread: None,
                type_: type_::int(),
            },
            Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: None,
                right_location: dummy_span(),
                left_side_string: "pre".into(),
                right_side_assignment: AssignName::Variable("rest".into()),
            },
            Pattern::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            },
        ];

        for pattern in patterns {
            assert_eq!(
                plan_binding_pattern(pattern),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::InvalidPattern,
                }),
            );
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
