mod bit_array;
mod bool;
mod custom;
mod custom_field;
mod float;
mod function;
mod generic;
mod int;
mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

use super::id::list_function_local_at_target;
use crate::plan::module;

pub(super) use bool::bool_expr;
pub(super) use custom::{custom_expr, custom_expr_kind};
pub(super) use custom_field::custom_field_access;
pub(super) use float::float_expr;
pub(super) use function::{
    bit_array_function_expr, bool_function_expr, custom_function_expr, custom_function_expr_kind,
    float_function_expr, function_expr, function_function_expr, function_function_expr_kind,
    generic_bit_array_function_expr, generic_bool_function_expr, generic_custom_function_expr,
    generic_custom_function_expr_kind, generic_float_function_expr, generic_function_function_expr,
    generic_function_function_expr_kind, generic_int_function_expr, generic_list_function_expr,
    generic_nil_function_expr, generic_string_function_expr, generic_tuple_function_expr,
    generic_utf_codepoint_function_expr, int_function_expr, list_function_expr, nil_function_expr,
    string_function_expr, tuple_function_expr, typed_function_expr, utf_codepoint_function_expr,
};
pub(super) use generic::{
    generic_bit_array_expr, generic_bool_expr, generic_custom_expr, generic_custom_expr_kind,
    generic_expr, generic_float_expr, generic_function_value_expr, generic_int_expr,
    generic_list_value_expr, generic_nil_expr, generic_string_expr, generic_tuple_expr,
    generic_utf_codepoint_expr, generic_value_bit_array_function_expr,
    generic_value_bit_array_list_expr, generic_value_bool_function_expr,
    generic_value_bool_list_expr, generic_value_custom_function_expr_kind,
    generic_value_custom_list_expr, generic_value_float_function_expr,
    generic_value_float_list_expr, generic_value_function_function_expr_kind,
    generic_value_function_list_expr, generic_value_int_function_expr, generic_value_int_list_expr,
    generic_value_list_function_expr, generic_value_nested_list_expr,
    generic_value_nil_function_expr, generic_value_nil_list_expr,
    generic_value_string_function_expr, generic_value_string_list_expr,
    generic_value_tuple_function_expr, generic_value_tuple_list_expr,
    generic_value_utf_codepoint_function_expr, generic_value_utf_codepoint_list_expr,
};
pub(super) use int::int_expr;
pub(super) use list::{
    bit_array_list_expr, bool_list_expr, custom_list_expr, float_list_expr, function_list_expr,
    generic_bit_array_list_expr, generic_bool_list_expr, generic_custom_list_expr,
    generic_float_list_expr, generic_function_list_expr, generic_int_list_expr,
    generic_nested_list_expr, generic_nil_list_expr, generic_string_list_expr,
    generic_tuple_list_expr, generic_utf_codepoint_list_expr, int_list_expr, list_expr,
    list_list_expr, list_local_expr, nil_list_expr, string_list_expr, tuple_list_expr,
    utf_codepoint_list_expr,
};
pub(super) use nil::nil_expr;
pub(super) use string::string_expr;
pub(super) use tuple::tuple_expr;
pub(super) use utf_codepoint::utf_codepoint_expr;

use super::super as execution;

enum SpecializedValueBinding {
    Int {
        local: execution::IntLocalId,
        value: execution::IntExpr,
    },
    Float {
        local: execution::FloatLocalId,
        value: execution::FloatExpr,
    },
    String {
        local: execution::StringLocalId,
        value: execution::StringExpr,
    },
    BitArray {
        local: execution::BitArrayLocalId,
        value: execution::BitArrayExpr,
    },
    UtfCodepoint {
        local: execution::UtfCodepointLocalId,
        value: execution::UtfCodepointExpr,
    },
    Custom(execution::CustomLocalExpr),
    Bool {
        local: execution::BoolLocalId,
        value: execution::BoolExpr,
    },
    Nil {
        local: execution::NilLocalId,
        value: execution::NilExpr,
    },
    Tuple {
        local: execution::TupleLocalId,
        value: execution::TupleExpr,
    },
    List(execution::ListLocalExpr),
    Function(Box<function::SpecializedFunctionBinding>),
}

