mod bit_array;
mod bool;
mod custom;
mod custom_field;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

use super::id::list_function_local;
use crate::plan::module;

pub(super) use bool::bool_expr;
pub(super) use custom::custom_expr;
pub(super) use custom_field::custom_field_access;
pub(super) use float::float_expr;
pub(super) use function::{
    bit_array_function_expr, bool_function_expr, custom_function_expr, float_function_expr,
    function_expr, function_function_expr, int_function_expr, list_function_expr,
    nil_function_expr, string_function_expr, tuple_function_expr, utf_codepoint_function_expr,
};
pub(super) use int::int_expr;
pub(super) use list::{
    bit_array_list_expr, bool_list_expr, custom_list_expr, float_list_expr, function_list_expr,
    int_list_expr, list_expr, list_list_expr, list_local_expr, nil_list_expr, string_list_expr,
    tuple_list_expr, utf_codepoint_list_expr,
};
pub(super) use nil::nil_expr;
pub(super) use string::string_expr;
pub(super) use tuple::tuple_expr;
pub(super) use utf_codepoint::utf_codepoint_expr;

use super::super as execution;

pub(super) fn expr(
    expression: module::Expr,
    context: &mut super::LoweringContext,
) -> execution::Expr {
    execution::Expr::from_kind(match expression.into_kind() {
        module::ExprKind::Int(expression) => {
            execution::ExprKind::Int(int_expr(expression, context))
        }
        module::ExprKind::String(expression) => {
            execution::ExprKind::String(string_expr(expression, context))
        }
        module::ExprKind::BitArray(expression) => {
            execution::ExprKind::BitArray(bit_array_expr(expression, context))
        }
        module::ExprKind::UtfCodepoint(expression) => {
            execution::ExprKind::UtfCodepoint(utf_codepoint_expr(expression, context))
        }
        module::ExprKind::Custom(expression) => {
            execution::ExprKind::Custom(custom_expr(expression, context))
        }
        module::ExprKind::Float(expression) => {
            execution::ExprKind::Float(float_expr(expression, context))
        }
        module::ExprKind::Bool(expression) => {
            execution::ExprKind::Bool(bool_expr(expression, context))
        }
        module::ExprKind::Nil(expression) => {
            execution::ExprKind::Nil(nil_expr(expression, context))
        }
        module::ExprKind::Tuple(expression) => {
            execution::ExprKind::Tuple(tuple_expr(expression, context))
        }
        module::ExprKind::List(expression) => {
            execution::ExprKind::List(list_expr(expression, context))
        }
        module::ExprKind::Function(expression) => {
            execution::ExprKind::Function(function_expr(expression, context))
        }
    })
}

pub(super) fn panic_expr(
    expression: module::PanicExpr,
    context: &mut super::LoweringContext,
) -> execution::PanicExpr {
    let (site, kind) = expression.into_parts();
    let kind = match kind {
        module::PanicExprKind::Panic { message } => execution::PanicExprKind::Panic {
            message: message.map(|message| Box::new(string_expr(*message, context))),
        },
        module::PanicExprKind::Todo { message } => execution::PanicExprKind::Todo {
            message: message.map(|message| Box::new(string_expr(*message, context))),
        },
        module::PanicExprKind::EmptyFunction => execution::PanicExprKind::EmptyFunction,
        module::PanicExprKind::EmptyBlock => execution::PanicExprKind::EmptyBlock,
        module::PanicExprKind::IncompleteUse => execution::PanicExprKind::IncompleteUse,
    };
    execution::PanicExpr::from_parts(site, kind)
}

pub(super) fn call_args(
    args: Vec<module::CallArg>,
    context: &mut super::LoweringContext,
) -> Vec<execution::CallArg> {
    args.into_iter().map(|arg| call_arg(arg, context)).collect()
}

