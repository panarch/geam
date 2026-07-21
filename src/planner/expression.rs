mod bit_array;
mod block;
mod call;
mod case;
mod constant;
mod function;
mod operator;
mod pipeline;
mod record_access;
mod record_update;
mod var;

use crate::plan::{
    BitArrayExpr, BoolExpr, CustomExpr, CustomFunctionExpr, Expr, FloatExpr, FunctionExpr,
    FunctionFunctionExpr, FunctionShape, GenericExpr, GenericFunctionExpr, IntExpr, ListExpr,
    PanicExpr, StringExpr, TupleExpr, ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    UnsupportedExpressionKind,
};
use gleam_core::ast::TodoKind;
use gleam_core::ast::TypedExpr;

pub(super) fn plan_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let shape = context.value_shape(&expression.type_());
    let expression = match expression {
        TypedExpr::Int { int_value, .. } => Ok(Expr::int(IntExpr::value(int_value))),
        TypedExpr::String { value, .. } => Ok(Expr::string(StringExpr::value(value))),
        TypedExpr::Float { float_value, .. } => {
            Ok(Expr::float(FloatExpr::value(float_value.value())))
        }
        TypedExpr::Var {
            constructor, name, ..
        } => var::plan_var(name, constructor, shape.clone(), context),
        TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        } => call::plan_call(type_, *fun, arguments, context),
        TypedExpr::BinOp {
            operator,
            left,
            right,
            ..
        } => operator::plan_bin_op(operator, *left, *right, context),
        TypedExpr::NegateInt { value, .. } => operator::plan_negate_int(*value, context),
        TypedExpr::NegateBool { value, .. } => operator::plan_negate_bool(*value, context),
        TypedExpr::Block { statements, .. } => block::plan(statements, context),
        TypedExpr::Tuple {
            type_, elements, ..
        } => plan_tuple(type_, elements, context),
        TypedExpr::TupleIndex {
            type_,
            index,
            tuple,
            ..
        } => plan_tuple_index(type_, index, *tuple, context),
        TypedExpr::Pipeline {
            first_value,
            assignments,
            finally,
            finally_kind,
            ..
        } => pipeline::plan(first_value, assignments, *finally, finally_kind, context),
        TypedExpr::Fn {
            type_,
            kind,
            arguments,
            body,
            ..
        } => function::plan_anonymous(type_, kind, arguments, body, context),
        TypedExpr::List {
            type_,
            elements,
            tail,
            ..
        } => plan_list(type_, elements, tail.map(|tail| *tail), context),
        TypedExpr::Case {
            type_,
            subjects,
            clauses,
            ..
        } => case::plan_case(type_, subjects, clauses, context),
        TypedExpr::RecordAccess {
            type_,
            label,
            index,
            record,
            ..
        } => record_access::plan(type_, label, index, *record, context),
        TypedExpr::PositionalAccess { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::PositionalAccess,
            },
        }),
        TypedExpr::ModuleSelect { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleSelect,
            },
        }),
        TypedExpr::Todo {
            location,
            type_,
            kind,
            message,
            ..
        } => plan_todo_expr(
            location,
            kind,
            message.map(|message| *message),
            type_,
            context,
        ),
        TypedExpr::Panic {
            location,
            type_,
            message,
            ..
        } => plan_panic_expr(location, message.map(|message| *message), type_, context),
        TypedExpr::Echo { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Echo,
        }),
        TypedExpr::BitArray { segments, .. } => {
            bit_array::plan_expression(segments, context).map(Expr::bit_array)
        }
        TypedExpr::RecordUpdate {
            type_,
            updated_record,
            updated_record_assigned_name,
            constructor,
            arguments,
            ..
        } => record_update::plan(
            type_,
            *updated_record,
            updated_record_assigned_name,
            *constructor,
            arguments,
            context,
        ),
        TypedExpr::Invalid { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }?;
    let expression = if shape.value_type() == expression.value_type() {
        match expression.with_shape(shape.clone()) {
            Some(expression) => expression,
            None => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                });
            }
        }
    } else {
        expression
    };
    Ok(expression)
}

fn plan_panic_expr(
    location: gleam_core::ast::SrcSpan,
    message: Option<TypedExpr>,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_shape = context.value_shape(type_.as_ref());

    let site = context.panic_site(location);
    plan_panic_expr_with_shape(message, return_shape, site, context)
}

fn plan_todo_expr(
    location: gleam_core::ast::SrcSpan,
    kind: TodoKind,
    message: Option<TypedExpr>,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_shape = context.value_shape(type_.as_ref());

    plan_todo_expr_with_shape(location, kind, message, return_shape, context)
}

fn plan_panic_expr_with_shape(
    message: Option<TypedExpr>,
    return_shape: ValueShape,
    site: crate::plan::PanicSite,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let message = plan_panic_message(message, context)?;

    Ok(panic_expr(PanicExpr::panic_at(message, site), return_shape))
}

fn plan_todo_expr_with_shape(
    location: gleam_core::ast::SrcSpan,
    kind: TodoKind,
    message: Option<TypedExpr>,
    return_shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let site = match &kind {
        TodoKind::EmptyFunction { function_location } => context.panic_site(*function_location),
        TodoKind::Keyword | TodoKind::EmptyBlock | TodoKind::IncompleteUse => {
            context.panic_site(location)
        }
    };
    let panic = match kind {
        TodoKind::Keyword => PanicExpr::todo_at(plan_panic_message(message, context)?, site),
        TodoKind::EmptyFunction { .. } => {
            generated_todo_expr(message, PanicExpr::empty_function_at(site))?
        }
        TodoKind::EmptyBlock => generated_todo_expr(message, PanicExpr::empty_block_at(site))?,
        TodoKind::IncompleteUse => {
            generated_todo_expr(message, PanicExpr::incomplete_use_at(site))?
        }
    };

    Ok(panic_expr(panic, return_shape))
}

fn plan_panic_message(
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Option<StringExpr>, PlanError> {
    message
        .map(|message| plan_string_expr(message, context))
        .transpose()
}

fn generated_todo_expr(
    message: Option<TypedExpr>,
    expression: PanicExpr,
) -> Result<PanicExpr, PlanError> {
    if message.is_some() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        });
    }

    Ok(expression)
}

pub(super) fn plan_expr_with_expected_source_stop_type(
    expression: TypedExpr,
    expected: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    plan_expr_with_expected_source_stop_shape(
        expression,
        ValueShape::from_value_type(expected),
        context,
    )
}

