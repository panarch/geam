use super::super::function::ExecutableFunction;
use super::super::table::FunctionTables;
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionReturn, BitArrayListFunctionId,
    BitArrayListReturn, BitArrayReturn, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn,
    BoolListFunctionId, BoolListReturn, BoolReturn, CustomFunctionFunctionId, CustomFunctionReturn,
    CustomListFunctionId, CustomListReturn, CustomReturn, FloatFunctionFunctionId, FloatFunctionId,
    FloatFunctionReturn, FloatListFunctionId, FloatListReturn, FloatReturn,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionReturn, FunctionListFunctionId,
    FunctionListReturn, IntFunctionFunctionId, IntFunctionId, IntFunctionReturn, IntListFunctionId,
    IntListReturn, IntReturn, ListFunctionFunctionId, ListFunctionId, ListFunctionReturn,
    ListListFunctionId, ListListReturn, NilFunctionFunctionId, NilFunctionId, NilFunctionReturn,
    NilListFunctionId, NilListReturn, NilReturn, RuntimeFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringFunctionReturn, StringListFunctionId, StringListReturn, StringReturn,
    TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn, TupleListFunctionId,
    TupleListReturn, TupleReturn, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointFunctionReturn, UtfCodepointListFunctionId, UtfCodepointListReturn,
    UtfCodepointReturn,
};
use super::LoweringContext;
use crate::plan::module;

#[derive(Default)]
pub(super) struct FunctionTableBuilder {
    int_functions: Vec<(usize, ExecutableFunction<IntReturn>)>,
    float_functions: Vec<(usize, ExecutableFunction<FloatReturn>)>,
    string_functions: Vec<(usize, ExecutableFunction<StringReturn>)>,
    bit_array_functions: Vec<(usize, ExecutableFunction<BitArrayReturn>)>,
    utf_codepoint_functions: Vec<(usize, ExecutableFunction<UtfCodepointReturn>)>,
    custom_functions: Vec<(usize, ExecutableFunction<CustomReturn>)>,
    bool_functions: Vec<(usize, ExecutableFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, ExecutableFunction<NilReturn>)>,
    tuple_functions: Vec<(usize, ExecutableFunction<TupleReturn>)>,
    int_list_functions: Vec<(IntListFunctionId, ExecutableFunction<IntListReturn>)>,
    string_list_functions: Vec<(StringListFunctionId, ExecutableFunction<StringListReturn>)>,
    bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        ExecutableFunction<BitArrayListReturn>,
    )>,
    utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        ExecutableFunction<UtfCodepointListReturn>,
    )>,
    custom_list_functions: Vec<(CustomListFunctionId, ExecutableFunction<CustomListReturn>)>,
    float_list_functions: Vec<(FloatListFunctionId, ExecutableFunction<FloatListReturn>)>,
    bool_list_functions: Vec<(BoolListFunctionId, ExecutableFunction<BoolListReturn>)>,
    nil_list_functions: Vec<(NilListFunctionId, ExecutableFunction<NilListReturn>)>,
    tuple_list_functions: Vec<(TupleListFunctionId, ExecutableFunction<TupleListReturn>)>,
    list_list_functions: Vec<(ListListFunctionId, ExecutableFunction<ListListReturn>)>,
    function_list_functions: Vec<(
        FunctionListFunctionId,
        ExecutableFunction<FunctionListReturn>,
    )>,
    int_function_functions: Vec<(usize, ExecutableFunction<IntFunctionReturn>)>,
    float_function_functions: Vec<(usize, ExecutableFunction<FloatFunctionReturn>)>,
    string_function_functions: Vec<(usize, ExecutableFunction<StringFunctionReturn>)>,
    bit_array_function_functions: Vec<(usize, ExecutableFunction<BitArrayFunctionReturn>)>,
    utf_codepoint_function_functions: Vec<(usize, ExecutableFunction<UtfCodepointFunctionReturn>)>,
    custom_function_functions: Vec<(usize, ExecutableFunction<CustomFunctionReturn>)>,
    bool_function_functions: Vec<(usize, ExecutableFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, ExecutableFunction<NilFunctionReturn>)>,
    tuple_function_functions: Vec<(usize, ExecutableFunction<TupleFunctionReturn>)>,
    int_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    string_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    bit_array_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    utf_codepoint_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    custom_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    float_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    bool_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    nil_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    tuple_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    list_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    function_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    function_function_functions: Vec<(usize, ExecutableFunction<FunctionFunctionReturn>)>,
}

