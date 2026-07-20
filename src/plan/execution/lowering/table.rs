use super::super::function::ExecutableFunction;
use super::super::table::FunctionTables;
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionReturn, BitArrayListFunctionId,
    BitArrayListReturn, BitArrayReturn, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn,
    BoolListFunctionId, BoolListReturn, BoolReturn, CustomFunctionReturn, CustomListFunctionId,
    CustomListReturn, CustomReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn,
    FloatListFunctionId, FloatListReturn, FloatReturn, FunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, GenericFunctionReturn, IntFunctionFunctionId,
    IntFunctionId, IntFunctionReturn, IntListFunctionId, IntListReturn, IntReturn,
    ListFunctionFunctionId, ListFunctionId, ListFunctionReturn, ListListFunctionId, ListListReturn,
    NeverFunctionReturn, NeverReturn, NilFunctionFunctionId, NilFunctionId, NilFunctionReturn,
    NilListFunctionId, NilListReturn, NilReturn, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListReturn, ParameterListReturn, RuntimeFunctionId,
    StringFunctionFunctionId, StringFunctionId, StringFunctionReturn, StringListFunctionId,
    StringListReturn, StringReturn, TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn,
    TupleListFunctionId, TupleListReturn, TupleReturn, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointFunctionReturn, UtfCodepointListFunctionId,
    UtfCodepointListReturn, UtfCodepointReturn,
};
use super::LoweringContext;
use super::SpecializationOutcome;
use super::specialization::{
    FunctionRepresentation, Representability, SpecializationKey, SpecializedFunctionShape,
    SpecializedValueShape, StoredValueShape,
};
use crate::plan::module;
use std::collections::HashSet;

struct LoweredFunction<Return> {
    specialization: SpecializationKey,
    function: Representability<ExecutableFunction<Return>>,
}

struct LoweredSteps {
    specialization: SpecializationKey,
    steps: Representability<Vec<super::super::Step>>,
}

fn lowered_function<Return>(
    frame_layout: super::super::FrameLayout,
    steps: LoweredSteps,
    return_: Representability<Return>,
) -> LoweredFunction<Return> {
    LoweredFunction {
        specialization: steps.specialization,
        function: steps.steps.zip_with(return_, |steps, return_| {
            ExecutableFunction::new(frame_layout, steps, return_)
        }),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum FunctionTableFamily {
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    ParameterList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ParameterListList,
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
    GenericFunction,
    NeverFunction,
    ParameterListFunction,
    ParameterListListFunction,
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
    never_functions: Vec<(usize, LoweredFunction<NeverReturn>)>,
    int_functions: Vec<(usize, LoweredFunction<IntReturn>)>,
    float_functions: Vec<(usize, LoweredFunction<FloatReturn>)>,
    string_functions: Vec<(usize, LoweredFunction<StringReturn>)>,
    bit_array_functions: Vec<(usize, LoweredFunction<BitArrayReturn>)>,
    utf_codepoint_functions: Vec<(usize, LoweredFunction<UtfCodepointReturn>)>,
    custom_functions: Vec<(usize, LoweredFunction<CustomReturn>)>,
    bool_functions: Vec<(usize, LoweredFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, LoweredFunction<NilReturn>)>,
    tuple_functions: Vec<(usize, LoweredFunction<TupleReturn>)>,
    parameter_list_functions: Vec<(
        ParameterListFunctionId,
        LoweredFunction<ParameterListReturn>,
    )>,
    int_list_functions: Vec<(IntListFunctionId, LoweredFunction<IntListReturn>)>,
    string_list_functions: Vec<(StringListFunctionId, LoweredFunction<StringListReturn>)>,
    bit_array_list_functions: Vec<(BitArrayListFunctionId, LoweredFunction<BitArrayListReturn>)>,
    utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        LoweredFunction<UtfCodepointListReturn>,
    )>,
    custom_list_functions: Vec<(CustomListFunctionId, LoweredFunction<CustomListReturn>)>,
    float_list_functions: Vec<(FloatListFunctionId, LoweredFunction<FloatListReturn>)>,
    bool_list_functions: Vec<(BoolListFunctionId, LoweredFunction<BoolListReturn>)>,
    nil_list_functions: Vec<(NilListFunctionId, LoweredFunction<NilListReturn>)>,
    tuple_list_functions: Vec<(TupleListFunctionId, LoweredFunction<TupleListReturn>)>,
    parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        LoweredFunction<ParameterListListReturn>,
    )>,
    list_list_functions: Vec<(ListListFunctionId, LoweredFunction<ListListReturn>)>,
    function_list_functions: Vec<(FunctionListFunctionId, LoweredFunction<FunctionListReturn>)>,
    int_function_functions: Vec<(usize, LoweredFunction<IntFunctionReturn>)>,
    float_function_functions: Vec<(usize, LoweredFunction<FloatFunctionReturn>)>,
    string_function_functions: Vec<(usize, LoweredFunction<StringFunctionReturn>)>,
    bit_array_function_functions: Vec<(usize, LoweredFunction<BitArrayFunctionReturn>)>,
    utf_codepoint_function_functions: Vec<(usize, LoweredFunction<UtfCodepointFunctionReturn>)>,
    custom_function_functions: Vec<(usize, LoweredFunction<CustomFunctionReturn>)>,
    bool_function_functions: Vec<(usize, LoweredFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, LoweredFunction<NilFunctionReturn>)>,
    tuple_function_functions: Vec<(usize, LoweredFunction<TupleFunctionReturn>)>,
    generic_function_functions: Vec<(usize, LoweredFunction<GenericFunctionReturn>)>,
    never_function_functions: Vec<(usize, LoweredFunction<NeverFunctionReturn>)>,
    parameter_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    parameter_list_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    int_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    string_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    bit_array_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    utf_codepoint_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    custom_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    float_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    bool_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    nil_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    tuple_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    list_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    function_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    function_function_functions: Vec<(usize, LoweredFunction<FunctionFunctionReturn>)>,
}

