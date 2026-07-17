use super::super::function::ExecutableFunction;
use super::super::table::FunctionTables;
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionReturn, BitArrayListFunctionId,
    BitArrayListReturn, BitArrayReturn, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn,
    BoolListFunctionId, BoolListReturn, BoolReturn, CustomFunctionReturn, CustomListFunctionId,
    CustomListReturn, CustomReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn,
    FloatListFunctionId, FloatListReturn, FloatReturn, FunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, IntFunctionFunctionId, IntFunctionId,
    IntFunctionReturn, IntListFunctionId, IntListReturn, IntReturn, ListFunctionFunctionId,
    ListFunctionId, ListFunctionReturn, ListListFunctionId, ListListReturn, NilFunctionFunctionId,
    NilFunctionId, NilFunctionReturn, NilListFunctionId, NilListReturn, NilReturn,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, StringFunctionReturn,
    StringListFunctionId, StringListReturn, StringReturn, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionReturn, TupleListFunctionId, TupleListReturn, TupleReturn,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointFunctionReturn,
    UtfCodepointListFunctionId, UtfCodepointListReturn, UtfCodepointReturn,
};
use super::LoweringContext;
use super::specialization::{ConcreteFunctionShape, ConcreteValueShape};
use crate::plan::module;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum FunctionTableFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ListList,
    FunctionList,
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    CustomListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ListListFunction,
    FunctionListFunction,
    FunctionFunction,
}

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

