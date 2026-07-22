use super::{Instruction, Terminator};
use crate::plan::execution::ParamSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BlockId(usize);

pub(crate) struct Block {
    params: Box<[ParamSlot]>,
    instructions: Box<[Instruction]>,
    terminator: Terminator,
}

impl BlockId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl Block {
    pub(in crate::plan::execution) fn new(
        params: Vec<ParamSlot>,
        instructions: Vec<Instruction>,
        terminator: Terminator,
    ) -> Self {
        Self {
            params: params.into_boxed_slice(),
            instructions: instructions.into_boxed_slice(),
            terminator,
        }
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }

    pub(crate) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(crate) fn terminator(&self) -> &Terminator {
        &self.terminator
    }
}
