use super::super::specialization::Representability;
use super::{
    custom_field_access, float_expr, int_function_expr, int_list_expr, panic_expr, string_expr,
    tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn int_expr(
    expression: &module::IntExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::IntExpr> {
    use execution::IntExprKind as E;
    use module::IntExprKind as M;

    let kind = match expression.kind() {
        M::Value(value) => Representability::Inhabited(E::Value(value.clone())),
        M::Constant(reference) => context.int_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::IntLocalId(
                context.mapped_local(super::super::frame::LocalKind::Int, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.int_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| int_function_expr(function, context),
            |context| super::function::evaluated_int_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => int_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::Add { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::Add {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Sub { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::Sub {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Mult { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::Mult {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Div { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::Div {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Remainder { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::Remainder {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Negate(value) => int_expr(value, context).map(|value| E::Negate(Box::new(value))),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| int_expr(true_, context),
            |context| int_expr(false_, context),
            execution::IntExpr::into_kind,
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
                int_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                int_expr(fallback, context).map(|fallback| E::IntCase {
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
                int_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                int_expr(fallback, context).map(|fallback| E::StringCase {
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
                int_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                int_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                int_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(execution::IntExpr::from_kind)
}
