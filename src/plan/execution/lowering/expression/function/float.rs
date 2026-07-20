use super::super::super::specialization::Representability;
use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn float_function_expr(
    expression: &module::FloatFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::FloatFunctionExpr> {
    use execution::FloatFunctionExprKind as E;
    use module::FloatFunctionExprKind as M;

    let kind = match expression.kind() {
        M::Constant(value) => context.float_function_constant(value).map(E::Constant),
        M::Reference(value) => {
            super::function_reference(value, context, |id, context| context.float_function_id(id))
                .map(E::Reference)
        }
        M::Closure {
            function,
            params,
            captures,
        } => super::closure_template(function, params, captures, context, |id, context| {
            context.float_function_id(id)
        })
        .map(E::Closure),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::FloatFunctionLocalId(context.mapped_local(
                super::super::super::frame::LocalKind::FloatFunction,
                local.0,
            )),
        }),
        M::Call {
            function,
            args,
            type_: _,
        } => super::super::direct_call(function, args, context, |function, context| {
            context.float_function_function_id(function)
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
            |context| float_function_expr(true_, context),
            |context| float_function_expr(false_, context),
            execution::FloatFunctionExpr::into_kind,
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
                float_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                float_function_expr(fallback, context).map(|fallback| E::IntCase {
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
                float_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                float_function_expr(fallback, context).map(|fallback| E::StringCase {
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
                float_function_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                float_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            crate::plan::execution::lowering::step::steps(steps, context).and_then(|steps| {
                float_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };

    kind.map(execution::FloatFunctionExpr::from_kind)
}
