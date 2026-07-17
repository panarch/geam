use super::{
    bool_expr, call_args, custom_field_access, direct_call_args, float_expr, int_function_expr,
    int_list_expr, panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn int_expr(
    expression: &module::IntExpr,
    context: &mut super::super::LoweringContext,
) -> execution::IntExpr {
    use execution::IntExprKind as E;
    use module::IntExprKind as M;

    execution::IntExpr::from_kind(match expression.kind() {
        M::Value(value) => E::Value(value.clone()),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::IntLocalId(
                context.mapped_local(super::super::frame::LocalKind::Int, local.0),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.int_function_id(function),
            args: direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(int_function_expr(function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(int_list_expr(list, context)),
            index: *index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Add { left, right } => E::Add {
            left: Box::new(int_expr(left, context)),
            right: Box::new(int_expr(right, context)),
        },
        M::Sub { left, right } => E::Sub {
            left: Box::new(int_expr(left, context)),
            right: Box::new(int_expr(right, context)),
        },
        M::Mult { left, right } => E::Mult {
            left: Box::new(int_expr(left, context)),
            right: Box::new(int_expr(right, context)),
        },
        M::Div { left, right } => E::Div {
            left: Box::new(int_expr(left, context)),
            right: Box::new(int_expr(right, context)),
        },
        M::Remainder { left, right } => E::Remainder {
            left: Box::new(int_expr(left, context)),
            right: Box::new(int_expr(right, context)),
        },
        M::Negate(value) => E::Negate(Box::new(int_expr(value, context))),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(subject, context)),
            true_: Box::new(int_expr(true_, context)),
            false_: Box::new(int_expr(false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (pattern.clone(), int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (pattern.clone(), int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (*pattern, int_expr(branch, context)))
                .collect(),
            fallback: Box::new(int_expr(fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(int_expr(return_, context)),
        },
    })
}
