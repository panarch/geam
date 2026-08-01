mod host;

use crate::plan::execution;
use crate::plan::execution::function::FunctionTables;
use crate::plan::execution::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionId, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionBody,
    BoolListFunctionId, CoreListFunctionFunctionBody, CoreRuntimeFunctionId, CustomFunctionBody,
    CustomFunctionFunctionBody, CustomListFunctionBody, CustomListFunctionId, ExternalFunctionBody,
    ExternalFunctionFunctionBody, ExternalListFunctionBody, ExternalListFunctionFunctionBody,
    ExternalListFunctionFunctionId, ExternalListFunctionId, FloatFunctionBody,
    FloatFunctionFunctionBody, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionBody,
    FloatListFunctionId, FunctionFunctionFunctionBody, FunctionFunctionId,
    FunctionListFunctionBody, FunctionListFunctionId, GenericFunctionFunctionBody,
    GenericFunctionFunctionId, IntFunctionBody, IntFunctionFunctionBody, IntFunctionFunctionId,
    IntFunctionId, IntListFunctionBody, IntListFunctionId, ListFunctionFunctionId, ListFunctionId,
    ListListFunctionBody, ListListFunctionId, NeverFunctionBody, NeverFunctionFunctionBody,
    NeverFunctionFunctionId, NeverFunctionId, NilFunctionBody, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionBody, NilListFunctionId,
    ParameterListFunctionBody, ParameterListFunctionId, ParameterListListFunctionBody,
    ParameterListListFunctionId, ProfiledFunctionFunctionId, RuntimeFunctionFunctionTarget,
    RuntimeFunctionId, RuntimeListFunctionId, StringFunctionBody, StringFunctionFunctionBody,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionBody, StringListFunctionId,
    TupleFunctionBody, TupleFunctionFunctionBody, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionBody, TupleListFunctionId, UtfCodepointFunctionBody,
    UtfCodepointFunctionFunctionBody, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionBody, UtfCodepointListFunctionId,
};
use crate::plan::execution::function::{
    ExecutableFunction, ExecutionBitArrayFunctionBody, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBitArrayListFunctionBody, ExecutionBoolFunctionBody,
    ExecutionBoolFunctionFunctionBody, ExecutionBoolListFunctionBody,
    ExecutionCoreListFunctionFunctionBody, ExecutionCustomFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionCustomListFunctionBody,
    ExecutionExternalFunctionBody, ExecutionExternalFunctionFunctionBody,
    ExecutionExternalListFunctionBody, ExecutionExternalListFunctionFunctionBody,
    ExecutionFloatFunctionBody, ExecutionFloatFunctionFunctionBody, ExecutionFloatListFunctionBody,
    ExecutionFunction, ExecutionFunctionBody, ExecutionFunctionFunctionFunctionBody,
    ExecutionFunctionListFunctionBody, ExecutionGenericFunctionFunctionBody,
    ExecutionIntFunctionBody, ExecutionIntFunctionFunctionBody, ExecutionIntListFunctionBody,
    ExecutionListListFunctionBody, ExecutionNeverFunction, ExecutionNeverFunctionBody,
    ExecutionNeverFunctionFunctionBody, ExecutionNilFunctionBody, ExecutionNilFunctionFunctionBody,
    ExecutionNilListFunctionBody, ExecutionParameterListFunctionBody,
    ExecutionParameterListListFunctionBody, ExecutionProfile, ExecutionStringFunctionBody,
    ExecutionStringFunctionFunctionBody, ExecutionStringListFunctionBody,
    ExecutionTupleFunctionBody, ExecutionTupleFunctionFunctionBody, ExecutionTupleListFunctionBody,
    ExecutionUtfCodepointFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    ExecutionUtfCodepointListFunctionBody, FunctionFunctionTables, HostedExecutionGraph,
    ListFunctionTables, ProfiledCustomFunctionBody, ProfiledCustomFunctionFunctionBody,
    ProfiledFunctionBody, ProfiledFunctionFunctionFunctionBody, ProfiledListFunctionFunctionId,
    TypedFunctionBody, ValueFunctionTables,
};
use crate::plan::execution::graph::ExternalFunctionCallTarget;
use crate::plan::execution::host::HostedExecutionProfile;
use crate::plan::execution::lowering::SpecializationOutcome;
use crate::plan::execution::lowering::specialization::{
    FunctionRepresentation, Representability, SpecializationKey, SpecializedFunctionShape,
    SpecializedValueShape, StoredValueShape,
};
use std::collections::HashSet;
use std::convert::Infallible;

pub(in crate::plan::execution::lowering) struct LoweredSpecialization<Value> {
    specialization: SpecializationKey,
    value: Representability<Value>,
}

pub(super) type LoweredFunction<Return> = LoweredSpecialization<ExecutableFunction<Return>>;
type ProfiledLoweredFunction<Profile, Body> =
    LoweredSpecialization<ExecutionFunction<Profile, Body>>;
type ProfiledLoweredFunctionTable<Id, Profile, Body> =
    Vec<(Id, ProfiledLoweredFunction<Profile, Body>)>;
type ProfiledLoweredNeverFunctionTable<Profile> = Vec<(
    usize,
    LoweredSpecialization<ExecutionNeverFunction<Profile>>,
)>;

pub(in crate::plan::execution::lowering) use host::lowered_host_function;

pub(super) fn lowered_function<Return>(
    specialization: &SpecializationKey,
    graph: Representability<super::super::graph::LoweredFunctionGraph<Return>>,
) -> LoweredFunction<Return> {
    LoweredSpecialization {
        specialization: specialization.clone(),
        value: graph.map(|graph| ExecutableFunction::new(graph.parameter_count, graph.body)),
    }
}

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
    type_: execution::type_::FunctionType,
    return_: CoreListFunctionReturn,
}

#[derive(Clone)]
pub(in crate::plan::execution::lowering) struct ExternalListFunctionFunctionSignature {
    type_: execution::type_::FunctionType,
    list_type: execution::type_::ExternalListTypeId,
}