fn specialized_value_binding_for_shape(
    index: usize,
    value: &module::Expr,
    shape: &super::specialization::ConcreteValueShape,
    context: &mut super::LoweringContext,
) -> SpecializedValueBinding {
    match value.kind() {
        module::ExprKind::Generic(value) => {
            specialized_generic_value_binding_for_shape(index, value, shape, context)
        }
        module::ExprKind::Int(value) => SpecializedValueBinding::Int {
            local: execution::IntLocalId(index),
            value: int_expr(value, context),
        },
        module::ExprKind::Float(value) => SpecializedValueBinding::Float {
            local: execution::FloatLocalId(index),
            value: float_expr(value, context),
        },
        module::ExprKind::String(value) => SpecializedValueBinding::String {
            local: execution::StringLocalId(index),
            value: string_expr(value, context),
        },
        module::ExprKind::BitArray(value) => SpecializedValueBinding::BitArray {
            local: execution::BitArrayLocalId(index),
            value: bit_array_expr(value, context),
        },
        module::ExprKind::UtfCodepoint(value) => SpecializedValueBinding::UtfCodepoint {
            local: execution::UtfCodepointLocalId(index),
            value: utf_codepoint_expr(value, context),
        },
        module::ExprKind::Custom(value) => {
            SpecializedValueBinding::Custom(execution::CustomLocalExpr::new(
                execution::CustomLocal::new(
                    execution::CustomLocalId(index),
                    context.custom_value_shape(value.shape().clone()),
                ),
                custom_expr(value, context),
            ))
        }
        module::ExprKind::Bool(value) => SpecializedValueBinding::Bool {
            local: execution::BoolLocalId(index),
            value: bool_expr(value, context),
        },
        module::ExprKind::Nil(value) => SpecializedValueBinding::Nil {
            local: execution::NilLocalId(index),
            value: nil_expr(value, context),
        },
        module::ExprKind::Tuple(value) => SpecializedValueBinding::Tuple {
            local: execution::TupleLocalId(index),
            value: tuple_expr(value, context),
        },
        module::ExprKind::List(value) => SpecializedValueBinding::List(
            list::specialized_list_local_expr(index, list_expr(value, context)),
        ),
        module::ExprKind::Function(value) => SpecializedValueBinding::Function(Box::new(
            function::specialized_function_binding(index, value, context),
        )),
    }
}

fn specialized_generic_value_binding(
    index: usize,
    value: &module::GenericExpr,
    context: &mut super::LoweringContext,
) -> SpecializedValueBinding {
    let shape = context.concrete_parameter(value.parameter());
    specialized_generic_value_binding_for_shape(index, value, &shape, context)
}

fn specialized_generic_value_binding_for_shape(
    index: usize,
    value: &module::GenericExpr,
    shape: &super::specialization::ConcreteValueShape,
    context: &mut super::LoweringContext,
) -> SpecializedValueBinding {
    use super::specialization::ConcreteValueShape as S;

    match shape {
        S::Int => SpecializedValueBinding::Int {
            local: execution::IntLocalId(index),
            value: generic_int_expr(value, context),
        },
        S::Float => SpecializedValueBinding::Float {
            local: execution::FloatLocalId(index),
            value: generic_float_expr(value, context),
        },
        S::String => SpecializedValueBinding::String {
            local: execution::StringLocalId(index),
            value: generic_string_expr(value, context),
        },
        S::BitArray => SpecializedValueBinding::BitArray {
            local: execution::BitArrayLocalId(index),
            value: generic_bit_array_expr(value, context),
        },
        S::UtfCodepoint => SpecializedValueBinding::UtfCodepoint {
            local: execution::UtfCodepointLocalId(index),
            value: generic_utf_codepoint_expr(value, context),
        },
        S::Custom(shape) => SpecializedValueBinding::Custom(execution::CustomLocalExpr::new(
            execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(shape),
            ),
            generic_custom_expr(value, shape, context),
        )),
        S::Bool => SpecializedValueBinding::Bool {
            local: execution::BoolLocalId(index),
            value: generic_bool_expr(value, context),
        },
        S::Nil => SpecializedValueBinding::Nil {
            local: execution::NilLocalId(index),
            value: generic_nil_expr(value, context),
        },
        S::Tuple(elements) => SpecializedValueBinding::Tuple {
            local: execution::TupleLocalId(index),
            value: generic_tuple_expr(value, elements, context),
        },
        S::List(item) => SpecializedValueBinding::List(list::specialized_list_local_expr(
            index,
            generic_list_value_expr(value, item, context),
        )),
        S::Function(shape) => SpecializedValueBinding::Function(Box::new(
            generic::generic_function_value_binding(index, value, shape, context),
        )),
    }
}

