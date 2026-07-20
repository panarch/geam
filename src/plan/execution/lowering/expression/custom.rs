use super::super::specialization::Representability;
use super::{
    custom_field_access, custom_list_expr, expr, float_expr, int_expr, panic_expr, string_expr,
    tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_expr(
    expression: &module::CustomExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomExpr> {
    let shape = context.concrete_custom_value_shape(expression.shape());
    let type_ = context.lower_concrete_custom_shape(&shape);
    if let super::super::specialization::CompoundInhabitation::Uninhabited(proof) =
        context.representations.custom_inhabitation(&shape)
    {
        return super::custom_never_expr(expression, &proof, context).map(|expression| {
            execution::CustomExpr::from_parts(type_, execution::CustomExprKind::Never(expression))
        });
    }
    custom_expr_kind(expression.kind(), &shape, context)
        .map(|kind| execution::CustomExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn custom_expr_kind(
    kind: &module::CustomExprKind,
    shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomExprKind> {
    use execution::CustomExprKind as E;
    use module::CustomExprKind as M;

    match kind {
        M::Constructor(construction) => {
            let constructor = context.custom_constructor(construction.constructor().clone());
            Representability::collect(
                construction
                    .fields()
                    .iter()
                    .map(|field| expr(field, context)),
            )
            .map(|fields| {
                E::Constructor(execution::CustomConstruction::from_parts(
                    constructor,
                    fields.into_boxed_slice(),
                ))
            })
        }
        M::Constant(reference) => context.custom_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: super::super::id::custom_local(local, context),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.custom_function_id(function, shape)
            })
            .map(E::Call)
        }
        M::FunctionCall(call) => super::function_call(
            call.arguments(),
            context,
            |context| super::custom_function_expr(call.function(), context),
            |context| super::function::evaluated_custom_function_expr(call.function(), context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => custom_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| custom_expr_kind(true_, shape, context),
            |context| custom_expr_kind(false_, shape, context),
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
                custom_expr_kind(branch, shape, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_expr_kind(fallback, shape, context).map(|fallback| E::IntCase {
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
                custom_expr_kind(branch, shape, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_expr_kind(fallback, shape, context).map(|fallback| E::StringCase {
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
                custom_expr_kind(branch, shape, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                custom_expr_kind(fallback, shape, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                custom_expr_kind(return_, shape, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    }
}
