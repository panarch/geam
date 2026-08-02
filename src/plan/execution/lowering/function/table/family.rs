use crate::plan::execution;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CoreRuntimeFunctionId, CustomListFunctionId,
    ExternalListFunctionFunctionId, ExternalListFunctionId, FloatFunctionFunctionId,
    FloatFunctionId, FloatListFunctionId, FunctionFunctionId, FunctionListFunctionId,
    IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListFunctionFunctionId,
    ListFunctionId, ListListFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, ProfiledFunctionFunctionId,
    ProfiledListFunctionFunctionId, RuntimeFunctionFunctionTarget, RuntimeFunctionId,
    RuntimeListFunctionId, StringFunctionFunctionId, StringFunctionId, StringListFunctionId,
    TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::graph::ExternalFunctionCallTarget;
use crate::plan::execution::lowering::specialization::{
    FunctionRepresentation, SpecializedFunctionShape, SpecializedValueShape, StoredValueShape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::plan::execution::lowering) enum FunctionTableFamily {
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Bool,
    Nil,
    Tuple,
    ParameterList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    ExternalList,
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
    ExternalFunction,
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
    ExternalListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ListListFunction,
    FunctionListFunction,
    FunctionFunction,
}

#[derive(Clone)]
pub(in crate::plan::execution::lowering) enum ListFunctionFunctionSignature {
    Core(CoreListFunctionFunctionSignature),
    External(ExternalListFunctionFunctionSignature),
}

#[derive(Clone)]
pub(in crate::plan::execution::lowering) struct CoreListFunctionFunctionSignature {
    pub(super) type_: execution::type_::FunctionType,
    pub(super) return_: CoreListFunctionReturn,
}

#[derive(Clone)]
pub(in crate::plan::execution::lowering) struct ExternalListFunctionFunctionSignature {
    type_: execution::type_::FunctionType,
    list_type: execution::type_::ExternalListTypeId,
}

#[derive(Clone, Copy)]
pub(super) enum CoreListFunctionReturn {
    Parameter(execution::type_::ParameterListTypeId),
    ParameterList(execution::type_::ParameterListListTypeId),
    Int(execution::type_::IntListTypeId),
    String(execution::type_::StringListTypeId),
    BitArray(execution::type_::BitArrayListTypeId),
    UtfCodepoint(execution::type_::UtfCodepointListTypeId),
    Custom(execution::type_::CustomListTypeId),
    Float(execution::type_::FloatListTypeId),
    Bool(execution::type_::BoolListTypeId),
    Nil(execution::type_::NilListTypeId),
    Tuple(execution::type_::TupleListTypeId),
    List(execution::type_::ListListTypeId),
    Function(execution::type_::FunctionListTypeId),
}

impl ListFunctionFunctionSignature {
    pub(in crate::plan::execution::lowering) fn table_family(&self) -> FunctionTableFamily {
        match self {
            Self::Core(signature) => signature.table_family(),
            Self::External(_) => FunctionTableFamily::ExternalListFunction,
        }
    }

    pub(in crate::plan::execution::lowering) fn hosted_id(
        &self,
        index: usize,
    ) -> ListFunctionFunctionId {
        match self {
            Self::Core(signature) => signature.profiled_id(index),
            Self::External(signature) => ProfiledListFunctionFunctionId::External {
                id: ExternalListFunctionFunctionId(index),
                type_: signature.type_.clone(),
                list_type: signature.list_type,
            },
        }
    }

    fn runtime_id(&self, index: usize) -> RuntimeFunctionFunctionTarget {
        match self {
            Self::Core(signature) => RuntimeFunctionFunctionTarget::Core(
                ProfiledFunctionFunctionId::List(signature.profiled_id(index)),
            ),
            Self::External(signature) => {
                RuntimeFunctionFunctionTarget::External(ExternalFunctionCallTarget::ListFunction {
                    id: ExternalListFunctionFunctionId(index),
                    type_: signature.type_.clone(),
                    list_type: signature.list_type,
                })
            }
        }
    }
}