fn specialized_call_arg(binding: SpecializedValueBinding) -> execution::CallArg {
    use execution::CallArgKind as E;

    execution::CallArg::from_kind(match binding {
        SpecializedValueBinding::Int { local, value } => E::Int { local, value },
        SpecializedValueBinding::Float { local, value } => E::Float { local, value },
        SpecializedValueBinding::String { local, value } => E::String { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::BitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => E::UtfCodepoint { local, value },
        SpecializedValueBinding::Custom(value) => E::Custom(value),
        SpecializedValueBinding::Bool { local, value } => E::Bool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::Nil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::Tuple { local, value },
        SpecializedValueBinding::List(value) => E::List(value),
        SpecializedValueBinding::Function(value) => specialized_function_call_arg(*value),
    })
}

fn specialized_function_call_arg(
    binding: function::SpecializedFunctionBinding,
) -> execution::CallArgKind {
    use execution::CallArgKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::IntFunction { local, value },
        B::Float { local, value } => E::FloatFunction { local, value },
        B::String { local, value } => E::StringFunction { local, value },
        B::BitArray { local, value } => E::BitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::UtfCodepointFunction { local, value },
        B::Custom { local, value } => E::CustomFunction { local, value },
        B::Bool { local, value } => E::BoolFunction { local, value },
        B::Nil { local, value } => E::NilFunction { local, value },
        B::Tuple { local, value } => E::TupleFunction { local, value },
        B::List { local, value } => E::ListFunction { local, value },
        B::Function { local, value } => E::FunctionFunction { local, value },
    }
}

fn specialized_capture_arg(binding: SpecializedValueBinding) -> execution::CaptureArg {
    use execution::CaptureArgKind as E;

    execution::CaptureArg::from_kind(match binding {
        SpecializedValueBinding::Int { local, value } => E::Int { local, value },
        SpecializedValueBinding::Float { local, value } => E::Float { local, value },
        SpecializedValueBinding::String { local, value } => E::String { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::BitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => E::UtfCodepoint { local, value },
        SpecializedValueBinding::Custom(value) => E::Custom(value),
        SpecializedValueBinding::Bool { local, value } => E::Bool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::Nil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::Tuple { local, value },
        SpecializedValueBinding::List(value) => E::List(value),
        SpecializedValueBinding::Function(value) => specialized_function_capture_arg(*value),
    })
}

fn specialized_function_capture_arg(
    binding: function::SpecializedFunctionBinding,
) -> execution::CaptureArgKind {
    use execution::CaptureArgKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::IntFunction { local, value },
        B::Float { local, value } => E::FloatFunction { local, value },
        B::String { local, value } => E::StringFunction { local, value },
        B::BitArray { local, value } => E::BitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::UtfCodepointFunction { local, value },
        B::Custom { local, value } => E::CustomFunction { local, value },
        B::Bool { local, value } => E::BoolFunction { local, value },
        B::Nil { local, value } => E::NilFunction { local, value },
        B::Tuple { local, value } => E::TupleFunction { local, value },
        B::List { local, value } => E::ListFunction { local, value },
        B::Function { local, value } => E::FunctionFunction { local, value },
    }
}

pub(super) fn generic_step(
    local: crate::plan::GenericLocal,
    value: &module::GenericExpr,
    context: &mut super::LoweringContext,
) -> execution::StepKind {
    specialized_step(specialized_generic_value_binding(
        context.generic_local_index(local.id()),
        value,
        context,
    ))
}

pub(super) fn generic_function_step(
    local: &crate::plan::GenericFunctionLocal,
    value: &module::TypedFunctionExpr<module::GenericFunctionExpr>,
    context: &mut super::LoweringContext,
) -> execution::StepKind {
    specialized_function_step(function::specialized_typed_generic_function_binding(
        context.generic_function_local_index(local.id()),
        value,
        context,
    ))
}

