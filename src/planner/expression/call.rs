use super::{invalid_expression_type, plan_expr};
use crate::plan::{
    BoolExpr, CallArg, Expr, IntExpr, LocalId, NilExpr, RuntimeFunctionId, StringExpr, ValueType,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidPipelineShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedCallReason, UnsupportedPipelineReason,
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

    plan_local_call(
        type_,
        fun,
        arguments,
        context,
        FunctionRefMode::Normal,
        None,
    )
}

pub(super) fn plan_pipeline_direct_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    pipe_argument(&arguments)?;

    plan_local_call(
        type_,
        fun,
        arguments,
        context,
        FunctionRefMode::Pipeline,
        None,
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
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::InvalidHoleCapture,
        ));
    };
    let [capture_arg] = capture_args.as_slice() else {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::InvalidHoleCapture,
        ));
    };
    let Some(capture_name) = capture_arg.names.get_variable_name().cloned() else {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::InvalidHoleCapture,
        ));
    };

    let mut body = body.into_iter();
    let Some(Statement::Expression(TypedExpr::Call { fun, arguments, .. })) = body.next() else {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::NonCallStep,
        ));
    };
    if body.next().is_some() {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::InvalidHoleCapture,
        ));
    }
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::LabelledArguments,
        ));
    }
    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::UnsupportedImplicitArgument,
        ));
    }
    if count_capture_arguments(&arguments, &capture_name) != 1 {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::InvalidHoleCapture,
        ));
    }

    plan_local_call(
        type_,
        *fun,
        arguments,
        context,
        FunctionRefMode::Pipeline,
        Some(&CaptureSubstitution {
            name: capture_name,
            value: pipe_value,
        }),
    )
}

#[derive(Clone, Copy)]
enum FunctionRefMode {
    Normal,
    Pipeline,
}

struct CaptureSubstitution {
    name: EcoString,
    value: Expr,
}

fn plan_local_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    function_ref_mode: FunctionRefMode,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    let function = match function_ref_mode {
        FunctionRefMode::Normal => plan_function_ref(fun, context),
        FunctionRefMode::Pipeline => plan_pipeline_function_ref(fun, context),
    }?;
    if function.arity != arguments.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
            },
        });
    }
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
        },
    })?;
    let (Some(function_return_type), Some(function_id)) =
        (function.return_type, function.runtime_id)
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
            },
        });
    };
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

fn plan_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    arguments
        .into_iter()
        .zip(params)
        .map(|(argument, param)| {
            let expression = plan_argument_value(argument.value, capture, context)?;
            expression.into_call_arg(param.local).map_err(|other| {
                invalid_expression_type(expected_expression_type(param.local), &other)
            })
        })
        .collect()
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

fn plan_function_ref(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<FunctionInfo, PlanError> {
    let TypedExpr::Var { constructor, .. } = expression else {
        return Err(PlanError::UnsupportedCall {
            reason: UnsupportedCallReason::NonDirectLocalFunction,
        });
    };

    match constructor.variant {
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } if module == *context.module_name
            && external_erlang.is_none()
            && external_javascript.is_none() =>
        {
            context
                .lookup_function(&name)
                .ok_or(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                    },
                })
        }
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::NonCurrentModuleFunction,
            },
        }),
        ValueConstructorVariant::LocalVariable { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionValue,
            },
        }),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ModuleConstant,
            },
        }),
        ValueConstructorVariant::Record { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::RecordConstructor,
            },
        }),
    }
}

fn plan_pipeline_function_ref(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<FunctionInfo, PlanError> {
    let TypedExpr::Var { constructor, .. } = expression else {
        return if matches!(expression, TypedExpr::Call { .. }) {
            Err(PlanError::UnsupportedPipeline {
                reason: UnsupportedPipelineReason::FunctionCall,
            })
        } else {
            Err(PlanError::UnsupportedPipeline {
                reason: UnsupportedPipelineReason::NonDirectLocalFunction,
            })
        };
    };

    match constructor.variant {
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } if module == *context.module_name
            && external_erlang.is_none()
            && external_javascript.is_none() =>
        {
            context
                .lookup_function(&name)
                .ok_or(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                    },
                })
        }
        ValueConstructorVariant::ModuleFn { .. }
        | ValueConstructorVariant::LocalVariable { .. }
        | ValueConstructorVariant::ModuleConstant { .. }
        | ValueConstructorVariant::Record { .. } => Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::NonDirectLocalFunction,
        )),
    }
}

fn pipe_argument(arguments: &[GleamCallArg<TypedExpr>]) -> Result<&TypedExpr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(invalid_pipeline_shape(
            InvalidPipelineShapeReason::LabelledArguments,
        ));
    }

    let mut pipe_argument = None;
    for argument in arguments {
        match argument.implicit {
            None => {}
            Some(ImplicitCallArgOrigin::Pipe) => {
                if pipe_argument.replace(&argument.value).is_some() {
                    return Err(invalid_pipeline_shape(
                        InvalidPipelineShapeReason::MultiplePipeArguments,
                    ));
                }
            }
            Some(
                ImplicitCallArgOrigin::Use
                | ImplicitCallArgOrigin::PatternFieldSpread
                | ImplicitCallArgOrigin::IncorrectArityUse
                | ImplicitCallArgOrigin::RecordUpdate,
            ) => {
                return Err(invalid_pipeline_shape(
                    InvalidPipelineShapeReason::UnsupportedImplicitArgument,
                ));
            }
        }
    }

    pipe_argument
        .ok_or_else(|| invalid_pipeline_shape(InvalidPipelineShapeReason::MissingPipeArgument))
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

fn invalid_pipeline_shape(reason: InvalidPipelineShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PipelineShape { reason },
    }
}

fn call_expr(function: RuntimeFunctionId, args: Vec<CallArg>) -> Expr {
    match function {
        RuntimeFunctionId::Int(function) => Expr::int(IntExpr::call(function, args)),
        RuntimeFunctionId::String(function) => Expr::string(StringExpr::call(function, args)),
        RuntimeFunctionId::Bool(function) => Expr::bool(BoolExpr::call(function, args)),
        RuntimeFunctionId::Nil(function) => Expr::nil(NilExpr::call(function, args)),
    }
}

fn expected_expression_type(local: LocalId) -> InvalidExpressionType {
    match local {
        LocalId::Int(_) => InvalidExpressionType::Int,
        LocalId::String(_) => InvalidExpressionType::String,
        LocalId::Bool(_) => InvalidExpressionType::Bool,
        LocalId::Nil(_) => InvalidExpressionType::Nil,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{typed_int_expr, typed_string_expr};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCallReason,
    };
    use gleam_core::ast::{
        CallArg, ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule, TypedStatement,
    };
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{self, ValueConstructor, ValueConstructorVariant};

    #[test]
    fn reject_profile_non_direct_call() {
        assert_eq!(
            expect_plan_error(r#"pub fn main() { fn(x) { x }(1) }"#),
            PlanError::UnsupportedCall {
                reason: UnsupportedCallReason::NonDirectLocalFunction,
            },
        );
    }

    #[test]
    fn reject_margin_call_to_unsupported_return_function() {
        let mut module = compile(
            r#"
fn helper() {
  1
}

pub fn main() {
  helper()
}
"#,
        );
        module.definitions.functions[0].return_type = type_::list(type_::int());

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
                },
            }),
        );
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

        let mut local_function_value_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, fun, _) = expect_call_statement_mut(
            &mut local_function_value_call.definitions.functions[1].body[0],
        );
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(local_function_value_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionValue,
                },
            }),
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
