pub(crate) mod int_list;
pub(crate) mod numeric;

use crate::plan::execution::graph::BlockGraphExitId;

/// A compiled implementation returns to the existing graph owner at an
/// admitted checkpoint or exit. Payload contracts belong to each kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompiledProgress {
    Yield(usize),
    Interpreted(usize),
    Complete(BlockGraphExitId),
}

#[cfg(test)]
pub(crate) mod tests {
    use super::CompiledProgress;
    use super::int_list::{IntListOps, IntListValues};
    use super::numeric::NumericValues;
    use crate::runtime::state::list::RuntimeListStorage;

    // Metadata-only fixtures must never execute the Rust implementation.
    pub(crate) fn metadata_numeric(
        _: usize,
        _: &mut NumericValues,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a numeric kernel")
    }

    pub(crate) fn metadata_int_list(
        _: usize,
        _: &mut IntListValues,
        _: &IntListOps<'_>,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a list kernel")
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a numeric kernel")]
    fn numeric_metadata_fixture_rejects_execution() {
        metadata_numeric(0, &mut NumericValues::default(), &mut 1);
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a list kernel")]
    fn list_metadata_fixture_rejects_execution() {
        let storage = RuntimeListStorage::default();
        metadata_int_list(
            0,
            &mut IntListValues::default(),
            &IntListOps::new(&storage),
            &mut 1,
        );
    }
}