fn specialized_step(binding: SpecializedValueBinding) -> execution::StepKind {
    use execution::StepKind as E;

    match binding {
        SpecializedValueBinding::Int { local, value } => E::LetInt { local, value },
        SpecializedValueBinding::Float { local, value } => E::LetFloat { local, value },
        SpecializedValueBinding::String { local, value } => E::LetString { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::LetBitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => {
            E::LetUtfCodepoint { local, value }
        }
        SpecializedValueBinding::Custom(value) => E::LetCustom(value),
        SpecializedValueBinding::Bool { local, value } => E::LetBool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::LetNil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::LetTuple { local, value },
        SpecializedValueBinding::List(value) => E::LetList { value },
        SpecializedValueBinding::Function(value) => specialized_function_step(*value),
    }
}

fn specialized_function_step(binding: function::SpecializedFunctionBinding) -> execution::StepKind {
    use execution::StepKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::LetIntFunction { local, value },
        B::Float { local, value } => E::LetFloatFunction { local, value },
        B::String { local, value } => E::LetStringFunction { local, value },
        B::BitArray { local, value } => E::LetBitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::LetUtfCodepointFunction { local, value },
        B::Custom { local, value } => E::LetCustomFunction { local, value },
        B::Bool { local, value } => E::LetBoolFunction { local, value },
        B::Nil { local, value } => E::LetNilFunction { local, value },
        B::Tuple { local, value } => E::LetTupleFunction { local, value },
        B::List { local, value } => E::LetListFunction { local, value },
        B::Function { local, value } => E::LetFunctionFunction { local, value },
    }
}

pub(super) fn expr(
    expression: &module::Expr,
    context: &mut super::LoweringContext,
) -> execution::Expr {
    execution::Expr::from_kind(match expression.kind() {
        module::ExprKind::Generic(expression) => return generic_expr(expression, context),
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
    expression: &module::PanicExpr,
    context: &mut super::LoweringContext,
) -> execution::PanicExpr {
    let kind = match expression.kind() {
        module::PanicExprKind::Panic { message } => execution::PanicExprKind::Panic {
            message: message
                .as_deref()
                .map(|message| Box::new(string_expr(message, context))),
        },
        module::PanicExprKind::Todo { message } => execution::PanicExprKind::Todo {
            message: message
                .as_deref()
                .map(|message| Box::new(string_expr(message, context))),
        },
        module::PanicExprKind::EmptyFunction => execution::PanicExprKind::EmptyFunction,
        module::PanicExprKind::EmptyBlock => execution::PanicExprKind::EmptyBlock,
        module::PanicExprKind::IncompleteUse => execution::PanicExprKind::IncompleteUse,
    };
    execution::PanicExpr::from_parts(expression.site(), kind)
}

pub(super) fn call_args(
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
) -> Vec<execution::CallArg> {
    let mut prefix = super::frame::ParameterPrefix::default();
    args.iter()
        .map(|arg| {
            let shape = context.concrete_value_shape(&arg.parameter_shape());
            let index = prefix.allocate(&shape);
            let target = context.current_target(index, shape);
            call_arg_at(&target, arg, context)
        })
        .collect()
}

pub(super) fn direct_call_args(
    function: &module::FunctionInstantiation,
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
) -> Vec<execution::CallArg> {
    args.iter()
        .map(|arg| direct_call_arg(function, arg, context))
        .collect()
}

fn direct_call_arg(
    function: &module::FunctionInstantiation,
    arg: &module::CallArg,
    context: &mut super::LoweringContext,
) -> execution::CallArg {
    let target = context.target_local(function, call_arg_local_key(arg));
    call_arg_at(&target, arg, context)
}

fn call_arg_at(
    target: &super::TargetLocal,
    arg: &module::CallArg,
    context: &mut super::LoweringContext,
) -> execution::CallArg {
    use execution::CallArgKind as E;
    use module::CallArgKind as M;

    let index = target.index();

    execution::CallArg::from_kind(match arg.kind() {
        M::Parametric { slot, value } => {
            let _ = slot;
            return specialized_call_arg(specialized_value_binding_for_shape(
                index,
                value,
                target.shape(),
                context,
            ));
        }
        M::Int { local: _, value } => E::Int {
            local: execution::IntLocalId(index),
            value: int_expr(value, context),
        },
        M::String { local: _, value } => E::String {
            local: execution::StringLocalId(index),
            value: string_expr(value, context),
        },
        M::BitArray { local: _, value } => E::BitArray {
            local: execution::BitArrayLocalId(index),
            value: bit_array_expr(value, context),
        },
        M::UtfCodepoint { local: _, value } => E::UtfCodepoint {
            local: execution::UtfCodepointLocalId(index),
            value: utf_codepoint_expr(value, context),
        },
        M::Custom(binding) => E::Custom(execution::CustomLocalExpr::new(
            execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(&target.custom_shape(binding.local().shape())),
            ),
            custom_expr(binding.value(), context),
        )),
        M::Float { local: _, value } => E::Float {
            local: execution::FloatLocalId(index),
            value: float_expr(value, context),
        },
        M::Bool { local: _, value } => E::Bool {
            local: execution::BoolLocalId(index),
            value: bool_expr(value, context),
        },
        M::Nil { local: _, value } => E::Nil {
            local: execution::NilLocalId(index),
            value: nil_expr(value, context),
        },
        M::Tuple { local: _, value } => E::Tuple {
            local: execution::TupleLocalId(index),
            value: tuple_expr(value, context),
        },
        M::List(value) => E::List(list::list_local_expr_at(index, value, context)),
        M::IntFunction { local: _, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(index),
            value: typed_function_expr(value, context, int_function_expr),
        },
        M::StringFunction { local: _, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(index),
            value: typed_function_expr(value, context, string_function_expr),
        },
        M::BitArrayFunction { local: _, value } => E::BitArrayFunction {
            local: execution::BitArrayFunctionLocalId(index),
            value: typed_function_expr(value, context, bit_array_function_expr),
        },
        M::UtfCodepointFunction { local: _, value } => E::UtfCodepointFunction {
            local: execution::UtfCodepointFunctionLocalId(index),
            value: typed_function_expr(value, context, utf_codepoint_function_expr),
        },
        M::CustomFunction { local, value } => E::CustomFunction {
            local: execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                context
                    .custom_function_type_with_substitution(local.type_(), target.substitution()),
            ),
            value: typed_function_expr(value, context, custom_function_expr),
        },
        M::FloatFunction { local: _, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(index),
            value: typed_function_expr(value, context, float_function_expr),
        },
        M::BoolFunction { local: _, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(index),
            value: typed_function_expr(value, context, bool_function_expr),
        },
        M::NilFunction { local: _, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(index),
            value: typed_function_expr(value, context, nil_function_expr),
        },
        M::TupleFunction { local: _, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(index),
            value: typed_function_expr(value, context, tuple_function_expr),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local_at_target(index, local, target, context),
            value: typed_function_expr(value, context, list_function_expr),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                context
                    .function_function_type_with_substitution(local.type_(), target.substitution()),
            ),
            value: typed_function_expr(value, context, function_function_expr),
        },
        M::GenericFunction { local, value } => {
            let _ = local;
            return execution::CallArg::from_kind(specialized_function_call_arg(
                function::specialized_function_binding_for_shape(
                    index,
                    value,
                    target.function_shape(value.shape()),
                    context,
                ),
            ));
        }
    })
}