impl CoreListFunctionFunctionSignature {
    pub(in crate::plan::execution::lowering) fn table_family(&self) -> FunctionTableFamily {
        match self.return_ {
            CoreListFunctionReturn::Parameter(_) => FunctionTableFamily::ParameterListFunction,
            CoreListFunctionReturn::ParameterList(_) => {
                FunctionTableFamily::ParameterListListFunction
            }
            CoreListFunctionReturn::Int(_) => FunctionTableFamily::IntListFunction,
            CoreListFunctionReturn::String(_) => FunctionTableFamily::StringListFunction,
            CoreListFunctionReturn::BitArray(_) => FunctionTableFamily::BitArrayListFunction,
            CoreListFunctionReturn::UtfCodepoint(_) => {
                FunctionTableFamily::UtfCodepointListFunction
            }
            CoreListFunctionReturn::Custom(_) => FunctionTableFamily::CustomListFunction,
            CoreListFunctionReturn::Float(_) => FunctionTableFamily::FloatListFunction,
            CoreListFunctionReturn::Bool(_) => FunctionTableFamily::BoolListFunction,
            CoreListFunctionReturn::Nil(_) => FunctionTableFamily::NilListFunction,
            CoreListFunctionReturn::Tuple(_) => FunctionTableFamily::TupleListFunction,
            CoreListFunctionReturn::List(_) => FunctionTableFamily::ListListFunction,
            CoreListFunctionReturn::Function(_) => FunctionTableFamily::FunctionListFunction,
        }
    }