impl FunctionTableBuilder {
    pub(super) fn finish(self) -> SpecializationOutcome<Box<FunctionTables>> {
        let mut erased = HashSet::new();
        let tables = FunctionTables {
            never_functions: sort_functions(self.never_functions, &mut erased),
            int_functions: sort_functions(self.int_functions, &mut erased),
            float_functions: sort_functions(self.float_functions, &mut erased),
            string_functions: sort_functions(self.string_functions, &mut erased),
            bit_array_functions: sort_functions(self.bit_array_functions, &mut erased),
            utf_codepoint_functions: sort_functions(self.utf_codepoint_functions, &mut erased),
            custom_functions: sort_functions(self.custom_functions, &mut erased),
            bool_functions: sort_functions(self.bool_functions, &mut erased),
            nil_functions: sort_functions(self.nil_functions, &mut erased),
            tuple_functions: sort_functions(self.tuple_functions, &mut erased),
            parameter_list_functions: sort_list_functions(
                self.parameter_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            int_list_functions: sort_list_functions(
                self.int_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            string_list_functions: sort_list_functions(
                self.string_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            bit_array_list_functions: sort_list_functions(
                self.bit_array_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            utf_codepoint_list_functions: sort_list_functions(
                self.utf_codepoint_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            custom_list_functions: sort_list_functions(
                self.custom_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            float_list_functions: sort_list_functions(
                self.float_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            bool_list_functions: sort_list_functions(
                self.bool_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            nil_list_functions: sort_list_functions(
                self.nil_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            tuple_list_functions: sort_list_functions(
                self.tuple_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            parameter_list_list_functions: sort_list_functions(
                self.parameter_list_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            list_list_functions: sort_list_functions(
                self.list_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            function_list_functions: sort_list_functions(
                self.function_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            int_function_functions: sort_functions(self.int_function_functions, &mut erased),
            float_function_functions: sort_functions(self.float_function_functions, &mut erased),
            string_function_functions: sort_functions(self.string_function_functions, &mut erased),
            bit_array_function_functions: sort_functions(
                self.bit_array_function_functions,
                &mut erased,
            ),
            utf_codepoint_function_functions: sort_functions(
                self.utf_codepoint_function_functions,
                &mut erased,
            ),
            custom_function_functions: sort_functions(self.custom_function_functions, &mut erased),
            bool_function_functions: sort_functions(self.bool_function_functions, &mut erased),
            nil_function_functions: sort_functions(self.nil_function_functions, &mut erased),
            tuple_function_functions: sort_functions(self.tuple_function_functions, &mut erased),
            generic_function_functions: sort_functions(
                self.generic_function_functions,
                &mut erased,
            ),
            never_function_functions: sort_functions(self.never_function_functions, &mut erased),
            parameter_list_function_functions: sort_functions(
                self.parameter_list_function_functions,
                &mut erased,
            ),
            parameter_list_list_function_functions: sort_functions(
                self.parameter_list_list_function_functions,
                &mut erased,
            ),
            int_list_function_functions: sort_functions(
                self.int_list_function_functions,
                &mut erased,
            ),
            string_list_function_functions: sort_functions(
                self.string_list_function_functions,
                &mut erased,
            ),
            bit_array_list_function_functions: sort_functions(
                self.bit_array_list_function_functions,
                &mut erased,
            ),
            utf_codepoint_list_function_functions: sort_functions(
                self.utf_codepoint_list_function_functions,
                &mut erased,
            ),
            custom_list_function_functions: sort_functions(
                self.custom_list_function_functions,
                &mut erased,
            ),
            float_list_function_functions: sort_functions(
                self.float_list_function_functions,
                &mut erased,
            ),
            bool_list_function_functions: sort_functions(
                self.bool_list_function_functions,
                &mut erased,
            ),
            nil_list_function_functions: sort_functions(
                self.nil_list_function_functions,
                &mut erased,
            ),
            tuple_list_function_functions: sort_functions(
                self.tuple_list_function_functions,
                &mut erased,
            ),
            list_list_function_functions: sort_functions(
                self.list_list_function_functions,
                &mut erased,
            ),
            function_list_function_functions: sort_functions(
                self.function_list_function_functions,
                &mut erased,
            ),
            function_function_functions: sort_functions(
                self.function_function_functions,
                &mut erased,
            ),
        };
        SpecializationOutcome::complete_unless_erased(Box::new(tables), erased)
    }
}

pub(super) fn lower_specialized(
    template: &module::FunctionTemplate,
    key: &super::specialization::SpecializationKey,
    context: &mut LoweringContext,
) {
    let frame_layout = super::frame::frame_layout(context);
    let steps = LoweredSteps {
        specialization: key.clone(),
        steps: super::step::steps_until_never(template.steps(), context).map(|steps| match steps {
            super::step::StepsUntilNever::Complete(steps) => steps,
            super::step::StepsUntilNever::Diverging { prefix, expression } => {
                context.set_return_divergence(expression);
                prefix
            }
        }),
    };
    let index = context.specialization_index(key);
    let return_shape = SpecializedValueShape::instantiate(
        template.signature().shape().return_shape(),
        key.substitution(),
    );
    let return_inhabitation = context.representations.inhabitation(&return_shape);
    let mut functions = std::mem::take(&mut context.functions);

    match template.return_().kind() {
        module::ReturnExprKind::Generic { parameter: _, body } => match &return_inhabitation {
            super::specialization::ValueInhabitation::Uninhabited(_) => {
                functions.never_functions.push((
                    index,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::never_return(body, context),
                    ),
                ));
            }
            super::specialization::ValueInhabitation::Inhabited(shape) => lower_generic_return(
                index,
                frame_layout,
                steps,
                body,
                shape,
                &mut functions,
                context,
            ),
        },
        module::ReturnExprKind::Int { body } => functions.int_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::int_return(body, context),
            ),
        )),
        module::ReturnExprKind::Float { body } => functions.float_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::float_return(body, context),
            ),
        )),
        module::ReturnExprKind::String { body } => functions.string_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::string_return(body, context),
            ),
        )),
        module::ReturnExprKind::BitArray { body } => {
            functions.bit_array_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::bit_array_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::UtfCodepoint { body } => {
            functions.utf_codepoint_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::utf_codepoint_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::Custom { body } => {
            let signature_shape = context.concrete_custom_value_shape(body.signature_shape());
            match context
                .representations
                .custom_inhabitation(&signature_shape)
            {
                super::specialization::CompoundInhabitation::Inhabited => {
                    functions.custom_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::custom_return(body, context),
                        ),
                    ));
                }
                super::specialization::CompoundInhabitation::Uninhabited(proof) => {
                    functions.never_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::custom_never_return(body, &proof, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::Bool { body } => functions.bool_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::bool_return(body, context),
            ),
        )),
        module::ReturnExprKind::Nil { body } => functions.nil_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::nil_return(body, context),
            ),
        )),
        module::ReturnExprKind::Tuple { type_, body } => {
            let elements = type_
                .iter()
                .cloned()
                .map(crate::plan::ValueShape::from_value_type)
                .map(|shape| context.concrete_value_shape(&shape))
                .collect::<Vec<_>>();
            match context.representations.tuple_inhabitation(&elements) {
                super::specialization::CompoundInhabitation::Inhabited => {
                    functions.tuple_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::tuple_return(body, context),
                        ),
                    ));
                }
                super::specialization::CompoundInhabitation::Uninhabited(proof) => {
                    functions.never_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::tuple_never_return(body, &proof, context),
                        ),
                    ));
                }
            }
        }
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
        module::ReturnExprKind::ParameterListList { parameter, body } => {
            let item = context.concrete_parameter(*parameter);
            match item.storage_representation() {
                super::specialization::StorageRepresentation::Parameter(parameter) => {
                    let type_id = context.parameter_list_list_type(parameter);
                    let id = ParameterListListFunctionId::new(index, type_id);
                    functions.parameter_list_list_functions.push((
                        id,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::parameter_list_list_return(
                                body, parameter, type_id, context,
                            ),
                        ),
                    ));
                }
                super::specialization::StorageRepresentation::Stored(item) => {
                    let type_id = context.specialized_stored_list_list_type(&item);
                    let id = ListListFunctionId::new(index, type_id);
                    functions.list_list_functions.push((
                        id,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::stored_parameter_list_list_return(
                                body, &item, type_id, context,
                            ),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::IntList { body } => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::int_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::StringList { body } => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::string_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::BitArrayList { body } => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::bit_array_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::UtfCodepointList { body } => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::utf_codepoint_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::CustomList { item_type, body } => {
            let type_id = context.custom_list_type(item_type.clone());
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::custom_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::FloatList { body } => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::float_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::BoolList { body } => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::bool_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::NilList { body } => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::nil_list_return(body, context),
                ),
            ));
        }
        module::ReturnExprKind::TupleList { item_type, body } => {
            let type_id = context.tuple_list_type(item_type.clone());
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::tuple_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::ListList { item_shape, body } => {
            let type_id = context.stored_list_list_type(item_shape);
            let id = ListListFunctionId::new(index, type_id);
            functions.list_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::list_list_return(body, type_id, context),
                ),
            ));
        }
        module::ReturnExprKind::FunctionList { item_type, body } => {
            let type_id = context.function_list_type(item_type.clone());
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::function_list_return(body, type_id, context),
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
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_int_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.int_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::int_function_return(shape, body, context),
                        ),
                    ))
                }
            }
        }
        module::ReturnExprKind::FloatFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_float_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.float_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::float_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::StringFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_string_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.string_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::string_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::BitArrayFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_bit_array_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.bit_array_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::bit_array_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::UtfCodepointFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_utf_codepoint_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.utf_codepoint_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::utf_codepoint_function_return(
                                shape, body, context,
                            ),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::CustomFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(&crate::plan::FunctionShape::new(
                body.type_().argument_shapes().to_vec(),
                crate::plan::ValueShape::Custom(body.type_().return_().clone()),
            ));
            match context.function_representation(&function_shape) {
                FunctionRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_custom_function_return(
                                body,
                                &function_shape,
                                context,
                            ),
                        ),
                    ));
                }
                FunctionRepresentation::Never(_) => {
                    functions.never_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::custom_never_function_return(
                                body,
                                &function_shape,
                                context,
                            ),
                        ),
                    ));
                }
                FunctionRepresentation::Executable(_) => {
                    functions.custom_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::custom_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::BoolFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_bool_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.bool_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::bool_function_return(shape, body, context),
                        ),
                    ))
                }
            }
        }
        module::ReturnExprKind::NilFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_return(
                                body,
                                &function_shape,
                                context,
                                super::expression::symbolic_nil_function_expr,
                            ),
                        ),
                    ))
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.nil_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::nil_function_return(shape, body, context),
                        ),
                    ))
                }
            }
        }
        module::ReturnExprKind::TupleFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_representation(&function_shape) {
                FunctionRepresentation::Symbolic => functions.generic_function_functions.push((
                    index,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::symbolic_function_return(
                            body,
                            &function_shape,
                            context,
                            super::expression::symbolic_tuple_function_expr,
                        ),
                    ),
                )),
                FunctionRepresentation::Never(_) => {
                    functions.never_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::tuple_never_function_return(
                                body,
                                &function_shape,
                                context,
                            ),
                        ),
                    ));
                }
                FunctionRepresentation::Executable(_) => {
                    functions.tuple_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::tuple_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
        module::ReturnExprKind::ListFunction {
            shape,
            item_type,
            body,
        } => {
            let function_shape = context.concrete_function_shape(shape);
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_list_function_return(
                                body,
                                &function_shape,
                                context,
                            ),
                        ),
                    ));
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    let item = context.concrete_value_shape(
                        &crate::plan::ValueShape::from_value_type(item_type.clone()),
                    );
                    let lowered =
                        super::return_graph::list_function_return(shape, body, &item, context);
                    push_list_function_function(
                        &mut functions,
                        index,
                        &item,
                        lowered_function(frame_layout, steps, lowered),
                    );
                }
            }
        }
        module::ReturnExprKind::FunctionFunction { shape, body } => {
            let function_shape = context.concrete_function_shape(&crate::plan::FunctionShape::new(
                body.type_().argument_shapes().to_vec(),
                crate::plan::ValueShape::Function(Box::new(body.type_().return_shape().clone())),
            ));
            match context.function_arguments_representation(&function_shape) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    functions.generic_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::symbolic_function_function_return(
                                body,
                                &function_shape,
                                context,
                            ),
                        ),
                    ));
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {
                    functions.function_function_functions.push((
                        index,
                        lowered_function(
                            frame_layout,
                            steps,
                            super::return_graph::function_function_return(shape, body, context),
                        ),
                    ));
                }
            }
        }
    }

    context.functions = functions;
}