fn call_arg_local_key(arg: &module::CallArg) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};
    use module::CallArgKind as A;

    match arg.kind() {
        A::Parametric { slot, .. } => super::frame::param_local_key(slot.local()),
        A::Int { local, .. } => LocalKey::new(LocalKind::Int, local.0),
        A::String { local, .. } => LocalKey::new(LocalKind::String, local.0),
        A::BitArray { local, .. } => LocalKey::new(LocalKind::BitArray, local.0),
        A::UtfCodepoint { local, .. } => LocalKey::new(LocalKind::UtfCodepoint, local.0),
        A::Custom(binding) => LocalKey::new(LocalKind::Custom, binding.local().id().0),
        A::Float { local, .. } => LocalKey::new(LocalKind::Float, local.0),
        A::Bool { local, .. } => LocalKey::new(LocalKind::Bool, local.0),
        A::Nil { local, .. } => LocalKey::new(LocalKind::Nil, local.0),
        A::Tuple { local, .. } => LocalKey::new(LocalKind::Tuple, local.0),
        A::List(local) => list_local_expr_key(local),
        A::IntFunction { local, .. } => LocalKey::new(LocalKind::IntFunction, local.0),
        A::StringFunction { local, .. } => LocalKey::new(LocalKind::StringFunction, local.0),
        A::BitArrayFunction { local, .. } => LocalKey::new(LocalKind::BitArrayFunction, local.0),
        A::UtfCodepointFunction { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointFunction, local.0)
        }
        A::CustomFunction { local, .. } => LocalKey::new(LocalKind::CustomFunction, local.id().0),
        A::FloatFunction { local, .. } => LocalKey::new(LocalKind::FloatFunction, local.0),
        A::BoolFunction { local, .. } => LocalKey::new(LocalKind::BoolFunction, local.0),
        A::NilFunction { local, .. } => LocalKey::new(LocalKind::NilFunction, local.0),
        A::TupleFunction { local, .. } => LocalKey::new(LocalKind::TupleFunction, local.0),
        A::ListFunction { local, .. } => super::frame::list_function_local_key(local),
        A::FunctionFunction { local, .. } => {
            LocalKey::new(LocalKind::FunctionFunction, local.id().0)
        }
        A::GenericFunction { local, .. } => LocalKey::new(LocalKind::GenericFunction, local.id().0),
    }
}

