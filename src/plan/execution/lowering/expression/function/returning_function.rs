use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn function_function_expr(
    expression: &module::FunctionFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::FunctionFunctionExpr {
    let return_shape =
        context.concrete_function_shape(expression.function_function_type().return_shape());
    let type_ = context.function_function_type(expression.function_function_type().clone());
    let kind = function_function_expr_kind(expression.kind(), &return_shape, &type_, context);

    execution::FunctionFunctionExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_function_function_expr(
    expression: &module::GenericFunctionExpr,
    return_shape: &super::super::super::specialization::ConcreteFunctionShape,
    context: &mut super::super::super::LoweringContext,
) -> execution::FunctionFunctionExpr {
    let shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
        shape
            .arguments()
            .iter()
            .map(super::super::super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let kind = generic_function_function_expr_kind(expression, return_shape, &type_, context);

    execution::FunctionFunctionExpr::from_parts(type_, kind)
}

pub(in crate::plan::execution::lowering) fn generic_function_function_expr_kind(
    expression: &module::GenericFunctionExpr,
    return_shape: &super::super::super::specialization::ConcreteFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::super::LoweringContext,
) -> execution::FunctionFunctionExprKind {
    use execution::FunctionFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    match expression.kind() {
        M::Reference(reference) => E::Reference(super::function_reference(
            reference,
            context,
            |function, context| context.function_function_id(function, return_shape),
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
            |function, context| context.function_function_id(function, return_shape),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(
                    context.generic_function_local_index(local.id()),
                ),
                type_.clone(),
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.function_function_function_id(function, type_.clone()),
            args: super::super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(function_function_expr(function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(list, context)),
            index: *index,
        },
        M::Panic(panic) => E::Panic(super::super::panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(subject, context)),
            true_: Box::new(generic_function_function_expr_kind(
                true_,
                return_shape,
                type_,
                context,
            )),
            false_: Box::new(generic_function_function_expr_kind(
                false_,
                return_shape,
                type_,
                context,
            )),
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
                        generic_function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
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
                        generic_function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::super::step::steps(steps, context),
            return_: Box::new(generic_function_function_expr_kind(
                return_,
                return_shape,
                type_,
                context,
            )),
        },
    }
}

pub(in crate::plan::execution::lowering) fn function_function_expr_kind(
    kind: &module::FunctionFunctionExprKind,
    return_shape: &super::super::super::specialization::ConcreteFunctionShape,
    type_: &execution::FunctionFunctionType,
    context: &mut super::super::super::LoweringContext,
) -> execution::FunctionFunctionExprKind {
    use execution::FunctionFunctionExprKind as E;
    use module::FunctionFunctionExprKind as M;

    match kind {
        M::Reference(value) => E::Reference(super::function_reference(
            value,
            context,
            |function, context| context.function_function_id(function, return_shape),
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
            |function, context| context.function_function_id(function, return_shape),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: crate::plan::execution::lowering::id::function_function_local(local, context),
        },
        M::Call { function, args } => E::Call {
            function: context.function_function_function_id(function, type_.clone()),
            args: super::super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(function_function_expr(function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(list, context)),
            index: *index,
        },
        M::Panic(value) => E::Panic(super::super::panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(subject, context)),
            true_: Box::new(function_function_expr_kind(
                true_,
                return_shape,
                type_,
                context,
            )),
            false_: Box::new(function_function_expr_kind(
                false_,
                return_shape,
                type_,
                context,
            )),
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
                        function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
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
                        function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        function_function_expr_kind(branch, return_shape, type_, context),
                    )
                })
                .collect(),
            fallback: Box::new(function_function_expr_kind(
                fallback,
                return_shape,
                type_,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps, context),
            return_: Box::new(function_function_expr_kind(
                return_,
                return_shape,
                type_,
                context,
            )),
        },
    }
}
