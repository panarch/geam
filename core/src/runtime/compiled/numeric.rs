use super::CompiledProgress;

pub type NumericKernel = fn(usize, &mut NumericValues, &mut usize) -> CompiledProgress;

/// Actual numeric block parameters and completed instruction outputs, reused
/// across generated entry, yield, return, and Big side exits.
#[derive(Default)]
pub struct NumericValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
}
