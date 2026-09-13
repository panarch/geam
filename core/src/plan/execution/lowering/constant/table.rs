use super::LoweredConstantValue;
use crate::plan::execution::constant::{
    ConstantId, ProfiledConstantProgram, ProfiledConstantTable,
};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BitArrayListLocalId, BitArrayLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId,
    CustomLocal, ExternalListLocalId, FloatListLocalId, FloatLocalId, FunctionListLocalId,
    FunctionLocal, IntListLocalId, IntLocalId, ListListLocalId, NilListLocalId, NilLocalId,
    ParameterListListLocalId, ParameterListLocalId, StringListLocalId, StringLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
};

pub(super) struct ConstantTableBuilder<Graph: ExecutionGraphProfile> {
    pub(super) ints: Vec<ProfiledConstantProgram<IntLocalId, Graph>>,
    pub(super) strings: Vec<ProfiledConstantProgram<StringLocalId, Graph>>,
    pub(super) bit_arrays: Vec<ProfiledConstantProgram<BitArrayLocalId, Graph>>,
    pub(super) customs: Vec<ProfiledConstantProgram<CustomLocal, Graph>>,
    pub(super) floats: Vec<ProfiledConstantProgram<FloatLocalId, Graph>>,
    pub(super) bools: Vec<ProfiledConstantProgram<BoolLocalId, Graph>>,
    pub(super) nils: Vec<ProfiledConstantProgram<NilLocalId, Graph>>,
    pub(super) tuples: Vec<ProfiledConstantProgram<TupleLocalId, Graph>>,
    pub(super) parameter_lists: Vec<ProfiledConstantProgram<ParameterListLocalId, Graph>>,
    pub(super) parameter_list_lists: Vec<ProfiledConstantProgram<ParameterListListLocalId, Graph>>,
    pub(super) int_lists: Vec<ProfiledConstantProgram<IntListLocalId, Graph>>,
    pub(super) string_lists: Vec<ProfiledConstantProgram<StringListLocalId, Graph>>,
    pub(super) bit_array_lists: Vec<ProfiledConstantProgram<BitArrayListLocalId, Graph>>,
    pub(super) utf_codepoint_lists: Vec<ProfiledConstantProgram<UtfCodepointListLocalId, Graph>>,
    pub(super) custom_lists: Vec<ProfiledConstantProgram<CustomListLocalId, Graph>>,
    pub(super) external_lists: Vec<ProfiledConstantProgram<ExternalListLocalId, Graph>>,
    pub(super) float_lists: Vec<ProfiledConstantProgram<FloatListLocalId, Graph>>,
    pub(super) bool_lists: Vec<ProfiledConstantProgram<BoolListLocalId, Graph>>,
    pub(super) nil_lists: Vec<ProfiledConstantProgram<NilListLocalId, Graph>>,
    pub(super) tuple_lists: Vec<ProfiledConstantProgram<TupleListLocalId, Graph>>,
    pub(super) list_lists: Vec<ProfiledConstantProgram<ListListLocalId, Graph>>,
    pub(super) function_lists: Vec<ProfiledConstantProgram<FunctionListLocalId, Graph>>,
    pub(super) functions: Vec<ProfiledConstantProgram<FunctionLocal, Graph>>,
}

impl<Graph: ExecutionGraphProfile> ConstantTableBuilder<Graph> {
    pub(super) fn push<Return: LoweredConstantValue>(
        &mut self,
        program: ProfiledConstantProgram<Return, Graph>,
    ) -> ConstantId<Return> {
        let programs = Return::programs_mut(self);
        let id = ConstantId::new(programs.len());
        programs.push(program);
        id
    }

    pub(super) fn finish(self) -> ProfiledConstantTable<Graph> {
        ProfiledConstantTable {
            ints: self.ints.into(),
            strings: self.strings.into(),
            bit_arrays: self.bit_arrays.into(),
            customs: self.customs.into(),
            floats: self.floats.into(),
            bools: self.bools.into(),
            nils: self.nils.into(),
            tuples: self.tuples.into(),
            parameter_lists: self.parameter_lists.into(),
            parameter_list_lists: self.parameter_list_lists.into(),
            int_lists: self.int_lists.into(),
            string_lists: self.string_lists.into(),
            bit_array_lists: self.bit_array_lists.into(),
            utf_codepoint_lists: self.utf_codepoint_lists.into(),
            custom_lists: self.custom_lists.into(),
            external_lists: self.external_lists.into(),
            float_lists: self.float_lists.into(),
            bool_lists: self.bool_lists.into(),
            nil_lists: self.nil_lists.into(),
            tuple_lists: self.tuple_lists.into(),
            list_lists: self.list_lists.into(),
            function_lists: self.function_lists.into(),
            functions: self.functions.into(),
        }
    }
}

impl<Graph: ExecutionGraphProfile> Default for ConstantTableBuilder<Graph> {
    fn default() -> Self {
        Self {
            ints: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            customs: Vec::new(),
            floats: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            parameter_lists: Vec::new(),
            parameter_list_lists: Vec::new(),
            int_lists: Vec::new(),
            string_lists: Vec::new(),
            bit_array_lists: Vec::new(),
            utf_codepoint_lists: Vec::new(),
            custom_lists: Vec::new(),
            external_lists: Vec::new(),
            float_lists: Vec::new(),
            bool_lists: Vec::new(),
            nil_lists: Vec::new(),
            tuple_lists: Vec::new(),
            list_lists: Vec::new(),
            function_lists: Vec::new(),
            functions: Vec::new(),
        }
    }
}
