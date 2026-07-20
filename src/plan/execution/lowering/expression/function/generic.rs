use super::super::super::specialization::Representability;
use crate::plan::{execution, module};

fn symbolic_closure(
    function: &module::FunctionInstantiation,
    captures: Vec<execution::CaptureArg>,
    context: &mut super::super::super::LoweringContext,
) -> execution::GenericFunctionExprKind {
    execution::GenericFunctionExprKind::Closure {
        target: context.generic_callable_id(function),
        captures,
    }
}

macro_rules! symbolic_primitive_function_expr {
    (
        $lower:ident,
        $lower_kind:ident,
        $module_expr:ident,
        $module_kind:ident,
        $constant:ident,
        $local_kind:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::$module_expr,
            shape: &super::super::super::specialization::SpecializedFunctionShape,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<execution::GenericFunctionExpr> {
            $lower_kind(expression.kind(), shape, context).map(|kind| {
                execution::GenericFunctionExpr::from_parts(
                    context.generic_function_type(shape),
                    kind,
                )
            })
        }

        fn $lower_kind(
            kind: &module::$module_kind,
            shape: &super::super::super::specialization::SpecializedFunctionShape,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<execution::GenericFunctionExprKind> {
            use execution::GenericFunctionExprKind as E;
            use module::$module_kind as M;
            let type_ = context.generic_function_type(shape);

            match kind {
                M::Constant(value) => context.$constant(value, shape).map(E::Constant),
                M::Reference(reference) => Representability::Inhabited(E::Reference {
                    target: context.generic_callable_id(reference.instantiation()),
                }),
                M::Closure {
                    function, captures, ..
                } => super::super::symbolic_capture_args(function, captures, context)
                    .map(|captures| symbolic_closure(function, captures, context)),
                M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(context.mapped_local(
                            super::super::super::frame::LocalKind::$local_kind,
                            local.0,
                        )),
                        type_.clone(),
                    ),
                }),
                M::Call { function, args, .. } => {
                    super::super::direct_call(function, args, context, |function, context| {
                        context.generic_function_function_id(function, type_.clone())
                    })
                    .map(E::Call)
                }
                M::FunctionCall { function, args, .. } => super::super::function_call(
                    args,
                    context,
                    |context| super::function_function_expr(function, context),
                    |context| super::evaluated_function_function_expr(function, context),
                )
                .map(E::FunctionCall),
                M::TupleIndex { tuple, index, .. } => {
                    super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                        tuple: Box::new(tuple),
                        index: *index,
                    })
                }
                M::CustomField(access) => {
                    super::super::custom_field_access(access, context).map(E::CustomField)
                }
                M::ListIndex { list, index, .. } => super::super::function_list_expr(list, context)
                    .map(|list| E::ListIndex {
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
                    |context| $lower(true_, shape, context),
                    |context| $lower(false_, shape, context),
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
                } => super::super::int_expr(subject, context).and_then(|subject| {
                    Representability::collect(clauses.iter().map(|(pattern, branch)| {
                        $lower(branch, shape, context)
                            .map(|branch| (pattern.clone(), branch.into_kind()))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, shape, context).map(|fallback| E::IntCase {
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
                        $lower(branch, shape, context)
                            .map(|branch| (pattern.clone(), branch.into_kind()))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, shape, context).map(|fallback| E::StringCase {
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
                        $lower(branch, shape, context).map(|branch| (*pattern, branch.into_kind()))
                    }))
                    .and_then(|clauses| {
                        $lower(fallback, shape, context).map(|fallback| E::FloatCase {
                            subject: Box::new(subject),
                            clauses,
                            fallback: Box::new(fallback.into_kind()),
                        })
                    })
                }),
                M::Block { steps, return_ } => super::super::super::step::steps(steps, context)
                    .and_then(|steps| {
                        $lower(return_, shape, context).map(|return_| E::Block {
                            steps,
                            return_: Box::new(return_.into_kind()),
                        })
                    }),
            }
        }
    };
}

