use super::{invalid_expression_type, invalid_expression_type_for_value, plan_expr};
use crate::plan::{
    BoolExpr, CallArg, Expr, FunctionCallArg, FunctionExpr, FunctionFunctionExpr, IntExpr, NilExpr,
    RuntimeFunctionId, StringExpr, ValueType,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidPipelineShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedPipelineReason,
};
use ecow::EcoString;
use gleam_core::ast::{
    CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Statement, TypedArg,
    TypedExpr, TypedStatement,
};
use gleam_core::type_::{Type, ValueConstructorVariant};
use std::sync::Arc;
use vec1::Vec1;

pub(super) fn plan_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LabelledArguments,
            },
        });
    }

    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ImplicitArguments,
            },
        });
    }

    plan_call_expression(
        type_,
        fun,
        arguments,
        context,
        None,
        FunctionValueCallMode::Allow,
    )
}

pub(super) fn plan_pipeline_direct_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    pipe_argument(&arguments)?;

    plan_call_expression(
        type_,
        fun,
        arguments,
        context,
        None,
        FunctionValueCallMode::Reject,
    )
}

pub(super) fn plan_pipeline_hole_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let pipe_value = plan_expr(pipe_argument(&arguments)?.clone(), context)?;

    let (kind, capture_args, body) = pipeline_capture_function_parts(fun)?;
    if !matches!(kind, FunctionLiteralKind::Capture { .. }) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
    }
    let capture_arg = single_capture_argument(&capture_args)?;
    let capture_name = match capture_arg.names.get_variable_name().cloned() {
        Some(capture_name) => capture_name,
        None => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PipelineShape {
                    reason: InvalidPipelineShapeReason::InvalidHoleCapture,
                },
            });
        }
    };

    let mut body = body.into_iter();
    let (fun, arguments) = pipeline_hole_body_call(body.next())?;
    if body.next().is_some() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
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
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
    }

    plan_call_expression(
        type_,
        *fun,
        arguments,
        context,
        Some(&CaptureSubstitution {
            name: capture_name,
            value: pipe_value,
        }),
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
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        }),
    }
}

fn single_capture_argument(capture_args: &[TypedArg]) -> Result<&TypedArg, PlanError> {
    if capture_args.len() == 1 {
        Ok(&capture_args[0])
    } else {
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        })
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

struct CaptureSubstitution {
    name: EcoString,
    value: Expr,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FunctionValueCallMode {
    Allow,
    Reject,
}

fn plan_call_expression(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
    function_value_call_mode: FunctionValueCallMode,
) -> Result<Expr, PlanError> {
    if let TypedExpr::Var { constructor, .. } = &fun {
        match &constructor.variant {
            ValueConstructorVariant::ModuleFn {
                module,
                name,
                external_erlang,
                external_javascript,
                ..
            } if module == context.module_name
                && external_erlang.is_none()
                && external_javascript.is_none() =>
            {
                let function = context
                    .lookup_function(name)
                    .ok_or(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::CallShape {
                            reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                        },
                    })?;
                return plan_direct_function_call(type_, function, arguments, context, capture);
            }
            ValueConstructorVariant::ModuleConstant { .. } => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::ModuleConstant,
                    },
                });
            }
            ValueConstructorVariant::Record { .. } => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::RecordConstructor,
                    },
                });
            }
            ValueConstructorVariant::ModuleFn { .. } => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::NonCurrentModuleFunction,
                    },
                });
            }
            ValueConstructorVariant::LocalVariable { .. } => {}
        }
    }

    if function_value_call_mode == FunctionValueCallMode::Reject {
        return Err(PlanError::UnsupportedPipeline {
            reason: UnsupportedPipelineReason::FunctionValueCall,
        });
    }

    plan_function_value_call(type_, fun, arguments, context, capture)
}

fn plan_direct_function_call(
    type_: Arc<Type>,
    function: FunctionInfo,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    if function.arity() != arguments.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
            },
        });
    }
    let function_return_type = function.return_type();
    let function_id = function.runtime_id;
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
        },
    })?;
    if return_type != function_return_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
            },
        });
    }
    let args = plan_call_args(arguments, &function.params, context, capture)?;

    Ok(call_expr(function_id, args))
}

fn plan_function_value_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    let function = match plan_expr(fun, context)?.into_function() {
        Ok(function) => function,
        Err(other) => {
            return Err(invalid_expression_type(
                InvalidExpressionType::Function,
                &other,
            ));
        }
    };
    let function_type = function.type_().clone();
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::FunctionCallUnsupportedReturnType,
        },
    })?;
    if &return_type != function_type.return_() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
            },
        });
    }
    if arguments.len() != function_type.argument_types().len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArityMismatch,
            },
        });
    }

    let args =
        plan_function_call_args(arguments, function_type.argument_types(), context, capture)?;

    function_call_expr(function, args, return_type)
}

