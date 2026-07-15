use super::LoweringContext;
use super::expression::{
    bit_array_expr, bit_array_function_expr, bit_array_list_expr, bool_expr, bool_function_expr,
    bool_list_expr, call_args, custom_expr_kind, custom_function_expr_kind, custom_list_expr,
    float_expr, float_function_expr, float_list_expr, function_function_expr_kind,
    function_list_expr, int_expr, int_function_expr, int_list_expr, list_function_expr,
    list_list_expr, nil_expr, nil_function_expr, nil_list_expr, string_expr, string_function_expr,
    string_list_expr, tuple_expr, tuple_function_expr, tuple_list_expr, utf_codepoint_expr,
    utf_codepoint_function_expr, utf_codepoint_list_expr,
};
use super::id::list_function_function_id;
use crate::plan::{execution, module};

pub(super) fn int_return(
    body: module::IntReturn,
    context: &mut LoweringContext,
) -> execution::IntReturn {
    return_body(body, context, int_expr, |id, _| {
        execution::IntFunctionId(id.0)
    })
}

pub(super) fn float_return(
    body: module::FloatReturn,
    context: &mut LoweringContext,
) -> execution::FloatReturn {
    return_body(body, context, float_expr, |id, _| {
        execution::FloatFunctionId(id.0)
    })
}

pub(super) fn string_return(
    body: module::StringReturn,
    context: &mut LoweringContext,
) -> execution::StringReturn {
    return_body(body, context, string_expr, |id, _| {
        execution::StringFunctionId(id.0)
    })
}