symbolic_primitive_function_expr!(
    symbolic_int_function_expr,
    symbolic_int_function_expr_kind,
    IntFunctionExpr,
    IntFunctionExprKind,
    symbolic_int_function_constant,
    IntFunction
);
symbolic_primitive_function_expr!(
    symbolic_float_function_expr,
    symbolic_float_function_expr_kind,
    FloatFunctionExpr,
    FloatFunctionExprKind,
    symbolic_float_function_constant,
    FloatFunction
);
symbolic_primitive_function_expr!(
    symbolic_string_function_expr,
    symbolic_string_function_expr_kind,
    StringFunctionExpr,
    StringFunctionExprKind,
    symbolic_string_function_constant,
    StringFunction
);
symbolic_primitive_function_expr!(
    symbolic_bit_array_function_expr,
    symbolic_bit_array_function_expr_kind,
    BitArrayFunctionExpr,
    BitArrayFunctionExprKind,
    symbolic_bit_array_function_constant,
    BitArrayFunction
);
symbolic_primitive_function_expr!(
    symbolic_utf_codepoint_function_expr,
    symbolic_utf_codepoint_function_expr_kind,
    UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind,
    symbolic_utf_codepoint_function_constant,
    UtfCodepointFunction
);
symbolic_primitive_function_expr!(
    symbolic_bool_function_expr,
    symbolic_bool_function_expr_kind,
    BoolFunctionExpr,
    BoolFunctionExprKind,
    symbolic_bool_function_constant,
    BoolFunction
);
symbolic_primitive_function_expr!(
    symbolic_nil_function_expr,
    symbolic_nil_function_expr_kind,
    NilFunctionExpr,
    NilFunctionExprKind,
    symbolic_nil_function_constant,
    NilFunction
);
symbolic_primitive_function_expr!(
    symbolic_tuple_function_expr,
    symbolic_tuple_function_expr_kind,
    TupleFunctionExpr,
    TupleFunctionExprKind,
    symbolic_tuple_function_constant,
    TupleFunction
);

