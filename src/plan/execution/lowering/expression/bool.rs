use super::{
    bool_function_expr, bool_list_expr, call_args, custom_expr, custom_field_access, expr,
    float_expr, int_expr, list_expr, panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bool_expr(
    expression: module::BoolExpr,
    context: &mut super::super::LoweringContext,
) -> execution::BoolExpr {
    use execution::BoolExprKind as E;
    use module::BoolExprKind as M;

    execution::BoolExpr::from_kind(match expression.into_kind() {
        M::Value(value) => E::Value(value),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::BoolLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::BoolFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(bool_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::CustomField(access) => E::CustomField(custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(bool_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::Not(value) => E::Not(Box::new(bool_expr(*value, context))),
        M::LtInt { left, right } => E::LtInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::LtEqInt { left, right } => E::LtEqInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::GtInt { left, right } => E::GtInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::GtEqInt { left, right } => E::GtEqInt {
            left: Box::new(int_expr(*left, context)),
            right: Box::new(int_expr(*right, context)),
        },
        M::LtFloat { left, right } => E::LtFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::LtEqFloat { left, right } => E::LtEqFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::GtFloat { left, right } => E::GtFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::GtEqFloat { left, right } => E::GtEqFloat {
            left: Box::new(float_expr(*left, context)),
            right: Box::new(float_expr(*right, context)),
        },
        M::Equal { left, right } => E::Equal {
            left: Box::new(expr(*left, context)),
            right: Box::new(expr(*right, context)),
        },
        M::NotEqual { left, right } => E::NotEqual {
            left: Box::new(expr(*left, context)),
            right: Box::new(expr(*right, context)),
        },
        M::StringStartsWith { value, prefix } => E::StringStartsWith {
            value: Box::new(string_expr(*value, context)),
            prefix,
        },
        M::ListLengthEquals { value, length } => E::ListLengthEquals {
            value: Box::new(list_expr(*value, context)),
            length,
        },
        M::ListLengthAtLeast { value, length } => E::ListLengthAtLeast {
            value: Box::new(list_expr(*value, context)),
            length,
        },
        M::BitArrayMatches { value, pattern } => E::BitArrayMatches {
            value: Box::new(super::bit_array_expr(*value, context)),
            pattern: super::super::pattern::bit_array_pattern(pattern),
        },
        M::CustomMatches { value, pattern } => E::CustomMatches {
            value: Box::new(custom_expr(*value, context)),
            pattern: Box::new(super::super::step::assert_pattern(*pattern, context)),
        },
        M::And { left, right } => E::And {
            left: Box::new(bool_expr(*left, context)),
            right: Box::new(bool_expr(*right, context)),
        },
        M::Or { left, right } => E::Or {
            left: Box::new(bool_expr(*left, context)),
            right: Box::new(bool_expr(*right, context)),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(bool_expr(*true_, context)),
            false_: Box::new(bool_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bool_expr(branch, context)))
                .collect(),
            fallback: Box::new(bool_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(bool_expr(*return_, context)),
        },
    })
}
