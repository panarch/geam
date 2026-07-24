use super::{block, call, plan_expr};
use crate::plan::{Expr, Step};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError, UnsupportedPipelineReason,
};
use crate::planner::statement::plan_variable_runtime_step;
use gleam_core::ast::{PipelineAssignmentKind, TypedExpr, TypedPipelineAssignment};

pub(super) fn plan(
    first_value: TypedPipelineAssignment,
    assignments: Vec<(TypedPipelineAssignment, PipelineAssignmentKind)>,
    finally: TypedExpr,
    finally_kind: PipelineAssignmentKind,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let mut steps = Vec::with_capacity(assignments.len() + 1);
        steps.push(plan_first_assignment(first_value, context)?);
        for (assignment, kind) in assignments {
            steps.push(plan_assignment(assignment, kind, context)?);
        }
        let return_ = plan_pipeline_value(finally, finally_kind, context)?;

        Ok(block::block_expr(steps, return_))
    })
}

fn plan_first_assignment(
    assignment: TypedPipelineAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    let value = plan_expr(*assignment.value, context)?;

    Ok(plan_variable_runtime_step(assignment.name, value, context))
}

fn plan_assignment(
    assignment: TypedPipelineAssignment,
    kind: PipelineAssignmentKind,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    let value = plan_pipeline_value(*assignment.value, kind, context)?;

    Ok(plan_variable_runtime_step(assignment.name, value, context))
}

fn plan_pipeline_value(
    expression: TypedExpr,
    kind: PipelineAssignmentKind,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match kind {
        PipelineAssignmentKind::FirstArgument { .. } | PipelineAssignmentKind::FunctionCall => {
            plan_direct_call(expression, context)
        }
        PipelineAssignmentKind::Hole { .. } => plan_hole_call(expression, context),
        PipelineAssignmentKind::Echo => Err(PlanError::UnsupportedPipeline {
            reason: UnsupportedPipelineReason::Echo,
        }),
    }
}

fn plan_direct_call(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let TypedExpr::Call {
        type_,
        fun,
        arguments,
        ..
    } = expression
    else {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::NonCallStep,
        ));
    };

    call::plan_pipeline_direct_call(type_, *fun, arguments, context)
}

fn plan_hole_call(expression: TypedExpr, context: &mut PlanContext<'_>) -> Result<Expr, PlanError> {
    let TypedExpr::Call {
        type_,
        fun,
        arguments,
        ..
    } = expression
    else {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::NonCallStep,
        ));
    };

    call::plan_pipeline_hole_call(type_, *fun, arguments, context)
}