macro_rules! primitive_generic_function_expr {
    (
        $lower:ident,
        $expression:ident,
        $kind:ident,
        $local:ident,
        $constant:ident,
        $function_id:ident,
        $function_function_id:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericFunctionExpr,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<execution::$expression> {
            use execution::$kind as E;
            use module::GenericFunctionExprKind as M;

            let shape = context.concrete_function_shape(&expression.shape());
            let type_ = context.lower_concrete_function_type(&shape);
            let kind = match expression.kind() {
                M::Constant(value) => context.$constant(value).map(E::Constant),
                M::Reference(reference) => {
                    super::function_reference(reference, context, |function, context| {
                        context.$function_id(function)
                    })
                    .map(E::Reference)
                }
                M::Closure { function, captures } => {
                    super::closure_template(function, captures, context, |function, context| {
                        context.$function_id(function)
                    })
                    .map(E::Closure)
                }
                M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
                    local: execution::$local(context.generic_function_local_index(local.id())),
                }),
                M::Call { function, args } => {
                    super::super::direct_call(function, args, context, |function, context| {
                        context.$function_function_id(function)
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
                        type_: type_.clone(),
                    })
                }
                M::CustomField(access) => {
                    super::super::custom_field_access(access, context).map(E::CustomField)
                }
                M::ListIndex { list, index } => super::super::function_list_expr(list, context)
                    .map(|list| E::ListIndex {
                        list: Box::new(list),
                        index: *index,
                        type_: type_.clone(),
                    }),
                M::Panic(panic) => super::super::panic_expr(panic, context).map(E::Panic),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => super::super::bool_case_into(
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
                } => super::super::int_expr(subject, context).and_then(|subject| {
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
                } => super::super::string_expr(subject, context).and_then(|subject| {
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
                } => super::super::float_expr(subject, context).and_then(|subject| {
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
                M::Block { steps, return_ } => super::super::super::step::steps(steps, context)
                    .and_then(|steps| {
                        $lower(return_, context).map(|return_| E::Block {
                            steps,
                            return_: Box::new(return_),
                        })
                    }),
            };

            kind.map(execution::$expression::from_kind)
        }
    };
}

primitive_generic_function_expr!(
    generic_int_function_expr,
    IntFunctionExpr,
    IntFunctionExprKind,
    IntFunctionLocalId,
    generic_int_function_constant,
    int_function_id,
    int_function_function_id
);
primitive_generic_function_expr!(
    generic_float_function_expr,
    FloatFunctionExpr,
    FloatFunctionExprKind,
    FloatFunctionLocalId,
    generic_float_function_constant,
    float_function_id,
    float_function_function_id
);
primitive_generic_function_expr!(
    generic_string_function_expr,
    StringFunctionExpr,
    StringFunctionExprKind,
    StringFunctionLocalId,
    generic_string_function_constant,
    string_function_id,
    string_function_function_id
);
primitive_generic_function_expr!(
    generic_bit_array_function_expr,
    BitArrayFunctionExpr,
    BitArrayFunctionExprKind,
    BitArrayFunctionLocalId,
    generic_bit_array_function_constant,
    bit_array_function_id,
    bit_array_function_function_id
);
primitive_generic_function_expr!(
    generic_utf_codepoint_function_expr,
    UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind,
    UtfCodepointFunctionLocalId,
    generic_utf_codepoint_function_constant,
    utf_codepoint_function_id,
    utf_codepoint_function_function_id
);
primitive_generic_function_expr!(
    generic_bool_function_expr,
    BoolFunctionExpr,
    BoolFunctionExprKind,
    BoolFunctionLocalId,
    generic_bool_function_constant,
    bool_function_id,
    bool_function_function_id
);
primitive_generic_function_expr!(
    generic_nil_function_expr,
    NilFunctionExpr,
    NilFunctionExprKind,
    NilFunctionLocalId,
    generic_nil_function_constant,
    nil_function_id,
    nil_function_function_id
);

pub(in crate::plan::execution::lowering) fn generic_tuple_function_expr(
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::TupleFunctionExpr> {
    use execution::TupleFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.lower_concrete_function_type(&shape);
    let kind = match expression.kind() {
        M::Constant(value) => context
            .generic_tuple_function_constant(value)
            .map(E::Constant),
        M::Reference(reference) => {
            super::function_reference(reference, context, |function, context| {
                context.tuple_function_id(function)
            })
            .map(E::Reference)
        }
        M::Closure { function, captures } => {
            super::closure_template(function, captures, context, |function, context| {
                context.tuple_function_id(function)
            })
            .map(E::Closure)
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::TupleFunctionLocalId(
                context.generic_function_local_index(local.id()),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.tuple_function_function_id(function)
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
            |context| generic_tuple_function_expr(true_, context),
            |context| generic_tuple_function_expr(false_, context),
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
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_tuple_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_tuple_function_expr(fallback, context).map(|fallback| E::IntCase {
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
                generic_tuple_function_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                generic_tuple_function_expr(fallback, context).map(|fallback| E::StringCase {
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
                generic_tuple_function_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                generic_tuple_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                generic_tuple_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };

    kind.map(|kind| execution::TupleFunctionExpr::from_parts(type_, kind))
}

pub(in crate::plan::execution::lowering) fn symbolic_list_function_expr(
    expression: &module::ListFunctionExpr,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    symbolic_list_function_expr_kind(expression.kind(), shape, context).map(|kind| {
        execution::GenericFunctionExpr::from_parts(context.generic_function_type(shape), kind)
    })
}

pub(in crate::plan::execution::lowering) fn symbolic_list_function_expr_kind(
    kind: &module::ListFunctionExprKind,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExprKind> {
    use execution::GenericFunctionExprKind as E;
    use module::ListFunctionExprKind as M;
    let type_ = context.generic_function_type(shape);

    match kind {
        M::Constant(value) => context
            .symbolic_list_function_constant(value, shape)
            .map(E::Constant),
        M::Reference(reference) => Representability::Inhabited(E::Reference {
            target: context.generic_callable_id(reference.instantiation()),
        }),
        M::Closure { function, captures } => {
            super::super::symbolic_capture_args(function, captures, context)
                .map(|captures| symbolic_closure(function, captures, context))
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(
                    context.local_index(super::super::super::frame::list_function_local_key(local)),
                ),
                type_.clone(),
            ),
        }),
        M::Call { function, args, .. } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.generic_function_function_id(function, type_.clone())
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args, .. } => super::super::function_call(
            args,
            context,
            |context| super::function_function_expr(function, context),
            |context| super::evaluated_function_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index, .. } => {
            super::super::tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
                tuple: Box::new(tuple),
                index: *index,
            })
        }
        M::CustomField(access) => {
            super::super::custom_field_access(access, context).map(E::CustomField)
        }
        M::ListIndex { list, index, .. } => {
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
            |context| symbolic_list_function_expr(true_, shape, context),
            |context| symbolic_list_function_expr(false_, shape, context),
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
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                symbolic_list_function_expr(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                symbolic_list_function_expr(fallback, shape, context).map(|fallback| E::IntCase {
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
                symbolic_list_function_expr(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                symbolic_list_function_expr(fallback, shape, context).map(|fallback| {
                    E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
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
                symbolic_list_function_expr(branch, shape, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                symbolic_list_function_expr(fallback, shape, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                symbolic_list_function_expr(return_, shape, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_.into_kind()),
                })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn symbolic_custom_function_expr(
    expression: &module::CustomFunctionExpr,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    symbolic_custom_function_expr_kind(expression.kind(), shape, context).map(|kind| {
        execution::GenericFunctionExpr::from_parts(context.generic_function_type(shape), kind)
    })
}

pub(in crate::plan::execution::lowering) fn symbolic_function_function_expr(
    expression: &module::FunctionFunctionExpr,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    symbolic_function_function_expr_kind(expression.kind(), shape, context).map(|kind| {
        execution::GenericFunctionExpr::from_parts(context.generic_function_type(shape), kind)
    })
}

pub(in crate::plan::execution::lowering) fn symbolic_function_function_expr_kind(
    kind: &module::FunctionFunctionExprKind,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExprKind> {
    use execution::GenericFunctionExprKind as E;
    use module::FunctionFunctionExprKind as M;
    let type_ = context.generic_function_type(shape);

    match kind {
        M::Constant(value) => context
            .symbolic_function_function_constant(value, shape)
            .map(E::Constant),
        M::Reference(reference) => Representability::Inhabited(E::Reference {
            target: context.generic_callable_id(reference.instantiation()),
        }),
        M::Closure { function, captures } => {
            super::super::symbolic_capture_args(function, captures, context)
                .map(|captures| symbolic_closure(function, captures, context))
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(context.mapped_local(
                    super::super::super::frame::LocalKind::FunctionFunction,
                    local.id().0,
                )),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.generic_function_function_id(function, type_.clone())
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
            |context| symbolic_function_function_expr_kind(true_, shape, context),
            |context| symbolic_function_function_expr_kind(false_, shape, context),
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
                symbolic_function_function_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                symbolic_function_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_function_function_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                symbolic_function_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_function_function_expr_kind(branch, shape, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                symbolic_function_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_function_function_expr_kind(return_, shape, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_),
                    }
                })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn symbolic_custom_function_expr_kind(
    kind: &module::CustomFunctionExprKind,
    shape: &super::super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExprKind> {
    use execution::GenericFunctionExprKind as E;
    use module::CustomFunctionExprKind as M;
    let type_ = context.generic_function_type(shape);

    match kind {
        M::Constant(value) => context
            .symbolic_custom_function_constant(value, shape)
            .map(E::Constant),
        M::Constructor(constructor) => Representability::Inhabited(E::Constructor {
            target: context.generic_constructor_callable_id(constructor.clone()),
        }),
        M::Reference(reference) => Representability::Inhabited(E::Reference {
            target: context.generic_callable_id(reference.instantiation()),
        }),
        M::Closure { function, captures } => {
            super::super::symbolic_capture_args(function, captures, context)
                .map(|captures| symbolic_closure(function, captures, context))
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(context.mapped_local(
                    super::super::super::frame::LocalKind::CustomFunction,
                    local.id().0,
                )),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.generic_function_function_id(function, type_.clone())
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
            |context| symbolic_custom_function_expr_kind(true_, shape, context),
            |context| symbolic_custom_function_expr_kind(false_, shape, context),
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
                symbolic_custom_function_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                symbolic_custom_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_custom_function_expr_kind(branch, shape, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                symbolic_custom_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_custom_function_expr_kind(branch, shape, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                symbolic_custom_function_expr_kind(fallback, shape, context).map(|fallback| {
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
                symbolic_custom_function_expr_kind(return_, shape, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_),
                    }
                })
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_symbolic_function_expr(
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    use execution::GenericFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.generic_function_type(&shape);
    let kind = match expression.kind() {
        M::Constant(value) => context.generic_function_constant(value).map(E::Constant),
        M::Reference(reference) => Representability::Inhabited(E::Reference {
            target: context.generic_callable_id(reference.instantiation()),
        }),
        M::Closure { function, captures } => {
            super::super::symbolic_capture_args(function, captures, context)
                .map(|captures| symbolic_closure(function, captures, context))
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(context.generic_function_local_index(local.id())),
                type_.clone(),
            ),
        }),
        M::Call { function, args } => {
            super::super::direct_call(function, args, context, |function, context| {
                context.generic_function_function_id(function, type_.clone())
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
            |context| generic_symbolic_function_expr(true_, context),
            |context| generic_symbolic_function_expr(false_, context),
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
        } => super::super::int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                generic_symbolic_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_symbolic_function_expr(fallback, context).map(|fallback| E::IntCase {
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
                generic_symbolic_function_expr(branch, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_symbolic_function_expr(fallback, context).map(|fallback| E::StringCase {
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
                generic_symbolic_function_expr(branch, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                generic_symbolic_function_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::super::step::steps(steps, context).and_then(|steps| {
                generic_symbolic_function_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_.into_kind()),
                })
            })
        }
    };

    kind.map(|kind| execution::GenericFunctionExpr::from_parts(type_, kind))
}