fn plan_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, param) in arguments.into_iter().zip(params) {
        let expression = plan_argument_value(argument.value, capture, context)?;
        let arg = match expression.into_call_arg(&param.local) {
            Ok(arg) => arg,
            Err(other) => return Err(call_arg_type_mismatch(param.local.value_type(), &other)),
        };
        args.push(arg);
    }
    Ok(args)
}

fn plan_function_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[ValueType],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<FunctionCallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, type_) in arguments.into_iter().zip(params) {
        let expression = plan_argument_value(argument.value, capture, context)?;
        let arg = match expression.into_function_call_arg(type_) {
            Ok(arg) => arg,
            Err(other) => return Err(call_arg_type_mismatch(type_.clone(), &other)),
        };
        args.push(arg);
    }
    Ok(args)
}

fn call_arg_type_mismatch(expected: ValueType, actual: &Expr) -> PlanError {
    if matches!(expected, ValueType::Function(_))
        && matches!(actual.value_type(), ValueType::Function(_))
    {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
            },
        }
    } else {
        invalid_expression_type_for_value(expected, actual)
    }
}

fn plan_argument_value(
    argument: TypedExpr,
    capture: Option<&CaptureSubstitution>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if let Some(capture) = capture
        && is_capture_local(&argument, &capture.name)
    {
        return Ok(capture.value.clone());
    }

    plan_expr(argument, context)
}

