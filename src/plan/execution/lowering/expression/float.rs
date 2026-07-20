use super::super::specialization::Representability;
use super::{
    custom_field_access, float_function_expr, float_list_expr, int_expr, panic_expr, string_expr,
    tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn float_expr(
    expression: &module::FloatExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FloatExpr> {
    use execution::FloatExprKind as E;
    use module::FloatExprKind as M;

    let kind = match expression.kind() {
        M::Value(value) => Representability::Inhabited(E::Value(*value)),
        M::Constant(reference) => context.float_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::FloatLocalId(
                context.mapped_local(super::super::frame::LocalKind::Float, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.float_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| float_function_expr(function, context),
            |context| super::function::evaluated_float_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => float_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::Add { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| E::Add {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Sub { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| E::Sub {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Mult { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| E::Mult {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Div { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| E::Div {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| float_expr(true_, context),
            |context| float_expr(false_, context),
            execution::FloatExpr::into_kind,
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
                float_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                float_expr(fallback, context).map(|fallback| E::IntCase {
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
                float_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                float_expr(fallback, context).map(|fallback| E::StringCase {
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
                float_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                float_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                float_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(execution::FloatExpr::from_kind)
}