pub(super) fn bit_array_return(
    body: module::BitArrayReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayReturn {
    return_body(body, context, bit_array_expr, |id, _| {
        execution::BitArrayFunctionId(id.0)
    })
}

pub(super) fn utf_codepoint_return(
    body: module::UtfCodepointReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointReturn {
    return_body(body, context, utf_codepoint_expr, |id, _| {
        execution::UtfCodepointFunctionId(id.0)
    })
}

pub(super) fn custom_return(
    body: module::CustomReturn,
    context: &mut LoweringContext,
) -> execution::CustomReturn {
    let (shape, body) = body.into_parts();
    let shape = context.custom_value_shape(shape);
    let body = return_body(body, context, custom_expr_kind, |index, _| index);
    execution::CustomReturn::from_parts(shape, body)
}

pub(super) fn bool_return(
    body: module::BoolReturn,
    context: &mut LoweringContext,
) -> execution::BoolReturn {
    return_body(body, context, bool_expr, |id, _| {
        execution::BoolFunctionId(id.0)
    })
}

pub(super) fn nil_return(
    body: module::NilReturn,
    context: &mut LoweringContext,
) -> execution::NilReturn {
    return_body(body, context, nil_expr, |id, _| {
        execution::NilFunctionId(id.0)
    })
}

pub(super) fn tuple_return(
    body: module::TupleReturn,
    context: &mut LoweringContext,
) -> execution::TupleReturn {
    return_body(body, context, tuple_expr, |id, _| {
        execution::TupleFunctionId(id.0)
    })
}
pub(super) fn int_list_return(
    body: module::IntListReturn,
    context: &mut LoweringContext,
) -> execution::IntListReturn {
    let type_id = context.int_list_type();
    return_body(body, context, int_list_expr, move |id, _| {
        execution::IntListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn string_list_return(
    body: module::StringListReturn,
    context: &mut LoweringContext,
) -> execution::StringListReturn {
    let type_id = context.string_list_type();
    return_body(body, context, string_list_expr, move |id, _| {
        execution::StringListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn bit_array_list_return(
    body: module::BitArrayListReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayListReturn {
    let type_id = context.bit_array_list_type();
    return_body(body, context, bit_array_list_expr, move |id, _| {
        execution::BitArrayListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn utf_codepoint_list_return(
    body: module::UtfCodepointListReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointListReturn {
    let type_id = context.utf_codepoint_list_type();
    return_body(body, context, utf_codepoint_list_expr, move |id, _| {
        execution::UtfCodepointListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn custom_list_return(
    body: module::CustomListReturn,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> execution::CustomListReturn {
    return_body(body, context, custom_list_expr, move |id, _| {
        execution::CustomListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn float_list_return(
    body: module::FloatListReturn,
    context: &mut LoweringContext,
) -> execution::FloatListReturn {
    let type_id = context.float_list_type();
    return_body(body, context, float_list_expr, move |id, _| {
        execution::FloatListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn bool_list_return(
    body: module::BoolListReturn,
    context: &mut LoweringContext,
) -> execution::BoolListReturn {
    let type_id = context.bool_list_type();
    return_body(body, context, bool_list_expr, move |id, _| {
        execution::BoolListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn nil_list_return(
    body: module::NilListReturn,
    context: &mut LoweringContext,
) -> execution::NilListReturn {
    let type_id = context.nil_list_type();
    return_body(body, context, nil_list_expr, move |id, _| {
        execution::NilListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn tuple_list_return(
    body: module::TupleListReturn,
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> execution::TupleListReturn {
    return_body(body, context, tuple_list_expr, move |id, _| {
        execution::TupleListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn list_list_return(
    body: module::ListListReturn,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> execution::ListListReturn {
    return_body(body, context, list_list_expr, move |id, _| {
        execution::ListListFunctionId::new(id.0, type_id)
    })
}

pub(super) fn function_list_return(
    body: module::FunctionListReturn,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> execution::FunctionListReturn {
    return_body(body, context, function_list_expr, move |id, _| {
        execution::FunctionListFunctionId::new(id.0, type_id)
    })
}
pub(super) fn int_function_return(
    shape: crate::plan::FunctionShape,
    body: module::IntFunctionReturn,
    context: &mut LoweringContext,
) -> execution::IntFunctionReturn {
    let body = return_body(body, context, int_function_expr, |id, _| {
        execution::IntFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn float_function_return(
    shape: crate::plan::FunctionShape,
    body: module::FloatFunctionReturn,
    context: &mut LoweringContext,
) -> execution::FloatFunctionReturn {
    let body = return_body(body, context, float_function_expr, |id, _| {
        execution::FloatFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn string_function_return(
    shape: crate::plan::FunctionShape,
    body: module::StringFunctionReturn,
    context: &mut LoweringContext,
) -> execution::StringFunctionReturn {
    let body = return_body(body, context, string_function_expr, |id, _| {
        execution::StringFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn bit_array_function_return(
    shape: crate::plan::FunctionShape,
    body: module::BitArrayFunctionReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayFunctionReturn {
    let body = return_body(body, context, bit_array_function_expr, |id, _| {
        execution::BitArrayFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn utf_codepoint_function_return(
    shape: crate::plan::FunctionShape,
    body: module::UtfCodepointFunctionReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointFunctionReturn {
    let body = return_body(body, context, utf_codepoint_function_expr, |id, _| {
        execution::UtfCodepointFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn custom_function_return(
    shape: crate::plan::FunctionShape,
    body: module::CustomFunctionReturn,
    context: &mut LoweringContext,
) -> execution::CustomFunctionReturn {
    let (type_, body) = body.into_parts();
    let type_ = context.custom_function_type(type_);
    let body = return_body(body, context, custom_function_expr_kind, |index, _| index);
    execution::CustomFunctionReturn::from_parts(context.function_shape(shape), type_, body)
}

pub(super) fn bool_function_return(
    shape: crate::plan::FunctionShape,
    body: module::BoolFunctionReturn,
    context: &mut LoweringContext,
) -> execution::BoolFunctionReturn {
    let body = return_body(body, context, bool_function_expr, |id, _| {
        execution::BoolFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn nil_function_return(
    shape: crate::plan::FunctionShape,
    body: module::NilFunctionReturn,
    context: &mut LoweringContext,
) -> execution::NilFunctionReturn {
    let body = return_body(body, context, nil_function_expr, |id, _| {
        execution::NilFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn tuple_function_return(
    shape: crate::plan::FunctionShape,
    body: module::TupleFunctionReturn,
    context: &mut LoweringContext,
) -> execution::TupleFunctionReturn {
    let body = return_body(body, context, tuple_function_expr, |id, _| {
        execution::TupleFunctionFunctionId(id.0)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn list_function_return(
    shape: crate::plan::FunctionShape,
    body: module::ListFunctionReturn,
    context: &mut LoweringContext,
) -> execution::ListFunctionReturn {
    let body = return_body(body, context, list_function_expr, list_function_function_id);
    execution::TypedFunctionReturn::new(context.function_shape(shape), body)
}

pub(super) fn function_function_return(
    shape: crate::plan::FunctionShape,
    body: module::FunctionFunctionReturn,
    context: &mut LoweringContext,
) -> execution::FunctionFunctionReturn {
    let (type_, body) = body.into_parts();
    let type_ = context.function_function_type(type_);
    let body = return_body(body, context, function_function_expr_kind, |index, _| index);
    execution::FunctionFunctionReturn::from_parts(context.function_shape(shape), type_, body)
}

fn return_body<ModuleExpression, ModuleFunction, ExecutionExpression, ExecutionFunction>(
    body: module::ReturnBody<ModuleExpression, ModuleFunction>,
    context: &mut LoweringContext,
    lower_expression: impl Copy + Fn(ModuleExpression, &mut LoweringContext) -> ExecutionExpression,
    lower_function: impl Copy + Fn(ModuleFunction, &mut LoweringContext) -> ExecutionFunction,
) -> execution::ReturnBody<ExecutionExpression, ExecutionFunction> {
    use execution::ReturnBodyKind as E;
    use module::ReturnBodyKind as M;

    let kind = match body.into_kind() {
        M::Expr(expression) => E::Expr(lower_expression(expression, context)),
        M::TailCall { function, args } => E::TailCall {
            function: lower_function(function, context),
            args: call_args(args, context),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: bool_expr(subject, context),
            true_: Box::new(return_body(
                *true_,
                context,
                lower_expression,
                lower_function,
            )),
            false_: Box::new(return_body(
                *false_,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: int_expr(subject, context),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                *fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: float_expr(subject, context),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                *fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: string_expr(subject, context),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| {
                    (
                        pattern,
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                *fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(return_body(
                *return_,
                context,
                lower_expression,
                lower_function,
            )),
        },
    };

    execution::ReturnBody::from_kind(kind)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        ExecutionPlan, FunctionFunctionId, ListFunctionId, ListListFunctionId, ReturnBody,
        ReturnBodyKind, RuntimeFunctionId,
    };

    #[test]
    fn lowering_seals_custom_callable_return_type_around_tail_indices() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn build(value: Int) -> Boxed { Boxed(value) }

fn factory() -> fn(Int) -> Boxed { factory() }

pub fn main() -> fn(Int) -> Boxed { factory() }
"#,
        );
        let main = plan.custom_function_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Custom(main.clone()),
                return_type: main.type_().to_function_type(),
            },
        );
        let return_ = plan.custom_function_function(&main).return_();

        assert_eq!(return_.type_(), main.type_());
        assert_eq!(return_.function_id(1).type_(), main.type_());
    }

    #[test]
    fn lowering_seals_nested_callable_return_type_around_tail_indices() {
        let plan = execution_plan(
            r#"
fn factory() -> fn() -> fn(Int) -> Int { factory() }

pub fn main() -> fn() -> fn(Int) -> Int { factory() }
"#,
        );
        let main = plan.function_function_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(main.clone()),
                return_type: main.type_().to_function_type(),
            },
        );
        let return_ = plan.function_function_function(&main).return_();

        assert_eq!(return_.type_(), main.type_());
        assert_eq!(return_.function_id(1).type_(), main.type_());
    }

    #[test]
    fn lowering_carries_exact_nested_list_type_through_tail_calls() {
        let source = r#"
fn repeat(values: List(List(Int))) -> List(List(Int)) {
  repeat(values)
}

pub fn main() -> List(List(Int)) { [] }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let function = plan.list_list_function_id(1);
        let next = expect_tail_call(plan.list_list_function(function).return_());
        let main = expect_list_list_main(&plan);

        assert_eq!(*next, function);
        assert_eq!(next.type_id(), function.type_id());
        assert_eq!(main.type_id(), function.type_id());
    }

    #[test]
    #[should_panic(expected = "expected a tail-call return body")]
    fn tail_call_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(List(Int)) { [] }");
        let main = expect_list_list_main(&plan);
        let _ = expect_tail_call(plan.list_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_tail_call<Expression>(
        body: &ReturnBody<Expression, ListListFunctionId>,
    ) -> &ListListFunctionId {
        match body.kind() {
            ReturnBodyKind::TailCall { function, .. } => function,
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_list_list_main(plan: &ExecutionPlan) -> ListListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::List(main)) => main,
            _ => panic!("expected a List(List) main function"),
        }
    }
}
