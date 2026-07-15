use super::{
    bool_expr, call_args, custom_field_access, expr, float_expr, int_expr, panic_expr, string_expr,
    tuple_function_expr, tuple_list_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn tuple_expr(
    expression: module::TupleExpr,
    context: &mut super::super::LoweringContext,
) -> execution::TupleExpr {
    use execution::TupleExprKind as E;
    use module::TupleExprKind as M;

    let (type_, _shape, kind) = expression.into_parts();
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
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
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
            steps: super::super::step::steps(steps, context),
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
