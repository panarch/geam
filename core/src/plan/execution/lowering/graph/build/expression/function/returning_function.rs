use super::super::{call_args, custom, list, tuple};
use super::{closure, reference, source_stop};
use crate::plan::execution::lowering::graph::DraftFunctionTarget;
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftFlow, DraftFunction, DraftFunctionFunction, DraftGraph,
};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedFunctionShape, StoredValueShape,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn function_function_expr(
    expression: &module::FunctionFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunctionFunction>> {
    let return_shape =
        context.concrete_function_shape(expression.function_function_type().return_shape());
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.function_function_type().to_function_type(),
    ));
    function_function_expr_kind(
        expression.kind(),
        &return_shape,
        &shape,
        cursor,
        graph,
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_function_function_expr(
    expression: &module::GenericFunctionExpr,
    return_shape: &SpecializedFunctionShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    generic_function_function_expr_kind(
        expression.kind(),
        return_shape,
        shape,
        cursor,
        graph,
        context,
    )
}

pub(in crate::plan::execution::lowering) fn function_function_expr_kind(
    kind: &module::FunctionFunctionExprKind,
    return_shape: &SpecializedFunctionShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunctionFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::FunctionFunctionExprKind as E;

    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    let type_ = context.specialized_function_function_type(shape.arguments(), return_shape);
    match kind {
        E::Constant(value) => context.function_function_constant(value).map(|id| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::constant::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, DraftFunctionFunction::new(value))
        }),
        E::Reference(value) => context
            .function_function_id(value.instantiation(), return_shape)
            .map(|target| {
                reference(
                    shape.clone(),
                    DraftFunctionTarget::Function(target),
                    cursor,
                    graph,
                )
                .map(DraftFunctionFunction::new)
            }),
        E::Closure { function, captures } => context
            .function_function_id(function, return_shape)
            .and_then(|target| {
                closure(
                    function,
                    captures,
                    shape.clone(),
                    DraftFunctionTarget::Function(target),
                    cursor,
                    graph,
                    context,
                )
                .map(|flow| flow.map(DraftFunctionFunction::new))
            }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::FunctionFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftFunctionFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => context
                .function_function_function_id(function, type_.clone())
                .map(|function| {
                    let function = execution::function::FunctionFunctionId::Function(function);
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, DraftFunctionFunction::new(value))
                }),
        }),
        E::FunctionCall {
            function,
            args,
            site,
        } => super::function_value_call(function, args, site, shape, cursor, graph, context)
            .map(|flow| flow.map(DraftFunctionFunction::new)),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftFunctionFunction::new(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, DraftFunctionFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftFunctionFunction::new(value))
            }
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context)
            .map(|flow| flow.map(DraftFunctionFunction::new)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| {
                function_function_expr_kind(true_, return_shape, shape, cursor, graph, context)
            },
            |cursor, graph, context| {
                function_function_expr_kind(false_, return_shape, shape, cursor, graph, context)
            },
            DraftFunctionFunction::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                function_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftFunctionFunction::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                function_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftFunctionFunction::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                function_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftFunctionFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => function_function_expr_kind(
                        return_,
                        return_shape,
                        shape,
                        cursor,
                        graph,
                        context,
                    ),
                }
            })
        }
    }
}

pub(super) fn generic_function_function_expr_kind(
    kind: &module::GenericFunctionExprKind,
    return_shape: &SpecializedFunctionShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    super::generic::lower_executable_kind(
        kind,
        shape,
        cursor,
        graph,
        context,
        super::generic::executable_kind_lowering(
            std::convert::identity,
            |value, context| {
                context
                    .generic_function_function_constant(value, return_shape, shape)
                    .map(|id| id.index())
            },
            |function, context| {
                context
                    .function_function_id(function, return_shape)
                    .map(DraftFunctionTarget::Function)
            },
            |function, context| {
                let type_ =
                    context.specialized_function_function_type(shape.arguments(), return_shape);
                context
                    .function_function_function_id(function, type_)
                    .map(execution::function::FunctionFunctionId::Function)
            },
            |branch, cursor, graph, context| {
                generic_function_function_expr_kind(
                    branch,
                    return_shape,
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        ),
    )
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::lowering::graph::DraftValueRef;
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::specialization::Representability;
    use crate::plan::{
        CallArg, Expr, FunctionFunctionExpr, FunctionFunctionType, FunctionShape, FunctionType,
        GenericExpr, GenericLocal, GenericLocalId, PanicExpr, PanicSite, TypeParameterId,
        ValueShape, ValueType,
    };

    #[test]
    fn function_function_call_preserves_callee_stop_before_an_uninhabited_argument() {
        let parameter = TypeParameterId(0);
        let returned_function = FunctionShape::new(
            Vec::new(),
            ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int))),
        );
        let callee = FunctionFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            FunctionFunctionType::from_shapes(
                vec![ValueShape::Parameter(parameter)],
                returned_function.clone(),
            ),
        );
        let argument = Expr::generic(GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "value".into(),
        ));
        let call = FunctionFunctionExpr::function_call(
            callee,
            vec![CallArg::new(argument)],
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int)),
        );
        assert_eq!(
            call.function_function_type().to_function_type(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        );

        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let flow = super::function_function_expr(&call, cursor, &mut graph, &mut context)
            .map(|flow| flow.fold(false, |_, _| true));

        assert_eq!(flow, Representability::Inhabited(false));
    }
}