pub(super) fn plan_expr_with_expected_source_stop_shape(
    expression: TypedExpr,
    expected: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match expression {
        TypedExpr::Todo {
            location,
            kind,
            message,
            ..
        } => plan_todo_expr_with_shape(
            location,
            kind,
            message.map(|message| *message),
            expected,
            context,
        ),
        TypedExpr::Panic {
            location, message, ..
        } => {
            let site = context.panic_site(location);
            plan_panic_expr_with_shape(message.map(|message| *message), expected, site, context)
        }
        TypedExpr::Block { statements, .. } => {
            block::plan_with_expected_source_stop_shape(statements, &expected, context)
        }
        expression => plan_expr(expression, context),
    }
}

fn panic_expr(panic: PanicExpr, return_shape: ValueShape) -> Expr {
    match return_shape {
        ValueShape::Parameter(parameter) => Expr::generic(GenericExpr::panic(parameter, panic)),
        ValueShape::Int => Expr::int(IntExpr::panic(panic)),
        ValueShape::String => Expr::string(StringExpr::panic(panic)),
        ValueShape::BitArray => Expr::bit_array(BitArrayExpr::panic(panic)),
        ValueShape::UtfCodepoint => {
            Expr::utf_codepoint(crate::plan::UtfCodepointExpr::panic(panic))
        }
        ValueShape::Custom(shape) => Expr::custom(CustomExpr::panic_shape(panic, shape)),
        ValueShape::Float => Expr::float(FloatExpr::panic(panic)),
        ValueShape::Bool => Expr::bool(BoolExpr::panic(panic)),
        ValueShape::Nil => Expr::nil(crate::plan::NilExpr::panic(panic)),
        ValueShape::Tuple(shape) => {
            let type_ = shape.iter().map(ValueShape::value_type).collect();
            Expr::tuple(TupleExpr::panic(panic, type_).with_shape(shape))
        }
        ValueShape::List(item_shape) => {
            let item_type = item_shape.value_type();
            Expr::list(ListExpr::panic(panic, item_type).with_item_shape(*item_shape))
        }
        ValueShape::Function(shape) => panic_function_expr(panic, *shape),
    }
}

fn panic_function_expr(panic: PanicExpr, shape: FunctionShape) -> Expr {
    let type_ = shape.type_();
    match shape.return_shape().clone() {
        ValueShape::Parameter(parameter) => {
            let callable =
                crate::plan::GenericFunctionType::new(shape.argument_shapes().to_vec(), parameter);
            Expr::function(FunctionExpr::generic(GenericFunctionExpr::panic(
                panic, callable,
            )))
        }
        ValueShape::Int => Expr::function(FunctionExpr::int_with_shape(
            crate::plan::IntFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::String => Expr::function(FunctionExpr::string_with_shape(
            crate::plan::StringFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::BitArray => Expr::function(FunctionExpr::bit_array_with_shape(
            crate::plan::BitArrayFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint_with_shape(
            crate::plan::UtfCodepointFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::Custom(return_shape) => {
            let callable = crate::plan::CustomFunctionType::from_shapes(
                shape.argument_shapes().to_vec(),
                return_shape,
            );
            Expr::function(FunctionExpr::custom(CustomFunctionExpr::panic(
                panic, callable,
            )))
        }
        ValueShape::Float => Expr::function(FunctionExpr::float_with_shape(
            crate::plan::FloatFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::Bool => Expr::function(FunctionExpr::bool_with_shape(
            crate::plan::BoolFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::Nil => Expr::function(FunctionExpr::nil_with_shape(
            crate::plan::NilFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::Tuple(_) => Expr::function(FunctionExpr::tuple_with_shape(
            crate::plan::TupleFunctionExpr::panic(panic, type_),
            shape,
        )),
        ValueShape::List(item_shape) => Expr::function(FunctionExpr::list_with_shape(
            crate::plan::ListFunctionExpr::panic(panic, type_, item_shape.value_type()),
            shape,
        )),
        ValueShape::Function(return_shape) => {
            let callable = crate::plan::FunctionFunctionType::from_shapes(
                shape.argument_shapes().to_vec(),
                *return_shape,
            );
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::panic(
                panic, callable,
            )))
        }
    }
}

fn plan_tuple(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let expected_shape = match context.value_shape(type_.as_ref()) {
        ValueShape::Tuple(shape) => shape,
        actual => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::from_value_type(actual.value_type()),
                },
            });
        }
    };

    let expected_type = expected_shape
        .iter()
        .map(ValueShape::value_type)
        .collect::<Vec<_>>();
    if elements.len() != expected_shape.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Tuple,
                actual: InvalidExpressionType::Tuple,
            },
        });
    }

    let planned_elements = elements
        .into_iter()
        .zip(&expected_shape)
        .map(|(element, expected)| {
            plan_expr_with_expected_source_stop_shape(element, expected.clone(), context)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let actual_type = planned_elements
        .iter()
        .map(Expr::value_type)
        .collect::<Vec<_>>();

    if expected_type != actual_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Tuple,
                actual: InvalidExpressionType::Tuple,
            },
        });
    }

    Ok(Expr::tuple(TupleExpr::value(
        planned_elements,
        expected_type,
    )))
}

fn plan_list(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
    tail: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let Some(list_element_type) = type_.list_type() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::from_value_type(context.value_type(type_.as_ref())),
            },
        });
    };

    let expected_item_shape = context.value_shape(list_element_type.as_ref());
    let expected_element_type = expected_item_shape.value_type();

    let planned_elements = elements
        .into_iter()
        .map(|element| {
            plan_expr_with_expected_source_stop_shape(element, expected_item_shape.clone(), context)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let Some(tail) = tail else {
        let list = match ListExpr::try_value(planned_elements, expected_element_type) {
            Ok(list) => list,
            Err(error) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::from_value_type(error.expected),
                        actual: InvalidExpressionType::from_value_type(error.actual),
                    },
                });
            }
        };
        return Ok(Expr::list(list));
    };

    let tail = plan_expr_with_expected_source_stop_shape(
        tail,
        ValueShape::List(Box::new(expected_item_shape.clone())),
        context,
    )?;
    let actual = tail.value_type();
    let Some(tail) = tail.into_list() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::from_value_type(actual),
            },
        });
    };

    let elements = match crate::plan::ListElements::from_exprs(
        expected_element_type.clone(),
        planned_elements,
    ) {
        Ok(elements) => elements,
        Err(error) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::from_value_type(error.expected),
                    actual: InvalidExpressionType::from_value_type(error.actual),
                },
            });
        }
    };
    let elements = match crate::plan::ListSpreadElements::from_parts(elements, tail) {
        Ok(elements) => elements,
        Err(crate::plan::ListSpreadConstructionError::EmptyPrefix) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            });
        }
        Err(crate::plan::ListSpreadConstructionError::ElementTypeMismatch(_)) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::List,
                },
            });
        }
    };
    Ok(Expr::list(ListExpr::from_spread_elements(elements)))
}

