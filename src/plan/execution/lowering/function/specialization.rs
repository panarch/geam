use super::table::{FunctionTableBuilder, lowered_function, push_list_function_function};
use crate::plan::execution::lowering::LoweringContext;
use crate::plan::execution::lowering::graph;
use crate::plan::execution::lowering::specialization::{
    FunctionArgumentsRepresentation, FunctionRepresentation, Representability,
    SpecializedFunctionShape, SpecializedValueShape, StorageRepresentation, StoredValueShape,
    ValueInhabitation,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn lower_specialized(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    context: &mut LoweringContext,
) {
    let index = context.specialization_index(key);
    let return_shape = SpecializedValueShape::instantiate(
        template.signature().shape().return_shape(),
        key.substitution(),
    );
    let return_inhabitation = context.representations.inhabitation(&return_shape);
    let mut functions = std::mem::take(&mut context.functions);

    use module::ReturnExprKind as R;
    match template.return_().kind() {
        R::Generic { body, .. } => match &return_inhabitation {
            ValueInhabitation::Uninhabited(_) => {
                let graph = graph::lower_never_function_graph(
                    template,
                    body,
                    context,
                    graph::never_expr,
                    never_function_id,
                );
                functions
                    .never_functions
                    .push((index, lowered_function(key, graph)));
            }
            ValueInhabitation::Inhabited(shape) => {
                lower_generic_value(template, key, index, body, shape, &mut functions, context);
            }
        },
        R::Int { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::int_expr,
                |target, context| {
                    context.int_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .int_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::Float { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::float_expr,
                |target, context| {
                    context
                        .float_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .float_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::String { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::string_expr,
                |target, context| {
                    context
                        .string_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .string_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::BitArray { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bit_array_expr,
                |target, context| {
                    context
                        .bit_array_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .bit_array_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::UtfCodepoint { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::utf_codepoint_expr,
                |target, context| {
                    context
                        .utf_codepoint_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .utf_codepoint_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::Custom { body } => {
            lower_custom_return(template, key, index, body, &mut functions, context)
        }
        R::Bool { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bool_expr,
                |target, context| {
                    context.bool_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .bool_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::Nil { body } => {
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::nil_expr,
                |target, context| {
                    context.nil_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .nil_functions
                .push((index, lowered_function(key, lowered)));
        }
        R::Tuple { type_, body } => {
            lower_tuple_return(template, key, index, type_, body, &mut functions, context)
        }
        R::GenericList { parameter, body } => {
            let item = context.concrete_parameter(*parameter);
            lower_generic_list(template, key, index, body, &item, &mut functions, context);
        }
        R::ParameterListList { parameter, body } => {
            let item = context.concrete_parameter(*parameter);
            lower_parameter_list_list(template, key, index, body, &item, &mut functions, context);
        }
        R::IntList { body } => {
            let id = execution::function::IntListFunctionId::new(index, context.int_list_type());
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::int_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.int_list_function_id(function)
                    })
                },
            );
            functions
                .int_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::StringList { body } => {
            let id =
                execution::function::StringListFunctionId::new(index, context.string_list_type());
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::string_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.string_list_function_id(function)
                    })
                },
            );
            functions
                .string_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::BitArrayList { body } => {
            let id = execution::function::BitArrayListFunctionId::new(
                index,
                context.bit_array_list_type(),
            );
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bit_array_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.bit_array_list_function_id(function)
                    })
                },
            );
            functions
                .bit_array_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::UtfCodepointList { body } => {
            let id = execution::function::UtfCodepointListFunctionId::new(
                index,
                context.utf_codepoint_list_type(),
            );
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::utf_codepoint_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.utf_codepoint_list_function_id(function)
                    })
                },
            );
            functions
                .utf_codepoint_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::CustomList { item_type, body } => {
            let type_id = context.custom_list_type(item_type.clone());
            let id = execution::function::CustomListFunctionId::new(index, type_id);
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::custom_list_expr,
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.custom_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .custom_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::FloatList { body } => {
            let id =
                execution::function::FloatListFunctionId::new(index, context.float_list_type());
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::float_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.float_list_function_id(function)
                    })
                },
            );
            functions
                .float_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::BoolList { body } => {
            let id = execution::function::BoolListFunctionId::new(index, context.bool_list_type());
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bool_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.bool_list_function_id(function)
                    })
                },
            );
            functions
                .bool_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::NilList { body } => {
            let id = execution::function::NilListFunctionId::new(index, context.nil_list_type());
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::nil_list_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.nil_list_function_id(function)
                    })
                },
            );
            functions
                .nil_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::TupleList { item_type, body } => {
            let type_id = context.tuple_list_type(item_type.clone());
            let id = execution::function::TupleListFunctionId::new(index, type_id);
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::tuple_list_expr,
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.tuple_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .tuple_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::ListList { item_shape, body } => {
            let type_id = context.stored_list_list_type(item_shape);
            let id = execution::function::ListListFunctionId::new(index, type_id);
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::list_list_expr,
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.list_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .list_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::FunctionList { item_type, body } => {
            let type_id = context.function_list_type(item_type.clone());
            let id = execution::function::FunctionListFunctionId::new(index, type_id);
            let lowered = graph::lower_function_graph(
                template,
                body,
                context,
                graph::function_list_expr,
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.function_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .function_list_functions
                .push((id, lowered_function(key, lowered)));
        }
        R::GenericFunction { shape, body } => {
            let function = context.concrete_function_shape(shape);
            lower_generic_function(
                template,
                key,
                index,
                body,
                &function,
                &mut functions,
                context,
            );
        }
        R::IntFunction { shape, body } => {
            lower_int_function(template, key, index, shape, body, &mut functions, context)
        }
        R::FloatFunction { shape, body } => {
            lower_float_function(template, key, index, shape, body, &mut functions, context)
        }
        R::StringFunction { shape, body } => {
            lower_string_function(template, key, index, shape, body, &mut functions, context)
        }
        R::BitArrayFunction { shape, body } => {
            lower_bit_array_function(template, key, index, shape, body, &mut functions, context)
        }
        R::UtfCodepointFunction { shape, body } => {
            lower_utf_codepoint_function(template, key, index, shape, body, &mut functions, context)
        }
        R::CustomFunction { shape, body } => {
            lower_custom_function(template, key, index, shape, body, &mut functions, context)
        }
        R::BoolFunction { shape, body } => {
            lower_bool_function(template, key, index, shape, body, &mut functions, context)
        }
        R::NilFunction { shape, body } => {
            lower_nil_function(template, key, index, shape, body, &mut functions, context)
        }
        R::TupleFunction { shape, body } => {
            lower_tuple_function(template, key, index, shape, body, &mut functions, context)
        }
        R::ListFunction {
            shape,
            item_type,
            body,
        } => lower_list_function(
            template,
            key,
            index,
            ListFunctionDefinition {
                shape,
                item_type,
                body,
            },
            &mut functions,
            context,
        ),
        R::FunctionFunction { shape, body } => {
            lower_function_function(template, key, index, shape, body, &mut functions, context)
        }
    }

    context.functions = functions;
}

fn lower_custom_return(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::CustomReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let signature_shape = context.concrete_custom_value_shape(body.signature_shape());
    match context
        .representations
        .custom_inhabitation(&signature_shape)
    {
        super::super::specialization::CompoundInhabitation::Inhabited => {
            let body_shape = context.concrete_custom_value_shape(body.shape());
            let lowered_signature = context.lower_concrete_custom_shape(&signature_shape);
            let lowered_body = context.lower_concrete_custom_shape(&body_shape);
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::custom_expr_kind(kind, &body_shape, cursor, graph, context)
                },
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context
                            .custom_function_id(function, &signature_shape)
                            .map(|function| function.index())
                    })
                },
            )
            .map(|graph| {
                graph.map(|body| {
                    execution::function::CustomFunctionBody::from_parts(
                        lowered_signature,
                        lowered_body,
                        body,
                    )
                })
            });
            functions
                .custom_functions
                .push((index, lowered_function(key, graph)));
        }
        super::super::specialization::CompoundInhabitation::Uninhabited(proof) => {
            let graph = graph::lower_never_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::custom_never_expr_kind(kind, &proof, cursor, graph, context)
                },
                never_function_id,
            );
            functions
                .never_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_tuple_return(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    type_: &[crate::plan::ValueType],
    body: &module::TupleReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let elements = type_
        .iter()
        .cloned()
        .map(crate::plan::ValueShape::from_value_type)
        .map(|shape| context.concrete_value_shape(&shape))
        .collect::<Vec<_>>();
    match context.representations.tuple_inhabitation(&elements) {
        super::super::specialization::CompoundInhabitation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::tuple_expr,
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.tuple_function_id(function)
                    })
                },
            );
            functions
                .tuple_functions
                .push((index, lowered_function(key, graph)));
        }
        super::super::specialization::CompoundInhabitation::Uninhabited(proof) => {
            let graph = graph::lower_never_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::tuple_never_expr(expression, &proof, cursor, graph, context)
                },
                never_function_id,
            );
            functions
                .never_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn never_function_id(
    target: &crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    context: &mut LoweringContext,
) -> Representability<crate::plan::FunctionCallTarget<execution::function::NeverFunctionId>> {
    lower_call_target(target, context, |function, context| {
        context.never_function_id(function)
    })
}

