use super::super::specialization::Representability;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn generic_expr(
    expression: &module::GenericExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::Expr> {
    use super::super::specialization::SpecializedValueShape as S;

    let kind = match context.concrete_parameter(expression.parameter()) {
        S::Parameter(_) => never_expr(expression, context).map(execution::ExprKind::Never),
        S::Int => generic_int_expr(expression, context).map(execution::ExprKind::Int),
        S::Float => generic_float_expr(expression, context).map(execution::ExprKind::Float),
        S::String => generic_string_expr(expression, context).map(execution::ExprKind::String),
        S::BitArray => {
            generic_bit_array_expr(expression, context).map(execution::ExprKind::BitArray)
        }
        S::UtfCodepoint => {
            generic_utf_codepoint_expr(expression, context).map(execution::ExprKind::UtfCodepoint)
        }
        S::Bool => generic_bool_expr(expression, context).map(execution::ExprKind::Bool),
        S::Nil => generic_nil_expr(expression, context).map(execution::ExprKind::Nil),
        S::Tuple(elements) => {
            generic_tuple_expr(expression, &elements, context).map(execution::ExprKind::Tuple)
        }
        S::List(item) => {
            generic_list_value_expr(expression, &item, context).map(execution::ExprKind::List)
        }
        S::Function(function) => generic_function_value_expr(expression, &function, context)
            .map(execution::ExprKind::Function),
        S::Custom(shape) => {
            generic_custom_expr(expression, &shape, context).map(execution::ExprKind::Custom)
        }
    };

    kind.map(execution::Expr::from_kind)
}

pub(in crate::plan::execution::lowering) fn never_expr(
    expression: &module::GenericExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverExpr> {
    use execution::NeverExprKind as E;
    use module::GenericExprKind as M;

    let kind = match expression.kind() {
        M::LocalGet { .. } => Representability::Uninhabited,
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
                |context| super::function::generic_never_function_expr(function, context),
                |context| super::function::evaluated_generic_function_expr(function, context),
            )
            .map(|call| match call {
                execution::FunctionCall::Executable { function, args } => {
                    execution::NeverExpr::from_kind(E::FunctionCall { function, args })
                }
                execution::FunctionCall::Diverging(expression) => expression,
            });
        }
        M::TupleIndex { tuple, index: _ } => {
            return super::tuple_inhabitation(tuple, context)
                .and_then(|proof| super::tuple_never_expr(tuple, &proof, context));
        }
        M::CustomField(access) => {
            return super::custom_inhabitation(access.source(), context)
                .and_then(|proof| super::custom_never_expr(access.source(), &proof, context));
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
            |context| never_expr(true_, context),
            |context| never_expr(false_, context),
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
                never_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                never_expr(fallback, context).map(|fallback| E::IntCase {
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
                never_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                never_expr(fallback, context).map(|fallback| E::StringCase {
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
                never_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                never_expr(fallback, context).map(|fallback| E::FloatCase {
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
                        never_expr(return_, context).map(|return_| {
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

macro_rules! primitive_generic_expr {
    (
        $lower:ident,
        $expression:ident,
        $kind:ident,
        $local:ident,
        $function_id:ident,
        $function_expr:ident,
        $list_expr:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericExpr,
            context: &mut super::super::LoweringContext,
        ) -> Representability<execution::$expression> {
            use execution::$kind as E;
            use module::GenericExprKind as M;

            let kind = match expression.kind() {
                M::LocalGet { local, name: _ } => {
                    context
                        .generic_local_index(local.id())
                        .map(|index| E::LocalGet {
                            local: execution::$local(index),
                        })
                }
                M::Call { function, args } => {
                    super::direct_call(function, args, context, |function, context| {
                        context.$function_id(function)
                    })
                    .map(E::Call)
                }
                M::FunctionCall { function, args } => super::function_call(
                    args,
                    context,
                    |context| super::$function_expr(function, context),
                    |context| super::function::evaluated_generic_function_expr(function, context),
                )
                .map(E::FunctionCall),
                M::TupleIndex { tuple, index } => {
                    super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                        tuple: Box::new(tuple),
                        index: *index,
                    })
                }
                M::CustomField(access) => {
                    super::custom_field_access(access, context).map(E::CustomField)
                }
                M::ListIndex { list, index } => {
                    super::$list_expr(list, context).map(|list| E::ListIndex {
                        list: Box::new(list),
                        index: *index,
                    })
                }
                M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => super::bool_case_into(
                    subject,
                    context,
                    |context| $lower(true_, context),
                    |context| $lower(false_, context),
                    execution::$expression::into_kind,
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
                        $lower(branch, context).map(|branch| (pattern.clone(), branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, context).map(|fallback| E::IntCase {
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
                        $lower(branch, context).map(|branch| (pattern.clone(), branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, context).map(|fallback| E::StringCase {
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
                        $lower(branch, context).map(|branch| (*pattern, branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, context).map(|fallback| E::FloatCase {
                            subject: Box::new(subject),
                            clauses,
                            fallback: Box::new(fallback),
                        })
                    })
                }),
                M::Block { steps, return_ } => {
                    super::super::step::steps(steps, context).and_then(|steps| {
                        $lower(return_, context).map(|return_| E::Block {
                            steps,
                            return_: Box::new(return_),
                        })
                    })
                }
            };
            kind.map(execution::$expression::from_kind)
        }
    };
}

macro_rules! primitive_generic_function_value_expr {
    (
        $lower:ident,
        $expression:ident,
        $kind:ident,
        $local:ident,
        $function_function_id:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericExpr,
            function_shape: &super::super::specialization::SpecializedFunctionShape,
            context: &mut super::super::LoweringContext,
        ) -> Representability<execution::$expression> {
            use execution::$kind as E;
            use module::GenericExprKind as M;

            let type_ = context.lower_concrete_function_type(function_shape);
            let kind = match expression.kind() {
                M::LocalGet { local, name: _ } => {
                    context
                        .generic_local_index(local.id())
                        .map(|index| E::LocalGet {
                            local: execution::$local(index),
                        })
                }
                M::Call { function, args } => {
                    super::direct_call(function, args, context, |function, context| {
                        context.$function_function_id(function)
                    })
                    .map(E::Call)
                }
                M::FunctionCall { function, args } => super::function_call(
                    args,
                    context,
                    |context| {
                        super::generic_function_function_expr(function, function_shape, context)
                    },
                    |context| super::function::evaluated_generic_function_expr(function, context),
                )
                .map(E::FunctionCall),
                M::TupleIndex { tuple, index } => {
                    super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                        tuple: Box::new(tuple),
                        index: *index,
                        type_: type_.clone(),
                    })
                }
                M::CustomField(access) => {
                    super::custom_field_access(access, context).map(E::CustomField)
                }
                M::ListIndex { list, index } => {
                    super::generic_function_list_expr(list, function_shape, context).map(|list| {
                        E::ListIndex {
                            list: Box::new(list),
                            index: *index,
                            type_: type_.clone(),
                        }
                    })
                }
                M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => super::bool_case_into(
                    subject,
                    context,
                    |context| $lower(true_, function_shape, context),
                    |context| $lower(false_, function_shape, context),
                    execution::$expression::into_kind,
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
                        $lower(branch, function_shape, context)
                            .map(|branch| (pattern.clone(), branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, function_shape, context).map(|fallback| E::IntCase {
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
                        $lower(branch, function_shape, context)
                            .map(|branch| (pattern.clone(), branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, function_shape, context).map(|fallback| E::StringCase {
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
                        $lower(branch, function_shape, context).map(|branch| (*pattern, branch))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, function_shape, context).map(|fallback| E::FloatCase {
                            subject: Box::new(subject),
                            clauses,
                            fallback: Box::new(fallback),
                        })
                    })
                }),
                M::Block { steps, return_ } => {
                    super::super::step::steps(steps, context).and_then(|steps| {
                        $lower(return_, function_shape, context).map(|return_| E::Block {
                            steps,
                            return_: Box::new(return_),
                        })
                    })
                }
            };

            kind.map(execution::$expression::from_kind)
        }
    };
}

primitive_generic_expr!(
    generic_int_expr,
    IntExpr,
    IntExprKind,
    IntLocalId,
    int_function_id,
    generic_int_function_expr,
    generic_int_list_expr
);
primitive_generic_expr!(
    generic_float_expr,
    FloatExpr,
    FloatExprKind,
    FloatLocalId,
    float_function_id,
    generic_float_function_expr,
    generic_float_list_expr
);
primitive_generic_expr!(
    generic_string_expr,
    StringExpr,
    StringExprKind,
    StringLocalId,
    string_function_id,
    generic_string_function_expr,
    generic_string_list_expr
);
primitive_generic_expr!(
    generic_bit_array_expr,
    BitArrayExpr,
    BitArrayExprKind,
    BitArrayLocalId,
    bit_array_function_id,
    generic_bit_array_function_expr,
    generic_bit_array_list_expr
);
primitive_generic_expr!(
    generic_utf_codepoint_expr,
    UtfCodepointExpr,
    UtfCodepointExprKind,
    UtfCodepointLocalId,
    utf_codepoint_function_id,
    generic_utf_codepoint_function_expr,
    generic_utf_codepoint_list_expr
);
primitive_generic_expr!(
    generic_bool_expr,
    BoolExpr,
    BoolExprKind,
    BoolLocalId,
    bool_function_id,
    generic_bool_function_expr,
    generic_bool_list_expr
);
primitive_generic_expr!(
    generic_nil_expr,
    NilExpr,
    NilExprKind,
    NilLocalId,
    nil_function_id,
    generic_nil_function_expr,
    generic_nil_list_expr
);

primitive_generic_function_value_expr!(
    generic_value_int_function_expr,
    IntFunctionExpr,
    IntFunctionExprKind,
    IntFunctionLocalId,
    int_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_float_function_expr,
    FloatFunctionExpr,
    FloatFunctionExprKind,
    FloatFunctionLocalId,
    float_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_string_function_expr,
    StringFunctionExpr,
    StringFunctionExprKind,
    StringFunctionLocalId,
    string_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_bit_array_function_expr,
    BitArrayFunctionExpr,
    BitArrayFunctionExprKind,
    BitArrayFunctionLocalId,
    bit_array_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_utf_codepoint_function_expr,
    UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind,
    UtfCodepointFunctionLocalId,
    utf_codepoint_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_bool_function_expr,
    BoolFunctionExpr,
    BoolFunctionExprKind,
    BoolFunctionLocalId,
    bool_function_function_id
);
primitive_generic_function_value_expr!(
    generic_value_nil_function_expr,
    NilFunctionExpr,
    NilFunctionExprKind,
    NilFunctionLocalId,
    nil_function_function_id
);

pub(in crate::plan::execution::lowering) fn generic_value_tuple_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::TupleFunctionExpr> {
    use execution::TupleFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.lower_concrete_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::TupleFunctionLocalId(index),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.tuple_function_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
                type_: type_.clone(),
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                    type_: type_.clone(),
                }
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| generic_value_tuple_function_expr(true_, function_shape, context),
            |context| generic_value_tuple_function_expr(false_, function_shape, context),
            execution::TupleFunctionExpr::into_kind,
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
                generic_value_tuple_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_tuple_function_expr(fallback, function_shape, context).map(
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
        } => super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_tuple_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_tuple_function_expr(fallback, function_shape, context).map(
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
        } => super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_tuple_function_expr(branch, function_shape, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_value_tuple_function_expr(fallback, function_shape, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_tuple_function_expr(return_, function_shape, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_),
                    }
                })
            })
        }
    };

    kind.map(|kind| execution::TupleFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_value_custom_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    return_shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomFunctionExpr> {
    let type_ = context.specialized_custom_function_type(function_shape.arguments(), return_shape);
    generic_value_custom_function_expr_kind(expression, function_shape, &type_, context)
        .map(|kind| execution::CustomFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_value_custom_function_expr_kind(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    type_: &execution::CustomFunctionType,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomFunctionExprKind> {
    use execution::CustomFunctionExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::CustomFunctionLocal::new(
                        execution::CustomFunctionLocalId(index),
                        type_.clone(),
                    ),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.custom_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                }
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| {
                generic_value_custom_function_expr_kind(true_, function_shape, type_, context)
            },
            |context| {
                generic_value_custom_function_expr_kind(false_, function_shape, type_, context)
            },
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
                generic_value_custom_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_custom_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::IntCase {
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
                generic_value_custom_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_custom_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::StringCase {
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
                generic_value_custom_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_value_custom_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_custom_function_expr_kind(return_, function_shape, type_, context)
                    .map(|return_| E::Block {
                        steps,
                        return_: Box::new(return_),
                    })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_value_list_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    item_shape: &super::super::specialization::SpecializedValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::ListFunctionExpr> {
    use execution::ListFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.lower_concrete_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: super::super::frame::list_function_local_at(
                        item_shape,
                        type_.clone(),
                        index,
                        context,
                    ),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.list_function_function_id(function, function_shape, item_shape)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
                type_: type_.clone(),
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                    type_: type_.clone(),
                }
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| generic_value_list_function_expr(true_, function_shape, item_shape, context),
            |context| generic_value_list_function_expr(false_, function_shape, item_shape, context),
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
        } => super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_list_function_expr(branch, function_shape, item_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_list_function_expr(fallback, function_shape, item_shape, context).map(
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
        } => super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_list_function_expr(branch, function_shape, item_shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_list_function_expr(fallback, function_shape, item_shape, context).map(
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
        } => super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_list_function_expr(branch, function_shape, item_shape, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_value_list_function_expr(fallback, function_shape, item_shape, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_list_function_expr(return_, function_shape, item_shape, context).map(
                    |return_| E::Block {
                        steps,
                        return_: Box::new(return_),
                    },
                )
            })
        }
    };

    kind.map(execution::ListFunctionExpr::from_kind)
}

pub(in crate::plan::execution::lowering) fn generic_value_function_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    return_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExpr> {
    let type_ =
        context.specialized_function_function_type(function_shape.arguments(), return_shape);
    generic_value_function_function_expr_kind(expression, function_shape, &type_, context)
        .map(|kind| execution::FunctionFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_value_function_function_expr_kind(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionFunctionExprKind> {
    use execution::FunctionFunctionExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::FunctionFunctionLocal::new(
                        execution::FunctionFunctionLocalId(index),
                        type_.clone(),
                    ),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.function_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                }
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| {
                generic_value_function_function_expr_kind(true_, function_shape, type_, context)
            },
            |context| {
                generic_value_function_function_expr_kind(false_, function_shape, type_, context)
            },
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
                generic_value_function_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_function_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::IntCase {
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
                generic_value_function_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_function_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::StringCase {
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
                generic_value_function_function_expr_kind(branch, function_shape, type_, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_value_function_function_expr_kind(fallback, function_shape, type_, context)
                    .map(|fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_function_function_expr_kind(return_, function_shape, type_, context)
                    .map(|return_| E::Block {
                        steps,
                        return_: Box::new(return_),
                    })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_value_generic_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    use execution::GenericFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.generic_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(index),
                        type_.clone(),
                    ),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.generic_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_function_function_expr(function, function_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_function_list_expr(list, function_shape, context).map(|list| {
                E::ListIndex {
                    list: Box::new(list),
                    index: *index,
                }
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| generic_value_generic_function_expr(true_, function_shape, context),
            |context| generic_value_generic_function_expr(false_, function_shape, context),
            execution::GenericFunctionExpr::into_kind,
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
        } => super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_generic_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_generic_function_expr(fallback, function_shape, context).map(
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
        } => super::string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_generic_function_expr(branch, function_shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_generic_function_expr(fallback, function_shape, context).map(
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
        } => super::float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_value_generic_function_expr(branch, function_shape, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_value_generic_function_expr(fallback, function_shape, context).map(
                    |fallback| E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
                    },
                )
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_generic_function_expr(return_, function_shape, context).map(
                    |return_| E::Block {
                        steps,
                        return_: Box::new(return_.into_kind()),
                    },
                )
            })
        }
    };

    kind.map(|kind| execution::GenericFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_function_value_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    use super::super::specialization::{FunctionRepresentation, StoredValueShape as S};

    let shape = context.lower_concrete_function_shape(function_shape);
    let kind = match context.function_representation(function_shape) {
        FunctionRepresentation::Symbolic => {
            generic_value_generic_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Generic)
        }
        FunctionRepresentation::Never(_) => {
            super::function::generic_value_never_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Never)
        }
        FunctionRepresentation::Executable(return_shape) => match return_shape {
            S::Int => generic_value_int_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Int),
            S::String => generic_value_string_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::String),
            S::BitArray => {
                generic_value_bit_array_function_expr(expression, function_shape, context)
                    .map(execution::FunctionExprKind::BitArray)
            }
            S::UtfCodepoint => {
                generic_value_utf_codepoint_function_expr(expression, function_shape, context)
                    .map(execution::FunctionExprKind::UtfCodepoint)
            }
            S::Custom(return_shape) => generic_value_custom_function_expr(
                expression,
                function_shape,
                &return_shape,
                context,
            )
            .map(execution::FunctionExprKind::Custom),
            S::Float => generic_value_float_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Float),
            S::Bool => generic_value_bool_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Bool),
            S::Nil => generic_value_nil_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Nil),
            S::Tuple(_) => generic_value_tuple_function_expr(expression, function_shape, context)
                .map(execution::FunctionExprKind::Tuple),
            S::List(item) => {
                generic_value_list_function_expr(expression, function_shape, &item, context)
                    .map(execution::FunctionExprKind::List)
            }
            S::Function(return_shape) => generic_value_function_function_expr(
                expression,
                function_shape,
                &return_shape,
                context,
            )
            .map(execution::FunctionExprKind::Function),
        },
    };

    kind.map(|kind| execution::FunctionExpr::from_parts(shape, kind))
}

pub(super) fn generic_function_value_binding(
    index: usize,
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<super::function::SpecializedFunctionBinding> {
    use super::super::specialization::{FunctionRepresentation, StoredValueShape as S};
    use super::function::SpecializedFunctionBinding as B;

    let shape = context.lower_concrete_function_shape(function_shape);
    match context.function_representation(function_shape) {
        FunctionRepresentation::Symbolic => {
            generic_value_generic_function_expr(expression, function_shape, context).map(|value| {
                B::Generic {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(index),
                        value.generic_function_type().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        FunctionRepresentation::Never(_) => {
            super::function::generic_value_never_function_expr(expression, function_shape, context)
                .map(|value| B::Never {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        value.type_().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                })
        }
        FunctionRepresentation::Executable(return_shape) => {
            match return_shape {
                S::Int => generic_value_int_function_expr(expression, function_shape, context).map(
                    |value| B::Int {
                        local: execution::IntFunctionLocalId(index),
                        value: execution::TypedFunctionExpr::new(shape, value),
                    },
                ),
                S::Float => generic_value_float_function_expr(expression, function_shape, context)
                    .map(|value| B::Float {
                        local: execution::FloatFunctionLocalId(index),
                        value: execution::TypedFunctionExpr::new(shape, value),
                    }),
                S::String => {
                    generic_value_string_function_expr(expression, function_shape, context).map(
                        |value| B::String {
                            local: execution::StringFunctionLocalId(index),
                            value: execution::TypedFunctionExpr::new(shape, value),
                        },
                    )
                }
                S::BitArray => {
                    generic_value_bit_array_function_expr(expression, function_shape, context).map(
                        |value| B::BitArray {
                            local: execution::BitArrayFunctionLocalId(index),
                            value: execution::TypedFunctionExpr::new(shape, value),
                        },
                    )
                }
                S::UtfCodepoint => {
                    generic_value_utf_codepoint_function_expr(expression, function_shape, context)
                        .map(|value| B::UtfCodepoint {
                            local: execution::UtfCodepointFunctionLocalId(index),
                            value: execution::TypedFunctionExpr::new(shape, value),
                        })
                }
                S::Custom(return_shape) => generic_value_custom_function_expr(
                    expression,
                    function_shape,
                    &return_shape,
                    context,
                )
                .map(|value| {
                    let local = execution::CustomFunctionLocal::new(
                        execution::CustomFunctionLocalId(index),
                        value.custom_function_type().clone(),
                    );
                    B::Custom {
                        local,
                        value: execution::TypedFunctionExpr::new(shape, value),
                    }
                }),
                S::Bool => generic_value_bool_function_expr(expression, function_shape, context)
                    .map(|value| B::Bool {
                        local: execution::BoolFunctionLocalId(index),
                        value: execution::TypedFunctionExpr::new(shape, value),
                    }),
                S::Nil => generic_value_nil_function_expr(expression, function_shape, context).map(
                    |value| B::Nil {
                        local: execution::NilFunctionLocalId(index),
                        value: execution::TypedFunctionExpr::new(shape, value),
                    },
                ),
                S::Tuple(_) => {
                    generic_value_tuple_function_expr(expression, function_shape, context).map(
                        |value| B::Tuple {
                            local: execution::TupleFunctionLocalId(index),
                            value: execution::TypedFunctionExpr::new(shape, value),
                        },
                    )
                }
                S::List(item) => {
                    let type_ = shape.type_().clone();
                    generic_value_list_function_expr(expression, function_shape, &item, context)
                        .map(|value| B::List {
                            local: super::super::frame::list_function_local_at(
                                &item, type_, index, context,
                            ),
                            value: execution::TypedFunctionExpr::new(shape, value),
                        })
                }
                S::Function(return_shape) => generic_value_function_function_expr(
                    expression,
                    function_shape,
                    &return_shape,
                    context,
                )
                .map(|value| {
                    let local = execution::FunctionFunctionLocal::new(
                        execution::FunctionFunctionLocalId(index),
                        value.function_function_type().clone(),
                    );
                    B::Function {
                        local,
                        value: execution::TypedFunctionExpr::new(shape, value),
                    }
                }),
            }
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_tuple_expr(
    expression: &module::GenericExpr,
    elements: &[super::super::specialization::SpecializedValueShape],
    context: &mut super::super::LoweringContext,
) -> Representability<execution::TupleExpr> {
    use execution::TupleExprKind as E;
    use module::GenericExprKind as M;

    let type_ = elements
        .iter()
        .map(|element| context.lower_concrete_value_type(element))
        .collect::<Vec<_>>();
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::TupleLocalId(index),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.tuple_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_tuple_function_expr(function, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => super::generic_tuple_list_expr(list, elements, context)
            .map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
            }),
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| generic_tuple_expr(true_, elements, context),
            |context| generic_tuple_expr(false_, elements, context),
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
        } => super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_tuple_expr(branch, elements, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_tuple_expr(fallback, elements, context).map(|fallback| E::IntCase {
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
                generic_tuple_expr(branch, elements, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_tuple_expr(fallback, elements, context).map(|fallback| E::StringCase {
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
                generic_tuple_expr(branch, elements, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_tuple_expr(fallback, elements, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_tuple_expr(return_, elements, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };

    kind.map(|kind| execution::TupleExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn generic_custom_expr(
    expression: &module::GenericExpr,
    shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomExpr> {
    let lowered_shape = context.lower_concrete_custom_shape(shape);
    generic_custom_expr_kind(expression, shape, context)
        .map(|kind| execution::CustomExpr::from_parts(lowered_shape, kind))
}

pub(in crate::plan::execution::lowering) fn generic_custom_expr_kind(
    expression: &module::GenericExpr,
    shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomExprKind> {
    use execution::CustomExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::CustomLocal::new(
                        execution::CustomLocalId(index),
                        context.lower_concrete_custom_shape(shape),
                    ),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.custom_function_id(function, shape)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_custom_function_expr(function, shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            super::generic_custom_list_expr(list, shape, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
            })
        }
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| generic_custom_expr_kind(true_, shape, context),
            |context| generic_custom_expr_kind(false_, shape, context),
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
                generic_custom_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_custom_expr_kind(fallback, shape, context).map(|fallback| E::IntCase {
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
                generic_custom_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_custom_expr_kind(fallback, shape, context).map(|fallback| E::StringCase {
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
                generic_custom_expr_kind(branch, shape, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_custom_expr_kind(fallback, shape, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_custom_expr_kind(return_, shape, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_list_value_expr(
    expression: &module::GenericExpr,
    item_shape: &super::super::specialization::SpecializedValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::ListExpr> {
    use super::super::specialization::SpecializedValueShape as S;

    match item_shape {
        S::Parameter(parameter) => {
            super::parameter_list_value_expr(expression, *parameter, context)
                .map(execution::ListExpr::Parameter)
        }
        S::Int => generic_value_int_list_expr(expression, context).map(execution::ListExpr::Int),
        S::String => {
            generic_value_string_list_expr(expression, context).map(execution::ListExpr::String)
        }
        S::BitArray => generic_value_bit_array_list_expr(expression, context)
            .map(execution::ListExpr::BitArray),
        S::UtfCodepoint => generic_value_utf_codepoint_list_expr(expression, context)
            .map(execution::ListExpr::UtfCodepoint),
        S::Custom(shape) => generic_value_custom_list_expr(expression, shape, context)
            .map(execution::ListExpr::Custom),
        S::Float => {
            generic_value_float_list_expr(expression, context).map(execution::ListExpr::Float)
        }
        S::Bool => generic_value_bool_list_expr(expression, context).map(execution::ListExpr::Bool),
        S::Nil => generic_value_nil_list_expr(expression, context).map(execution::ListExpr::Nil),
        S::Tuple(elements) => generic_value_tuple_list_expr(expression, elements, context)
            .map(execution::ListExpr::Tuple),
        S::List(item) => generic_value_nested_list_facade(expression, item, context),
        S::Function(function) => generic_value_function_list_expr(expression, function, context)
            .map(execution::ListExpr::Function),
    }
}

macro_rules! primitive_generic_value_list_expr {
    (
        $lower:ident,
        $result:ty,
        $shape:ident,
        $item:ident,
        $type_id:ident,
        $local:ident,
        $function:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericExpr,
            context: &mut super::super::LoweringContext,
        ) -> super::super::specialization::Representability<$result> {
            let item = execution::$item::new(context.$type_id());
            generic_value_typed_list_expr(
                expression,
                item,
                &super::super::specialization::SpecializedValueShape::$shape,
                execution::$local,
                |function, _, context| context.$function(function),
                |list, context| {
                    super::generic_stored_nested_list_expr(
                        list,
                        &super::super::specialization::StoredValueShape::$shape,
                        context,
                    )
                },
                context,
            )
        }
    };
}

primitive_generic_value_list_expr!(
    generic_value_int_list_expr,
    execution::IntListExpr,
    Int,
    IntListItem,
    int_list_type,
    IntListLocalId,
    int_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_string_list_expr,
    execution::StringListExpr,
    String,
    StringListItem,
    string_list_type,
    StringListLocalId,
    string_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_bit_array_list_expr,
    execution::BitArrayListExpr,
    BitArray,
    BitArrayListItem,
    bit_array_list_type,
    BitArrayListLocalId,
    bit_array_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_utf_codepoint_list_expr,
    execution::UtfCodepointListExpr,
    UtfCodepoint,
    UtfCodepointListItem,
    utf_codepoint_list_type,
    UtfCodepointListLocalId,
    utf_codepoint_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_float_list_expr,
    execution::FloatListExpr,
    Float,
    FloatListItem,
    float_list_type,
    FloatListLocalId,
    float_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_bool_list_expr,
    execution::BoolListExpr,
    Bool,
    BoolListItem,
    bool_list_type,
    BoolListLocalId,
    bool_list_function_id
);
primitive_generic_value_list_expr!(
    generic_value_nil_list_expr,
    execution::NilListExpr,
    Nil,
    NilListItem,
    nil_list_type,
    NilListLocalId,
    nil_list_function_id
);

pub(in crate::plan::execution::lowering) fn generic_value_custom_list_expr(
    expression: &module::GenericExpr,
    shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::CustomListExpr> {
    let item_shape = super::super::specialization::SpecializedValueShape::Custom(shape.clone());
    let item = execution::CustomListItem::new(context.specialized_custom_list_type(shape));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::CustomListLocalId,
        |function, item, context| context.custom_list_function_id(function, item.type_id()),
        |list, context| {
            super::generic_stored_nested_list_expr(
                list,
                &super::super::specialization::StoredValueShape::Custom(shape.clone()),
                context,
            )
        },
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_tuple_list_expr(
    expression: &module::GenericExpr,
    elements: &[super::super::specialization::SpecializedValueShape],
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::TupleListExpr> {
    let item_shape = super::super::specialization::SpecializedValueShape::Tuple(
        elements.to_vec().into_boxed_slice(),
    );
    let item = execution::TupleListItem::new(context.specialized_tuple_list_type(elements));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::TupleListLocalId,
        |function, item, context| context.tuple_list_function_id(function, item.type_id()),
        |list, context| {
            super::generic_stored_nested_list_expr(
                list,
                &super::super::specialization::StoredValueShape::Tuple(
                    elements.to_vec().into_boxed_slice(),
                ),
                context,
            )
        },
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_nested_list_expr(
    expression: &module::GenericExpr,
    nested_item: &super::super::specialization::SpecializedValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::StoredListExpr> {
    match nested_item.storage_representation() {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            generic_value_parameter_list_list_expr(expression, parameter, context)
                .map(execution::StoredListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(item) => {
            generic_value_stored_nested_list_expr(expression, &item, context)
                .map(execution::StoredListExpr::List)
        }
    }
}

fn generic_value_nested_list_facade(
    expression: &module::GenericExpr,
    nested_item: &super::super::specialization::SpecializedValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::ListExpr> {
    match nested_item.storage_representation() {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            generic_value_parameter_list_list_expr(expression, parameter, context)
                .map(execution::ListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(item) => {
            generic_value_stored_nested_list_expr(expression, &item, context)
                .map(execution::ListExpr::List)
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_value_stored_list_expr(
    expression: &module::GenericExpr,
    item_shape: &super::super::specialization::StoredValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::StoredListExpr> {
    use super::super::specialization::StoredValueShape as S;

    match item_shape {
        S::Int => {
            generic_value_int_list_expr(expression, context).map(execution::StoredListExpr::Int)
        }
        S::String => generic_value_string_list_expr(expression, context)
            .map(execution::StoredListExpr::String),
        S::BitArray => generic_value_bit_array_list_expr(expression, context)
            .map(execution::StoredListExpr::BitArray),
        S::UtfCodepoint => generic_value_utf_codepoint_list_expr(expression, context)
            .map(execution::StoredListExpr::UtfCodepoint),
        S::Custom(shape) => generic_value_custom_list_expr(expression, shape, context)
            .map(execution::StoredListExpr::Custom),
        S::Float => {
            generic_value_float_list_expr(expression, context).map(execution::StoredListExpr::Float)
        }
        S::Bool => {
            generic_value_bool_list_expr(expression, context).map(execution::StoredListExpr::Bool)
        }
        S::Nil => {
            generic_value_nil_list_expr(expression, context).map(execution::StoredListExpr::Nil)
        }
        S::Tuple(elements) => generic_value_tuple_list_expr(expression, elements, context)
            .map(execution::StoredListExpr::Tuple),
        S::List(item) => generic_value_nested_list_expr(expression, item, context),
        S::Function(function) => generic_value_function_list_expr(expression, function, context)
            .map(execution::StoredListExpr::Function),
    }
}

pub(in crate::plan::execution::lowering) fn generic_value_parameter_list_list_expr(
    expression: &module::GenericExpr,
    parameter: crate::plan::TypeParameterId,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::ParameterListListExpr> {
    let item_shape = super::super::specialization::SpecializedValueShape::List(Box::new(
        super::super::specialization::SpecializedValueShape::Parameter(parameter),
    ));
    let item = execution::ParameterListListItem::new(context.parameter_list_list_type(parameter));
    let nested_item = super::super::specialization::StoredValueShape::List(Box::new(
        super::super::specialization::SpecializedValueShape::Parameter(parameter),
    ));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::ParameterListListLocalId,
        |function, item, context| context.parameter_list_list_function_id(function, item.type_id()),
        |list, context| super::generic_stored_nested_list_expr(list, &nested_item, context),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_stored_nested_list_expr(
    expression: &module::GenericExpr,
    nested_item: &super::super::specialization::StoredValueShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::ListListExpr> {
    let specialized_item = nested_item.to_specialized();
    let item_shape =
        super::super::specialization::SpecializedValueShape::List(Box::new(specialized_item));
    let item = execution::ListListItem::new(context.specialized_stored_list_list_type(nested_item));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::ListListLocalId,
        |function, item, context| context.list_list_function_id(function, item.type_id()),
        |list, context| super::generic_stored_nested_list_expr(list, nested_item, context),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_function_list_expr(
    expression: &module::GenericExpr,
    function: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::FunctionListExpr> {
    let item_shape =
        super::super::specialization::SpecializedValueShape::Function(Box::new(function.clone()));
    let item = execution::FunctionListItem::new(context.specialized_function_list_type(function));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::FunctionListLocalId,
        |function_id, item, context| context.function_list_function_id(function_id, item.type_id()),
        |list, context| {
            super::generic_stored_nested_list_expr(
                list,
                &super::super::specialization::StoredValueShape::Function(Box::new(
                    function.clone(),
                )),
                context,
            )
        },
        context,
    )
}

fn generic_value_typed_list_expr<Item>(
    expression: &module::GenericExpr,
    item: Item,
    item_shape: &super::super::specialization::SpecializedValueShape,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut super::super::LoweringContext,
    ) -> super::super::specialization::Representability<Item::Function>,
    lower_index_source: impl Copy
    + Fn(
        &module::GenericListExpr,
        &mut super::super::LoweringContext,
    )
        -> super::super::specialization::Representability<Item::IndexSource>,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::TypedListExpr<Item>>
where
    Item: execution::ListItem,
{
    generic_value_typed_list_kind(
        expression,
        &item,
        item_shape,
        lower_local,
        lower_function,
        lower_index_source,
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

fn generic_value_typed_list_kind<Item>(
    expression: &module::GenericExpr,
    item: &Item,
    item_shape: &super::super::specialization::SpecializedValueShape,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut super::super::LoweringContext,
    ) -> super::super::specialization::Representability<Item::Function>,
    lower_index_source: impl Copy
    + Fn(
        &module::GenericListExpr,
        &mut super::super::LoweringContext,
    )
        -> super::super::specialization::Representability<Item::IndexSource>,
    context: &mut super::super::LoweringContext,
) -> super::super::specialization::Representability<execution::TypedListExprKind<Item>>
where
    Item: execution::ListItem,
{
    use execution::TypedListExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: lower_local(index),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                lower_function(function, item, context)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| super::generic_list_function_expr(function, item_shape, context),
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => {
            super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => lower_index_source(list, context)
            .map(|list| E::ListIndex(execution::ListIndexSource::from_parts(list, *index))),
        M::Panic(panic) => super::panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| {
                generic_value_typed_list_kind(
                    true_,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
            },
            |context| {
                generic_value_typed_list_kind(
                    false_,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
            },
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
                generic_value_typed_list_kind(
                    branch,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_typed_list_kind(
                    fallback,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|fallback| E::IntCase {
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
                generic_value_typed_list_kind(
                    branch,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_value_typed_list_kind(
                    fallback,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|fallback| E::StringCase {
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
                generic_value_typed_list_kind(
                    branch,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_value_typed_list_kind(
                    fallback,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                generic_value_typed_list_kind(
                    return_,
                    item,
                    item_shape,
                    lower_local,
                    lower_function,
                    lower_index_source,
                    context,
                )
                .map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    }
}
