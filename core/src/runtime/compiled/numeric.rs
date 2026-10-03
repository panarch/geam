use super::CompiledProgress;

/// A compiled implementation consumes canonical graph steps from `budget`.
/// Checkpoint indices and returned exits correspond to its admitted graph.
/// Rust implementations are trusted code; data admission does not prove their
/// arithmetic or control-flow semantics.
pub type NumericKernel = fn(usize, &mut NumericValues, &mut usize) -> CompiledProgress;

/// Actual numeric block parameters and completed instruction outputs, reused
/// across generated entry, yield, return, and Big side exits.
#[derive(Default)]
pub struct NumericValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
}
