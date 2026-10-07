pub(crate) mod bit_array;
pub(crate) mod calls;
pub(crate) mod custom;
pub(crate) mod custom_loop;
pub(crate) mod int_list;
pub(crate) mod native_calls;
pub(crate) mod native_loop;
pub(crate) mod numeric;
pub(crate) mod string;

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
    use super::calls::{CallExecution, CallInputs, CallStorage};
    use super::custom::CustomValues;
    use super::custom_loop::{
        CallbackInputs, CallbackProgress, CustomListOps, CustomLoopProgress, CustomLoopValues,
    };
    use super::int_list::{IntListOps, IntListValues};
    use super::numeric::NumericValues;
    use super::string::StringValues;
    use crate::runtime::state::list::RuntimeListStorage;

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

    pub(crate) fn metadata_int_list(
        _: usize,
        _: &mut IntListValues,
        _: &IntListOps<'_>,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a list kernel")
    }

    pub(crate) fn metadata_callback<Value>(
        _: &CallbackInputs<'_>,
        _: &mut CustomValues,
        _: &mut usize,
    ) -> CallbackProgress<Value> {
        panic!("metadata fixture must not execute a callback kernel")
    }

    pub(crate) fn metadata_custom_loop(
        _: usize,
        _: &mut CustomLoopValues,
        _: &CustomListOps<'_>,
        _: &mut usize,
    ) -> CustomLoopProgress {
        panic!("metadata fixture must not execute a custom-loop kernel")
    }

    pub(crate) fn metadata_string(
        _: usize,
        _: &mut StringValues,
        _: &mut usize,
    ) -> CompiledProgress {
        panic!("metadata fixture must not execute a string kernel")
    }

    pub(crate) fn metadata_calls(
        _: usize,
        _: CallInputs<'_>,
        _: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        panic!("metadata fixture must not execute a function-call kernel")
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

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a custom-loop kernel")]
    fn custom_loop_metadata_fixture_rejects_execution() {
        let storage = RuntimeListStorage::default();
        metadata_custom_loop(
            0,
            &mut CustomLoopValues::default(),
            &CustomListOps::new(&storage),
            &mut 1,
        );
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a string kernel")]
    fn string_metadata_fixture_rejects_execution() {
        metadata_string(0, &mut StringValues::default(), &mut 1);
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a function-call kernel")]
    fn function_call_metadata_fixture_rejects_execution() {
        metadata_calls(
            0,
            CallInputs::new(&[], &[], &[], &[], &[]),
            &mut CallStorage::default(),
        );
    }
}
