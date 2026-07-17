use super::function_function_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn list_function_expr(
    expression: &module::ListFunctionExpr,
    context: &mut super::super::super::LoweringContext,
) -> execution::ListFunctionExpr {
    use execution::ListFunctionExprKind as E;
    use module::ListFunctionExprKind as M;

    let function_shape = context.concrete_function_shape(
        &crate::plan::FunctionShape::from_function_type(expression.type_().clone()),
    );
    let item_shape = context.concrete_value_shape(&crate::plan::ValueShape::from_value_type(
        expression.return_item_type(),
    ));
    let kind = match expression.kind() {
        M::Reference(value) => E::Reference(super::function_reference(
            value,
            context,
            |function, context| context.list_function_id(function, &item_shape),
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
            |function, context| context.list_function_id(function, &item_shape),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: crate::plan::execution::lowering::id::list_function_local(local, context),
        },
        M::Call {
            function,
            args,
            type_: _,
        } => E::Call {
            function: context.list_function_function_id(function, &function_shape, &item_shape),
            args: super::super::direct_call_args(function, args, context),
        },
        M::FunctionCall {
            function,
            args,
            type_: _,
        } => E::FunctionCall {
            function: Box::new(function_function_expr(function, context)),
            args: super::super::call_args(args, context),
        },
        M::TupleIndex {
            tuple,
            index,
            type_,
        } => E::TupleIndex {
            tuple: Box::new(super::super::tuple_expr(tuple, context)),
            index: *index,
            type_: context.function_type(type_.clone()),
        },
        M::CustomField(access) => {
            E::CustomField(super::super::custom_field_access(access, context))
        }
        M::ListIndex { list, index, type_ } => E::ListIndex {
            list: Box::new(super::super::function_list_expr(list, context)),
            index: *index,
            type_: context.function_type(type_.clone()),
        },
        M::Panic(value) => E::Panic(super::super::panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(super::super::bool_expr(subject, context)),
            true_: Box::new(list_function_expr(true_, context)),
            false_: Box::new(list_function_expr(false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(super::super::int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (pattern.clone(), list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(super::super::string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (pattern.clone(), list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(super::super::float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (*pattern, list_function_expr(branch, context)))
                .collect(),
            fallback: Box::new(list_function_expr(fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: crate::plan::execution::lowering::step::steps(steps, context),
            return_: Box::new(list_function_expr(return_, context)),
        },
    };

    execution::ListFunctionExpr::from_kind(kind)
}

pub(in crate::plan::execution::lowering) fn generic_list_function_expr(
    expression: &module::GenericFunctionExpr,
    item_shape: &super::super::super::specialization::ConcreteValueShape,
    context: &mut super::super::super::LoweringContext,
) -> execution::ListFunctionExpr {
    use execution::ListFunctionExprKind as E;
    use module::GenericFunctionExprKind as M;

    let function_shape = context.concrete_function_shape(&expression.shape());
    let type_ = context.lower_concrete_function_type(&function_shape);
    let kind = match expression.kind() {
        M::Reference(reference) => E::Reference(super::function_reference(
            reference,
            context,
            |function, context| context.list_function_id(function, item_shape),
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
            |function, context| context.list_function_id(function, item_shape),
        )),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: super::super::super::frame::generic_list_returning_function_local(
                local, item_shape, context,
            ),
        },
        M::Call { function, args } => E::Call {
            function: context.list_function_function_id(function, &function_shape, item_shape),
            args: super::super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(function_function_expr(function, context)),
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
            true_: Box::new(generic_list_function_expr(true_, item_shape, context)),
            false_: Box::new(generic_list_function_expr(false_, item_shape, context)),
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
                        generic_list_function_expr(branch, item_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_list_function_expr(fallback, item_shape, context)),
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
                        generic_list_function_expr(branch, item_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_list_function_expr(fallback, item_shape, context)),
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
                        generic_list_function_expr(branch, item_shape, context),
                    )
                })
                .collect(),
            fallback: Box::new(generic_list_function_expr(fallback, item_shape, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::super::step::steps(steps, context),
            return_: Box::new(generic_list_function_expr(return_, item_shape, context)),
        },
    };

    execution::ListFunctionExpr::from_kind(kind)
}