impl FunctionTableBuilder {
    pub(super) fn push(
        &mut self,
        function: module::FunctionPlan,
        context: &mut LoweringContext,
    ) -> RuntimeFunctionId {
        let module::FunctionExecutionParts {
            frame_layout,
            steps,
            return_,
        } = function.into_execution_parts();
        let frame_layout = super::frame::frame_layout(frame_layout, context);
        let steps = super::step::steps(steps, context);

        match return_.into_kind() {
            module::ReturnExprKind::Int { runtime_id, body } => {
                let id = IntFunctionId(runtime_id.0);
                self.int_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::int_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Int(id)
            }
            module::ReturnExprKind::Float { runtime_id, body } => {
                let id = FloatFunctionId(runtime_id.0);
                self.float_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::float_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Float(id)
            }
            module::ReturnExprKind::String { runtime_id, body } => {
                let id = StringFunctionId(runtime_id.0);
                self.string_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::string_return(body, context),
                    ),
                ));
                RuntimeFunctionId::String(id)
            }
            module::ReturnExprKind::BitArray { runtime_id, body } => {
                let id = BitArrayFunctionId(runtime_id.0);
                self.bit_array_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bit_array_return(body, context),
                    ),
                ));
                RuntimeFunctionId::BitArray(id)
            }
            module::ReturnExprKind::UtfCodepoint { runtime_id, body } => {
                let id = UtfCodepointFunctionId(runtime_id.0);
                self.utf_codepoint_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::utf_codepoint_return(body, context),
                    ),
                ));
                RuntimeFunctionId::UtfCodepoint(id)
            }
            module::ReturnExprKind::Custom { runtime_id, body } => {
                let id = super::id::custom_function_id(runtime_id, context);
                self.custom_functions.push((
                    id.index(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::custom_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Custom(id)
            }
            module::ReturnExprKind::Bool { runtime_id, body } => {
                let id = BoolFunctionId(runtime_id.0);
                self.bool_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bool_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Bool(id)
            }
            module::ReturnExprKind::Nil { runtime_id, body } => {
                let id = NilFunctionId(runtime_id.0);
                self.nil_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::nil_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Nil(id)
            }
            module::ReturnExprKind::Tuple {
                runtime_id,
                type_,
                body,
            } => {
                let id = TupleFunctionId(runtime_id.0);
                self.tuple_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::tuple_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Tuple {
                    id,
                    return_type: type_
                        .into_iter()
                        .map(|type_| context.value_type(type_))
                        .collect(),
                }
            }
            module::ReturnExprKind::IntList { runtime_id, body } => {
                let id = IntListFunctionId::new(runtime_id.0, context.int_list_type());
                self.int_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::int_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Int(id))
            }
            module::ReturnExprKind::StringList { runtime_id, body } => {
                let id = StringListFunctionId::new(runtime_id.0, context.string_list_type());
                self.string_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::string_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::String(id))
            }
            module::ReturnExprKind::BitArrayList { runtime_id, body } => {
                let id = BitArrayListFunctionId::new(runtime_id.0, context.bit_array_list_type());
                self.bit_array_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bit_array_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::BitArray(id))
            }
            module::ReturnExprKind::UtfCodepointList { runtime_id, body } => {
                let id = UtfCodepointListFunctionId::new(
                    runtime_id.0,
                    context.utf_codepoint_list_type(),
                );
                self.utf_codepoint_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::utf_codepoint_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::UtfCodepoint(id))
            }
            module::ReturnExprKind::CustomList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.custom_list_type(item_type);
                let id = CustomListFunctionId::new(runtime_id.0, type_id);
                self.custom_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::custom_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Custom(id))
            }
            module::ReturnExprKind::FloatList { runtime_id, body } => {
                let id = FloatListFunctionId::new(runtime_id.0, context.float_list_type());
                self.float_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::float_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Float(id))
            }
            module::ReturnExprKind::BoolList { runtime_id, body } => {
                let id = BoolListFunctionId::new(runtime_id.0, context.bool_list_type());
                self.bool_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bool_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Bool(id))
            }
            module::ReturnExprKind::NilList { runtime_id, body } => {
                let id = NilListFunctionId::new(runtime_id.0, context.nil_list_type());
                self.nil_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::nil_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Nil(id))
            }
            module::ReturnExprKind::TupleList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.tuple_list_type(item_type);
                let id = TupleListFunctionId::new(runtime_id.0, type_id);
                self.tuple_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::tuple_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Tuple(id))
            }
            module::ReturnExprKind::ListList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.list_list_type(*item_type);
                let id = ListListFunctionId::new(runtime_id.0, type_id);
                self.list_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::list_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::List(id))
            }
            module::ReturnExprKind::FunctionList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.function_list_type(item_type);
                let id = FunctionListFunctionId::new(runtime_id.0, type_id);
                self.function_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::function_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(ListFunctionId::Function(id))
            }
            module::ReturnExprKind::IntFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = IntFunctionFunctionId(runtime_id.0);
                self.int_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::int_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::FloatFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = FloatFunctionFunctionId(runtime_id.0);
                self.float_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::float_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Float(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::StringFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = StringFunctionFunctionId(runtime_id.0);
                self.string_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::string_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::String(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::BitArrayFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = BitArrayFunctionFunctionId(runtime_id.0);
                self.bit_array_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bit_array_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::BitArray(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::UtfCodepointFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = UtfCodepointFunctionFunctionId(runtime_id.0);
                self.utf_codepoint_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::utf_codepoint_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::UtfCodepoint(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::CustomFunction { runtime_id, body } => {
                let id = CustomFunctionFunctionId::new(
                    runtime_id.index(),
                    context.custom_function_type(runtime_id.type_().clone()),
                );
                self.custom_function_functions.push((
                    runtime_id.index(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::custom_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    return_type: id.type_().to_function_type(),
                    id: FunctionFunctionId::Custom(id),
                }
            }
            module::ReturnExprKind::BoolFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = BoolFunctionFunctionId(runtime_id.0);
                self.bool_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::bool_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Bool(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::NilFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = NilFunctionFunctionId(runtime_id.0);
                self.nil_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::nil_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Nil(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::TupleFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = TupleFunctionFunctionId(runtime_id.0);
                self.tuple_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::tuple_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Tuple(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::ListFunction { runtime_id, body } => {
                let runtime_id = super::id::list_function_function_id(runtime_id, context);
                let return_type = runtime_id.type_().clone();
                self.push_list_function_function(
                    runtime_id.clone(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::list_function_return(body, context),
                    ),
                );
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::List(runtime_id),
                    return_type,
                }
            }
            module::ReturnExprKind::FunctionFunction { runtime_id, body } => {
                let id = FunctionFunctionFunctionId::new(
                    runtime_id.index(),
                    context.function_function_type(runtime_id.type_().clone()),
                );
                self.function_function_functions.push((
                    runtime_id.index(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        super::return_::function_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    return_type: id.type_().to_function_type(),
                    id: FunctionFunctionId::Function(id),
                }
            }
        }
    }

    fn push_list_function_function(
        &mut self,
        id: ListFunctionFunctionId,
        function: ExecutableFunction<ListFunctionReturn>,
    ) {
        match id {
            ListFunctionFunctionId::Int { id, .. } => {
                self.int_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::String { id, .. } => {
                self.string_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::BitArray { id, .. } => {
                self.bit_array_list_function_functions
                    .push((id.0, function));
            }
            ListFunctionFunctionId::UtfCodepoint { id, .. } => {
                self.utf_codepoint_list_function_functions
                    .push((id.0, function));
            }
            ListFunctionFunctionId::Custom { id, .. } => {
                self.custom_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Float { id, .. } => {
                self.float_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Bool { id, .. } => {
                self.bool_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Nil { id, .. } => {
                self.nil_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Tuple { id, .. } => {
                self.tuple_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::List { id, .. } => {
                self.list_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Function { id, .. } => {
                self.function_list_function_functions.push((id.0, function));
            }
        }
    }

    pub(super) fn finish(self) -> FunctionTables {
        FunctionTables {
            int_functions: sort_functions(self.int_functions),
            float_functions: sort_functions(self.float_functions),
            string_functions: sort_functions(self.string_functions),
            bit_array_functions: sort_functions(self.bit_array_functions),
            utf_codepoint_functions: sort_functions(self.utf_codepoint_functions),
            custom_functions: sort_functions(self.custom_functions),
            bool_functions: sort_functions(self.bool_functions),
            nil_functions: sort_functions(self.nil_functions),
            tuple_functions: sort_functions(self.tuple_functions),
            int_list_functions: sort_list_functions(self.int_list_functions, |id| id.index()),
            string_list_functions: sort_list_functions(self.string_list_functions, |id| id.index()),
            bit_array_list_functions: sort_list_functions(self.bit_array_list_functions, |id| {
                id.index()
            }),
            utf_codepoint_list_functions: sort_list_functions(
                self.utf_codepoint_list_functions,
                |id| id.index(),
            ),
            custom_list_functions: sort_list_functions(self.custom_list_functions, |id| id.index()),
            float_list_functions: sort_list_functions(self.float_list_functions, |id| id.index()),
            bool_list_functions: sort_list_functions(self.bool_list_functions, |id| id.index()),
            nil_list_functions: sort_list_functions(self.nil_list_functions, |id| id.index()),
            tuple_list_functions: sort_list_functions(self.tuple_list_functions, |id| id.index()),
            list_list_functions: sort_list_functions(self.list_list_functions, |id| id.index()),
            function_list_functions: sort_list_functions(self.function_list_functions, |id| {
                id.index()
            }),
            int_function_functions: sort_functions(self.int_function_functions),
            float_function_functions: sort_functions(self.float_function_functions),
            string_function_functions: sort_functions(self.string_function_functions),
            bit_array_function_functions: sort_functions(self.bit_array_function_functions),
            utf_codepoint_function_functions: sort_functions(self.utf_codepoint_function_functions),
            custom_function_functions: sort_functions(self.custom_function_functions),
            bool_function_functions: sort_functions(self.bool_function_functions),
            nil_function_functions: sort_functions(self.nil_function_functions),
            tuple_function_functions: sort_functions(self.tuple_function_functions),
            int_list_function_functions: sort_functions(self.int_list_function_functions),
            string_list_function_functions: sort_functions(self.string_list_function_functions),
            bit_array_list_function_functions: sort_functions(
                self.bit_array_list_function_functions,
            ),
            utf_codepoint_list_function_functions: sort_functions(
                self.utf_codepoint_list_function_functions,
            ),
            custom_list_function_functions: sort_functions(self.custom_list_function_functions),
            float_list_function_functions: sort_functions(self.float_list_function_functions),
            bool_list_function_functions: sort_functions(self.bool_list_function_functions),
            nil_list_function_functions: sort_functions(self.nil_list_function_functions),
            tuple_list_function_functions: sort_functions(self.tuple_list_function_functions),
            list_list_function_functions: sort_functions(self.list_list_function_functions),
            function_list_function_functions: sort_functions(self.function_list_function_functions),
            function_function_functions: sort_functions(self.function_function_functions),
        }
    }
}

fn sort_functions<Return>(
    mut functions: Vec<(usize, ExecutableFunction<Return>)>,
) -> Vec<ExecutableFunction<Return>> {
    functions.sort_by_key(|(index, _)| *index);
    functions
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}

fn sort_list_functions<Id, Return>(
    mut functions: Vec<(Id, ExecutableFunction<Return>)>,
    index: impl Fn(&Id) -> usize,
) -> Vec<(Id, ExecutableFunction<Return>)> {
    functions.sort_by_key(|(id, _)| index(id));
    functions
}

#[cfg(test)]
mod tests {
    use super::super::super::{ExecutionPlan, IntFunctionId, RuntimeFunctionId};

    #[test]
    fn lowering_builds_every_typed_function_table() {
        let source = r#"
fn int_value() { 1 }
fn float_value() { 1.0 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }

fn int_list() { [1] }
fn string_list() { ["one"] }
fn bit_array_list() { [<<1>>] }
fn utf_codepoint_list() { [utf_codepoint_value()] }
fn float_list() { [1.0] }
fn bool_list() { [True] }
fn nil_list() { [Nil] }
fn tuple_list() { [#(1)] }
fn list_list() { [[1]] }
fn function_list() { [int_value] }

fn int_function() { int_value }
fn float_function() { float_value }
fn string_function() { string_value }
fn bit_array_function() { bit_array_value }
fn utf_codepoint_function() { utf_codepoint_value }
fn bool_function() { bool_value }
fn nil_function() { nil_value }
fn tuple_function() { tuple_value }
fn int_list_function() { int_list }
fn string_list_function() { string_list }
fn bit_array_list_function() { bit_array_list }
fn utf_codepoint_list_function() { utf_codepoint_list }
fn float_list_function() { float_list }
fn bool_list_function() { bool_list }
fn nil_list_function() { nil_list }
fn tuple_list_function() { tuple_list }
fn list_list_function() { list_list }
fn function_list_function() { function_list }
fn function_function() { int_function }

pub fn main() { int_value() }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
        assert_eq!(plan.functions.int_functions.len(), 2);
        assert_eq!(plan.functions.float_functions.len(), 1);
        assert_eq!(plan.functions.string_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_functions.len(), 1);
        assert_eq!(plan.functions.utf_codepoint_functions.len(), 1);
        assert_eq!(plan.functions.bool_functions.len(), 1);
        assert_eq!(plan.functions.nil_functions.len(), 1);
        assert_eq!(plan.functions.tuple_functions.len(), 1);
        assert_eq!(plan.functions.int_list_functions.len(), 1);
        assert_eq!(plan.functions.string_list_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_list_functions.len(), 1);
        assert_eq!(plan.functions.utf_codepoint_list_functions.len(), 1);
        assert_eq!(plan.functions.float_list_functions.len(), 1);
        assert_eq!(plan.functions.bool_list_functions.len(), 1);
        assert_eq!(plan.functions.nil_list_functions.len(), 1);
        assert_eq!(plan.functions.tuple_list_functions.len(), 1);
        assert_eq!(plan.functions.list_list_functions.len(), 1);
        assert_eq!(plan.functions.function_list_functions.len(), 1);
        assert_eq!(plan.functions.int_function_functions.len(), 1);
        assert_eq!(plan.functions.float_function_functions.len(), 1);
        assert_eq!(plan.functions.string_function_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_function_functions.len(), 1);
        assert_eq!(plan.functions.utf_codepoint_function_functions.len(), 1);
        assert_eq!(plan.functions.bool_function_functions.len(), 1);
        assert_eq!(plan.functions.nil_function_functions.len(), 1);
        assert_eq!(plan.functions.tuple_function_functions.len(), 1);
        assert_eq!(plan.functions.int_list_function_functions.len(), 1);
        assert_eq!(plan.functions.string_list_function_functions.len(), 1);
        assert_eq!(plan.functions.bit_array_list_function_functions.len(), 1);
        assert_eq!(
            plan.functions.utf_codepoint_list_function_functions.len(),
            1
        );
        assert_eq!(plan.functions.float_list_function_functions.len(), 1);
        assert_eq!(plan.functions.bool_list_function_functions.len(), 1);
        assert_eq!(plan.functions.nil_list_function_functions.len(), 1);
        assert_eq!(plan.functions.tuple_list_function_functions.len(), 1);
        assert_eq!(plan.functions.list_list_function_functions.len(), 1);
        assert_eq!(plan.functions.function_list_function_functions.len(), 1);
        assert_eq!(plan.functions.function_function_functions.len(), 1);
    }
}
