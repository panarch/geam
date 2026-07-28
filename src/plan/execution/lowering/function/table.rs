mod host;

use crate::plan::execution;
use crate::plan::execution::function::FunctionTables;
use crate::plan::execution::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionId, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionBody,
    BoolListFunctionId, CustomFunctionBody, CustomFunctionFunctionBody, CustomListFunctionBody,
    CustomListFunctionId, FloatFunctionBody, FloatFunctionFunctionBody, FloatFunctionFunctionId,
    FloatFunctionId, FloatListFunctionBody, FloatListFunctionId, FunctionFunctionFunctionBody,
    FunctionFunctionId, FunctionListFunctionBody, FunctionListFunctionId,
    GenericFunctionFunctionBody, IntFunctionBody, IntFunctionFunctionBody, IntFunctionFunctionId,
    IntFunctionId, IntListFunctionBody, IntListFunctionId, ListFunctionFunctionBody,
    ListFunctionFunctionId, ListFunctionId, ListListFunctionBody, ListListFunctionId,
    NeverFunctionBody, NeverFunctionFunctionBody, NilFunctionBody, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionBody, NilListFunctionId,
    ParameterListFunctionBody, ParameterListFunctionId, ParameterListListFunctionBody,
    ParameterListListFunctionId, RuntimeFunctionId, StringFunctionBody, StringFunctionFunctionBody,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionBody, StringListFunctionId,
    TupleFunctionBody, TupleFunctionFunctionBody, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionBody, TupleListFunctionId, UtfCodepointFunctionBody,
    UtfCodepointFunctionFunctionBody, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionBody, UtfCodepointListFunctionId,
};
use crate::plan::execution::function::{
    ExecutableFunction, ExecutionFunction, ExecutionFunctionBody, ExecutionProfile,
    FunctionFunctionTables, ListFunctionTables, ValueFunctionTables,
};
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
pub(in crate::plan::execution::lowering) struct FunctionTableBuilder {
    pub(super) never_functions: Vec<(usize, LoweredFunction<NeverFunctionBody>)>,
    pub(super) int_functions: Vec<(usize, LoweredFunction<IntFunctionBody>)>,
    pub(super) float_functions: Vec<(usize, LoweredFunction<FloatFunctionBody>)>,
    pub(super) string_functions: Vec<(usize, LoweredFunction<StringFunctionBody>)>,
    pub(super) bit_array_functions: Vec<(usize, LoweredFunction<BitArrayFunctionBody>)>,
    pub(super) utf_codepoint_functions: Vec<(usize, LoweredFunction<UtfCodepointFunctionBody>)>,
    pub(super) custom_functions: Vec<(usize, LoweredFunction<CustomFunctionBody>)>,
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
    pub(super) bool_function_functions: Vec<(usize, LoweredFunction<BoolFunctionFunctionBody>)>,
    pub(super) nil_function_functions: Vec<(usize, LoweredFunction<NilFunctionFunctionBody>)>,
    pub(super) tuple_function_functions: Vec<(usize, LoweredFunction<TupleFunctionFunctionBody>)>,
    pub(super) generic_function_functions:
        Vec<(usize, LoweredFunction<GenericFunctionFunctionBody>)>,
    pub(super) never_function_functions: Vec<(usize, LoweredFunction<NeverFunctionFunctionBody>)>,
    pub(super) parameter_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) parameter_list_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) int_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) string_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) bit_array_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) utf_codepoint_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) custom_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) float_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) bool_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) nil_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) tuple_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) list_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) function_list_function_functions:
        Vec<(usize, LoweredFunction<ListFunctionFunctionBody>)>,
    pub(super) function_function_functions:
        Vec<(usize, LoweredFunction<FunctionFunctionFunctionBody>)>,
}

