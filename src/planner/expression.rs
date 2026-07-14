mod bit_array;
mod block;
mod call;
mod case;
mod constant;
mod function;
mod operator;
mod pipeline;
mod var;

use crate::plan::{
    BitArrayExpr, BoolExpr, CustomExpr, CustomFunctionExpr, Expr, FloatExpr, FunctionExpr,
    FunctionFunctionExpr, FunctionType, IntExpr, ListExpr, PanicExpr, StringExpr, TupleExpr,
    ValueType,
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
    match expression {
        TypedExpr::Int { int_value, .. } => Ok(Expr::int(IntExpr::value(int_value))),
        TypedExpr::String { value, .. } => Ok(Expr::string(StringExpr::value(value))),
        TypedExpr::Float { float_value, .. } => {
            Ok(Expr::float(FloatExpr::value(float_value.value())))
        }
        TypedExpr::Var {
            constructor, name, ..
        } => var::plan_var(name, constructor, context),
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
        TypedExpr::RecordAccess { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::RecordAccess,
        }),
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
        TypedExpr::RecordUpdate { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::RecordUpdate,
        }),
        TypedExpr::Invalid { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }
}

fn plan_panic_expr(
    location: gleam_core::ast::SrcSpan,
    message: Option<TypedExpr>,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or_else(|| {
        invalid_expression_type(
            InvalidExpressionType::Unsupported,
            InvalidExpressionType::Unsupported,
        )
    })?;

    let site = context.panic_site(location);
    plan_panic_expr_with_type(message, return_type, site, context)
}

fn plan_todo_expr(
    location: gleam_core::ast::SrcSpan,
    kind: TodoKind,
    message: Option<TypedExpr>,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or_else(|| {
        invalid_expression_type(
            InvalidExpressionType::Unsupported,
            InvalidExpressionType::Unsupported,
        )
    })?;

    plan_todo_expr_with_type(location, kind, message, return_type, context)
}

fn plan_panic_expr_with_type(
    message: Option<TypedExpr>,
    return_type: ValueType,
    site: crate::plan::PanicSite,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let message = plan_panic_message(message, context)?;

    Ok(panic_expr(PanicExpr::panic_at(message, site), return_type))
}

fn plan_todo_expr_with_type(
    location: gleam_core::ast::SrcSpan,
    kind: TodoKind,
    message: Option<TypedExpr>,
    return_type: ValueType,
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
            generated_todo_expr(message, || PanicExpr::empty_function_at(site))?
        }
        TodoKind::EmptyBlock => generated_todo_expr(message, || PanicExpr::empty_block_at(site))?,
        TodoKind::IncompleteUse => {
            generated_todo_expr(message, || PanicExpr::incomplete_use_at(site))?
        }
    };

    Ok(panic_expr(panic, return_type))
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
    expression: impl FnOnce() -> PanicExpr,
) -> Result<PanicExpr, PlanError> {
    if message.is_some() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        });
    }

    Ok(expression())
}

pub(super) fn plan_expr_with_expected_source_stop_type(
    expression: TypedExpr,
    expected: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match expression {
        TypedExpr::Todo {
            location,
            kind,
            message,
            ..
        } => plan_todo_expr_with_type(
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
            plan_panic_expr_with_type(message.map(|message| *message), expected, site, context)
        }
        TypedExpr::Block { statements, .. } => {
            block::plan_with_expected_source_stop_type(statements, &expected, context)
        }
        expression => plan_expr(expression, context),
    }
}

fn panic_expr(panic: PanicExpr, return_type: ValueType) -> Expr {
    match return_type {
        ValueType::Int => Expr::int(IntExpr::panic(panic)),
        ValueType::String => Expr::string(StringExpr::panic(panic)),
        ValueType::BitArray => Expr::bit_array(BitArrayExpr::panic(panic)),
        ValueType::UtfCodepoint => Expr::utf_codepoint(crate::plan::UtfCodepointExpr::panic(panic)),
        ValueType::Custom(type_) => Expr::custom(CustomExpr::panic(panic, type_)),
        ValueType::Float => Expr::float(FloatExpr::panic(panic)),
        ValueType::Bool => Expr::bool(BoolExpr::panic(panic)),
        ValueType::Nil => Expr::nil(crate::plan::NilExpr::panic(panic)),
        ValueType::Tuple(type_) => Expr::tuple(TupleExpr::panic(panic, type_)),
        ValueType::List(type_) => Expr::list(ListExpr::panic(panic, *type_)),
        ValueType::Function(type_) => panic_function_expr(panic, *type_),
    }
}