#[derive(Clone, Copy)]
enum CoreListFunctionReturn {
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

#[derive(Default)]
pub(in crate::plan::execution::lowering) struct FunctionTableBuilder {
    pub(super) never_functions: Vec<(usize, LoweredFunction<NeverFunctionBody>)>,
    pub(super) int_functions: Vec<(usize, LoweredFunction<IntFunctionBody>)>,
    pub(super) float_functions: Vec<(usize, LoweredFunction<FloatFunctionBody>)>,
    pub(super) string_functions: Vec<(usize, LoweredFunction<StringFunctionBody>)>,
    pub(super) bit_array_functions: Vec<(usize, LoweredFunction<BitArrayFunctionBody>)>,
    pub(super) utf_codepoint_functions: Vec<(usize, LoweredFunction<UtfCodepointFunctionBody>)>,
    pub(super) custom_functions: Vec<(usize, LoweredFunction<CustomFunctionBody>)>,
    pub(super) external_functions: Vec<(usize, LoweredFunction<ExternalFunctionBody>)>,
    pub(super) bool_functions: Vec<(usize, LoweredFunction<BoolFunctionBody>)>,
    pub(super) nil_functions: Vec<(usize, LoweredFunction<NilFunctionBody>)>,
    pub(super) tuple_functions: Vec<(usize, LoweredFunction<TupleFunctionBody>)>,
    pub(super) parameter_list_functions: Vec<(
        ParameterListFunctionId,
        LoweredFunction<ParameterListFunctionBody>,
    )>,
    pub(super) int_list_functions: Vec<(IntListFunctionId, LoweredFunction<IntListFunctionBody>)>,
    pub(super) string_list_functions: Vec<(
        StringListFunctionId,
        LoweredFunction<StringListFunctionBody>,
    )>,
    pub(super) bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        LoweredFunction<BitArrayListFunctionBody>,
    )>,
    pub(super) utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        LoweredFunction<UtfCodepointListFunctionBody>,
    )>,
    pub(super) custom_list_functions: Vec<(
        CustomListFunctionId,
        LoweredFunction<CustomListFunctionBody>,
    )>,
    pub(super) external_list_functions: Vec<(
        ExternalListFunctionId,
        LoweredFunction<ExternalListFunctionBody>,
    )>,
    pub(super) float_list_functions:
        Vec<(FloatListFunctionId, LoweredFunction<FloatListFunctionBody>)>,
    pub(super) bool_list_functions:
        Vec<(BoolListFunctionId, LoweredFunction<BoolListFunctionBody>)>,
    pub(super) nil_list_functions: Vec<(NilListFunctionId, LoweredFunction<NilListFunctionBody>)>,
    pub(super) tuple_list_functions:
        Vec<(TupleListFunctionId, LoweredFunction<TupleListFunctionBody>)>,
    pub(super) parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        LoweredFunction<ParameterListListFunctionBody>,
    )>,
    pub(super) list_list_functions:
        Vec<(ListListFunctionId, LoweredFunction<ListListFunctionBody>)>,
    pub(super) function_list_functions: Vec<(
        FunctionListFunctionId,
        LoweredFunction<FunctionListFunctionBody>,
    )>,
    pub(super) int_function_functions: Vec<(usize, LoweredFunction<IntFunctionFunctionBody>)>,
    pub(super) float_function_functions: Vec<(usize, LoweredFunction<FloatFunctionFunctionBody>)>,
    pub(super) string_function_functions: Vec<(usize, LoweredFunction<StringFunctionFunctionBody>)>,
    pub(super) bit_array_function_functions:
        Vec<(usize, LoweredFunction<BitArrayFunctionFunctionBody>)>,
    pub(super) utf_codepoint_function_functions:
        Vec<(usize, LoweredFunction<UtfCodepointFunctionFunctionBody>)>,
    pub(super) custom_function_functions: Vec<(usize, LoweredFunction<CustomFunctionFunctionBody>)>,
    pub(super) external_function_functions:
        Vec<(usize, LoweredFunction<ExternalFunctionFunctionBody>)>,
    pub(super) bool_function_functions: Vec<(usize, LoweredFunction<BoolFunctionFunctionBody>)>,
    pub(super) nil_function_functions: Vec<(usize, LoweredFunction<NilFunctionFunctionBody>)>,
    pub(super) tuple_function_functions: Vec<(usize, LoweredFunction<TupleFunctionFunctionBody>)>,
    pub(super) generic_function_functions:
        Vec<(usize, LoweredFunction<GenericFunctionFunctionBody>)>,
    pub(super) never_function_functions: Vec<(usize, LoweredFunction<NeverFunctionFunctionBody>)>,
    pub(super) parameter_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) parameter_list_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) int_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) string_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) bit_array_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) utf_codepoint_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) custom_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) external_list_function_functions:
        Vec<(usize, LoweredFunction<ExternalListFunctionFunctionBody>)>,
    pub(super) float_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) bool_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) nil_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) tuple_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) list_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) function_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(super) function_function_functions:
        Vec<(usize, LoweredFunction<FunctionFunctionFunctionBody>)>,
}

