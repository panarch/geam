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

pub(super) fn expr(expression: module::Expr) -> execution::Expr {
    execution::Expr::from_kind(match expression.into_kind() {
        module::ExprKind::Int(expression) => execution::ExprKind::Int(int_expr(expression)),
        module::ExprKind::String(expression) => {
            execution::ExprKind::String(string_expr(expression))
        }
        module::ExprKind::Float(expression) => execution::ExprKind::Float(float_expr(expression)),
        module::ExprKind::Bool(expression) => execution::ExprKind::Bool(bool_expr(expression)),
        module::ExprKind::Nil(expression) => execution::ExprKind::Nil(nil_expr(expression)),
        module::ExprKind::Tuple(expression) => execution::ExprKind::Tuple(tuple_expr(expression)),
        module::ExprKind::List(expression) => execution::ExprKind::List(list_expr(expression)),
        module::ExprKind::Function(expression) => {
            execution::ExprKind::Function(function_expr(expression))
        }
    })
}

pub(super) fn int_expr(expression: module::IntExpr) -> execution::IntExpr {
    use execution::IntExprKind as E;
    use module::IntExprKind as M;

    execution::IntExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::IntLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::IntFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(int_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(int_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::Add { left, right } => E::Add {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::Sub { left, right } => E::Sub {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::Mult { left, right } => E::Mult {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::Div { left, right } => E::Div {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::Remainder { left, right } => E::Remainder {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::Negate(value) => E::Negate(Box::new(int_expr(*value))),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(int_expr(*true_)),
            false_: Box::new(int_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch)))
                .collect(),
            fallback: Box::new(int_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch)))
                .collect(),
            fallback: Box::new(int_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, int_expr(branch)))
                .collect(),
            fallback: Box::new(int_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(int_expr(*return_)),
        },
    })
}

pub(super) fn float_expr(expression: module::FloatExpr) -> execution::FloatExpr {
    use execution::FloatExprKind as E;
    use module::FloatExprKind as M;

    execution::FloatExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::FloatLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::FloatFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(float_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(float_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::Add { left, right } => E::Add {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::Sub { left, right } => E::Sub {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::Mult { left, right } => E::Mult {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::Div { left, right } => E::Div {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(float_expr(*true_)),
            false_: Box::new(float_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch)))
                .collect(),
            fallback: Box::new(float_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch)))
                .collect(),
            fallback: Box::new(float_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_expr(branch)))
                .collect(),
            fallback: Box::new(float_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(float_expr(*return_)),
        },
    })
}

pub(super) fn string_expr(expression: module::StringExpr) -> execution::StringExpr {
    use execution::StringExprKind as E;
    use module::StringExprKind as M;

    execution::StringExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::StringLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::StringFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(string_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(string_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::Concatenate { left, right } => E::Concatenate {
            left: Box::new(string_expr(*left)),
            right: Box::new(string_expr(*right)),
        },
        M::DropPrefix { value, prefix } => E::DropPrefix {
            value: Box::new(string_expr(*value)),
            prefix,
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(string_expr(*true_)),
            false_: Box::new(string_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch)))
                .collect(),
            fallback: Box::new(string_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch)))
                .collect(),
            fallback: Box::new(string_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, string_expr(branch)))
                .collect(),
            fallback: Box::new(string_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(string_expr(*return_)),
        },
    })
}

