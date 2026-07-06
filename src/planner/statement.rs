use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr,
    NilExpr, NilFunctionExpr, Step, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    TupleLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionType, InvalidTypedAstReason, InvalidUseShapeReason, PlanError,
    UnsupportedAssignmentKind, UnsupportedPatternKind, UnsupportedStatementKind,
};
use crate::planner::expression::{plan_expr, plan_use_call, tuple_index_expr};
use ecow::EcoString;
use gleam_core::ast::{
    AssignmentKind, Pattern, Statement, TypedAssignment, TypedPattern, TypedUseAssignment,
};
use vec1::Vec1;

pub(super) struct PlannedStatements {
    pub(super) steps: Vec<Step>,
    pub(super) return_: crate::plan::Expr,
}

pub(super) fn plan_steps_and_return(
    mut statements: Vec<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
    empty_error: PlanError,
) -> Result<PlannedStatements, PlanError> {
    let Some(last_statement) = statements.pop() else {
        return Err(empty_error);
    };

    plan_ordered_steps_and_return(statements, last_statement, context)
}

pub(super) fn plan_non_empty_steps_and_return(
    statements: Vec1<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedStatements, PlanError> {
    let (statements, last_statement) = statements.split_off_last();

    plan_ordered_steps_and_return(statements, last_statement, context)
}

fn plan_ordered_steps_and_return(
    statements: Vec<gleam_core::ast::TypedStatement>,
    last_statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<PlannedStatements, PlanError> {
    let mut steps = Vec::new();
    for statement in statements {
        steps.extend(plan_runtime_steps(statement, context)?);
    }

    let return_ = match last_statement {
        Statement::Expression(expression) => plan_expr(expression, context)?,
        Statement::Assignment(assignment) => {
            let planned = plan_final_assignment(*assignment, context)?;
            steps.extend(planned.steps);
            planned.value
        }
        Statement::Use(use_) => plan_use_statement(use_, context)?,
        Statement::Assert(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssertAsFinalStatement,
            });
        }
    };

    Ok(PlannedStatements { steps, return_ })
}

fn plan_use_statement(
    use_: gleam_core::ast::TypedUse,
    context: &mut PlanContext<'_>,
) -> Result<crate::plan::Expr, PlanError> {
    validate_use_assignments(&use_.assignments)?;
    plan_use_call(*use_.call, context)
}

pub(super) fn plan_runtime_steps(
    statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match statement {
        Statement::Expression(expression) => {
            Ok(vec![Step::evaluate(plan_expr(expression, context)?)])
        }
        Statement::Assignment(assignment) => plan_assignment(*assignment, context),
        Statement::Use(_) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::UseStatement,
        }),
        Statement::Assert(_) => Err(PlanError::UnsupportedStatement {
            kind: UnsupportedStatementKind::Assert,
        }),
    }
}

fn plan_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    let (pattern, value) = plan_assignment_parts(assignment, context)?;
    plan_assignment_steps(pattern, value, context)
}

struct PlannedAssignment {
    steps: Vec<Step>,
    value: Expr,
}

fn plan_final_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let (pattern, value) = plan_assignment_parts(assignment, context)?;
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
    }
}

fn plan_assignment_parts(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<(BindingPattern, Expr), PlanError> {
    match assignment.kind {
        AssignmentKind::Let => {}
        AssignmentKind::Generated => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            });
        }
        AssignmentKind::Assert { .. } => {
            return Err(PlanError::UnsupportedAssignment {
                kind: UnsupportedAssignmentKind::LetAssert,
            });
        }
    }

    let pattern = plan_binding_pattern(assignment.pattern)?;
    let value = plan_expr(assignment.value, context)?;
    Ok((pattern, value))
}

#[derive(Debug, PartialEq, Eq)]
enum BindingPattern {
    Named(EcoString),
    Discard,
    Tuple(Vec<BindingPattern>),
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
    }
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

fn plan_binding_pattern(pattern: TypedPattern) -> Result<BindingPattern, PlanError> {
    match pattern {
        Pattern::Variable { name, .. } => Ok(BindingPattern::Named(name)),
        Pattern::Discard { .. } => Ok(BindingPattern::Discard),
        Pattern::Tuple { elements, .. } => elements
            .into_iter()
            .map(plan_binding_pattern)
            .collect::<Result<Vec<_>, _>>()
            .map(BindingPattern::Tuple),
        pattern => Err(non_variable_pattern_error(&pattern)),
    }
}

fn validate_use_assignments(assignments: &[TypedUseAssignment]) -> Result<(), PlanError> {
    for assignment in assignments {
        validate_use_assignment_pattern(&assignment.pattern)?;
    }
    Ok(())
}

