use super::{CallArgumentMode, CaptureSubstitution, FunctionValueCallMode};
use crate::plan::Expr;
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCallShapeReason, InvalidPipelineShapeReason, InvalidTypedAstReason,
    InvalidUseShapeReason, PlanError,
};
use ecow::EcoString;
use gleam_core::ast::{
    CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Statement, TypedArg,
    TypedExpr, TypedStatement,
};
use vec1::Vec1;

pub(super) fn plan_use_call(
    call: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match call {
        TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        } => {
            validate_use_call_arguments(&arguments)?;
            super::plan_call_expression(
                type_,
                *fun,
                arguments,
                context,
                None,
                CallArgumentMode::Use,
                FunctionValueCallMode::Allow,
            )
        }
        _ => Err(super::invalid_use_shape(InvalidUseShapeReason::NonCallRhs)),
    }
}

pub(super) fn plan_pipeline_direct_call(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    pipe_argument(&arguments)?;

    super::plan_call_expression(
        type_,
        fun,
        arguments,
        context,
        None,
        CallArgumentMode::Normal,
        FunctionValueCallMode::Reject,
    )
}

pub(super) fn plan_pipeline_hole_call(
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
    let (fun, arguments) = pipeline_hole_body_call(body.next())?;
    if body.next().is_some() {
        return Err(invalid_hole_capture());
    }
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::LabelledArguments,
            },
        });
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
        type_,
        *fun,
        arguments,
        context,
        Some(&CaptureSubstitution {
            name: capture_name,
            value: pipe_value,
        }),
        CallArgumentMode::Normal,
        FunctionValueCallMode::Reject,
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

fn pipeline_hole_body_call(
    statement: Option<TypedStatement>,
) -> Result<(Box<TypedExpr>, Vec<GleamCallArg<TypedExpr>>), PlanError> {
    match statement {
        Some(Statement::Expression(TypedExpr::Call { fun, arguments, .. })) => Ok((fun, arguments)),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::NonCallStep,
            },
        }),
    }
}

fn validate_use_call_arguments(arguments: &[GleamCallArg<TypedExpr>]) -> Result<(), PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LabelledArguments,
            },
        });
    }

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

    match callback_index {
        Some(index) if index + 1 == arguments.len() => Ok(()),
        Some(_) => Err(super::invalid_use_shape(
            InvalidUseShapeReason::CallbackNotLast,
        )),
        None => Err(super::invalid_use_shape(
            InvalidUseShapeReason::MissingCallback,
        )),
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

fn pipe_argument(arguments: &[GleamCallArg<TypedExpr>]) -> Result<&TypedExpr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::LabelledArguments,
            },
        });
    }

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

fn invalid_hole_capture() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PipelineShape {
            reason: InvalidPipelineShapeReason::InvalidHoleCapture,
        },
    }
}
