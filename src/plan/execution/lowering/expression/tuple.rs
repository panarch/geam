use super::super::specialization::Representability;
use super::{
    custom_field_access, expr, float_expr, int_expr, panic_expr, string_expr, tuple_function_expr,
    tuple_list_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn tuple_expr(
    expression: &module::TupleExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::TupleExpr> {
    use execution::TupleExprKind as E;
    use module::TupleExprKind as M;

    let specialized_elements = expression
        .shape()
        .iter()
        .map(|shape| context.concrete_value_shape(shape))
        .collect::<Vec<_>>();
    let type_ = expression
        .type_()
        .iter()
        .cloned()
        .map(|type_| context.value_type(type_))
        .collect();
    if let super::super::specialization::CompoundInhabitation::Uninhabited(proof) = context
        .representations
        .tuple_inhabitation(&specialized_elements)
    {
        return super::tuple_never_expr(expression, &proof, context)
            .map(|expression| execution::TupleExpr::from_parts(type_, E::Never(expression)));
    }

    let kind = match expression.kind() {
        M::Value(values) => {
            Representability::collect(values.iter().map(|value| expr(value, context))).map(E::Value)
        }
        M::Constant(reference) => context.tuple_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::TupleLocalId(
                context.mapped_local(super::super::frame::LocalKind::Tuple, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.tuple_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| tuple_function_expr(function, context),
            |context| super::function::evaluated_tuple_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => tuple_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| tuple_expr(true_, context),
            |context| tuple_expr(false_, context),
            execution::TupleExpr::into_kind,
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
                tuple_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                tuple_expr(fallback, context).map(|fallback| E::IntCase {
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
                tuple_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                tuple_expr(fallback, context).map(|fallback| E::StringCase {
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
                tuple_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                tuple_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                tuple_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(|kind| execution::TupleExpr::from_parts(type_, kind))
}