fn validate_use_assignment_pattern(pattern: &TypedPattern) -> Result<(), PlanError> {
    match pattern {
        Pattern::Variable { .. } => Err(invalid_use_shape(
            InvalidUseShapeReason::UnexpectedVariableAssignment,
        )),
        pattern => Err(non_variable_pattern_error(pattern)),
    }
}

fn non_variable_pattern_error(pattern: &TypedPattern) -> PlanError {
    match pattern {
        Pattern::Assign { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Assign,
        },
        Pattern::Tuple { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Tuple,
        },
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
        | Pattern::Variable { .. } => PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        },
    }
}

fn invalid_use_shape(reason: InvalidUseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::UseShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingPattern, plan_binding_pattern};
    use crate::plan::{
        BoolLocalId, Expr, FunctionType, IntLocalId, LocalId, NilLocalId, StringLocalId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_case_int_function, bool_function_ref, call_int_function, capture_int, function,
        int, int_arg, int_function_arg, int_function_call_arg, int_function_closure,
        int_function_ref, int_return_tail_call, let_bool_function_step, let_int_function_step,
        let_nil_function_step, let_string_function_step, let_tuple_step, local_bool, local_int,
        local_int_function, local_nil, local_string, local_tuple, module, module_with_anonymous,
        nil_function_ref, string_function_ref, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidFunctionShapeReason, InvalidTypedAstReason,
        InvalidUseShapeReason, PlanError, UnsupportedAssignmentKind, UnsupportedExpressionKind,
        UnsupportedPatternKind, UnsupportedStatementKind,
    };
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArraySize, CallArg, FunctionLiteralKind,
        ImplicitCallArgOrigin, Pattern, Statement, TypedAssignment, TypedExpr, TypedModule,
        TypedUse, UseAssignment,
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
  let _ = panic
  42
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Panic,
            },
        );
    }

    #[test]
    fn plan_expression_statement_steps() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
  2
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(2)).evaluate(int(1)), []);

        assert_eq!(actual, expected);
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
    fn reject_profile_assert_statement() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  assert True
  1
}
"#,
            ),
            PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::Assert,
            },
        );
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
    fn reject_profile_final_assert_statement() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  assert True
}
"#,
            ),
            PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssertAsFinalStatement,
            },
        );
    }

    #[test]
    fn plan_use_syntax() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  use <- pair
  1
}