fn plan_tuple_index(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    index: u64,
    tuple: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    #[cfg(target_pointer_width = "64")]
    let index = index as usize;
    #[cfg(not(target_pointer_width = "64"))]
    let index = usize::try_from(index).map_err(|_| PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Tuple,
            actual: InvalidExpressionType::Tuple,
        },
    })?;
    let tuple = plan_expr(tuple, context)?;
    let actual = expression_type(&tuple);
    let Some(tuple) = tuple.into_tuple() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Tuple,
                actual,
            },
        });
    };
    let expected = context.value_type(type_.as_ref());
    tuple_index_expr(tuple, index, expected)
}

pub(super) fn tuple_index_expr(
    tuple: TupleExpr,
    index: usize,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    let Some(shape) = tuple.shape().get(index).cloned() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::from_value_type(return_type),
                actual: InvalidExpressionType::Tuple,
            },
        });
    };
    let actual = shape.value_type();
    if actual != return_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::from_value_type(return_type),
                actual: InvalidExpressionType::from_value_type(actual),
            },
        });
    }
    Ok(Expr::tuple_index_shape(tuple, index, shape))
}

pub(super) fn list_index_expr(
    list: ListExpr,
    index: usize,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    let item_shape = list.item_shape().clone();
    let expected = ValueType::List(Box::new(return_type.clone()));
    let actual = list.element_type();
    if item_shape.value_type() != return_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::from_value_type(expected),
                actual: InvalidExpressionType::from_value_type(actual),
            },
        });
    }
    Ok(match (item_shape, list) {
        (crate::plan::ValueShape::Parameter(_), ListExpr::Generic(list)) => {
            Expr::generic(GenericExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::Int, ListExpr::Int(list)) => {
            Expr::int(IntExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::String, ListExpr::String(list)) => {
            Expr::string(StringExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::BitArray, ListExpr::BitArray(list)) => {
            Expr::bit_array(BitArrayExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::UtfCodepoint, ListExpr::UtfCodepoint(list)) => {
            Expr::utf_codepoint(crate::plan::UtfCodepointExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::Custom(shape), ListExpr::Custom(list))
            if list.item().item_type() == *shape.type_() =>
        {
            Expr::custom(CustomExpr::list_index_shape(list, index, shape))
        }
        (crate::plan::ValueShape::Float, ListExpr::Float(list)) => {
            Expr::float(FloatExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::Bool, ListExpr::Bool(list)) => {
            Expr::bool(BoolExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::Nil, ListExpr::Nil(list)) => {
            Expr::nil(crate::plan::NilExpr::list_index(list, index))
        }
        (crate::plan::ValueShape::Tuple(shape), ListExpr::Tuple(list))
            if list.item().item_type()
                == shape
                    .iter()
                    .map(crate::plan::ValueShape::value_type)
                    .collect::<Vec<_>>() =>
        {
            let type_ = shape
                .iter()
                .map(crate::plan::ValueShape::value_type)
                .collect();
            Expr::tuple(TupleExpr::list_index(list, index, type_).with_shape(shape))
        }
        (crate::plan::ValueShape::List(item_shape), ListExpr::ParameterList(list))
            if matches!(item_shape.as_ref(), crate::plan::ValueShape::Parameter(_)) =>
        {
            Expr::list(ListExpr::parameter_list_index(list, index).with_item_shape(*item_shape))
        }
        (crate::plan::ValueShape::List(item_shape), ListExpr::List(list))
            if list.item().item_type() == Box::new(item_shape.value_type()) =>
        {
            Expr::list(ListExpr::list_index(list, index).with_item_shape(*item_shape))
        }
        (crate::plan::ValueShape::Function(shape), ListExpr::Function(list))
            if list.item().item_type() == shape.type_() =>
        {
            list_index_function_expr(list, index, *shape)
        }
        _ => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::from_value_type(expected),
                    actual: InvalidExpressionType::from_value_type(actual),
                },
            });
        }
    })
}

fn list_index_function_expr(
    list: crate::plan::FunctionListExpr,
    index: usize,
    shape: crate::plan::FunctionShape,
) -> Expr {
    let type_ = shape.type_();
    match shape.return_shape().clone() {
        crate::plan::ValueShape::Parameter(parameter) => {
            Expr::function(FunctionExpr::generic_with_shape(
                crate::plan::GenericFunctionExpr::list_index(
                    list.clone(),
                    index,
                    crate::plan::GenericFunctionType::new(
                        shape.argument_shapes().to_vec(),
                        parameter,
                    ),
                ),
                shape,
            ))
        }
        crate::plan::ValueShape::Int => Expr::function(FunctionExpr::int_with_shape(
            crate::plan::IntFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::String => Expr::function(FunctionExpr::string_with_shape(
            crate::plan::StringFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::BitArray => Expr::function(FunctionExpr::bit_array_with_shape(
            crate::plan::BitArrayFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::UtfCodepoint => {
            Expr::function(FunctionExpr::utf_codepoint_with_shape(
                crate::plan::UtfCodepointFunctionExpr::list_index(list.clone(), index, type_),
                shape,
            ))
        }
        crate::plan::ValueShape::Custom(return_shape) => {
            Expr::function(FunctionExpr::custom(CustomFunctionExpr::list_index(
                list.clone(),
                index,
                crate::plan::CustomFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    return_shape,
                ),
            )))
        }
        crate::plan::ValueShape::Float => Expr::function(FunctionExpr::float_with_shape(
            crate::plan::FloatFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::Bool => Expr::function(FunctionExpr::bool_with_shape(
            crate::plan::BoolFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::Nil => Expr::function(FunctionExpr::nil_with_shape(
            crate::plan::NilFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::Tuple(_) => Expr::function(FunctionExpr::tuple_with_shape(
            crate::plan::TupleFunctionExpr::list_index(list.clone(), index, type_),
            shape,
        )),
        crate::plan::ValueShape::List(item_shape) => Expr::function(FunctionExpr::list_with_shape(
            crate::plan::ListFunctionExpr::list_index(
                list.clone(),
                index,
                type_,
                item_shape.value_type(),
            ),
            shape,
        )),
        crate::plan::ValueShape::Function(return_shape) => {
            Expr::function(FunctionExpr::function_with_shape(
                FunctionFunctionExpr::list_index(
                    list,
                    index,
                    crate::plan::FunctionFunctionType::from_shapes(
                        shape.argument_shapes().to_vec(),
                        *return_shape,
                    ),
                ),
                shape,
            ))
        }
    }
}

pub(super) fn plan_use_call(
    call: TypedExpr,
    use_assignment_count: usize,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    call::plan_use_call(call, use_assignment_count, context)
}

fn plan_int_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<IntExpr, PlanError> {
    let expression = plan_expr_with_expected_source_stop_type(expression, ValueType::Int, context)?;
    let actual = expression_type(&expression);
    match expression.into_int() {
        Some(expression) => Ok(expression),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Int,
                actual,
            },
        }),
    }
}

pub(super) fn plan_string_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::String, context)?;
    let actual = expression_type(&expression);
    match expression.into_string() {
        Some(expression) => Ok(expression),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::String,
                actual,
            },
        }),
    }
}

fn plan_float_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<FloatExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::Float, context)?;
    let actual = expression_type(&expression);
    match expression.into_float() {
        Some(expression) => Ok(expression),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Float,
                actual,
            },
        }),
    }
}

