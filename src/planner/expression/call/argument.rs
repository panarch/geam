use super::CaptureSubstitution;
use crate::plan::{
    BitArrayListLocalId, BoolListLocalId, CallArg, Expr, FloatListLocalId, FunctionListLocalId,
    IntListLocalId, ListFunctionLocal, ListListLocalId, ListLocal, NilListLocalId, ParamLocal,
    StringListLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointListLocalId, ValueType,
};
use crate::planner::context::{FunctionParam, PlanContext};
use crate::planner::error::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};

pub(super) fn plan_call_args(
    arguments: Vec<GleamCallArg<TypedExpr>>,
    params: &[FunctionParam],
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Vec<CallArg>, PlanError> {
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, param) in arguments.into_iter().zip(params) {
        let expression = plan_argument_value(argument, param.local.value_type(), capture, context)?;
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
) -> Result<Vec<CallArg>, PlanError> {
    let locals = function_call_param_locals(params);
    let mut args = Vec::with_capacity(arguments.len());
    for (argument, local) in arguments.into_iter().zip(&locals) {
        let expression = plan_argument_value(argument, local.value_type(), capture, context)?;
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
    let mut next_bit_array = 0;
    let mut next_utf_codepoint = 0;
    let mut next_float = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_tuple = 0;
    let mut next_int_list = 0;
    let mut next_string_list = 0;
    let mut next_bit_array_list = 0;
    let mut next_utf_codepoint_list = 0;
    let mut next_float_list = 0;
    let mut next_bool_list = 0;
    let mut next_nil_list = 0;
    let mut next_tuple_list = 0;
    let mut next_list_list = 0;
    let mut next_function_list = 0;
    let mut next_int_function = 0;
    let mut next_string_function = 0;
    let mut next_bit_array_function = 0;
    let mut next_utf_codepoint_function = 0;
    let mut next_float_function = 0;
    let mut next_bool_function = 0;
    let mut next_nil_function = 0;
    let mut next_tuple_function = 0;
    let mut next_list_function = 0;
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
            ValueType::BitArray => {
                let local = ParamLocal::bit_array(crate::plan::BitArrayLocalId(next_bit_array));
                next_bit_array += 1;
                local
            }
            ValueType::UtfCodepoint => {
                let local =
                    ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(next_utf_codepoint));
                next_utf_codepoint += 1;
                local
            }
            ValueType::Float => {
                let local = ParamLocal::float(crate::plan::FloatLocalId(next_float));
                next_float += 1;
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
            ValueType::Tuple(type_) => {
                let local = ParamLocal::tuple(TupleLocalId(next_tuple), type_.clone());
                next_tuple += 1;
                local
            }
            ValueType::List(element_type) => {
                let local = match element_type.as_ref() {
                    ValueType::Int => {
                        let local = ListLocal::int(IntListLocalId(next_int_list));
                        next_int_list += 1;
                        local
                    }
                    ValueType::String => {
                        let local = ListLocal::string(StringListLocalId(next_string_list));
                        next_string_list += 1;
                        local
                    }
                    ValueType::BitArray => {
                        let local = ListLocal::bit_array(BitArrayListLocalId(next_bit_array_list));
                        next_bit_array_list += 1;
                        local
                    }
                    ValueType::UtfCodepoint => {
                        let local = ListLocal::utf_codepoint(UtfCodepointListLocalId(
                            next_utf_codepoint_list,
                        ));
                        next_utf_codepoint_list += 1;
                        local
                    }
                    ValueType::Float => {
                        let local = ListLocal::float(FloatListLocalId(next_float_list));
                        next_float_list += 1;
                        local
                    }
                    ValueType::Bool => {
                        let local = ListLocal::bool(BoolListLocalId(next_bool_list));
                        next_bool_list += 1;
                        local
                    }
                    ValueType::Nil => {
                        let local = ListLocal::nil(NilListLocalId(next_nil_list));
                        next_nil_list += 1;
                        local
                    }
                    ValueType::Tuple(item_type) => {
                        let local =
                            ListLocal::tuple(TupleListLocalId(next_tuple_list), item_type.clone());
                        next_tuple_list += 1;
                        local
                    }
                    ValueType::List(item_type) => {
                        let local =
                            ListLocal::list(ListListLocalId(next_list_list), *item_type.clone());
                        next_list_list += 1;
                        local
                    }
                    ValueType::Function(item_type) => {
                        let local = ListLocal::function(
                            FunctionListLocalId(next_function_list),
                            *item_type.clone(),
                        );
                        next_function_list += 1;
                        local
                    }
                };
                ParamLocal::list(local)
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
                ValueType::BitArray => {
                    let local = ParamLocal::bit_array_function(
                        crate::plan::BitArrayFunctionLocalId(next_bit_array_function),
                        type_.as_ref().clone(),
                    );
                    next_bit_array_function += 1;
                    local
                }
                ValueType::UtfCodepoint => {
                    let local = ParamLocal::utf_codepoint_function(
                        crate::plan::UtfCodepointFunctionLocalId(next_utf_codepoint_function),
                        type_.as_ref().clone(),
                    );
                    next_utf_codepoint_function += 1;
                    local
                }
                ValueType::Float => {
                    let local = ParamLocal::float_function(
                        crate::plan::FloatFunctionLocalId(next_float_function),
                        type_.as_ref().clone(),
                    );
                    next_float_function += 1;
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
                ValueType::Tuple(_) => {
                    let local = ParamLocal::tuple_function(
                        TupleFunctionLocalId(next_tuple_function),
                        type_.as_ref().clone(),
                    );
                    next_tuple_function += 1;
                    local
                }
                ValueType::List(item_type) => {
                    let local = ParamLocal::list_function(ListFunctionLocal::from_item_type(
                        next_list_function,
                        type_.as_ref().clone(),
                        item_type.as_ref().clone(),
                    ));
                    next_list_function += 1;
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
    expected: ValueType,
    capture: Option<&CaptureSubstitution>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if let Some(capture) = capture
        && super::is_capture_local(&argument.value, &capture.name)
    {
        return Ok(capture.value.clone());
    }

    super::super::plan_expr_with_expected_source_stop_type(argument.value, expected, context)
}

#[cfg(test)]
mod tests {
    use super::function_call_param_locals;
    use crate::plan::{
        BitArrayListLocalId, BoolListLocalId, FloatListLocalId, FunctionListLocalId, FunctionType,
        IntListLocalId, ListListLocalId, ListLocal, NilListLocalId, ParamLocal, StringListLocalId,
        TupleListLocalId, UtfCodepointListLocalId, ValueType,
    };

    #[test]
    fn function_call_param_locals_preserve_family_local_order() {
        assert_eq!(
            function_call_param_locals(&[
                ValueType::Int,
                ValueType::Float,
                ValueType::String,
                ValueType::BitArray,
                ValueType::UtfCodepoint,
                ValueType::Bool,
                ValueType::Nil,
                ValueType::Int,
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::List(Box::new(ValueType::String)),
                ValueType::List(Box::new(ValueType::BitArray)),
                ValueType::List(Box::new(ValueType::UtfCodepoint)),
                ValueType::List(Box::new(ValueType::Float)),
                ValueType::List(Box::new(ValueType::Bool)),
                ValueType::List(Box::new(ValueType::Nil)),
                ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                ValueType::List(Box::new(ValueType::List(Box::new(ValueType::String)))),
                ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::String,
                ))))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::String],
                    ValueType::String,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::BitArray],
                    ValueType::BitArray,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::UtfCodepoint],
                    ValueType::UtfCodepoint,
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Float],
                    ValueType::Float,
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
                    vec![ValueType::Tuple(vec![ValueType::Int])],
                    ValueType::Tuple(vec![ValueType::Int]),
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::List(Box::new(ValueType::Int))],
                    ValueType::List(Box::new(ValueType::Int)),
                ))),
                ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int,))),
                ))),
            ]),
            vec![
                ParamLocal::int(crate::plan::IntLocalId(0)),
                ParamLocal::float(crate::plan::FloatLocalId(0)),
                ParamLocal::string(crate::plan::StringLocalId(0)),
                ParamLocal::bit_array(crate::plan::BitArrayLocalId(0)),
                ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(0)),
                ParamLocal::bool(crate::plan::BoolLocalId(0)),
                ParamLocal::nil(crate::plan::NilLocalId(0)),
                ParamLocal::int(crate::plan::IntLocalId(1)),
                ParamLocal::list(ListLocal::int(IntListLocalId(0))),
                ParamLocal::list(ListLocal::string(StringListLocalId(0))),
                ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(0))),
                ParamLocal::list(ListLocal::utf_codepoint(UtfCodepointListLocalId(0))),
                ParamLocal::list(ListLocal::float(FloatListLocalId(0))),
                ParamLocal::list(ListLocal::bool(BoolListLocalId(0))),
                ParamLocal::list(ListLocal::nil(NilListLocalId(0))),
                ParamLocal::list(ListLocal::tuple(TupleListLocalId(0), vec![ValueType::Int])),
                ParamLocal::list(ListLocal::list(ListListLocalId(0), ValueType::String)),
                ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::String),
                )),
                ParamLocal::int_function(
                    crate::plan::IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
                ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                ),
                ParamLocal::bit_array_function(
                    crate::plan::BitArrayFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
                ),
                ParamLocal::utf_codepoint_function(
                    crate::plan::UtfCodepointFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
                ),
                ParamLocal::float_function(
                    crate::plan::FloatFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Float], ValueType::Float),
                ),
                ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                ),
                ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                ),
                ParamLocal::tuple_function(
                    crate::plan::TupleFunctionLocalId(0),
                    FunctionType::new(
                        vec![ValueType::Tuple(vec![ValueType::Int])],
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                ),
                ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    FunctionType::new(
                        vec![ValueType::List(Box::new(ValueType::Int))],
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    ValueType::Int,
                )),
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
