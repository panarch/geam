use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn list_function_expr(
    expression: module::ListFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::ListFunctionExpr {
    use execution::ListFunctionExprKind as E;
    use module::ListFunctionExprKind as M;

    let (_, _, kind) = expression.into_parts();
    let kind = match kind {
        M::Reference(value) => {
            E::Reference(super::function_reference(value, context, |id, context| {
                crate::plan::execution::lowering::id::list_function_id(id, context)
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
            crate::plan::execution::lowering::id::list_function_id,
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: crate::plan::execution::lowering::id::list_function_local(local, context),
        },
        M::Call {
            function,
            args,
            type_: _,
        } => E::Call {
            function: crate::plan::execution::lowering::id::list_function_function_id(
                function, context,
            ),
            args: super::super::call_args(args, context),
        },
        M::FunctionCall {
            function,
            args,
            type_: _,
        } => E::FunctionCall {
            function: Box::new(function_function_expr(*function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex {
            tuple,
            index,
            type_,
        } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(*tuple, context)),
            index,
            type_: context.function_type(type_),
        },
        M::ListIndex { list, index, type_ } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(*list, context)),
            index,
            type_: context.function_type(type_),
        },
        M::Panic(value) => E::Panic(super::super::panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(*subject, context)),
            true_: Box::new(list_function_expr(*true_, context)),
            false_: Box::new(list_function_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps, context),
            return_: Box::new(list_function_expr(*return_, context)),
        },
    };

    execution::ListFunctionExpr::from_kind(kind)
}
