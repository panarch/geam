use super::{
    bool_expr, call_args, custom_function_expr, custom_list_expr, expr, float_expr, int_expr,
    panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_expr(
    expression: module::CustomExpr,
    context: &mut super::super::LoweringContext,
) -> execution::CustomExpr {
    use execution::CustomExprKind as E;
    use module::CustomExprKind as M;

    let (type_, kind) = expression.into_parts();
    let kind = match kind {
        M::Constructor {
            constructor,
            arguments,
        } => E::Constructor {
            constructor: context.custom_constructor(constructor),
            arguments: arguments
                .into_iter()
                .map(|argument| expr(argument, context))
                .collect(),
        },
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::CustomLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::CustomFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(custom_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(custom_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(custom_expr(*true_, context)),
            false_: Box::new(custom_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(custom_expr(*return_, context)),
        },
    };

    execution::CustomExpr::from_parts(context.custom_type(type_), kind)
}