fn lower_call_target<Function>(
    target: &crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower: impl FnOnce(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<Function>,
) -> Representability<crate::plan::FunctionCallTarget<Function>> {
    lower(target.function(), context)
        .map(|function| crate::plan::FunctionCallTarget::new(function, target.site().clone()))
}

fn lower_generic_value(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericReturn,
    shape: &StoredValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    use StoredValueShape as S;
    match shape {
        S::Int => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftInt::from_owned,
                    )
                },
                |target, context| {
                    context.int_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .int_functions
                .push((index, lowered_function(key, graph)));
        }
        S::Float => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftFloat::from_owned,
                    )
                },
                |target, context| {
                    context
                        .float_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .float_functions
                .push((index, lowered_function(key, graph)));
        }
        S::String => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftString::from_owned,
                    )
                },
                |target, context| {
                    context
                        .string_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .string_functions
                .push((index, lowered_function(key, graph)));
        }
        S::BitArray => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftBitArray::from_owned,
                    )
                },
                |target, context| {
                    context
                        .bit_array_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .bit_array_functions
                .push((index, lowered_function(key, graph)));
        }
        S::UtfCodepoint => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftUtfCodepoint::from_owned,
                    )
                },
                |target, context| {
                    context
                        .utf_codepoint_function_id(target.function())
                        .map(|function| {
                            crate::plan::FunctionCallTarget::new(function, target.site().clone())
                        })
                },
            );
            functions
                .utf_codepoint_functions
                .push((index, lowered_function(key, graph)));
        }
        S::Custom(shape) => {
            let lowered_shape = context.lower_concrete_custom_shape(shape);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftCustom::from_owned,
                    )
                },
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context
                            .custom_function_id(function, shape)
                            .map(|function| function.index())
                    })
                },
            )
            .map(|graph| {
                graph.map(|body| {
                    execution::function::CustomFunctionBody::from_parts(
                        lowered_shape,
                        lowered_shape,
                        body,
                    )
                })
            });
            functions
                .custom_functions
                .push((index, lowered_function(key, graph)));
        }
        S::Bool => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftBool::from_owned,
                    )
                },
                |target, context| {
                    context.bool_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .bool_functions
                .push((index, lowered_function(key, graph)));
        }
        S::Nil => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftNil::from_owned,
                    )
                },
                |target, context| {
                    context.nil_function_id(target.function()).map(|function| {
                        crate::plan::FunctionCallTarget::new(function, target.site().clone())
                    })
                },
            );
            functions
                .nil_functions
                .push((index, lowered_function(key, graph)));
        }
        S::Tuple(_) => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    lower_generic_as(
                        expression,
                        cursor,
                        graph,
                        context,
                        graph::DraftTuple::from_owned,
                    )
                },
                |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.tuple_function_id(function)
                    })
                },
            );
            functions
                .tuple_functions
                .push((index, lowered_function(key, graph)));
        }
        S::List(item) => {
            lower_generic_value_list(template, key, index, body, item, functions, context)
        }
        S::Function(function) => {
            lower_generic_value_function(template, key, index, body, function, functions, context)
        }
    }
}