fn panic_function_expr(panic: PanicExpr, type_: FunctionType) -> Expr {
    match type_.return_().clone() {
        ValueType::Int => Expr::function(FunctionExpr::int(crate::plan::IntFunctionExpr::panic(
            panic, type_,
        ))),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::panic(panic, type_),
        )),
        ValueType::BitArray => Expr::function(FunctionExpr::bit_array(
            crate::plan::BitArrayFunctionExpr::panic(panic, type_),
        )),
        ValueType::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint(
            crate::plan::UtfCodepointFunctionExpr::panic(panic, type_),
        )),
        ValueType::Custom(_) => Expr::function(FunctionExpr::custom(CustomFunctionExpr::panic(
            panic, type_,
        ))),
        ValueType::Float => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::panic(panic, type_),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::panic(panic, type_),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(crate::plan::NilFunctionExpr::panic(
            panic, type_,
        ))),
        ValueType::Tuple(_) => Expr::function(FunctionExpr::tuple(
            crate::plan::TupleFunctionExpr::panic(panic, type_),
        )),
        ValueType::List(item_type) => Expr::function(FunctionExpr::list(
            crate::plan::ListFunctionExpr::panic(panic, type_, *item_type),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::panic(panic, type_),
        )),
    }
}

fn plan_tuple(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let expected_type = match ValueType::from_gleam(type_.as_ref()) {
        Some(ValueType::Tuple(type_)) => type_,
        Some(actual) => {
            return Err(invalid_expression_type_for_value(
                ValueType::Tuple(vec![]),
                actual,
            ));
        }
        None => {
            return Err(invalid_expression_type(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Unsupported,
            ));
        }
    };

    if elements.len() != expected_type.len() {
        return Err(invalid_expression_type_for_value(
            ValueType::Tuple(expected_type),
            ValueType::Tuple(Vec::new()),
        ));
    }

    let planned_elements = elements
        .into_iter()
        .zip(&expected_type)
        .map(|(element, expected)| {
            plan_expr_with_expected_source_stop_type(element, expected.clone(), context)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let actual_type = planned_elements
        .iter()
        .map(Expr::value_type)
        .collect::<Vec<_>>();

    if expected_type != actual_type {
        return Err(invalid_expression_type_for_value(
            ValueType::Tuple(expected_type),
            ValueType::Tuple(actual_type),
        ));
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
        return match ValueType::from_gleam(type_.as_ref()) {
            Some(actual) => Err(invalid_expression_type_for_value(
                ValueType::List(Box::new(ValueType::Nil)),
                actual,
            )),
            None => Err(invalid_expression_type(
                InvalidExpressionType::List,
                InvalidExpressionType::Unsupported,
            )),
        };
    };

    let expected_element_type = match ValueType::from_gleam(list_element_type.as_ref()) {
        Some(type_) => type_,
        None => {
            return Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            });
        }
    };

    let planned_elements = elements
        .into_iter()
        .map(|element| {
            plan_expr_with_expected_source_stop_type(
                element,
                expected_element_type.clone(),
                context,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let Some(tail) = tail else {
        return Ok(Expr::list(
            ListExpr::try_value(planned_elements, expected_element_type)
                .map_err(|error| invalid_expression_type_for_value(error.expected, error.actual))?,
        ));
    };

    let tail = plan_expr_with_expected_source_stop_type(
        tail,
        ValueType::List(Box::new(expected_element_type.clone())),
        context,
    )?;
    let actual = tail.value_type();
    let tail = tail.into_list().ok_or_else(|| {
        invalid_expression_type_for_value(
            ValueType::List(Box::new(expected_element_type.clone())),
            actual,
        )
    })?;

    let elements =
        crate::plan::ListElements::from_exprs(expected_element_type.clone(), planned_elements)
            .map_err(|error| invalid_expression_type_for_value(error.expected, error.actual))?;
    let elements =
        crate::plan::ListSpreadElements::from_parts(elements, tail).map_err(|error| {
            invalid_expression_type_for_value(
                ValueType::List(Box::new(error.expected)),
                ValueType::List(Box::new(error.actual)),
            )
        })?;
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
    let tuple = tuple
        .into_tuple()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Tuple, actual))?;
    let expected = ValueType::from_gleam(type_.as_ref()).ok_or_else(|| {
        invalid_expression_type(
            InvalidExpressionType::Unsupported,
            InvalidExpressionType::Tuple,
        )
    })?;
    let actual = tuple.type_().get(index).cloned().ok_or_else(|| {
        invalid_expression_type_for_value(expected.clone(), ValueType::Tuple(vec![]))
    })?;
    if actual != expected {
        return Err(invalid_expression_type_for_value(expected.clone(), actual));
    }

    Ok(tuple_index_expr(tuple, index, expected))
}

pub(super) fn tuple_index_expr(tuple: TupleExpr, index: usize, return_type: ValueType) -> Expr {
    match return_type {
        ValueType::Int => Expr::int(IntExpr::tuple_index(tuple, index)),
        ValueType::String => Expr::string(StringExpr::tuple_index(tuple, index)),
        ValueType::BitArray => Expr::bit_array(BitArrayExpr::tuple_index(tuple, index)),
        ValueType::UtfCodepoint => {
            Expr::utf_codepoint(crate::plan::UtfCodepointExpr::tuple_index(tuple, index))
        }
        ValueType::Custom(type_) => Expr::custom(CustomExpr::tuple_index(tuple, index, type_)),
        ValueType::Float => Expr::float(FloatExpr::tuple_index(tuple, index)),
        ValueType::Bool => Expr::bool(BoolExpr::tuple_index(tuple, index)),
        ValueType::Nil => Expr::nil(crate::plan::NilExpr::tuple_index(tuple, index)),
        ValueType::Tuple(type_) => Expr::tuple(TupleExpr::tuple_index(tuple, index, type_)),
        ValueType::List(type_) => Expr::list(ListExpr::tuple_index(tuple, index, *type_)),
        ValueType::Function(type_) => tuple_index_function_expr(tuple, index, *type_),
    }
}

pub(super) fn list_index_expr(
    list: ListExpr,
    index: usize,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    let expected = ValueType::List(Box::new(return_type.clone()));
    let actual = list.element_type();
    Ok(match (return_type, list) {
        (ValueType::Int, ListExpr::Int(list)) => Expr::int(IntExpr::list_index(list, index)),
        (ValueType::String, ListExpr::String(list)) => {
            Expr::string(StringExpr::list_index(list, index))
        }
        (ValueType::BitArray, ListExpr::BitArray(list)) => {
            Expr::bit_array(BitArrayExpr::list_index(list, index))
        }
        (ValueType::UtfCodepoint, ListExpr::UtfCodepoint(list)) => {
            Expr::utf_codepoint(crate::plan::UtfCodepointExpr::list_index(list, index))
        }
        (ValueType::Custom(type_), ListExpr::Custom(list)) if list.item().item_type() == type_ => {
            Expr::custom(CustomExpr::list_index(list, index, type_))
        }
        (ValueType::Float, ListExpr::Float(list)) => {
            Expr::float(FloatExpr::list_index(list, index))
        }
        (ValueType::Bool, ListExpr::Bool(list)) => Expr::bool(BoolExpr::list_index(list, index)),
        (ValueType::Nil, ListExpr::Nil(list)) => {
            Expr::nil(crate::plan::NilExpr::list_index(list, index))
        }
        (ValueType::Tuple(type_), ListExpr::Tuple(list)) if list.item().item_type() == type_ => {
            Expr::tuple(TupleExpr::list_index(list, index, type_))
        }
        (ValueType::List(type_), ListExpr::List(list)) if list.item().item_type() == type_ => {
            Expr::list(ListExpr::list_index(list, index))
        }
        (ValueType::Function(type_), ListExpr::Function(list))
            if list.item().item_type() == *type_.as_ref() =>
        {
            list_index_function_expr(list, index, *type_)
        }
        _ => return Err(invalid_expression_type_for_value(expected, actual)),
    })
}

fn tuple_index_function_expr(tuple: TupleExpr, index: usize, type_: FunctionType) -> Expr {
    match type_.return_().clone() {
        ValueType::Int => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::BitArray => Expr::function(FunctionExpr::bit_array(
            crate::plan::BitArrayFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint(
            crate::plan::UtfCodepointFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Custom(_) => Expr::function(FunctionExpr::custom(
            CustomFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Float => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Tuple(_) => Expr::function(FunctionExpr::tuple(
            crate::plan::TupleFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::List(item_type) => Expr::function(FunctionExpr::list(
            crate::plan::ListFunctionExpr::tuple_index(tuple, index, type_, *item_type),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::tuple_index(tuple, index, type_),
        )),
    }
}

fn list_index_function_expr(
    list: crate::plan::FunctionListExpr,
    index: usize,
    type_: FunctionType,
) -> Expr {
    match type_.return_().clone() {
        ValueType::Int => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::BitArray => Expr::function(FunctionExpr::bit_array(
            crate::plan::BitArrayFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint(
            crate::plan::UtfCodepointFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::Custom(_) => Expr::function(FunctionExpr::custom(
            CustomFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::Float => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::Tuple(_) => Expr::function(FunctionExpr::tuple(
            crate::plan::TupleFunctionExpr::list_index(list.clone(), index, type_),
        )),
        ValueType::List(item_type) => Expr::function(FunctionExpr::list(
            crate::plan::ListFunctionExpr::list_index(list.clone(), index, type_, *item_type),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::list_index(list, index, type_),
        )),
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
    expression
        .into_int()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Int, actual))
}

pub(super) fn plan_string_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::String, context)?;
    let actual = expression_type(&expression);
    expression
        .into_string()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::String, actual))
}

fn plan_float_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<FloatExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::Float, context)?;
    let actual = expression_type(&expression);
    expression
        .into_float()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Float, actual))
}

pub(super) fn plan_bool_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<BoolExpr, PlanError> {
    let expression =
        plan_expr_with_expected_source_stop_type(expression, ValueType::Bool, context)?;
    let actual = expression_type(&expression);
    expression
        .into_bool()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Bool, actual))
}

fn invalid_expression_type(
    expected: InvalidExpressionType,
    actual: InvalidExpressionType,
) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType { expected, actual },
    }
}

