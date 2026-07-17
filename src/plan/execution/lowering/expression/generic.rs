use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn generic_expr(
    expression: &module::GenericExpr,
    context: &mut super::super::LoweringContext,
) -> execution::Expr {
    use super::super::specialization::ConcreteValueShape as S;

    let kind = match context.concrete_parameter(expression.parameter()) {
        S::Int => execution::ExprKind::Int(generic_int_expr(expression, context)),
        S::Float => execution::ExprKind::Float(generic_float_expr(expression, context)),
        S::String => execution::ExprKind::String(generic_string_expr(expression, context)),
        S::BitArray => execution::ExprKind::BitArray(generic_bit_array_expr(expression, context)),
        S::UtfCodepoint => {
            execution::ExprKind::UtfCodepoint(generic_utf_codepoint_expr(expression, context))
        }
        S::Bool => execution::ExprKind::Bool(generic_bool_expr(expression, context)),
        S::Nil => execution::ExprKind::Nil(generic_nil_expr(expression, context)),
        S::Tuple(elements) => {
            execution::ExprKind::Tuple(generic_tuple_expr(expression, &elements, context))
        }
        S::List(item) => {
            execution::ExprKind::List(generic_list_value_expr(expression, &item, context))
        }
        S::Function(function) => execution::ExprKind::Function(generic_function_value_expr(
            expression, &function, context,
        )),
        S::Custom(shape) => {
            execution::ExprKind::Custom(generic_custom_expr(expression, &shape, context))
        }
    };

    execution::Expr::from_kind(kind)
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
        ) -> execution::$expression {
            use execution::$kind as E;
            use module::GenericExprKind as M;

            execution::$expression::from_kind(match expression.kind() {
                M::LocalGet { local, name: _ } => E::LocalGet {
                    local: execution::$local(context.generic_local_index(local.id())),
                },
                M::Call { function, args } => E::Call {
                    function: context.$function_id(function),
                    args: super::call_args(args, context),
                },
                M::FunctionCall { function, args } => E::FunctionCall {
                    function: Box::new(super::$function_expr(function, context)),
                    args: super::call_args(args, context),
                },
                M::TupleIndex { tuple, index } => E::TupleIndex {
                    tuple: Box::new(super::tuple_expr(tuple, context)),
                    index: *index,
                },
                M::CustomField(access) => {
                    E::CustomField(super::custom_field_access(access, context))
                }
                M::ListIndex { list, index } => E::ListIndex {
                    list: Box::new(super::$list_expr(list, context)),
                    index: *index,
                },
                M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => E::BoolCase {
                    subject: Box::new(super::bool_expr(subject, context)),
                    true_: Box::new($lower(true_, context)),
                    false_: Box::new($lower(false_, context)),
                },
                M::IntCase {
                    subject,
                    clauses,
                    fallback,
                } => E::IntCase {
                    subject: Box::new(super::int_expr(subject, context)),
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
                    subject: Box::new(super::string_expr(subject, context)),
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
                    subject: Box::new(super::float_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| (*pattern, $lower(branch, context)))
                        .collect(),
                    fallback: Box::new($lower(fallback, context)),
                },
                M::Block { steps, return_ } => E::Block {
                    steps: super::super::step::steps(steps, context),
                    return_: Box::new($lower(return_, context)),
                },
            })
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
            function_shape: &super::super::specialization::ConcreteFunctionShape,
            context: &mut super::super::LoweringContext,
        ) -> execution::$expression {
            use execution::$kind as E;
            use module::GenericExprKind as M;

            let type_ = context.lower_concrete_function_type(function_shape);
            let kind = match expression.kind() {
                M::LocalGet { local, name: _ } => E::LocalGet {
                    local: execution::$local(context.generic_local_index(local.id())),
                },
                M::Call { function, args } => E::Call {
                    function: context.$function_function_id(function),
                    args: super::call_args(args, context),
                },
                M::FunctionCall { function, args } => E::FunctionCall {
                    function: Box::new(super::generic_function_function_expr(
                        function,
                        function_shape,
                        context,
                    )),
                    args: super::call_args(args, context),
                },
                M::TupleIndex { tuple, index } => E::TupleIndex {
                    tuple: Box::new(super::tuple_expr(tuple, context)),
                    index: *index,
                    type_: type_.clone(),
                },
                M::CustomField(access) => {
                    E::CustomField(super::custom_field_access(access, context))
                }
                M::ListIndex { list, index } => E::ListIndex {
                    list: Box::new(super::generic_function_list_expr(
                        list,
                        function_shape,
                        context,
                    )),
                    index: *index,
                    type_: type_.clone(),
                },
                M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
                M::BoolCase {
                    subject,
                    true_,
                    false_,
                } => E::BoolCase {
                    subject: Box::new(super::bool_expr(subject, context)),
                    true_: Box::new($lower(true_, function_shape, context)),
                    false_: Box::new($lower(false_, function_shape, context)),
                },
                M::IntCase {
                    subject,
                    clauses,
                    fallback,
                } => E::IntCase {
                    subject: Box::new(super::int_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| {
                            (pattern.clone(), $lower(branch, function_shape, context))
                        })
                        .collect(),
                    fallback: Box::new($lower(fallback, function_shape, context)),
                },
                M::StringCase {
                    subject,
                    clauses,
                    fallback,
                } => E::StringCase {
                    subject: Box::new(super::string_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| {
                            (pattern.clone(), $lower(branch, function_shape, context))
                        })
                        .collect(),
                    fallback: Box::new($lower(fallback, function_shape, context)),
                },
                M::FloatCase {
                    subject,
                    clauses,
                    fallback,
                } => E::FloatCase {
                    subject: Box::new(super::float_expr(subject, context)),
                    clauses: clauses
                        .iter()
                        .map(|(pattern, branch)| {
                            (*pattern, $lower(branch, function_shape, context))
                        })
                        .collect(),
                    fallback: Box::new($lower(fallback, function_shape, context)),
                },
                M::Block { steps, return_ } => E::Block {
                    steps: super::super::step::steps(steps, context),
                    return_: Box::new($lower(return_, function_shape, context)),
                },
            };

            execution::$expression::from_kind(kind)
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
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::LoweringContext,
) -> execution::TupleFunctionExpr {
    use execution::TupleFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.lower_concrete_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::TupleFunctionLocalId(context.generic_local_index(local.id())),
        },
        M::Call { function, args } => E::Call {
            function: context.tuple_function_function_id(function),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_function_function_expr(
                function,
                function_shape,
                context,
            )),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
            type_: type_.clone(),
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_function_list_expr(
                list,
                function_shape,
                context,
            )),
            index: *index,
            type_: type_.clone(),
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_value_tuple_function_expr(
                true_,
                function_shape,
                context,
            )),
            false_: Box::new(generic_value_tuple_function_expr(
                false_,
                function_shape,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_tuple_function_expr(branch, function_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_tuple_function_expr(
                fallback,
                function_shape,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_tuple_function_expr(branch, function_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_tuple_function_expr(
                fallback,
                function_shape,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_value_tuple_function_expr(branch, function_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_tuple_function_expr(
                fallback,
                function_shape,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_value_tuple_function_expr(
                return_,
                function_shape,
                context,
            )),
        },
    };

    execution::TupleFunctionExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_value_custom_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    return_shape: &super::super::specialization::ConcreteCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::CustomFunctionExpr {
    let type_ = context.custom_function_type(crate::plan::CustomFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let kind = generic_value_custom_function_expr_kind(expression, function_shape, &type_, context);
    execution::CustomFunctionExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_value_custom_function_expr_kind(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    type_: &execution::CustomFunctionType,
    context: &mut super::super::LoweringContext,
) -> execution::CustomFunctionExprKind {
    use execution::CustomFunctionExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(context.generic_local_index(local.id())),
                type_.clone(),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.custom_function_function_id(function, type_.clone()),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_function_function_expr(
                function,
                function_shape,
                context,
            )),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_function_list_expr(
                list,
                function_shape,
                context,
            )),
            index: *index,
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_value_custom_function_expr_kind(
                true_,
                function_shape,
                type_,
                context,
            )),
            false_: Box::new(generic_value_custom_function_expr_kind(
                false_,
                function_shape,
                type_,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_custom_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_custom_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_custom_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_custom_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_value_custom_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_custom_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_value_custom_function_expr_kind(
                return_,
                function_shape,
                type_,
                context,
            )),
        },
    }
}

pub(in crate::plan::execution::lowering) fn generic_value_list_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    item_shape: &super::super::specialization::ConcreteValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::ListFunctionExpr {
    use execution::ListFunctionExprKind as E;
    use module::GenericExprKind as M;

    let type_ = context.lower_concrete_function_type(function_shape);
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: super::super::frame::list_function_local_at(
                item_shape,
                type_.clone(),
                context.generic_local_index(local.id()),
                context,
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.list_function_function_id(function, function_shape, item_shape),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_function_function_expr(
                function,
                function_shape,
                context,
            )),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
            type_: type_.clone(),
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_function_list_expr(
                list,
                function_shape,
                context,
            )),
            index: *index,
            type_: type_.clone(),
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_value_list_function_expr(
                true_,
                function_shape,
                item_shape,
                context,
            )),
            false_: Box::new(generic_value_list_function_expr(
                false_,
                function_shape,
                item_shape,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_list_function_expr(
                            branch,
                            function_shape,
                            item_shape,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_list_function_expr(
                fallback,
                function_shape,
                item_shape,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_list_function_expr(
                            branch,
                            function_shape,
                            item_shape,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_list_function_expr(
                fallback,
                function_shape,
                item_shape,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_value_list_function_expr(
                            branch,
                            function_shape,
                            item_shape,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_list_function_expr(
                fallback,
                function_shape,
                item_shape,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_value_list_function_expr(
                return_,
                function_shape,
                item_shape,
                context,
            )),
        },
    };

    execution::ListFunctionExpr::from_kind(kind)
}

pub(in crate::plan::execution::lowering) fn generic_value_function_function_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    return_shape: &super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::LoweringContext,
) -> execution::FunctionFunctionExpr {
    let type_ = context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let kind =
        generic_value_function_function_expr_kind(expression, function_shape, &type_, context);
    execution::FunctionFunctionExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_value_function_function_expr_kind(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::LoweringContext,
) -> execution::FunctionFunctionExprKind {
    use execution::FunctionFunctionExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(context.generic_local_index(local.id())),
                type_.clone(),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.function_function_function_id(function, type_.clone()),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_function_function_expr(
                function,
                function_shape,
                context,
            )),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_function_list_expr(
                list,
                function_shape,
                context,
            )),
            index: *index,
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_value_function_function_expr_kind(
                true_,
                function_shape,
                type_,
                context,
            )),
            false_: Box::new(generic_value_function_function_expr_kind(
                false_,
                function_shape,
                type_,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_function_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_function_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_function_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_function_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_value_function_function_expr_kind(
                            branch,
                            function_shape,
                            type_,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_function_function_expr_kind(
                fallback,
                function_shape,
                type_,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_value_function_function_expr_kind(
                return_,
                function_shape,
                type_,
                context,
            )),
        },
    }
}

pub(in crate::plan::execution::lowering) fn generic_function_value_expr(
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::LoweringContext,
) -> execution::FunctionExpr {
    use super::super::specialization::ConcreteValueShape as S;

    let shape = context.function_shape(function_shape.to_module_shape());
    let kind = match function_shape.return_() {
        S::Int => execution::FunctionExprKind::Int(generic_value_int_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::String => execution::FunctionExprKind::String(generic_value_string_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::BitArray => execution::FunctionExprKind::BitArray(
            generic_value_bit_array_function_expr(expression, function_shape, context),
        ),
        S::UtfCodepoint => execution::FunctionExprKind::UtfCodepoint(
            generic_value_utf_codepoint_function_expr(expression, function_shape, context),
        ),
        S::Custom(return_shape) => execution::FunctionExprKind::Custom(
            generic_value_custom_function_expr(expression, function_shape, return_shape, context),
        ),
        S::Float => execution::FunctionExprKind::Float(generic_value_float_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::Bool => execution::FunctionExprKind::Bool(generic_value_bool_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::Nil => execution::FunctionExprKind::Nil(generic_value_nil_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::Tuple(_) => execution::FunctionExprKind::Tuple(generic_value_tuple_function_expr(
            expression,
            function_shape,
            context,
        )),
        S::List(item) => execution::FunctionExprKind::List(generic_value_list_function_expr(
            expression,
            function_shape,
            item,
            context,
        )),
        S::Function(return_shape) => execution::FunctionExprKind::Function(
            generic_value_function_function_expr(expression, function_shape, return_shape, context),
        ),
    };

    execution::FunctionExpr::from_parts(shape, kind)
}

pub(super) fn generic_function_value_binding(
    index: usize,
    expression: &module::GenericExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::LoweringContext,
) -> super::function::SpecializedFunctionBinding {
    use super::super::specialization::ConcreteValueShape as S;
    use super::function::SpecializedFunctionBinding as B;

    let shape = context.function_shape(function_shape.to_module_shape());
    match function_shape.return_() {
        S::Int => B::Int {
            local: execution::IntFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_int_function_expr(expression, function_shape, context),
            ),
        },
        S::Float => B::Float {
            local: execution::FloatFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_float_function_expr(expression, function_shape, context),
            ),
        },
        S::String => B::String {
            local: execution::StringFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_string_function_expr(expression, function_shape, context),
            ),
        },
        S::BitArray => B::BitArray {
            local: execution::BitArrayFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_bit_array_function_expr(expression, function_shape, context),
            ),
        },
        S::UtfCodepoint => B::UtfCodepoint {
            local: execution::UtfCodepointFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_utf_codepoint_function_expr(expression, function_shape, context),
            ),
        },
        S::Custom(return_shape) => {
            let value = generic_value_custom_function_expr(
                expression,
                function_shape,
                return_shape,
                context,
            );
            let local = execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                value.custom_function_type().clone(),
            );
            B::Custom {
                local,
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }
        S::Bool => B::Bool {
            local: execution::BoolFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_bool_function_expr(expression, function_shape, context),
            ),
        },
        S::Nil => B::Nil {
            local: execution::NilFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_nil_function_expr(expression, function_shape, context),
            ),
        },
        S::Tuple(_) => B::Tuple {
            local: execution::TupleFunctionLocalId(index),
            value: execution::TypedFunctionExpr::new(
                shape,
                generic_value_tuple_function_expr(expression, function_shape, context),
            ),
        },
        S::List(item) => {
            let type_ = shape.type_().clone();
            B::List {
                local: super::super::frame::list_function_local_at(item, type_, index, context),
                value: execution::TypedFunctionExpr::new(
                    shape,
                    generic_value_list_function_expr(expression, function_shape, item, context),
                ),
            }
        }
        S::Function(return_shape) => {
            let value = generic_value_function_function_expr(
                expression,
                function_shape,
                return_shape,
                context,
            );
            let local = execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                value.function_function_type().clone(),
            );
            B::Function {
                local,
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_tuple_expr(
    expression: &module::GenericExpr,
    elements: &[super::super::specialization::ConcreteValueShape],
    context: &mut super::super::LoweringContext,
) -> execution::TupleExpr {
    use execution::TupleExprKind as E;
    use module::GenericExprKind as M;

    let type_ = elements
        .iter()
        .map(|element| context.lower_concrete_value_type(element))
        .collect::<Vec<_>>();
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::TupleLocalId(context.generic_local_index(local.id())),
        },
        M::Call { function, args } => E::Call {
            function: context.tuple_function_id(function),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_tuple_function_expr(function, context)),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_tuple_list_expr(list, elements, context)),
            index: *index,
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_tuple_expr(true_, elements, context)),
            false_: Box::new(generic_tuple_expr(false_, elements, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_tuple_expr(branch, elements, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_tuple_expr(fallback, elements, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_tuple_expr(branch, elements, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_tuple_expr(fallback, elements, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (*pattern, generic_tuple_expr(branch, elements, context)))
                .collect(),
            fallback: Box::new(generic_tuple_expr(fallback, elements, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_tuple_expr(return_, elements, context)),
        },
    };

    execution::TupleExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_custom_expr(
    expression: &module::GenericExpr,
    shape: &super::super::specialization::ConcreteCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::CustomExpr {
    let lowered_shape = context.lower_concrete_custom_shape(shape);
    let kind = generic_custom_expr_kind(expression, shape, context);
    execution::CustomExpr::from_parts(lowered_shape, kind)
}

pub(in crate::plan::execution::lowering) fn generic_custom_expr_kind(
    expression: &module::GenericExpr,
    shape: &super::super::specialization::ConcreteCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::CustomExprKind {
    use execution::CustomExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::CustomLocal::new(
                execution::CustomLocalId(context.generic_local_index(local.id())),
                context.lower_concrete_custom_shape(shape),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.custom_function_id(function, shape),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => {
            E::FunctionCall(execution::CustomFunctionCall::from_parts(
                super::generic_custom_function_expr(function, shape, context),
                super::call_args(args, context).into_boxed_slice(),
            ))
        }
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::generic_custom_list_expr(list, shape, context)),
            index: *index,
        },
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_custom_expr_kind(true_, shape, context)),
            false_: Box::new(generic_custom_expr_kind(false_, shape, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_custom_expr_kind(branch, shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_custom_expr_kind(fallback, shape, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_custom_expr_kind(branch, shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_custom_expr_kind(fallback, shape, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (*pattern, generic_custom_expr_kind(branch, shape, context))
                })
                .collect(),
            fallback: Box::new(generic_custom_expr_kind(fallback, shape, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_custom_expr_kind(return_, shape, context)),
        },
    }
}

pub(in crate::plan::execution::lowering) fn generic_list_value_expr(
    expression: &module::GenericExpr,
    item_shape: &super::super::specialization::ConcreteValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::ListExpr {
    use super::super::specialization::ConcreteValueShape as S;

    match item_shape {
        S::Int => execution::ListExpr::Int(generic_value_int_list_expr(expression, context)),
        S::String => {
            execution::ListExpr::String(generic_value_string_list_expr(expression, context))
        }
        S::BitArray => {
            execution::ListExpr::BitArray(generic_value_bit_array_list_expr(expression, context))
        }
        S::UtfCodepoint => execution::ListExpr::UtfCodepoint(
            generic_value_utf_codepoint_list_expr(expression, context),
        ),
        S::Custom(shape) => {
            execution::ListExpr::Custom(generic_value_custom_list_expr(expression, shape, context))
        }
        S::Float => execution::ListExpr::Float(generic_value_float_list_expr(expression, context)),
        S::Bool => execution::ListExpr::Bool(generic_value_bool_list_expr(expression, context)),
        S::Nil => execution::ListExpr::Nil(generic_value_nil_list_expr(expression, context)),
        S::Tuple(elements) => {
            execution::ListExpr::Tuple(generic_value_tuple_list_expr(expression, elements, context))
        }
        S::List(item) => {
            execution::ListExpr::List(generic_value_nested_list_expr(expression, item, context))
        }
        S::Function(function) => execution::ListExpr::Function(generic_value_function_list_expr(
            expression, function, context,
        )),
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
        ) -> $result {
            let item = execution::$item::new(context.$type_id());
            generic_value_typed_list_expr(
                expression,
                item,
                &super::super::specialization::ConcreteValueShape::$shape,
                execution::$local,
                |function, _, context| context.$function(function),
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
    shape: &super::super::specialization::ConcreteCustomValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::CustomListExpr {
    let item_shape = super::super::specialization::ConcreteValueShape::Custom(shape.clone());
    let item = execution::CustomListItem::new(
        context.custom_list_type(shape.to_module_shape().type_().clone()),
    );
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::CustomListLocalId,
        |function, item, context| context.custom_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_tuple_list_expr(
    expression: &module::GenericExpr,
    elements: &[super::super::specialization::ConcreteValueShape],
    context: &mut super::super::LoweringContext,
) -> execution::TupleListExpr {
    let item_shape = super::super::specialization::ConcreteValueShape::Tuple(
        elements.to_vec().into_boxed_slice(),
    );
    let item = execution::TupleListItem::new(
        context.tuple_list_type(elements.iter().map(|shape| shape.value_type()).collect()),
    );
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::TupleListLocalId,
        |function, item, context| context.tuple_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_nested_list_expr(
    expression: &module::GenericExpr,
    nested_item: &super::super::specialization::ConcreteValueShape,
    context: &mut super::super::LoweringContext,
) -> execution::ListListExpr {
    let item_shape =
        super::super::specialization::ConcreteValueShape::List(Box::new(nested_item.clone()));
    let item = execution::ListListItem::new(context.list_list_type(nested_item.value_type()));
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::ListListLocalId,
        |function, item, context| context.list_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_value_function_list_expr(
    expression: &module::GenericExpr,
    function: &super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::LoweringContext,
) -> execution::FunctionListExpr {
    let item_shape =
        super::super::specialization::ConcreteValueShape::Function(Box::new(function.clone()));
    let item = execution::FunctionListItem::new(
        context.function_list_type(function.to_module_shape().type_()),
    );
    generic_value_typed_list_expr(
        expression,
        item,
        &item_shape,
        execution::FunctionListLocalId,
        |function, item, context| context.function_list_function_id(function, item.type_id()),
        context,
    )
}

fn generic_value_typed_list_expr<Item>(
    expression: &module::GenericExpr,
    item: Item,
    item_shape: &super::super::specialization::ConcreteValueShape,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut super::super::LoweringContext,
    ) -> Item::Function,
    context: &mut super::super::LoweringContext,
) -> execution::TypedListExpr<Item>
where
    Item: execution::ListItem,
{
    let kind = generic_value_typed_list_kind(
        expression,
        &item,
        item_shape,
        lower_local,
        lower_function,
        context,
    );
    execution::TypedListExpr::from_item_and_kind(item, kind)
}

fn generic_value_typed_list_kind<Item>(
    expression: &module::GenericExpr,
    item: &Item,
    item_shape: &super::super::specialization::ConcreteValueShape,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut super::super::LoweringContext,
    ) -> Item::Function,
    context: &mut super::super::LoweringContext,
) -> execution::TypedListExprKind<Item>
where
    Item: execution::ListItem,
{
    use execution::TypedListExprKind as E;
    use module::GenericExprKind as M;

    match expression.kind() {
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: lower_local(context.generic_local_index(local.id())),
        },
        M::Call { function, args } => E::Call {
            function: lower_function(function, item, context),
            args: super::call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(super::generic_list_function_expr(
                function, item_shape, context,
            )),
            args: super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex { list, index } => E::ListIndex(execution::ListIndexSource::from_parts(
            super::generic_nested_list_expr(list, item_shape, context),
            *index,
        )),
        M::Panic(panic) => E::Panic(super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::bool_expr(subject, context)),
            true_: Box::new(generic_value_typed_list_kind(
                true_,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
            false_: Box::new(generic_value_typed_list_kind(
                false_,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_typed_list_kind(
                            branch,
                            item,
                            item_shape,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_typed_list_kind(
                fallback,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_value_typed_list_kind(
                            branch,
                            item,
                            item_shape,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_typed_list_kind(
                fallback,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_value_typed_list_kind(
                            branch,
                            item,
                            item_shape,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_value_typed_list_kind(
                fallback,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_value_typed_list_kind(
                return_,
                item,
                item_shape,
                lower_local,
                lower_function,
                context,
            )),
        },
    }
}