pub(super) fn lower_specialized(
    template: &module::FunctionTemplate,
    key: &super::specialization::SpecializationKey,
    context: &mut LoweringContext,
) {
    let frame_layout = super::frame::frame_layout(context);
    let steps = super::step::steps(template.steps(), context);
    let index = context.specialization_index(key);
    let mut functions = std::mem::take(&mut context.functions);

    match template.return_().kind() {
        module::ReturnExprKind::Generic { parameter, body } => {
            let shape = context.concrete_parameter(*parameter);
            lower_generic_return(
                index,
                frame_layout,
                steps,
                body,
                &shape,
                &mut functions,
                context,
            );
        }
        module::ReturnExprKind::Int { body } => functions.int_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::int_return(body, context),
            ),
        )),
        module::ReturnExprKind::Float { body } => functions.float_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::float_return(body, context),
            ),
        )),
        module::ReturnExprKind::String { body } => functions.string_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::string_return(body, context),
            ),
        )),
        module::ReturnExprKind::BitArray { body } => {
            functions.bit_array_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::bit_array_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::UtfCodepoint { body } => {
            functions.utf_codepoint_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::utf_codepoint_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::Custom { body } => functions.custom_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::custom_return(body, context),
            ),
        )),
        module::ReturnExprKind::Bool { body } => functions.bool_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::bool_return(body, context),
            ),
        )),
        module::ReturnExprKind::Nil { body } => functions.nil_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::nil_return(body, context),
            ),
        )),
        module::ReturnExprKind::Tuple { body, .. } => functions.tuple_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::tuple_return(body, context),
            ),
        )),
        module::ReturnExprKind::GenericList { parameter, body } => {
            let item = context.concrete_parameter(*parameter);
            lower_generic_list_return(
                index,
                frame_layout,
                steps,
                body,
                &item,
                &mut functions,
                context,
            );
        }
        module::ReturnExprKind::IntList { body } => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::int_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::StringList { body } => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::string_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::BitArrayList { body } => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::bit_array_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::UtfCodepointList { body } => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::utf_codepoint_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::CustomList { item_type, body } => {
            let type_id = context.custom_list_type(item_type.clone());
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::custom_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::FloatList { body } => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::float_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::BoolList { body } => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::bool_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::NilList { body } => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::nil_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::TupleList { item_type, body } => {
            let type_id = context.tuple_list_type(item_type.clone());
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::tuple_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::ListList { item_type, body } => {
            let type_id = context.list_list_type(*item_type.clone());
            let id = ListListFunctionId::new(index, type_id);
            functions.list_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::list_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::FunctionList { item_type, body } => {
            let type_id = context.function_list_type(item_type.clone());
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::function_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::GenericFunction { shape, body } => {
            let function = context.concrete_function_shape(shape);
            lower_generic_function_return(
                index,
                frame_layout,
                steps,
                body,
                &function,
                &mut functions,
                context,
            );
        }
        module::ReturnExprKind::IntFunction { shape, body } => {
            functions.int_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::int_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::FloatFunction { shape, body } => {
            functions.float_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::float_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::StringFunction { shape, body } => {
            functions.string_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::string_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::BitArrayFunction { shape, body } => {
            functions.bit_array_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::bit_array_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::UtfCodepointFunction { shape, body } => {
            functions.utf_codepoint_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::utf_codepoint_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::CustomFunction { shape, body } => {
            functions.custom_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::custom_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::BoolFunction { shape, body } => {
            functions.bool_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::bool_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::NilFunction { shape, body } => {
            functions.nil_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::nil_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::TupleFunction { shape, body } => {
            functions.tuple_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::tuple_function_return(shape, body, context),
                ),
            ));
        }
        module::ReturnExprKind::ListFunction {
            shape,
            item_type,
            body,
        } => {
            let item = context
                .concrete_value_shape(&crate::plan::ValueShape::from_value_type(item_type.clone()));
            let lowered = super::return_::list_function_return(shape, body, &item, context);
            push_list_function_function(
                &mut functions,
                index,
                &item,
                ExecutableFunction::new(frame_layout, steps, lowered),
            );
        }
        module::ReturnExprKind::FunctionFunction { shape, body } => {
            functions.function_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::function_function_return(shape, body, context),
                ),
            ));
        }
    }

    context.functions = functions;
}

fn lower_generic_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: Vec<super::super::Step>,
    body: &module::GenericReturn,
    shape: &ConcreteValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match shape {
        ConcreteValueShape::Int => functions.int_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_int_return(body, context),
            ),
        )),
        ConcreteValueShape::Float => functions.float_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_float_return(body, context),
            ),
        )),
        ConcreteValueShape::String => functions.string_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_string_return(body, context),
            ),
        )),
        ConcreteValueShape::BitArray => functions.bit_array_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_bit_array_return(body, context),
            ),
        )),
        ConcreteValueShape::UtfCodepoint => {
            functions.utf_codepoint_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_utf_codepoint_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Custom(shape) => functions.custom_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_custom_return(body, shape, context),
            ),
        )),
        ConcreteValueShape::Bool => functions.bool_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_bool_return(body, context),
            ),
        )),
        ConcreteValueShape::Nil => functions.nil_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_nil_return(body, context),
            ),
        )),
        ConcreteValueShape::Tuple(elements) => functions.tuple_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_tuple_return(body, elements, context),
            ),
        )),
        ConcreteValueShape::List(item) => {
            lower_generic_value_list_return(
                index,
                frame_layout,
                steps,
                body,
                item,
                functions,
                context,
            );
        }
        ConcreteValueShape::Function(function) => {
            lower_generic_value_function_return(
                index,
                frame_layout,
                steps,
                body,
                function,
                functions,
                context,
            );
        }
    }
}

fn lower_generic_value_list_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: Vec<super::super::Step>,
    body: &module::GenericReturn,
    item: &ConcreteValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item {
        ConcreteValueShape::Int => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_int_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::String => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_string_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::BitArray => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_bit_array_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::UtfCodepoint => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_utf_codepoint_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Custom(shape) => {
            let type_id = context.custom_list_type(shape.to_module_shape().type_().clone());
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_custom_list_return(body, shape, type_id, context),
                ),
            ));
        }
        ConcreteValueShape::Float => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_float_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Bool => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_bool_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Nil => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_nil_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Tuple(elements) => {
            let type_id = context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            );
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_tuple_list_return(
                        body, elements, type_id, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::List(item) => {
            let type_id = context.list_list_type(item.value_type());
            let id = ListListFunctionId::new(index, type_id);
            functions.list_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_nested_list_return(body, item, type_id, context),
                ),
            ));
        }
        ConcreteValueShape::Function(function) => {
            let type_id = context.function_list_type(function.to_module_shape().type_());
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_function_list_return(
                        body, function, type_id, context,
                    ),
                ),
            ));
        }
    }
}

