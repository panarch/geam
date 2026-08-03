use super::{FunctionTableBuilder, LoweredFunction, LoweredSpecialization};
use crate::plan::execution::function::FunctionTables;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
    FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId, FunctionListFunctionId,
    GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId, IntListFunctionId,
    ListListFunctionId, NeverFunctionBody, NeverFunctionFunctionId, NeverFunctionId,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionId, ParameterListFunctionId,
    ParameterListListFunctionId, StringFunctionFunctionId, StringFunctionId, StringListFunctionId,
    TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointListFunctionId,
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
use crate::plan::execution::host::HostedExecutionProfile;
use crate::plan::execution::lowering::SpecializationOutcome;
use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
use std::collections::HashSet;
use std::convert::Infallible;

type ProfiledLoweredFunction<Profile, Body> =
    LoweredSpecialization<ExecutionFunction<Profile, Body>>;
type ProfiledLoweredFunctionTable<Id, Profile, Body> =
    Vec<(Id, ProfiledLoweredFunction<Profile, Body>)>;
type ProfiledLoweredNeverFunctionTable<Profile> = Vec<(
    usize,
    LoweredSpecialization<ExecutionNeverFunction<Profile>>,
)>;

pub(in crate::plan::execution::lowering) struct ProfiledFunctionEntries<Profile: ExecutionProfile> {
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

impl<Profile: ExecutionProfile> ProfiledFunctionEntries<Profile> {
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
        Self::finish_profile(self.profile_plain(), ProfiledFunctionEntries::empty())
    }

    fn profile_plain(self) -> ProfiledFunctionEntries<Infallible> {
        ProfiledFunctionEntries {
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

    pub(super) fn profile_hosted(self) -> ProfiledFunctionEntries<HostedExecutionProfile> {
        ProfiledFunctionEntries {
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
        mut functions: ProfiledFunctionEntries<Profile>,
        additional: ProfiledFunctionEntries<Profile>,
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
    use super::{erase_plain_return_family, sort_inhabited};
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::function::{ExecutionExternalFunctionBody, ExternalFunctionBody};
    use crate::plan::execution::lowering::function::table::{
        LoweredFunction, LoweredSpecialization,
    };
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