pub(super) fn nil_expr(expression: module::NilExpr) -> execution::NilExpr {
    use execution::NilExprKind as E;
    use module::NilExprKind as M;

    execution::NilExpr::from_kind(match expression.into_kind() {
        M::Value => E::Value,
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::NilLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::NilFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(nil_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(nil_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(nil_expr(*true_)),
            false_: Box::new(nil_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(nil_expr(*return_)),
        },
    })
}

pub(super) fn tuple_expr(expression: module::TupleExpr) -> execution::TupleExpr {
    use execution::TupleExprKind as E;
    use module::TupleExprKind as M;

    let (type_, kind) = expression.into_parts();
    let kind = match kind {
        M::Value(values) => E::Value(values.into_iter().map(expr).collect()),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::TupleLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::TupleFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(tuple_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(tuple_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(tuple_expr(*true_)),
            false_: Box::new(tuple_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, tuple_expr(branch)))
                .collect(),
            fallback: Box::new(tuple_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(tuple_expr(*return_)),
        },
    };

    execution::TupleExpr::from_parts(type_, kind)
}

pub(super) fn bool_expr(expression: module::BoolExpr) -> execution::BoolExpr {
    use execution::BoolExprKind as E;
    use module::BoolExprKind as M;

    execution::BoolExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::BoolLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::BoolFunctionId(function.0),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(bool_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(bool_list_expr(*list)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::Not(value) => E::Not(Box::new(bool_expr(*value))),
        M::LtInt { left, right } => E::LtInt {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::LtEqInt { left, right } => E::LtEqInt {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::GtInt { left, right } => E::GtInt {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::GtEqInt { left, right } => E::GtEqInt {
            left: Box::new(int_expr(*left)),
            right: Box::new(int_expr(*right)),
        },
        M::LtFloat { left, right } => E::LtFloat {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::LtEqFloat { left, right } => E::LtEqFloat {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::GtFloat { left, right } => E::GtFloat {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::GtEqFloat { left, right } => E::GtEqFloat {
            left: Box::new(float_expr(*left)),
            right: Box::new(float_expr(*right)),
        },
        M::Equal { left, right } => E::Equal {
            left: Box::new(expr(*left)),
            right: Box::new(expr(*right)),
        },
        M::NotEqual { left, right } => E::NotEqual {
            left: Box::new(expr(*left)),
            right: Box::new(expr(*right)),
        },
        M::StringStartsWith { value, prefix } => E::StringStartsWith {
            value: Box::new(string_expr(*value)),
            prefix,
        },
        M::ListLengthEquals { value, length } => E::ListLengthEquals {
            value: Box::new(list_expr(*value)),
            length,
        },
        M::ListLengthAtLeast { value, length } => E::ListLengthAtLeast {
            value: Box::new(list_expr(*value)),
            length,
        },
        M::And { left, right } => E::And {
            left: Box::new(bool_expr(*left)),
            right: Box::new(bool_expr(*right)),
        },
        M::Or { left, right } => E::Or {
            left: Box::new(bool_expr(*left)),
            right: Box::new(bool_expr(*right)),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(bool_expr(*true_)),
            false_: Box::new(bool_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(bool_expr(*return_)),
        },
    })
}

pub(super) fn panic_expr(expression: module::PanicExpr) -> execution::PanicExpr {
    let (site, kind) = expression.into_parts();
    let kind = match kind {
        module::PanicExprKind::Panic { message } => execution::PanicExprKind::Panic {
            message: message.map(|message| Box::new(string_expr(*message))),
        },
        module::PanicExprKind::Todo { message } => execution::PanicExprKind::Todo {
            message: message.map(|message| Box::new(string_expr(*message))),
        },
        module::PanicExprKind::EmptyFunction => execution::PanicExprKind::EmptyFunction,
        module::PanicExprKind::EmptyBlock => execution::PanicExprKind::EmptyBlock,
        module::PanicExprKind::IncompleteUse => execution::PanicExprKind::IncompleteUse,
    };
    execution::PanicExpr::from_parts(site, kind)
}

pub(super) fn call_args(args: Vec<module::CallArg>) -> Vec<execution::CallArg> {
    args.into_iter().map(call_arg).collect()
}

pub(super) fn call_arg(arg: module::CallArg) -> execution::CallArg {
    use execution::CallArgKind as E;
    use module::CallArgKind as M;

    execution::CallArg::from_kind(match arg.into_kind() {
        M::Int { local, value } => E::Int {
            local: execution::IntLocalId(local.0),
            value: int_expr(value),
        },
        M::String { local, value } => E::String {
            local: execution::StringLocalId(local.0),
            value: string_expr(value),
        },
        M::Float { local, value } => E::Float {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value),
        },
        M::Bool { local, value } => E::Bool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value),
        },
        M::Nil { local, value } => E::Nil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value),
        },
        M::Tuple { local, value } => E::Tuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value),
        },
        M::List(value) => E::List(list::list_local_expr(value)),
        M::IntFunction { local, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value),
        },
        M::StringFunction { local, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value),
        },
        M::FloatFunction { local, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: float_function_expr(value),
        },
        M::BoolFunction { local, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: bool_function_expr(value),
        },
        M::NilFunction { local, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value),
        },
        M::TupleFunction { local, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local(local),
            value: list_function_expr(value),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value),
        },
    })
}

pub(super) fn capture_args(args: Vec<module::CaptureArg>) -> Vec<execution::CaptureArg> {
    args.into_iter().map(capture_arg).collect()
}

fn capture_arg(arg: module::CaptureArg) -> execution::CaptureArg {
    use execution::CaptureArgKind as E;
    use module::CaptureArgKind as M;

    execution::CaptureArg::from_kind(match arg.into_kind() {
        M::Int { local, value } => E::Int {
            local: execution::IntLocalId(local.0),
            value: int_expr(value),
        },
        M::String { local, value } => E::String {
            local: execution::StringLocalId(local.0),
            value: string_expr(value),
        },
        M::Float { local, value } => E::Float {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value),
        },
        M::Bool { local, value } => E::Bool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value),
        },
        M::Nil { local, value } => E::Nil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value),
        },
        M::Tuple { local, value } => E::Tuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value),
        },
        M::List(value) => E::List(list::list_local_expr(value)),
        M::IntFunction { local, value } => E::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value),
        },
        M::StringFunction { local, value } => E::StringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value),
        },
        M::FloatFunction { local, value } => E::FloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: float_function_expr(value),
        },
        M::BoolFunction { local, value } => E::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: bool_function_expr(value),
        },
        M::NilFunction { local, value } => E::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value),
        },
        M::TupleFunction { local, value } => E::TupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value),
        },
        M::ListFunction { local, value } => E::ListFunction {
            local: list_function_local(local),
            value: list_function_expr(value),
        },
        M::FunctionFunction { local, value } => E::FunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value),
        },
    })
}
