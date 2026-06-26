use super::{invalid_expression_type, plan_expr};
use crate::plan::{
    BoolCaseBranches, BoolExpr, CallArg, Expr, ExprKind, FunctionArgumentType, FunctionExpr,
    FunctionExprKind, IntCaseBranches, IntExpr, LocalId, NilExpr, RuntimeFunctionId, StringExpr,
    ValueType,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidPipelineShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedPipelineReason,
};
use ecow::EcoString;
use gleam_core::ast::{
    CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, Statement, TypedExpr,
};
use gleam_core::type_::{Type, ValueConstructorVariant};
use std::sync::Arc;

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

    let TypedExpr::Fn {
        kind: FunctionLiteralKind::Capture { .. },
        arguments: capture_args,
        body,
        ..
    } = fun
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
    };
    let [capture_arg] = capture_args.as_slice() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
    };
    let Some(capture_name) = capture_arg.names.get_variable_name().cloned() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::InvalidHoleCapture,
            },
        });
    };

    let mut body = body.into_iter();
    let Some(Statement::Expression(TypedExpr::Call { fun, arguments, .. })) = body.next() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PipelineShape {
                reason: InvalidPipelineShapeReason::NonCallStep,
            },
        });
    };
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

    let params = function_call_params(&function_type);
    plan_function_expr_call(function, arguments, &params, context, capture)
}

fn plan_function_expr_call(
    function: FunctionExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    match function.kind().clone() {
        FunctionExprKind::Value(function) => {
            let args = plan_call_args(arguments, params, context, capture)?;
            Ok(call_expr(function.runtime_id(), args))
        }
        FunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            let true_ =
                plan_function_expr_call(*true_, arguments.clone(), params, context, capture)?;
            let false_ = plan_function_expr_call(*false_, arguments, params, context, capture)?;
            bool_case_call_expr(*subject, true_, false_)
        }
        FunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let mut call_clauses = Vec::with_capacity(clauses.len());
            for (pattern, branch) in clauses {
                call_clauses.push((
                    pattern,
                    plan_function_expr_call(branch, arguments.clone(), params, context, capture)?,
                ));
            }
            let fallback = plan_function_expr_call(*fallback, arguments, params, context, capture)?;
            int_case_call_expr(*subject, call_clauses, fallback)
        }
        FunctionExprKind::Block { steps, return_ } => {
            let return_ = plan_function_expr_call(*return_, arguments, params, context, capture)?;
            block_call_expr(steps, return_)
        }
    }
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
        let arg = match expression.into_call_arg(param.local) {
            Ok(arg) => arg,
            Err(other) => {
                let expected = match param.local {
                    LocalId::Int(_) => InvalidExpressionType::Int,
                    LocalId::String(_) => InvalidExpressionType::String,
                    LocalId::Bool(_) => InvalidExpressionType::Bool,
                    LocalId::Nil(_) => InvalidExpressionType::Nil,
                };
                return Err(invalid_expression_type(expected, &other));
            }
        };
        args.push(arg);
    }
    Ok(args)
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
    }
}

fn function_call_params(function_type: &crate::plan::FunctionType) -> Vec<FunctionParam> {
    let mut next_int = 0;
    let mut next_string = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;

    function_type
        .argument_types()
        .iter()
        .map(|type_| {
            let local = match type_ {
                FunctionArgumentType::Int => {
                    let local = LocalId::Int(crate::plan::IntLocalId(next_int));
                    next_int += 1;
                    local
                }
                FunctionArgumentType::String => {
                    let local = LocalId::String(crate::plan::StringLocalId(next_string));
                    next_string += 1;
                    local
                }
                FunctionArgumentType::Bool => {
                    let local = LocalId::Bool(crate::plan::BoolLocalId(next_bool));
                    next_bool += 1;
                    local
                }
                FunctionArgumentType::Nil => {
                    let local = LocalId::Nil(crate::plan::NilLocalId(next_nil));
                    next_nil += 1;
                    local
                }
            };

            FunctionParam {
                local,
                name: EcoString::default(),
            }
        })
        .collect()
}