    pub(in crate::plan::execution::lowering) fn profiled_id<
        Graph: execution::function::ExecutionGraphProfile,
    >(
        &self,
        index: usize,
    ) -> ProfiledListFunctionFunctionId<Graph> {
        let type_ = self.type_.clone();
        match self.return_ {
            CoreListFunctionReturn::Parameter(list_type) => {
                ProfiledListFunctionFunctionId::Parameter {
                    id: execution::function::ParameterListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            CoreListFunctionReturn::ParameterList(list_type) => {
                ProfiledListFunctionFunctionId::ParameterList {
                    id: execution::function::ParameterListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            CoreListFunctionReturn::Int(list_type) => ProfiledListFunctionFunctionId::Int {
                id: execution::function::IntListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::String(list_type) => ProfiledListFunctionFunctionId::String {
                id: execution::function::StringListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::BitArray(list_type) => {
                ProfiledListFunctionFunctionId::BitArray {
                    id: execution::function::BitArrayListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            CoreListFunctionReturn::UtfCodepoint(list_type) => {
                ProfiledListFunctionFunctionId::UtfCodepoint {
                    id: execution::function::UtfCodepointListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            CoreListFunctionReturn::Custom(list_type) => ProfiledListFunctionFunctionId::Custom {
                id: execution::function::CustomListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::Float(list_type) => ProfiledListFunctionFunctionId::Float {
                id: execution::function::FloatListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::Bool(list_type) => ProfiledListFunctionFunctionId::Bool {
                id: execution::function::BoolListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::Nil(list_type) => ProfiledListFunctionFunctionId::Nil {
                id: execution::function::NilListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::Tuple(list_type) => ProfiledListFunctionFunctionId::Tuple {
                id: execution::function::TupleListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::List(list_type) => ProfiledListFunctionFunctionId::List {
                id: execution::function::ListListFunctionFunctionId(index),
                type_,
                list_type,
            },
            CoreListFunctionReturn::Function(list_type) => {
                ProfiledListFunctionFunctionId::Function {
                    id: execution::function::FunctionListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
        }
    }
}

impl ExternalListFunctionFunctionSignature {
    pub(in crate::plan::execution::lowering) fn id(
        &self,
        index: usize,
    ) -> ExternalListFunctionFunctionId {
        ExternalListFunctionFunctionId(index)
    }
}

pub(in crate::plan::execution::lowering) fn function_id(
    shape: &StoredValueShape,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
    representations: &crate::plan::execution::lowering::specialization::RepresentationContext,
) -> RuntimeFunctionId {
    match shape {
        StoredValueShape::Int => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(index)))
        }
        StoredValueShape::Float => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Float(FloatFunctionId(index)))
        }
        StoredValueShape::String => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::String(StringFunctionId(index)))
        }
        StoredValueShape::BitArray => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::BitArray(BitArrayFunctionId(index)))
        }
        StoredValueShape::UtfCodepoint => RuntimeFunctionId::Core(
            CoreRuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(index)),
        ),
        StoredValueShape::Custom(shape) => RuntimeFunctionId::Core(CoreRuntimeFunctionId::Custom(
            execution::function::CustomFunctionId::new(index, types.custom_value_shape(shape)),
        )),
        StoredValueShape::External(type_) => RuntimeFunctionId::External(
            execution::function::ExternalFunctionId::new(index, types.external_type(type_)),
        ),
        StoredValueShape::Bool => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Bool(BoolFunctionId(index)))
        }
        StoredValueShape::Nil => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Nil(NilFunctionId(index)))
        }
        StoredValueShape::Tuple(elements) => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Tuple {
                id: TupleFunctionId(index),
                return_type: elements
                    .iter()
                    .map(|shape| types.value_type(shape))
                    .collect(),
            })
        }
        StoredValueShape::List(item) => RuntimeFunctionId::Core(CoreRuntimeFunctionId::List(
            list_function_id(item, index, types),
        )),
        StoredValueShape::Function(function) => {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Function {
                id: runtime_function_function_id(function, index, types, representations),
                return_type: types.function_type(function),
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn stored_function_table_family(
    shape: &StoredValueShape,
    representations: &crate::plan::execution::lowering::specialization::RepresentationContext,
) -> FunctionTableFamily {
    match shape {
        StoredValueShape::Int => FunctionTableFamily::Int,
        StoredValueShape::Float => FunctionTableFamily::Float,
        StoredValueShape::String => FunctionTableFamily::String,
        StoredValueShape::BitArray => FunctionTableFamily::BitArray,
        StoredValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepoint,
        StoredValueShape::Custom(_) => FunctionTableFamily::Custom,
        StoredValueShape::External(_) => FunctionTableFamily::External,
        StoredValueShape::Bool => FunctionTableFamily::Bool,
        StoredValueShape::Nil => FunctionTableFamily::Nil,
        StoredValueShape::Tuple(_) => FunctionTableFamily::Tuple,
        StoredValueShape::List(item) => list_function_table_family(item),
        StoredValueShape::Function(function) => {
            function_function_table_family(function, representations)
        }
    }
}

pub(in crate::plan::execution::lowering) fn list_function_table_family(
    item: &SpecializedValueShape,
) -> FunctionTableFamily {
    match item {
        SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterList,
        SpecializedValueShape::Int => FunctionTableFamily::IntList,
        SpecializedValueShape::String => FunctionTableFamily::StringList,
        SpecializedValueShape::BitArray => FunctionTableFamily::BitArrayList,
        SpecializedValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointList,
        SpecializedValueShape::Custom(_) => FunctionTableFamily::CustomList,
        SpecializedValueShape::External(_) => FunctionTableFamily::ExternalList,
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

pub(in crate::plan::execution::lowering) fn function_function_table_family(
    function: &SpecializedFunctionShape,
    representations: &crate::plan::execution::lowering::specialization::RepresentationContext,
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
        StoredValueShape::External(_) => FunctionTableFamily::ExternalFunction,
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
            SpecializedValueShape::External(_) => FunctionTableFamily::ExternalListFunction,
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

pub(in crate::plan::execution::lowering) fn list_function_function_signature(
    function: &SpecializedFunctionShape,
    item: &SpecializedValueShape,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
) -> ListFunctionFunctionSignature {
    let type_ = types.function_type(function);
    let return_ = match item {
        SpecializedValueShape::Parameter(parameter) => {
            CoreListFunctionReturn::Parameter(types.parameter_list_type(*parameter))
        }
        SpecializedValueShape::Int => CoreListFunctionReturn::Int(types.int_list_type()),
        SpecializedValueShape::String => CoreListFunctionReturn::String(types.string_list_type()),
        SpecializedValueShape::BitArray => {
            CoreListFunctionReturn::BitArray(types.bit_array_list_type())
        }
        SpecializedValueShape::UtfCodepoint => {
            CoreListFunctionReturn::UtfCodepoint(types.utf_codepoint_list_type())
        }
        SpecializedValueShape::Custom(item) => {
            CoreListFunctionReturn::Custom(types.custom_list_type(item))
        }
        SpecializedValueShape::External(item) => {
            return ListFunctionFunctionSignature::External(
                ExternalListFunctionFunctionSignature {
                    type_,
                    list_type: types.external_list_type(item),
                },
            );
        }
        SpecializedValueShape::Float => CoreListFunctionReturn::Float(types.float_list_type()),
        SpecializedValueShape::Bool => CoreListFunctionReturn::Bool(types.bool_list_type()),
        SpecializedValueShape::Nil => CoreListFunctionReturn::Nil(types.nil_list_type()),
        SpecializedValueShape::Tuple(item) => {
            CoreListFunctionReturn::Tuple(types.tuple_list_type(item))
        }
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            crate::plan::execution::lowering::value_type::NestedListTypeId::Parameter(
                list_type,
            ) => CoreListFunctionReturn::ParameterList(list_type),
            crate::plan::execution::lowering::value_type::NestedListTypeId::Stored(list_type) => {
                CoreListFunctionReturn::List(list_type)
            }
        },
        SpecializedValueShape::Function(item) => {
            CoreListFunctionReturn::Function(types.function_list_type(item))
        }
    };

    ListFunctionFunctionSignature::Core(CoreListFunctionFunctionSignature { type_, return_ })
}

pub(in crate::plan::execution::lowering) fn list_function_id(
    item: &SpecializedValueShape,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
) -> RuntimeListFunctionId {
    let function = match item {
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
        SpecializedValueShape::External(item) => {
            return RuntimeListFunctionId::External(ExternalListFunctionId::new(
                index,
                types.external_list_type(item),
            ));
        }
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
            crate::plan::execution::lowering::value_type::NestedListTypeId::Parameter(type_id) => {
                ListFunctionId::ParameterList(ParameterListListFunctionId::new(index, type_id))
            }
            crate::plan::execution::lowering::value_type::NestedListTypeId::Stored(type_id) => {
                ListFunctionId::List(ListListFunctionId::new(index, type_id))
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionId::Function(
            FunctionListFunctionId::new(index, types.function_list_type(item)),
        ),
    };
    RuntimeListFunctionId::Core(function)
}

pub(in crate::plan::execution::lowering) fn function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
    representations: &crate::plan::execution::lowering::specialization::RepresentationContext,
) -> FunctionFunctionId {
    runtime_function_function_id(function, index, types, representations).runtime_id()
}

fn runtime_function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
    representations: &crate::plan::execution::lowering::specialization::RepresentationContext,
) -> RuntimeFunctionFunctionTarget {
    let return_ = match function.representation(representations) {
        FunctionRepresentation::Symbolic => {
            return RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Generic(
                execution::function::GenericFunctionFunctionId::new(
                    index,
                    types.generic_function_type(function),
                ),
            ));
        }
        FunctionRepresentation::Never(_) => {
            return RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Never(
                execution::function::NeverFunctionFunctionId::new(
                    index,
                    types.generic_function_type(function),
                ),
            ));
        }
        FunctionRepresentation::Executable(return_) => return_,
    };

    match return_ {
        StoredValueShape::Int => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Int(IntFunctionFunctionId(index)),
        ),
        StoredValueShape::Float => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Float(FloatFunctionFunctionId(index)),
        ),
        StoredValueShape::String => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::String(StringFunctionFunctionId(index)),
        ),
        StoredValueShape::BitArray => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::BitArray(BitArrayFunctionFunctionId(index)),
        ),
        StoredValueShape::UtfCodepoint => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(index)),
        ),
        StoredValueShape::Custom(return_) => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Custom(execution::function::CustomFunctionFunctionId::new(
                index,
                types.custom_function_type(function.arguments(), &return_),
            )),
        ),
        StoredValueShape::External(return_) => {
            RuntimeFunctionFunctionTarget::External(ExternalFunctionCallTarget::Function(
                execution::function::ExternalFunctionFunctionId::new(
                    index,
                    types.external_function_type(function.arguments(), &return_),
                ),
            ))
        }
        StoredValueShape::Bool => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Bool(BoolFunctionFunctionId(index)),
        ),
        StoredValueShape::Nil => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Nil(NilFunctionFunctionId(index)),
        ),
        StoredValueShape::Tuple(_) => RuntimeFunctionFunctionTarget::Core(
            ProfiledFunctionFunctionId::Tuple(TupleFunctionFunctionId(index)),
        ),
        StoredValueShape::List(item) => {
            runtime_list_function_function_id(function, &item, index, types)
        }
        StoredValueShape::Function(return_) => {
            RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Function(
                execution::function::FunctionFunctionFunctionId::new(
                    index,
                    types.function_function_type(function.arguments(), &return_),
                ),
            ))
        }
    }
}

