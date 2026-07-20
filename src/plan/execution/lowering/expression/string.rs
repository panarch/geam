use super::super::specialization::Representability;
use super::{
    custom_field_access, float_expr, int_expr, panic_expr, string_function_expr, string_list_expr,
    tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn string_expr(
    expression: &module::StringExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::StringExpr> {
    use execution::StringExprKind as E;
    use module::StringExprKind as M;

    let kind = match expression.kind() {
        M::Value(value) => Representability::Inhabited(E::Value(value.clone())),
        M::Constant(reference) => context.string_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::StringLocalId(
                context.mapped_local(super::super::frame::LocalKind::String, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.string_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| string_function_expr(function, context),
            |context| super::function::evaluated_string_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => string_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::Concatenate { left, right } => {
            string_expr(left, context).zip_with(string_expr(right, context), |left, right| {
                E::Concatenate {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
        }
        M::DropPrefix { value, prefix } => string_expr(value, context).map(|value| E::DropPrefix {
            value: Box::new(value),
            prefix: prefix.clone(),
        }),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| string_expr(true_, context),
            |context| string_expr(false_, context),
            execution::StringExpr::into_kind,
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                string_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                string_expr(fallback, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                string_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                string_expr(fallback, context).map(|fallback| E::StringCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                string_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                string_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                string_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(execution::StringExpr::from_kind)
}
