use super::ListFunctionLocal;
use crate::plan::{FunctionType, ValueType};

#[derive(Default)]
pub(crate) struct FrameLayout {
    ints: usize,
    floats: usize,
    strings: usize,
    bools: usize,
    tuples: usize,
    int_lists: usize,
    string_lists: usize,
    float_lists: usize,
    bool_lists: usize,
    nil_lists: usize,
    tuple_lists: Vec<Vec<ValueType>>,
    list_lists: Vec<ValueType>,
    function_lists: Vec<FunctionType>,
    int_functions: usize,
    float_functions: usize,
    string_functions: usize,
    bool_functions: usize,
    nil_functions: usize,
    tuple_functions: usize,
    list_functions: Vec<ListFunctionLocal>,
    function_functions: usize,
}

pub(super) struct FrameLayoutParts {
    pub(super) ints: usize,
    pub(super) floats: usize,
    pub(super) strings: usize,
    pub(super) bools: usize,
    pub(super) tuples: usize,
    pub(super) int_lists: usize,
    pub(super) string_lists: usize,
    pub(super) float_lists: usize,
    pub(super) bool_lists: usize,
    pub(super) nil_lists: usize,
    pub(super) tuple_lists: Vec<Vec<ValueType>>,
    pub(super) list_lists: Vec<ValueType>,
    pub(super) function_lists: Vec<FunctionType>,
    pub(super) int_functions: usize,
    pub(super) float_functions: usize,
    pub(super) string_functions: usize,
    pub(super) bool_functions: usize,
    pub(super) nil_functions: usize,
    pub(super) tuple_functions: usize,
    pub(super) list_functions: Vec<ListFunctionLocal>,
    pub(super) function_functions: usize,
}

impl FrameLayout {
    pub(super) fn from_parts(parts: FrameLayoutParts) -> Self {
        Self {
            ints: parts.ints,
            floats: parts.floats,
            strings: parts.strings,
            bools: parts.bools,
            tuples: parts.tuples,
            int_lists: parts.int_lists,
            string_lists: parts.string_lists,
            float_lists: parts.float_lists,
            bool_lists: parts.bool_lists,
            nil_lists: parts.nil_lists,
            tuple_lists: parts.tuple_lists,
            list_lists: parts.list_lists,
            function_lists: parts.function_lists,
            int_functions: parts.int_functions,
            float_functions: parts.float_functions,
            string_functions: parts.string_functions,
            bool_functions: parts.bool_functions,
            nil_functions: parts.nil_functions,
            tuple_functions: parts.tuple_functions,
            list_functions: parts.list_functions,
            function_functions: parts.function_functions,
        }
    }

    pub(crate) fn ints(&self) -> usize {
        self.ints
    }

    pub(crate) fn floats(&self) -> usize {
        self.floats
    }

    pub(crate) fn strings(&self) -> usize {
        self.strings
    }

    pub(crate) fn bools(&self) -> usize {
        self.bools
    }

    pub(crate) fn tuples(&self) -> usize {
        self.tuples
    }

    pub(crate) fn int_lists(&self) -> usize {
        self.int_lists
    }

    pub(crate) fn string_lists(&self) -> usize {
        self.string_lists
    }

    pub(crate) fn float_lists(&self) -> usize {
        self.float_lists
    }

    pub(crate) fn bool_lists(&self) -> usize {
        self.bool_lists
    }

    pub(crate) fn nil_lists(&self) -> usize {
        self.nil_lists
    }

    pub(crate) fn tuple_lists(&self) -> &[Vec<ValueType>] {
        &self.tuple_lists
    }

    pub(crate) fn list_lists(&self) -> &[ValueType] {
        &self.list_lists
    }

    pub(crate) fn function_lists(&self) -> &[FunctionType] {
        &self.function_lists
    }

    pub(crate) fn int_functions(&self) -> usize {
        self.int_functions
    }

    pub(crate) fn float_functions(&self) -> usize {
        self.float_functions
    }

    pub(crate) fn string_functions(&self) -> usize {
        self.string_functions
    }

    pub(crate) fn bool_functions(&self) -> usize {
        self.bool_functions
    }

    pub(crate) fn nil_functions(&self) -> usize {
        self.nil_functions
    }

    pub(crate) fn tuple_functions(&self) -> usize {
        self.tuple_functions
    }

    pub(crate) fn list_functions(&self) -> &[ListFunctionLocal] {
        &self.list_functions
    }

    pub(crate) fn function_functions(&self) -> usize {
        self.function_functions
    }
}
