use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_function_expr(
    expression: module::CustomFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::CustomFunctionExpr {
    let (type_, kind) = expression.into_parts();
    let kind = custom_function_expr_kind(kind, context);

    execution::CustomFunctionExpr::from_parts(context.custom_function_type(type_), kind)
}

pub(in crate::plan::execution::lowering) fn custom_function_expr_kind(
    kind: module::CustomFunctionExprKind,
    context: &mut super::super::super::LoweringContext,
) -> execution::CustomFunctionExprKind {
    use execution::CustomFunctionExprKind as E;
    use module::CustomFunctionExprKind as M;

    match kind {
        M::Constructor(constructor) => E::Constructor(context.custom_constructor(constructor)),
        M::Reference(value) => {
            E::Reference(super::function_reference(value, context, |id, context| {
                super::super::super::id::custom_function_id(id, context)
            }))
        }
        M::Closure {
            runtime_id,
            params,
            captures,
        } => E::Closure(super::closure_template(
            runtime_id,
            params,
            captures,
            context,
            super::super::super::id::custom_function_id,
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: crate::plan::execution::lowering::id::custom_function_local(local, context),
        },
        M::Call { function, args } => E::Call {
            function: execution::CustomFunctionFunctionId::new(
                function.index(),
                context.custom_function_type(function.type_().clone()),
            ),
            args: super::super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(function_function_expr(*function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(*tuple, context)),
            index,
        },
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(super::super::panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(*subject, context)),
            true_: Box::new(custom_function_expr_kind(*true_, context)),
            false_: Box::new(custom_function_expr_kind(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_function_expr_kind(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_function_expr_kind(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_expr_kind(branch, context)))
                .collect(),
            fallback: Box::new(custom_function_expr_kind(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps, context),
            return_: Box::new(custom_function_expr_kind(*return_, context)),
        },
    }
}