fn bool_case_call_expr(subject: BoolExpr, true_: Expr, false_: Expr) -> Result<Expr, PlanError> {
    let branches = match (true_.into_kind(), false_.into_kind()) {
        (ExprKind::Int(true_), ExprKind::Int(false_)) => BoolCaseBranches::Int { true_, false_ },
        (ExprKind::String(true_), ExprKind::String(false_)) => {
            BoolCaseBranches::String { true_, false_ }
        }
        (ExprKind::Bool(true_), ExprKind::Bool(false_)) => BoolCaseBranches::Bool { true_, false_ },
        (ExprKind::Nil(true_), ExprKind::Nil(false_)) => BoolCaseBranches::Nil { true_, false_ },
        _ => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        }
    };

    Ok(Expr::bool_case(subject, branches))
}

fn int_case_call_expr(
    subject: IntExpr,
    clauses: Vec<(num_bigint::BigInt, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    let branches = match fallback.into_kind() {
        ExprKind::Int(fallback) => IntCaseBranches::Int {
            clauses: int_call_clauses(clauses)?,
            fallback,
        },
        ExprKind::String(fallback) => IntCaseBranches::String {
            clauses: string_call_clauses(clauses)?,
            fallback,
        },
        ExprKind::Bool(fallback) => IntCaseBranches::Bool {
            clauses: bool_call_clauses(clauses)?,
            fallback,
        },
        ExprKind::Nil(fallback) => IntCaseBranches::Nil {
            clauses: nil_call_clauses(clauses)?,
            fallback,
        },
        ExprKind::Function(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        }
    };

    Ok(Expr::int_case(subject, branches))
}

fn int_call_clauses(
    clauses: Vec<(num_bigint::BigInt, Expr)>,
) -> Result<Vec<(num_bigint::BigInt, IntExpr)>, PlanError> {
    let mut typed = Vec::with_capacity(clauses.len());
    for (pattern, clause) in clauses {
        let ExprKind::Int(clause) = clause.into_kind() else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        };
        typed.push((pattern, clause));
    }
    Ok(typed)
}

fn string_call_clauses(
    clauses: Vec<(num_bigint::BigInt, Expr)>,
) -> Result<Vec<(num_bigint::BigInt, StringExpr)>, PlanError> {
    let mut typed = Vec::with_capacity(clauses.len());
    for (pattern, clause) in clauses {
        let ExprKind::String(clause) = clause.into_kind() else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        };
        typed.push((pattern, clause));
    }
    Ok(typed)
}

fn bool_call_clauses(
    clauses: Vec<(num_bigint::BigInt, Expr)>,
) -> Result<Vec<(num_bigint::BigInt, BoolExpr)>, PlanError> {
    let mut typed = Vec::with_capacity(clauses.len());
    for (pattern, clause) in clauses {
        let ExprKind::Bool(clause) = clause.into_kind() else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        };
        typed.push((pattern, clause));
    }
    Ok(typed)
}

fn nil_call_clauses(
    clauses: Vec<(num_bigint::BigInt, Expr)>,
) -> Result<Vec<(num_bigint::BigInt, NilExpr)>, PlanError> {
    let mut typed = Vec::with_capacity(clauses.len());
    for (pattern, clause) in clauses {
        let ExprKind::Nil(clause) = clause.into_kind() else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        };
        typed.push((pattern, clause));
    }
    Ok(typed)
}

fn block_call_expr(steps: Vec<crate::plan::Step>, return_: Expr) -> Result<Expr, PlanError> {
    Ok(match return_.into_kind() {
        ExprKind::Int(return_) => Expr::int(IntExpr::block(steps, return_)),
        ExprKind::String(return_) => Expr::string(StringExpr::block(steps, return_)),
        ExprKind::Bool(return_) => Expr::bool(BoolExpr::block(steps, return_)),
        ExprKind::Nil(return_) => Expr::nil(NilExpr::block(steps, return_)),
        ExprKind::Function(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            });
        }
    })
}

