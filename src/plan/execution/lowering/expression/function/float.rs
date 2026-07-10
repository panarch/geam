use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn float_function_expr(
    expression: module::FloatFunctionExpr,
) -> execution::FloatFunctionExpr {
    use execution::FloatFunctionExprKind as E;
    use module::FloatFunctionExprKind as M;

    let (_, kind) = expression.into_parts();
    let kind = match kind {
        M::Reference(value) => E::Reference(super::function_reference(value, |id| {
            execution::FloatFunctionId(id.0)
        })),
        M::Closure {
            runtime_id,
            params,
            captures,
        } => E::Closure(super::closure_template(
            runtime_id,
            params,
            captures,
            |id| execution::FloatFunctionId(id.0),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::FloatFunctionLocalId(local.0),
        },
        M::Call {
            function,
            args,
            type_: _,
        } => E::Call {
            function: execution::FloatFunctionFunctionId(function.0),
            args: super::super::call_args(args),
        },
        M::FunctionCall {
            function,
            args,
            type_: _,
        } => E::FunctionCall {
            function: Box::new(function_function_expr(*function)),
            args: super::super::call_args(args),
        },
        M::TupleIndex {
            tuple,
            index,
            type_,
        } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(*tuple)),
            index,
            type_,
        },
        M::ListIndex { list, index, type_ } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(*list)),
            index,
            type_,
        },
        M::Panic(value) => E::Panic(super::super::panic_expr(value)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(*subject)),
            true_: Box::new(float_function_expr(*true_)),
            false_: Box::new(float_function_expr(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_function_expr(branch)))
                .collect(),
            fallback: Box::new(float_function_expr(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_function_expr(branch)))
                .collect(),
            fallback: Box::new(float_function_expr(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, float_function_expr(branch)))
                .collect(),
            fallback: Box::new(float_function_expr(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps),
            return_: Box::new(float_function_expr(*return_)),
        },
    };

    execution::FloatFunctionExpr::from_kind(kind)
}