pub(super) fn call_arg(
    arg: module::CallArg,
    context: &mut super::LoweringContext,
) -> execution::CallArg {
    use execution::CallArgKind as E;
    use module::CallArgKind as M;

    execution::CallArg::from_kind(match arg.into_kind() {
        M::Int { local, value } => E::Int {
            local: execution::IntLocalId(local.0),
            value: int_expr(value, context),
        },
        M::String { local, value } => E::String {
            local: execution::StringLocalId(local.0),
            value: string_expr(value, context),
        },
        M::BitArray { local, value } => E::BitArray {
            local: execution::BitArrayLocalId(local.0),
            value: bit_array_expr(value, context),
        },
        M::UtfCodepoint { local, value } => E::UtfCodepoint {
            local: execution::UtfCodepointLocalId(local.0),
            value: utf_codepoint_expr(value, context),
        },
        M::Custom { local, value } => E::Custom {
            local: execution::CustomLocalId(local.0),
            value: custom_expr(value, context),
        },
        M::Float { local, value } => E::Float {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value, context),
        },
        M::Bool { local, value } => E::Bool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value, context),
        },
        M::Nil { local, value } => E::Nil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value, context),
        },
        M::Tuple { local, value } => E::Tuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value, context),
        },
        M::List(value) => E::List(list::list_local_expr(value, context)),
        M::IntFunction { local, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value, context),
        },
        M::StringFunction { local, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value, context),
        },
        M::BitArrayFunction { local, value } => E::BitArrayFunction {
            local: execution::BitArrayFunctionLocalId(local.0),
            value: bit_array_function_expr(value, context),
        },
        M::UtfCodepointFunction { local, value } => E::UtfCodepointFunction {
            local: execution::UtfCodepointFunctionLocalId(local.0),
            value: utf_codepoint_function_expr(value, context),
        },
        M::CustomFunction { local, value } => E::CustomFunction {
            local: execution::CustomFunctionLocalId(local.0),
            value: custom_function_expr(value, context),
        },
        M::FloatFunction { local, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: float_function_expr(value, context),
        },
        M::BoolFunction { local, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: bool_function_expr(value, context),
        },
        M::NilFunction { local, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value, context),
        },
        M::TupleFunction { local, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value, context),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local(local, context),
            value: list_function_expr(value, context),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value, context),
        },
    })
}

pub(super) fn capture_args(
    args: Vec<module::CaptureArg>,
    context: &mut super::LoweringContext,
) -> Vec<execution::CaptureArg> {
    args.into_iter()
        .map(|arg| capture_arg(arg, context))
        .collect()
}

fn capture_arg(
    arg: module::CaptureArg,
    context: &mut super::LoweringContext,
) -> execution::CaptureArg {
    use execution::CaptureArgKind as E;
    use module::CaptureArgKind as M;

    execution::CaptureArg::from_kind(match arg.into_kind() {
        M::Int { local, value } => E::Int {
            local: execution::IntLocalId(local.0),
            value: int_expr(value, context),
        },
        M::String { local, value } => E::String {
            local: execution::StringLocalId(local.0),
            value: string_expr(value, context),
        },
        M::BitArray { local, value } => E::BitArray {
            local: execution::BitArrayLocalId(local.0),
            value: bit_array_expr(value, context),
        },
        M::UtfCodepoint { local, value } => E::UtfCodepoint {
            local: execution::UtfCodepointLocalId(local.0),
            value: utf_codepoint_expr(value, context),
        },
        M::Custom { local, value } => E::Custom {
            local: execution::CustomLocalId(local.0),
            value: custom_expr(value, context),
        },
        M::Float { local, value } => E::Float {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value, context),
        },
        M::Bool { local, value } => E::Bool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value, context),
        },
        M::Nil { local, value } => E::Nil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value, context),
        },
        M::Tuple { local, value } => E::Tuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value, context),
        },
        M::List(value) => E::List(list::list_local_expr(value, context)),
        M::IntFunction { local, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value, context),
        },
        M::StringFunction { local, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value, context),
        },
        M::BitArrayFunction { local, value } => E::BitArrayFunction {
            local: execution::BitArrayFunctionLocalId(local.0),
            value: bit_array_function_expr(value, context),
        },
        M::UtfCodepointFunction { local, value } => E::UtfCodepointFunction {
            local: execution::UtfCodepointFunctionLocalId(local.0),
            value: utf_codepoint_function_expr(value, context),
        },
        M::CustomFunction { local, value } => E::CustomFunction {
            local: execution::CustomFunctionLocalId(local.0),
            value: custom_function_expr(value, context),
        },
        M::FloatFunction { local, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: float_function_expr(value, context),
        },
        M::BoolFunction { local, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: bool_function_expr(value, context),
        },
        M::NilFunction { local, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value, context),
        },
        M::TupleFunction { local, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value, context),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local(local, context),
            value: list_function_expr(value, context),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value, context),
        },
    })
}
pub(super) use bit_array::bit_array_expr;
