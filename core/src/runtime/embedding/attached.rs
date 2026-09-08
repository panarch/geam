use crate::host::HostProfile;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
    IntFunctionId, LibraryListFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::runtime::function;
use crate::runtime::work::driver::Driver;
use crate::runtime::{EmbeddingOutput, HostCallOrigin, TransferInputs, TransferValues};

macro_rules! scalar {
    ($method:ident, $entry:ty, $output:ty, $run:ident) => {
        impl<Profile: HostProfile> Driver<'_, Profile> {
            pub(crate) fn $method(
                &mut self,
                function: $entry,
                inputs: TransferInputs,
            ) -> Result<$output, crate::AsyncExecutionError> {
                self.call(|plan, state| {
                    function::$run(
                        plan,
                        state,
                        function,
                        HostCallOrigin::Entry,
                        inputs.into_retained(),
                    )
                })
            }
        }
    };
}

scalar!(run_int, IntFunctionId, num_bigint::BigInt, run_int);
scalar!(run_float, FloatFunctionId, f64, run_float);
scalar!(run_string, StringFunctionId, ecow::EcoString, run_string);
scalar!(
    run_utf_codepoint,
    UtfCodepointFunctionId,
    char,
    run_utf_codepoint
);
scalar!(run_bool, BoolFunctionId, bool, run_bool);
scalar!(run_nil, NilFunctionId, (), run_nil);
scalar!(
    run_external,
    ExternalFunctionId,
    crate::runtime::EvaluatedExternalValue<TransferValues>,
    run_external
);

impl<Profile: HostProfile> Driver<'_, Profile> {
    pub(crate) fn run_bit_array(
        &mut self,
        function: BitArrayFunctionId,
        inputs: TransferInputs,
    ) -> Result<crate::BitArrayValue, crate::AsyncExecutionError> {
        self.call(|plan, state| {
            function::run_bit_array(
                plan,
                state,
                function,
                HostCallOrigin::Entry,
                inputs.into_retained(),
            )
        })
        .map(crate::runtime::EvaluatedBitArray::into_value)
    }

    pub(crate) fn run_tuple(
        &mut self,
        function: TupleFunctionId,
        inputs: TransferInputs,
    ) -> Result<EmbeddingOutput<TransferValues>, crate::AsyncExecutionError> {
        self.call(|plan, state| {
            function::run_tuple(
                plan,
                state,
                function,
                HostCallOrigin::Entry,
                inputs.into_retained(),
            )
        })
        .map(EmbeddingOutput::from_tuple)
    }

    pub(crate) fn run_custom(
        &mut self,
        function: CustomFunctionId,
        inputs: TransferInputs,
    ) -> Result<EmbeddingOutput<TransferValues>, crate::AsyncExecutionError> {
        self.call(|plan, state| {
            function::run_custom(
                plan,
                state,
                function,
                HostCallOrigin::Entry,
                inputs.into_retained(),
            )
        })
        .map(EmbeddingOutput::from_custom)
    }

    pub(crate) fn run_list(
        &mut self,
        function: LibraryListFunctionId,
        inputs: TransferInputs,
    ) -> Result<EmbeddingOutput<TransferValues>, crate::AsyncExecutionError> {
        self.call(|plan, state| {
            function::run_list(
                plan,
                state,
                function.runtime_id(),
                HostCallOrigin::Entry,
                inputs.into_retained(),
            )
        })
        .map(|value| EmbeddingOutput::from_value(value.into()))
    }
}
