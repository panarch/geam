use super::super::specialization::{
    CompoundInhabitation, Representability, UninhabitedCustomValueShape,
    UninhabitedTupleValueShape, UninhabitedValueShape, ValueInhabitation,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn uninhabited_expr(
    expression: &module::Expr,
    proof: &UninhabitedValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    match (expression.kind(), proof) {
        (module::ExprKind::Generic(expression), _) => {
            super::generic::never_expr(expression, context)
        }
        (module::ExprKind::Tuple(expression), UninhabitedValueShape::Tuple(proof)) => {
            tuple_never_expr(expression, proof, context)
        }
        (module::ExprKind::Custom(expression), UninhabitedValueShape::Custom(proof)) => {
            custom_never_expr(expression, proof, context)
        }
        _ => Representability::Uninhabited,
    }
}

pub(in crate::plan::execution::lowering) fn tuple_never_expr(
    expression: &module::TupleExpr,
    proof: &UninhabitedTupleValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    use execution::NeverExprKind as E;
    use module::TupleExprKind as M;

    let kind = match expression.kind() {
        M::Value(values) => {
            return diverging_values(values, proof.diverging(), context);
        }
        M::Constant(_) | M::LocalGet { .. } => Representability::Uninhabited,
        M::Call { function, args } => {
            return super::direct_call(function, args, context, |function, context| {
                context.never_function_id(function)
            })
            .map(|call| match call {
                execution::DirectCall::Executable { function, args } => {
                    execution::NeverExpr::from_kind(E::Call { function, args })
                }
                execution::DirectCall::Diverging(expression) => expression,
            });
        }
        M::FunctionCall { function, args } => {
            return super::function_call(
                args,
                context,
                |context| super::function::tuple_never_function_expr(function, context),
                |context| super::function::evaluated_tuple_function_expr(function, context),
            )
            .map(|call| match call {
                execution::FunctionCall::Executable { function, args } => {
                    execution::NeverExpr::from_kind(E::FunctionCall { function, args })
                }
                execution::FunctionCall::Diverging(expression) => expression,
            });
        }
        M::TupleIndex { tuple, index: _ } => {
            return tuple_inhabitation(tuple, context)
                .and_then(|proof| tuple_never_expr(tuple, &proof, context));
        }
        M::CustomField(access) => {
            return custom_inhabitation(access.source(), context)
                .and_then(|proof| custom_never_expr(access.source(), &proof, context));
        }
        M::ListIndex { .. } => Representability::Uninhabited,
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| tuple_never_expr(true_, proof, context),
            |context| tuple_never_expr(false_, proof, context),
            execution::NeverExpr::into_kind,
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
        } => super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_expr(branch, proof, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                tuple_never_expr(fallback, proof, context).map(|fallback| E::IntCase {
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
        } => super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_expr(branch, proof, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                tuple_never_expr(fallback, proof, context).map(|fallback| E::StringCase {
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
        } => super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_expr(branch, proof, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                tuple_never_expr(fallback, proof, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            return super::super::step::steps_until_never(steps, context).and_then(|steps| {
                match steps {
                    super::super::step::StepsUntilNever::Complete(steps) => {
                        tuple_never_expr(return_, proof, context).map(|return_| {
                            execution::NeverExpr::from_kind(E::Block {
                                steps,
                                return_: Box::new(return_),
                            })
                        })
                    }
                    super::super::step::StepsUntilNever::Diverging { prefix, expression } => {
                        Representability::Inhabited(execution::NeverExpr::from_kind(E::Block {
                            steps: prefix,
                            return_: Box::new(expression),
                        }))
                    }
                }
            });
        }
    };

    kind.map(execution::NeverExpr::from_kind)
}

pub(in crate::plan::execution::lowering) fn custom_never_expr(
    expression: &module::CustomExpr,
    proof: &UninhabitedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    custom_never_expr_kind(expression.kind(), proof, context)
}

pub(in crate::plan::execution::lowering) fn custom_never_expr_kind(
    kind: &module::CustomExprKind,
    proof: &UninhabitedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    use execution::NeverExprKind as E;
    use module::CustomExprKind as M;

    let kind = match kind {
        M::Constructor(construction) => {
            return diverging_values(
                construction.fields(),
                proof.diverging_field(construction.constructor().index()),
                context,
            );
        }
        M::Constant(_) | M::LocalGet { .. } => Representability::Uninhabited,
        M::Call { function, args } => {
            return super::direct_call(function, args, context, |function, context| {
                context.never_function_id(function)
            })
            .map(|call| match call {
                execution::DirectCall::Executable { function, args } => {
                    execution::NeverExpr::from_kind(E::Call { function, args })
                }
                execution::DirectCall::Diverging(expression) => expression,
            });
        }
        M::FunctionCall(call) => {
            return super::function_call(
                call.arguments(),
                context,
                |context| super::function::custom_never_function_expr(call.function(), context),
                |context| super::function::evaluated_custom_function_expr(call.function(), context),
            )
            .map(|call| match call {
                execution::FunctionCall::Executable { function, args } => {
                    execution::NeverExpr::from_kind(E::FunctionCall { function, args })
                }
                execution::FunctionCall::Diverging(expression) => expression,
            });
        }
        M::TupleIndex { tuple, index: _ } => {
            return tuple_inhabitation(tuple, context)
                .and_then(|proof| tuple_never_expr(tuple, &proof, context));
        }
        M::CustomField(access) => {
            return custom_inhabitation(access.source(), context)
                .and_then(|proof| custom_never_expr(access.source(), &proof, context));
        }
        M::ListIndex { .. } => Representability::Uninhabited,
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| custom_never_expr_kind(true_, proof, context),
            |context| custom_never_expr_kind(false_, proof, context),
            execution::NeverExpr::into_kind,
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
        } => super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                custom_never_expr_kind(branch, proof, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_never_expr_kind(fallback, proof, context).map(|fallback| E::IntCase {
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
        } => super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                custom_never_expr_kind(branch, proof, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_never_expr_kind(fallback, proof, context).map(|fallback| E::StringCase {
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
        } => super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                custom_never_expr_kind(branch, proof, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                custom_never_expr_kind(fallback, proof, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            return super::super::step::steps_until_never(steps, context).and_then(|steps| {
                match steps {
                    super::super::step::StepsUntilNever::Complete(steps) => {
                        custom_never_expr_kind(return_, proof, context).map(|return_| {
                            execution::NeverExpr::from_kind(E::Block {
                                steps,
                                return_: Box::new(return_),
                            })
                        })
                    }
                    super::super::step::StepsUntilNever::Diverging { prefix, expression } => {
                        Representability::Inhabited(execution::NeverExpr::from_kind(E::Block {
                            steps: prefix,
                            return_: Box::new(expression),
                        }))
                    }
                }
            });
        }
    };

    kind.map(execution::NeverExpr::from_kind)
}

pub(in crate::plan::execution::lowering) fn tuple_inhabitation(
    expression: &module::TupleExpr,
    context: &super::super::LoweringContext,
) -> Representability<UninhabitedTupleValueShape> {
    let elements = expression
        .shape()
        .iter()
        .map(|shape| context.concrete_value_shape(shape))
        .collect::<Vec<_>>();
    match context.representations.tuple_inhabitation(&elements) {
        CompoundInhabitation::Inhabited => Representability::Uninhabited,
        CompoundInhabitation::Uninhabited(proof) => Representability::Inhabited(proof),
    }
}

pub(in crate::plan::execution::lowering) fn custom_inhabitation(
    expression: &module::CustomExpr,
    context: &super::super::LoweringContext,
) -> Representability<UninhabitedCustomValueShape> {
    let shape = context.concrete_custom_value_shape(expression.shape());
    match context.representations.custom_inhabitation(&shape) {
        CompoundInhabitation::Inhabited => Representability::Uninhabited,
        CompoundInhabitation::Uninhabited(proof) => Representability::Inhabited(proof),
    }
}

fn diverging_values(
    values: &[module::Expr],
    diverging: usize,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    let mut prefix = Vec::with_capacity(diverging);
    for (index, value) in values.iter().enumerate() {
        if index == diverging {
            let shape = context.concrete_value_shape(value.value_shape());
            return match context.representations.inhabitation(&shape) {
                ValueInhabitation::Inhabited(_) => Representability::Uninhabited,
                ValueInhabitation::Uninhabited(proof) => uninhabited_expr(value, &proof, context)
                    .map(|diverging| {
                        execution::NeverExpr::from_kind(execution::NeverExprKind::Values {
                            prefix,
                            diverging: Box::new(diverging),
                        })
                    }),
            };
        }
        match super::expr(value, context) {
            Representability::Inhabited(value) => prefix.push(value),
            Representability::Uninhabited => return Representability::Uninhabited,
        }
    }
    Representability::Uninhabited
}
