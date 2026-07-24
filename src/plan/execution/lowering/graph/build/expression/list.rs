use super::{call_args, custom, function, generic, panic_expr, tuple};
use crate::plan::execution::lowering::graph::draft::{DraftStoredList, DraftTypedList};
use crate::plan::execution::lowering::graph::{
    DraftBitArray, DraftBitArrayList, DraftBool, DraftBoolList, DraftCursor, DraftCustom,
    DraftCustomList, DraftFloat, DraftFloatList, DraftFlow, DraftFunction, DraftFunctionList,
    DraftGraph, DraftGraphValue, DraftInt, DraftIntList, DraftList, DraftListList, DraftNil,
    DraftNilList, DraftString, DraftStringList, DraftTuple, DraftTupleList, DraftUtfCodepoint,
    DraftUtfCodepointList, DraftValueRef,
};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedValueShape, StorageRepresentation, StoredValueShape,
};
use crate::plan::{execution, module};

type Lowered<T> = Representability<DraftFlow<T>>;

trait GraphListItem:
    module::ListItem<Function = module::FunctionInstantiation, IndexSource = module::ListListExpr>
{
    type DraftElement: DraftGraphValue;
    type DraftList: DraftGraphValue + EraseDraftList;
    type ExecutionLocal: Copy;
    type ExecutionFunction: Clone;
    type ExecutionType: Copy;

    fn lower_element(
        element: &Self::ElementExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::LoweringContext,
    ) -> Lowered<Self::DraftElement>;

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut super::super::LoweringContext,
    ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>>;

    fn local_key(local: &<Self as module::ListItem>::Local) -> super::super::local::LocalKey;

    fn lower_function(
        &self,
        function: &module::FunctionInstantiation,
        context: &mut super::super::LoweringContext,
    ) -> Representability<Self::ExecutionFunction>;

    fn list_type(&self, context: &mut super::super::LoweringContext) -> Self::ExecutionType;

    fn instruction(
        type_id: Self::ExecutionType,
        instruction: super::super::instruction::DraftTypedListInstruction<
            Self::DraftElement,
            Self::ExecutionLocal,
            Self::ExecutionFunction,
        >,
    ) -> super::super::instruction::DraftListInstruction;

    fn wrap(value: DraftList) -> Self::DraftList;
    fn from_ref(value: &DraftValueRef) -> Self::DraftList;
}

enum GenericListOperation {
    FunctionCall {
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    },
    TupleIndex {
        source: DraftTuple,
        index: usize,
    },
    CustomField {
        source: DraftCustom,
        index: usize,
    },
    ListIndex {
        source: DraftList,
        index: usize,
    },
}

fn typed_generic_list_operation<Element, Local, Function>(
    operation: GenericListOperation,
) -> super::super::instruction::DraftTypedListInstruction<Element, Local, Function> {
    use super::super::instruction::DraftTypedListInstruction as I;

    match operation {
        GenericListOperation::FunctionCall { function, args } => I::FunctionCall { function, args },
        GenericListOperation::TupleIndex { source, index } => I::TupleIndex {
            tuple: source,
            index,
        },
        GenericListOperation::CustomField { source, index } => I::CustomField { source, index },
        GenericListOperation::ListIndex { source, index } => I::ListIndex {
            list: source,
            index,
        },
    }
}

fn parameter_list_operation(
    operation: GenericListOperation,
) -> super::super::instruction::DraftParameterListInstruction {
    use super::super::instruction::DraftParameterListInstruction as I;

    match operation {
        GenericListOperation::FunctionCall { function, args } => I::FunctionCall { function, args },
        GenericListOperation::TupleIndex { source, index } => I::TupleIndex {
            tuple: source,
            index,
        },
        GenericListOperation::CustomField { source, index } => I::CustomField { source, index },
        GenericListOperation::ListIndex { source, index } => I::ListIndex {
            list: source,
            index,
        },
    }
}