fn lower_generic_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: LoweredSteps,
    body: &module::GenericReturn,
    shape: &StoredValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match shape {
        StoredValueShape::Int => functions.int_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_int_return(body, context),
            ),
        )),
        StoredValueShape::Float => functions.float_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_float_return(body, context),
            ),
        )),
        StoredValueShape::String => functions.string_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_string_return(body, context),
            ),
        )),
        StoredValueShape::BitArray => functions.bit_array_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_bit_array_return(body, context),
            ),
        )),
        StoredValueShape::UtfCodepoint => {
            functions.utf_codepoint_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_utf_codepoint_return(body, context),
                ),
            ));
        }
        StoredValueShape::Custom(shape) => functions.custom_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_custom_return(body, shape, context),
            ),
        )),
        StoredValueShape::Bool => functions.bool_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_bool_return(body, context),
            ),
        )),
        StoredValueShape::Nil => functions.nil_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_nil_return(body, context),
            ),
        )),
        StoredValueShape::Tuple(elements) => functions.tuple_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_tuple_return(body, elements, context),
            ),
        )),
        StoredValueShape::List(item) => {
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
        StoredValueShape::Function(function) => {
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
    steps: LoweredSteps,
    body: &module::GenericReturn,
    item: &SpecializedValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item {
        SpecializedValueShape::Parameter(parameter) => {
            let type_id = context.parameter_list_type(*parameter);
            let id = ParameterListFunctionId::new(index, type_id);
            functions.parameter_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_parameter_list_return(
                        body, *parameter, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::Int => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_int_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::String => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_string_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::BitArray => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_bit_array_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::UtfCodepoint => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_utf_codepoint_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Custom(shape) => {
            let type_id = context.specialized_custom_list_type(shape);
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_custom_list_return(
                        body, shape, type_id, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::Float => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_float_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Bool => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_bool_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Nil => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_nil_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Tuple(elements) => {
            let type_id = context.specialized_tuple_list_type(elements);
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_tuple_list_return(
                        body, elements, type_id, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::List(item) => match item.storage_representation() {
            super::specialization::StorageRepresentation::Parameter(parameter) => {
                let type_id = context.parameter_list_list_type(parameter);
                let id = ParameterListListFunctionId::new(index, type_id);
                functions.parameter_list_list_functions.push((
                    id,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::generic_value_parameter_list_list_return(
                            body, parameter, type_id, context,
                        ),
                    ),
                ));
            }
            super::specialization::StorageRepresentation::Stored(item) => {
                let type_id = context.specialized_stored_list_list_type(&item);
                let id = ListListFunctionId::new(index, type_id);
                functions.list_list_functions.push((
                    id,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::generic_value_nested_list_return(
                            body, &item, type_id, context,
                        ),
                    ),
                ));
            }
        },
        SpecializedValueShape::Function(function) => {
            let type_id = context.specialized_function_list_type(function);
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_function_list_return(
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
    steps: LoweredSteps,
    body: &module::GenericListReturn,
    item: &SpecializedValueShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match item {
        SpecializedValueShape::Parameter(parameter) => {
            let type_id = context.parameter_list_type(*parameter);
            let id = ParameterListFunctionId::new(index, type_id);
            functions.parameter_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_parameter_list_return(
                        body, *parameter, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::Int => {
            let id = IntListFunctionId::new(index, context.int_list_type());
            functions.int_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_int_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::String => {
            let id = StringListFunctionId::new(index, context.string_list_type());
            functions.string_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_string_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::BitArray => {
            let id = BitArrayListFunctionId::new(index, context.bit_array_list_type());
            functions.bit_array_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_bit_array_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::UtfCodepoint => {
            let id = UtfCodepointListFunctionId::new(index, context.utf_codepoint_list_type());
            functions.utf_codepoint_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_utf_codepoint_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Custom(shape) => {
            let type_id = context.specialized_custom_list_type(shape);
            let id = CustomListFunctionId::new(index, type_id);
            functions.custom_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_custom_list_return(
                        body, shape, type_id, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::Float => {
            let id = FloatListFunctionId::new(index, context.float_list_type());
            functions.float_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_float_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Bool => {
            let id = BoolListFunctionId::new(index, context.bool_list_type());
            functions.bool_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_bool_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Nil => {
            let id = NilListFunctionId::new(index, context.nil_list_type());
            functions.nil_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_nil_list_return(body, context),
                ),
            ));
        }
        SpecializedValueShape::Tuple(elements) => {
            let type_id = context.specialized_tuple_list_type(elements);
            let id = TupleListFunctionId::new(index, type_id);
            functions.tuple_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_tuple_list_return(
                        body, elements, type_id, context,
                    ),
                ),
            ));
        }
        SpecializedValueShape::List(item) => match item.storage_representation() {
            super::specialization::StorageRepresentation::Parameter(parameter) => {
                let type_id = context.parameter_list_list_type(parameter);
                let id = ParameterListListFunctionId::new(index, type_id);
                functions.parameter_list_list_functions.push((
                    id,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::generic_item_parameter_list_list_return(
                            body, parameter, type_id, context,
                        ),
                    ),
                ));
            }
            super::specialization::StorageRepresentation::Stored(item) => {
                let type_id = context.specialized_stored_list_list_type(&item);
                let id = ListListFunctionId::new(index, type_id);
                functions.list_list_functions.push((
                    id,
                    lowered_function(
                        frame_layout,
                        steps,
                        super::return_graph::generic_item_nested_list_return(
                            body, &item, type_id, context,
                        ),
                    ),
                ));
            }
        },
        SpecializedValueShape::Function(function) => {
            let type_id = context.specialized_function_list_type(function);
            let id = FunctionListFunctionId::new(index, type_id);
            functions.function_list_functions.push((
                id,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_item_function_list_return(
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
    steps: LoweredSteps,
    body: &module::GenericReturn,
    function: &SpecializedFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match context.function_representation(function) {
        FunctionRepresentation::Symbolic => functions.generic_function_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_value_generic_function_return(body, function, context),
            ),
        )),
        FunctionRepresentation::Never(_) => functions.never_function_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_value_never_function_return(body, function, context),
            ),
        )),
        FunctionRepresentation::Executable(StoredValueShape::Int) => {
            functions.int_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_int_function_return(body, function, context),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Float) => {
            functions.float_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_float_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::String) => {
            functions.string_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_string_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::BitArray) => {
            functions.bit_array_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_bit_array_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::UtfCodepoint) => {
            functions.utf_codepoint_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_utf_codepoint_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::Custom(return_)) => {
            functions.custom_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_custom_function_return(
                        body, function, &return_, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::Bool) => {
            functions.bool_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_bool_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Nil) => {
            functions.nil_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_nil_function_return(body, function, context),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Tuple(_)) => {
            functions.tuple_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_tuple_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            let lowered = super::return_graph::generic_value_list_function_return(
                body, function, &item, context,
            );
            push_list_function_function(
                functions,
                index,
                &item,
                lowered_function(frame_layout, steps, lowered),
            );
        }
        FunctionRepresentation::Executable(StoredValueShape::Function(return_)) => {
            functions.function_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_value_function_function_return(
                        body, function, &return_, context,
                    ),
                ),
            ));
        }
    }
}

fn lower_generic_function_return(
    index: usize,
    frame_layout: super::super::FrameLayout,
    steps: LoweredSteps,
    body: &module::GenericFunctionReturn,
    function: &SpecializedFunctionShape,
    functions: &mut FunctionTableBuilder,
    context: &mut LoweringContext,
) {
    match context.function_representation(function) {
        FunctionRepresentation::Symbolic => functions.generic_function_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_result_generic_function_return(
                    body, function, context,
                ),
            ),
        )),
        FunctionRepresentation::Never(_) => functions.never_function_functions.push((
            index,
            lowered_function(
                frame_layout,
                steps,
                super::return_graph::generic_result_never_function_return(body, function, context),
            ),
        )),
        FunctionRepresentation::Executable(StoredValueShape::Int) => {
            functions.int_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_int_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Float) => {
            functions.float_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_float_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::String) => {
            functions.string_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_string_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::BitArray) => {
            functions.bit_array_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_bit_array_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::UtfCodepoint) => {
            functions.utf_codepoint_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_utf_codepoint_function_return(
                        body, function, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::Custom(return_)) => {
            functions.custom_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_custom_function_return(
                        body, function, &return_, context,
                    ),
                ),
            ));
        }
        FunctionRepresentation::Executable(StoredValueShape::Bool) => {
            functions.bool_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_bool_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Nil) => {
            functions.nil_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_nil_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Tuple(_)) => {
            functions.tuple_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_tuple_function_return(
                        body, function, context,
                    ),
                ),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            let lowered = super::return_graph::generic_result_list_function_return(
                body, function, &item, context,
            );
            push_list_function_function(
                functions,
                index,
                &item,
                lowered_function(frame_layout, steps, lowered),
            );
        }
        FunctionRepresentation::Executable(StoredValueShape::Function(return_)) => {
            functions.function_function_functions.push((
                index,
                lowered_function(
                    frame_layout,
                    steps,
                    super::return_graph::generic_result_function_function_return(
                        body, function, &return_, context,
                    ),
                ),
            ));
        }
    }
}

fn push_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    item: &SpecializedValueShape,
    function: LoweredFunction<ListFunctionReturn>,
) {
    match item {
        SpecializedValueShape::Parameter(_) => functions
            .parameter_list_function_functions
            .push((index, function)),
        SpecializedValueShape::Int => functions
            .int_list_function_functions
            .push((index, function)),
        SpecializedValueShape::String => {
            functions
                .string_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::BitArray => {
            functions
                .bit_array_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::UtfCodepoint => {
            functions
                .utf_codepoint_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Custom(_) => {
            functions
                .custom_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Float => {
            functions
                .float_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Bool => {
            functions
                .bool_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Nil => {
            functions
                .nil_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Tuple(_) => {
            functions
                .tuple_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => functions
                .parameter_list_list_function_functions
                .push((index, function)),
            _ => functions
                .list_list_function_functions
                .push((index, function)),
        },
        SpecializedValueShape::Function(_) => {
            functions
                .function_list_function_functions
                .push((index, function));
        }
    }
}

pub(super) fn function_id(
    shape: &StoredValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
    representations: &super::specialization::RepresentationContext,
) -> RuntimeFunctionId {
    match shape {
        StoredValueShape::Int => RuntimeFunctionId::Int(IntFunctionId(index)),
        StoredValueShape::Float => RuntimeFunctionId::Float(FloatFunctionId(index)),
        StoredValueShape::String => RuntimeFunctionId::String(StringFunctionId(index)),
        StoredValueShape::BitArray => RuntimeFunctionId::BitArray(BitArrayFunctionId(index)),
        StoredValueShape::UtfCodepoint => {
            RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(index))
        }
        StoredValueShape::Custom(shape) => RuntimeFunctionId::Custom(
            super::super::CustomFunctionId::new(index, types.custom_value_shape(shape)),
        ),
        StoredValueShape::Bool => RuntimeFunctionId::Bool(BoolFunctionId(index)),
        StoredValueShape::Nil => RuntimeFunctionId::Nil(NilFunctionId(index)),
        StoredValueShape::Tuple(elements) => RuntimeFunctionId::Tuple {
            id: TupleFunctionId(index),
            return_type: elements
                .iter()
                .map(|shape| types.value_type(shape))
                .collect(),
        },
        StoredValueShape::List(item) => {
            RuntimeFunctionId::List(list_function_id(item, index, types))
        }
        StoredValueShape::Function(function) => RuntimeFunctionId::Function {
            id: function_function_id(function, index, types, representations),
            return_type: types.function_type(function),
        },
    }
}

pub(super) fn stored_function_table_family(
    shape: &StoredValueShape,
    representations: &super::specialization::RepresentationContext,
) -> FunctionTableFamily {
    match shape {
        StoredValueShape::Int => FunctionTableFamily::Int,
        StoredValueShape::Float => FunctionTableFamily::Float,
        StoredValueShape::String => FunctionTableFamily::String,
        StoredValueShape::BitArray => FunctionTableFamily::BitArray,
        StoredValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepoint,
        StoredValueShape::Custom(_) => FunctionTableFamily::Custom,
        StoredValueShape::Bool => FunctionTableFamily::Bool,
        StoredValueShape::Nil => FunctionTableFamily::Nil,
        StoredValueShape::Tuple(_) => FunctionTableFamily::Tuple,
        StoredValueShape::List(item) => list_function_table_family(item),
        StoredValueShape::Function(function) => {
            function_function_table_family(function, representations)
        }
    }
}

pub(super) fn list_function_table_family(item: &SpecializedValueShape) -> FunctionTableFamily {
    match item {
        SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterList,
        SpecializedValueShape::Int => FunctionTableFamily::IntList,
        SpecializedValueShape::String => FunctionTableFamily::StringList,
        SpecializedValueShape::BitArray => FunctionTableFamily::BitArrayList,
        SpecializedValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointList,
        SpecializedValueShape::Custom(_) => FunctionTableFamily::CustomList,
        SpecializedValueShape::Float => FunctionTableFamily::FloatList,
        SpecializedValueShape::Bool => FunctionTableFamily::BoolList,
        SpecializedValueShape::Nil => FunctionTableFamily::NilList,
        SpecializedValueShape::Tuple(_) => FunctionTableFamily::TupleList,
        SpecializedValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterListList,
            _ => FunctionTableFamily::ListList,
        },
        SpecializedValueShape::Function(_) => FunctionTableFamily::FunctionList,
    }
}

pub(super) fn function_function_table_family(
    function: &SpecializedFunctionShape,
    representations: &super::specialization::RepresentationContext,
) -> FunctionTableFamily {
    match function.representation(representations) {
        FunctionRepresentation::Symbolic => FunctionTableFamily::GenericFunction,
        FunctionRepresentation::Never(_) => FunctionTableFamily::NeverFunction,
        FunctionRepresentation::Executable(return_) => {
            executable_function_function_table_family(&return_)
        }
    }
}

fn executable_function_function_table_family(return_: &StoredValueShape) -> FunctionTableFamily {
    match return_ {
        StoredValueShape::Int => FunctionTableFamily::IntFunction,
        StoredValueShape::Float => FunctionTableFamily::FloatFunction,
        StoredValueShape::String => FunctionTableFamily::StringFunction,
        StoredValueShape::BitArray => FunctionTableFamily::BitArrayFunction,
        StoredValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointFunction,
        StoredValueShape::Custom(_) => FunctionTableFamily::CustomFunction,
        StoredValueShape::Bool => FunctionTableFamily::BoolFunction,
        StoredValueShape::Nil => FunctionTableFamily::NilFunction,
        StoredValueShape::Tuple(_) => FunctionTableFamily::TupleFunction,
        StoredValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterListFunction,
            SpecializedValueShape::Int => FunctionTableFamily::IntListFunction,
            SpecializedValueShape::String => FunctionTableFamily::StringListFunction,
            SpecializedValueShape::BitArray => FunctionTableFamily::BitArrayListFunction,
            SpecializedValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointListFunction,
            SpecializedValueShape::Custom(_) => FunctionTableFamily::CustomListFunction,
            SpecializedValueShape::Float => FunctionTableFamily::FloatListFunction,
            SpecializedValueShape::Bool => FunctionTableFamily::BoolListFunction,
            SpecializedValueShape::Nil => FunctionTableFamily::NilListFunction,
            SpecializedValueShape::Tuple(_) => FunctionTableFamily::TupleListFunction,
            SpecializedValueShape::List(item) => match item.as_ref() {
                SpecializedValueShape::Parameter(_) => {
                    FunctionTableFamily::ParameterListListFunction
                }
                _ => FunctionTableFamily::ListListFunction,
            },
            SpecializedValueShape::Function(_) => FunctionTableFamily::FunctionListFunction,
        },
        StoredValueShape::Function(_) => FunctionTableFamily::FunctionFunction,
    }
}

pub(super) fn list_function_function_table_family(
    item: &SpecializedValueShape,
) -> FunctionTableFamily {
    executable_function_function_table_family(&StoredValueShape::List(Box::new(item.clone())))
}

pub(super) fn list_function_id(
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionId {
    match item {
        SpecializedValueShape::Parameter(parameter) => ListFunctionId::Parameter(
            ParameterListFunctionId::new(index, types.parameter_list_type(*parameter)),
        ),
        SpecializedValueShape::Int => {
            ListFunctionId::Int(IntListFunctionId::new(index, types.int_list_type()))
        }
        SpecializedValueShape::String => {
            ListFunctionId::String(StringListFunctionId::new(index, types.string_list_type()))
        }
        SpecializedValueShape::BitArray => ListFunctionId::BitArray(BitArrayListFunctionId::new(
            index,
            types.bit_array_list_type(),
        )),
        SpecializedValueShape::UtfCodepoint => ListFunctionId::UtfCodepoint(
            UtfCodepointListFunctionId::new(index, types.utf_codepoint_list_type()),
        ),
        SpecializedValueShape::Custom(item) => ListFunctionId::Custom(CustomListFunctionId::new(
            index,
            types.custom_list_type(item),
        )),
        SpecializedValueShape::Float => {
            ListFunctionId::Float(FloatListFunctionId::new(index, types.float_list_type()))
        }
        SpecializedValueShape::Bool => {
            ListFunctionId::Bool(BoolListFunctionId::new(index, types.bool_list_type()))
        }
        SpecializedValueShape::Nil => {
            ListFunctionId::Nil(NilListFunctionId::new(index, types.nil_list_type()))
        }
        SpecializedValueShape::Tuple(item) => {
            ListFunctionId::Tuple(TupleListFunctionId::new(index, types.tuple_list_type(item)))
        }
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(type_id) => {
                ListFunctionId::ParameterList(ParameterListListFunctionId::new(index, type_id))
            }
            super::value_type::NestedListTypeId::Stored(type_id) => {
                ListFunctionId::List(ListListFunctionId::new(index, type_id))
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionId::Function(
            FunctionListFunctionId::new(index, types.function_list_type(item)),
        ),
    }
}

pub(super) fn function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
    representations: &super::specialization::RepresentationContext,
) -> FunctionFunctionId {
    use super::super as execution;

    let return_ = match function.representation(representations) {
        FunctionRepresentation::Symbolic => {
            return FunctionFunctionId::Generic(execution::GenericFunctionFunctionId::new(
                index,
                types.generic_function_type(function),
            ));
        }
        FunctionRepresentation::Never(_) => {
            return FunctionFunctionId::Never(execution::NeverFunctionFunctionId::new(
                index,
                types.generic_function_type(function),
            ));
        }
        FunctionRepresentation::Executable(return_) => return_,
    };

    match return_ {
        StoredValueShape::Int => FunctionFunctionId::Int(IntFunctionFunctionId(index)),
        StoredValueShape::Float => FunctionFunctionId::Float(FloatFunctionFunctionId(index)),
        StoredValueShape::String => FunctionFunctionId::String(StringFunctionFunctionId(index)),
        StoredValueShape::BitArray => {
            FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(index))
        }
        StoredValueShape::UtfCodepoint => {
            FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(index))
        }
        StoredValueShape::Custom(return_) => {
            FunctionFunctionId::Custom(execution::CustomFunctionFunctionId::new(
                index,
                types.custom_function_type(function.arguments(), &return_),
            ))
        }
        StoredValueShape::Bool => FunctionFunctionId::Bool(BoolFunctionFunctionId(index)),
        StoredValueShape::Nil => FunctionFunctionId::Nil(NilFunctionFunctionId(index)),
        StoredValueShape::Tuple(_) => FunctionFunctionId::Tuple(TupleFunctionFunctionId(index)),
        StoredValueShape::List(item) => {
            FunctionFunctionId::List(list_function_function_id(function, &item, index, types))
        }
        StoredValueShape::Function(return_) => {
            FunctionFunctionId::Function(execution::FunctionFunctionFunctionId::new(
                index,
                types.function_function_type(function.arguments(), &return_),
            ))
        }
    }
}

pub(super) fn list_function_function_id(
    function: &SpecializedFunctionShape,
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionFunctionId {
    let type_ = types.function_type(function);

    match item {
        SpecializedValueShape::Parameter(parameter) => ListFunctionFunctionId::Parameter {
            id: super::super::ParameterListFunctionFunctionId(index),
            type_,
            list_type: types.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => ListFunctionFunctionId::Int {
            id: super::super::IntListFunctionFunctionId(index),
            type_,
            list_type: types.int_list_type(),
        },
        SpecializedValueShape::String => ListFunctionFunctionId::String {
            id: super::super::StringListFunctionFunctionId(index),
            type_,
            list_type: types.string_list_type(),
        },
        SpecializedValueShape::BitArray => ListFunctionFunctionId::BitArray {
            id: super::super::BitArrayListFunctionFunctionId(index),
            type_,
            list_type: types.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => ListFunctionFunctionId::UtfCodepoint {
            id: super::super::UtfCodepointListFunctionFunctionId(index),
            type_,
            list_type: types.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(item) => ListFunctionFunctionId::Custom {
            id: super::super::CustomListFunctionFunctionId(index),
            type_,
            list_type: types.custom_list_type(item),
        },
        SpecializedValueShape::Float => ListFunctionFunctionId::Float {
            id: super::super::FloatListFunctionFunctionId(index),
            type_,
            list_type: types.float_list_type(),
        },
        SpecializedValueShape::Bool => ListFunctionFunctionId::Bool {
            id: super::super::BoolListFunctionFunctionId(index),
            type_,
            list_type: types.bool_list_type(),
        },
        SpecializedValueShape::Nil => ListFunctionFunctionId::Nil {
            id: super::super::NilListFunctionFunctionId(index),
            type_,
            list_type: types.nil_list_type(),
        },
        SpecializedValueShape::Tuple(item) => ListFunctionFunctionId::Tuple {
            id: super::super::TupleListFunctionFunctionId(index),
            type_,
            list_type: types.tuple_list_type(item),
        },
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(list_type) => {
                ListFunctionFunctionId::ParameterList {
                    id: super::super::ParameterListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            super::value_type::NestedListTypeId::Stored(list_type) => {
                ListFunctionFunctionId::List {
                    id: super::super::ListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionFunctionId::Function {
            id: super::super::FunctionListFunctionFunctionId(index),
            type_,
            list_type: types.function_list_type(item),
        },
    }
}

fn sort_functions<Return>(
    mut functions: Vec<(usize, LoweredFunction<Return>)>,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<ExecutableFunction<Return>> {
    functions.sort_by_key(|(index, _)| *index);
    let mut lowered = Vec::new();
    for (_, function) in functions {
        match function.function {
            Representability::Inhabited(function) => lowered.push(function),
            Representability::Uninhabited => {
                erased.insert(function.specialization);
            }
        }
    }
    lowered
}

fn sort_list_functions<Id, Return>(
    mut functions: Vec<(Id, LoweredFunction<Return>)>,
    index: fn(&Id) -> usize,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<(Id, ExecutableFunction<Return>)> {
    functions.sort_by_key(|(id, _)| index(id));
    let mut lowered = Vec::new();
    for (id, function) in functions {
        match function.function {
            Representability::Inhabited(function) => lowered.push((id, function)),
            Representability::Uninhabited => {
                erased.insert(function.specialization);
            }
        }
    }
    lowered
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        ExecutionPlan, IntFunctionId, IntListFunctionId, IntListReturn, NeverFunctionId,
        RuntimeFunctionId,
    };
    use super::super::specialization::{Representability, SpecializationKey};
    use super::{LoweredFunction, sort_functions, sort_list_functions};
    use std::collections::HashSet;

    #[test]
    fn function_table_lowering_preserves_provisional_specialization_erasure_keys() {
        let function = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(3));
        let mut specializations = HashSet::new();
        let lowered: Vec<super::ExecutableFunction<super::IntReturn>> = sort_functions(
            vec![(
                0,
                LoweredFunction {
                    specialization: function.clone(),
                    function: Representability::Uninhabited,
                },
            )],
            &mut specializations,
        );

        assert!(lowered.is_empty());
        assert_eq!(specializations, HashSet::from([function]));
    }

    #[test]
    fn list_function_table_lowering_preserves_provisional_specialization_erasure_keys() {
        let first = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(5));
        let second = SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(6));
        let plan = execution_plan("fn values() { [1] } pub fn main() { values() }");
        let first_id = plan.int_list_function_id(0);
        let second_id = plan.int_list_function_id(1);
        let mut specializations = HashSet::new();
        let lowered: Vec<(IntListFunctionId, super::ExecutableFunction<IntListReturn>)> =
            sort_list_functions(
                vec![
                    (
                        second_id,
                        LoweredFunction {
                            specialization: second.clone(),
                            function: Representability::Uninhabited,
                        },
                    ),
                    (
                        first_id,
                        LoweredFunction {
                            specialization: first.clone(),
                            function: Representability::Uninhabited,
                        },
                    ),
                ],
                |id| id.index(),
                &mut specializations,
            );

        assert!(lowered.is_empty());
        assert_eq!(specializations, HashSet::from([first, second]));
    }

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

pub fn main() {
  let _ = #(
    float_value,
    string_value,
    bit_array_value,
    utf_codepoint_value,
    bool_value,
    nil_value,
    tuple_value,
    int_list,
    string_list,
    bit_array_list,
    utf_codepoint_list,
    float_list,
    bool_list,
    nil_list,
    tuple_list,
    list_list,
    function_list,
    int_function,
    float_function,
    string_function,
    bit_array_function,
    utf_codepoint_function,
    bool_function,
    nil_function,
    tuple_function,
    int_list_function,
    string_list_function,
    bit_array_list_function,
    utf_codepoint_list_function,
    float_list_function,
    bool_list_function,
    nil_list_function,
    tuple_list_function,
    list_list_function,
    function_list_function,
    function_function,
  )
  int_value()
}
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

    #[test]
    fn lowering_preserves_returned_function_evaluation_before_diverging_arguments() {
        let source = r#"
pub type Token { Token }

fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn result_int(_value) -> fn() -> Int { fn() { 1 } }
fn result_float(_value) -> fn() -> Float { fn() { 1.5 } }
fn result_string(_value) -> fn() -> String { fn() { "value" } }
fn result_bit_array(_value) -> fn() -> BitArray { fn() { <<1>> } }
fn result_utf_codepoint(_value) -> fn() -> UtfCodepoint { fn() { codepoint() } }
fn result_custom(_value) -> fn() -> Token { fn() { Token } }
fn result_bool(_value) -> fn() -> Bool { fn() { True } }
fn result_nil(_value) -> fn() -> Nil { fn() { Nil } }
fn result_tuple(_value) -> fn() -> #(Int) { fn() { #(1) } }
fn result_list(_value) -> fn() -> List(Int) { fn() { [1] } }
fn result_function(_value) -> fn() -> fn() -> Int { fn() { fn() { 1 } } }

fn call_int() -> fn() -> Int { let function = result_int function(panic) }
fn call_float() -> fn() -> Float { let function = result_float function(panic) }
fn call_string() -> fn() -> String { let function = result_string function(panic) }
fn call_bit_array() -> fn() -> BitArray { let function = result_bit_array function(panic) }
fn call_utf_codepoint() -> fn() -> UtfCodepoint {
  let function = result_utf_codepoint
  function(panic)
}
fn call_custom() -> fn() -> Token { let function = result_custom function(panic) }
fn call_bool() -> fn() -> Bool { let function = result_bool function(panic) }
fn call_nil() -> fn() -> Nil { let function = result_nil function(panic) }
fn call_tuple() -> fn() -> #(Int) { let function = result_tuple function(panic) }
fn call_list() -> fn() -> List(Int) { let function = result_list function(panic) }
fn call_function() -> fn() -> fn() -> Int { let function = result_function function(panic) }

pub fn main() {
  let _ = #(
    call_int,
    call_float,
    call_string,
    call_bit_array,
    call_utf_codepoint,
    call_custom,
    call_bool,
    call_nil,
    call_tuple,
    call_list,
    call_function,
  )
  1
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            (
                plan.functions.int_function_functions.len(),
                plan.functions.float_function_functions.len(),
                plan.functions.string_function_functions.len(),
                plan.functions.bit_array_function_functions.len(),
                plan.functions.utf_codepoint_function_functions.len(),
                plan.functions.custom_function_functions.len(),
                plan.functions.bool_function_functions.len(),
                plan.functions.nil_function_functions.len(),
                plan.functions.tuple_function_functions.len(),
                plan.functions.int_list_function_functions.len(),
                plan.functions.function_function_functions.len(),
            ),
            (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
        );
    }

    #[test]
    fn lowering_preserves_prefix_evaluation_for_recursive_never_returns() {
        let tuple_plan = execution_plan(
            r#"
fn build() -> #(Int, value) {
  #(panic as "tuple prefix", panic as "tuple field")
}

pub fn main() { build() }
"#,
        );
        assert_eq!(
            tuple_plan.main_runtime(),
            RuntimeFunctionId::Never(NeverFunctionId(0)),
        );
        assert_eq!(
            crate::run_main(&tuple_plan)
                .expect_err("recursive uninhabited tuple should stop execution")
                .to_string(),
            "panic: tuple prefix",
        );

        let custom_plan = execution_plan(
            r#"
pub type Boxed(value) {
  Boxed(prefix: Int, value: value)
}

fn build() -> Boxed(value) {
  Boxed(prefix: panic as "custom prefix", value: panic as "custom field")
}

pub fn main() { build() }
"#,
        );
        assert_eq!(
            custom_plan.main_runtime(),
            RuntimeFunctionId::Never(NeverFunctionId(0)),
        );
        assert_eq!(
            crate::run_main(&custom_plan)
                .expect_err("recursive uninhabited custom should stop execution")
                .to_string(),
            "panic: custom prefix",
        );
    }

    #[test]
    fn inhabitable_custom_signatures_keep_diverging_bodies_in_the_custom_table() {
        let plan = execution_plan(
            r#"
pub type Maybe(value) {
  None
  Some(value)
}

fn impossible() -> Maybe(value) {
  Some(panic as "generic custom specialization failed")
}

pub fn main() {
  impossible()
}
"#,
        );
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Custom(plan.custom_function_id(0)),
        );
        assert_eq!(plan.functions.custom_functions.len(), 2);
        assert!(plan.functions.never_functions.is_empty());
        assert_eq!(
            crate::run_main(&plan)
                .expect_err("diverging custom body should stop execution")
                .to_string(),
            "panic: generic custom specialization failed",
        );
    }

    #[test]
    fn uncallable_empty_custom_function_references_remain_values() {
        let plan = execution_plan(
            r#"
pub type Never

fn identity(value: Never) -> Never { value }

pub fn main() { identity == identity }
"#,
        );

        assert_eq!(
            crate::run_main(&plan).expect("function references should remain comparable values"),
            crate::Value::Bool(true),
        );
    }

    #[test]
    fn lowering_preserves_tuple_and_custom_never_function_handoffs() {
        let plan = execution_plan(
            r#"
pub type Boxed(value) {
  Boxed(value)
}

fn tuple_target() -> #(value) { #(panic) }
fn custom_target() -> Boxed(value) { Boxed(panic) }
fn tuple_provider() { tuple_target }
fn custom_provider() { custom_target }

pub fn main() {
  let tuple_function = tuple_provider()
  let custom_function = custom_provider()
  let assert [tuple_list_function] = [tuple_target]
  let assert [custom_list_function] = [custom_target]
  #(
    tuple_function == tuple_function,
    custom_function == custom_function,
    tuple_list_function == tuple_target,
    custom_list_function == custom_target,
  )
}
"#,
        );

        assert_eq!(
            crate::run_main(&plan).expect("main should compare preserved function identities"),
            crate::Value::Tuple(vec![
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
            ]),
        );
    }

    #[test]
    fn generic_specialization_tables_lower_every_symbolic_handoff() {
        let fixtures = [
            (
                "unresolved list handoffs",
                include_str!(
                    "../../../../tests/fixtures/execution/functions/generic_unresolved_list_handoffs.gleam"
                ),
                20,
            ),
            (
                "nested list handoffs",
                include_str!(
                    "../../../../tests/fixtures/execution/functions/generic_nested_list_handoffs.gleam"
                ),
                14,
            ),
            (
                "symbolic tail returns",
                include_str!(
                    "../../../../tests/fixtures/execution/functions/generic_symbolic_tail_returns.gleam"
                ),
                5,
            ),
            (
                "never function handoffs",
                include_str!(
                    "../../../../tests/fixtures/execution/functions/generic_never_function_handoffs.gleam"
                ),
                38,
            ),
            (
                "recursive never value handoffs",
                include_str!(
                    "../../../../tests/fixtures/execution/functions/generic_recursive_never_value_handoffs.gleam"
                ),
                28,
            ),
        ];

        for (name, source, result_count) in fixtures {
            let plan = execution_plan(source);

            assert_eq!(
                crate::run_main(&plan).expect("generic handoff fixture should execute"),
                crate::Value::Tuple(vec![crate::Value::Bool(true); result_count]),
                "{name}",
            );
        }
    }

    #[test]
    fn generic_specialization_preserves_every_diverging_handoff() {
        let fixtures = [
            (
                include_str!(
                    "../../../../tests/fixtures/execution_errors/functions/generic_never_block.gleam"
                ),
                "panic: generic block failed",
            ),
            (
                include_str!(
                    "../../../../tests/fixtures/execution_errors/functions/generic_recursive_never_block_handoffs.gleam"
                ),
                "panic: generic tuple block failed",
            ),
            (
                include_str!(
                    "../../../../tests/fixtures/execution_errors/functions/generic_symbolic_function_call_family_lowering.gleam"
                ),
                "panic: symbolic function argument failed",
            ),
            (
                include_str!(
                    "../../../../tests/fixtures/execution_errors/functions/generic_concrete_function_specialization_divergence.gleam"
                ),
                "panic: concrete function specialization failed",
            ),
            (
                include_str!(
                    "../../../../tests/fixtures/execution_errors/functions/generic_diverging_function_call_family_lowering.gleam"
                ),
                "panic: generic function argument failed",
            ),
        ];

        for (source, expected) in fixtures {
            let plan = execution_plan(source);

            assert_eq!(
                crate::run_main(&plan)
                    .expect_err("generic divergence fixture should stop execution")
                    .to_string(),
                expected,
            );
        }
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