fn lower_generic_list_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: Vec<super::super::Step>,
    body: &module::GenericListReturn,
    item: &ConcreteValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item {
        ConcreteValueShape::Int => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_int_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::String => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_string_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::BitArray => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_bit_array_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::UtfCodepoint => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_utf_codepoint_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Custom(shape) => {
            let type_id = context.custom_list_type(shape.to_module_shape().type_().clone());
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_custom_list_return(body, shape, type_id, context),
                ),
            ));
        }
        ConcreteValueShape::Float => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_float_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Bool => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_bool_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Nil => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_nil_list_return(body, context),
                ),
            ));
        }
        ConcreteValueShape::Tuple(elements) => {
            let type_id = context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            );
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_tuple_list_return(
                        body, elements, type_id, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::List(item) => {
            let type_id = context.list_list_type(item.value_type());
            let id = ListListFunctionId::new(index, type_id);
            functions.list_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_nested_list_return(body, item, type_id, context),
                ),
            ));
        }
        ConcreteValueShape::Function(function) => {
            let type_id = context.function_list_type(function.to_module_shape().type_());
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_item_function_list_return(
                        body, function, type_id, context,
                    ),
                ),
            ));
        }
    }
}

fn lower_generic_value_function_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: Vec<super::super::Step>,
    body: &module::GenericReturn,
    function: &ConcreteFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match function.return_() {
        ConcreteValueShape::Int => functions.int_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_int_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Float => functions.float_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_float_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::String => functions.string_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_string_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::BitArray => {
            functions.bit_array_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_bit_array_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::UtfCodepoint => {
            functions.utf_codepoint_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_utf_codepoint_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::Custom(return_) => {
            functions.custom_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_custom_function_return(
                        body, function, return_, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::Bool => functions.bool_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_bool_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Nil => functions.nil_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_nil_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Tuple(_) => functions.tuple_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_value_tuple_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::List(item) => {
            let lowered =
                super::return_::generic_value_list_function_return(body, function, item, context);
            push_list_function_function(
                functions,
                index,
                item,
                ExecutableFunction::new(frame_layout, steps, lowered),
            );
        }
        ConcreteValueShape::Function(return_) => {
            functions.function_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_value_function_function_return(
                        body, function, return_, context,
                    ),
                ),
            ));
        }
    }
}

fn lower_generic_function_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: Vec<super::super::Step>,
    body: &module::GenericFunctionReturn,
    function: &ConcreteFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match function.return_() {
        ConcreteValueShape::Int => functions.int_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_int_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Float => functions.float_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_float_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::String => functions.string_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_string_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::BitArray => {
            functions.bit_array_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_result_bit_array_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::UtfCodepoint => {
            functions.utf_codepoint_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_result_utf_codepoint_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::Custom(return_) => {
            functions.custom_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_result_custom_function_return(
                        body, function, return_, context,
                    ),
                ),
            ));
        }
        ConcreteValueShape::Bool => functions.bool_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_bool_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Nil => functions.nil_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_nil_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::Tuple(_) => functions.tuple_function_functions.push((
            index,
            ExecutableFunction::new(
                frame_layout,
                steps,
                super::return_::generic_result_tuple_function_return(body, function, context),
            ),
        )),
        ConcreteValueShape::List(item) => {
            let lowered =
                super::return_::generic_result_list_function_return(body, function, item, context);
            push_list_function_function(
                functions,
                index,
                item,
                ExecutableFunction::new(frame_layout, steps, lowered),
            );
        }
        ConcreteValueShape::Function(return_) => {
            functions.function_function_functions.push((
                index,
                ExecutableFunction::new(
                    frame_layout,
                    steps,
                    super::return_::generic_result_function_function_return(
                        body, function, return_, context,
                    ),
                ),
            ));
        }
    }
}