fn lower_generic_as<Value>(
    expression: &module::GenericExpr,
    cursor: graph::DraftCursor,
    graph: &mut graph::DraftGraph,
    context: &mut LoweringContext,
    make: impl Copy + Fn(graph::DraftValueRef) -> Value,
) -> Representability<graph::DraftFlow<Value>> {
    graph::generic_expr(expression, cursor, graph, context).map(|flow| flow.map(make))
}

fn lower_generic_list(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericListReturn,
    item: &SpecializedValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item.storage_representation() {
        StorageRepresentation::Parameter(parameter) => {
            let type_id = context.parameter_list_type(parameter);
            let id = execution::function::ParameterListFunctionId::new(index, type_id);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::generic_list_expr(expression, cursor, graph, context)
                        .map(|flow| flow.map(graph::DraftParameterList::new))
                },
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.parameter_list_function_id(function, parameter)
                    })
                },
            );
            functions
                .parameter_list_functions
                .push((id, lowered_function(key, graph)));
        }
        StorageRepresentation::Stored(item) => {
            lower_generic_item_list(template, key, index, body, &item, functions, context)
        }
    }
}

fn lower_parameter_list_list(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::ParameterListListReturn,
    item: &SpecializedValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item.storage_representation() {
        StorageRepresentation::Parameter(parameter) => {
            let type_id = context.parameter_list_list_type(parameter);
            let id = execution::function::ParameterListListFunctionId::new(index, type_id);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::parameter_list_list_expr(expression, cursor, graph, context).map(
                        |flow| {
                            flow.map(|value| graph::DraftParameterListList::new(value.into_list()))
                        },
                    )
                },
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.parameter_list_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .parameter_list_list_functions
                .push((id, lowered_function(key, graph)));
        }
        StorageRepresentation::Stored(item) => {
            let type_id = context.specialized_stored_list_list_type(&item);
            let id = execution::function::ListListFunctionId::new(index, type_id);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::parameter_list_list_expr(expression, cursor, graph, context)
                        .map(|flow| flow.map(|value| graph::DraftListList::new(value.into_list())))
                },
                move |target, context| {
                    lower_call_target(target, context, |function, context| {
                        context.list_list_function_id(function, type_id)
                    })
                },
            );
            functions
                .list_list_functions
                .push((id, lowered_function(key, graph)));
        }
    }
}