pub(in crate::plan::execution::lowering) struct AdditionalFunctions<Profile: ExecutionProfile> {
    pub(in crate::plan::execution::lowering) never: ProfiledLoweredNeverFunctionTable<Profile>,
    pub(in crate::plan::execution::lowering) custom:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionCustomFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) external:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionExternalFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) int:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionIntFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) float:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionFloatFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) string:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionStringFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) bit_array:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionBitArrayFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) utf_codepoint:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionUtfCodepointFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) bool:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionBoolFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) nil:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionNilFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) tuple:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionTupleFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) parameter_list: ProfiledLoweredFunctionTable<
        ParameterListFunctionId,
        Profile,
        ExecutionParameterListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) int_list: ProfiledLoweredFunctionTable<
        IntListFunctionId,
        Profile,
        ExecutionIntListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) string_list: ProfiledLoweredFunctionTable<
        StringListFunctionId,
        Profile,
        ExecutionStringListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) bit_array_list: ProfiledLoweredFunctionTable<
        BitArrayListFunctionId,
        Profile,
        ExecutionBitArrayListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) utf_codepoint_list: ProfiledLoweredFunctionTable<
        UtfCodepointListFunctionId,
        Profile,
        ExecutionUtfCodepointListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) custom_list: ProfiledLoweredFunctionTable<
        CustomListFunctionId,
        Profile,
        ExecutionCustomListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) external_list: ProfiledLoweredFunctionTable<
        ExternalListFunctionId,
        Profile,
        ExecutionExternalListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) float_list: ProfiledLoweredFunctionTable<
        FloatListFunctionId,
        Profile,
        ExecutionFloatListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) bool_list: ProfiledLoweredFunctionTable<
        BoolListFunctionId,
        Profile,
        ExecutionBoolListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) nil_list: ProfiledLoweredFunctionTable<
        NilListFunctionId,
        Profile,
        ExecutionNilListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) tuple_list: ProfiledLoweredFunctionTable<
        TupleListFunctionId,
        Profile,
        ExecutionTupleListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) parameter_list_list: ProfiledLoweredFunctionTable<
        ParameterListListFunctionId,
        Profile,
        ExecutionParameterListListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) list_list: ProfiledLoweredFunctionTable<
        ListListFunctionId,
        Profile,
        ExecutionListListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) function_list: ProfiledLoweredFunctionTable<
        FunctionListFunctionId,
        Profile,
        ExecutionFunctionListFunctionBody<Profile>,
    >,
    pub(in crate::plan::execution::lowering) int_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionIntFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) float_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionFloatFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) string_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionStringFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) bit_array_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionBitArrayFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) utf_codepoint_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionUtfCodepointFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) custom_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionCustomFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) external_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionExternalFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) bool_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionBoolFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) nil_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionNilFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) tuple_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionTupleFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) generic_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionGenericFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) never_function_functions:
        ProfiledLoweredFunctionTable<usize, Profile, ExecutionNeverFunctionFunctionBody<Profile>>,
    pub(in crate::plan::execution::lowering) parameter_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) parameter_list_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) int_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) string_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) bit_array_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) utf_codepoint_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) custom_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) external_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionExternalListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) float_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) bool_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) nil_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) tuple_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) list_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) function_list_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionCoreListFunctionFunctionBody<Profile>,
        >,
    pub(in crate::plan::execution::lowering) function_function_functions:
        ProfiledLoweredFunctionTable<
            usize,
            Profile,
            ExecutionFunctionFunctionFunctionBody<Profile>,
        >,
}

impl<Profile: ExecutionProfile> AdditionalFunctions<Profile> {
    fn empty() -> Self {
        Self {
            never: Vec::new(),
            custom: Vec::new(),
            external: Vec::new(),
            int: Vec::new(),
            float: Vec::new(),
            string: Vec::new(),
            bit_array: Vec::new(),
            utf_codepoint: Vec::new(),
            bool: Vec::new(),
            nil: Vec::new(),
            tuple: Vec::new(),
            parameter_list: Vec::new(),
            int_list: Vec::new(),
            string_list: Vec::new(),
            bit_array_list: Vec::new(),
            utf_codepoint_list: Vec::new(),
            custom_list: Vec::new(),
            external_list: Vec::new(),
            float_list: Vec::new(),
            bool_list: Vec::new(),
            nil_list: Vec::new(),
            tuple_list: Vec::new(),
            parameter_list_list: Vec::new(),
            list_list: Vec::new(),
            function_list: Vec::new(),
            int_function_functions: Vec::new(),
            float_function_functions: Vec::new(),
            string_function_functions: Vec::new(),
            bit_array_function_functions: Vec::new(),
            utf_codepoint_function_functions: Vec::new(),
            custom_function_functions: Vec::new(),
            external_function_functions: Vec::new(),
            bool_function_functions: Vec::new(),
            nil_function_functions: Vec::new(),
            tuple_function_functions: Vec::new(),
            generic_function_functions: Vec::new(),
            never_function_functions: Vec::new(),
            parameter_list_function_functions: Vec::new(),
            parameter_list_list_function_functions: Vec::new(),
            int_list_function_functions: Vec::new(),
            string_list_function_functions: Vec::new(),
            bit_array_list_function_functions: Vec::new(),
            utf_codepoint_list_function_functions: Vec::new(),
            custom_list_function_functions: Vec::new(),
            external_list_function_functions: Vec::new(),
            float_list_function_functions: Vec::new(),
            bool_list_function_functions: Vec::new(),
            nil_list_function_functions: Vec::new(),
            tuple_list_function_functions: Vec::new(),
            list_list_function_functions: Vec::new(),
            function_list_function_functions: Vec::new(),
            function_function_functions: Vec::new(),
        }
    }
}

impl FunctionTableBuilder {
    pub(in crate::plan::execution::lowering) fn finish(
        self,
    ) -> SpecializationOutcome<Box<FunctionTables<Infallible>>> {
        Self::finish_profile(self.profile_plain(), AdditionalFunctions::empty())
    }