fn invalid_expression_type_for_value(expected: ValueType, actual: ValueType) -> PlanError {
    invalid_expression_type(
        value_type_expression_type(expected),
        value_type_expression_type(actual),
    )
}

fn expression_type(expression: &Expr) -> InvalidExpressionType {
    match expression.value_type() {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::BitArray => InvalidExpressionType::BitArray,
        ValueType::UtfCodepoint => InvalidExpressionType::UtfCodepoint,
        ValueType::Custom(_) => InvalidExpressionType::Custom,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::List(_) => InvalidExpressionType::List,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

fn value_type_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::BitArray => InvalidExpressionType::BitArray,
        ValueType::UtfCodepoint => InvalidExpressionType::UtfCodepoint,
        ValueType::Custom(_) => InvalidExpressionType::Custom,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::List(_) => InvalidExpressionType::List,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
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
mod tests {
    use super::{
        expression_type, invalid_expression_type, invalid_expression_type_for_value,
        list_index_function_expr, module_returning_typed_expr, typed_int_expr, typed_string_expr,
        typed_tuple_expr,
    };
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionId, BoolLocalId, CustomExpr,
        CustomLocalId, CustomType, CustomTypeName, Expr, FloatExpr, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionId, FunctionReference, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId, ListExpr, NilExpr,
        NilFunctionId, NilLocalId, PanicExpr, PanicSite, ParamLocal, ReturnBody, RuntimeFunctionId,
        SourceSpan, StringExpr, StringLocalId, TupleExpr, UtfCodepointExpr, UtfCodepointLocalId,
        ValueType,
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
    use gleam_core::ast::{Constant, TypedExpr};
    use gleam_core::type_::{self, ModuleValueConstructor};
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
            &crate::plan::ReturnExpr::int_body(
                IntFunctionId(0),
                ReturnBody::block(
                    Vec::new(),
                    ReturnBody::expr(IntExpr::panic(PanicExpr::empty_block_at(PanicSite::new(
                        "main".into(),
                        "main".into(),
                        SourceSpan::new(26, 28)
                    ),))),
                ),
            ),
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
        let function_function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
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
        assert_eq!(
            super::plan_expr(
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::generic_var(0),
                    message: None,
                },
                &mut context
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Unsupported,
                    actual: InvalidExpressionType::Unsupported,
                },
            }),
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
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Unsupported,
                    actual: InvalidExpressionType::Unsupported,
                },
            }),
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
        let cases = [
            (
                r#"
pub fn main() {
  let values = [Ok(1)]
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::UnsupportedListElementType,
                },
            ),
            (
                r#"pub fn main() { echo 1 }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Echo,
                },
            ),
            (
                r#"
pub type Boxed {
  Boxed(value: Int)
}

pub fn main() {
  Boxed(1).value
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::RecordAccess,
                },
            ),
            (
                r#"
pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::RecordUpdate,
                },
            ),
            (
                r#"
pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  case person {
    Person(..) if person.age > 0 -> 1
    _ -> 0
  }
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::RecordAccess,
                },
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
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
    fn reject_margin_function_expression_type() {
        let expression = Expr::function(FunctionExpr::reference(FunctionReference::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            Vec::new(),
        )));
        let list_expression = Expr::from(list([int(1)], ValueType::Int));
        let nil_expression = Expr::from(nil());
        let bit_array_expression = Expr::bit_array(BitArrayExpr::value(Vec::new()));
        let utf_codepoint_expression = Expr::utf_codepoint(UtfCodepointExpr::local_get(
            UtfCodepointLocalId(0),
            "codepoint".into(),
        ));
        let custom_expression = Expr::custom(CustomExpr::local_get(
            CustomLocalId(0),
            "boxed".into(),
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                Vec::new(),
            ),
        ));

        assert_eq!(
            invalid_expression_type(InvalidExpressionType::Int, expression_type(&expression)),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Function,
                },
            },
        );
        assert_eq!(
            invalid_expression_type(
                InvalidExpressionType::Int,
                expression_type(&list_expression),
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::List,
                },
            },
        );
        assert_eq!(
            invalid_expression_type(InvalidExpressionType::Int, expression_type(&nil_expression)),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Nil,
                },
            },
        );
        assert_eq!(
            invalid_expression_type(
                InvalidExpressionType::Int,
                expression_type(&bit_array_expression),
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::BitArray,
                },
            },
        );
        assert_eq!(
            invalid_expression_type_for_value(ValueType::Float, ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::Int,
                },
            },
        );
        assert_eq!(
            invalid_expression_type_for_value(ValueType::BitArray, ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::BitArray,
                    actual: InvalidExpressionType::Int,
                },
            },
        );
        assert_eq!(
            invalid_expression_type(
                InvalidExpressionType::Int,
                expression_type(&utf_codepoint_expression),
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::UtfCodepoint,
                },
            },
        );
        assert_eq!(
            invalid_expression_type_for_value(ValueType::UtfCodepoint, ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::UtfCodepoint,
                    actual: InvalidExpressionType::Int,
                },
            },
        );
        assert_eq!(
            invalid_expression_type(
                InvalidExpressionType::Int,
                expression_type(&custom_expression),
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Custom,
                },
            },
        );
        assert_eq!(
            invalid_expression_type_for_value(
                ValueType::List(Box::new(ValueType::Nil)),
                ValueType::Nil,
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Nil,
                },
            },
        );
    }

    #[test]
    fn list_index_function_expr_preserves_bit_array_return_family() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let list = ListExpr::value(Vec::new(), ValueType::Function(Box::new(type_.clone())))
            .into_function()
            .expect("function list");

        assert_eq!(
            list_index_function_expr(list.clone(), 2, type_.clone()),
            Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::list_index(
                list, 2, type_,
            ))),
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
                        Expr::from(int_function_ref(0, [ParamLocal::int(IntLocalId(0))])),
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
                        0,
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
                        0,
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
                        0,
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
                        0,
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
                        0,
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
                        0,
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
                        0,
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
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        int_to_int_type(),
                    ))]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                    function("get", int_function_ref(0, [ParamLocal::int(IntLocalId(0))])),
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
                        actual: InvalidExpressionType::Unsupported,
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
                        expected: InvalidExpressionType::Unsupported,
                        actual: InvalidExpressionType::Tuple,
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
                        actual: InvalidExpressionType::Unsupported,
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
