use crate::plan::execution::CustomValueShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NeverFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomFunctionId {
    index: usize,
    return_shape: CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionId(pub(crate) usize);

impl CustomFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, return_shape: CustomValueShape) -> Self {
        Self {
            index,
            return_shape,
        }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}