fn lower_generic_item_list(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericListReturn,
    item: &StoredValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    use StoredValueShape as S;
    match item {
        S::Int => {
            let id = execution::function::IntListFunctionId::new(index, context.int_list_type());
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftIntList::new,
                |function, context| context.int_list_function_id(function),
            );
            functions
                .int_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::String => {
            let id =
                execution::function::StringListFunctionId::new(index, context.string_list_type());
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftStringList::new,
                |function, context| context.string_list_function_id(function),
            );
            functions
                .string_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::BitArray => {
            let id = execution::function::BitArrayListFunctionId::new(
                index,
                context.bit_array_list_type(),
            );
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftBitArrayList::new,
                |function, context| context.bit_array_list_function_id(function),
            );
            functions
                .bit_array_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::UtfCodepoint => {
            let id = execution::function::UtfCodepointListFunctionId::new(
                index,
                context.utf_codepoint_list_type(),
            );
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftUtfCodepointList::new,
                |function, context| context.utf_codepoint_list_function_id(function),
            );
            functions
                .utf_codepoint_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::Custom(shape) => {
            let type_id = context.specialized_custom_list_type(shape);
            let id = execution::function::CustomListFunctionId::new(index, type_id);
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftCustomList::new,
                move |function, context| context.custom_list_function_id(function, type_id),
            );
            functions
                .custom_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::Float => {
            let id =
                execution::function::FloatListFunctionId::new(index, context.float_list_type());
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftFloatList::new,
                |function, context| context.float_list_function_id(function),
            );
            functions
                .float_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::Bool => {
            let id = execution::function::BoolListFunctionId::new(index, context.bool_list_type());
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftBoolList::new,
                |function, context| context.bool_list_function_id(function),
            );
            functions
                .bool_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::Nil => {
            let id = execution::function::NilListFunctionId::new(index, context.nil_list_type());
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftNilList::new,
                |function, context| context.nil_list_function_id(function),
            );
            functions
                .nil_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::Tuple(shape) => {
            let type_id = context.specialized_tuple_list_type(shape);
            let id = execution::function::TupleListFunctionId::new(index, type_id);
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftTupleList::new,
                move |function, context| context.tuple_list_function_id(function, type_id),
            );
            functions
                .tuple_list_functions
                .push((id, lowered_function(key, graph)));
        }
        S::List(item) => match item.storage_representation() {
            StorageRepresentation::Parameter(parameter) => {
                let type_id = context.parameter_list_list_type(parameter);
                let id = execution::function::ParameterListListFunctionId::new(index, type_id);
                let graph = generic_item_list_graph(
                    template,
                    body,
                    context,
                    graph::DraftParameterListList::new,
                    move |function, context| {
                        context.parameter_list_list_function_id(function, type_id)
                    },
                );
                functions
                    .parameter_list_list_functions
                    .push((id, lowered_function(key, graph)));
            }
            StorageRepresentation::Stored(stored) => {
                let type_id = context.specialized_stored_list_list_type(&stored);
                let id = execution::function::ListListFunctionId::new(index, type_id);
                let graph = generic_item_list_graph(
                    template,
                    body,
                    context,
                    graph::DraftListList::new,
                    move |function, context| context.list_list_function_id(function, type_id),
                );
                functions
                    .list_list_functions
                    .push((id, lowered_function(key, graph)));
            }
        },
        S::Function(shape) => {
            let type_id = context.specialized_function_list_type(shape);
            let id = execution::function::FunctionListFunctionId::new(index, type_id);
            let graph = generic_item_list_graph(
                template,
                body,
                context,
                graph::DraftFunctionList::new,
                move |function, context| context.function_list_function_id(function, type_id),
            );
            functions
                .function_list_functions
                .push((id, lowered_function(key, graph)));
        }
    }
}

