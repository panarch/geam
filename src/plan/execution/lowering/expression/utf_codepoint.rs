use super::{
    bool_expr, call_args, float_expr, int_expr, panic_expr, string_expr, tuple_expr,
    utf_codepoint_function_expr, utf_codepoint_list_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn utf_codepoint_expr(
    expression: module::UtfCodepointExpr,
    context: &mut super::super::LoweringContext,
) -> execution::UtfCodepointExpr {
    use execution::UtfCodepointExprKind as E;
    use module::UtfCodepointExprKind as M;

    execution::UtfCodepointExpr::from_kind(match expression.into_kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::UtfCodepointLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::UtfCodepointFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(utf_codepoint_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(utf_codepoint_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(utf_codepoint_expr(*true_, context)),
            false_: Box::new(utf_codepoint_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, utf_codepoint_expr(branch, context)))
                .collect(),
            fallback: Box::new(utf_codepoint_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, utf_codepoint_expr(branch, context)))
                .collect(),
            fallback: Box::new(utf_codepoint_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, utf_codepoint_expr(branch, context)))
                .collect(),
            fallback: Box::new(utf_codepoint_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(utf_codepoint_expr(*return_, context)),
        },
    })
}