    fn profile_plain(self) -> AdditionalFunctions<Infallible> {
        AdditionalFunctions {
            never: profile_never_functions::<Infallible>(self.never_functions),
            custom: profile_functions::<Infallible, _, _>(self.custom_functions),
            external: erase_plain_return_family(self.external_functions),
            int: profile_functions::<Infallible, _, _>(self.int_functions),
            float: profile_functions::<Infallible, _, _>(self.float_functions),
            string: profile_functions::<Infallible, _, _>(self.string_functions),
            bit_array: profile_functions::<Infallible, _, _>(self.bit_array_functions),
            utf_codepoint: profile_functions::<Infallible, _, _>(self.utf_codepoint_functions),
            bool: profile_functions::<Infallible, _, _>(self.bool_functions),
            nil: profile_functions::<Infallible, _, _>(self.nil_functions),
            tuple: profile_functions::<Infallible, _, _>(self.tuple_functions),
            parameter_list: profile_functions::<Infallible, _, _>(self.parameter_list_functions),
            int_list: profile_functions::<Infallible, _, _>(self.int_list_functions),
            string_list: profile_functions::<Infallible, _, _>(self.string_list_functions),
            bit_array_list: profile_functions::<Infallible, _, _>(self.bit_array_list_functions),
            utf_codepoint_list: profile_functions::<Infallible, _, _>(
                self.utf_codepoint_list_functions,
            ),
            custom_list: profile_functions::<Infallible, _, _>(self.custom_list_functions),
            external_list: erase_plain_return_family(self.external_list_functions),
            float_list: profile_functions::<Infallible, _, _>(self.float_list_functions),
            bool_list: profile_functions::<Infallible, _, _>(self.bool_list_functions),
            nil_list: profile_functions::<Infallible, _, _>(self.nil_list_functions),
            tuple_list: profile_functions::<Infallible, _, _>(self.tuple_list_functions),
            parameter_list_list: profile_functions::<Infallible, _, _>(
                self.parameter_list_list_functions,
            ),
            list_list: profile_functions::<Infallible, _, _>(self.list_list_functions),
            function_list: profile_functions::<Infallible, _, _>(self.function_list_functions),
            int_function_functions: profile_functions::<Infallible, _, _>(
                self.int_function_functions,
            ),
            float_function_functions: profile_functions::<Infallible, _, _>(
                self.float_function_functions,
            ),
            string_function_functions: profile_functions::<Infallible, _, _>(
                self.string_function_functions,
            ),
            bit_array_function_functions: profile_functions::<Infallible, _, _>(
                self.bit_array_function_functions,
            ),
            utf_codepoint_function_functions: profile_functions::<Infallible, _, _>(
                self.utf_codepoint_function_functions,
            ),
            custom_function_functions: profile_functions::<Infallible, _, _>(
                self.custom_function_functions,
            ),
            external_function_functions: erase_plain_return_family(
                self.external_function_functions,
            ),
            bool_function_functions: profile_functions::<Infallible, _, _>(
                self.bool_function_functions,
            ),
            nil_function_functions: profile_functions::<Infallible, _, _>(
                self.nil_function_functions,
            ),
            tuple_function_functions: profile_functions::<Infallible, _, _>(
                self.tuple_function_functions,
            ),
            generic_function_functions: profile_functions::<Infallible, _, _>(
                self.generic_function_functions,
            ),
            never_function_functions: profile_functions::<Infallible, _, _>(
                self.never_function_functions,
            ),
            parameter_list_function_functions: profile_functions::<Infallible, _, _>(
                self.parameter_list_function_functions,
            ),
            parameter_list_list_function_functions: profile_functions::<Infallible, _, _>(
                self.parameter_list_list_function_functions,
            ),
            int_list_function_functions: profile_functions::<Infallible, _, _>(
                self.int_list_function_functions,
            ),
            string_list_function_functions: profile_functions::<Infallible, _, _>(
                self.string_list_function_functions,
            ),
            bit_array_list_function_functions: profile_functions::<Infallible, _, _>(
                self.bit_array_list_function_functions,
            ),
            utf_codepoint_list_function_functions: profile_functions::<Infallible, _, _>(
                self.utf_codepoint_list_function_functions,
            ),
            custom_list_function_functions: profile_functions::<Infallible, _, _>(
                self.custom_list_function_functions,
            ),
            external_list_function_functions: erase_plain_return_family(
                self.external_list_function_functions,
            ),
            float_list_function_functions: profile_functions::<Infallible, _, _>(
                self.float_list_function_functions,
            ),
            bool_list_function_functions: profile_functions::<Infallible, _, _>(
                self.bool_list_function_functions,
            ),
            nil_list_function_functions: profile_functions::<Infallible, _, _>(
                self.nil_list_function_functions,
            ),
            tuple_list_function_functions: profile_functions::<Infallible, _, _>(
                self.tuple_list_function_functions,
            ),
            list_list_function_functions: profile_functions::<Infallible, _, _>(
                self.list_list_function_functions,
            ),
            function_list_function_functions: profile_functions::<Infallible, _, _>(
                self.function_list_function_functions,
            ),
            function_function_functions: profile_functions::<Infallible, _, _>(
                self.function_function_functions,
            ),
        }
    }