fn generic_item_list_graph<DraftList, FrozenList, TailCall>(
    template: &module::FunctionTemplate,
    body: &module::GenericListReturn,
    context: &mut LoweringContext,
    make: impl Copy + Fn(graph::DraftList) -> DraftList,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<
    graph::LoweredFunctionGraph<
        execution::function::FunctionBody<FrozenList, crate::plan::FunctionCallTarget<TailCall>>,
    >,
>
where
    DraftList: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = FrozenList>,
    TailCall: Clone,
{
    graph::lower_function_graph(
        template,
        body,
        context,
        move |expression, cursor, graph, context| {
            graph::generic_list_expr(expression, cursor, graph, context).map(|flow| flow.map(make))
        },
        move |target, context| lower_call_target(target, context, lower_function),
    )
}

fn lower_generic_value_list(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericReturn,
    item: &SpecializedValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item {
        SpecializedValueShape::Parameter(parameter) => {
            let type_id = context.parameter_list_type(*parameter);
            let id = execution::function::ParameterListFunctionId::new(index, type_id);
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftParameterList::new,
                move |function, context| context.parameter_list_function_id(function, *parameter),
            );
            functions
                .parameter_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Int => {
            let id = execution::function::IntListFunctionId::new(index, context.int_list_type());
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftIntList::new,
                |function, context| context.int_list_function_id(function),
            );
            functions
                .int_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::String => {
            let id =
                execution::function::StringListFunctionId::new(index, context.string_list_type());
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftStringList::new,
                |function, context| context.string_list_function_id(function),
            );
            functions
                .string_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::BitArray => {
            let id = execution::function::BitArrayListFunctionId::new(
                index,
                context.bit_array_list_type(),
            );
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftBitArrayList::new,
                |function, context| context.bit_array_list_function_id(function),
            );
            functions
                .bit_array_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::UtfCodepoint => {
            let id = execution::function::UtfCodepointListFunctionId::new(
                index,
                context.utf_codepoint_list_type(),
            );
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftUtfCodepointList::new,
                |function, context| context.utf_codepoint_list_function_id(function),
            );
            functions
                .utf_codepoint_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Custom(item) => {
            let type_id = context.specialized_custom_list_type(item);
            let id = execution::function::CustomListFunctionId::new(index, type_id);
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftCustomList::new,
                move |function, context| context.custom_list_function_id(function, type_id),
            );
            functions
                .custom_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Float => {
            let id =
                execution::function::FloatListFunctionId::new(index, context.float_list_type());
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftFloatList::new,
                |function, context| context.float_list_function_id(function),
            );
            functions
                .float_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Bool => {
            let id = execution::function::BoolListFunctionId::new(index, context.bool_list_type());
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftBoolList::new,
                |function, context| context.bool_list_function_id(function),
            );
            functions
                .bool_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Nil => {
            let id = execution::function::NilListFunctionId::new(index, context.nil_list_type());
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftNilList::new,
                |function, context| context.nil_list_function_id(function),
            );
            functions
                .nil_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::Tuple(item) => {
            let type_id = context.specialized_tuple_list_type(item);
            let id = execution::function::TupleListFunctionId::new(index, type_id);
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftTupleList::new,
                move |function, context| context.tuple_list_function_id(function, type_id),
            );
            functions
                .tuple_list_functions
                .push((id, lowered_function(key, graph)));
        }
        SpecializedValueShape::List(item) => match item.storage_representation() {
            StorageRepresentation::Parameter(parameter) => {
                let type_id = context.parameter_list_list_type(parameter);
                let id = execution::function::ParameterListListFunctionId::new(index, type_id);
                let graph = generic_value_list_graph(
                    template,
                    body,
                    context,
                    graph::DraftParameterListList::new,
                    move |function, context| {
                        context.parameter_list_list_function_id(function, type_id)
                    },
                );
                functions
                    .parameter_list_list_functions
                    .push((id, lowered_function(key, graph)));
            }
            StorageRepresentation::Stored(stored) => {
                let type_id = context.specialized_stored_list_list_type(&stored);
                let id = execution::function::ListListFunctionId::new(index, type_id);
                let graph = generic_value_list_graph(
                    template,
                    body,
                    context,
                    graph::DraftListList::new,
                    move |function, context| context.list_list_function_id(function, type_id),
                );
                functions
                    .list_list_functions
                    .push((id, lowered_function(key, graph)));
            }
        },
        SpecializedValueShape::Function(item) => {
            let type_id = context.specialized_function_list_type(item);
            let id = execution::function::FunctionListFunctionId::new(index, type_id);
            let graph = generic_value_list_graph(
                template,
                body,
                context,
                graph::DraftFunctionList::new,
                move |function, context| context.function_list_function_id(function, type_id),
            );
            functions
                .function_list_functions
                .push((id, lowered_function(key, graph)));
        }
    }
}