pub(super) fn list_local_expr_key(local: &module::ListLocalExpr) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};

    match local {
        module::ListLocalExpr::Generic { local, .. } => {
            LocalKey::new(LocalKind::GenericList, local.0)
        }
        module::ListLocalExpr::Int { local, .. } => LocalKey::new(LocalKind::IntList, local.0),
        module::ListLocalExpr::String { local, .. } => {
            LocalKey::new(LocalKind::StringList, local.0)
        }
        module::ListLocalExpr::BitArray { local, .. } => {
            LocalKey::new(LocalKind::BitArrayList, local.0)
        }
        module::ListLocalExpr::UtfCodepoint { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointList, local.0)
        }
        module::ListLocalExpr::Custom { local, .. } => {
            LocalKey::new(LocalKind::CustomList, local.0)
        }
        module::ListLocalExpr::Float { local, .. } => LocalKey::new(LocalKind::FloatList, local.0),
        module::ListLocalExpr::Bool { local, .. } => LocalKey::new(LocalKind::BoolList, local.0),
        module::ListLocalExpr::Nil { local, .. } => LocalKey::new(LocalKind::NilList, local.0),
        module::ListLocalExpr::Tuple { local, .. } => LocalKey::new(LocalKind::TupleList, local.0),
        module::ListLocalExpr::List { local, .. } => LocalKey::new(LocalKind::ListList, local.0),
        module::ListLocalExpr::Function { local, .. } => {
            LocalKey::new(LocalKind::FunctionList, local.0)
        }
    }
}

pub(super) fn capture_args(
    function: &module::FunctionInstantiation,
    args: &[module::CaptureArg],
    context: &mut super::LoweringContext,
) -> Vec<execution::CaptureArg> {
    args.iter()
        .map(|arg| {
            let target = context.target_local(function, capture_arg_local_key(arg));
            capture_arg_at(&target, arg, context)
        })
        .collect()
}

