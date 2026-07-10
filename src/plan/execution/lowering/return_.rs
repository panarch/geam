use super::expression::{
    bool_expr, bool_function_expr, bool_list_expr, call_args, float_expr, float_function_expr,
    float_list_expr, function_function_expr, function_list_expr, int_expr, int_function_expr,
    int_list_expr, list_function_expr, list_list_expr, nil_expr, nil_function_expr, nil_list_expr,
    string_expr, string_function_expr, string_list_expr, tuple_expr, tuple_function_expr,
    tuple_list_expr,
};
use super::id::list_function_function_id;
use crate::plan::{execution, module};

pub(super) fn int_return(body: module::IntReturn) -> execution::IntReturn {
    return_body(body, int_expr, |id| execution::IntFunctionId(id.0))
}

pub(super) fn float_return(body: module::FloatReturn) -> execution::FloatReturn {
    return_body(body, float_expr, |id| execution::FloatFunctionId(id.0))
}

pub(super) fn string_return(body: module::StringReturn) -> execution::StringReturn {
    return_body(body, string_expr, |id| execution::StringFunctionId(id.0))
}

pub(super) fn bool_return(body: module::BoolReturn) -> execution::BoolReturn {
    return_body(body, bool_expr, |id| execution::BoolFunctionId(id.0))
}

pub(super) fn nil_return(body: module::NilReturn) -> execution::NilReturn {
    return_body(body, nil_expr, |id| execution::NilFunctionId(id.0))
}

pub(super) fn tuple_return(body: module::TupleReturn) -> execution::TupleReturn {
    return_body(body, tuple_expr, |id| execution::TupleFunctionId(id.0))
}

pub(super) fn int_list_return(body: module::IntListReturn) -> execution::IntListReturn {
    return_body(body, int_list_expr, |id| execution::IntListFunctionId(id.0))
}

pub(super) fn string_list_return(body: module::StringListReturn) -> execution::StringListReturn {
    return_body(body, string_list_expr, |id| {
        execution::StringListFunctionId(id.0)
    })
}

pub(super) fn float_list_return(body: module::FloatListReturn) -> execution::FloatListReturn {
    return_body(body, float_list_expr, |id| {
        execution::FloatListFunctionId(id.0)
    })
}

pub(super) fn bool_list_return(body: module::BoolListReturn) -> execution::BoolListReturn {
    return_body(body, bool_list_expr, |id| {
        execution::BoolListFunctionId(id.0)
    })
}

pub(super) fn nil_list_return(body: module::NilListReturn) -> execution::NilListReturn {
    return_body(body, nil_list_expr, |id| execution::NilListFunctionId(id.0))
}

pub(super) fn tuple_list_return(body: module::TupleListReturn) -> execution::TupleListReturn {
    return_body(body, tuple_list_expr, |id| {
        execution::TupleListFunctionId(id.0)
    })
}

pub(super) fn list_list_return(body: module::ListListReturn) -> execution::ListListReturn {
    return_body(body, list_list_expr, |id| {
        execution::ListListFunctionId(id.0)
    })
}

pub(super) fn function_list_return(
    body: module::FunctionListReturn,
) -> execution::FunctionListReturn {
    return_body(body, function_list_expr, |id| {
        execution::FunctionListFunctionId(id.0)
    })
}

pub(super) fn int_function_return(body: module::IntFunctionReturn) -> execution::IntFunctionReturn {
    return_body(body, int_function_expr, |id| {
        execution::IntFunctionFunctionId(id.0)
    })
}

pub(super) fn float_function_return(
    body: module::FloatFunctionReturn,
) -> execution::FloatFunctionReturn {
    return_body(body, float_function_expr, |id| {
        execution::FloatFunctionFunctionId(id.0)
    })
}

pub(super) fn string_function_return(
    body: module::StringFunctionReturn,
) -> execution::StringFunctionReturn {
    return_body(body, string_function_expr, |id| {
        execution::StringFunctionFunctionId(id.0)
    })
}

pub(super) fn bool_function_return(
    body: module::BoolFunctionReturn,
) -> execution::BoolFunctionReturn {
    return_body(body, bool_function_expr, |id| {
        execution::BoolFunctionFunctionId(id.0)
    })
}

pub(super) fn nil_function_return(body: module::NilFunctionReturn) -> execution::NilFunctionReturn {
    return_body(body, nil_function_expr, |id| {
        execution::NilFunctionFunctionId(id.0)
    })
}

pub(super) fn tuple_function_return(
    body: module::TupleFunctionReturn,
) -> execution::TupleFunctionReturn {
    return_body(body, tuple_function_expr, |id| {
        execution::TupleFunctionFunctionId(id.0)
    })
}

pub(super) fn list_function_return(
    body: module::ListFunctionReturn,
) -> execution::ListFunctionReturn {
    return_body(body, list_function_expr, list_function_function_id)
}

pub(super) fn function_function_return(
    body: module::FunctionFunctionReturn,
) -> execution::FunctionFunctionReturn {
    return_body(body, function_function_expr, |id| {
        execution::FunctionFunctionFunctionId(id.0)
    })
}

fn return_body<ModuleExpression, ModuleFunction, ExecutionExpression, ExecutionFunction>(
    body: module::ReturnBody<ModuleExpression, ModuleFunction>,
    lower_expression: fn(ModuleExpression) -> ExecutionExpression,
    lower_function: fn(ModuleFunction) -> ExecutionFunction,
) -> execution::ReturnBody<ExecutionExpression, ExecutionFunction> {
    use execution::ReturnBodyKind as E;
    use module::ReturnBodyKind as M;

    let kind = match body.into_kind() {
        M::Expr(expression) => E::Expr(lower_expression(expression)),
        M::TailCall { function, args } => E::TailCall {
            function: lower_function(function),
            args: call_args(args),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: bool_expr(subject),
            true_: Box::new(return_body(*true_, lower_expression, lower_function)),
            false_: Box::new(return_body(*false_, lower_expression, lower_function)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: int_expr(subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(*fallback, lower_expression, lower_function)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: float_expr(subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(*fallback, lower_expression, lower_function)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: string_expr(subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(*fallback, lower_expression, lower_function)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps),
            return_: Box::new(return_body(*return_, lower_expression, lower_function)),
        },
    };

    execution::ReturnBody::from_kind(kind)
}
