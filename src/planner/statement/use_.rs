use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidTypedAstReason, InvalidUseShapeReason, PlanError};
use crate::planner::expression::plan_use_call;
use gleam_core::ast::{Pattern, TypedPattern, TypedUseAssignment};

use super::assignment::{non_variable_pattern_error, plan_binding_pattern};

pub(super) fn plan_use_statement(
    use_: gleam_core::ast::TypedUse,
    context: &mut PlanContext<'_>,
) -> Result<crate::plan::Expr, PlanError> {
    let use_assignment_count = validate_use_assignments(&use_.assignments)?;
    plan_use_call(*use_.call, use_assignment_count, context)
}

fn validate_use_assignments(assignments: &[TypedUseAssignment]) -> Result<usize, PlanError> {
    for assignment in assignments {
        validate_use_assignment_pattern(&assignment.pattern)?;
    }
    Ok(assignments.len())
}

fn validate_use_assignment_pattern(pattern: &TypedPattern) -> Result<(), PlanError> {
    match pattern {
        Pattern::Variable { .. } => Err(invalid_use_shape(
            InvalidUseShapeReason::UnexpectedVariableAssignment,
        )),
        Pattern::Tuple { .. } | Pattern::Assign { .. } => {
            plan_binding_pattern(pattern.clone()).map(|_| ())
        }
        pattern => Err(non_variable_pattern_error(pattern)),
    }
}