fn invalid_pipeline_shape(reason: InvalidPipelineShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PipelineShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{IntLocalId, LocalId};
    use crate::planner::dsl::{
        block_int, bool_, bool_arg, bool_return_block, bool_return_tail_call, call_int,
        call_int_function, function, int, int_arg, int_function_call_arg, int_function_closure,
        int_function_ref, int_return_block, int_return_expr, int_return_tail_call, let_bool_step,
        let_int_function_step, let_int_step, let_nil_step, let_string_step, local_bool, local_int,
        local_int_function, local_nil, local_string, module, module_with_anonymous, nil, nil_arg,
        nil_return_block, nil_return_tail_call, string, string_arg, string_return_block,
        string_return_tail_call,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionShapeKind, InvalidExpressionType,
        InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason, UnsupportedPipelineReason,
    };
    use gleam_core::ast::{
        ArgNames, CallArg, FunctionLiteralKind, ImplicitCallArgOrigin, PipelineAssignmentKind,
        Statement, TypedArg, TypedExpr, TypedPipelineAssignment, TypedStatement,
    };
    use gleam_core::type_::{self, ValueConstructor, ValueConstructorVariant};
    use std::sync::Arc;
    use vec1::Vec1;

    #[test]
    fn plan_pipeline_direct_local_function() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  1 |> add_one
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_tail_call(1, [int_arg(local_int(0, "_pipe"))]),
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_first_argument_call() {
        let actual = plan_module(compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  1 |> add(2)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_tail_call(1, [int_arg(local_int(0, "_pipe")), int_arg(int(2))]),
                ),
            ),
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_chained_pipeline() {
        let actual = plan_module(compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

fn multiply(left: Int, right: Int) {
  left * right
}

pub fn main() {
  1 |> add(2) |> multiply(3)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [
                        let_int_step(0, "_pipe", int(1)),
                        let_int_step(
                            1,
                            "_pipe",
                            call_int(1, [int_arg(local_int(0, "_pipe")), int_arg(int(2))]),
                        ),
                    ],
                    int_return_tail_call(2, [int_arg(local_int(1, "_pipe")), int_arg(int(3))]),
                ),
            ),
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
                function(
                    "multiply",
                    local_int(0, "left").mult_int(local_int(1, "right")),
                )
                .param_int(0, "left")
                .param_int(1, "right"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_block_first_value() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  {
    let x = 1
    x
  }
  |> add_one
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(
                        1,
                        "_pipe",
                        block_int([let_int_step(0, "x", int(1))], local_int(0, "x")),
                    )],
                    int_return_tail_call(1, [int_arg(local_int(1, "_pipe"))]),
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_explicit_hole() {
        let actual = plan_module(compile(
            r#"
fn subtract(left: Int, right: Int) {
  left - right
}

pub fn main() {
  1 |> subtract(10, _)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_tail_call(1, [int_arg(int(10)), int_arg(local_int(0, "_pipe"))]),
                ),
            ),
            [function(
                "subtract",
                local_int(0, "left").sub_int(local_int(1, "right")),
            )
            .param_int(0, "left")
            .param_int(1, "right")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_labelled_direct_call_uses_function_param_order() {
        let actual = plan_module(compile(
            r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  2 |> add(to: 40)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(2))],
                    int_return_tail_call(1, [int_arg(int(40)), int_arg(local_int(0, "_pipe"))]),
                ),
            ),
            [
                function("add", local_int(0, "base").add_int(local_int(1, "amount")))
                    .param_int(0, "base")
                    .param_int(1, "amount"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_labelled_hole_call_uses_hole_position() {
        let actual = plan_module(compile(
            r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  2 |> add(to: 40, value: _)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(2))],
                    int_return_tail_call(1, [int_arg(int(40)), int_arg(local_int(0, "_pipe"))]),
                ),
            ),
            [
                function("add", local_int(0, "base").add_int(local_int(1, "amount")))
                    .param_int(0, "base")
                    .param_int(1, "amount"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_function_value_call_shape() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let f = add_one
  1 |> f
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_expr(call_int_function(
                        local_int_function(0, "f", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "_pipe"))],
                    )),
                ),
            )
            .step(let_int_function_step(
                0,
                "f",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_anonymous_function_value_call_shape() {
        let actual = plan_module(compile(r#"pub fn main() { 1 |> fn(value) { value } }"#))
            .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_expr(call_int_function(
                        int_function_closure(1, [LocalId::Int(IntLocalId(0))], []),
                        [int_function_call_arg(local_int(0, "_pipe"))],
                    )),
                ),
            ),
            [],
            [function("<anonymous:0>", local_int(0, "value")).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_function_value_hole_call_shape() {
        let actual = plan_module(compile(
            r#"
fn subtract(left: Int, right: Int) {
  left - right
}

pub fn main() {
  let f = subtract
  1 |> f(10, _)
}
"#,
        ))
        .expect("source should plan");
        let params = [LocalId::Int(IntLocalId(0)), LocalId::Int(IntLocalId(1))];
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_expr(call_int_function(
                        local_int_function(0, "f", params),
                        [
                            int_function_call_arg(int(10)),
                            int_function_call_arg(local_int(0, "_pipe")),
                        ],
                    )),
                ),
            )
            .step(let_int_function_step(0, "f", int_function_ref(1, params))),
            [function(
                "subtract",
                local_int(0, "left").sub_int(local_int(1, "right")),
            )
            .param_int(0, "left")
            .param_int(1, "right")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_middle_explicit_hole() {
        let actual = plan_module(compile(
            r#"
fn three_arg(a: Int, b: Int, c: Int) {
  a + b + c
}

pub fn main() {
  1 |> three_arg(10, _, 3)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_tail_call(
                        1,
                        [
                            int_arg(int(10)),
                            int_arg(local_int(0, "_pipe")),
                            int_arg(int(3)),
                        ],
                    ),
                ),
            ),
            [function(
                "three_arg",
                local_int(0, "a")
                    .add_int(local_int(1, "b"))
                    .add_int(local_int(2, "c")),
            )
            .param_int(0, "a")
            .param_int(1, "b")
            .param_int(2, "c")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_pipeline_return_types() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  "geam" |> identity_string
}

fn identity_string(value: String) {
  value
}

pub fn bool_main() {
  True |> identity_bool
}

fn identity_bool(value: Bool) {
  value
}

pub fn nil_main() {
  Nil |> identity_nil
}

fn identity_nil(value: Nil) {
  value
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "_pipe", string("geam"))],
                    string_return_tail_call(1, [string_arg(local_string(0, "_pipe"))]),
                ),
            ),
            [
                function("identity_string", local_string(0, "value")).param_string(0, "value"),
                function("identity_bool", local_bool(0, "value")).param_bool(0, "value"),
                function(
                    "bool_main",
                    bool_return_block(
                        [let_bool_step(0, "_pipe", bool_(true))],
                        bool_return_tail_call(2, [bool_arg(local_bool(0, "_pipe"))]),
                    ),
                ),
                function("identity_nil", local_nil(0, "value")).param_nil(0, "value"),
                function(
                    "nil_main",
                    nil_return_block(
                        [let_nil_step(0, "_pipe", nil())],
                        nil_return_tail_call(4, [nil_arg(local_nil(0, "_pipe"))]),
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_pipeline_forms() {
        assert_eq!(
            expect_plan_error(r#"pub fn main() { 1 |> echo }"#),
            PlanError::UnsupportedPipeline {
                reason: UnsupportedPipelineReason::Echo,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  {
    <<1:native>>
    1
  } |> add(1)
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  1 |> add({
    <<1:native>>
    1
  })
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn reject_margin_pipeline_shapes() {
        let mut unsupported_first_assignment = compile_pipeline_module();
        let (first_value, _, _, _) = expect_pipeline_statement_mut(
            &mut unsupported_first_assignment.definitions.functions[1].body[0],
        );
        *first_value.value = TypedExpr::Invalid {
            location: dummy_span(),
            type_: type_::int(),
            extra_information: None,
        };
        assert_eq!(
            plan_module(unsupported_first_assignment),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );

        let mut non_call_step = compile_pipeline_module();
        let (_, _, finally, _) =
            expect_pipeline_statement_mut(&mut non_call_step.definitions.functions[1].body[0]);
        **finally = super::super::typed_int_expr(1);
        assert_eq!(
            plan_module(non_call_step),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::NonCallStep,
            )),
        );

        let mut missing_pipe_argument = compile_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut missing_pipe_argument.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(finally);
        arguments[0].implicit = None;
        assert_eq!(
            plan_module(missing_pipe_argument),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::MissingPipeArgument,
            )),
        );

        let mut multiple_pipe_arguments = compile_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut multiple_pipe_arguments.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(finally);
        arguments.push(arguments[0].clone());
        assert_eq!(
            plan_module(multiple_pipe_arguments),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::MultiplePipeArguments,
            )),
        );

        let mut unsupported_implicit_argument = compile_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut unsupported_implicit_argument.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(finally);
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Use);
        assert_eq!(
            plan_module(unsupported_implicit_argument),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::UnsupportedImplicitArgument,
            )),
        );

        let mut return_type_mismatch = compile_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut return_type_mismatch.definitions.functions[1].body[0],
        );
        let (type_, _, _) = expect_pipeline_final_call_mut(finally);
        *type_ = type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_pipeline_labelled_argument_reaches_call_boundary() {
        let mut labelled_argument = compile_pipeline_module();
        let (_, _, finally, _) =
            expect_pipeline_statement_mut(&mut labelled_argument.definitions.functions[1].body[0]);
        let arguments = expect_call_arguments_mut(finally);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_argument),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_pipeline_hole_kind_shapes() {
        let mut invalid_hole_capture = compile_pipeline_module();
        let (_, _, _, finally_kind) = expect_pipeline_statement_mut(
            &mut invalid_hole_capture.definitions.functions[1].body[0],
        );
        *finally_kind = PipelineAssignmentKind::Hole { hole: dummy_span() };
        assert_eq!(
            plan_module(invalid_hole_capture),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );
    }

    #[test]
    fn reject_margin_pipeline_hole_capture_shapes() {
        let mut missing_pipe_argument = compile_hole_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut missing_pipe_argument.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(finally);
        let pipe_argument = arguments
            .iter_mut()
            .find(|argument| argument.implicit == Some(ImplicitCallArgOrigin::Pipe))
            .expect("expected pipe argument");
        pipe_argument.implicit = None;
        assert_eq!(
            plan_module(missing_pipe_argument),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::MissingPipeArgument,
            )),
        );

        let mut unsupported_pipe_value = compile_hole_pipeline_module();
        let (_, _, finally, _) = expect_pipeline_statement_mut(
            &mut unsupported_pipe_value.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(finally);
        let pipe_argument = arguments
            .iter_mut()
            .find(|argument| argument.implicit == Some(ImplicitCallArgOrigin::Pipe))
            .expect("expected pipe argument");
        pipe_argument.value = typed_list_expr();
        assert_eq!(
            plan_module(unsupported_pipe_value),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::List,
                },
            }),
        );

        let mut missing_capture_arg = compile_hole_pipeline_module();
        let (_, capture_args, _) = expect_pipeline_hole_capture_mut(
            &mut missing_capture_arg.definitions.functions[1].body[0],
        );
        capture_args.clear();
        assert_eq!(
            plan_module(missing_capture_arg),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut non_capture_function_kind = compile_hole_pipeline_module();
        let (kind, _, _) = expect_pipeline_hole_capture_mut(
            &mut non_capture_function_kind.definitions.functions[1].body[0],
        );
        *kind = FunctionLiteralKind::Anonymous { head: dummy_span() };
        assert_eq!(
            plan_module(non_capture_function_kind),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut discard_capture_arg = compile_hole_pipeline_module();
        let (_, capture_args, _) = expect_pipeline_hole_capture_mut(
            &mut discard_capture_arg.definitions.functions[1].body[0],
        );
        capture_args[0].names = ArgNames::Discard {
            name: "_".into(),
            location: dummy_span(),
        };
        assert_eq!(
            plan_module(discard_capture_arg),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut non_call_body = compile_hole_pipeline_module();
        let (_, _, body) =
            expect_pipeline_hole_capture_mut(&mut non_call_body.definitions.functions[1].body[0]);
        body[0] = Statement::Expression(super::super::typed_int_expr(1));
        assert_eq!(
            plan_module(non_call_body),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::NonCallStep,
            )),
        );

        let mut extra_body_statement = compile_hole_pipeline_module();
        let (_, _, body) = expect_pipeline_hole_capture_mut(
            &mut extra_body_statement.definitions.functions[1].body[0],
        );
        body.push(Statement::Expression(super::super::typed_int_expr(1)));
        assert_eq!(
            plan_module(extra_body_statement),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut implicit_inner_argument = compile_hole_pipeline_module();
        let (_, _, body) = expect_pipeline_hole_capture_mut(
            &mut implicit_inner_argument.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(expect_expression_statement_mut(&mut body[0]));
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Use);
        assert_eq!(
            plan_module(implicit_inner_argument),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::UnsupportedImplicitArgument,
            )),
        );

        let mut missing_capture_usage = compile_hole_pipeline_module();
        let (_, _, body) = expect_pipeline_hole_capture_mut(
            &mut missing_capture_usage.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(expect_expression_statement_mut(&mut body[0]));
        arguments[1].value = super::super::typed_int_expr(2);
        assert_eq!(
            plan_module(missing_capture_usage),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut duplicate_capture_usage = compile_hole_pipeline_module();
        let (_, _, body) = expect_pipeline_hole_capture_mut(
            &mut duplicate_capture_usage.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(expect_expression_statement_mut(&mut body[0]));
        arguments[0].value = arguments[1].value.clone();
        assert_eq!(
            plan_module(duplicate_capture_usage),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );

        let mut non_local_capture_argument = compile_hole_pipeline_module();
        let (_, _, body) = expect_pipeline_hole_capture_mut(
            &mut non_local_capture_argument.definitions.functions[1].body[0],
        );
        let arguments = expect_call_arguments_mut(expect_expression_statement_mut(&mut body[0]));
        let constructor = expect_var_constructor_mut(&mut arguments[1].value);
        constructor.variant = ValueConstructorVariant::Record {
            name: "_capture".into(),
            arity: 0,
            field_map: None,
            location: dummy_span(),
            module: "main".into(),
            variants_count: 1,
            variant_index: 0,
            documentation: None,
        };
        assert_eq!(
            plan_module(non_local_capture_argument),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::InvalidHoleCapture,
            )),
        );
    }

    #[test]
    fn reject_margin_pipeline_hole_non_call_step() {
        let mut non_call_hole_step = compile_pipeline_module();
        let (_, _, finally, finally_kind) =
            expect_pipeline_statement_mut(&mut non_call_hole_step.definitions.functions[1].body[0]);
        **finally = super::super::typed_int_expr(1);
        *finally_kind = PipelineAssignmentKind::Hole { hole: dummy_span() };
        assert_eq!(
            plan_module(non_call_hole_step),
            Err(invalid_pipeline_shape(
                InvalidPipelineShapeReason::NonCallStep,
            )),
        );
    }

    fn compile_pipeline_module() -> gleam_core::ast::TypedModule {
        compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  1 |> add_one
}
"#,
        )
    }

    fn compile_hole_pipeline_module() -> gleam_core::ast::TypedModule {
        compile(
            r#"
fn subtract(left: Int, right: Int) {
  left - right
}

pub fn main() {
  1 |> subtract(10, _)
}
"#,
        )
    }

    fn expect_pipeline_statement_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut TypedPipelineAssignment,
        &mut Vec<(TypedPipelineAssignment, PipelineAssignmentKind)>,
        &mut Box<TypedExpr>,
        &mut PipelineAssignmentKind,
    ) {
        let Statement::Expression(TypedExpr::Pipeline {
            first_value,
            assignments,
            finally,
            finally_kind,
            ..
        }) = statement
        else {
            panic!("expected pipeline expression statement");
        };

        (first_value, assignments, finally, finally_kind)
    }

    fn typed_list_expr() -> TypedExpr {
        let mut module = compile(
            r#"
pub fn main() {
  [1]
}
"#,
        );
        let mut statement = module.definitions.functions[0].body.remove(0);

        expect_expression_statement_mut(&mut statement).clone()
    }

    #[test]
    #[should_panic(expected = "expected pipeline expression statement")]
    fn expect_pipeline_statement_mut_panics_on_int() {
        let mut module = crate::planner::support::compile_minimal_module();

        expect_pipeline_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    fn expect_call_arguments_mut(expression: &mut TypedExpr) -> &mut Vec<CallArg<TypedExpr>> {
        let TypedExpr::Call { arguments, .. } = expression else {
            panic!("expected call expression");
        };

        arguments
    }

    fn expect_pipeline_final_call_mut(
        expression: &mut TypedExpr,
    ) -> (
        &mut Arc<type_::Type>,
        &mut Box<TypedExpr>,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        let TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        } = expression
        else {
            panic!("expected pipeline final call");
        };

        (type_, fun, arguments)
    }

    #[test]
    #[should_panic(expected = "expected pipeline final call")]
    fn expect_pipeline_final_call_mut_panics_on_int() {
        let mut expression = super::super::typed_int_expr(1);

        expect_pipeline_final_call_mut(&mut expression);
    }

    fn expect_pipeline_hole_capture_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut FunctionLiteralKind,
        &mut Vec<TypedArg>,
        &mut Vec1<TypedStatement>,
    ) {
        let (_, _, finally, _) = expect_pipeline_statement_mut(statement);
        let (_, fun, _) = expect_pipeline_final_call_mut(finally);
        let TypedExpr::Fn {
            kind,
            arguments,
            body,
            ..
        } = fun.as_mut()
        else {
            panic!("expected pipeline hole capture function");
        };

        (kind, arguments, body)
    }

    #[test]
    #[should_panic(expected = "expected pipeline hole capture function")]
    fn expect_pipeline_hole_capture_mut_panics_on_direct_call() {
        let mut module = compile_pipeline_module();

        expect_pipeline_hole_capture_mut(&mut module.definitions.functions[1].body[0]);
    }

    fn expect_expression_statement_mut(statement: &mut TypedStatement) -> &mut TypedExpr {
        let Statement::Expression(expression) = statement else {
            panic!("expected expression statement");
        };

        expression
    }

    fn expect_var_constructor_mut(expression: &mut TypedExpr) -> &mut ValueConstructor {
        let TypedExpr::Var { constructor, .. } = expression else {
            panic!("expected variable expression");
        };

        constructor
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_var_constructor_mut_panics_on_int() {
        let mut expression = super::super::typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn expect_expression_statement_mut_panics_on_assignment() {
        let mut module = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        expect_expression_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected call expression")]
    fn expect_call_arguments_mut_panics_on_int() {
        let mut expression = super::super::typed_int_expr(1);

        expect_call_arguments_mut(&mut expression);
    }

    fn invalid_pipeline_shape(reason: InvalidPipelineShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape { reason },
        }
    }
}
