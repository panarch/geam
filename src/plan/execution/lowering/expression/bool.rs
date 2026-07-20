use super::super::specialization::Representability;
use super::{
    bool_function_expr, bool_list_expr, custom_expr, custom_field_access, expr, float_expr,
    int_expr, list_expr, panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bool_expr(
    expression: &module::BoolExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::BoolExpr> {
    use execution::BoolExprKind as E;
    use module::BoolExprKind as M;

    let kind = match expression.kind() {
        M::Value(value) => Representability::Inhabited(E::Value(*value)),
        M::Constant(reference) => context.bool_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::BoolLocalId(
                context.mapped_local(super::super::frame::LocalKind::Bool, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.bool_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| bool_function_expr(function, context),
            |context| super::function::evaluated_bool_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => bool_list_expr(list, context).map(|list| E::ListIndex {
            list: Box::new(list),
            index: *index,
        }),
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::Not(value) => bool_expr(value, context).map(|value| E::Not(Box::new(value))),
        M::LtInt { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::LtInt {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::LtEqInt { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::LtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::GtInt { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::GtInt {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::GtEqInt { left, right } => {
            int_expr(left, context).zip_with(int_expr(right, context), |left, right| E::GtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::LtFloat { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| {
                E::LtFloat {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
        }
        M::LtEqFloat { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| {
                E::LtEqFloat {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
        }
        M::GtFloat { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| {
                E::GtFloat {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
        }
        M::GtEqFloat { left, right } => {
            float_expr(left, context).zip_with(float_expr(right, context), |left, right| {
                E::GtEqFloat {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
        }
        M::Equal { left, right } => {
            expr(left, context).zip_with(expr(right, context), |left, right| E::Equal {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::NotEqual { left, right } => {
            expr(left, context).zip_with(expr(right, context), |left, right| E::NotEqual {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::StringStartsWith { value, prefix } => {
            string_expr(value, context).map(|value| E::StringStartsWith {
                value: Box::new(value),
                prefix: prefix.clone(),
            })
        }
        M::ListLengthEquals { value, length } => {
            list_expr(value, context).map(|value| match value {
                execution::ListExpr::Parameter(_) => E::Value(*length == 0),
                value => E::ListLengthEquals {
                    value: Box::new(value),
                    length: *length,
                },
            })
        }
        M::ListLengthAtLeast { value, length } => {
            list_expr(value, context).map(|value| match value {
                execution::ListExpr::Parameter(_) => E::Value(*length == 0),
                value => E::ListLengthAtLeast {
                    value: Box::new(value),
                    length: *length,
                },
            })
        }
        M::BitArrayMatches { value, pattern } => {
            let pattern = super::super::pattern::bit_array_pattern(pattern, context);
            super::bit_array_expr(value, context).map(|value| E::BitArrayMatches {
                value: Box::new(value),
                pattern,
            })
        }
        M::CustomMatches { value, pattern } => {
            let custom_pattern = custom_pattern(pattern);
            let constructor_match = custom_pattern.map(|pattern| {
                let source = context.concrete_custom_value_shape(value.shape());
                context
                    .representations
                    .custom_constructor_match(&source, pattern.constructor().index())
            });
            match constructor_match {
                Some(super::super::specialization::CustomConstructorMatch::Impossible) => {
                    custom_expr(value, context).map(|_| E::Value(false))
                }
                Some(super::super::specialization::CustomConstructorMatch::Certain) => {
                    if let Some((constructor, fields)) = custom_pattern.and_then(|pattern| {
                        pattern
                            .total_fields()
                            .map(|fields| (pattern.constructor(), fields))
                    }) {
                        custom_expr(value, context).and_then(|value| {
                            super::super::step::custom_field_binding_pattern(
                                constructor,
                                fields,
                                context,
                            )
                            .map(|pattern| E::Block {
                                steps: vec![execution::Step::from_kind(
                                    execution::StepKind::BindCustomValueFields { value, pattern },
                                )],
                                return_: Box::new(execution::BoolExpr::from_kind(E::Value(true))),
                            })
                        })
                    } else {
                        custom_expr(value, context).map(|value| E::CustomMatches {
                            value: Box::new(value),
                            pattern: Box::new(super::super::step::assert_pattern(pattern, context)),
                        })
                    }
                }
                Some(super::super::specialization::CustomConstructorMatch::Dynamic) | None => {
                    custom_expr(value, context).map(|value| E::CustomMatches {
                        value: Box::new(value),
                        pattern: Box::new(super::super::step::assert_pattern(pattern, context)),
                    })
                }
            }
        }
        M::And { left, right } => {
            bool_expr(left, context).zip_with(bool_expr(right, context), |left, right| E::And {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
        M::Or { left, right } => {
            bool_expr(left, context).zip_with(bool_expr(right, context), |left, right| E::Or {
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
            |context| bool_expr(true_, context),
            |context| bool_expr(false_, context),
            execution::BoolExpr::into_kind,
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
                bool_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                bool_expr(fallback, context).map(|fallback| E::IntCase {
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
                bool_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                bool_expr(fallback, context).map(|fallback| E::StringCase {
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
                bool_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                bool_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                bool_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(execution::BoolExpr::from_kind)
}

fn custom_pattern(pattern: &module::AssertPattern) -> Option<&crate::plan::CustomPattern> {
    match pattern {
        module::AssertPattern::Custom(pattern) => Some(pattern),
        _ => None,
    }
}
