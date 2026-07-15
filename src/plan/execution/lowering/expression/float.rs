use super::{
    bool_expr, call_args, custom_field_access, float_function_expr, float_list_expr, int_expr,
    panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn float_expr(
    expression: module::FloatExpr,
    context: &mut super::super::LoweringContext,
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
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
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
            steps: super::super::step::steps(steps, context),
            return_: Box::new(float_expr(*return_, context)),
        },
    })
}
