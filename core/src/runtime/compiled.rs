pub(crate) mod bit_array;
pub(crate) mod numeric;

use crate::plan::execution::graph::BlockGraphExitId;

/// Compiled implementations consume canonical graph steps from the budget and
/// return to the existing graph owner at an admitted checkpoint or exit.
/// Payload contracts belong to each kernel. Rust implementations are trusted
/// code; data admission does not prove arithmetic or control-flow semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompiledProgress {
    Yield(usize),
    Interpreted(usize),
    Complete(BlockGraphExitId),
}

#[cfg(test)]
pub(crate) mod tests {
    use super::CompiledProgress;
    use super::bit_array::BitArrayValues;
    use super::numeric::NumericValues;

    // Admission and static-link fixtures describe metadata, never execution.
    pub(crate) fn metadata_numeric(
        _: usize,
        _: &mut NumericValues,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a numeric kernel")
    }

    pub(crate) fn metadata_bit_array(
        _: usize,
        _: &mut BitArrayValues,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a bit-array kernel")
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a numeric kernel")]
    fn numeric_metadata_fixture_rejects_execution() {
        metadata_numeric(0, &mut NumericValues::default(), &mut 1);
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a bit-array kernel")]
    fn bit_array_metadata_fixture_rejects_execution() {
        metadata_bit_array(0, &mut BitArrayValues::default(), &mut 1);
    }
}