fn generic_list_operation(
    item: &SpecializedValueShape,
    operation: GenericListOperation,
    context: &mut super::super::LoweringContext,
) -> super::super::instruction::DraftListInstruction {
    use super::super::instruction::DraftListInstruction as I;

    match item {
        SpecializedValueShape::Parameter(parameter) => I::Parameter(
            context.parameter_list_type(*parameter),
            parameter_list_operation(operation),
        ),
        SpecializedValueShape::Int => I::Int(
            context.int_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::String => I::String(
            context.string_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::BitArray => I::BitArray(
            context.bit_array_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::UtfCodepoint => I::UtfCodepoint(
            context.utf_codepoint_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::Custom(item) => I::Custom(
            context.specialized_custom_list_type(item),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::Float => I::Float(
            context.float_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::Bool => I::Bool(
            context.bool_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::Nil => I::Nil(
            context.nil_list_type(),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::Tuple(item) => I::Tuple(
            context.specialized_tuple_list_type(item),
            typed_generic_list_operation(operation),
        ),
        SpecializedValueShape::List(item) => match item.storage_representation() {
            StorageRepresentation::Parameter(parameter) => I::ParameterList(
                context.parameter_list_list_type(parameter),
                typed_generic_list_operation(operation),
            ),
            StorageRepresentation::Stored(item) => I::List(
                context.specialized_stored_list_list_type(&item),
                typed_generic_list_operation(operation),
            ),
        },
        SpecializedValueShape::Function(item) => I::Function(
            context.specialized_function_list_type(item),
            typed_generic_list_operation(operation),
        ),
    }
}

pub(in crate::plan::execution::lowering) fn list_expr(
    expression: &module::ListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    match expression {
        module::ListExpr::Generic(value) => generic_list_expr(value, cursor, graph, context),
        module::ListExpr::ParameterList(value) => {
            parameter_list_list_expr(value, cursor, graph, context)
                .map(|flow| flow.map(DraftStoredList::into_list))
        }
        module::ListExpr::Int(value) => int_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::String(value) => string_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::BitArray(value) => bit_array_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::UtfCodepoint(value) => {
            utf_codepoint_list_expr(value, cursor, graph, context)
                .map(|flow| flow.map(|value| value.value().clone()))
        }
        module::ListExpr::Custom(value) => custom_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::Float(value) => float_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::Bool(value) => bool_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::Nil(value) => nil_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::Tuple(value) => tuple_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::List(value) => list_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        module::ListExpr::Function(value) => function_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
    }
}

pub(super) fn generic_direct_call(
    item: &SpecializedValueShape,
    target: &module::FunctionInstantiation,
    args: Vec<DraftValueRef>,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    use super::super::instruction::{
        DraftListInstruction as I, DraftParameterListInstruction as P,
        DraftTypedListInstruction as T,
    };

    context.list_function_id(target, item).map(|function| {
        let kind = match function {
            execution::function::ListFunctionId::Parameter(function) => {
                I::Parameter(function.type_id(), P::Call { function, args })
            }
            execution::function::ListFunctionId::ParameterList(function) => {
                I::ParameterList(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Int(function) => {
                I::Int(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::String(function) => {
                I::String(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::BitArray(function) => {
                I::BitArray(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::UtfCodepoint(function) => {
                I::UtfCodepoint(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Custom(function) => {
                I::Custom(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Float(function) => {
                I::Float(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Bool(function) => {
                I::Bool(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Nil(function) => {
                I::Nil(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Tuple(function) => {
                I::Tuple(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::List(function) => {
                I::List(function.type_id(), T::Call { function, args })
            }
            execution::function::ListFunctionId::Function(function) => {
                I::Function(function.type_id(), T::Call { function, args })
            }
        };
        let value = graph.list_instruction(&mut cursor, item.clone(), kind);
        DraftFlow::value(cursor, value)
    })
}

pub(super) fn generic_function_call(
    item: &SpecializedValueShape,
    function: DraftFunction,
    args: Vec<DraftValueRef>,
    cursor: &mut DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftList {
    graph.list_instruction(
        cursor,
        item.clone(),
        generic_list_operation(
            item,
            GenericListOperation::FunctionCall { function, args },
            context,
        ),
    )
}

pub(super) fn generic_tuple_index(
    item: &SpecializedValueShape,
    source: DraftTuple,
    index: usize,
    cursor: &mut DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftList {
    graph.list_instruction(
        cursor,
        item.clone(),
        generic_list_operation(
            item,
            GenericListOperation::TupleIndex { source, index },
            context,
        ),
    )
}

pub(super) fn generic_custom_field(
    item: &SpecializedValueShape,
    source: DraftCustom,
    index: usize,
    cursor: &mut DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftList {
    graph.list_instruction(
        cursor,
        item.clone(),
        generic_list_operation(
            item,
            GenericListOperation::CustomField { source, index },
            context,
        ),
    )
}

pub(super) fn generic_list_index(
    item: &SpecializedValueShape,
    source: DraftList,
    index: usize,
    cursor: &mut DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftList {
    graph.list_instruction(
        cursor,
        item.clone(),
        generic_list_operation(
            item,
            GenericListOperation::ListIndex { source, index },
            context,
        ),
    )
}

pub(in crate::plan::execution::lowering) fn generic_list_expr(
    expression: &module::GenericListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    let item = context.concrete_parameter(expression.item().parameter());
    match item.storage_representation() {
        StorageRepresentation::Parameter(parameter) => {
            parameter_list_kind(expression.kind(), parameter, cursor, graph, context)
        }
        StorageRepresentation::Stored(item) => {
            stored_generic_list_kind(expression.kind(), &item, cursor, graph, context)
        }
    }
}

fn parameter_list_kind(
    kind: &module::TypedListExprKind<module::GenericListItem>,
    parameter: crate::plan::TypeParameterId,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    use super::super::instruction::{
        DraftListInstruction as I, DraftParameterListInstruction as P,
    };
    use module::TypedListExprKind as E;

    let item = SpecializedValueShape::Parameter(parameter);
    let result_shape = StoredValueShape::List(Box::new(item.clone()));
    let list_type = context.parameter_list_type(parameter);
    match kind {
        E::Value(elements) => match elements.as_slice() {
            [] => {
                let mut cursor = cursor;
                let value =
                    graph.list_instruction(&mut cursor, item, I::Parameter(list_type, P::Empty));
                Representability::Inhabited(DraftFlow::value(cursor, value))
            }
            [first, ..] => {
                generic::never_expr(first, cursor, graph, context).map(|()| DraftFlow::Diverged)
            }
        },
        E::Constant(constant) => context
            .generic_parameter_list_constant(constant, parameter)
            .map(|id| {
                let mut cursor = cursor;
                let value = graph.list_instruction(
                    &mut cursor,
                    item,
                    I::Parameter(
                        list_type,
                        P::Constant(execution::constant::ConstantId::new(id.index())),
                    ),
                );
                DraftFlow::value(cursor, value)
            }),
        E::Spread { elements, tail: _ } => {
            generic::never_expr(elements.first(), cursor, graph, context)
                .map(|()| DraftFlow::Diverged)
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().list(super::super::local::LocalKey::new(
                super::super::local::LocalKind::GenericList,
                local.0,
            ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => call_args(args, cursor, graph, context).and_then(|flow| {
            flow.and_then(|mut cursor, args| {
                context
                    .parameter_list_function_id(function, parameter)
                    .map(|function| {
                        let value = graph.list_instruction(
                            &mut cursor,
                            item,
                            I::Parameter(list_type, P::Call { function, args }),
                        );
                        DraftFlow::value(cursor, value)
                    })
            })
        }),
        E::FunctionCall {
            function: value,
            args,
        } => function::list_function_expr(value, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, function| {
                call_args(args, cursor, graph, context).map(|flow| {
                    flow.map_cursor(|cursor, args| {
                        graph.list_instruction(
                            cursor,
                            item,
                            I::Parameter(
                                list_type,
                                P::FunctionCall {
                                    function: function.value().clone(),
                                    args,
                                },
                            ),
                        )
                    })
                })
            })
        }),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, tuple| {
                graph.list_instruction(
                    cursor,
                    item,
                    I::Parameter(
                        list_type,
                        P::TupleIndex {
                            tuple,
                            index: *index,
                        },
                    ),
                )
            })
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, source| {
                    graph.list_instruction(
                        cursor,
                        item,
                        I::Parameter(
                            list_type,
                            P::CustomField {
                                source,
                                index: access.index(),
                            },
                        ),
                    )
                })
            })
        }
        E::ListIndex(source) => {
            let index = source.index();
            parameter_list_list_expr(source.list(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.list_instruction(
                        &mut cursor,
                        item,
                        I::Parameter(
                            list_type,
                            P::ListIndex {
                                list: list.into_list(),
                                index,
                            },
                        ),
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::DropFirst { list, count: _ } => {
            parameter_list_kind(list, parameter, cursor, graph, context)
        }
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |cursor, graph, context| parameter_list_kind(true_, parameter, cursor, graph, context),
            |cursor, graph, context| parameter_list_kind(false_, parameter, cursor, graph, context),
            DraftList::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_kind(branch, parameter, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_kind(branch, parameter, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_kind(branch, parameter, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| {
                flow.and_then(|cursor, ()| {
                    parameter_list_kind(return_, parameter, cursor, graph, context)
                })
            }),
    }
}

fn generic_elements(
    elements: &[module::GenericExpr],
    item: &StoredValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<DraftValueRef>> {
    lower_element_sequence(
        elements,
        cursor,
        graph,
        context,
        |element, cursor, graph, context| {
            generic::stored_expr(element, item, cursor, graph, context)
        },
    )
}

fn typed_generic_list_value(
    item: &StoredValueShape,
    elements: Vec<DraftValueRef>,
    context: &mut super::super::LoweringContext,
) -> super::super::instruction::DraftListInstruction {
    use super::super::instruction::{DraftListInstruction as I, DraftTypedListInstruction as T};

    match item {
        StoredValueShape::Int => I::Int(
            context.int_list_type(),
            T::Value(elements.into_iter().map(DraftInt::from_owned).collect()),
        ),
        StoredValueShape::String => I::String(
            context.string_list_type(),
            T::Value(elements.into_iter().map(DraftString::from_owned).collect()),
        ),
        StoredValueShape::BitArray => I::BitArray(
            context.bit_array_list_type(),
            T::Value(
                elements
                    .into_iter()
                    .map(DraftBitArray::from_owned)
                    .collect(),
            ),
        ),
        StoredValueShape::UtfCodepoint => I::UtfCodepoint(
            context.utf_codepoint_list_type(),
            T::Value(
                elements
                    .into_iter()
                    .map(DraftUtfCodepoint::from_owned)
                    .collect(),
            ),
        ),
        StoredValueShape::Custom(item) => I::Custom(
            context.specialized_custom_list_type(item),
            T::Value(elements.into_iter().map(DraftCustom::from_owned).collect()),
        ),
        StoredValueShape::Float => I::Float(
            context.float_list_type(),
            T::Value(elements.into_iter().map(DraftFloat::from_owned).collect()),
        ),
        StoredValueShape::Bool => I::Bool(
            context.bool_list_type(),
            T::Value(elements.into_iter().map(DraftBool::from_owned).collect()),
        ),
        StoredValueShape::Nil => I::Nil(
            context.nil_list_type(),
            T::Value(elements.into_iter().map(DraftNil::from_owned).collect()),
        ),
        StoredValueShape::Tuple(item) => I::Tuple(
            context.specialized_tuple_list_type(item),
            T::Value(elements.into_iter().map(DraftTuple::from_owned).collect()),
        ),
        StoredValueShape::List(inner) => match inner.storage_representation() {
            StorageRepresentation::Parameter(parameter) => I::ParameterList(
                context.parameter_list_list_type(parameter),
                T::Value(elements.into_iter().map(DraftList::from_owned).collect()),
            ),
            StorageRepresentation::Stored(inner) => I::List(
                context.specialized_stored_list_list_type(&inner),
                T::Value(
                    elements
                        .into_iter()
                        .map(|value| draft_stored_list(&inner, value))
                        .collect(),
                ),
            ),
        },
        StoredValueShape::Function(item) => I::Function(
            context.specialized_function_list_type(item),
            T::Value(
                elements
                    .into_iter()
                    .map(DraftFunction::from_owned)
                    .collect(),
            ),
        ),
    }
}

fn typed_generic_list_spread(
    item: &StoredValueShape,
    elements: Vec<DraftValueRef>,
    tail: DraftList,
    context: &mut super::super::LoweringContext,
) -> super::super::instruction::DraftListInstruction {
    use super::super::instruction::{DraftListInstruction as I, DraftTypedListInstruction as T};

    match item {
        StoredValueShape::Int => I::Int(
            context.int_list_type(),
            T::Spread {
                elements: elements.into_iter().map(DraftInt::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::String => I::String(
            context.string_list_type(),
            T::Spread {
                elements: elements.into_iter().map(DraftString::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::BitArray => I::BitArray(
            context.bit_array_list_type(),
            T::Spread {
                elements: elements
                    .into_iter()
                    .map(DraftBitArray::from_owned)
                    .collect(),
                tail,
            },
        ),
        StoredValueShape::UtfCodepoint => I::UtfCodepoint(
            context.utf_codepoint_list_type(),
            T::Spread {
                elements: elements
                    .into_iter()
                    .map(DraftUtfCodepoint::from_owned)
                    .collect(),
                tail,
            },
        ),
        StoredValueShape::Custom(item) => I::Custom(
            context.specialized_custom_list_type(item),
            T::Spread {
                elements: elements.into_iter().map(DraftCustom::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::Float => I::Float(
            context.float_list_type(),
            T::Spread {
                elements: elements.into_iter().map(DraftFloat::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::Bool => I::Bool(
            context.bool_list_type(),
            T::Spread {
                elements: elements.into_iter().map(DraftBool::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::Nil => I::Nil(
            context.nil_list_type(),
            T::Spread {
                elements: elements.into_iter().map(DraftNil::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::Tuple(item) => I::Tuple(
            context.specialized_tuple_list_type(item),
            T::Spread {
                elements: elements.into_iter().map(DraftTuple::from_owned).collect(),
                tail,
            },
        ),
        StoredValueShape::List(inner) => match inner.storage_representation() {
            StorageRepresentation::Parameter(parameter) => I::ParameterList(
                context.parameter_list_list_type(parameter),
                T::Spread {
                    elements: elements.into_iter().map(DraftList::from_owned).collect(),
                    tail,
                },
            ),
            StorageRepresentation::Stored(inner) => I::List(
                context.specialized_stored_list_list_type(&inner),
                T::Spread {
                    elements: elements
                        .into_iter()
                        .map(|value| draft_stored_list(&inner, value))
                        .collect(),
                    tail,
                },
            ),
        },
        StoredValueShape::Function(item) => I::Function(
            context.specialized_function_list_type(item),
            T::Spread {
                elements: elements
                    .into_iter()
                    .map(DraftFunction::from_owned)
                    .collect(),
                tail,
            },
        ),
    }
}

fn draft_stored_list(item: &StoredValueShape, value: DraftValueRef) -> DraftStoredList {
    match item {
        StoredValueShape::Int => DraftStoredList::Int(DraftList::from_owned(value)),
        StoredValueShape::String => DraftStoredList::String(DraftList::from_owned(value)),
        StoredValueShape::BitArray => DraftStoredList::BitArray(DraftList::from_owned(value)),
        StoredValueShape::UtfCodepoint => {
            DraftStoredList::UtfCodepoint(DraftList::from_owned(value))
        }
        StoredValueShape::Custom(_) => DraftStoredList::Custom(DraftList::from_owned(value)),
        StoredValueShape::Float => DraftStoredList::Float(DraftList::from_owned(value)),
        StoredValueShape::Bool => DraftStoredList::Bool(DraftList::from_owned(value)),
        StoredValueShape::Nil => DraftStoredList::Nil(DraftList::from_owned(value)),
        StoredValueShape::Tuple(_) => DraftStoredList::Tuple(DraftList::from_owned(value)),
        StoredValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => {
                DraftStoredList::ParameterList(DraftList::from_owned(value))
            }
            _ => DraftStoredList::List(DraftList::from_owned(value)),
        },
        StoredValueShape::Function(_) => DraftStoredList::Function(DraftList::from_owned(value)),
    }
}

fn typed_generic_list_drop_first(
    item: &StoredValueShape,
    list: DraftList,
    count: usize,
    context: &mut super::super::LoweringContext,
) -> super::super::instruction::DraftListInstruction {
    use super::super::instruction::{DraftListInstruction as I, DraftTypedListInstruction as T};

    match item {
        StoredValueShape::Int => I::Int(context.int_list_type(), T::DropFirst { list, count }),
        StoredValueShape::String => {
            I::String(context.string_list_type(), T::DropFirst { list, count })
        }
        StoredValueShape::BitArray => {
            I::BitArray(context.bit_array_list_type(), T::DropFirst { list, count })
        }
        StoredValueShape::UtfCodepoint => I::UtfCodepoint(
            context.utf_codepoint_list_type(),
            T::DropFirst { list, count },
        ),
        StoredValueShape::Custom(item) => I::Custom(
            context.specialized_custom_list_type(item),
            T::DropFirst { list, count },
        ),
        StoredValueShape::Float => {
            I::Float(context.float_list_type(), T::DropFirst { list, count })
        }
        StoredValueShape::Bool => I::Bool(context.bool_list_type(), T::DropFirst { list, count }),
        StoredValueShape::Nil => I::Nil(context.nil_list_type(), T::DropFirst { list, count }),
        StoredValueShape::Tuple(item) => I::Tuple(
            context.specialized_tuple_list_type(item),
            T::DropFirst { list, count },
        ),
        StoredValueShape::List(inner) => match inner.storage_representation() {
            StorageRepresentation::Parameter(parameter) => I::ParameterList(
                context.parameter_list_list_type(parameter),
                T::DropFirst { list, count },
            ),
            StorageRepresentation::Stored(inner) => I::List(
                context.specialized_stored_list_list_type(&inner),
                T::DropFirst { list, count },
            ),
        },
        StoredValueShape::Function(item) => I::Function(
            context.specialized_function_list_type(item),
            T::DropFirst { list, count },
        ),
    }
}

fn stored_generic_list_constant(
    item: &StoredValueShape,
    constant: &module::ConstantGenericListInstantiation,
    context: &mut super::super::LoweringContext,
) -> Representability<super::super::instruction::DraftListInstruction> {
    use super::super::instruction::{DraftListInstruction as I, DraftTypedListInstruction as T};

    match item {
        StoredValueShape::Int => {
            let type_id = context.int_list_type();
            context.generic_int_list_constant(constant).map(|id| {
                I::Int(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::String => {
            let type_id = context.string_list_type();
            context.generic_string_list_constant(constant).map(|id| {
                I::String(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::BitArray => {
            let type_id = context.bit_array_list_type();
            context.generic_bit_array_list_constant(constant).map(|id| {
                I::BitArray(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::UtfCodepoint => {
            let type_id = context.utf_codepoint_list_type();
            context
                .generic_utf_codepoint_list_constant(constant)
                .map(|id| {
                    I::UtfCodepoint(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
        StoredValueShape::Custom(shape) => {
            let type_id = context.specialized_custom_list_type(shape);
            context
                .generic_custom_list_constant(constant, shape)
                .map(|id| {
                    I::Custom(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
        StoredValueShape::Float => {
            let type_id = context.float_list_type();
            context.generic_float_list_constant(constant).map(|id| {
                I::Float(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::Bool => {
            let type_id = context.bool_list_type();
            context.generic_bool_list_constant(constant).map(|id| {
                I::Bool(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::Nil => {
            let type_id = context.nil_list_type();
            context.generic_nil_list_constant(constant).map(|id| {
                I::Nil(
                    type_id,
                    T::Constant(execution::constant::ConstantId::new(id.index())),
                )
            })
        }
        StoredValueShape::Tuple(elements) => {
            let type_id = context.specialized_tuple_list_type(elements);
            context
                .generic_tuple_list_constant(constant, elements)
                .map(|id| {
                    I::Tuple(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
        StoredValueShape::List(inner) => match inner.storage_representation() {
            StorageRepresentation::Parameter(parameter) => {
                let type_id = context.parameter_list_list_type(parameter);
                context
                    .generic_parameter_list_list_constant(constant, parameter)
                    .map(|id| {
                        I::ParameterList(
                            type_id,
                            T::Constant(execution::constant::ConstantId::new(id.index())),
                        )
                    })
            }
            StorageRepresentation::Stored(inner) => {
                let type_id = context.specialized_stored_list_list_type(&inner);
                context
                    .generic_list_list_constant(constant, &inner)
                    .map(|id| {
                        I::List(
                            type_id,
                            T::Constant(execution::constant::ConstantId::new(id.index())),
                        )
                    })
            }
        },
        StoredValueShape::Function(shape) => {
            let type_id = context.specialized_function_list_type(shape);
            context
                .generic_function_list_constant(constant, shape)
                .map(|id| {
                    I::Function(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
    }
}

fn stored_generic_list_kind(
    kind: &module::TypedListExprKind<module::GenericListItem>,
    item: &StoredValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    use module::TypedListExprKind as E;

    let item_shape = item.to_specialized();
    let result_shape = StoredValueShape::List(Box::new(item_shape.clone()));
    match kind {
        E::Value(elements) => {
            generic_elements(elements, item, cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, elements| {
                    graph.list_instruction(
                        cursor,
                        item_shape.clone(),
                        typed_generic_list_value(item, elements, context),
                    )
                })
            })
        }
        E::Constant(constant) => {
            stored_generic_list_constant(item, constant, context).map(|kind| {
                let mut cursor = cursor;
                let value = graph.list_instruction(&mut cursor, item_shape.clone(), kind);
                DraftFlow::value(cursor, value)
            })
        }
        E::Spread { elements, tail } => generic_elements(elements, item, cursor, graph, context)
            .and_then(|flow| {
                flow.and_then(|cursor, elements| {
                    stored_generic_list_kind(tail, item, cursor, graph, context).map(|flow| {
                        flow.map_cursor(|cursor, tail| {
                            graph.list_instruction(
                                cursor,
                                item_shape.clone(),
                                typed_generic_list_spread(item, elements, tail, context),
                            )
                        })
                    })
                })
            }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().list(super::super::local::LocalKey::new(
                super::super::local::LocalKind::GenericList,
                local.0,
            ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => call_args(args, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, args| {
                generic_direct_call(&item_shape, function, args, cursor, graph, context)
            })
        }),
        E::FunctionCall {
            function: value,
            args,
        } => function::list_function_expr(value, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, function| {
                call_args(args, cursor, graph, context).map(|flow| {
                    flow.map_cursor(|cursor, args| {
                        generic_function_call(
                            &item_shape,
                            function.value().clone(),
                            args,
                            cursor,
                            graph,
                            context,
                        )
                    })
                })
            })
        }),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, source| {
                generic_tuple_index(&item_shape, source, *index, cursor, graph, context)
            })
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, source| {
                    generic_custom_field(
                        &item_shape,
                        source,
                        access.index(),
                        cursor,
                        graph,
                        context,
                    )
                })
            })
        }
        E::ListIndex(source) => {
            let index = source.index();
            parameter_list_list_expr(source.list(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, list| {
                    generic_list_index(&item_shape, list.into_list(), index, cursor, graph, context)
                })
            })
        }
        E::DropFirst { list, count } => {
            stored_generic_list_kind(list, item, cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, list| {
                    graph.list_instruction(
                        cursor,
                        item_shape.clone(),
                        typed_generic_list_drop_first(item, list, *count, context),
                    )
                })
            })
        }
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |cursor, graph, context| stored_generic_list_kind(true_, item, cursor, graph, context),
            |cursor, graph, context| stored_generic_list_kind(false_, item, cursor, graph, context),
            DraftList::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                stored_generic_list_kind(branch, item, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                stored_generic_list_kind(branch, item, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                stored_generic_list_kind(branch, item, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| {
                flow.and_then(|cursor, ()| {
                    stored_generic_list_kind(return_, item, cursor, graph, context)
                })
            }),
    }
}

pub(in crate::plan::execution::lowering) fn parameter_list_list_expr(
    expression: &module::ParameterListListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftStoredList> {
    let item = context.concrete_parameter(expression.item().parameter());
    match item.storage_representation() {
        StorageRepresentation::Parameter(parameter) => parameter_list_list_kind(
            expression.kind(),
            &SpecializedValueShape::Parameter(parameter),
            cursor,
            graph,
            context,
        )
        .map(|flow| flow.map(DraftStoredList::ParameterList)),
        StorageRepresentation::Stored(item) => parameter_list_list_kind(
            expression.kind(),
            &item.to_specialized(),
            cursor,
            graph,
            context,
        )
        .map(|flow| flow.map(DraftStoredList::List)),
    }
}

fn parameter_list_list_elements(
    elements: &[module::GenericListExpr],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<DraftValueRef>> {
    lower_element_sequence(
        elements,
        cursor,
        graph,
        context,
        |element, cursor, graph, context| {
            generic_list_expr(element, cursor, graph, context)
                .map(|flow| flow.map(|value| value.erase()))
        },
    )
}

fn parameter_list_list_constant(
    inner: &SpecializedValueShape,
    constant: &module::ConstantParameterListListInstantiation,
    context: &mut super::super::LoweringContext,
) -> Representability<super::super::instruction::DraftListInstruction> {
    use super::super::instruction::{DraftListInstruction as I, DraftTypedListInstruction as T};

    match inner.storage_representation() {
        StorageRepresentation::Parameter(parameter) => {
            let type_id = context.parameter_list_list_type(parameter);
            context
                .parameter_list_list_constant(constant, parameter)
                .map(|id| {
                    I::ParameterList(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
        StorageRepresentation::Stored(inner) => {
            let type_id = context.specialized_stored_list_list_type(&inner);
            context
                .parameter_list_list_as_stored_constant(constant, &inner)
                .map(|id| {
                    I::List(
                        type_id,
                        T::Constant(execution::constant::ConstantId::new(id.index())),
                    )
                })
        }
    }
}

fn parameter_list_list_kind(
    kind: &module::TypedListExprKind<module::ParameterListListItem>,
    inner: &SpecializedValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftList> {
    use module::TypedListExprKind as E;

    let item = StoredValueShape::List(Box::new(inner.clone()));
    let item_shape = item.to_specialized();
    let result_shape = StoredValueShape::List(Box::new(item_shape.clone()));
    match kind {
        E::Value(elements) => {
            parameter_list_list_elements(elements, cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, elements| {
                    graph.list_instruction(
                        cursor,
                        item_shape.clone(),
                        typed_generic_list_value(&item, elements, context),
                    )
                })
            })
        }
        E::Constant(constant) => {
            parameter_list_list_constant(inner, constant, context).map(|kind| {
                let mut cursor = cursor;
                let value = graph.list_instruction(&mut cursor, item_shape.clone(), kind);
                DraftFlow::value(cursor, value)
            })
        }
        E::Spread { elements, tail } => {
            parameter_list_list_elements(elements, cursor, graph, context).and_then(|flow| {
                flow.and_then(|cursor, elements| {
                    parameter_list_list_kind(tail, inner, cursor, graph, context).map(|flow| {
                        flow.map_cursor(|cursor, tail| {
                            graph.list_instruction(
                                cursor,
                                item_shape.clone(),
                                typed_generic_list_spread(&item, elements, tail, context),
                            )
                        })
                    })
                })
            })
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().list(super::super::local::LocalKey::new(
                super::super::local::LocalKind::ListList,
                local.0,
            ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => call_args(args, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, args| {
                generic_direct_call(&item_shape, function, args, cursor, graph, context)
            })
        }),
        E::FunctionCall {
            function: value,
            args,
        } => function::list_function_expr(value, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, function| {
                call_args(args, cursor, graph, context).map(|flow| {
                    flow.map_cursor(|cursor, args| {
                        generic_function_call(
                            &item_shape,
                            function.value().clone(),
                            args,
                            cursor,
                            graph,
                            context,
                        )
                    })
                })
            })
        }),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, source| {
                generic_tuple_index(&item_shape, source, *index, cursor, graph, context)
            })
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, source| {
                    generic_custom_field(
                        &item_shape,
                        source,
                        access.index(),
                        cursor,
                        graph,
                        context,
                    )
                })
            })
        }
        E::ListIndex(source) => {
            let index = source.index();
            list_list_expr(source.list(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, list| {
                    generic_list_index(
                        &item_shape,
                        list.value().clone(),
                        index,
                        cursor,
                        graph,
                        context,
                    )
                })
            })
        }
        E::DropFirst { list, count } => {
            parameter_list_list_kind(list, inner, cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, list| {
                    graph.list_instruction(
                        cursor,
                        item_shape.clone(),
                        typed_generic_list_drop_first(&item, list, *count, context),
                    )
                })
            })
        }
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |cursor, graph, context| parameter_list_list_kind(true_, inner, cursor, graph, context),
            |cursor, graph, context| {
                parameter_list_list_kind(false_, inner, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_list_kind(branch, inner, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_list_kind(branch, inner, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                parameter_list_list_kind(branch, inner, cursor, graph, context)
            },
            DraftList::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| {
                flow.and_then(|cursor, ()| {
                    parameter_list_list_kind(return_, inner, cursor, graph, context)
                })
            }),
    }
}

fn stored_list_expr(
    expression: &module::StoredListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftStoredList> {
    match expression {
        module::StoredListExpr::ParameterList(value) => {
            parameter_list_list_expr(value, cursor, graph, context)
        }
        module::StoredListExpr::Int(value) => int_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Int(value.value().clone()))),
        module::StoredListExpr::String(value) => string_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::String(value.value().clone()))),
        module::StoredListExpr::BitArray(value) => {
            bit_array_list_expr(value, cursor, graph, context)
                .map(|flow| flow.map(|value| DraftStoredList::BitArray(value.value().clone())))
        }
        module::StoredListExpr::UtfCodepoint(value) => {
            utf_codepoint_list_expr(value, cursor, graph, context)
                .map(|flow| flow.map(|value| DraftStoredList::UtfCodepoint(value.value().clone())))
        }
        module::StoredListExpr::Custom(value) => custom_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Custom(value.value().clone()))),
        module::StoredListExpr::Float(value) => float_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Float(value.value().clone()))),
        module::StoredListExpr::Bool(value) => bool_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Bool(value.value().clone()))),
        module::StoredListExpr::Nil(value) => nil_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Nil(value.value().clone()))),
        module::StoredListExpr::Tuple(value) => tuple_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::Tuple(value.value().clone()))),
        module::StoredListExpr::List(value) => list_list_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| DraftStoredList::List(value.value().clone()))),
        module::StoredListExpr::Function(value) => {
            function_list_expr(value, cursor, graph, context)
                .map(|flow| flow.map(|value| DraftStoredList::Function(value.value().clone())))
        }
    }
}

fn typed_list_expr<Item>(
    expression: &module::TypedListExpr<Item>,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Item::DraftList>
where
    Item: GraphListItem,
{
    let item_shape = context.concrete_value_shape(expression.item_shape());
    typed_list_kind(
        expression.item(),
        expression.kind(),
        &item_shape,
        cursor,
        graph,
        context,
    )
}

fn typed_list_kind<Item>(
    item: &Item,
    kind: &module::TypedListExprKind<Item>,
    item_shape: &SpecializedValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Item::DraftList>
where
    Item: GraphListItem,
{
    use super::super::instruction::DraftTypedListInstruction as I;
    use module::TypedListExprKind as E;

    let result_shape = StoredValueShape::List(Box::new(item_shape.clone()));
    let list_type = item.list_type(context);
    match kind {
        E::Value(elements) => {
            lower_elements::<Item>(elements, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: elements,
                } => {
                    let value = graph.list_instruction(
                        &mut cursor,
                        item_shape.clone(),
                        Item::instruction(list_type, I::Value(elements)),
                    );
                    DraftFlow::value(cursor, Item::wrap(value))
                }
            })
        }
        E::Constant(value) => Item::lower_constant(value, context).map(|id| {
            let mut cursor = cursor;
            let value = graph.list_instruction(
                &mut cursor,
                item_shape.clone(),
                Item::instruction(list_type, I::Constant(id)),
            );
            DraftFlow::value(cursor, Item::wrap(value))
        }),
        E::Spread { elements, tail } => lower_elements::<Item>(elements, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    cursor,
                    value: elements,
                } => typed_list_kind(item, tail, item_shape, cursor, graph, context).map(|flow| {
                    match flow {
                        DraftFlow::Diverged => DraftFlow::Diverged,
                        DraftFlow::Value {
                            mut cursor,
                            value: tail,
                        } => {
                            let value = graph.list_instruction(
                                &mut cursor,
                                item_shape.clone(),
                                Item::instruction(
                                    list_type,
                                    I::Spread {
                                        elements,
                                        tail: tail.erase_list(),
                                    },
                                ),
                            );
                            DraftFlow::value(cursor, Item::wrap(value))
                        }
                    }
                }),
            }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().list(Item::local_key(local));
            Representability::Inhabited(DraftFlow::value(cursor, Item::wrap(value)))
        }
        E::Call {
            function: target,
            args,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => Item::lower_function(item, target, context).map(|function| {
                let value = graph.list_instruction(
                    &mut cursor,
                    item_shape.clone(),
                    Item::instruction(list_type, I::Call { function, args }),
                );
                DraftFlow::value(cursor, Item::wrap(value))
            }),
        }),
        E::FunctionCall {
            function: value,
            args,
        } => function::list_function_expr(value, cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, function| {
                call_args(args, cursor, graph, context).and_then(|flow| {
                    flow.and_then(|mut cursor, args| {
                        let value = graph.list_instruction(
                            &mut cursor,
                            item_shape.clone(),
                            Item::instruction(
                                list_type,
                                I::FunctionCall {
                                    function: function.value().clone(),
                                    args,
                                },
                            ),
                        );
                        Representability::Inhabited(DraftFlow::value(cursor, Item::wrap(value)))
                    })
                })
            })
        }),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.list_instruction(
                    &mut cursor,
                    item_shape.clone(),
                    Item::instruction(
                        list_type,
                        I::TupleIndex {
                            tuple,
                            index: *index,
                        },
                    ),
                );
                DraftFlow::value(cursor, Item::wrap(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.list_instruction(
                        &mut cursor,
                        item_shape.clone(),
                        Item::instruction(
                            list_type,
                            I::CustomField {
                                source,
                                index: access.index(),
                            },
                        ),
                    );
                    DraftFlow::value(cursor, Item::wrap(value))
                }
            })
        }
        E::ListIndex(source) => {
            list_list_expr(source.list(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.list_instruction(
                        &mut cursor,
                        item_shape.clone(),
                        Item::instruction(
                            list_type,
                            I::ListIndex {
                                list: list.value().clone(),
                                index: source.index(),
                            },
                        ),
                    );
                    DraftFlow::value(cursor, Item::wrap(value))
                }
            })
        }
        E::DropFirst { list, count } => {
            typed_list_kind(item, list, item_shape, cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, list| {
                    let value = graph.list_instruction(
                        cursor,
                        item_shape.clone(),
                        Item::instruction(
                            list_type,
                            I::DropFirst {
                                list: list.erase_list(),
                                count: *count,
                            },
                        ),
                    );
                    Item::wrap(value)
                })
            })
        }
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |cursor, graph, context| {
                typed_list_kind(item, true_, item_shape, cursor, graph, context)
            },
            |cursor, graph, context| {
                typed_list_kind(item, false_, item_shape, cursor, graph, context)
            },
            Item::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                typed_list_kind(item, branch, item_shape, cursor, graph, context)
            },
            Item::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                typed_list_kind(item, branch, item_shape, cursor, graph, context)
            },
            Item::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, result_shape),
            |branch, cursor, graph, context| {
                typed_list_kind(item, branch, item_shape, cursor, graph, context)
            },
            Item::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| {
                flow.and_then(|cursor, ()| {
                    typed_list_kind(item, return_, item_shape, cursor, graph, context)
                })
            }),
    }
}

fn lower_element_sequence<Element, Value>(
    elements: &[Element],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    mut lower: impl FnMut(
        &Element,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
) -> Lowered<Vec<Value>> {
    elements.iter().fold(
        Representability::Inhabited(DraftFlow::value(cursor, Vec::with_capacity(elements.len()))),
        |lowered, element| {
            lowered.and_then(|flow| {
                flow.and_then(|cursor, mut values| {
                    lower(element, cursor, graph, context).map(|flow| {
                        flow.map(|value| {
                            values.push(value);
                            values
                        })
                    })
                })
            })
        },
    )
}

fn lower_elements<Item>(
    elements: &[Item::ElementExpr],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<Item::DraftElement>>
where
    Item: GraphListItem,
{
    lower_element_sequence(elements, cursor, graph, context, Item::lower_element)
}

trait EraseDraftList {
    fn erase_list(&self) -> DraftList;
}

impl<Family> EraseDraftList for DraftTypedList<Family> {
    fn erase_list(&self) -> DraftList {
        self.value().clone()
    }
}

macro_rules! primitive_list_item {
    (
        $module_item:ty,
        $draft_element:ty,
        $draft_list:ty,
        $local:ty,
        $function:ty,
        $type_id:ty,
        $local_kind:ident,
        $element:ident,
        $constant:ident,
        $function_id:ident,
        $list_type:ident,
        $instruction:ident
    ) => {
        impl GraphListItem for $module_item {
            type DraftElement = $draft_element;
            type DraftList = $draft_list;
            type ExecutionLocal = $local;
            type ExecutionFunction = $function;
            type ExecutionType = $type_id;

            fn lower_element(
                element: &Self::ElementExpr,
                cursor: DraftCursor,
                graph: &mut DraftGraph,
                context: &mut super::super::LoweringContext,
            ) -> Lowered<Self::DraftElement> {
                super::$element(element, cursor, graph, context)
            }

            fn lower_constant(
                constant: &Self::Constant,
                context: &mut super::super::LoweringContext,
            ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>> {
                context
                    .$constant(constant)
                    .map(|id| execution::constant::ConstantId::new(id.index()))
            }

            fn local_key(
                local: &<Self as module::ListItem>::Local,
            ) -> super::super::local::LocalKey {
                super::super::local::LocalKey::new(
                    super::super::local::LocalKind::$local_kind,
                    local.0,
                )
            }

            fn lower_function(
                &self,
                function: &module::FunctionInstantiation,
                context: &mut super::super::LoweringContext,
            ) -> Representability<Self::ExecutionFunction> {
                context.$function_id(function)
            }

            fn list_type(
                &self,
                context: &mut super::super::LoweringContext,
            ) -> Self::ExecutionType {
                context.$list_type()
            }

            fn instruction(
                type_id: Self::ExecutionType,
                instruction: super::super::instruction::DraftTypedListInstruction<
                    Self::DraftElement,
                    Self::ExecutionLocal,
                    Self::ExecutionFunction,
                >,
            ) -> super::super::instruction::DraftListInstruction {
                super::super::instruction::DraftListInstruction::$instruction(type_id, instruction)
            }

            fn wrap(value: DraftList) -> Self::DraftList {
                <$draft_list>::new(value)
            }

            fn from_ref(value: &DraftValueRef) -> Self::DraftList {
                <$draft_list>::from_ref(value)
            }
        }
    };
}

primitive_list_item!(
    module::IntListItem,
    DraftInt,
    DraftIntList,
    execution::graph::IntListLocalId,
    execution::function::IntListFunctionId,
    execution::type_::IntListTypeId,
    IntList,
    int_expr,
    int_list_constant,
    int_list_function_id,
    int_list_type,
    Int
);
primitive_list_item!(
    module::StringListItem,
    DraftString,
    DraftStringList,
    execution::graph::StringListLocalId,
    execution::function::StringListFunctionId,
    execution::type_::StringListTypeId,
    StringList,
    string_expr,
    string_list_constant,
    string_list_function_id,
    string_list_type,
    String
);
primitive_list_item!(
    module::BitArrayListItem,
    DraftBitArray,
    DraftBitArrayList,
    execution::graph::BitArrayListLocalId,
    execution::function::BitArrayListFunctionId,
    execution::type_::BitArrayListTypeId,
    BitArrayList,
    bit_array_expr,
    bit_array_list_constant,
    bit_array_list_function_id,
    bit_array_list_type,
    BitArray
);
primitive_list_item!(
    module::UtfCodepointListItem,
    DraftUtfCodepoint,
    DraftUtfCodepointList,
    execution::graph::UtfCodepointListLocalId,
    execution::function::UtfCodepointListFunctionId,
    execution::type_::UtfCodepointListTypeId,
    UtfCodepointList,
    utf_codepoint_expr,
    utf_codepoint_list_constant,
    utf_codepoint_list_function_id,
    utf_codepoint_list_type,
    UtfCodepoint
);
primitive_list_item!(
    module::FloatListItem,
    DraftFloat,
    DraftFloatList,
    execution::graph::FloatListLocalId,
    execution::function::FloatListFunctionId,
    execution::type_::FloatListTypeId,
    FloatList,
    float_expr,
    float_list_constant,
    float_list_function_id,
    float_list_type,
    Float
);
primitive_list_item!(
    module::BoolListItem,
    DraftBool,
    DraftBoolList,
    execution::graph::BoolListLocalId,
    execution::function::BoolListFunctionId,
    execution::type_::BoolListTypeId,
    BoolList,
    bool_expr,
    bool_list_constant,
    bool_list_function_id,
    bool_list_type,
    Bool
);
primitive_list_item!(
    module::NilListItem,
    DraftNil,
    DraftNilList,
    execution::graph::NilListLocalId,
    execution::function::NilListFunctionId,
    execution::type_::NilListTypeId,
    NilList,
    nil_expr,
    nil_list_constant,
    nil_list_function_id,
    nil_list_type,
    Nil
);

impl GraphListItem for module::CustomListItem {
    type DraftElement = DraftCustom;
    type DraftList = DraftCustomList;
    type ExecutionLocal = execution::graph::CustomListLocalId;
    type ExecutionFunction = execution::function::CustomListFunctionId;
    type ExecutionType = execution::type_::CustomListTypeId;

    fn lower_element(
        element: &Self::ElementExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::LoweringContext,
    ) -> Lowered<Self::DraftElement> {
        custom::custom_expr(element, cursor, graph, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut super::super::LoweringContext,
    ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>> {
        context
            .custom_list_constant(constant)
            .map(|id| execution::constant::ConstantId::new(id.index()))
    }

    fn local_key(local: &<Self as module::ListItem>::Local) -> super::super::local::LocalKey {
        super::super::local::LocalKey::new(super::super::local::LocalKind::CustomList, local.0)
    }

    fn lower_function(
        &self,
        function: &module::FunctionInstantiation,
        context: &mut super::super::LoweringContext,
    ) -> Representability<Self::ExecutionFunction> {
        let type_id = context.custom_list_type(self.item_type());
        context.custom_list_function_id(function, type_id)
    }

    fn list_type(&self, context: &mut super::super::LoweringContext) -> Self::ExecutionType {
        context.custom_list_type(self.item_type())
    }

    fn instruction(
        type_id: Self::ExecutionType,
        instruction: super::super::instruction::DraftTypedListInstruction<
            Self::DraftElement,
            Self::ExecutionLocal,
            Self::ExecutionFunction,
        >,
    ) -> super::super::instruction::DraftListInstruction {
        super::super::instruction::DraftListInstruction::Custom(type_id, instruction)
    }

    fn wrap(value: DraftList) -> Self::DraftList {
        DraftCustomList::new(value)
    }

    fn from_ref(value: &DraftValueRef) -> Self::DraftList {
        DraftCustomList::from_ref(value)
    }
}

impl GraphListItem for module::TupleListItem {
    type DraftElement = DraftTuple;
    type DraftList = DraftTupleList;
    type ExecutionLocal = execution::graph::TupleListLocalId;
    type ExecutionFunction = execution::function::TupleListFunctionId;
    type ExecutionType = execution::type_::TupleListTypeId;

    fn lower_element(
        element: &Self::ElementExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::LoweringContext,
    ) -> Lowered<Self::DraftElement> {
        tuple::tuple_expr(element, cursor, graph, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut super::super::LoweringContext,
    ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>> {
        context
            .tuple_list_constant(constant)
            .map(|id| execution::constant::ConstantId::new(id.index()))
    }

    fn local_key(local: &<Self as module::ListItem>::Local) -> super::super::local::LocalKey {
        super::super::local::LocalKey::new(super::super::local::LocalKind::TupleList, local.0)
    }

    fn lower_function(
        &self,
        function: &module::FunctionInstantiation,
        context: &mut super::super::LoweringContext,
    ) -> Representability<Self::ExecutionFunction> {
        let type_id = context.tuple_list_type(self.item_type());
        context.tuple_list_function_id(function, type_id)
    }

    fn list_type(&self, context: &mut super::super::LoweringContext) -> Self::ExecutionType {
        context.tuple_list_type(self.item_type())
    }

    fn instruction(
        type_id: Self::ExecutionType,
        instruction: super::super::instruction::DraftTypedListInstruction<
            Self::DraftElement,
            Self::ExecutionLocal,
            Self::ExecutionFunction,
        >,
    ) -> super::super::instruction::DraftListInstruction {
        super::super::instruction::DraftListInstruction::Tuple(type_id, instruction)
    }

    fn wrap(value: DraftList) -> Self::DraftList {
        DraftTupleList::new(value)
    }

    fn from_ref(value: &DraftValueRef) -> Self::DraftList {
        DraftTupleList::from_ref(value)
    }
}

impl GraphListItem for module::ListListItem {
    type DraftElement = DraftStoredList;
    type DraftList = DraftListList;
    type ExecutionLocal = execution::graph::ListListLocalId;
    type ExecutionFunction = execution::function::ListListFunctionId;
    type ExecutionType = execution::type_::ListListTypeId;

    fn lower_element(
        element: &Self::ElementExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::LoweringContext,
    ) -> Lowered<Self::DraftElement> {
        stored_list_expr(element, cursor, graph, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut super::super::LoweringContext,
    ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>> {
        context
            .list_list_constant(constant)
            .map(|id| execution::constant::ConstantId::new(id.index()))
    }

    fn local_key(local: &<Self as module::ListItem>::Local) -> super::super::local::LocalKey {
        super::super::local::LocalKey::new(super::super::local::LocalKind::ListList, local.0)
    }

    fn lower_function(
        &self,
        function: &module::FunctionInstantiation,
        context: &mut super::super::LoweringContext,
    ) -> Representability<Self::ExecutionFunction> {
        let type_id = context.stored_list_list_type(self.item_shape());
        context.list_list_function_id(function, type_id)
    }

    fn list_type(&self, context: &mut super::super::LoweringContext) -> Self::ExecutionType {
        context.stored_list_list_type(self.item_shape())
    }

    fn instruction(
        type_id: Self::ExecutionType,
        instruction: super::super::instruction::DraftTypedListInstruction<
            Self::DraftElement,
            Self::ExecutionLocal,
            Self::ExecutionFunction,
        >,
    ) -> super::super::instruction::DraftListInstruction {
        super::super::instruction::DraftListInstruction::List(type_id, instruction)
    }

    fn wrap(value: DraftList) -> Self::DraftList {
        DraftListList::new(value)
    }

    fn from_ref(value: &DraftValueRef) -> Self::DraftList {
        DraftListList::from_ref(value)
    }
}

impl GraphListItem for module::FunctionListItem {
    type DraftElement = DraftFunction;
    type DraftList = DraftFunctionList;
    type ExecutionLocal = execution::graph::FunctionListLocalId;
    type ExecutionFunction = execution::function::FunctionListFunctionId;
    type ExecutionType = execution::type_::FunctionListTypeId;

    fn lower_element(
        element: &Self::ElementExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::LoweringContext,
    ) -> Lowered<Self::DraftElement> {
        function::function_expr(element, cursor, graph, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut super::super::LoweringContext,
    ) -> Representability<execution::constant::ConstantId<Self::ExecutionLocal>> {
        context
            .function_list_constant(constant)
            .map(|id| execution::constant::ConstantId::new(id.index()))
    }

    fn local_key(local: &<Self as module::ListItem>::Local) -> super::super::local::LocalKey {
        super::super::local::LocalKey::new(super::super::local::LocalKind::FunctionList, local.0)
    }

    fn lower_function(
        &self,
        function: &module::FunctionInstantiation,
        context: &mut super::super::LoweringContext,
    ) -> Representability<Self::ExecutionFunction> {
        let type_id = context.function_list_type(self.item_type());
        context.function_list_function_id(function, type_id)
    }

    fn list_type(&self, context: &mut super::super::LoweringContext) -> Self::ExecutionType {
        context.function_list_type(self.item_type())
    }

    fn instruction(
        type_id: Self::ExecutionType,
        instruction: super::super::instruction::DraftTypedListInstruction<
            Self::DraftElement,
            Self::ExecutionLocal,
            Self::ExecutionFunction,
        >,
    ) -> super::super::instruction::DraftListInstruction {
        super::super::instruction::DraftListInstruction::Function(type_id, instruction)
    }

    fn wrap(value: DraftList) -> Self::DraftList {
        DraftFunctionList::new(value)
    }

    fn from_ref(value: &DraftValueRef) -> Self::DraftList {
        DraftFunctionList::from_ref(value)
    }
}

pub(in crate::plan::execution::lowering) fn int_list_expr(
    expression: &module::IntListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftIntList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn string_list_expr(
    expression: &module::StringListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftStringList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn bit_array_list_expr(
    expression: &module::BitArrayListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftBitArrayList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn utf_codepoint_list_expr(
    expression: &module::UtfCodepointListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftUtfCodepointList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn float_list_expr(
    expression: &module::FloatListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftFloatList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn bool_list_expr(
    expression: &module::BoolListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftBoolList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn nil_list_expr(
    expression: &module::NilListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftNilList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn custom_list_expr(
    expression: &module::CustomListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftCustomList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn tuple_list_expr(
    expression: &module::TupleListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftTupleList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn list_list_expr(
    expression: &module::ListListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftListList> {
    typed_list_expr(expression, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn function_list_expr(
    expression: &module::FunctionListExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftFunctionList> {
    typed_list_expr(expression, cursor, graph, context)
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::{
        CustomConstructorDefinition, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, Expr, FunctionShape, FunctionTemplateId, FunctionType, IntExpr,
        ListExpr, PanicExpr, PanicSite, TypeParameterId, ValueShape, ValueType,
    };

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome {
        Uninhabited,
        Diverged,
        Value,
    }

    fn flow_outcome<T>(flow: Representability<DraftFlow<T>>) -> FlowOutcome {
        match flow {
            Representability::Uninhabited => FlowOutcome::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => FlowOutcome::Diverged,
            Representability::Inhabited(DraftFlow::Value { .. }) => FlowOutcome::Value,
        }
    }

    struct ListFamily {
        item_type: &'static str,
        item: &'static str,
        constant: bool,
    }

    const LIST_FAMILIES: &[ListFamily] = &[
        ListFamily {
            item_type: "Int",
            item: "1",
            constant: true,
        },
        ListFamily {
            item_type: "Float",
            item: "1.5",
            constant: true,
        },
        ListFamily {
            item_type: "String",
            item: "\"one\"",
            constant: true,
        },
        ListFamily {
            item_type: "BitArray",
            item: "<<1>>",
            constant: true,
        },
        ListFamily {
            item_type: "UtfCodepoint",
            item: "codepoint()",
            constant: false,
        },
        ListFamily {
            item_type: "Marker",
            item: "Marker(1)",
            constant: true,
        },
        ListFamily {
            item_type: "Bool",
            item: "True",
            constant: true,
        },
        ListFamily {
            item_type: "Nil",
            item: "Nil",
            constant: true,
        },
        ListFamily {
            item_type: "#(Int)",
            item: "#(1)",
            constant: true,
        },
        ListFamily {
            item_type: "List(Int)",
            item: "[1]",
            constant: true,
        },
        ListFamily {
            item_type: "fn() -> Int",
            item: "int_value",
            constant: true,
        },
    ];

    #[test]
    fn every_list_family_lowers_each_list_expression_owner() {
        for family in LIST_FAMILIES {
            let mut expressions = vec![
                format!("[{}]", family.item),
                format!("[{}, ..[]]", family.item),
                format!("{{ let local = [{}] local }}", family.item),
                "provider()".to_owned(),
                "{ let callable = provider callable() }".to_owned(),
                format!("#([{}]).0", family.item),
                format!("Holder(selected: [{}]).selected", family.item),
                format!(
                    "case [[{item}]] {{ [selected] -> selected _ -> [] }}",
                    item = family.item,
                ),
                format!(
                    "case [{item}, {item}] {{ [_, ..tail] -> tail _ -> [] }}",
                    item = family.item,
                ),
                format!(
                    "case True {{ True -> [{item}] False -> [] }}",
                    item = family.item,
                ),
                format!("case 1 {{ 1 -> [{item}] _ -> [] }}", item = family.item,),
                format!(
                    "case \"selected\" {{ \"selected\" -> [{item}] _ -> [] }}",
                    item = family.item,
                ),
                format!("case 1.0 {{ 1.0 -> [{item}] _ -> [] }}", item = family.item,),
                format!("{{ let _ = Nil [{}] }}", family.item),
            ];
            if family.constant {
                expressions.push("selected_constant".to_owned());
            }

            for expression in expressions {
                let source = source(family, &expression);
                assert_eq!(
                    crate::run_main(&execution_plan(&source), &mut Vec::new()),
                    Ok(Value::Bool(true)),
                    "failed list item family {} expression {expression}",
                    family.item_type,
                );
            }
        }
    }

    #[test]
    fn every_list_family_preserves_its_source_stop() {
        for family in LIST_FAMILIES {
            let source = format!(
                r#"
pub type Marker {{ Marker(Int) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn int_value() -> Int {{ 1 }}

fn selected() -> List({item_type}) {{ panic as "selected" }}

pub fn main() {{
  selected() == [{item}]
}}
"#,
                item_type = family.item_type,
                item = family.item,
            );
            let error = crate::run_main(&execution_plan(&source), &mut Vec::new()).unwrap_err();
            assert_eq!(error.to_string(), "panic: selected");
        }
    }

    #[test]
    fn specialized_generic_list_panic_stops_before_value_construction() {
        let source = r#"
fn selected() -> List(item) {
  panic as "selected"
}

pub fn main() {
  selected() == [1]
}
"#;

        assert_eq!(
            crate::run_main(&execution_plan(source), &mut Vec::new())
                .unwrap_err()
                .to_string(),
            "panic: selected",
        );
    }

    #[test]
    fn specialized_generic_list_constant_lowers_to_stored_list_storage() {
        let source = r#"
const empty = []

fn selected(_sample: item) -> List(item) {
  empty
}

pub fn main() {
  selected(1) == []
}
"#;

        assert_eq!(
            crate::run_main(&execution_plan(source), &mut Vec::new()),
            Ok(Value::Bool(true)),
        );
    }

    #[test]
    fn element_sequence_propagates_an_uninhabited_element() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        let flow =
            super::lower_element_sequence(&[()], cursor, &mut graph, &mut context, |_, _, _, _| {
                Representability::<DraftFlow<()>>::Uninhabited
            });

        assert_eq!(flow_outcome(flow), FlowOutcome::Uninhabited);
    }

    #[test]
    fn every_list_family_stops_when_an_owner_source_diverges() {
        for family in LIST_FAMILIES {
            let expressions = [
                "[panic as \"source\"]".to_owned(),
                "[panic as \"source\", ..[]]".to_owned(),
                format!("[{}, ..panic as \"source\"]", family.item),
                "provider(panic as \"source\")".to_owned(),
                "{ panic as \"source\" }()".to_owned(),
                format!(
                    "{{ let callable: fn() -> List({}) = panic as \"source\" callable() }}",
                    family.item_type,
                ),
                format!("#(panic as \"source\", [{}]).1", family.item),
                "Holder(selected: panic as \"source\").selected".to_owned(),
                "{ panic as \"source\" }[0]".to_owned(),
                format!(
                    "case [[panic as \"source\"]] {{ [selected] -> selected _ -> [{}] }}",
                    family.item,
                ),
                format!(
                    "case [panic as \"source\"] {{ [_, ..tail] -> tail _ -> [{}] }}",
                    family.item,
                ),
                format!(
                    "case panic as \"source\" {{ True -> [{item}] False -> [] }}",
                    item = family.item,
                ),
                format!(
                    "case panic as \"source\" {{ 1 -> [{item}] _ -> [] }}",
                    item = family.item,
                ),
                format!(
                    "case panic as \"source\" {{ \"selected\" -> [{item}] _ -> [] }}",
                    item = family.item,
                ),
                format!(
                    "case panic as \"source\" {{ 1.0 -> [{item}] _ -> [] }}",
                    item = family.item,
                ),
                format!(
                    "{{ let failed: Int = panic as \"source\" let _ = failed [{}] }}",
                    family.item,
                ),
            ];

            for expression in expressions {
                let source = diverging_source(family, &expression);
                let error = crate::run_main(&execution_plan(&source), &mut Vec::new()).unwrap_err();
                assert_eq!(
                    error.to_string(),
                    "panic: source",
                    "failed list item family {} expression {expression}",
                    family.item_type,
                );
            }
        }
    }

    #[test]
    fn planner_generated_nested_list_projections_stop_before_every_list_family_output() {
        let parameter = TypeParameterId(0);
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let custom_type = CustomType::new(custom_name.clone(), Vec::new());
        let custom_definition = CustomTypeDefinition::new(
            custom_name,
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Marker".into(),
                0,
                Vec::new(),
            )],
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let item_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(function_type)),
        ];
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let parameter_source = ListExpr::panic(
            panic(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("a parameter item type should create a parameter-list list");
        let expressions = std::iter::once(ListExpr::parameter_list_index(parameter_source, 0))
            .chain(item_types.into_iter().map(|item| {
                ListExpr::list_index(
                    ListExpr::panic(panic(), ValueType::List(Box::new(item)))
                        .into_list()
                        .expect("a list item type should create a nested list"),
                    0,
                )
            }))
            .collect::<Vec<_>>();
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let value = ListExpr::try_value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int)
            .expect("an Int item should create an Int list expression");

        assert_eq!(
            flow_outcome(super::list_expr(&value, cursor, &mut graph, &mut context,)),
            FlowOutcome::Value,
        );

        let erased_call = ListExpr::call(
            crate::plan::monomorphic_function_instantiation(
                0,
                FunctionShape::new(Vec::new(), ValueShape::List(Box::new(ValueShape::Int))),
            ),
            Vec::new(),
            ValueShape::Int,
        );
        context
            .erased_specializations
            .insert(SpecializationKey::monomorphic(FunctionTemplateId::new(0)));
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::list_expr(
                &erased_call,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
        context.erased_specializations.clear();

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(super::list_expr(
                    &expression,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );
        }
    }

    fn source(family: &ListFamily, expression: &str) -> String {
        let constant = if family.constant {
            format!("const selected_constant = [{}]", family.item)
        } else {
            String::new()
        };
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn int_value() -> Int {{ 1 }}
fn provider() -> List({item_type}) {{ [{item}] }}
{constant}

pub fn main() {{
  let selected: List({item_type}) = {expression}
  selected == [{item}]
}}
"#,
            item_type = family.item_type,
            item = family.item,
        )
    }

    fn diverging_source(family: &ListFamily, expression: &str) -> String {
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn int_value() -> Int {{ 1 }}
fn provider(_value: Int) -> List({item_type}) {{ [{item}] }}

pub fn main() {{
  let selected: List({item_type}) = {expression}
  selected == [{item}]
}}
"#,
            item_type = family.item_type,
            item = family.item,
        )
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module)
    }
}