fn runtime_list_function_function_id(
    function: &SpecializedFunctionShape,
    item: &SpecializedValueShape,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
) -> RuntimeFunctionFunctionTarget {
    list_function_function_signature(function, item, types).runtime_id(index)
}

#[cfg(test)]
mod tests {
    use super::{
        FunctionTableFamily, function_function_table_family, list_function_table_family,
        stored_function_table_family,
    };
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializedFunctionShape, SpecializedValueShape, StoredValueShape,
    };

    #[test]
    fn maps_stored_shapes_to_exact_function_table_families() {
        let representations = RepresentationContext::new(Vec::new());
        let int_function = SpecializedFunctionShape::new(
            vec![SpecializedValueShape::Bool],
            SpecializedValueShape::Int,
        );

        assert_eq!(
            stored_function_table_family(&StoredValueShape::Int, &representations),
            FunctionTableFamily::Int,
        );
        assert_eq!(
            stored_function_table_family(
                &StoredValueShape::List(Box::new(SpecializedValueShape::String)),
                &representations,
            ),
            FunctionTableFamily::StringList,
        );
        assert_eq!(
            stored_function_table_family(
                &StoredValueShape::Function(Box::new(int_function.clone())),
                &representations,
            ),
            FunctionTableFamily::IntFunction,
        );
        assert_eq!(
            function_function_table_family(&int_function, &representations),
            FunctionTableFamily::IntFunction,
        );
        assert_eq!(
            list_function_table_family(&SpecializedValueShape::Function(Box::new(int_function))),
            FunctionTableFamily::FunctionList,
        );
    }
}