fn generic_value_list_graph<DraftList, FrozenList, TailCall>(
    template: &module::FunctionTemplate,
    body: &module::GenericReturn,
    context: &mut LoweringContext,
    make: impl Copy + Fn(graph::DraftList) -> DraftList,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<
    graph::LoweredFunctionGraph<
        execution::function::FunctionBody<FrozenList, crate::plan::FunctionCallTarget<TailCall>>,
    >,
>
where
    DraftList: graph::DraftGraphValue + graph::FreezeGraphValue<Frozen = FrozenList>,
    TailCall: Clone,
{
    graph::lower_function_graph(
        template,
        body,
        context,
        move |expression, cursor, graph, context| {
            graph::generic_expr(expression, cursor, graph, context)
                .map(|flow| flow.map(|value| make(graph::DraftList::from_owned(value))))
        },
        move |target, context| lower_call_target(target, context, lower_function),
    )
}

fn lower_int_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::IntFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_int_function_expr(expression, &function, cursor, graph, context)
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::int_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.int_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .int_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_float_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::FloatFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_float_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::float_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.float_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .float_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_string_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::StringFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_string_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::string_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.string_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .string_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_bit_array_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::BitArrayFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_bit_array_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bit_array_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.bit_array_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .bit_array_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_utf_codepoint_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::UtfCodepointFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_utf_codepoint_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::utf_codepoint_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.utf_codepoint_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .utf_codepoint_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_bool_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::BoolFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_bool_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::bool_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.bool_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .bool_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_nil_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::NilFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_nil_function_expr(expression, &function, cursor, graph, context)
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::nil_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.nil_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .nil_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn typed_function_return<Body>(
    shape: &SpecializedFunctionShape,
    body: Representability<graph::LoweredFunctionGraph<Body>>,
    context: &mut LoweringContext,
) -> Representability<graph::LoweredFunctionGraph<execution::function::TypedFunctionBody<Body>>> {
    let shape = context.lower_concrete_function_shape(shape);
    body.map(|graph| graph.map(|body| execution::function::TypedFunctionBody::new(shape, body)))
}

fn lower_tuple_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::TupleFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(shape);
    match context.function_representation(&function) {
        FunctionRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_tuple_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Never(_) => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::tuple_never_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.never_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .never_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Executable(_) => {
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::tuple_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.tuple_function_function_id(function)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .tuple_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_custom_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::CustomFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(&crate::plan::FunctionShape::new(
        body.type_().argument_shapes().to_vec(),
        crate::plan::ValueShape::Custom(body.type_().return_().clone()),
    ));
    match context.function_representation(&function) {
        FunctionRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::symbolic_custom_function_expr_kind(
                        kind, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Never(_) => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::custom_never_function_expr_kind(kind, &function, cursor, graph, context)
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.never_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .never_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Executable(_) => {
            let return_shape = context.concrete_custom_value_shape(body.type_().return_());
            let type_ = context.custom_function_type(body.type_().clone());
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::custom_function_expr_kind(
                        kind,
                        &return_shape,
                        &function,
                        cursor,
                        graph,
                        context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context
                            .custom_function_function_id(function, type_.clone())
                            .map(|function| function.index())
                    })
                },
            );
            let shape = context.function_shape(shape.clone());
            let graph = graph.map(|graph| {
                graph.map(|body| {
                    execution::function::CustomFunctionFunctionBody::from_parts(shape, type_, body)
                })
            });
            functions
                .custom_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

struct ListFunctionDefinition<'a> {
    shape: &'a crate::plan::FunctionShape,
    item_type: &'a crate::plan::ValueType,
    body: &'a module::ListFunctionReturn,
}

fn lower_list_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    definition: ListFunctionDefinition<'_>,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let ListFunctionDefinition {
        shape,
        item_type,
        body,
    } = definition;
    let function = context.concrete_function_shape(shape);
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    graph::symbolic_list_function_expr(
                        expression, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let item = context
                .concrete_value_shape(&crate::plan::ValueShape::from_value_type(item_type.clone()));
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                graph::list_function_expr,
                |tail, context| {
                    lower_call_target(tail, context, |target, context| {
                        context.list_function_function_id(target, &function, &item)
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            push_list_function_function(functions, index, &item, lowered_function(key, graph));
        }
    }
}

fn lower_function_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    shape: &crate::plan::FunctionShape,
    body: &module::FunctionFunctionReturn,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    let function = context.concrete_function_shape(&crate::plan::FunctionShape::new(
        body.type_().argument_shapes().to_vec(),
        crate::plan::ValueShape::Function(Box::new(body.type_().return_shape().clone())),
    ));
    match context.function_arguments_representation(&function) {
        FunctionArgumentsRepresentation::Symbolic => {
            let generic_type = context.generic_function_type(&function);
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::symbolic_function_function_expr_kind(
                        kind, &function, cursor, graph, context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context.generic_function_function_id(function, generic_type.clone())
                    })
                },
            );
            let graph = typed_function_return(&function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionArgumentsRepresentation::Inhabited => {
            let return_shape = context.concrete_function_shape(body.type_().return_shape());
            let type_ =
                context.specialized_function_function_type(function.arguments(), &return_shape);
            let graph = graph::lower_function_graph(
                template,
                body.body(),
                context,
                |kind, cursor, graph, context| {
                    graph::function_function_expr_kind(
                        kind,
                        &return_shape,
                        &function,
                        cursor,
                        graph,
                        context,
                    )
                },
                |tail, context| {
                    lower_call_target(tail, context, |function, context| {
                        context
                            .function_function_function_id(function, type_.clone())
                            .map(|function| function.index())
                    })
                },
            );
            let shape = context.function_shape(shape.clone());
            let graph = graph.map(|graph| {
                graph.map(|body| {
                    execution::function::FunctionFunctionFunctionBody::from_parts(
                        shape, type_, body,
                    )
                })
            });
            functions
                .function_function_functions
                .push((index, lowered_function(key, graph)));
        }
    }
}