pub(in crate::plan::execution::lowering) struct AdditionalFunctions<Profile: ExecutionProfile> {
    pub(in crate::plan::execution::lowering) never: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, NeverFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) custom: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, CustomFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) int: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, IntFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) float: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, FloatFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) string: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, StringFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bit_array: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, BitArrayFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) utf_codepoint: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, UtfCodepointFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bool: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, BoolFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) nil: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, NilFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) tuple: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, TupleFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) parameter_list: Vec<(
        ParameterListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, ParameterListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) int_list: Vec<(
        IntListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, IntListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) string_list: Vec<(
        StringListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, StringListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bit_array_list: Vec<(
        BitArrayListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, BitArrayListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) utf_codepoint_list: Vec<(
        UtfCodepointListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, UtfCodepointListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) custom_list: Vec<(
        CustomListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, CustomListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) float_list: Vec<(
        FloatListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, FloatListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bool_list: Vec<(
        BoolListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, BoolListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) nil_list: Vec<(
        NilListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, NilListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) tuple_list: Vec<(
        TupleListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, TupleListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) parameter_list_list: Vec<(
        ParameterListListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, ParameterListListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) list_list: Vec<(
        ListListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, ListListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) function_list: Vec<(
        FunctionListFunctionId,
        LoweredSpecialization<ExecutionFunction<Profile, FunctionListFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) int_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, IntFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) float_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, FloatFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) string_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, StringFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bit_array_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, BitArrayFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) utf_codepoint_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, UtfCodepointFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) custom_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, CustomFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bool_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, BoolFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) nil_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, NilFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) tuple_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, TupleFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) generic_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, GenericFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) never_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, NeverFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) parameter_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) parameter_list_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) int_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) string_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bit_array_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) utf_codepoint_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) custom_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) float_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) bool_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) nil_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) tuple_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) list_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) function_list_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, ListFunctionFunctionBody>>,
    )>,
    pub(in crate::plan::execution::lowering) function_function_functions: Vec<(
        usize,
        LoweredSpecialization<ExecutionFunction<Profile, FunctionFunctionFunctionBody>>,
    )>,
}

