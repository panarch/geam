use super::CaptureSubstitution;
use crate::plan::Expr;
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidPipelineShapeReason, InvalidTypedAstReason, InvalidUseShapeReason, PlanError,
};
use ecow::EcoString;
use gleam_core::ast::{
    AssignmentKind, CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Statement,
    TypedArg, TypedExpr, TypedStatement,
};
use vec1::Vec1;

pub(super) fn plan_use_call(
    call: TypedExpr,
    use_assignments: Vec<super::UseAssignmentNormalization>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match call {
        TypedExpr::Call {
            location,
            type_,
            fun,
            arguments,
            ..
        } => {
            let arguments = normalize_use_call_arguments(arguments, use_assignments)?;
            super::plan_call_expression(location, type_, *fun, arguments, context, None)
        }
        _ => Err(super::invalid_use_shape(InvalidUseShapeReason::NonCallRhs)),
    }
}

fn normalize_use_call_arguments(
    mut arguments: Vec<GleamCallArg<TypedExpr>>,
    use_assignments: Vec<super::UseAssignmentNormalization>,
) -> Result<Vec<GleamCallArg<TypedExpr>>, PlanError> {
    let mut callback_index = None;
    for (index, argument) in arguments.iter().enumerate() {
        match argument.implicit {
            None => {}
            Some(ImplicitCallArgOrigin::Use) => {
                if callback_index.replace(index).is_some() {
                    return Err(super::invalid_use_shape(
                        InvalidUseShapeReason::MultipleCallbacks,
                    ));
                }
            }
            Some(
                ImplicitCallArgOrigin::Pipe
                | ImplicitCallArgOrigin::PatternFieldSpread
                | ImplicitCallArgOrigin::IncorrectArityUse
                | ImplicitCallArgOrigin::RecordUpdate,
            ) => {
                return Err(super::invalid_use_shape(
                    InvalidUseShapeReason::UnsupportedImplicitArgument,
                ));
            }
        }
    }

    let callback_index = match callback_index {
        Some(index) if index + 1 == arguments.len() => index,
        Some(_) => Err(super::invalid_use_shape(
            InvalidUseShapeReason::CallbackNotLast,
        ))?,
        None => Err(super::invalid_use_shape(
            InvalidUseShapeReason::MissingCallback,
        ))?,
    };

    let callback = &mut arguments[callback_index];
    callback.implicit = None;
    normalize_use_callback(&mut callback.value, use_assignments)?;

    Ok(arguments)
}

fn normalize_use_callback(
    callback: &mut TypedExpr,
    use_assignments: Vec<super::UseAssignmentNormalization>,
) -> Result<(), PlanError> {
    match callback {
        TypedExpr::Fn { kind, body, .. } => match kind {
            FunctionLiteralKind::Use { location } => {
                normalize_use_generated_assignments(body, use_assignments)?;
                *kind = FunctionLiteralKind::Anonymous { head: *location };
                Ok(())
            }
            FunctionLiteralKind::Anonymous { .. } | FunctionLiteralKind::Capture { .. } => Err(
                super::invalid_use_shape(InvalidUseShapeReason::CallbackLiteralKindNotUse),
            ),
        },
        _ => Err(super::invalid_use_shape(
            InvalidUseShapeReason::CallbackNotFunctionLiteral,
        )),
    }
}

fn normalize_use_generated_assignments(
    body: &mut Vec1<TypedStatement>,
    use_assignments: Vec<super::UseAssignmentNormalization>,
) -> Result<(), PlanError> {
    let statements = body.as_mut_slice();
    let use_assignment_count = use_assignments.len();
    if statements.len() < use_assignment_count {
        return Err(super::invalid_use_shape(
            InvalidUseShapeReason::InvalidGeneratedAssignment,
        ));
    }

    for (statement, use_assignment) in statements[..use_assignment_count]
        .iter_mut()
        .zip(use_assignments)
    {
        let (expected, normalized) = use_assignment.into_parts();
        match statement {
            Statement::Assignment(assignment)
                if matches!(assignment.kind, AssignmentKind::Generated)
                    && assignment.pattern == expected =>
            {
                assignment.kind = AssignmentKind::Let;
                assignment.pattern = normalized;
            }
            _ => {
                return Err(super::invalid_use_shape(
                    InvalidUseShapeReason::InvalidGeneratedAssignment,
                ));
            }
        }
    }

    Ok(())
}

pub(super) fn plan_pipeline_direct_call(
    location: gleam_core::ast::SrcSpan,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    pipe_argument(&arguments)?;

    super::plan_call_expression(location, type_, fun, arguments, context, None)
}

fn pipe_argument(arguments: &[GleamCallArg<TypedExpr>]) -> Result<&TypedExpr, PlanError> {
    let mut pipe_argument = None;
    for argument in arguments {
        match argument.implicit {
            None => {}
            Some(ImplicitCallArgOrigin::Pipe) => {
                if pipe_argument.replace(&argument.value).is_some() {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::PipelineShape {
                            reason: InvalidPipelineShapeReason::MultiplePipeArguments,
                        },
                    });
                }
            }
            Some(
                ImplicitCallArgOrigin::Use
                | ImplicitCallArgOrigin::PatternFieldSpread
                | ImplicitCallArgOrigin::IncorrectArityUse
                | ImplicitCallArgOrigin::RecordUpdate,
            ) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::PipelineShape {
                        reason: InvalidPipelineShapeReason::UnsupportedImplicitArgument,
                    },
                });
            }
        }
    }

    match pipe_argument {
        Some(pipe_argument) => Ok(pipe_argument),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::MissingPipeArgument,
            },
        }),
    }
}