#[cfg(test)]
mod tests {
    use super::super::{typed_int_expr, typed_string_expr};
    use crate::plan::{
        BoolLocalId, IntFunctionId, IntLocalId, LocalId, NilLocalId, RuntimeFunctionId,
        StringFunctionId, StringLocalId,
    };
    use crate::plan::{FunctionArgumentType, FunctionExpr, FunctionType, ValueType};
    use crate::planner::dsl::{
        block_bool, block_int, block_nil, block_string, bool_, bool_case_bool, bool_case_int,
        bool_case_nil, bool_case_string, call_int, function, function_ref, int, int_arg,
        int_case_bool, int_case_int, int_case_nil, int_case_string, local_int, module, nil, string,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
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
            function("main", call_int(1, [int_arg(0, int(1))])),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
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
            function("main", call_int(1, [int_arg(0, int(1))])).let_int(0, "function", int(1)),
            [
                add_one,
                function("primitive_shadow", local_int(0, "function").add_int(int(1))).let_int(
                    0,
                    "function",
                    int(1),
                ),
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
            function("main", block_int([], call_int(1, [int_arg(0, int(1))]))),
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
                bool_case_int(
                    bool_(true),
                    call_int(1, [int_arg(0, int(1))]),
                    call_int(2, [int_arg(0, int(1))]),
                ),
            )
            .let_int(
                1,
                "int_result",
                int_case_int(
                    int(0),
                    [(0, call_int(2, [int_arg(0, int(1))]))],
                    call_int(1, [int_arg(0, int(1))]),
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
    fn reject_margin_function_value_call_lowering_shape_mismatch() {
        assert_eq!(
            super::bool_case_call_expr(bool_(true).into(), int(1).into(), string("wrong").into()),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_call_expr(
                int(1).into(),
                vec![(1.into(), string("wrong").into())],
                int(0).into()
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_call_expr(
                int(1).into(),
                vec![(1.into(), int(1).into())],
                string("fallback").into()
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_call_expr(
                int(1).into(),
                vec![(1.into(), int(1).into())],
                bool_(false).into()
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_call_expr(int(1).into(), vec![(1.into(), int(1).into())], nil().into()),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_call_expr(int(1).into(), Vec::new(), function_expr()),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            super::block_call_expr(Vec::new(), function_expr()),
            Err(function_call_return_type_mismatch()),
        );
    }

    #[test]
    fn plan_function_value_call_lowering_return_shapes() {
        assert_eq!(
            super::bool_case_call_expr(
                bool_(true).into(),
                string("yes").into(),
                string("no").into(),
            ),
            Ok(bool_case_string(bool_(true), string("yes"), string("no")).into()),
        );
        assert_eq!(
            super::bool_case_call_expr(bool_(true).into(), bool_(true).into(), bool_(false).into()),
            Ok(bool_case_bool(bool_(true), bool_(true), bool_(false)).into()),
        );
        assert_eq!(
            super::bool_case_call_expr(bool_(true).into(), nil().into(), nil().into()),
            Ok(bool_case_nil(bool_(true), nil(), nil()).into()),
        );
        assert_eq!(
            super::int_case_call_expr(
                int(1).into(),
                vec![(1.into(), string("one").into())],
                string("other").into(),
            ),
            Ok(int_case_string(int(1), [(1, string("one"))], string("other")).into()),
        );
        assert_eq!(
            super::int_case_call_expr(
                int(1).into(),
                vec![(1.into(), bool_(true).into())],
                bool_(false).into(),
            ),
            Ok(int_case_bool(int(1), [(1, bool_(true))], bool_(false)).into()),
        );
        assert_eq!(
            super::int_case_call_expr(int(1).into(), vec![(1.into(), nil().into())], nil().into()),
            Ok(int_case_nil(int(1), [(1, nil())], nil()).into()),
        );
        assert_eq!(
            super::block_call_expr(Vec::new(), string("value").into()),
            Ok(block_string([], string("value")).into()),
        );
        assert_eq!(
            super::block_call_expr(Vec::new(), bool_(true).into()),
            Ok(block_bool([], bool_(true)).into()),
        );
        assert_eq!(
            super::block_call_expr(Vec::new(), nil().into()),
            Ok(block_nil([], nil()).into()),
        );
    }

    #[test]
    fn function_call_params_supports_primitive_argument_shapes() {
        let params = super::function_call_params(&FunctionType::new(
            vec![
                FunctionArgumentType::String,
                FunctionArgumentType::Bool,
                FunctionArgumentType::Nil,
            ],
            ValueType::Int,
        ));

        assert!(matches!(params[0].local, LocalId::String(StringLocalId(0)),));
        assert!(matches!(params[1].local, LocalId::Bool(BoolLocalId(0))));
        assert!(matches!(params[2].local, LocalId::Nil(NilLocalId(0))));
    }

    #[test]
    fn reject_margin_function_value_call_recursive_lowering_errors() {
        let module = EcoString::from("main");
        let functions = std::collections::HashMap::<EcoString, super::FunctionInfo>::new();
        let mut context = super::PlanContext::new(&module, &functions);
        let params = [super::FunctionParam {
            local: LocalId::Int(IntLocalId(0)),
            name: EcoString::default(),
        }];
        let string_argument = call_arguments(typed_string_expr("wrong"));
        let int_argument = call_arguments(typed_int_expr(1));
        let expected_type_error = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Int,
                actual: InvalidExpressionType::String,
            },
        });
        let expected_shape_error = Err(function_call_return_type_mismatch());

        assert_eq!(
            super::plan_function_expr_call(
                FunctionExpr::bool_case(
                    bool_(true).into(),
                    int_function_expr(),
                    int_function_expr()
                ),
                string_argument.clone(),
                &params,
                &mut context,
                None,
            ),
            expected_type_error,
        );
        assert_eq!(
            super::plan_function_expr_call(
                FunctionExpr::bool_case(
                    bool_(true).into(),
                    int_function_expr(),
                    mismatched_function_case(),
                ),
                int_argument.clone(),
                &params,
                &mut context,
                None,
            ),
            expected_shape_error.clone(),
        );
        assert_eq!(
            super::plan_function_expr_call(
                FunctionExpr::int_case(
                    int(1).into(),
                    vec![(1.into(), mismatched_function_case())],
                    int_function_expr(),
                ),
                int_argument.clone(),
                &params,
                &mut context,
                None,
            ),
            expected_shape_error.clone(),
        );
        assert_eq!(
            super::plan_function_expr_call(
                FunctionExpr::int_case(
                    int(1).into(),
                    vec![(1.into(), int_function_expr())],
                    mismatched_function_case(),
                ),
                int_argument.clone(),
                &params,
                &mut context,
                None,
            ),
            expected_shape_error.clone(),
        );
        assert_eq!(
            super::plan_function_expr_call(
                FunctionExpr::block(Vec::new(), mismatched_function_case()),
                int_argument,
                &params,
                &mut context,
                None,
            ),
            expected_shape_error,
        );
    }

    fn function_expr() -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn int_function_expr() -> FunctionExpr {
        function_ref(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn mismatched_function_case() -> FunctionExpr {
        FunctionExpr::bool_case(
            bool_(true).into(),
            int_function_expr(),
            function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                [LocalId::Int(IntLocalId(0))],
            )
            .into(),
        )
    }

    fn call_arguments(value: TypedExpr) -> Vec<CallArg<TypedExpr>> {
        vec![CallArg {
            label: None,
            location: dummy_span(),
            value,
            implicit: None,
        }]
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
        let Statement::Expression(module_constant) = statement else {
            panic!("expected expression statement");
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

    fn reject_margin_non_local_module_fn_call(mut non_local_module_fn: TypedModule) {
        let function = non_local_module_fn
            .definitions
            .functions
            .last_mut()
            .expect("expected test module to have a function");
        let (_, fun, _) = expect_call_statement_mut(&mut function.body[0]);
        let constructor = expect_var_constructor_mut(fun);
        let ValueConstructorVariant::ModuleFn { module, .. } = &mut constructor.variant else {
            panic!("expected module function constructor");
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
        let Statement::Expression(TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        }) = statement
        else {
            panic!("expected call expression statement");
        };
        (type_, fun.as_mut(), arguments)
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_expression() {
        let mut module = compile_minimal_module();

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
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
        let mut expression = typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }
}