impl<Profile: ExecutionProfile> AdditionalFunctions<Profile> {
    fn empty() -> Self {
        Self {
            never: Vec::new(),
            custom: Vec::new(),
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
        self.finish_profile(AdditionalFunctions::empty())
    }

    pub(super) fn finish_profile<Profile: ExecutionProfile>(
        mut self,
        additional: AdditionalFunctions<Profile>,
    ) -> SpecializationOutcome<Box<FunctionTables<Profile>>> {
        let mut erased = HashSet::new();
        let int_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.int_functions))
                .into_iter()
                .chain(additional.int)
                .collect();
        let float_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.float_functions))
                .into_iter()
                .chain(additional.float)
                .collect();
        let string_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.string_functions))
                .into_iter()
                .chain(additional.string)
                .collect();
        let bit_array_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.bit_array_functions))
                .into_iter()
                .chain(additional.bit_array)
                .collect();
        let utf_codepoint_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.utf_codepoint_functions))
                .into_iter()
                .chain(additional.utf_codepoint)
                .collect();
        let bool_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.bool_functions))
                .into_iter()
                .chain(additional.bool)
                .collect();
        let nil_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.nil_functions))
                .into_iter()
                .chain(additional.nil)
                .collect();
        let custom_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.custom_functions))
                .into_iter()
                .chain(additional.custom)
                .collect();
        let tuple_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.tuple_functions))
                .into_iter()
                .chain(additional.tuple)
                .collect();
        let never_functions =
            profile_functions::<Profile, _, _>(std::mem::take(&mut self.never_functions))
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
                bool_functions: sort_inhabited(bool_functions, |index| *index, &mut erased)
                    .into_iter()
                    .map(|(_, function)| function)
                    .collect(),
                nil_functions: sort_functions(nil_functions, &mut erased),
                tuple_functions: sort_functions(tuple_functions, &mut erased),
            },
            list_returns: ListFunctionTables {
                parameter_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.parameter_list_functions)
                        .into_iter()
                        .chain(additional.parameter_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                int_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.int_list_functions)
                        .into_iter()
                        .chain(additional.int_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                string_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.string_list_functions)
                        .into_iter()
                        .chain(additional.string_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                bit_array_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.bit_array_list_functions)
                        .into_iter()
                        .chain(additional.bit_array_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                utf_codepoint_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.utf_codepoint_list_functions)
                        .into_iter()
                        .chain(additional.utf_codepoint_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                custom_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.custom_list_functions)
                        .into_iter()
                        .chain(additional.custom_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                float_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.float_list_functions)
                        .into_iter()
                        .chain(additional.float_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                bool_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.bool_list_functions)
                        .into_iter()
                        .chain(additional.bool_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                nil_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.nil_list_functions)
                        .into_iter()
                        .chain(additional.nil_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                tuple_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.tuple_list_functions)
                        .into_iter()
                        .chain(additional.tuple_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                parameter_list_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.parameter_list_list_functions)
                        .into_iter()
                        .chain(additional.parameter_list_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                list_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.list_list_functions)
                        .into_iter()
                        .chain(additional.list_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
                function_list_functions: sort_list_functions(
                    profile_functions::<Profile, _, _>(self.function_list_functions)
                        .into_iter()
                        .chain(additional.function_list)
                        .collect(),
                    |id| id.index(),
                    &mut erased,
                ),
            },
            function_returns: FunctionFunctionTables {
                int_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.int_function_functions)
                        .into_iter()
                        .chain(additional.int_function_functions)
                        .collect(),
                    &mut erased,
                ),
                float_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.float_function_functions)
                        .into_iter()
                        .chain(additional.float_function_functions)
                        .collect(),
                    &mut erased,
                ),
                string_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.string_function_functions)
                        .into_iter()
                        .chain(additional.string_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bit_array_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.bit_array_function_functions)
                        .into_iter()
                        .chain(additional.bit_array_function_functions)
                        .collect(),
                    &mut erased,
                ),
                utf_codepoint_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.utf_codepoint_function_functions)
                        .into_iter()
                        .chain(additional.utf_codepoint_function_functions)
                        .collect(),
                    &mut erased,
                ),
                custom_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.custom_function_functions)
                        .into_iter()
                        .chain(additional.custom_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bool_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.bool_function_functions)
                        .into_iter()
                        .chain(additional.bool_function_functions)
                        .collect(),
                    &mut erased,
                ),
                nil_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.nil_function_functions)
                        .into_iter()
                        .chain(additional.nil_function_functions)
                        .collect(),
                    &mut erased,
                ),
                tuple_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.tuple_function_functions)
                        .into_iter()
                        .chain(additional.tuple_function_functions)
                        .collect(),
                    &mut erased,
                ),
                generic_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.generic_function_functions)
                        .into_iter()
                        .chain(additional.generic_function_functions)
                        .collect(),
                    &mut erased,
                ),
                never_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.never_function_functions)
                        .into_iter()
                        .chain(additional.never_function_functions)
                        .collect(),
                    &mut erased,
                ),
                parameter_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.parameter_list_function_functions)
                        .into_iter()
                        .chain(additional.parameter_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                parameter_list_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.parameter_list_list_function_functions)
                        .into_iter()
                        .chain(additional.parameter_list_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                int_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.int_list_function_functions)
                        .into_iter()
                        .chain(additional.int_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                string_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.string_list_function_functions)
                        .into_iter()
                        .chain(additional.string_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bit_array_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.bit_array_list_function_functions)
                        .into_iter()
                        .chain(additional.bit_array_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                utf_codepoint_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.utf_codepoint_list_function_functions)
                        .into_iter()
                        .chain(additional.utf_codepoint_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                custom_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.custom_list_function_functions)
                        .into_iter()
                        .chain(additional.custom_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                float_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.float_list_function_functions)
                        .into_iter()
                        .chain(additional.float_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                bool_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.bool_list_function_functions)
                        .into_iter()
                        .chain(additional.bool_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                nil_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.nil_list_function_functions)
                        .into_iter()
                        .chain(additional.nil_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                tuple_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.tuple_list_function_functions)
                        .into_iter()
                        .chain(additional.tuple_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                list_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.list_list_function_functions)
                        .into_iter()
                        .chain(additional.list_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                function_list_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.function_list_function_functions)
                        .into_iter()
                        .chain(additional.function_list_function_functions)
                        .collect(),
                    &mut erased,
                ),
                function_function_functions: sort_functions(
                    profile_functions::<Profile, _, _>(self.function_function_functions)
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

pub(super) fn push_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    item: &SpecializedValueShape,
    function: LoweredFunction<ListFunctionFunctionBody>,
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

pub(in crate::plan::execution::lowering) fn function_id(
    shape: &StoredValueShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
    representations: &super::super::specialization::RepresentationContext,
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
            execution::function::CustomFunctionId::new(index, types.custom_value_shape(shape)),
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

pub(in crate::plan::execution::lowering) fn list_function_function_table_family(
    item: &SpecializedValueShape,
) -> FunctionTableFamily {
    executable_function_function_table_family(&StoredValueShape::List(Box::new(item.clone())))
}

pub(in crate::plan::execution::lowering) fn list_function_id(
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
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
    }
}

pub(in crate::plan::execution::lowering) fn function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
    representations: &super::super::specialization::RepresentationContext,
) -> FunctionFunctionId {
    let return_ = match function.representation(representations) {
        FunctionRepresentation::Symbolic => {
            return FunctionFunctionId::Generic(
                execution::function::GenericFunctionFunctionId::new(
                    index,
                    types.generic_function_type(function),
                ),
            );
        }
        FunctionRepresentation::Never(_) => {
            return FunctionFunctionId::Never(execution::function::NeverFunctionFunctionId::new(
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
            FunctionFunctionId::Custom(execution::function::CustomFunctionFunctionId::new(
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
            FunctionFunctionId::Function(execution::function::FunctionFunctionFunctionId::new(
                index,
                types.function_function_type(function.arguments(), &return_),
            ))
        }
    }
}

pub(in crate::plan::execution::lowering) fn list_function_function_id(
    function: &SpecializedFunctionShape,
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::super::value_type::TypeInterner,
) -> ListFunctionFunctionId {
    let type_ = types.function_type(function);

    match item {
        SpecializedValueShape::Parameter(parameter) => ListFunctionFunctionId::Parameter {
            id: execution::function::ParameterListFunctionFunctionId(index),
            type_,
            list_type: types.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => ListFunctionFunctionId::Int {
            id: execution::function::IntListFunctionFunctionId(index),
            type_,
            list_type: types.int_list_type(),
        },
        SpecializedValueShape::String => ListFunctionFunctionId::String {
            id: execution::function::StringListFunctionFunctionId(index),
            type_,
            list_type: types.string_list_type(),
        },
        SpecializedValueShape::BitArray => ListFunctionFunctionId::BitArray {
            id: execution::function::BitArrayListFunctionFunctionId(index),
            type_,
            list_type: types.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => ListFunctionFunctionId::UtfCodepoint {
            id: execution::function::UtfCodepointListFunctionFunctionId(index),
            type_,
            list_type: types.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(item) => ListFunctionFunctionId::Custom {
            id: execution::function::CustomListFunctionFunctionId(index),
            type_,
            list_type: types.custom_list_type(item),
        },
        SpecializedValueShape::Float => ListFunctionFunctionId::Float {
            id: execution::function::FloatListFunctionFunctionId(index),
            type_,
            list_type: types.float_list_type(),
        },
        SpecializedValueShape::Bool => ListFunctionFunctionId::Bool {
            id: execution::function::BoolListFunctionFunctionId(index),
            type_,
            list_type: types.bool_list_type(),
        },
        SpecializedValueShape::Nil => ListFunctionFunctionId::Nil {
            id: execution::function::NilListFunctionFunctionId(index),
            type_,
            list_type: types.nil_list_type(),
        },
        SpecializedValueShape::Tuple(item) => ListFunctionFunctionId::Tuple {
            id: execution::function::TupleListFunctionFunctionId(index),
            type_,
            list_type: types.tuple_list_type(item),
        },
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            super::super::value_type::NestedListTypeId::Parameter(list_type) => {
                ListFunctionFunctionId::ParameterList {
                    id: execution::function::ParameterListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            super::super::value_type::NestedListTypeId::Stored(list_type) => {
                ListFunctionFunctionId::List {
                    id: execution::function::ListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionFunctionId::Function {
            id: execution::function::FunctionListFunctionFunctionId(index),
            type_,
            list_type: types.function_list_type(item),
        },
    }
}

fn profile_functions<Profile, Id, Body>(
    functions: Vec<(Id, LoweredFunction<Body>)>,
) -> Vec<(Id, LoweredSpecialization<ExecutionFunction<Profile, Body>>)>
where
    Profile: ExecutionProfile,
    Body: ExecutionFunctionBody,
{
    functions
        .into_iter()
        .map(|(id, function)| {
            (
                id,
                LoweredSpecialization {
                    specialization: function.specialization,
                    value: function.value.map(Profile::graph),
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
    use super::{LoweredSpecialization, sort_inhabited};
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use std::collections::HashSet;

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