fn lower_generic_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericFunctionReturn,
    function: &SpecializedFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    lower_polymorphic_function::<_, _, GenericFunctionExpression>(
        template, key, index, body, function, functions, context,
    );
}

fn lower_generic_value_function(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::GenericReturn,
    function: &SpecializedFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    lower_polymorphic_function::<_, _, GenericValueFunctionExpression>(
        template, key, index, body, function, functions, context,
    );
}

trait PolymorphicFunctionExpression<Expression, ModuleFunction> {
    fn lower(
        expression: &Expression,
        cursor: graph::DraftCursor,
        graph: &mut graph::DraftGraph,
        context: &mut LoweringContext,
    ) -> Representability<graph::DraftFlow<graph::DraftFunction>>;

    fn function_target(
        function: &ModuleFunction,
    ) -> &crate::plan::FunctionCallTarget<module::FunctionInstantiation>;
}

struct GenericFunctionExpression;

impl
    PolymorphicFunctionExpression<
        module::GenericFunctionExpr,
        crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    > for GenericFunctionExpression
{
    fn lower(
        expression: &module::GenericFunctionExpr,
        cursor: graph::DraftCursor,
        graph: &mut graph::DraftGraph,
        context: &mut LoweringContext,
    ) -> Representability<graph::DraftFlow<graph::DraftFunction>> {
        graph::generic_function_expr(expression, cursor, graph, context)
    }

    fn function_target(
        function: &crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    ) -> &crate::plan::FunctionCallTarget<module::FunctionInstantiation> {
        function
    }
}

struct GenericValueFunctionExpression;

impl
    PolymorphicFunctionExpression<
        module::GenericExpr,
        crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    > for GenericValueFunctionExpression
{
    fn lower(
        expression: &module::GenericExpr,
        cursor: graph::DraftCursor,
        graph: &mut graph::DraftGraph,
        context: &mut LoweringContext,
    ) -> Representability<graph::DraftFlow<graph::DraftFunction>> {
        graph::generic_expr(expression, cursor, graph, context)
            .map(|flow| flow.map(graph::DraftFunction::from_owned))
    }

    fn function_target(
        target: &crate::plan::FunctionCallTarget<module::FunctionInstantiation>,
    ) -> &crate::plan::FunctionCallTarget<module::FunctionInstantiation> {
        target
    }
}

