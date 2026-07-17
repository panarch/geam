use crate::plan::{execution, module};

macro_rules! primitive_generic_function_expr {
    (
        $lower:ident,
        $expression:ident,
        $kind:ident,
        $local:ident,
        $function_id:ident,
        $function_function_id:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericFunctionExpr,
            context: &mut super::super::super::LoweringContext,
        ) -> execution::$expression {
            use execution::$kind as E;
            use module::GenericFunctionExprKind as M;

            let shape = context.concrete_function_shape(&expression.shape());
            let type_ = context.lower_concrete_function_type(&shape);
            let kind = match expression.kind() {
                M::Reference(reference) => E::Reference(super::function_reference(
                    reference,
                    context,
                    |function, context| context.$function_id(function),
                )),
                M::Closure {
                    function,
                    params,
                    captures,
                } => E::Closure(super::closure_template(
                    function,
                    params,
                    captures,
                    context,
                    |function, context| context.$function_id(function),
                )),
                M::LocalGet { local, name: _ } => E::LocalGet {
                    local: execution::$local(context.generic_function_local_index(local.id())),
                },
                M::Call { function, args } => E::Call {
                    function: context.$function_function_id(function),
                    args: super::super::direct_call_args(function, args, context),
                },
                M::FunctionCall { function, args } => E::FunctionCall {
                    function: Box::new(super::function_function_expr(function, context)),
                    args: super::super::call_args(args, context),
                },
                M::TupleIndex { tuple, index } => E::TupleIndex {
                    tuple: Box::new(super::super::tuple_expr(tuple, context)),
                    index: *index,
                    type_: type_.clone(),
                },
                M::CustomField(access) => {
                    E::CustomField(super::super::custom_field_access(access, context))
                }
                M::ListIndex { list, index } => E::ListIndex {
                    list: Box::new(super::super::function_list_expr(list, context)),
                    index: *index,
                    type_: type_.clone(),
                },
                M::Panic(panic) => E::Panic(super::super::panic_expr(panic, context)),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => E::BoolCase {
                    subject: Box::new(super::super::bool_expr(subject, context)),
                    true_: Box::new($lower(true_, context)),
                    false_: Box::new($lower(false_, context)),
                },
                M::IntCase {
                    subject,
                    clauses,
                    fallback,
                } => E::IntCase {
                    subject: Box::new(super::super::int_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| (pattern.clone(), $lower(branch, context)))
                        .collect(),
                    fallback: Box::new($lower(fallback, context)),
                },
                M::StringCase {
                    subject,
                    clauses,
                    fallback,
                } => E::StringCase {
                    subject: Box::new(super::super::string_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| (pattern.clone(), $lower(branch, context)))
                        .collect(),
                    fallback: Box::new($lower(fallback, context)),
                },
                M::FloatCase {
                    subject,
                    clauses,
                    fallback,
                } => E::FloatCase {
                    subject: Box::new(super::super::float_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| (*pattern, $lower(branch, context)))
                        .collect(),
                    fallback: Box::new($lower(fallback, context)),
                },
                M::Block { steps, return_ } => E::Block {
                    steps: super::super::super::step::steps(steps, context),
                    return_: Box::new($lower(return_, context)),
                },
            };

            execution::$expression::from_kind(kind)
        }
    };
}

primitive_generic_function_expr!(
    generic_int_function_expr,
    IntFunctionExpr,
    IntFunctionExprKind,
    IntFunctionLocalId,
    int_function_id,
    int_function_function_id
);
primitive_generic_function_expr!(
    generic_float_function_expr,
    FloatFunctionExpr,
    FloatFunctionExprKind,
    FloatFunctionLocalId,
    float_function_id,
    float_function_function_id
);
primitive_generic_function_expr!(
    generic_string_function_expr,
    StringFunctionExpr,
    StringFunctionExprKind,
    StringFunctionLocalId,
    string_function_id,
    string_function_function_id
);
primitive_generic_function_expr!(
    generic_bit_array_function_expr,
    BitArrayFunctionExpr,
    BitArrayFunctionExprKind,
    BitArrayFunctionLocalId,
    bit_array_function_id,
    bit_array_function_function_id
);
primitive_generic_function_expr!(
    generic_utf_codepoint_function_expr,
    UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind,
    UtfCodepointFunctionLocalId,
    utf_codepoint_function_id,
    utf_codepoint_function_function_id
);
primitive_generic_function_expr!(
    generic_bool_function_expr,
    BoolFunctionExpr,
    BoolFunctionExprKind,
    BoolFunctionLocalId,
    bool_function_id,
    bool_function_function_id
);
primitive_generic_function_expr!(
    generic_nil_function_expr,
    NilFunctionExpr,
    NilFunctionExprKind,
    NilFunctionLocalId,
    nil_function_id,
    nil_function_function_id
);

pub(in crate::plan::execution::lowering) fn generic_tuple_function_expr(
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::TupleFunctionExpr {
    use execution::TupleFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.lower_concrete_function_type(&shape);
    let kind = match expression.kind() {
        M::Reference(reference) => E::Reference(super::function_reference(
            reference,
            context,
            |function, context| context.tuple_function_id(function),
        )),
        M::Closure {
            function,
            params,
            captures,
        } => E::Closure(super::closure_template(
            function,
            params,
            captures,
            context,
            |function, context| context.tuple_function_id(function),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::TupleFunctionLocalId(
                context.generic_function_local_index(local.id()),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.tuple_function_function_id(function),
            args: super::super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::function_function_expr(function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(tuple, context)),
            index: *index,
            type_: type_.clone(),
        },
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(list, context)),
            index: *index,
            type_: type_.clone(),
        },
        M::Panic(panic) => E::Panic(super::super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(subject, context)),
            true_: Box::new(generic_tuple_function_expr(true_, context)),
            false_: Box::new(generic_tuple_function_expr(false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_tuple_function_expr(branch, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_tuple_function_expr(fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_tuple_function_expr(branch, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_tuple_function_expr(fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (*pattern, generic_tuple_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(generic_tuple_function_expr(fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::super::step::steps(steps, context),
            return_: Box::new(generic_tuple_function_expr(return_, context)),
        },
    };

    execution::TupleFunctionExpr::from_parts(type_, kind)
}