fn count_capture_arguments(
    arguments: &[GleamCallArg<TypedExpr>],
    capture_name: &EcoString,
) -> usize {
    arguments
        .iter()
        .filter(|argument| is_capture_local(&argument.value, capture_name))
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

fn is_capture_local(expression: &TypedExpr, capture_name: &ecow::EcoString) -> bool {
    matches!(
        expression,
        TypedExpr::Var {
            name,
            constructor,
            ..
        } if name == capture_name
            && matches!(
                constructor.variant,
                ValueConstructorVariant::LocalVariable { .. }
            )
    )
}

fn call_expr(function: RuntimeFunctionId, args: Vec<CallArg>) -> Expr {
    match function {
        RuntimeFunctionId::Int(function) => Expr::int(IntExpr::call(function, args)),
        RuntimeFunctionId::String(function) => Expr::string(StringExpr::call(function, args)),
        RuntimeFunctionId::Bool(function) => Expr::bool(BoolExpr::call(function, args)),
        RuntimeFunctionId::Nil(function) => Expr::nil(NilExpr::call(function, args)),
        RuntimeFunctionId::Function { id, return_type } => {
            function_returning_function_call_expr(id, args, return_type)
        }
    }
}

fn function_returning_function_call_expr(
    function: crate::plan::FunctionFunctionId,
    args: Vec<CallArg>,
    return_type: crate::plan::FunctionType,
) -> Expr {
    match function {
        crate::plan::FunctionFunctionId::Int(function) => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::String(function) => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Bool(function) => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Nil(function) => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Function(function) => Expr::function(
            FunctionExpr::function(FunctionFunctionExpr::call(function, args, return_type)),
        ),
    }
}

fn function_call_expr(
    function: FunctionExpr,
    args: Vec<FunctionCallArg>,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    match return_type {
        ValueType::Int => match function.into_int() {
            Ok(function) => Ok(Expr::int(IntExpr::function_call(function, args))),
            Err(_) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        },
        ValueType::String => match function.into_string() {
            Ok(function) => Ok(Expr::string(StringExpr::function_call(function, args))),
            Err(_) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        },
        ValueType::Bool => match function.into_bool() {
            Ok(function) => Ok(Expr::bool(crate::plan::BoolExpr::function_call(
                function, args,
            ))),
            Err(_) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        },
        ValueType::Nil => match function.into_nil() {
            Ok(function) => Ok(Expr::nil(NilExpr::function_call(function, args))),
            Err(_) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        },
        ValueType::Function(return_type) => match function.into_function() {
            Ok(function) => Ok(function_returning_function_value_call_expr(
                function,
                args,
                *return_type,
            )),
            Err(_) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        },
    }
}

fn function_returning_function_value_call_expr(
    function: FunctionFunctionExpr,
    args: Vec<FunctionCallArg>,
    return_type: crate::plan::FunctionType,
) -> Expr {
    match return_type.return_() {
        ValueType::Int => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::function_call(function, args, return_type),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{typed_int_expr, typed_string_expr};
    use crate::plan::{
        BoolFunctionFunctionId, BoolFunctionId, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionType, IntFunctionFunctionId,
        IntLocalId, LocalId, NilFunctionFunctionId, NilFunctionId, ParamLocal, RuntimeFunctionId,
        StringFunctionFunctionId, StringFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        block_int_function, bool_, bool_case_int_function, call_int, call_int_function, function,
        function_function_ref, function_ref, int, int_arg, int_case_int_function, int_function_arg,
        int_function_call_arg, int_function_ref, let_int_function_step, local_int,
        local_int_function, module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::{
        CallArg, ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule, TypedStatement,
    };
    use gleam_core::type_::{
        self, ValueConstructor, ValueConstructorVariant, error::VariableOrigin,
    };

    #[test]
    fn reject_profile_anonymous_function_call() {
        assert_eq!(
            plan_module(compile(r#"pub fn main() { fn(x) { x }(1) }"#)),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::AnonymousFunction,
            }),
        );
    }

    #[test]
    fn reject_profile_function_call_argument_expression() {
        assert_eq!(
            plan_module(compile(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(todo)
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Todo,
            }),
        );
    }

    #[test]
    fn plan_function_value_assignment_before_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(1))],
                ),
            )
            .step(let_int_function_step(
                0,
                "function",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_value_argument_direct_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply(add_one, 41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int(
                    2,
                    [
                        int_function_arg(0, int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                        int_arg(0, int(41)),
                    ],
                ),
            ),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_local_function_value_argument_direct_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  let add = add_one
  apply(add, 41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int(
                    2,
                    [
                        int_function_arg(
                            0,
                            local_int_function(0, "add", [LocalId::Int(IntLocalId(0))]),
                        ),
                        int_arg(0, int(41)),
                    ],
                ),
            )
            .step(let_int_function_step(
                0,
                "add",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_value_and_primitive_shadowing_bindings() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = 1
  let function = add_one
  function(1)
}

pub fn primitive_shadow() {
  let function = add_one
  let function = 1
  function + 1
}
"#,
        ))
        .expect("source should plan");
        let add_one =
            function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(1))],
                ),
            )
            .let_int(0, "function", int(1))
            .step(let_int_function_step(
                0,
                "function",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [
                add_one,
                function("primitive_shadow", local_int(0, "function").add_int(int(1)))
                    .step(let_int_function_step(
                        0,
                        "function",
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ))
                    .let_int(0, "function", int(1)),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_block_callee() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  { add_one }(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    block_int_function([], int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                    [int_function_call_arg(int(1))],
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_case_callee() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_ten(value: Int) {
  value + 10
}

pub fn main() {
  let bool_result = case True {
    True -> add_one
    False -> add_ten
  }(1)
  let int_result = case 0 {
    0 -> add_ten
    _ -> add_one
  }(1)
  bool_result + int_result
}
"#,
        ))
        .expect("source should plan");
        let add_one =
            function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value");
        let add_ten =
            function("add_ten", local_int(0, "value").add_int(int(10))).param_int(0, "value");
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "bool_result").add_int(local_int(1, "int_result")),
            )
            .let_int(
                0,
                "bool_result",
                call_int_function(
                    bool_case_int_function(
                        bool_(true),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                        int_function_ref(2, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(int(1))],
                ),
            )
            .let_int(
                1,
                "int_result",
                call_int_function(
                    int_case_int_function(
                        int(0),
                        [(0, int_function_ref(2, [LocalId::Int(IntLocalId(0))]))],
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(int(1))],
                ),
            ),
            [add_one, add_ten],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_function_value_call_shapes() {
        let mut arity_mismatch_case_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case True {
    True -> add_one
    False -> add_one
  }(1)
}
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut arity_mismatch_case_call.definitions.functions[1].body[0],
        );
        let mut extra_argument = arguments[0].clone();
        extra_argument.value = typed_int_expr(2);
        arguments.push(extra_argument);
        assert_eq!(arguments.len(), 2);
        assert_eq!(
            plan_module(arity_mismatch_case_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );

        let mut non_function_callee = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, fun, _) =
            expect_call_statement_mut(&mut non_function_callee.definitions.functions[1].body[1]);
        *fun = typed_int_expr(1);
        assert_eq!(
            plan_module(non_function_callee),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        let mut unsupported_return_type_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut unsupported_return_type_call.definitions.functions[1].body[1],
        );
        *type_ = type_::list(type_::int());
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallUnsupportedReturnType,
                },
            }),
        );

        let mut return_type_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut return_type_mismatch_call.definitions.functions[1].body[1],
        );
        *type_ = type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        );

        let mut argument_type_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut argument_type_mismatch_call.definitions.functions[1].body[1],
        );
        arguments[0].value = typed_string_expr("wrong");
        assert_eq!(
            plan_module(argument_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );

        let mut arity_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut arity_mismatch_call.definitions.functions[1].body[1]);
        arguments.clear();
        assert_eq!(
            plan_module(arity_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_call_argument_function_shapes() {
        let mut function_mismatch_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn accept_string(function: fn(String) -> String) {
  function("ok")
}

pub fn main() {
  accept_string(string_identity)
  apply(add_one)
}
"#,
        );
        let wrong_function = {
            let (_, _, arguments) = expect_call_statement_mut(
                &mut function_mismatch_call.definitions.functions[4].body[0],
            );
            arguments[0].value.clone()
        };
        let (_, _, arguments) =
            expect_call_statement_mut(&mut function_mismatch_call.definitions.functions[4].body[1]);
        arguments[0].value = wrong_function;
        assert_eq!(
            plan_module(function_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );

        let mut non_function_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  apply(add_one)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut non_function_call.definitions.functions[2].body[0]);
        arguments[0].value = typed_int_expr(1);
        assert_eq!(
            plan_module(non_function_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_value_call_argument_type_shapes() {
        let mut function_mismatch_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn accept_string(function: fn(String) -> String) {
  function("ok")
}

pub fn main() {
  accept_string(string_identity)
  let apply_value = apply
  apply_value(add_one)
}
"#,
        );
        let wrong_function = {
            let (_, _, arguments) = expect_call_statement_mut(
                &mut function_mismatch_call.definitions.functions[4].body[0],
            );
            arguments[0].value.clone()
        };
        let (_, _, arguments) =
            expect_call_statement_mut(&mut function_mismatch_call.definitions.functions[4].body[2]);
        arguments[0].value = wrong_function;
        assert_eq!(
            plan_module(function_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );

        let mut non_function_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let apply_value = apply
  apply_value(add_one)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut non_function_call.definitions.functions[2].body[1]);
        arguments[0].value = typed_int_expr(1);
        assert_eq!(
            plan_module(non_function_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: String) {
  value
}

pub fn main() {
  let function = identity
  function("ok")
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::String,
            InvalidExpressionType::Int,
        );
        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  let function = identity
  function(True)
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::Bool,
            InvalidExpressionType::Int,
        );
        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  let function = identity
  function(Nil)
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::Nil,
            InvalidExpressionType::Int,
        );
    }

    #[test]
    fn function_call_expr_preserves_return_family() {
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::String,
            )
            .expect("string function call")
            .value_type(),
            ValueType::String,
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Bool,
            )
            .expect("bool function call")
            .value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Nil,
            )
            .expect("nil function call")
            .value_type(),
            ValueType::Nil,
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type.clone())),
            )
            .expect("function-returning function call")
            .value_type(),
            ValueType::Function(Box::new(returned_function_type)),
        );
    }

    #[test]
    fn function_returning_function_call_expr_preserves_return_family() {
        let cases = [
            (
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
            (
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
            (
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            ),
            (
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            ),
            (
                FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            ),
        ];

        for (function, returned_function_type) in cases {
            assert_eq!(
                super::function_returning_function_call_expr(
                    function,
                    Vec::new(),
                    returned_function_type.clone(),
                )
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }

    #[test]
    fn function_returning_function_value_call_expr_preserves_return_family() {
        let cases = [
            (
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
            (
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
            (
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            ),
            (
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            ),
            (
                FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            ),
        ];

        for (runtime_id, returned_function_type) in cases {
            let function = FunctionFunctionExpr::from(function_function_ref(
                runtime_id,
                Vec::<ParamLocal>::new(),
                returned_function_type.clone(),
            ));

            assert_eq!(
                super::function_returning_function_value_call_expr(
                    function,
                    Vec::new(),
                    returned_function_type.clone(),
                )
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }

    #[test]
    fn reject_margin_function_call_expr_return_family_mismatch() {
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Int,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::String,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Bool,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Nil,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
            Err(function_call_return_type_mismatch()),
        );
    }

    fn function_call_return_type_mismatch() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
            },
        }
    }

    fn reject_margin_module_constant_call(mut module_constant_call: TypedModule) {
        module_constant_call.definitions.constants.clear();
        let statement = module_constant_call.definitions.functions[0].body.remove(0);
        let module_constant = match statement {
            Statement::Expression(module_constant) => module_constant,
            _ => panic!("expected expression statement"),
        };
        module_constant_call.definitions.functions[0].body =
            vec![Statement::Expression(TypedExpr::Call {
                location: dummy_span(),
                type_: type_::int(),
                fun: Box::new(module_constant),
                arguments: Vec::new(),
                open_parenthesis: Some(0),
            })];
        assert_eq!(
            plan_module(module_constant_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ModuleConstant,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_module_constant_call_shape() {
        reject_margin_module_constant_call(compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        ));
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn reject_margin_module_constant_call_panics_on_assignment_statement() {
        reject_margin_module_constant_call(compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        ));
    }

    #[test]
    fn reject_margin_call_shapes() {
        let mut labelled_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut labelled_call.definitions.functions[1].body[0]);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );

        let mut implicit_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut implicit_call.definitions.functions[1].body[0]);
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Pipe);
        assert_eq!(
            plan_module(implicit_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ImplicitArguments,
                },
            }),
        );

        let mut local_variable_callee = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, fun, _) =
            expect_call_statement_mut(&mut local_variable_callee.definitions.functions[1].body[0]);
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(local_variable_callee),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "identity".into(),
                },
            }),
        );

        let mut arity_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut arity_mismatch_call.definitions.functions[1].body[0]);
        arguments.clear();
        assert_eq!(
            plan_module(arity_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
                },
            }),
        );

        let mut unsupported_return_type_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut unsupported_return_type_call.definitions.functions[1].body[0],
        );
        *type_ = type_::tuple(vec![type_::int()]);
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
                },
            }),
        );

        let mut return_type_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut return_type_mismatch_call.definitions.functions[1].body[0],
        );
        *type_ = type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
                },
            }),
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
            1,
            typed_string_expr("wrong"),
            InvalidExpressionType::Int,
            InvalidExpressionType::String,
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity("ok")
}
"#,
            1,
            typed_int_expr(1),
            InvalidExpressionType::String,
            InvalidExpressionType::Int,
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity(True)
}
"#,
            1,
            typed_int_expr(1),
            InvalidExpressionType::Bool,
            InvalidExpressionType::Int,
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity(Nil)
}
"#,
            1,
            typed_int_expr(1),
            InvalidExpressionType::Nil,
            InvalidExpressionType::Int,
        );

        let mut missing_current_module_fn = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        missing_current_module_fn.definitions.functions.remove(0);
        assert_eq!(
            plan_module(missing_current_module_fn),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                },
            }),
        );

        let non_local_module_fn = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(non_local_module_fn);

        let mut record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
  1
}
"#,
        );
        record_constructor_call.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructor,
                },
            }),
        );
    }

    fn assert_call_argument_type_mismatch(
        src: &str,
        function_index: usize,
        value: TypedExpr,
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    ) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[function_index].body[0]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType { expected, actual },
            }),
        );
    }

    fn assert_function_value_argument_type_mismatch(
        src: &str,
        value: TypedExpr,
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    ) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[1].body[1]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType { expected, actual },
            }),
        );
    }

    fn reject_margin_non_local_module_fn_call(mut non_local_module_fn: TypedModule) {
        let function = non_local_module_fn
            .definitions
            .functions
            .last_mut()
            .expect("expected test module to have a function");
        let (_, fun, _) = expect_call_statement_mut(&mut function.body[0]);
        let constructor = expect_var_constructor_mut(fun);
        let module = match &mut constructor.variant {
            ValueConstructorVariant::ModuleFn { module, .. } => module,
            _ => panic!("expected module function constructor"),
        };
        *module = "other".into();
        assert_eq!(
            plan_module(non_local_module_fn),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::NonCurrentModuleFunction,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn reject_margin_non_local_module_fn_call_panics_on_record_constructor() {
        let record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(record_constructor_call);
    }

    fn expect_call_statement_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        match statement {
            Statement::Expression(expression) => expect_call_expression_mut(expression),
            _ => panic!("expected call expression statement"),
        }
    }

    fn expect_call_expression_mut(
        expression: &mut TypedExpr,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        match expression {
            TypedExpr::Call {
                type_,
                fun,
                arguments,
                ..
            } => (type_, fun.as_mut(), arguments),
            _ => panic!("expected call expression statement"),
        }
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_expression() {
        let mut module = compile_minimal_module();

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_assignment() {
        let mut module = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    fn expect_var_constructor_mut(expression: &mut TypedExpr) -> &mut ValueConstructor {
        match expression {
            TypedExpr::Var { constructor, .. } => constructor,
            _ => panic!("expected variable expression"),
        }
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_var_constructor_mut_panics_on_int() {
        let mut expression = typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }
}
