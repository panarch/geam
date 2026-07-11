mod function;
mod list;

use super::id::list_function_local;
use crate::plan::module;

pub(super) use function::{
    bool_function_expr, float_function_expr, function_expr, function_function_expr,
    int_function_expr, list_function_expr, nil_function_expr, string_function_expr,
    tuple_function_expr,
};
pub(super) use list::{
    bool_list_expr, float_list_expr, function_list_expr, int_list_expr, list_expr, list_list_expr,
    list_local_expr, nil_list_expr, string_list_expr, tuple_list_expr,
};

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

pub(super) fn int_expr(
    expression: module::IntExpr,
    context: &mut super::LoweringContext,
) -> execution::IntExpr {
    use execution::IntExprKind as E;
    use module::IntExprKind as M;

    execution::IntExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::IntLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::IntFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(int_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(int_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Add { left, right } => E::Add {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::Sub { left, right } => E::Sub {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::Mult { left, right } => E::Mult {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::Div { left, right } => E::Div {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::Remainder { left, right } => E::Remainder {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::Negate(value) => E::Negate(Box::new(int_expr(*value, context))),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(int_expr(*true_, context)),
            false_: Box::new(int_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(int_expr(*return_, context)),
        },
    })
}

pub(super) fn float_expr(
    expression: module::FloatExpr,
    context: &mut super::LoweringContext,
) -> execution::FloatExpr {
    use execution::FloatExprKind as E;
    use module::FloatExprKind as M;

    execution::FloatExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::FloatLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::FloatFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(float_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(float_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Add { left, right } => E::Add {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::Sub { left, right } => E::Sub {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::Mult { left, right } => E::Mult {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::Div { left, right } => E::Div {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(float_expr(*true_, context)),
            false_: Box::new(float_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch, context)))
                .collect(),
            fallback: Box::new(float_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch, context)))
                .collect(),
            fallback: Box::new(float_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch, context)))
                .collect(),
            fallback: Box::new(float_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(float_expr(*return_, context)),
        },
    })
}

pub(super) fn string_expr(
    expression: module::StringExpr,
    context: &mut super::LoweringContext,
) -> execution::StringExpr {
    use execution::StringExprKind as E;
    use module::StringExprKind as M;

    execution::StringExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::StringLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::StringFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(string_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(string_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Concatenate { left, right } => E::Concatenate {
            left: Box::new(string_expr(*left, context)),
            right: Box::new(string_expr(*right, context)),
        },
        M::DropPrefix { value, prefix } => E::DropPrefix {
            value: Box::new(string_expr(*value, context)),
            prefix,
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(string_expr(*true_, context)),
            false_: Box::new(string_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch, context)))
                .collect(),
            fallback: Box::new(string_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch, context)))
                .collect(),
            fallback: Box::new(string_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch, context)))
                .collect(),
            fallback: Box::new(string_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(string_expr(*return_, context)),
        },
    })
}

pub(super) fn nil_expr(
    expression: module::NilExpr,
    context: &mut super::LoweringContext,
) -> execution::NilExpr {
    use execution::NilExprKind as E;
    use module::NilExprKind as M;

    execution::NilExpr::from_kind(match expression.into_kind() {
        M::Value => E::Value,
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::NilLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::NilFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(nil_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(nil_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(nil_expr(*true_, context)),
            false_: Box::new(nil_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(nil_expr(*return_, context)),
        },
    })
}

pub(super) fn tuple_expr(
    expression: module::TupleExpr,
    context: &mut super::LoweringContext,
) -> execution::TupleExpr {
    use execution::TupleExprKind as E;
    use module::TupleExprKind as M;

    let (type_, kind) = expression.into_parts();
    let kind = match kind {
        M::Value(values) => E::Value(
            values
                .into_iter()
                .map(|value| expr(value, context))
                .collect(),
        ),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::TupleLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::TupleFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(tuple_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(tuple_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(tuple_expr(*true_, context)),
            false_: Box::new(tuple_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch, context)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch, context)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch, context)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(tuple_expr(*return_, context)),
        },
    };

    execution::TupleExpr::from_parts(
        type_
            .into_iter()
            .map(|type_| context.value_type(type_))
            .collect(),
        kind,
    )
}

pub(super) fn bool_expr(
    expression: module::BoolExpr,
    context: &mut super::LoweringContext,
) -> execution::BoolExpr {
    use execution::BoolExprKind as E;
    use module::BoolExprKind as M;

    execution::BoolExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::BoolLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::BoolFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(bool_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(bool_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Not(value) => E::Not(Box::new(bool_expr(*value, context))),
        M::LtInt { left, right } => E::LtInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::LtEqInt { left, right } => E::LtEqInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::GtInt { left, right } => E::GtInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::GtEqInt { left, right } => E::GtEqInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::LtFloat { left, right } => E::LtFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::LtEqFloat { left, right } => E::LtEqFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::GtFloat { left, right } => E::GtFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::GtEqFloat { left, right } => E::GtEqFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::Equal { left, right } => E::Equal {
            left: Box::new(expr(*left, context)),
            right: Box::new(expr(*right, context)),
        },
        M::NotEqual { left, right } => E::NotEqual {
            left: Box::new(expr(*left, context)),
            right: Box::new(expr(*right, context)),
        },
        M::StringStartsWith { value, prefix } => E::StringStartsWith {
            value: Box::new(string_expr(*value, context)),
            prefix,
        },
        M::ListLengthEquals { value, length } => E::ListLengthEquals {
            value: Box::new(list_expr(*value, context)),
            length,
        },
        M::ListLengthAtLeast { value, length } => E::ListLengthAtLeast {
            value: Box::new(list_expr(*value, context)),
            length,
        },
        M::And { left, right } => E::And {
            left: Box::new(bool_expr(*left, context)),
            right: Box::new(bool_expr(*right, context)),
        },
        M::Or { left, right } => E::Or {
            left: Box::new(bool_expr(*left, context)),
            right: Box::new(bool_expr(*right, context)),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(bool_expr(*true_, context)),
            false_: Box::new(bool_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(bool_expr(*return_, context)),
        },
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