fn capture_arg_at(
    target: &super::TargetLocal,
    arg: &module::CaptureArg,
    context: &mut super::LoweringContext,
) -> execution::CaptureArg {
    use execution::CaptureArgKind as E;
    use module::CaptureArgKind as M;

    let index = target.index();

    execution::CaptureArg::from_kind(match arg.kind() {
        M::Generic { local, value } => {
            let _ = local;
            return specialized_capture_arg(specialized_generic_value_binding_for_shape(
                index,
                value,
                target.shape(),
                context,
            ));
        }
        M::Int { local: _, value } => E::Int {
            local: execution::IntLocalId(index),
            value: int_expr(value, context),
        },
        M::String { local: _, value } => E::String {
            local: execution::StringLocalId(index),
            value: string_expr(value, context),
        },
        M::BitArray { local: _, value } => E::BitArray {
            local: execution::BitArrayLocalId(index),
            value: bit_array_expr(value, context),
        },
        M::UtfCodepoint { local: _, value } => E::UtfCodepoint {
            local: execution::UtfCodepointLocalId(index),
            value: utf_codepoint_expr(value, context),
        },
        M::Custom(binding) => E::Custom(execution::CustomLocalExpr::new(
            execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(&target.custom_shape(binding.local().shape())),
            ),
            custom_expr(binding.value(), context),
        )),
        M::Float { local: _, value } => E::Float {
            local: execution::FloatLocalId(index),
            value: float_expr(value, context),
        },
        M::Bool { local: _, value } => E::Bool {
            local: execution::BoolLocalId(index),
            value: bool_expr(value, context),
        },
        M::Nil { local: _, value } => E::Nil {
            local: execution::NilLocalId(index),
            value: nil_expr(value, context),
        },
        M::Tuple { local: _, value } => E::Tuple {
            local: execution::TupleLocalId(index),
            value: tuple_expr(value, context),
        },
        M::List(value) => E::List(list::list_local_expr_at(index, value, context)),
        M::IntFunction { local: _, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(index),
            value: typed_function_expr(value, context, int_function_expr),
        },
        M::StringFunction { local: _, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(index),
            value: typed_function_expr(value, context, string_function_expr),
        },
        M::BitArrayFunction { local: _, value } => E::BitArrayFunction {
            local: execution::BitArrayFunctionLocalId(index),
            value: typed_function_expr(value, context, bit_array_function_expr),
        },
        M::UtfCodepointFunction { local: _, value } => E::UtfCodepointFunction {
            local: execution::UtfCodepointFunctionLocalId(index),
            value: typed_function_expr(value, context, utf_codepoint_function_expr),
        },
        M::CustomFunction { local, value } => E::CustomFunction {
            local: execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                context
                    .custom_function_type_with_substitution(local.type_(), target.substitution()),
            ),
            value: typed_function_expr(value, context, custom_function_expr),
        },
        M::FloatFunction { local: _, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(index),
            value: typed_function_expr(value, context, float_function_expr),
        },
        M::BoolFunction { local: _, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(index),
            value: typed_function_expr(value, context, bool_function_expr),
        },
        M::NilFunction { local: _, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(index),
            value: typed_function_expr(value, context, nil_function_expr),
        },
        M::TupleFunction { local: _, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(index),
            value: typed_function_expr(value, context, tuple_function_expr),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local_at_target(index, local, target, context),
            value: typed_function_expr(value, context, list_function_expr),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                context
                    .function_function_type_with_substitution(local.type_(), target.substitution()),
            ),
            value: typed_function_expr(value, context, function_function_expr),
        },
        M::GenericFunction { local, value } => {
            let _ = local;
            return execution::CaptureArg::from_kind(specialized_function_capture_arg(
                function::specialized_typed_generic_function_binding_for_shape(
                    index,
                    value,
                    target.function_shape(value.shape()),
                    context,
                ),
            ));
        }
    })
}

fn capture_arg_local_key(arg: &module::CaptureArg) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};
    use module::CaptureArgKind as A;

    match arg.kind() {
        A::Generic { local, .. } => LocalKey::new(LocalKind::Generic, local.id().0),
        A::Int { local, .. } => LocalKey::new(LocalKind::Int, local.0),
        A::String { local, .. } => LocalKey::new(LocalKind::String, local.0),
        A::BitArray { local, .. } => LocalKey::new(LocalKind::BitArray, local.0),
        A::UtfCodepoint { local, .. } => LocalKey::new(LocalKind::UtfCodepoint, local.0),
        A::Custom(binding) => LocalKey::new(LocalKind::Custom, binding.local().id().0),
        A::Float { local, .. } => LocalKey::new(LocalKind::Float, local.0),
        A::Bool { local, .. } => LocalKey::new(LocalKind::Bool, local.0),
        A::Nil { local, .. } => LocalKey::new(LocalKind::Nil, local.0),
        A::Tuple { local, .. } => LocalKey::new(LocalKind::Tuple, local.0),
        A::List(local) => list_local_expr_key(local),
        A::IntFunction { local, .. } => LocalKey::new(LocalKind::IntFunction, local.0),
        A::StringFunction { local, .. } => LocalKey::new(LocalKind::StringFunction, local.0),
        A::BitArrayFunction { local, .. } => LocalKey::new(LocalKind::BitArrayFunction, local.0),
        A::UtfCodepointFunction { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointFunction, local.0)
        }
        A::CustomFunction { local, .. } => LocalKey::new(LocalKind::CustomFunction, local.id().0),
        A::FloatFunction { local, .. } => LocalKey::new(LocalKind::FloatFunction, local.0),
        A::BoolFunction { local, .. } => LocalKey::new(LocalKind::BoolFunction, local.0),
        A::NilFunction { local, .. } => LocalKey::new(LocalKind::NilFunction, local.0),
        A::TupleFunction { local, .. } => LocalKey::new(LocalKind::TupleFunction, local.0),
        A::ListFunction { local, .. } => super::frame::list_function_local_key(local),
        A::FunctionFunction { local, .. } => {
            LocalKey::new(LocalKind::FunctionFunction, local.id().0)
        }
        A::GenericFunction { local, .. } => LocalKey::new(LocalKind::GenericFunction, local.id().0),
    }
}
pub(super) use bit_array::bit_array_expr;