fn invalid_use_shape(reason: InvalidUseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::UseShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use super::invalid_use_shape;
    use crate::plan::{Expr, IntLocalId, LocalId, ParamLocal, TupleLocalId, ValueType};
    use crate::planner::dsl::{
        call_int_function, capture_int, function, int, int_arg, int_function_arg,
        int_function_call_arg, int_function_closure, int_function_ref, int_return_tail_call,
        let_tuple_step, local_int, local_int_function, local_tuple, module_with_anonymous, tuple,
        tuple_arg,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidFunctionShapeReason, InvalidTypedAstReason,
        InvalidUseShapeReason, PlanError, UnsupportedExpressionKind, UnsupportedPatternKind,
    };
    use gleam_core::ast::{
        AssignmentKind, CallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Pattern, Statement,
        TypedAssignment, TypedExpr, TypedModule, TypedStatement, TypedUse, UseAssignment,
    };
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

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
  echo 1
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
    fn plan_use_syntax_with_tuple_assignment() {
        let actual = plan_module(compile(
            r#"
fn with_pair(continue: fn(#(Int, Int)) -> Int) {
  continue(#(1, 2))
}

pub fn main() {
  use #(one, two) <- with_pair
  one + two
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let tuple_type = ValueType::Tuple(type_.to_vec());
        let anonymous_ref =
            int_function_ref(2, [ParamLocal::tuple(TupleLocalId(0), type_.to_vec())]);
        let internal_tuple = local_tuple(1, "<tuple:1>", type_.clone());
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_function_arg(0, anonymous_ref)]),
            ),
            [function(
                "with_pair",
                call_int_function(
                    local_int_function(0, "continue", [tuple_type.clone()]),
                    [tuple_arg(0, tuple([int(1), int(2)]))],
                ),
            )
            .param_int_function(0, "continue", [tuple_type])],
            [function(
                "<anonymous:0>",
                local_int(0, "one").add_int(local_int(1, "two")),
            )
            .param_tuple(0, "_use0", type_.clone())
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                local_tuple(0, "_use0", type_.clone()),
            ))
            .let_int(
                0,
                "one",
                local_tuple(1, "<tuple:1>", type_.clone()).index_int(0),
            )
            .let_int(1, "two", internal_tuple.index_int(1))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_use_syntax_with_pattern_alias_assignment() {
        let actual = plan_module(compile(
            r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use value as alias <- with_value
  alias
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
            [function("<anonymous:0>", local_int(2, "alias"))
                .param_int(0, "_use0")
                .let_int(1, "value", local_int(0, "_use0"))
                .let_int(2, "alias", local_int(1, "value"))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_use_syntax_with_nested_tuple_alias_assignment() {
        let actual = plan_module(compile(
            r#"
fn with_pair(continue: fn(#(Int, #(Int, Int))) -> Int) {
  continue(#(1, #(2, 3)))
}

pub fn main() {
  use #(one, #(two, _) as inner) as pair <- with_pair
  one + two + inner.0 + pair.0
}
"#,
        ))
        .expect("source should plan");
        let inner_type = [ValueType::Int, ValueType::Int];
        let outer_type = [
            ValueType::Int,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        ];
        let outer_value_type = ValueType::Tuple(outer_type.to_vec());
        let inner_internal = local_tuple(2, "<tuple:2>", inner_type.clone());
        let inner_alias = local_tuple(3, "inner", inner_type.clone());
        let pair_alias = local_tuple(4, "pair", outer_type.clone());
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_tail_call(
                    1,
                    [int_function_arg(
                        0,
                        int_function_ref(
                            2,
                            [ParamLocal::tuple(TupleLocalId(0), outer_type.to_vec())],
                        ),
                    )],
                ),
            ),
            [function(
                "with_pair",
                call_int_function(
                    local_int_function(0, "continue", [outer_value_type.clone()]),
                    [tuple_arg(
                        0,
                        tuple([
                            Expr::from(int(1)),
                            Expr::from(tuple([Expr::from(int(2)), Expr::from(int(3))])),
                        ]),
                    )],
                ),
            )
            .param_int_function(0, "continue", [outer_value_type])],
            [function(
                "<anonymous:0>",
                local_int(0, "one")
                    .add_int(local_int(1, "two"))
                    .add_int(inner_alias.index_int(0))
                    .add_int(pair_alias.index_int(0)),
            )
            .param_tuple(0, "_use0", outer_type.clone())
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                local_tuple(0, "_use0", outer_type.clone()),
            ))
            .let_int(
                0,
                "one",
                local_tuple(1, "<tuple:1>", outer_type.clone()).index_int(0),
            )
            .step(let_tuple_step(
                2,
                "<tuple:2>",
                local_tuple(1, "<tuple:1>", outer_type.clone()).index_tuple(1, inner_type.clone()),
            ))
            .let_int(
                1,
                "two",
                local_tuple(2, "<tuple:2>", inner_type.clone()).index_int(0),
            )
            .evaluate(local_tuple(2, "<tuple:2>", inner_type.clone()).index_int(1))
            .step(let_tuple_step(3, "inner", inner_internal))
            .step(let_tuple_step(
                4,
                "pair",
                local_tuple(1, "<tuple:1>", outer_type),
            ))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_use_list_assignment() {
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

        let mut missing_generated_assignment = compile_use_module();
        expect_final_use_mut(&mut missing_generated_assignment).assignments = vec![
            UseAssignment {
                location: dummy_span(),
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Variable {
                        location: dummy_span(),
                        name: "value".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    }],
                },
                annotation: None,
            },
            UseAssignment {
                location: dummy_span(),
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Variable {
                        location: dummy_span(),
                        name: "other".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    }],
                },
                annotation: None,
            },
        ];
        assert_eq!(
            plan_module(missing_generated_assignment),
            Err(invalid_use_shape(
                InvalidUseShapeReason::InvalidGeneratedAssignment
            )),
        );

        let mut non_assignment_generated_step = compile_tuple_use_module();
        expect_use_callback_body_mut(&mut non_assignment_generated_step)[0] =
            Statement::Expression(typed_int_expr(1));
        assert_eq!(
            plan_module(non_assignment_generated_step),
            Err(invalid_use_shape(
                InvalidUseShapeReason::InvalidGeneratedAssignment
            )),
        );

        let mut non_generated_assignment = compile_tuple_use_module();
        expect_use_callback_assignment_mut(&mut non_generated_assignment).kind =
            AssignmentKind::Let;
        assert_eq!(
            plan_module(non_generated_assignment),
            Err(invalid_use_shape(
                InvalidUseShapeReason::InvalidGeneratedAssignment
            )),
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
    #[should_panic(expected = "use callback should be a function literal")]
    fn expect_use_callback_body_mut_panics_on_non_function() {
        let mut module = compile_use_module();
        expect_use_callback_argument_mut(&mut module).value = typed_int_expr(1);
        expect_use_callback_body_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected use callback assignment")]
    fn expect_use_callback_assignment_mut_panics_on_expression() {
        let mut module = compile_tuple_use_module();
        expect_use_callback_body_mut(&mut module)[0] = Statement::Expression(typed_int_expr(1));
        expect_use_callback_assignment_mut(&mut module);
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

    fn compile_tuple_use_module() -> TypedModule {
        compile(
            r#"
fn with_pair(continue: fn(#(Int, Int)) -> Int) {
  continue(#(1, 2))
}

pub fn main() {
  use #(one, two) <- with_pair
  one + two
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

    fn expect_use_callback_body_mut(module: &mut TypedModule) -> &mut vec1::Vec1<TypedStatement> {
        let callback = expect_use_callback_argument_mut(module);
        let TypedExpr::Fn { body, .. } = &mut callback.value else {
            panic!("use callback should be a function literal");
        };
        body
    }

    fn expect_use_callback_assignment_mut(module: &mut TypedModule) -> &mut TypedAssignment {
        match &mut expect_use_callback_body_mut(module)[0] {
            Statement::Assignment(assignment) => assignment,
            Statement::Expression(_) | Statement::Use(_) | Statement::Assert(_) => {
                panic!("expected use callback assignment");
            }
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
