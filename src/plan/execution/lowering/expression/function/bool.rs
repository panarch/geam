use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bool_function_expr(
    expression: module::BoolFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::BoolFunctionExpr {
    use execution::BoolFunctionExprKind as E;
    use module::BoolFunctionExprKind as M;

    let (_, kind) = expression.into_parts();
    let kind = match kind {
        M::Reference(value) => E::Reference(super::function_reference(value, context, |id, _| {
            execution::BoolFunctionId(id.0)
        })),
        M::Closure {
            runtime_id,
            params,
            captures,
        } => E::Closure(super::closure_template(
            runtime_id,
            params,
            captures,
            context,
            |id, _| execution::BoolFunctionId(id.0),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::BoolFunctionLocalId(local.0),
        },
        M::Call {
            function,
            args,
            type_: _,
        } => E::Call {
            function: execution::BoolFunctionFunctionId(function.0),
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
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
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
            true_: Box::new(bool_function_expr(*true_, context)),
            false_: Box::new(bool_function_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_function_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_function_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_function_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps, context),
            return_: Box::new(bool_function_expr(*return_, context)),
        },
    };

    execution::BoolFunctionExpr::from_kind(kind)
}
