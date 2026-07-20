use super::super::super::specialization::Representability;
use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn list_function_expr(
    expression: &module::ListFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::ListFunctionExpr> {
    use execution::ListFunctionExprKind as E;
    use module::ListFunctionExprKind as M;

    let function_shape = context.concrete_function_shape(
        &crate::plan::FunctionShape::from_function_type(expression.type_().clone()),
    );
    let item_shape = context.concrete_value_shape(&crate::plan::ValueShape::from_value_type(
        expression.return_item_type(),
    ));
    let kind = match expression.kind() {
        M::Constant(value) => context.list_function_constant(value).map(E::Constant),
        M::Reference(value) => super::function_reference(value, context, |function, context| {
            context.list_function_id(function, &item_shape)
        })
        .map(E::Reference),
        M::Closure {
            function,
            params,
            captures,
        } => super::closure_template(function, params, captures, context, |function, context| {
            context.list_function_id(function, &item_shape)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: crate::plan::execution::lowering::id::list_function_local(local, context),
        }),
        M::Call {
            function,
            args,
            type_: _,
        } => super::super::direct_call(function, args, context, |function, context| {
            context.list_function_function_id(function, &function_shape, &item_shape)
        })
        .map(E::Call),
        M::FunctionCall {
            function,
            args,
            type_: _,
        } => super::super::function_call(
            args,
            context,
            |context| function_function_expr(function, context),
            |context| super::evaluated_function_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex {
            tuple,
            index,
            type_,
        } => super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
            type_: context.function_type(type_.clone()),
        }),
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex { list, index, type_ } => {
            super::super::function_list_expr(list, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
                type_: context.function_type(type_.clone()),
            })
        }
        M::Panic(value) => super::super::panic_expr(value, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case_into(
            subject,
            context,
            |context| list_function_expr(true_, context),
            |context| list_function_expr(false_, context),
            execution::ListFunctionExpr::into_kind,
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
                list_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                list_function_expr(fallback, context).map(|fallback| E::IntCase {
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
        } => super::super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                list_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                list_function_expr(fallback, context).map(|fallback| E::StringCase {
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
        } => super::super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                list_function_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                list_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            crate::plan::execution::lowering::step::steps(steps, context).and_then(|steps| {
                list_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };

    kind.map(execution::ListFunctionExpr::from_kind)
}

pub(in crate::plan::execution::lowering) fn generic_list_function_expr(
    expression: &module::GenericFunctionExpr,
    item_shape: &super::super::super::specialization::SpecializedValueShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::ListFunctionExpr> {
    use execution::ListFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let function_shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.lower_concrete_function_type(&function_shape);
    let kind = match expression.kind() {
        M::Constant(value) => context
            .generic_list_function_constant(value, item_shape)
            .map(E::Constant),
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.list_function_id(function, item_shape)
            })
            .map(E::Reference)
        }
        M::Closure {
            function,
            params,
            captures,
        } => super::closure_template(function, params, captures, context, |function, context| {
            context.list_function_id(function, item_shape)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: super::super::super::frame::generic_list_returning_function_local(
                local, item_shape, context,
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.list_function_function_id(function, &function_shape, item_shape)
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
                type_: type_.clone(),
            })
        }
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex { list, index } => {
            super::super::function_list_expr(list, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
                type_: type_.clone(),
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
            |context| generic_list_function_expr(true_, item_shape, context),
            |context| generic_list_function_expr(false_, item_shape, context),
            execution::ListFunctionExpr::into_kind,
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
                generic_list_function_expr(branch, item_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_list_function_expr(fallback, item_shape, context).map(|fallback| {
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
                generic_list_function_expr(branch, item_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_list_function_expr(fallback, item_shape, context).map(|fallback| {
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
                generic_list_function_expr(branch, item_shape, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_list_function_expr(fallback, item_shape, context).map(|fallback| {
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
                generic_list_function_expr(return_, item_shape, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };

    kind.map(execution::ListFunctionExpr::from_kind)
}