fn lower_polymorphic_function<Expression, ModuleFunction, Lower>(
    template: &module::FunctionTemplate,
    key: &super::super::specialization::SpecializationKey,
    index: usize,
    body: &module::ReturnBody<Expression, ModuleFunction>,
    function: &SpecializedFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) where
    Lower: PolymorphicFunctionExpression<Expression, ModuleFunction>,
{
    match context.function_representation(function) {
        FunctionRepresentation::Symbolic => {
            let type_ = context.generic_function_type(function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    Lower::lower(expression, cursor, graph, context)
                        .map(|flow| flow.map(graph::DraftGenericFunction::new))
                },
                |tail, context| {
                    lower_call_target(
                        Lower::function_target(tail),
                        context,
                        |function, context| {
                            context.generic_function_function_id(function, type_.clone())
                        },
                    )
                },
            );
            let graph = typed_function_return(function, graph, context);
            functions
                .generic_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Never(_) => {
            let type_ = context.generic_function_type(function);
            let graph = graph::lower_function_graph(
                template,
                body,
                context,
                |expression, cursor, graph, context| {
                    Lower::lower(expression, cursor, graph, context)
                        .map(|flow| flow.map(graph::DraftNeverFunction::new))
                },
                |tail, context| {
                    lower_call_target(
                        Lower::function_target(tail),
                        context,
                        |function, context| {
                            context.never_function_function_id(function, type_.clone())
                        },
                    )
                },
            );
            let graph = typed_function_return(function, graph, context);
            functions
                .never_function_functions
                .push((index, lowered_function(key, graph)));
        }
        FunctionRepresentation::Executable(return_) => match return_ {
            StoredValueShape::Int => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftIntFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.int_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .int_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::Float => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftFloatFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.float_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .float_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::String => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftStringFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.string_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .string_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::BitArray => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftBitArrayFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.bit_array_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .bit_array_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::UtfCodepoint => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftUtfCodepointFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| {
                                context.utf_codepoint_function_function_id(function)
                            },
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .utf_codepoint_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::Custom(return_shape) => {
                let type_ =
                    context.specialized_custom_function_type(function.arguments(), &return_shape);
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftCustomFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| {
                                context
                                    .custom_function_function_id(function, type_.clone())
                                    .map(|function| function.index())
                            },
                        )
                    },
                );
                let shape = context.lower_concrete_function_shape(function);
                let graph = graph.map(|graph| {
                    graph.map(|body| {
                        execution::function::CustomFunctionFunctionBody::from_parts(
                            shape, type_, body,
                        )
                    })
                });
                functions
                    .custom_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::Bool => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftBoolFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.bool_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .bool_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::Nil => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftNilFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.nil_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .nil_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::Tuple(_) => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftTupleFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| context.tuple_function_function_id(function),
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                functions
                    .tuple_function_functions
                    .push((index, lowered_function(key, graph)));
            }
            StoredValueShape::List(item) => {
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftListFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |target, context| {
                                context.list_function_function_id(target, function, &item)
                            },
                        )
                    },
                );
                let graph = typed_function_return(function, graph, context);
                push_list_function_function(functions, index, &item, lowered_function(key, graph));
            }
            StoredValueShape::Function(return_shape) => {
                let type_ =
                    context.specialized_function_function_type(function.arguments(), &return_shape);
                let graph = graph::lower_function_graph(
                    template,
                    body,
                    context,
                    |expression, cursor, graph, context| {
                        Lower::lower(expression, cursor, graph, context)
                            .map(|flow| flow.map(graph::DraftFunctionFunction::new))
                    },
                    |tail, context| {
                        lower_call_target(
                            Lower::function_target(tail),
                            context,
                            |function, context| {
                                context
                                    .function_function_function_id(function, type_.clone())
                                    .map(|function| function.index())
                            },
                        )
                    },
                );
                let shape = context.lower_concrete_function_shape(function);
                let graph = graph.map(|graph| {
                    graph.map(|body| {
                        execution::function::FunctionFunctionFunctionBody::from_parts(
                            shape, type_, body,
                        )
                    })
                });
                functions
                    .function_function_functions
                    .push((index, lowered_function(key, graph)));
            }
        },
    }
}