    pub(super) fn profile_hosted(self) -> AdditionalFunctions<HostedExecutionProfile> {
        AdditionalFunctions {
            never: profile_never_functions::<HostedExecutionProfile>(self.never_functions),
            custom: profile_functions::<HostedExecutionProfile, _, _>(self.custom_functions),
            external: profile_functions::<HostedExecutionProfile, _, _>(self.external_functions),
            int: profile_functions::<HostedExecutionProfile, _, _>(self.int_functions),
            float: profile_functions::<HostedExecutionProfile, _, _>(self.float_functions),
            string: profile_functions::<HostedExecutionProfile, _, _>(self.string_functions),
            bit_array: profile_functions::<HostedExecutionProfile, _, _>(self.bit_array_functions),
            utf_codepoint: profile_functions::<HostedExecutionProfile, _, _>(
                self.utf_codepoint_functions,
            ),
            bool: profile_functions::<HostedExecutionProfile, _, _>(self.bool_functions),
            nil: profile_functions::<HostedExecutionProfile, _, _>(self.nil_functions),
            tuple: profile_functions::<HostedExecutionProfile, _, _>(self.tuple_functions),
            parameter_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.parameter_list_functions,
            ),
            int_list: profile_functions::<HostedExecutionProfile, _, _>(self.int_list_functions),
            string_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.string_list_functions,
            ),
            bit_array_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.bit_array_list_functions,
            ),
            utf_codepoint_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.utf_codepoint_list_functions,
            ),
            custom_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.custom_list_functions,
            ),
            external_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.external_list_functions,
            ),
            float_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.float_list_functions,
            ),
            bool_list: profile_functions::<HostedExecutionProfile, _, _>(self.bool_list_functions),
            nil_list: profile_functions::<HostedExecutionProfile, _, _>(self.nil_list_functions),
            tuple_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.tuple_list_functions,
            ),
            parameter_list_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.parameter_list_list_functions,
            ),
            list_list: profile_functions::<HostedExecutionProfile, _, _>(self.list_list_functions),
            function_list: profile_functions::<HostedExecutionProfile, _, _>(
                self.function_list_functions,
            ),
            int_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.int_function_functions,
            ),
            float_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.float_function_functions,
            ),
            string_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.string_function_functions,
            ),
            bit_array_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.bit_array_function_functions,
            ),
            utf_codepoint_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.utf_codepoint_function_functions,
            ),
            custom_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.custom_function_functions,
            ),
            external_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.external_function_functions,
            ),
            bool_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.bool_function_functions,
            ),
            nil_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.nil_function_functions,
            ),
            tuple_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.tuple_function_functions,
            ),
            generic_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.generic_function_functions,
            ),
            never_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.never_function_functions,
            ),
            parameter_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.parameter_list_function_functions,
            ),
            parameter_list_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.parameter_list_list_function_functions,
            ),
            int_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.int_list_function_functions,
            ),
            string_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.string_list_function_functions,
            ),
            bit_array_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.bit_array_list_function_functions,
            ),
            utf_codepoint_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.utf_codepoint_list_function_functions,
            ),
            custom_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.custom_list_function_functions,
            ),
            external_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.external_list_function_functions,
            ),
            float_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.float_list_function_functions,
            ),
            bool_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.bool_list_function_functions,
            ),
            nil_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.nil_list_function_functions,
            ),
            tuple_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.tuple_list_function_functions,
            ),
            list_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.list_list_function_functions,
            ),
            function_list_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.function_list_function_functions,
            ),
            function_function_functions: profile_functions::<HostedExecutionProfile, _, _>(
                self.function_function_functions,
            ),
        }
    }

    pub(super) fn finish_profile<Profile>(
        mut functions: AdditionalFunctions<Profile>,
        additional: AdditionalFunctions<Profile>,
    ) -> SpecializationOutcome<Box<FunctionTables<Profile>>>
    where
        Profile: ExecutionProfile,
    {
        let mut erased = HashSet::new();
        let int_functions = std::mem::take(&mut functions.int)
            .into_iter()
            .chain(additional.int)
            .collect();
        let float_functions = std::mem::take(&mut functions.float)
            .into_iter()
            .chain(additional.float)
            .collect();
        let string_functions = std::mem::take(&mut functions.string)
            .into_iter()
            .chain(additional.string)
            .collect();
        let bit_array_functions = std::mem::take(&mut functions.bit_array)
            .into_iter()
            .chain(additional.bit_array)
            .collect();
        let utf_codepoint_functions = std::mem::take(&mut functions.utf_codepoint)
            .into_iter()
            .chain(additional.utf_codepoint)
            .collect();
        let bool_functions = std::mem::take(&mut functions.bool)
            .into_iter()
            .chain(additional.bool)
            .collect();
        let nil_functions = std::mem::take(&mut functions.nil)
            .into_iter()
            .chain(additional.nil)
            .collect();
        let custom_functions = std::mem::take(&mut functions.custom)
            .into_iter()
            .chain(additional.custom)
            .collect();
        let external_functions = std::mem::take(&mut functions.external)
            .into_iter()
            .chain(additional.external)
            .collect();
        let tuple_functions = std::mem::take(&mut functions.tuple)
            .into_iter()
            .chain(additional.tuple)
            .collect();
        let never_functions = std::mem::take(&mut functions.never)
            .into_iter()
            .chain(additional.never)
            .collect();
        let tables = FunctionTables {
            value_returns: ValueFunctionTables {
                never_functions: sort_functions(never_functions, &mut erased),
                int_functions: sort_inhabited(int_functions, |index| *index, &mut erased)
                    .into_iter()
                    .map(|(_, function)| function)
                    .collect(),
                float_functions: sort_functions(float_functions, &mut erased),
                string_functions: sort_functions(string_functions, &mut erased),
                bit_array_functions: sort_functions(bit_array_functions, &mut erased),
                utf_codepoint_functions: sort_functions(utf_codepoint_functions, &mut erased),
                custom_functions: sort_functions(custom_functions, &mut erased),
                external_functions: sort_functions(external_functions, &mut erased),
                bool_functions: sort_inhabited(bool_functions, |index| *index, &mut erased)
                    .into_iter()
                    .map(|(_, function)| function)
                    .collect(),
                nil_functions: sort_functions(nil_functions, &mut erased),
                tuple_functions: sort_functions(tuple_functions, &mut erased),
            },
            list_returns: ListFunctionTables {
                parameter_list_functions: sort_list_functions(
                    functions
                        .parameter_list
                        .into_iter()
                        .chain(additional.parameter_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                int_list_functions: sort_list_functions(
                    functions
                        .int_list
                        .into_iter()
                        .chain(additional.int_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                string_list_functions: sort_list_functions(
                    functions
                        .string_list
                        .into_iter()
                        .chain(additional.string_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                bit_array_list_functions: sort_list_functions(
                    functions
                        .bit_array_list
                        .into_iter()
                        .chain(additional.bit_array_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                utf_codepoint_list_functions: sort_list_functions(
                    functions
                        .utf_codepoint_list
                        .into_iter()
                        .chain(additional.utf_codepoint_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                custom_list_functions: sort_list_functions(
                    functions
                        .custom_list
                        .into_iter()
                        .chain(additional.custom_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                external_list_functions: sort_list_functions(
                    functions
                        .external_list
                        .into_iter()
                        .chain(additional.external_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                float_list_functions: sort_list_functions(
                    functions
                        .float_list
                        .into_iter()
                        .chain(additional.float_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                bool_list_functions: sort_list_functions(
                    functions
                        .bool_list
                        .into_iter()
                        .chain(additional.bool_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                nil_list_functions: sort_list_functions(
                    functions
                        .nil_list
                        .into_iter()
                        .chain(additional.nil_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                tuple_list_functions: sort_list_functions(
                    functions
                        .tuple_list
                        .into_iter()
                        .chain(additional.tuple_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                parameter_list_list_functions: sort_list_functions(
                    functions
                        .parameter_list_list
                        .into_iter()
                        .chain(additional.parameter_list_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                list_list_functions: sort_list_functions(
                    functions
                        .list_list
                        .into_iter()
                        .chain(additional.list_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                function_list_functions: sort_list_functions(
                    functions
                        .function_list
                        .into_iter()
                        .chain(additional.function_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
            },
            function_returns: FunctionFunctionTables {
                int_function_functions: sort_functions(
                    functions
                        .int_function_functions
                        .into_iter()
                        .chain(additional.int_function_functions)
                        .collect(),
                    &mut erased,
                ),
                float_function_functions: sort_functions(
                    functions
                        .float_function_functions
                        .into_iter()
                        .chain(additional.float_function_functions)
                        .collect(),
                    &mut erased,
                ),
                string_function_functions: sort_functions(
                    functions
                        .string_function_functions
                        .into_iter()
                        .chain(additional.string_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bit_array_function_functions: sort_functions(
                    functions
                        .bit_array_function_functions
                        .into_iter()
                        .chain(additional.bit_array_function_functions)
                        .collect(),
                    &mut erased,
                ),
                utf_codepoint_function_functions: sort_functions(
                    functions
                        .utf_codepoint_function_functions
                        .into_iter()
                        .chain(additional.utf_codepoint_function_functions)
                        .collect(),
                    &mut erased,
                ),
                custom_function_functions: sort_functions(
                    functions
                        .custom_function_functions
                        .into_iter()
                        .chain(additional.custom_function_functions)
                        .collect(),
                    &mut erased,
                ),
                external_function_functions: sort_functions(
                    functions
                        .external_function_functions
                        .into_iter()
                        .chain(additional.external_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bool_function_functions: sort_functions(
                    functions
                        .bool_function_functions
                        .into_iter()
                        .chain(additional.bool_function_functions)
                        .collect(),
                    &mut erased,
                ),
                nil_function_functions: sort_functions(
                    functions
                        .nil_function_functions
                        .into_iter()
                        .chain(additional.nil_function_functions)
                        .collect(),
                    &mut erased,
                ),
                tuple_function_functions: sort_functions(
                    functions
                        .tuple_function_functions
                        .into_iter()
                        .chain(additional.tuple_function_functions)
                        .collect(),
                    &mut erased,
                ),
                generic_function_functions: sort_functions(
                    functions
                        .generic_function_functions
                        .into_iter()
                        .chain(additional.generic_function_functions)
                        .collect(),
                    &mut erased,
                ),
                never_function_functions: sort_functions(
                    functions
                        .never_function_functions
                        .into_iter()
                        .chain(additional.never_function_functions)
                        .collect(),
                    &mut erased,
                ),
                parameter_list_function_functions: sort_functions(
                    functions
                        .parameter_list_function_functions
                        .into_iter()
                        .chain(additional.parameter_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                parameter_list_list_function_functions: sort_functions(
                    functions
                        .parameter_list_list_function_functions
                        .into_iter()
                        .chain(additional.parameter_list_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                int_list_function_functions: sort_functions(
                    functions
                        .int_list_function_functions
                        .into_iter()
                        .chain(additional.int_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                string_list_function_functions: sort_functions(
                    functions
                        .string_list_function_functions
                        .into_iter()
                        .chain(additional.string_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bit_array_list_function_functions: sort_functions(
                    functions
                        .bit_array_list_function_functions
                        .into_iter()
                        .chain(additional.bit_array_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                utf_codepoint_list_function_functions: sort_functions(
                    functions
                        .utf_codepoint_list_function_functions
                        .into_iter()
                        .chain(additional.utf_codepoint_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                custom_list_function_functions: sort_functions(
                    functions
                        .custom_list_function_functions
                        .into_iter()
                        .chain(additional.custom_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                external_list_function_functions: sort_functions(
                    functions
                        .external_list_function_functions
                        .into_iter()
                        .chain(additional.external_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                float_list_function_functions: sort_functions(
                    functions
                        .float_list_function_functions
                        .into_iter()
                        .chain(additional.float_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bool_list_function_functions: sort_functions(
                    functions
                        .bool_list_function_functions
                        .into_iter()
                        .chain(additional.bool_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                nil_list_function_functions: sort_functions(
                    functions
                        .nil_list_function_functions
                        .into_iter()
                        .chain(additional.nil_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                tuple_list_function_functions: sort_functions(
                    functions
                        .tuple_list_function_functions
                        .into_iter()
                        .chain(additional.tuple_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                list_list_function_functions: sort_functions(
                    functions
                        .list_list_function_functions
                        .into_iter()
                        .chain(additional.list_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                function_list_function_functions: sort_functions(
                    functions
                        .function_list_function_functions
                        .into_iter()
                        .chain(additional.function_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                function_function_functions: sort_functions(
                    functions
                        .function_function_functions
                        .into_iter()
                        .chain(additional.function_function_functions)
                        .collect(),
                    &mut erased,
                ),
            },
        };
        SpecializationOutcome::complete_unless_erased(Box::new(tables), erased)
    }
}

pub(super) fn push_core_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    signature: &CoreListFunctionFunctionSignature,
    function: LoweredFunction<CoreListFunctionFunctionBody>,
) {
    match signature.return_ {
        CoreListFunctionReturn::Parameter(_) => functions
            .parameter_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::ParameterList(_) => functions
            .parameter_list_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::Int(_) => functions
            .int_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::String(_) => {
            functions
                .string_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::BitArray(_) => {
            functions
                .bit_array_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::UtfCodepoint(_) => {
            functions
                .utf_codepoint_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Custom(_) => {
            functions
                .custom_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Float(_) => {
            functions
                .float_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Bool(_) => {
            functions
                .bool_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Nil(_) => {
            functions
                .nil_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Tuple(_) => {
            functions
                .tuple_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::List(_) => functions
            .list_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::Function(_) => {
            functions
                .function_list_function_functions
                .push((index, function));
        }
    }
}

pub(super) fn push_external_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    function: LoweredFunction<ExternalListFunctionFunctionBody>,
) {
    functions
        .external_list_function_functions
        .push((index, function));
}

pub(in crate::plan::execution::lowering) fn function_id(
    shape: &StoredValueShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
    representations: &super::super::specialization::RepresentationContext,
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
    representations: &super::super::specialization::RepresentationContext,
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
    representations: &super::super::specialization::RepresentationContext,
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
    types: &mut super::super::value_type::TypeInterner,
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
            super::super::value_type::NestedListTypeId::Parameter(list_type) => {
                CoreListFunctionReturn::ParameterList(list_type)
            }
            super::super::value_type::NestedListTypeId::Stored(list_type) => {
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
    types: &mut super::super::value_type::TypeInterner,
) -> RuntimeListFunctionId {
    let function =
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
            SpecializedValueShape::BitArray => ListFunctionId::BitArray(
                BitArrayListFunctionId::new(index, types.bit_array_list_type()),
            ),
            SpecializedValueShape::UtfCodepoint => ListFunctionId::UtfCodepoint(
                UtfCodepointListFunctionId::new(index, types.utf_codepoint_list_type()),
            ),
            SpecializedValueShape::Custom(item) => ListFunctionId::Custom(
                CustomListFunctionId::new(index, types.custom_list_type(item)),
            ),
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
                super::super::value_type::NestedListTypeId::Parameter(type_id) => {
                    ListFunctionId::ParameterList(ParameterListListFunctionId::new(index, type_id))
                }
                super::super::value_type::NestedListTypeId::Stored(type_id) => {
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
    types: &mut super::super::value_type::TypeInterner,
    representations: &super::super::specialization::RepresentationContext,
) -> FunctionFunctionId {
    runtime_function_function_id(function, index, types, representations).runtime_id()
}

fn runtime_function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
    representations: &super::super::specialization::RepresentationContext,
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
    types: &mut super::super::value_type::TypeInterner,
) -> RuntimeFunctionFunctionTarget {
    list_function_function_signature(function, item, types).runtime_id(index)
}

trait SealFunctionBody<Profile: ExecutionProfile> {
    type Sealed: ExecutionFunctionBody;

    fn seal(self) -> Representability<Self::Sealed>;
}

trait ProfileIndependentTailCallId {}

impl ProfileIndependentTailCallId for usize {}
impl ProfileIndependentTailCallId for NeverFunctionId {}
impl ProfileIndependentTailCallId for IntFunctionId {}
impl ProfileIndependentTailCallId for FloatFunctionId {}
impl ProfileIndependentTailCallId for StringFunctionId {}
impl ProfileIndependentTailCallId for BitArrayFunctionId {}
impl ProfileIndependentTailCallId for UtfCodepointFunctionId {}
impl ProfileIndependentTailCallId for BoolFunctionId {}
impl ProfileIndependentTailCallId for NilFunctionId {}
impl ProfileIndependentTailCallId for TupleFunctionId {}
impl ProfileIndependentTailCallId for ParameterListFunctionId {}
impl ProfileIndependentTailCallId for IntListFunctionId {}
impl ProfileIndependentTailCallId for FloatListFunctionId {}
impl ProfileIndependentTailCallId for StringListFunctionId {}
impl ProfileIndependentTailCallId for BitArrayListFunctionId {}
impl ProfileIndependentTailCallId for UtfCodepointListFunctionId {}
impl ProfileIndependentTailCallId for CustomListFunctionId {}
impl ProfileIndependentTailCallId for ExternalListFunctionId {}
impl ProfileIndependentTailCallId for BoolListFunctionId {}
impl ProfileIndependentTailCallId for NilListFunctionId {}
impl ProfileIndependentTailCallId for TupleListFunctionId {}
impl ProfileIndependentTailCallId for ParameterListListFunctionId {}
impl ProfileIndependentTailCallId for ListListFunctionId {}
impl ProfileIndependentTailCallId for FunctionListFunctionId {}
impl ProfileIndependentTailCallId for IntFunctionFunctionId {}
impl ProfileIndependentTailCallId for FloatFunctionFunctionId {}
impl ProfileIndependentTailCallId for StringFunctionFunctionId {}
impl ProfileIndependentTailCallId for BitArrayFunctionFunctionId {}
impl ProfileIndependentTailCallId for UtfCodepointFunctionFunctionId {}
impl ProfileIndependentTailCallId for GenericFunctionFunctionId {}
impl ProfileIndependentTailCallId for NeverFunctionFunctionId {}
impl ProfileIndependentTailCallId for BoolFunctionFunctionId {}
impl ProfileIndependentTailCallId for NilFunctionFunctionId {}
impl ProfileIndependentTailCallId for TupleFunctionFunctionId {}
impl ProfileIndependentTailCallId for ProfiledListFunctionFunctionId<Infallible> {}

trait ProfileIndependentTailCall {}

impl<Id: ProfileIndependentTailCallId> ProfileIndependentTailCall
    for crate::plan::FunctionCallTarget<Id>
{
}

impl<Body> SealFunctionBody<HostedExecutionProfile> for Body
where
    Body: ExecutionFunctionBody,
{
    type Sealed = Body;

    fn seal(self) -> Representability<Self::Sealed> {
        Representability::Inhabited(self)
    }
}

impl<Return, TailCall> SealFunctionBody<Infallible>
    for ProfiledFunctionBody<Return, TailCall, HostedExecutionGraph>
where
    TailCall: ProfileIndependentTailCall,
{
    type Sealed = ProfiledFunctionBody<Return, TailCall, Infallible>;

    fn seal(self) -> Representability<Self::Sealed> {
        seal_plain_profiled_function_body(self)
    }
}

impl SealFunctionBody<Infallible> for ProfiledCustomFunctionBody<HostedExecutionGraph> {
    type Sealed = ProfiledCustomFunctionBody<Infallible>;

    fn seal(self) -> Representability<Self::Sealed> {
        let (signature_shape, body_shape, body) = self.into_parts();
        seal_plain_profiled_function_body(body)
            .map(|body| ProfiledCustomFunctionBody::from_parts(signature_shape, body_shape, body))
    }
}

impl<Body> SealFunctionBody<Infallible> for TypedFunctionBody<Body>
where
    Body: SealFunctionBody<Infallible>,
{
    type Sealed = TypedFunctionBody<Body::Sealed>;

    fn seal(self) -> Representability<Self::Sealed> {
        let (shape, body) = self.into_parts();
        body.seal().map(|body| TypedFunctionBody::new(shape, body))
    }
}

impl SealFunctionBody<Infallible> for ProfiledCustomFunctionFunctionBody<HostedExecutionGraph> {
    type Sealed = ProfiledCustomFunctionFunctionBody<Infallible>;

    fn seal(self) -> Representability<Self::Sealed> {
        let (shape, type_, body) = self.into_parts();
        seal_plain_profiled_function_body(body)
            .map(|body| ProfiledCustomFunctionFunctionBody::from_parts(shape, type_, body))
    }
}

impl SealFunctionBody<Infallible> for ProfiledFunctionFunctionFunctionBody<HostedExecutionGraph> {
    type Sealed = ProfiledFunctionFunctionFunctionBody<Infallible>;

    fn seal(self) -> Representability<Self::Sealed> {
        let (shape, type_, body) = self.into_parts();
        seal_plain_profiled_function_body(body)
            .map(|body| ProfiledFunctionFunctionFunctionBody::from_parts(shape, type_, body))
    }
}

fn seal_plain_profiled_function_body<Return, TailCall>(
    body: ProfiledFunctionBody<Return, TailCall, HostedExecutionGraph>,
) -> Representability<ProfiledFunctionBody<Return, TailCall, Infallible>> {
    let (graph, exits) = body.into_parts();
    crate::plan::execution::lowering::graph::seal_plain_block_graph(graph)
        .map(|graph| ProfiledFunctionBody::from_parts(graph, exits.into_vec()))
}

fn profile_functions<Profile, Id, Body>(
    functions: Vec<(Id, LoweredFunction<Body>)>,
) -> ProfiledLoweredFunctionTable<Id, Profile, Body::Sealed>
where
    Profile: ExecutionProfile,
    Body: SealFunctionBody<Profile>,
{
    functions
        .into_iter()
        .map(|(id, function)| {
            (
                id,
                LoweredSpecialization {
                    specialization: function.specialization,
                    value: function.value.and_then(|function| {
                        let (entry, body) = function.into_parts();
                        body.seal()
                            .map(|body| Profile::graph(ExecutableFunction::from_parts(entry, body)))
                    }),
                },
            )
        })
        .collect()
}

fn erase_plain_return_family<Id, Body, SealedBody>(
    functions: Vec<(Id, LoweredFunction<Body>)>,
) -> ProfiledLoweredFunctionTable<Id, Infallible, SealedBody>
where
    SealedBody: ExecutionFunctionBody,
{
    functions
        .into_iter()
        .map(|(id, function)| {
            (
                id,
                LoweredSpecialization {
                    specialization: function.specialization,
                    value: Representability::Uninhabited,
                },
            )
        })
        .collect()
}

fn profile_never_functions<Profile>(
    functions: Vec<(usize, LoweredFunction<NeverFunctionBody>)>,
) -> ProfiledLoweredNeverFunctionTable<Profile>
where
    Profile: ExecutionProfile,
    NeverFunctionBody: SealFunctionBody<Profile, Sealed = ExecutionNeverFunctionBody<Profile>>,
{
    functions
        .into_iter()
        .map(|(id, function)| {
            (
                id,
                LoweredSpecialization {
                    specialization: function.specialization,
                    value: function.value.and_then(|function| {
                        let (entry, body) = function.into_parts();
                        body.seal().map(|body| {
                            Profile::never_graph(ExecutableFunction::from_parts(entry, body))
                        })
                    }),
                },
            )
        })
        .collect()
}

fn sort_functions<Value>(
    functions: Vec<(usize, LoweredSpecialization<Value>)>,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<Value> {
    sort_inhabited(functions, |index| *index, erased)
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}

fn sort_list_functions<Id, Value>(
    functions: Vec<(Id, LoweredSpecialization<Value>)>,
    index: fn(&Id) -> usize,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<(Id, Value)> {
    sort_inhabited(functions, index, erased)
}

fn sort_inhabited<Id, Value>(
    mut values: Vec<(Id, LoweredSpecialization<Value>)>,
    index: fn(&Id) -> usize,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<(Id, Value)> {
    values.sort_by_key(|(id, _)| index(id));
    let mut lowered = Vec::new();
    for (id, specialization) in values {
        match specialization.value {
            Representability::Inhabited(value) => lowered.push((id, value)),
            Representability::Uninhabited => {
                erased.insert(specialization.specialization);
            }
        }
    }
    lowered
}

#[cfg(test)]
mod tests {
    use super::{
        LoweredFunction, LoweredSpecialization, erase_plain_return_family, sort_inhabited,
    };
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::function::{ExecutionExternalFunctionBody, ExternalFunctionBody};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use std::collections::HashSet;
    use std::convert::Infallible;

    #[test]
    fn lowered_specializations_sort_by_id_and_record_erased_keys() {
        let erased_key = key(2);
        let mut erased = HashSet::new();

        let values = sort_inhabited(
            vec![
                inhabited(3, key(3), "value#3"),
                (
                    2,
                    LoweredSpecialization {
                        specialization: erased_key.clone(),
                        value: Representability::Uninhabited,
                    },
                ),
                inhabited(1, key(1), "value#1"),
            ],
            |index| *index,
            &mut erased,
        );

        assert_eq!(values, vec![(1, "value#1"), (3, "value#3")]);
        assert_eq!(erased, HashSet::from([erased_key]));
    }

    #[test]
    fn plain_external_return_family_preserves_erased_specialization_identity() {
        let specialization = key(5);
        let functions: Vec<(usize, LoweredFunction<ExternalFunctionBody>)> = vec![(
            7,
            LoweredSpecialization {
                specialization: specialization.clone(),
                value: Representability::Uninhabited,
            },
        )];

        let profiled =
            erase_plain_return_family::<_, _, ExecutionExternalFunctionBody<Infallible>>(functions);
        assert_eq!(profiled.len(), 1);
        let (id, function) = &profiled[0];

        assert_eq!(*id, 7);
        assert_eq!(function.specialization, specialization);
        assert_eq!(
            std::mem::discriminant(&function.value),
            std::mem::discriminant(&Representability::Uninhabited),
        );
    }

    fn inhabited(
        index: usize,
        specialization: SpecializationKey,
        value: &'static str,
    ) -> (usize, LoweredSpecialization<&'static str>) {
        (
            index,
            LoweredSpecialization {
                specialization,
                value: Representability::Inhabited(value),
            },
        )
    }

    fn key(index: usize) -> SpecializationKey {
        SpecializationKey::monomorphic(FunctionTemplateId::new(index))
    }
}
