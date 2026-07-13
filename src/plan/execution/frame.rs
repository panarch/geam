use super::{
    BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId,
    ListFunctionLocal, ListListTypeId, NilListTypeId, StringListTypeId, TupleListTypeId,
};

#[derive(Default)]
pub(crate) struct FrameLayout {
    slots: FrameSlots,
}

#[derive(Default)]
pub(super) struct FrameSlots {
    pub(super) ints: usize,
    pub(super) floats: usize,
    pub(super) strings: usize,
    pub(super) bit_arrays: usize,
    pub(super) bools: usize,
    pub(super) tuples: usize,
    pub(super) int_lists: Vec<IntListTypeId>,
    pub(super) string_lists: Vec<StringListTypeId>,
    pub(super) bit_array_lists: Vec<BitArrayListTypeId>,
    pub(super) float_lists: Vec<FloatListTypeId>,
    pub(super) bool_lists: Vec<BoolListTypeId>,
    pub(super) nil_lists: Vec<NilListTypeId>,
    pub(super) tuple_lists: Vec<TupleListTypeId>,
    pub(super) list_lists: Vec<ListListTypeId>,
    pub(super) function_lists: Vec<FunctionListTypeId>,
    pub(super) int_functions: usize,
    pub(super) float_functions: usize,
    pub(super) string_functions: usize,
    pub(super) bit_array_functions: usize,
    pub(super) bool_functions: usize,
    pub(super) nil_functions: usize,
    pub(super) tuple_functions: usize,
    pub(super) list_functions: Vec<ListFunctionLocal>,
    pub(super) function_functions: usize,
}

impl FrameLayout {
    pub(super) fn from_slots(slots: FrameSlots) -> Self {
        Self { slots }
    }

    pub(crate) fn ints(&self) -> usize {
        self.slots.ints
    }

    pub(crate) fn floats(&self) -> usize {
        self.slots.floats
    }

    pub(crate) fn strings(&self) -> usize {
        self.slots.strings
    }

    pub(crate) fn bit_arrays(&self) -> usize {
        self.slots.bit_arrays
    }

    pub(crate) fn bools(&self) -> usize {
        self.slots.bools
    }

    pub(crate) fn tuples(&self) -> usize {
        self.slots.tuples
    }

    pub(crate) fn int_lists(&self) -> &[IntListTypeId] {
        &self.slots.int_lists
    }

    pub(crate) fn string_lists(&self) -> &[StringListTypeId] {
        &self.slots.string_lists
    }

    pub(crate) fn bit_array_lists(&self) -> &[BitArrayListTypeId] {
        &self.slots.bit_array_lists
    }

    pub(crate) fn float_lists(&self) -> &[FloatListTypeId] {
        &self.slots.float_lists
    }

    pub(crate) fn bool_lists(&self) -> &[BoolListTypeId] {
        &self.slots.bool_lists
    }

    pub(crate) fn nil_lists(&self) -> &[NilListTypeId] {
        &self.slots.nil_lists
    }

    pub(crate) fn tuple_lists(&self) -> &[TupleListTypeId] {
        &self.slots.tuple_lists
    }

    pub(crate) fn list_lists(&self) -> &[ListListTypeId] {
        &self.slots.list_lists
    }

    pub(crate) fn function_lists(&self) -> &[FunctionListTypeId] {
        &self.slots.function_lists
    }

    pub(crate) fn int_functions(&self) -> usize {
        self.slots.int_functions
    }

    pub(crate) fn float_functions(&self) -> usize {
        self.slots.float_functions
    }

    pub(crate) fn string_functions(&self) -> usize {
        self.slots.string_functions
    }

    pub(crate) fn bit_array_functions(&self) -> usize {
        self.slots.bit_array_functions
    }

    pub(crate) fn bool_functions(&self) -> usize {
        self.slots.bool_functions
    }

    pub(crate) fn nil_functions(&self) -> usize {
        self.slots.nil_functions
    }

    pub(crate) fn tuple_functions(&self) -> usize {
        self.slots.tuple_functions
    }

    pub(crate) fn list_functions(&self) -> &[ListFunctionLocal] {
        &self.slots.list_functions
    }

    pub(crate) fn function_functions(&self) -> usize {
        self.slots.function_functions
    }
}