fn pair(callback: fn() -> Int) {
  callback()
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(
                    1,
                    [int_function_arg(
                        0,
                        int_function_ref(2, Vec::<LocalId>::new()),
                    )],
                ),
            ),
            [function(
                "pair",
                call_int_function(
                    local_int_function(0, "callback", Vec::<ValueType>::new()),
                    [],
                ),
            )
            .param_int_function(0, "callback", Vec::<ValueType>::new())],
            [function("<anonymous:0>", int(1))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_use_syntax_with_discard_assignment() {
        let actual = plan_module(compile(
            r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use _ <- with_value
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(
                    1,
                    [int_function_arg(
                        0,
                        int_function_ref(2, [LocalId::Int(IntLocalId(0))]),
                    )],
                ),
            ),
            [function(
                "with_value",
                call_int_function(
                    local_int_function(0, "continue", [ValueType::Int]),
                    [int_function_call_arg(0, int(1))],
                ),
            )
            .param_int_function(0, "continue", [ValueType::Int])],
            [function("<anonymous:0>", int(42)).discard_int_param(0)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_use_discard_callback_body_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use _ <- with_value
  todo
  42
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Todo,
            },
        );
    }

    #[test]
    fn plan_use_syntax_with_assignment_and_capture() {
        let actual = plan_module(compile(
            r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(32)
}

pub fn main() {
  let base = 10
  use value <- with_value
  value + base
}
"#,
        ))
        .expect("source should plan");
        let callback = int_function_closure(
            2,
            [LocalId::Int(IntLocalId(0))],
            [capture_int(1, local_int(0, "base"))],
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_function_arg(0, callback)]),
            )
            .let_int(0, "base", int(10)),
            [function(
                "with_value",
                call_int_function(
                    local_int_function(0, "continue", [ValueType::Int]),
                    [int_function_call_arg(0, int(32))],
                ),
            )
            .param_int_function(0, "continue", [ValueType::Int])],
            [function(
                "<anonymous:0>",
                local_int(0, "value").add_int(local_int(1, "base")),
            )
            .param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_use_syntax_with_labelled_provider_argument() {
        let actual = plan_module(compile(
            r#"
fn with_value(value value: Int, continue continue: fn(Int) -> Int) {
  continue(value)
}

pub fn main() {
  use value <- with_value(value: 41)
  value + 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(
                    1,
                    [
                        int_arg(0, int(41)),
                        int_function_arg(0, int_function_ref(2, [LocalId::Int(IntLocalId(0))])),
                    ],
                ),
            ),
            [function(
                "with_value",
                call_int_function(
                    local_int_function(0, "continue", [ValueType::Int]),
                    [int_function_call_arg(0, local_int(0, "value"))],
                ),
            )
            .param_int(0, "value")
            .param_int_function(0, "continue", [ValueType::Int])],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_use_assignment_patterns() {
        assert_eq!(
            expect_plan_error(
                r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use value as alias <- with_value
  alias
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Assign,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn with_pair(continue: fn(#(Int, Int)) -> Int) {
  continue(#(1, 2))
}

pub fn main() {
  use #(one, two) <- with_pair
  one + two
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Tuple,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn with_values(continue: fn(List(Int)) -> List(Int)) {
  continue([1])
}

pub fn main() {
  use [..rest] <- with_values
  rest
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            },
        );
    }

    #[test]
    fn reject_margin_use_call_shapes() {
        let mut non_call_rhs = compile_minimal_module();
        non_call_rhs.definitions.functions[0].body = vec![Statement::Use(gleam_core::ast::Use {
            call: Box::new(typed_int_expr(1)),
            location: dummy_span(),
            right_hand_side_location: dummy_span(),
            assignments_location: dummy_span(),
            assignments: Vec::new(),
        })];
        assert_eq!(
            plan_module(non_call_rhs),
            Err(invalid_use_shape(InvalidUseShapeReason::NonCallRhs)),
        );

        let mut missing_callback = compile_use_module();
        expect_use_call_arguments_mut(&mut missing_callback).pop();
        assert_eq!(
            plan_module(missing_callback),
            Err(invalid_use_shape(InvalidUseShapeReason::MissingCallback)),
        );

        let mut unexpected_assignment = compile_use_module();
        expect_final_use_mut(&mut unexpected_assignment)
            .assignments
            .push(UseAssignment {
                location: dummy_span(),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                annotation: None,
            });
        assert_eq!(
            plan_module(unexpected_assignment),
            Err(invalid_use_shape(
                InvalidUseShapeReason::UnexpectedVariableAssignment
            )),
        );

        let mut labelled_argument = compile_use_with_argument_module();
        let arguments = expect_use_call_arguments_mut(&mut labelled_argument);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_argument),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: crate::planner::InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );

        let mut multiple_callbacks = compile_use_module();
        let arguments = expect_use_call_arguments_mut(&mut multiple_callbacks);
        let callback = arguments
            .last()
            .expect("use call should have callback")
            .clone();
        arguments.push(callback);
        assert_eq!(
            plan_module(multiple_callbacks),
            Err(invalid_use_shape(InvalidUseShapeReason::MultipleCallbacks)),
        );

        let mut callback_not_last = compile_use_with_argument_module();
        let arguments = expect_use_call_arguments_mut(&mut callback_not_last);
        arguments.swap(0, 1);
        assert_eq!(
            plan_module(callback_not_last),
            Err(invalid_use_shape(InvalidUseShapeReason::CallbackNotLast)),
        );

        let mut unsupported_implicit = compile_use_module();
        let callback = expect_use_callback_argument_mut(&mut unsupported_implicit);
        callback.implicit = Some(ImplicitCallArgOrigin::IncorrectArityUse);
        assert_eq!(
            plan_module(unsupported_implicit),
            Err(invalid_use_shape(
                InvalidUseShapeReason::UnsupportedImplicitArgument
            )),
        );

        let mut callback_not_function = compile_use_module();
        expect_use_callback_argument_mut(&mut callback_not_function).value = typed_int_expr(1);
        assert_eq!(
            plan_module(callback_not_function),
            Err(invalid_use_shape(
                InvalidUseShapeReason::CallbackNotFunctionLiteral
            )),
        );

        let mut callback_literal_kind = compile_use_module();
        let (_, kind, _) = expect_use_callback_function_mut(&mut callback_literal_kind);
        *kind = FunctionLiteralKind::Anonymous { head: dummy_span() };
        assert_eq!(
            plan_module(callback_literal_kind),
            Err(invalid_use_shape(
                InvalidUseShapeReason::CallbackLiteralKindNotUse
            )),
        );

        let mut callback_non_function_type = compile_use_module();
        let (type_, _, _) = expect_use_callback_function_mut(&mut callback_non_function_type);
        *type_ = type_::int();
        assert_eq!(
            plan_module(callback_non_function_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        let mut callback_unsupported_function_type = compile_use_module();
        let (type_, _, _) =
            expect_use_callback_function_mut(&mut callback_unsupported_function_type);
        *type_ = type_::list(type_::int());
        assert_eq!(
            plan_module(callback_unsupported_function_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::List,
                },
            }),
        );

        let mut callback_argument_type = compile_use_module();
        let (_, _, arguments) = expect_use_callback_function_mut(&mut callback_argument_type);
        arguments[0].type_ = type_::string();
        assert_eq!(
            plan_module(callback_argument_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected final use statement")]
    fn expect_final_use_mut_panics_on_expression() {
        let mut module = compile_minimal_module();
        expect_final_use_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected use call expression")]
    fn expect_use_call_arguments_mut_panics_on_non_call_rhs() {
        let mut module = compile_use_module();
        *expect_final_use_mut(&mut module).call = typed_int_expr(1);
        expect_use_call_arguments_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "use callback should be a function literal")]
    fn expect_use_callback_function_mut_panics_on_non_function() {
        let mut module = compile_use_module();
        expect_use_callback_argument_mut(&mut module).value = typed_int_expr(1);
        expect_use_callback_function_mut(&mut module);
    }

    #[test]
    fn reject_margin_step_use_statement_shape() {
        let mut step_use = compile_minimal_module();
        step_use.definitions.functions[0].body = vec![
            Statement::Use(gleam_core::ast::Use {
                call: Box::new(typed_int_expr(1)),
                location: dummy_span(),
                right_hand_side_location: dummy_span(),
                assignments_location: dummy_span(),
                assignments: Vec::new(),
            }),
            Statement::Expression(typed_int_expr(1)),
        ];
        assert_eq!(
            plan_module(step_use),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UseStatement,
            }),
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
    }

    #[test]
    fn reject_profile_let_assert_assignment() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert x = 1
  x
}
"#,
            ),
            PlanError::UnsupportedAssignment {
                kind: UnsupportedAssignmentKind::LetAssert,
            },
        );
    }

    #[test]
    fn reject_profile_non_variable_pattern_shapes() {
        let cases = [
            (
                r#"
pub fn main() {
  let value as alias = 1
  alias
}
"#,
                PlanError::UnsupportedPattern {
                    kind: UnsupportedPatternKind::Assign,
                },
            ),
            (
                r#"
pub fn main() {
  let [..rest] = [1]
  rest
}
"#,
                PlanError::UnsupportedPattern {
                    kind: UnsupportedPatternKind::List,
                },
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
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

    fn compile_use_module() -> TypedModule {
        compile(
            r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use value <- with_value
  value
}
"#,
        )
    }

    fn compile_use_with_argument_module() -> TypedModule {
        compile(
            r#"
fn with_value(value: Int, continue: fn(Int) -> Int) {
  continue(value)
}

pub fn main() {
  use value <- with_value(1)
  value
}
"#,
        )
    }

    fn expect_final_use_mut(module: &mut TypedModule) -> &mut TypedUse {
        let main = module
            .definitions
            .functions
            .iter_mut()
            .find(|function| function.name.as_ref().is_some_and(|name| name.1 == "main"))
            .expect("module should have main");
        let [Statement::Use(use_)] = main.body.as_mut_slice() else {
            panic!("expected final use statement");
        };
        use_
    }

    fn expect_use_call_arguments_mut(module: &mut TypedModule) -> &mut Vec<CallArg<TypedExpr>> {
        let use_ = expect_final_use_mut(module);
        let TypedExpr::Call { arguments, .. } = use_.call.as_mut() else {
            panic!("expected use call expression");
        };
        arguments
    }

    fn expect_use_callback_argument_mut(module: &mut TypedModule) -> &mut CallArg<TypedExpr> {
        expect_use_call_arguments_mut(module)
            .last_mut()
            .expect("use call should have callback")
    }

    fn expect_use_callback_function_mut(
        module: &mut TypedModule,
    ) -> (
        &mut std::sync::Arc<gleam_core::type_::Type>,
        &mut FunctionLiteralKind,
        &mut Vec<gleam_core::ast::TypedArg>,
    ) {
        let callback = expect_use_callback_argument_mut(module);
        let TypedExpr::Fn {
            type_,
            kind,
            arguments,
            ..
        } = &mut callback.value
        else {
            panic!("use callback should be a function literal");
        };
        (type_, kind, arguments)
    }

    fn invalid_use_shape(reason: InvalidUseShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::UseShape { reason },
        }
    }
}
