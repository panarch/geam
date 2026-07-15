use super::{
    bool_expr, call_args, custom_field_access, custom_function_expr, custom_list_expr, expr,
    float_expr, int_expr, panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_expr(
    expression: module::CustomExpr,
    context: &mut super::super::LoweringContext,
) -> execution::CustomExpr {
    let (shape, kind) = expression.into_parts();
    execution::CustomExpr::from_parts(
        context.custom_value_shape(shape),
        custom_expr_kind(kind, context),
    )
}

pub(in crate::plan::execution::lowering) fn custom_expr_kind(
    kind: module::CustomExprKind,
    context: &mut super::super::LoweringContext,
) -> execution::CustomExprKind {
    use execution::CustomExprKind as E;
    use module::CustomExprKind as M;

    match kind {
        M::Constructor(construction) => {
            let (constructor, fields) = construction.into_parts();
            E::Constructor(execution::CustomConstruction::from_parts(
                context.custom_constructor(constructor),
                fields
                    .into_vec()
                    .into_iter()
                    .map(|field| expr(field, context))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ))
        }
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: super::super::id::custom_local(local, context),
        },
        M::Call { function, args } => E::Call {
            function: super::super::id::custom_function_id(function, context),
            args: call_args(args, context),
        },
        M::FunctionCall(call) => {
            let (function, arguments) = call.into_parts();
            E::FunctionCall(execution::CustomFunctionCall::from_parts(
                custom_function_expr(function, context),
                call_args(arguments.into_vec(), context).into_boxed_slice(),
            ))
        }
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
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
            true_: Box::new(custom_expr_kind(*true_, context)),
            false_: Box::new(custom_expr_kind(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr_kind(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr_kind(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_expr_kind(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(custom_expr_kind(*return_, context)),
        },
    }
}
