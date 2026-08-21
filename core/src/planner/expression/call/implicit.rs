use super::CaptureSubstitution;
use crate::plan::Expr;
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_compiler_core::ast::{
    CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Statement, TypedExpr,
};

pub(super) fn plan_pipeline_direct_call(
    location: gleam_compiler_core::ast::SrcSpan,
    type_: std::sync::Arc<gleam_compiler_core::type_::Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    pipe_argument(&arguments)?;
    let arguments = super::argument::NormalizedCallArguments::specialized(arguments);

    super::plan_call_expression(location, type_, fun, arguments, context, None)
}

fn pipe_argument(arguments: &[GleamCallArg<TypedExpr>]) -> Result<&TypedExpr, PlanError> {
    let mut pipe_argument = None;
    for (index, argument) in arguments.iter().enumerate() {
        match argument.implicit {
            None => {}
            Some(ImplicitCallArgOrigin::Pipe) => {
                if let Some((first, _)) = pipe_argument.replace((index, &argument.value)) {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::PipelineShape {
                            reason: InvalidPipelineShapeReason::MultiplePipeArguments {
                                first,
                                second: index,
                            },
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
                        reason: InvalidPipelineShapeReason::UnsupportedPipeArgument { index },
                    },
                });
            }
        }
    }

    match pipe_argument {
        Some((_, pipe_argument)) => Ok(pipe_argument),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::MissingPipeArgument,
            },
        }),
    }
}

pub(super) fn plan_pipeline_hole_call(
    _location: gleam_compiler_core::ast::SrcSpan,
    type_: std::sync::Arc<gleam_compiler_core::type_::Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let pipe_argument = pipe_argument(&arguments)?.clone();
    if arguments.len() != 1 {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleWrapperArgumentCount {
                    actual: arguments.len(),
                },
            },
        });
    }
    let pipe_value = super::super::plan_expr(pipe_argument, context)?;
    let PipelineHoleBodyCall {
        location: call_location,
        fun,
        arguments,
        capture_name,
    } = normalize_pipeline_hole_call(fun)?;
    let arguments = super::argument::NormalizedCallArguments::specialized(arguments);

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

struct PipelineHoleBodyCall {
    location: gleam_compiler_core::ast::SrcSpan,
    fun: Box<TypedExpr>,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    capture_name: EcoString,
}

fn normalize_pipeline_hole_call(fun: TypedExpr) -> Result<PipelineHoleBodyCall, PlanError> {
    let TypedExpr::Fn {
        kind,
        arguments: capture_args,
        body,
        ..
    } = fun
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleCaptureFunction,
            },
        });
    };
    if !matches!(kind, FunctionLiteralKind::Capture { .. }) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleCaptureLiteralKind,
            },
        });
    }
    if capture_args.len() != 1 {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleCaptureArgumentCount {
                    actual: capture_args.len(),
                },
            },
        });
    }
    let Some(capture_name) = capture_args[0].names.get_variable_name().cloned() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleCaptureBinding,
            },
        });
    };

    let mut body = body.into_iter();
    let first = body.next();
    let second = body.next();
    let statement = match (first, second) {
        (Some(statement), None) => statement,
        (first, second) => {
            let actual =
                usize::from(first.is_some()) + usize::from(second.is_some()) + body.count();
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PipelineShape {
                    reason: InvalidPipelineShapeReason::HoleBodyStatementCount { actual },
                },
            });
        }
    };
    let Statement::Expression(TypedExpr::Call {
        location,
        fun,
        arguments,
        ..
    }) = statement
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleBodyNotCall,
            },
        });
    };
    if let Some((index, _)) = arguments
        .iter()
        .enumerate()
        .find(|(_, argument)| argument.implicit.is_some())
    {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleBodyImplicitArgument { index },
            },
        });
    }
    let capture_uses = arguments
        .iter()
        .filter(|argument| super::is_capture_local(&argument.value, &capture_name))
        .count();
    if capture_uses != 1 {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::HoleCaptureUseCount {
                    actual: capture_uses,
                },
            },
        });
    }

    Ok(PipelineHoleBodyCall {
        location,
        fun,
        arguments,
        capture_name,
    })
}

#[cfg(test)]
mod tests {
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span};
    use crate::planner::{InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError};
    use gleam_compiler_core::ast::{
        CallArg, Constant, ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule,
    };
    use gleam_compiler_core::type_::{self, ModuleValueConstructor};
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
        statement: &mut gleam_compiler_core::ast::TypedStatement,
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
        statement: &mut gleam_compiler_core::ast::TypedStatement,
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