pub(super) fn plan_bool_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<BoolExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::Bool, context)?;
    let actual = expression_type(&expression);
    match expression.into_bool() {
        Some(expression) => Ok(expression),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Bool,
                actual,
            },
        }),
    }
}

fn expression_type(expression: &Expr) -> InvalidExpressionType {
    InvalidExpressionType::from_value_type(expression.value_type())
}

#[cfg(test)]
pub(in crate::planner::expression) fn module_returning_typed_expr(
    expression: TypedExpr,
) -> gleam_core::ast::TypedModule {
    let mut module = crate::planner::support::compile_minimal_module();
    module.definitions.functions[0].body = vec![gleam_core::ast::Statement::Expression(expression)];
    module
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_int_expr(value: i64) -> TypedExpr {
    use num_bigint::BigInt;

    TypedExpr::Int {
        location: crate::planner::support::dummy_span(),
        type_: gleam_core::type_::int(),
        value: value.to_string().into(),
        int_value: BigInt::from(value),
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_string_expr(value: &str) -> TypedExpr {
    TypedExpr::String {
        location: crate::planner::support::dummy_span(),
        type_: gleam_core::type_::string(),
        value: value.into(),
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_tuple_expr(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
) -> TypedExpr {
    TypedExpr::Tuple {
        location: crate::planner::support::dummy_span(),
        type_,
        elements,
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_prelude_constructor(
    name: &str,
    type_: std::sync::Arc<gleam_core::type_::Type>,
) -> TypedExpr {
    use gleam_core::ast::Publicity;
    use gleam_core::type_::{
        Deprecation, PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant,
    };

    TypedExpr::Var {
        location: crate::planner::support::dummy_span(),
        name: name.into(),
        constructor: ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_,
            variant: ValueConstructorVariant::Record {
                name: name.into(),
                arity: 0,
                field_map: None,
                location: crate::planner::support::dummy_span(),
                module: PRELUDE_MODULE_NAME.into(),
                variants_count: 1,
                variant_index: 0,
                documentation: None,
            },
        },
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{
        expression_type, list_index_function_expr, module_returning_typed_expr, typed_int_expr,
        typed_string_expr, typed_tuple_expr,
    };
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionId, BoolLocalId, CustomExpr,
        CustomLocalId, CustomType, CustomTypeName, Expr, FloatExpr, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionType, FunctionReference,
        FunctionType, GenericExpr, GenericFunctionExpr, GenericFunctionLocal,
        GenericFunctionLocalId, GenericFunctionType, GenericLocal, GenericLocalId, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId, ListExpr, NilExpr,
        NilLocalId, PanicExpr, PanicSite, ParamLocal, ReturnBody, SourceSpan, StringExpr,
        StringLocalId, TupleExpr, TypeParameterId, UtfCodepointExpr, UtfCodepointLocalId,
        ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bool_, bool_function_ref, float, float_function_ref, function, function_function_ref, int,
        int_function_ref, let_list_step, let_tuple_step, list, list_function_ref, list_spread,
        local_bool, local_float, local_int, local_list, local_nil, local_string, local_tuple,
        module, nil, nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::{Constant, Statement, TypedExpr, TypedModule};
    use gleam_core::type_::{self, ModuleValueConstructor, Type};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn plan_panic_and_todo_return_shapes() {
        let actual = plan_module(compile(
            r#"
pub fn main() -> Int {
  panic as "boom"
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::int(
                IntFunctionId(0),
                IntExpr::panic(PanicExpr::panic_at(
                    Some(StringExpr::value("boom".into())),
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(26, 41)),
                )),
            ),
        );

        let actual = plan_module(compile(
            r#"
pub fn main() -> Bool {
  todo
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::bool(
                BoolFunctionId(0),
                BoolExpr::panic(PanicExpr::todo_at(
                    None,
                    PanicSite::new("main".into(), "main".into(), SourceSpan::new(27, 31)),
                )),
            ),
        );
    }

    #[test]
    fn expression_planning_rejects_conflicting_constructor_refinement_metadata() {
        let mut module = compile(
            r#"
pub type Choice {
  First
  Second
}

pub fn main() {
  First
}
"#,
        );
        set_main_constructor_inferred_variant(&mut module, 1);

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_conflicting_local_refinement_metadata() {
        let source = r#"
pub type Choice {
  First
  Second
}

pub fn main() {
  let value = First
  value
}
"#;
        let mut refinement_mismatch = compile(source);
        let expression = main_final_expression_mut(&mut refinement_mismatch);
        set_expression_constructor_inferred_variant(expression, 1);
        assert_eq!(
            plan_module(refinement_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "test final statement should be an expression")]
    fn final_expression_fixture_guard_rejects_assignment() {
        let mut module = compile("pub fn main() { let value = 1 value }");
        module.definitions.functions[0].body.pop();

        main_final_expression_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "test expression should be a constructor value")]
    fn constructor_refinement_fixture_guard_rejects_assignment() {
        let mut module = compile("pub fn main() { let value = 1 value }");

        set_main_constructor_inferred_variant(&mut module, 0);
    }

    #[test]
    #[should_panic(expected = "test expression should be a constructor value")]
    fn constructor_refinement_fixture_guard_rejects_non_variable_expression() {
        let mut module = compile("pub fn main() { 1 }");

        set_main_constructor_inferred_variant(&mut module, 0);
    }

    #[test]
    #[should_panic(expected = "test constructor should have a named custom type")]
    fn constructor_refinement_fixture_guard_rejects_tuple_variable() {
        let mut module = compile("pub fn main(value: #(Int)) { value }");

        set_main_constructor_inferred_variant(&mut module, 0);
    }

    fn set_main_constructor_inferred_variant(module: &mut TypedModule, index: u16) {
        let Statement::Expression(expression) = &mut module.definitions.functions[0].body[0] else {
            panic!("test expression should be a constructor value");
        };
        set_expression_constructor_inferred_variant(expression, index);
    }

    fn set_expression_constructor_inferred_variant(expression: &mut TypedExpr, index: u16) {
        let TypedExpr::Var { constructor, .. } = expression else {
            panic!("test expression should be a constructor value");
        };
        let Type::Named {
            publicity,
            package,
            module: type_module,
            name,
            arguments,
            ..
        } = constructor.type_.as_ref()
        else {
            panic!("test constructor should have a named custom type");
        };
        constructor.type_ = std::sync::Arc::new(Type::Named {
            publicity: *publicity,
            package: package.clone(),
            module: type_module.clone(),
            name: name.clone(),
            arguments: arguments.clone(),
            inferred_variant: Some(index),
        });
    }

    fn main_final_expression_mut(module: &mut TypedModule) -> &mut TypedExpr {
        let Some(Statement::Expression(expression)) =
            module.definitions.functions[0].body.last_mut()
        else {
            panic!("test final statement should be an expression");
        };
        expression
    }

    #[test]
    fn plan_generated_todo_kinds_are_distinct() {
        let actual = plan_module(compile(
            r#"
pub fn main() -> Int {
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::int(
                IntFunctionId(0),
                IntExpr::panic(PanicExpr::empty_function_at(PanicSite::new(
                    "main".into(),
                    "main".into(),
                    SourceSpan::new(1, 21),
                ))),
            ),
        );

        let actual = plan_module(compile(
            r#"
pub fn main() -> Int {
  {}
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::int_body(ReturnBody::block(
                Vec::new(),
                ReturnBody::expr(IntExpr::panic(PanicExpr::empty_block_at(PanicSite::new(
                    "main".into(),
                    "main".into(),
                    SourceSpan::new(26, 28)
                ),))),
            ),),
        );

        let actual = plan_module(compile(
            r#"
fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() -> Int {
  use value <- with_value
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.anonymous_functions()[0].return_(),
            &crate::plan::ReturnExpr::int(
                IntFunctionId(2),
                IntExpr::panic(PanicExpr::incomplete_use_at(PanicSite::new(
                    "main".into(),
                    "<anonymous:0>".into(),
                    SourceSpan::new(85, 108),
                ))),
            ),
        );
    }

    #[test]
    fn plan_source_stop_expression_shapes_directly() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let panic = PanicExpr::panic_at(None, context.panic_site(dummy_span()));

        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::int(),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::int(IntExpr::panic(panic.clone()))),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::string(),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::string(StringExpr::panic(panic.clone()))),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::float(),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::float(FloatExpr::panic(panic.clone()))),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::bool(),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::panic(panic.clone()))),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::tuple(TupleExpr::panic(
                panic.clone(),
                vec![ValueType::Int],
            ))),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::list(ListExpr::panic(panic.clone(), ValueType::Int))),
        );
        let int_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::fn_(vec![type_::int()], type_::int()),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::function(FunctionExpr::int(
                crate::plan::IntFunctionExpr::panic(panic.clone(), int_function_type),
            ))),
        );
        let function_function_type = FunctionFunctionType::new(
            Vec::new(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::fn_(Vec::new(), type_::fn_(vec![type_::int()], type_::int())),
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::panic(panic, function_function_type),
            ))),
        );

        assert_eq!(
            super::plan_expr(
                TypedExpr::Todo {
                    location: dummy_span(),
                    type_: type_::string(),
                    kind: gleam_core::ast::TodoKind::Keyword,
                    message: None,
                },
                &mut context,
            ),
            Ok(Expr::string(StringExpr::panic(PanicExpr::todo_at(
                None,
                context.panic_site(dummy_span()),
            )))),
        );
    }

    #[test]
    fn expression_family_mismatch_is_deferred_to_the_enclosing_owner() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_expr(
                TypedExpr::Int {
                    location: dummy_span(),
                    type_: type_::bool(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
                &mut context,
            ),
            Ok(Expr::int(IntExpr::value(BigInt::from(1)))),
        );
    }

    #[test]
    fn reject_profile_source_stop_message_expression_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() -> Int {
  panic as echo "boom"
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() -> Int {
  todo as echo "later"
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_source_stop_expression_shapes() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let site = context.panic_site(dummy_span());
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    message: None,
                },
                &mut context
            ),
            Ok(Expr::generic(GenericExpr::panic(
                parameter,
                PanicExpr::panic_at(None, site.clone()),
            ))),
        );

        assert_eq!(
            super::plan_expr(
                TypedExpr::Todo {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    kind: gleam_core::ast::TodoKind::Keyword,
                    message: None,
                },
                &mut context
            ),
            Ok(Expr::generic(GenericExpr::panic(
                parameter,
                PanicExpr::todo_at(None, site),
            ))),
        );

        for kind in [
            gleam_core::ast::TodoKind::EmptyFunction {
                function_location: dummy_span(),
            },
            gleam_core::ast::TodoKind::EmptyBlock,
            gleam_core::ast::TodoKind::IncompleteUse,
        ] {
            assert_eq!(
                super::plan_expr(
                    TypedExpr::Todo {
                        location: dummy_span(),
                        type_: type_::int(),
                        kind,
                        message: Some(Box::new(typed_string_expr("generated message"))),
                    },
                    &mut context
                ),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                }),
            );
        }
    }

    #[test]
    fn reject_profile_expression_variants() {
        assert_eq!(
            expect_plan_error(r#"pub fn main() { echo 1 }"#),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_positional_record_access() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::PositionalAccess {
                location: dummy_span(),
                type_: type_::int(),
                index: 0,
                record: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PositionalAccess,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_expression_shapes() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::ModuleSelect {
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
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_expression() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::Invalid {
                location: dummy_span(),
                type_: type_::int(),
                extra_information: None,
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_plan_expr_error_propagates_through_typed_expression_helpers() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let expected = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        };

        assert_eq!(
            super::plan_int_expr(invalid_expr(type_::int()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_string_expr(invalid_expr(type_::string()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_float_expr(invalid_expr(type_::float()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_bool_expr(invalid_expr(type_::bool()), &mut context),
            Err(expected),
        );
    }

    #[test]
    fn expression_type_classifies_parameter_and_compound_families() {
        let expression = Expr::function(FunctionExpr::reference(FunctionReference::new(
            crate::plan::monomorphic_function_instantiation(
                0,
                crate::plan::FunctionShape::new(Vec::new(), ValueShape::Nil),
            ),
        )));
        let list_expression = Expr::from(list([int(1)], ValueType::Int));
        let nil_expression = Expr::from(nil());
        let bit_array_expression = Expr::bit_array(BitArrayExpr::value(Vec::new()));
        let utf_codepoint_expression = Expr::utf_codepoint(UtfCodepointExpr::local_get(
            UtfCodepointLocalId(0),
            "codepoint".into(),
        ));
        let custom_expression = Expr::custom(CustomExpr::local_get(
            crate::plan::CustomLocal::new(
                CustomLocalId(0),
                CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    Vec::new(),
                ),
            ),
            "boxed".into(),
        ));
        let generic_expression = Expr::generic(crate::plan::GenericExpr::local_get(
            crate::plan::GenericLocal::new(
                crate::plan::GenericLocalId(0),
                crate::plan::TypeParameterId(0),
            ),
            "value".into(),
        ));

        assert_eq!(
            [
                expression_type(&generic_expression),
                expression_type(&expression),
                expression_type(&list_expression),
                expression_type(&nil_expression),
                expression_type(&bit_array_expression),
                expression_type(&utf_codepoint_expression),
                expression_type(&custom_expression),
            ],
            [
                InvalidExpressionType::TypeParameter,
                InvalidExpressionType::Function,
                InvalidExpressionType::List,
                InvalidExpressionType::Nil,
                InvalidExpressionType::BitArray,
                InvalidExpressionType::UtfCodepoint,
                InvalidExpressionType::Custom,
            ],
        );
    }

    #[test]
    fn list_index_function_expr_preserves_bit_array_return_family() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let shape = crate::plan::FunctionShape::from_function_type(type_.clone());
        let list = ListExpr::value(Vec::new(), ValueType::Function(Box::new(type_.clone())))
            .into_function()
            .expect("function list");

        assert_eq!(
            list_index_function_expr(list.clone(), 2, shape.clone()),
            Expr::function(FunctionExpr::bit_array_with_shape(
                BitArrayFunctionExpr::list_index(list, 2, type_),
                shape,
            )),
        );
    }

    #[test]
    fn plan_list_index_expr_preserves_bit_array_item_family() {
        let list = ListExpr::value(Vec::new(), ValueType::BitArray);

        assert_eq!(
            super::list_index_expr(list.clone(), 2, ValueType::BitArray),
            Ok(Expr::bit_array(BitArrayExpr::list_index(
                list.into_bit_array().expect("bit array list"),
                2,
            ))),
        );
    }

    #[test]
    fn generic_projection_and_panic_helpers_preserve_parameter_shapes() {
        let parameter = TypeParameterId(0);
        let local = GenericLocal::new(GenericLocalId(0), parameter);
        let generic = GenericExpr::local_get(local, "value".into());
        let tuple = TupleExpr::value(
            vec![Expr::generic(generic)],
            vec![ValueType::Parameter(parameter)],
        );
        assert_eq!(
            super::tuple_index_expr(tuple.clone(), 0, ValueType::Parameter(parameter)),
            Ok(Expr::generic(
                GenericExpr::tuple_index(parameter, tuple, 0,)
            )),
        );

        let list = ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty parameter list has generic list storage");
        let typed_list = list
            .clone()
            .into_generic()
            .expect("a parameter item list has generic list storage");
        assert_eq!(
            super::list_index_expr(list, 1, ValueType::Parameter(parameter)),
            Ok(Expr::generic(GenericExpr::list_index(typed_list, 1))),
        );

        let nested_list = ListExpr::try_value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .expect("an empty nested parameter list should preserve its item shape");
        let typed_nested_list = nested_list
            .clone()
            .into_parameter_list()
            .expect("a nested parameter item should create a parameter-list expression");
        assert_eq!(
            super::list_index_expr(
                nested_list,
                2,
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
            ),
            Ok(Expr::list(
                ListExpr::parameter_list_index(typed_nested_list, 2)
                    .with_item_shape(ValueShape::Parameter(parameter)),
            )),
        );

        let parameter_list = ListExpr::try_value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .expect("an empty nested parameter list should preserve its item shape")
        .with_item_shape(ValueShape::List(Box::new(ValueShape::Int)));
        assert_eq!(
            super::list_index_expr(parameter_list, 2, ValueType::List(Box::new(ValueType::Int)),),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::List,
                },
            }),
        );

        let function_shape = crate::plan::FunctionShape::new(
            vec![ValueShape::Int],
            ValueShape::Parameter(parameter),
        );
        let function_type = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let function = GenericFunctionExpr::local_get(
            GenericFunctionLocal::new(GenericFunctionLocalId(0), function_type.clone()),
            "function".into(),
        );
        let tuple = TupleExpr::value(
            vec![Expr::function(FunctionExpr::generic(function))],
            vec![ValueType::Function(Box::new(function_shape.type_()))],
        );
        assert_eq!(
            super::tuple_index_expr(
                tuple.clone(),
                0,
                ValueType::Function(Box::new(function_shape.type_())),
            ),
            Ok(Expr::function(FunctionExpr::generic_with_shape(
                GenericFunctionExpr::tuple_index(tuple, 0, function_type.clone()),
                function_shape.clone(),
            ))),
        );

        let list = ListExpr::value(
            Vec::new(),
            ValueType::Function(Box::new(function_shape.type_())),
        );
        let typed_list = list
            .clone()
            .into_function()
            .expect("a function item list has function list storage");
        assert_eq!(
            super::list_index_expr(
                list,
                2,
                ValueType::Function(Box::new(function_shape.type_())),
            ),
            Ok(Expr::function(FunctionExpr::generic_with_shape(
                GenericFunctionExpr::list_index(typed_list, 2, function_type.clone()),
                function_shape.clone(),
            ))),
        );

        let panic = PanicExpr::panic_at(
            None,
            PanicSite::new("main".into(), "generic".into(), SourceSpan::new(0, 1)),
        );
        assert_eq!(
            super::panic_function_expr(panic.clone(), function_shape),
            Expr::function(FunctionExpr::generic(GenericFunctionExpr::panic(
                panic,
                function_type,
            ))),
        );
    }

    #[test]
    fn plan_tuple_index_result_families() {
        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let values = #(True, Nil, add_one)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [
                            ValueType::Bool,
                            ValueType::Nil,
                            ValueType::Function(Box::new(int_to_int_type())),
                        ],
                    )
                    .index_bool(0),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([
                        Expr::from(bool_(true)),
                        Expr::from(nil()),
                        Expr::from(int_function_ref(1, [ParamLocal::int(IntLocalId(0))])),
                    ]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #("ok")
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::String]).index_string(0),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(string("ok"))]),
                )),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(1.5)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Float]).index_float(0),
                )
                .step(let_tuple_step(0, "values", tuple([Expr::from(float(1.5))]))),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(#(1))
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Tuple(vec![ValueType::Int])])
                        .index_tuple(0, [ValueType::Int]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(tuple([Expr::from(int(1))]))]),
                )),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #([1])
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::List(Box::new(ValueType::Int))])
                        .index_list(0, ValueType::Int),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(list([int(1)], ValueType::Int))]),
                )),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let values = #(add_one)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(int_to_int_type()))],
                    )
                    .index_int_function(0, [ValueType::Int]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(int_function_ref(
                        1,
                        [ParamLocal::int(IntLocalId(0))],
                    ))]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn text(value: String) {
  value
}

