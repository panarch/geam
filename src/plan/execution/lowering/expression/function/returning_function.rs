use super::super::super::specialization::Representability;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn function_function_expr(
    expression: &module::FunctionFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExpr> {
    let return_shape =
        context.concrete_function_shape(expression.function_function_type().return_shape());
    let type_ = context.function_function_type(expression.function_function_type().clone());
    let kind = function_function_expr_kind(expression.kind(), &return_shape, &type_, context);

    kind.map(|kind| execution::FunctionFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_function_function_expr(
    expression: &module::GenericFunctionExpr,
    return_shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExpr> {
    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.specialized_function_function_type(shape.arguments(), return_shape);
    let kind = generic_function_function_expr_kind(expression, return_shape, &type_, context);

    kind.map(|kind| execution::FunctionFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_function_function_expr_kind(
    expression: &module::GenericFunctionExpr,
    return_shape: &super::super::super::specialization::SpecializedFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExprKind> {
    use execution::FunctionFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    match expression.kind() {
        M::Constant(value) => context
            .generic_function_function_constant(value, return_shape)
            .map(E::Constant),
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.function_function_id(function, return_shape)
            })
            .map(E::Reference)
        }
        M::Closure { function, captures } => {
            super::closure_template(function, captures, context, |function, context| {
                context.function_function_id(function, return_shape)
            })
            .map(E::Closure)
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(
                    context.generic_function_local_index(local.id()),
                ),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.function_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::super::function_call(
            args,
            context,
            |context| function_function_expr(function, context),
            |context| super::evaluated_function_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex { list, index } => {
            super::super::function_list_expr(list, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
            })
        }
        M::Panic(panic) => super::super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            context,
            |context| generic_function_function_expr_kind(true_, return_shape, type_, context),
            |context| generic_function_function_expr_kind(false_, return_shape, type_, context),
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
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::IntCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                generic_function_function_expr_kind(return_, return_shape, type_, context).map(
                    |return_| E::Block {
                        steps,
                        return_: Box::new(return_),
                    },
                )
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn function_function_expr_kind(
    kind: &module::FunctionFunctionExprKind,
    return_shape: &super::super::super::specialization::SpecializedFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExprKind> {
    use execution::FunctionFunctionExprKind as E;
    use module::FunctionFunctionExprKind as M;

    match kind {
        M::Constant(value) => context.function_function_constant(value).map(E::Constant),
        M::Reference(value) => super::function_reference(value, context, |function, context| {
            context.function_function_id(function, return_shape)
        })
        .map(E::Reference),
        M::Closure { function, captures } => {
            super::closure_template(function, captures, context, |function, context| {
                context.function_function_id(function, return_shape)
            })
            .map(E::Closure)
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: crate::plan::execution::lowering::id::function_function_local(local, context),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.function_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::super::function_call(
            args,
            context,
            |context| function_function_expr(function, context),
            |context| super::evaluated_function_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex { list, index } => {
            super::super::function_list_expr(list, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
            })
        }
        M::Panic(value) => super::super::panic_expr(value, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            context,
            |context| function_function_expr_kind(true_, return_shape, type_, context),
            |context| function_function_expr_kind(false_, return_shape, type_, context),
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
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::IntCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                function_function_expr_kind(branch, return_shape, type_, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                function_function_expr_kind(fallback, return_shape, type_, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            crate::plan::execution::lowering::step::steps(steps, context).and_then(|steps| {
                function_function_expr_kind(return_, return_shape, type_, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_),
                    }
                })
            })
        }
    }
}
