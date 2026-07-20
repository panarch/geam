use super::super::super::specialization::Representability;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn tuple_never_function_expr(
    expression: &module::TupleFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExpr> {
    use execution::NeverFunctionExprKind as E;
    use module::TupleFunctionExprKind as M;

    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.type_().clone(),
    ));
    let type_ = context.generic_function_type(&shape);
    let kind = match expression.kind() {
        M::Constant(value) => context
            .tuple_never_function_constant(value)
            .map(E::Constant),
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.never_function_id(function)
            })
            .map(E::Reference)
        }
        M::Closure {
            function,
            params,
            captures,
            return_type: _,
        } => super::closure_template(function, params, captures, context, |function, context| {
            context.never_function_id(function)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::NeverFunctionLocal::new(
                execution::NeverFunctionLocalId(context.mapped_local(
                    super::super::super::frame::LocalKind::TupleFunction,
                    local.0,
                )),
                type_.clone(),
            ),
        }),
        M::Call {
            function,
            args,
            type_: _,
        } => super::super::direct_call(function, args, context, |function, context| {
            context.never_function_function_id(function, type_.clone())
        })
        .map(E::Call),
        M::FunctionCall {
            function,
            args,
            type_: _,
        } => super::super::function_call(
            args,
            context,
            |context| super::function_function_expr(function, context),
            |context| super::evaluated_function_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex {
            tuple,
            index,
            type_: _,
        } => super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex {
            list,
            index,
            type_: _,
        } => super::super::function_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(panic) => super::super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case_into(
            subject,
            context,
            |context| tuple_never_function_expr(true_, context),
            |context| tuple_never_function_expr(false_, context),
            execution::NeverFunctionExpr::into_kind,
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_.into_kind()),
                false_: Box::new(false_.into_kind()),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                tuple_never_function_expr(fallback, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                tuple_never_function_expr(fallback, context).map(|fallback| E::StringCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                tuple_never_function_expr(branch, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                tuple_never_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                tuple_never_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_.into_kind()),
                })
            })
        }
    };

    kind.map(|kind| execution::NeverFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn custom_never_function_expr(
    expression: &module::CustomFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExpr> {
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.custom_function_type().to_function_type(),
    ));
    let type_ = context.generic_function_type(&shape);
    custom_never_function_expr_kind(expression.kind(), &type_, context)
        .map(|kind| execution::NeverFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn custom_never_function_expr_kind(
    kind: &module::CustomFunctionExprKind,
    type_: &execution::GenericFunctionType,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExprKind> {
    use execution::NeverFunctionExprKind as E;
    use module::CustomFunctionExprKind as M;

    match kind {
        M::Constant(value) => context
            .custom_never_function_constant(value)
            .map(E::Constant),
        M::Constructor(_) => Representability::Uninhabited,
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.never_function_id(function)
            })
            .map(E::Reference)
        }
        M::Closure {
            function,
            params,
            captures,
        } => super::closure_template(function, params, captures, context, |function, context| {
            context.never_function_id(function)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::NeverFunctionLocal::new(
                execution::NeverFunctionLocalId(context.mapped_local(
                    super::super::super::frame::LocalKind::CustomFunction,
                    local.id().0,
                )),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.never_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::super::function_call(
            args,
            context,
            |context| super::function_function_expr(function, context),
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
            |context| custom_never_function_expr_kind(true_, type_, context),
            |context| custom_never_function_expr_kind(false_, type_, context),
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
                custom_never_function_expr_kind(branch, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_never_function_expr_kind(fallback, type_, context).map(|fallback| {
                    E::IntCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                custom_never_function_expr_kind(branch, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                custom_never_function_expr_kind(fallback, type_, context).map(|fallback| {
                    E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                custom_never_function_expr_kind(branch, type_, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                custom_never_function_expr_kind(fallback, type_, context).map(|fallback| {
                    E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                custom_never_function_expr_kind(return_, type_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_never_function_expr(
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExpr> {
    use execution::NeverFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.generic_function_type(&shape);
    let kind = match expression.kind() {
        M::Constant(value) => context
            .generic_never_function_constant(value)
            .map(E::Constant),
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.never_function_id(function)
            })
            .map(E::Reference)
        }
        M::Closure {
            function,
            params,
            captures,
        } => super::closure_template(function, params, captures, context, |function, context| {
            context.never_function_id(function)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::NeverFunctionLocal::new(
                execution::NeverFunctionLocalId(context.generic_function_local_index(local.id())),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.never_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::super::function_call(
            args,
            context,
            |context| super::function_function_expr(function, context),
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
        } => super::super::bool_case_into(
            subject,
            context,
            |context| generic_never_function_expr(true_, context),
            |context| generic_never_function_expr(false_, context),
            execution::NeverFunctionExpr::into_kind,
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_.into_kind()),
                false_: Box::new(false_.into_kind()),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_never_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_never_function_expr(fallback, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_never_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_never_function_expr(fallback, context).map(|fallback| E::StringCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_never_function_expr(branch, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_never_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                generic_never_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_.into_kind()),
                })
            })
        }
    };

    kind.map(|kind| execution::NeverFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_value_never_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExpr> {
    use execution::NeverFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.generic_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        type_.clone(),
                    ),
                })
        }
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.never_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::evaluated_generic_function_expr(function, context),
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
            super::super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                }
            })
        }
        M::Panic(panic) => super::super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case_into(
            subject,
            context,
            |context| generic_value_never_function_expr(true_, function_shape, context),
            |context| generic_value_never_function_expr(false_, function_shape, context),
            execution::NeverFunctionExpr::into_kind,
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_.into_kind()),
                false_: Box::new(false_.into_kind()),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_never_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_never_function_expr(fallback, function_shape, context).map(
                    |fallback| E::IntCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
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
                generic_value_never_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_never_function_expr(fallback, function_shape, context).map(
                    |fallback| E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
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
                generic_value_never_function_expr(branch, function_shape, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_never_function_expr(fallback, function_shape, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_never_function_expr(return_, function_shape, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_.into_kind()),
                    }
                })
            })
        }
    };

    kind.map(|kind| execution::NeverFunctionExpr::from_parts(type_, kind))
}