pub fn main() {
  let values = #(text)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(string_to_string_type()))],
                    )
                    .index_string_function(0, [ValueType::String]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(string_function_ref(
                        1,
                        [ParamLocal::string(StringLocalId(0))],
                    ))]),
                )),
                [function("text", local_string(0, "value")).param_string(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn number(value: Float) {
  value
}

pub fn main() {
  let values = #(number)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(float_to_float_type()))],
                    )
                    .index_float_function(0, [ValueType::Float]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(float_function_ref(
                        1,
                        [ParamLocal::float(crate::plan::FloatLocalId(0))],
                    ))]),
                )),
                [function("number", local_float(0, "value")).param_float(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn flag(value: Bool) {
  value
}

pub fn main() {
  let values = #(flag)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(bool_to_bool_type()))],
                    )
                    .index_bool_function(0, [ValueType::Bool]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(bool_function_ref(
                        1,
                        [ParamLocal::bool(BoolLocalId(0))],
                    ))]),
                )),
                [function("flag", local_bool(0, "value")).param_bool(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn unit(value: Nil) {
  value
}

pub fn main() {
  let values = #(unit)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(nil_to_nil_type()))],
                    )
                    .index_nil_function(0, [ValueType::Nil]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(nil_function_ref(
                        1,
                        [ParamLocal::nil(NilLocalId(0))],
                    ))]),
                )),
                [function("unit", local_nil(0, "value")).param_nil(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn tuple(value: Int) {
  #(value)
}

pub fn main() {
  let values = #(tuple)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(int_to_tuple_type()))],
                    )
                    .index_tuple_function(
                        0,
                        [ValueType::Int],
                        [ValueType::Int],
                    ),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(tuple_function_ref(
                        1,
                        [ParamLocal::int(IntLocalId(0))],
                        [ValueType::Int],
                    ))]),
                )),
                [
                    function("tuple", tuple([Expr::from(local_int(0, "value"))]))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn values(value: Int) {
  [value]
}

pub fn main() {
  let functions = #(values)
  functions.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "functions",
                        [ValueType::Function(Box::new(int_to_list_type()))],
                    )
                    .index_list_function(0, [ValueType::Int], ValueType::Int),
                )
                .step(let_tuple_step(
                    0,
                    "functions",
                    tuple([Expr::from(list_function_ref(
                        1,
                        [ParamLocal::int(IntLocalId(0))],
                        ValueType::Int,
                    ))]),
                )),
                [function(
                    "values",
                    list([Expr::from(local_int(0, "value"))], ValueType::Int),
                )
                .param_int(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  let values = #(get)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Function(Box::new(getter_type()))])
                        .index_function_function(0, Vec::<ValueType>::new(), int_to_int_type()),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(2)),
                        Vec::<ParamLocal>::new(),
                        int_to_int_type(),
                    ))]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                    function("get", int_function_ref(1, [ParamLocal::int(IntLocalId(0))])),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(Nil)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Nil]).index_nil(0),
                )
                .step(let_tuple_step(0, "values", tuple([Expr::from(nil())]))),
                [],
            ),
        );
    }

    fn assert_tuple_index_plan(src: &str, expected: crate::plan::ModulePlan) {
        let actual = plan_module(compile(src)).expect("source should plan");

        assert_eq!(actual, expected);
    }

    fn int_to_int_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_to_string_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn float_to_float_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn bool_to_bool_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_to_nil_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn int_to_tuple_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]))
    }

    fn int_to_list_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn getter_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Function(Box::new(int_to_int_type())))
    }

    #[test]
    fn plan_list_literal_shape() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let values = [1]
  values
}
"#,
            )),
            Ok(module(
                "main",
                function("main", local_list(0, "values", ValueType::Int)).step(let_list_step(
                    0,
                    "values",
                    list([int(1)], ValueType::Int),
                )),
                [],
            )),
        );
    }

    #[test]
    fn plan_list_spread_literal_shape() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let rest = [2, 3]
  [1, ..rest]
}
"#,
            )),
            Ok(module(
                "main",
                function(
                    "main",
                    list_spread(
                        [int(1)],
                        local_list(0, "rest", ValueType::Int),
                        ValueType::Int
                    ),
                )
                .step(let_list_step(
                    0,
                    "rest",
                    list([int(2), int(3)], ValueType::Int),
                )),
                [],
            )),
        );
    }

    #[test]
    fn reject_margin_tuple_expression_shapes() {
        let tuple_int = type_::tuple(vec![type_::int()]);
        let cases = [
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::int(),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::result(type_::int(), type_::nil()),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Custom,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::TypeParameter,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::list(type_::int())]),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::string()]),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![typed_int_expr(1), typed_int_expr(2)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![invalid_expr(type_::int())],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    tuple: Box::new(typed_int_expr(1)),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 1,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Int,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::result(type_::int(), type_::nil()),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Custom,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::TypeParameter,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::string(),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int, vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::String,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    tuple: Box::new(invalid_expr(type_::tuple(vec![type_::int()]))),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
        ];

        for (expression, expected) in cases {
            assert_eq!(
                plan_module(module_returning_typed_expr(expression)),
                Err(expected)
            );
        }
    }

    #[test]
    fn reject_margin_list_expression_shapes() {
        let cases = [
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::int(),
                    elements: vec![typed_int_expr(1)],
                    tail: None,
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::result(type_::int(), type_::nil()),
                    elements: vec![typed_int_expr(1)],
                    tail: None,
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::Custom,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    elements: vec![typed_int_expr(1)],
                    tail: None,
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::TypeParameter,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::string()),
                    elements: vec![typed_int_expr(1)],
                    tail: None,
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::String,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    elements: Vec::new(),
                    tail: Some(Box::new(TypedExpr::List {
                        location: dummy_span(),
                        type_: type_::list(type_::int()),
                        elements: vec![typed_int_expr(1)],
                        tail: None,
                    })),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::string()),
                    elements: vec![typed_int_expr(1)],
                    tail: Some(Box::new(TypedExpr::List {
                        location: dummy_span(),
                        type_: type_::list(type_::string()),
                        elements: vec![typed_string_expr("tail")],
                        tail: None,
                    })),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::String,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    elements: vec![invalid_expr(type_::int())],
                    tail: None,
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    elements: vec![typed_int_expr(1)],
                    tail: Some(Box::new(invalid_expr(type_::list(type_::int())))),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    elements: vec![typed_int_expr(1)],
                    tail: Some(Box::new(typed_int_expr(2))),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::List {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    elements: vec![typed_int_expr(1)],
                    tail: Some(Box::new(TypedExpr::List {
                        location: dummy_span(),
                        type_: type_::list(type_::string()),
                        elements: vec![typed_string_expr("two")],
                        tail: None,
                    })),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::List,
                        actual: InvalidExpressionType::List,
                    },
                },
            ),
        ];

        for (expression, expected) in cases {
            assert_eq!(
                plan_module(module_returning_typed_expr(expression)),
                Err(expected)
            );
        }
    }

    #[test]
    fn list_index_expr_preserves_typed_item_family_shapes() {
        let int_list = ListExpr::value(Vec::new(), ValueType::Int);
        let typed_int_list = int_list
            .clone()
            .into_int()
            .expect("int item list should build int list expression");
        assert_eq!(
            super::list_index_expr(int_list, 0, ValueType::Int),
            Ok(Expr::int(IntExpr::list_index(typed_int_list, 0))),
        );

        let string_list = ListExpr::value(Vec::new(), ValueType::String);
        let typed_string_list = string_list
            .clone()
            .into_string()
            .expect("string item list should build string list expression");
        assert_eq!(
            super::list_index_expr(string_list, 0, ValueType::String),
            Ok(Expr::string(StringExpr::list_index(typed_string_list, 0))),
        );

        let utf_codepoint_list = ListExpr::value(Vec::new(), ValueType::UtfCodepoint);
        let typed_utf_codepoint_list = utf_codepoint_list
            .clone()
            .into_utf_codepoint()
            .expect("utf codepoint item list should build utf codepoint list expression");
        assert_eq!(
            super::list_index_expr(utf_codepoint_list, 0, ValueType::UtfCodepoint),
            Ok(Expr::utf_codepoint(UtfCodepointExpr::list_index(
                typed_utf_codepoint_list,
                0,
            ))),
        );

        let float_list = ListExpr::value(Vec::new(), ValueType::Float);
        let typed_float_list = float_list
            .clone()
            .into_float()
            .expect("float item list should build float list expression");
        assert_eq!(
            super::list_index_expr(float_list, 0, ValueType::Float),
            Ok(Expr::float(FloatExpr::list_index(typed_float_list, 0))),
        );

        let bool_list = ListExpr::value(Vec::new(), ValueType::Bool);
        let typed_bool_list = bool_list
            .clone()
            .into_bool()
            .expect("bool item list should build bool list expression");
        assert_eq!(
            super::list_index_expr(bool_list, 0, ValueType::Bool),
            Ok(Expr::bool(BoolExpr::list_index(typed_bool_list, 0))),
        );

        let nil_list = ListExpr::value(Vec::new(), ValueType::Nil);
        let typed_nil_list = nil_list
            .clone()
            .into_nil()
            .expect("nil item list should build nil list expression");
        assert_eq!(
            super::list_index_expr(nil_list, 0, ValueType::Nil),
            Ok(Expr::nil(NilExpr::list_index(typed_nil_list, 0))),
        );

        let tuple_item_type = vec![ValueType::Int];
        let tuple_list = ListExpr::value(Vec::new(), ValueType::Tuple(tuple_item_type.clone()));
        let typed_tuple_list = tuple_list
            .clone()
            .into_tuple()
            .expect("tuple item list should build tuple list expression");
        assert_eq!(
            super::list_index_expr(tuple_list, 0, ValueType::Tuple(tuple_item_type.clone())),
            Ok(Expr::tuple(TupleExpr::list_index(
                typed_tuple_list,
                0,
                tuple_item_type,
            ))),
        );

        let nested_item_type = ValueType::String;
        let nested_list = ListExpr::value(
            Vec::new(),
            ValueType::List(Box::new(nested_item_type.clone())),
        );
        let typed_nested_list = nested_list
            .clone()
            .into_list()
            .expect("nested list item should build list list expression");
        assert_eq!(
            super::list_index_expr(nested_list, 0, ValueType::List(Box::new(nested_item_type)),),
            Ok(Expr::list(ListExpr::list_index(typed_nested_list, 0))),
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let list = ListExpr::value(
            Vec::new(),
            ValueType::Function(Box::new(function_type.clone())),
        );
        let typed_list = list
            .clone()
            .into_function()
            .expect("function item list should build function list expression");

        assert_eq!(
            super::list_index_expr(
                list,
                0,
                ValueType::Function(Box::new(function_type.clone()))
            ),
            Ok(Expr::function(FunctionExpr::int(
                IntFunctionExpr::list_index(typed_list, 0, function_type),
            ))),
        );

        let expected_function_type = FunctionType::new(Vec::new(), ValueType::String);
        let actual_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            super::list_index_expr(
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(actual_function_type)),
                ),
                0,
                ValueType::Function(Box::new(expected_function_type)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Function,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_index_rejects_facade_and_shape_family_conflict() {
        let list = ListExpr::value(Vec::new(), ValueType::Int).with_item_shape(ValueShape::String);

        assert_eq!(
            super::list_index_expr(list, 0, ValueType::String),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_float_expression_type_direct() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_float_expr(typed_string_expr("not float"), &mut context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    fn invalid_expr(type_: std::sync::Arc<type_::Type>) -> TypedExpr {
        TypedExpr::Invalid {
            location: dummy_span(),
            type_,
            extra_information: None,
        }
    }
}
