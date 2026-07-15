use super::{
    bool_expr, call_args, custom_field_access, float_expr, int_expr, nil_function_expr,
    nil_list_expr, panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn nil_expr(
    expression: module::NilExpr,
    context: &mut super::super::LoweringContext,
) -> execution::NilExpr {
    use execution::NilExprKind as E;
    use module::NilExprKind as M;

    execution::NilExpr::from_kind(match expression.into_kind() {
        M::Value => E::Value,
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::NilLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::NilFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(nil_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(nil_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(nil_expr(*true_, context)),
            false_: Box::new(nil_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, nil_expr(branch, context)))
                .collect(),
            fallback: Box::new(nil_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(nil_expr(*return_, context)),
        },
    })
}
