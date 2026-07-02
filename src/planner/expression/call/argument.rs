use super::{CallArgumentMode, CaptureSubstitution};
use crate::plan::{CallArg, Expr, ParamLocal, ValueType};
use crate::planner::context::{FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidTypedAstReason, InvalidUseShapeReason, PlanError,
};
use gleam_core::ast::{
    CallArg as GleamCallArg, FunctionLiteralKind, ImplicitCallArgOrigin, TypedExpr,
};

pub(super) fn plan_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
    call_argument_mode: CallArgumentMode,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, param) in arguments.into_iter().zip(params) {
        let expression = plan_argument_value(argument, capture, context, call_argument_mode)?;
        let actual = expression.value_type();
        let arg = match expression.into_call_arg(&param.local) {
            Some(arg) => arg,
            None => return Err(call_arg_type_mismatch(param.local.value_type(), actual)),
        };
        args.push(arg);
    }
    Ok(args)
}

pub(super) fn plan_function_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[ValueType],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
    call_argument_mode: CallArgumentMode,
) -> Result<Vec<CallArg>, PlanError> {
    let locals = function_call_param_locals(params);
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, local) in arguments.into_iter().zip(&locals) {
        let expression = plan_argument_value(argument, capture, context, call_argument_mode)?;
        let actual = expression.value_type();
        let arg = match expression.into_call_arg(local) {
            Some(arg) => arg,
            None => return Err(call_arg_type_mismatch(local.value_type(), actual)),
        };
        args.push(arg);
    }
    Ok(args)
}

fn function_call_param_locals(params: &[ValueType]) -> Vec<ParamLocal> {
    let mut next_int = 0;
    let mut next_string = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_int_function = 0;
    let mut next_string_function = 0;
    let mut next_bool_function = 0;
    let mut next_nil_function = 0;
    let mut next_function_function = 0;

    params
        .iter()
        .map(|type_| match type_ {
            ValueType::Int => {
                let local = ParamLocal::int(crate::plan::IntLocalId(next_int));
                next_int += 1;
                local
            }
            ValueType::String => {
                let local = ParamLocal::string(crate::plan::StringLocalId(next_string));
                next_string += 1;
                local
            }
            ValueType::Bool => {
                let local = ParamLocal::bool(crate::plan::BoolLocalId(next_bool));
                next_bool += 1;
                local
            }
            ValueType::Nil => {
                let local = ParamLocal::nil(crate::plan::NilLocalId(next_nil));
                next_nil += 1;
                local
            }
            ValueType::Function(type_) => match type_.return_() {
                ValueType::Int => {
                    let local = ParamLocal::int_function(
                        crate::plan::IntFunctionLocalId(next_int_function),
                        type_.as_ref().clone(),
                    );
                    next_int_function += 1;
                    local
                }
                ValueType::String => {
                    let local = ParamLocal::string_function(
                        crate::plan::StringFunctionLocalId(next_string_function),
                        type_.as_ref().clone(),
                    );
                    next_string_function += 1;
                    local
                }
                ValueType::Bool => {
                    let local = ParamLocal::bool_function(
                        crate::plan::BoolFunctionLocalId(next_bool_function),
                        type_.as_ref().clone(),
                    );
                    next_bool_function += 1;
                    local
                }
                ValueType::Nil => {
                    let local = ParamLocal::nil_function(
                        crate::plan::NilFunctionLocalId(next_nil_function),
                        type_.as_ref().clone(),
                    );
                    next_nil_function += 1;
                    local
                }
                ValueType::Function(_) => {
                    let local = ParamLocal::function_function(
                        crate::plan::FunctionFunctionLocalId(next_function_function),
                        type_.as_ref().clone(),
                    );
                    next_function_function += 1;
                    local
                }
            },
        })
        .collect()
}

fn call_arg_type_mismatch(expected: ValueType, actual: ValueType) -> PlanError {
    if matches!(expected, ValueType::Function(_)) && matches!(actual, ValueType::Function(_)) {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
            },
        }
    } else {
        super::super::invalid_expression_type_for_value(expected, actual)
    }
}

fn plan_argument_value(
    argument: GleamCallArg<TypedExpr>,
    capture: Option<&CaptureSubstitution>,
    context: &mut PlanContext<'_>,
    call_argument_mode: CallArgumentMode,
) -> Result<Expr, PlanError> {
    if matches!(call_argument_mode, CallArgumentMode::Use)
        && matches!(argument.implicit, Some(ImplicitCallArgOrigin::Use))
    {
        return plan_use_callback_argument(argument.value, context);
    }

    if let Some(capture) = capture
        && super::is_capture_local(&argument.value, &capture.name)
    {
        return Ok(capture.value.clone());
    }

    super::super::plan_expr(argument.value, context)
}

fn plan_use_callback_argument(
    argument: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match argument {
        TypedExpr::Fn {
            type_,
            kind: FunctionLiteralKind::Use { .. },
            arguments,
            body,
            ..
        } => super::super::function::plan_use_callback(type_, arguments, body, context),
        TypedExpr::Fn { .. } => Err(super::invalid_use_shape(
            InvalidUseShapeReason::CallbackLiteralKindNotUse,
        )),
        _ => Err(super::invalid_use_shape(
            InvalidUseShapeReason::CallbackNotFunctionLiteral,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::function_call_param_locals;
    use crate::plan::{FunctionType, ParamLocal, ValueType};

    #[test]
    fn function_call_param_locals_preserve_family_local_order() {
        assert_eq!(
            function_call_param_locals(&[
                ValueType::Int,
                ValueType::String,
                ValueType::Bool,
                ValueType::Nil,
                ValueType::Int,
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::String],
                    ValueType::String,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Bool],
                    ValueType::Bool,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Nil],
                    ValueType::Nil,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int,))),
                ))),
            ]),
            vec![
                ParamLocal::int(crate::plan::IntLocalId(0)),
                ParamLocal::string(crate::plan::StringLocalId(0)),
                ParamLocal::bool(crate::plan::BoolLocalId(0)),
                ParamLocal::nil(crate::plan::NilLocalId(0)),
                ParamLocal::int(crate::plan::IntLocalId(1)),
                ParamLocal::int_function(
                    crate::plan::IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
                ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                ),
                ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                ),
                ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                ),
                ParamLocal::function_function(
                    crate::plan::FunctionFunctionLocalId(0),
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            ValueType::Int,
                        ))),
                    ),
                ),
            ],
        );
        assert_eq!(
            function_call_param_locals(&[ValueType::Int])[0],
            ParamLocal::int(crate::plan::IntLocalId(0)),
        );
    }
}
