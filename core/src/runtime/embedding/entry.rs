use super::EmbeddingOutput;
use crate::embedding::CallError;
use crate::host::HostProfile;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
    IntFunctionId, LibraryListFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::runtime::execution::EntryContext;
use crate::runtime::state::list::ListValueId;
use crate::runtime::{HostCallOrigin, RetainedInputs};
use std::future::Future;

pub(crate) trait EmbeddingEntry: Send + Copy {
    type Output: Send;

    fn call<Profile: HostProfile>(
        self,
        context: &EntryContext<Profile>,
        inputs: RetainedInputs,
    ) -> impl Future<Output = Result<Self::Output, CallError>> + Send;
}

macro_rules! scalar {
    ($entry:ty, $output:ty, $map:expr) => {
        impl EmbeddingEntry for $entry {
            type Output = $output;
            async fn call<Profile: HostProfile>(
                self,
                context: &EntryContext<Profile>,
                inputs: RetainedInputs,
            ) -> Result<Self::Output, CallError> {
                context
                    .call(self, HostCallOrigin::Entry, inputs.into_retained())
                    .await
                    .map_err(|_| CallError::Cancelled)?
                    .map($map)
                    .map_err(CallError::Execution)
            }
        }
    };
}

scalar!(IntFunctionId, num_bigint::BigInt, std::convert::identity);
scalar!(FloatFunctionId, f64, std::convert::identity);
scalar!(StringFunctionId, ecow::EcoString, std::convert::identity);
scalar!(
    BitArrayFunctionId,
    crate::BitArrayValue,
    crate::runtime::EvaluatedBitArray::into_value
);
scalar!(UtfCodepointFunctionId, char, std::convert::identity);
scalar!(BoolFunctionId, bool, std::convert::identity);
scalar!(NilFunctionId, (), std::convert::identity);
scalar!(
    TupleFunctionId,
    EmbeddingOutput,
    EmbeddingOutput::from_tuple
);
scalar!(
    CustomFunctionId,
    EmbeddingOutput,
    EmbeddingOutput::from_custom
);
scalar!(
    ExternalFunctionId,
    crate::runtime::EvaluatedExternalValue,
    std::convert::identity
);

impl EmbeddingEntry for LibraryListFunctionId {
    type Output = EmbeddingOutput;

    async fn call<Profile: HostProfile>(
        self,
        context: &EntryContext<Profile>,
        inputs: RetainedInputs,
    ) -> Result<Self::Output, CallError> {
        macro_rules! call {
            ($id:expr, $family:ident) => {
                context
                    .call($id, HostCallOrigin::Entry, inputs.into_retained())
                    .await
                    .map_err(|_| CallError::Cancelled)?
                    .map(|value| EmbeddingOutput::from_value(ListValueId::$family(value).into()))
                    .map_err(CallError::Execution)
            };
        }
        match self {
            Self::Int(id) => call!(id, Int),
            Self::Float(id) => call!(id, Float),
            Self::String(id) => call!(id, String),
            Self::BitArray(id) => call!(id, BitArray),
            Self::UtfCodepoint(id) => call!(id, UtfCodepoint),
            Self::Bool(id) => call!(id, Bool),
            Self::Nil(id) => call!(id, Nil),
            Self::Custom(id) => call!(id, Custom),
            Self::External(id) => call!(id, External),
            Self::Tuple(id) => call!(id, Tuple),
            Self::List(id) => call!(id, List),
        }
    }
}