fn push_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    item: &ConcreteValueShape,
    function: ExecutableFunction<ListFunctionReturn>,
) {
    match item {
        ConcreteValueShape::Int => functions
            .int_list_function_functions
            .push((index, function)),
        ConcreteValueShape::String => {
            functions
                .string_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::BitArray => {
            functions
                .bit_array_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::UtfCodepoint => {
            functions
                .utf_codepoint_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Custom(_) => {
            functions
                .custom_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Float => {
            functions
                .float_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Bool => {
            functions
                .bool_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Nil => {
            functions
                .nil_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Tuple(_) => {
            functions
                .tuple_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::List(_) => {
            functions
                .list_list_function_functions
                .push((index, function));
        }
        ConcreteValueShape::Function(_) => {
            functions
                .function_list_function_functions
                .push((index, function));
        }
    }
}

pub(super) fn function_id(
    shape: &ConcreteFunctionShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> RuntimeFunctionId {
    match shape.return_() {
        ConcreteValueShape::Int => RuntimeFunctionId::Int(IntFunctionId(index)),
        ConcreteValueShape::Float => RuntimeFunctionId::Float(FloatFunctionId(index)),
        ConcreteValueShape::String => RuntimeFunctionId::String(StringFunctionId(index)),
        ConcreteValueShape::BitArray => RuntimeFunctionId::BitArray(BitArrayFunctionId(index)),
        ConcreteValueShape::UtfCodepoint => {
            RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(index))
        }
        ConcreteValueShape::Custom(shape) => RuntimeFunctionId::Custom(
            super::super::CustomFunctionId::new(index, types.custom_value_shape(shape)),
        ),
        ConcreteValueShape::Bool => RuntimeFunctionId::Bool(BoolFunctionId(index)),
        ConcreteValueShape::Nil => RuntimeFunctionId::Nil(NilFunctionId(index)),
        ConcreteValueShape::Tuple(elements) => RuntimeFunctionId::Tuple {
            id: TupleFunctionId(index),
            return_type: elements
                .iter()
                .map(|shape| types.value_type(shape))
                .collect(),
        },
        ConcreteValueShape::List(item) => {
            RuntimeFunctionId::List(list_function_id(item, index, types))
        }
        ConcreteValueShape::Function(function) => RuntimeFunctionId::Function {
            id: function_function_id(function, index, types),
            return_type: types.function_type(function),
        },
    }
}

pub(super) fn function_table_family(shape: &ConcreteValueShape) -> FunctionTableFamily {
    match shape {
        ConcreteValueShape::Int => FunctionTableFamily::Int,
        ConcreteValueShape::Float => FunctionTableFamily::Float,
        ConcreteValueShape::String => FunctionTableFamily::String,
        ConcreteValueShape::BitArray => FunctionTableFamily::BitArray,
        ConcreteValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepoint,
        ConcreteValueShape::Custom(_) => FunctionTableFamily::Custom,
        ConcreteValueShape::Bool => FunctionTableFamily::Bool,
        ConcreteValueShape::Nil => FunctionTableFamily::Nil,
        ConcreteValueShape::Tuple(_) => FunctionTableFamily::Tuple,
        ConcreteValueShape::List(item) => list_function_table_family(item),
        ConcreteValueShape::Function(function) => {
            function_function_table_family(function.return_())
        }
    }
}

pub(super) fn list_function_table_family(item: &ConcreteValueShape) -> FunctionTableFamily {
    match item {
        ConcreteValueShape::Int => FunctionTableFamily::IntList,
        ConcreteValueShape::String => FunctionTableFamily::StringList,
        ConcreteValueShape::BitArray => FunctionTableFamily::BitArrayList,
        ConcreteValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointList,
        ConcreteValueShape::Custom(_) => FunctionTableFamily::CustomList,
        ConcreteValueShape::Float => FunctionTableFamily::FloatList,
        ConcreteValueShape::Bool => FunctionTableFamily::BoolList,
        ConcreteValueShape::Nil => FunctionTableFamily::NilList,
        ConcreteValueShape::Tuple(_) => FunctionTableFamily::TupleList,
        ConcreteValueShape::List(_) => FunctionTableFamily::ListList,
        ConcreteValueShape::Function(_) => FunctionTableFamily::FunctionList,
    }
}

pub(super) fn function_function_table_family(return_: &ConcreteValueShape) -> FunctionTableFamily {
    match return_ {
        ConcreteValueShape::Int => FunctionTableFamily::IntFunction,
        ConcreteValueShape::Float => FunctionTableFamily::FloatFunction,
        ConcreteValueShape::String => FunctionTableFamily::StringFunction,
        ConcreteValueShape::BitArray => FunctionTableFamily::BitArrayFunction,
        ConcreteValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointFunction,
        ConcreteValueShape::Custom(_) => FunctionTableFamily::CustomFunction,
        ConcreteValueShape::Bool => FunctionTableFamily::BoolFunction,
        ConcreteValueShape::Nil => FunctionTableFamily::NilFunction,
        ConcreteValueShape::Tuple(_) => FunctionTableFamily::TupleFunction,
        ConcreteValueShape::List(item) => match item.as_ref() {
            ConcreteValueShape::Int => FunctionTableFamily::IntListFunction,
            ConcreteValueShape::String => FunctionTableFamily::StringListFunction,
            ConcreteValueShape::BitArray => FunctionTableFamily::BitArrayListFunction,
            ConcreteValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointListFunction,
            ConcreteValueShape::Custom(_) => FunctionTableFamily::CustomListFunction,
            ConcreteValueShape::Float => FunctionTableFamily::FloatListFunction,
            ConcreteValueShape::Bool => FunctionTableFamily::BoolListFunction,
            ConcreteValueShape::Nil => FunctionTableFamily::NilListFunction,
            ConcreteValueShape::Tuple(_) => FunctionTableFamily::TupleListFunction,
            ConcreteValueShape::List(_) => FunctionTableFamily::ListListFunction,
            ConcreteValueShape::Function(_) => FunctionTableFamily::FunctionListFunction,
        },
        ConcreteValueShape::Function(_) => FunctionTableFamily::FunctionFunction,
    }
}

pub(super) fn list_function_id(
    item: &ConcreteValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionId {
    match item {
        ConcreteValueShape::Int => {
            ListFunctionId::Int(IntListFunctionId::new(index, types.int_list_type()))
        }
        ConcreteValueShape::String => {
            ListFunctionId::String(StringListFunctionId::new(index, types.string_list_type()))
        }
        ConcreteValueShape::BitArray => ListFunctionId::BitArray(BitArrayListFunctionId::new(
            index,
            types.bit_array_list_type(),
        )),
        ConcreteValueShape::UtfCodepoint => ListFunctionId::UtfCodepoint(
            UtfCodepointListFunctionId::new(index, types.utf_codepoint_list_type()),
        ),
        ConcreteValueShape::Custom(item) => ListFunctionId::Custom(CustomListFunctionId::new(
            index,
            types.custom_list_type(item),
        )),
        ConcreteValueShape::Float => {
            ListFunctionId::Float(FloatListFunctionId::new(index, types.float_list_type()))
        }
        ConcreteValueShape::Bool => {
            ListFunctionId::Bool(BoolListFunctionId::new(index, types.bool_list_type()))
        }
        ConcreteValueShape::Nil => {
            ListFunctionId::Nil(NilListFunctionId::new(index, types.nil_list_type()))
        }
        ConcreteValueShape::Tuple(item) => {
            ListFunctionId::Tuple(TupleListFunctionId::new(index, types.tuple_list_type(item)))
        }
        ConcreteValueShape::List(item) => {
            ListFunctionId::List(ListListFunctionId::new(index, types.list_list_type(item)))
        }
        ConcreteValueShape::Function(item) => ListFunctionId::Function(
            FunctionListFunctionId::new(index, types.function_list_type(item)),
        ),
    }
}

pub(super) fn function_function_id(
    function: &ConcreteFunctionShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> FunctionFunctionId {
    use super::super as execution;

    match function.return_() {
        ConcreteValueShape::Int => FunctionFunctionId::Int(IntFunctionFunctionId(index)),
        ConcreteValueShape::Float => FunctionFunctionId::Float(FloatFunctionFunctionId(index)),
        ConcreteValueShape::String => FunctionFunctionId::String(StringFunctionFunctionId(index)),
        ConcreteValueShape::BitArray => {
            FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(index))
        }
        ConcreteValueShape::UtfCodepoint => {
            FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(index))
        }
        ConcreteValueShape::Custom(return_) => {
            FunctionFunctionId::Custom(execution::CustomFunctionFunctionId::new(
                index,
                types.custom_function_type(function.arguments(), return_),
            ))
        }
        ConcreteValueShape::Bool => FunctionFunctionId::Bool(BoolFunctionFunctionId(index)),
        ConcreteValueShape::Nil => FunctionFunctionId::Nil(NilFunctionFunctionId(index)),
        ConcreteValueShape::Tuple(_) => FunctionFunctionId::Tuple(TupleFunctionFunctionId(index)),
        ConcreteValueShape::List(item) => {
            FunctionFunctionId::List(list_function_function_id(function, item, index, types))
        }
        ConcreteValueShape::Function(return_) => {
            FunctionFunctionId::Function(execution::FunctionFunctionFunctionId::new(
                index,
                types.function_function_type(function.arguments(), return_),
            ))
        }
    }
}

pub(super) fn list_function_function_id(
    function: &ConcreteFunctionShape,
    item: &ConcreteValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionFunctionId {
    let type_ = types.function_type(function);

    match item {
        ConcreteValueShape::Int => ListFunctionFunctionId::Int {
            id: super::super::IntListFunctionFunctionId(index),
            type_,
            list_type: types.int_list_type(),
        },
        ConcreteValueShape::String => ListFunctionFunctionId::String {
            id: super::super::StringListFunctionFunctionId(index),
            type_,
            list_type: types.string_list_type(),
        },
        ConcreteValueShape::BitArray => ListFunctionFunctionId::BitArray {
            id: super::super::BitArrayListFunctionFunctionId(index),
            type_,
            list_type: types.bit_array_list_type(),
        },
        ConcreteValueShape::UtfCodepoint => ListFunctionFunctionId::UtfCodepoint {
            id: super::super::UtfCodepointListFunctionFunctionId(index),
            type_,
            list_type: types.utf_codepoint_list_type(),
        },
        ConcreteValueShape::Custom(item) => ListFunctionFunctionId::Custom {
            id: super::super::CustomListFunctionFunctionId(index),
            type_,
            list_type: types.custom_list_type(item),
        },
        ConcreteValueShape::Float => ListFunctionFunctionId::Float {
            id: super::super::FloatListFunctionFunctionId(index),
            type_,
            list_type: types.float_list_type(),
        },
        ConcreteValueShape::Bool => ListFunctionFunctionId::Bool {
            id: super::super::BoolListFunctionFunctionId(index),
            type_,
            list_type: types.bool_list_type(),
        },
        ConcreteValueShape::Nil => ListFunctionFunctionId::Nil {
            id: super::super::NilListFunctionFunctionId(index),
            type_,
            list_type: types.nil_list_type(),
        },
        ConcreteValueShape::Tuple(item) => ListFunctionFunctionId::Tuple {
            id: super::super::TupleListFunctionFunctionId(index),
            type_,
            list_type: types.tuple_list_type(item),
        },
        ConcreteValueShape::List(item) => ListFunctionFunctionId::List {
            id: super::super::ListListFunctionFunctionId(index),
            type_,
            list_type: types.list_list_type(item),
        },
        ConcreteValueShape::Function(item) => ListFunctionFunctionId::Function {
            id: super::super::FunctionListFunctionFunctionId(index),
            type_,
            list_type: types.function_list_type(item),
        },
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