pub(super) fn plan_pipeline_hole_call(
    _location: gleam_core::ast::SrcSpan,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let pipe_value = super::super::plan_expr(pipe_argument(&arguments)?.clone(), context)?;

    let (kind, capture_args, body) = pipeline_capture_function_parts(fun)?;
    if !matches!(kind, FunctionLiteralKind::Capture { .. }) {
        return Err(invalid_hole_capture());
    }
    let capture_arg = single_capture_argument(&capture_args)?;
    let capture_name = match capture_arg.names.get_variable_name().cloned() {
        Some(capture_name) => capture_name,
        None => return Err(invalid_hole_capture()),
    };

    let mut body = body.into_iter();
    let PipelineHoleBodyCall {
        location: call_location,
        fun,
        arguments,
    } = pipeline_hole_body_call(body.next())?;
    if body.next().is_some() {
        return Err(invalid_hole_capture());
    }
    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::UnsupportedImplicitArgument,
            },
        });
    }
    if count_capture_arguments(&arguments, &capture_name) != 1 {
        return Err(invalid_hole_capture());
    }

    super::plan_call_expression(
        call_location,
        type_,
        *fun,
        arguments,
        context,
        Some(&CaptureSubstitution {
            name: capture_name,
            value: pipe_value,
        }),
    )
}

fn pipeline_capture_function_parts(
    fun: TypedExpr,
) -> Result<(FunctionLiteralKind, Vec<TypedArg>, Vec1<TypedStatement>), PlanError> {
    match fun {
        TypedExpr::Fn {
            kind,
            arguments,
            body,
            ..
        } => Ok((kind, arguments, body)),
        _ => Err(invalid_hole_capture()),
    }
}

fn single_capture_argument(capture_args: &[TypedArg]) -> Result<&TypedArg, PlanError> {
    if capture_args.len() == 1 {
        Ok(&capture_args[0])
    } else {
        Err(invalid_hole_capture())
    }
}

struct PipelineHoleBodyCall {
    location: gleam_core::ast::SrcSpan,
    fun: Box<TypedExpr>,
    arguments: Vec<GleamCallArg<TypedExpr>>,
}

fn pipeline_hole_body_call(
    statement: Option<TypedStatement>,
) -> Result<PipelineHoleBodyCall, PlanError> {
    match statement {
        Some(Statement::Expression(TypedExpr::Call {
            location,
            fun,
            arguments,
            ..
        })) => Ok(PipelineHoleBodyCall {
            location,
            fun,
            arguments,
        }),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::NonCallStep,
            },
        }),
    }
}

fn count_capture_arguments(
    arguments: &[GleamCallArg<TypedExpr>],
    capture_name: &EcoString,
) -> usize {
    arguments
        .iter()
        .filter(|argument| super::is_capture_local(&argument.value, capture_name))
        .count()
}

fn invalid_hole_capture() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PipelineShape {
            reason: InvalidPipelineShapeReason::InvalidHoleCapture,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span};
    use crate::planner::{InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError};
    use gleam_core::ast::{
        CallArg, Constant, ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule,
    };
    use gleam_core::type_::{self, ModuleValueConstructor};
    use num_bigint::BigInt;

    #[test]
    fn reject_margin_pipeline_hole_call_pipe_value_expression_shape() {
        let mut module = compile_hole_pipeline_module();
        let pipe_argument = expect_pipe_argument_mut(&mut module.definitions.functions[1].body[0]);
        pipe_argument.value = typed_module_select_expr();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected pipeline expression statement")]
    fn expect_pipe_argument_mut_panics_on_non_pipeline() {
        let mut module = compile_minimal_module();

        expect_pipe_argument_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected pipeline final call")]
    fn expect_pipe_argument_mut_panics_on_non_call_final() {
        let mut module = compile_hole_pipeline_module();
        let finally = expect_pipeline_final_mut(&mut module.definitions.functions[1].body[0]);
        **finally = super::super::super::typed_int_expr(1);

        expect_pipe_argument_mut(&mut module.definitions.functions[1].body[0]);
    }

    fn compile_hole_pipeline_module() -> TypedModule {
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

    fn expect_pipe_argument_mut(
        statement: &mut gleam_core::ast::TypedStatement,
    ) -> &mut CallArg<TypedExpr> {
        let finally = expect_pipeline_final_mut(statement);
        let TypedExpr::Call { arguments, .. } = finally.as_mut() else {
            panic!("expected pipeline final call");
        };

        arguments
            .iter_mut()
            .find(|argument| argument.implicit == Some(ImplicitCallArgOrigin::Pipe))
            .expect("expected pipe argument")
    }

    fn expect_pipeline_final_mut(
        statement: &mut gleam_core::ast::TypedStatement,
    ) -> &mut Box<TypedExpr> {
        let Statement::Expression(TypedExpr::Pipeline { finally, .. }) = statement else {
            panic!("expected pipeline expression statement");
        };

        finally
    }

    fn typed_module_select_expr() -> TypedExpr {
        TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: type_::int(),
            label: "answer".into(),
            module_name: "other".into(),
            module_alias: "other".into(),
            constructor: ModuleValueConstructor::Constant {
                literal: Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
                location: dummy_span(),
                documentation: None,
            },
        }
    }
}
