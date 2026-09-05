use super::{ResumableFuture, ResumableState, TransferInputs};
use crate::host::{AsyncHostCallError, AsyncHostCallbackCompletion, HostProfile};
use crate::plan::execution::AsyncHostedExecution;
use crate::runtime::{HostCallOrigin, TransferValues};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Mutex, Weak};

pub(crate) trait AsyncHostCallbackRequest<Profile: HostProfile>: Send {
    fn service<'call>(
        self: Box<Self>,
        plan: &'call AsyncHostedExecution<Profile>,
        state: &'call mut ResumableState<'_, Profile>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'call>>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send;
}

pub(crate) trait ResumableCallback<Profile: HostProfile>: Send + 'static {
    type Output: Send + 'static;

    fn request(
        &self,
        origin: HostCallOrigin,
        inputs: TransferInputs,
        completion: Weak<Mutex<AsyncHostCallbackCompletion<Self::Output>>>,
    ) -> Box<dyn AsyncHostCallbackRequest<Profile> + Send>;
}

trait ResumableCallbackTarget<Profile: HostProfile>: Send + 'static {
    type Output: Send + 'static;

    fn invoke<'call>(
        self,
        plan: &'call AsyncHostedExecution<Profile>,
        state: &'call mut ResumableState<'_, Profile>,
        origin: HostCallOrigin,
        inputs: crate::runtime::ProfiledRetainedValues<TransferValues>,
    ) -> ResumableFuture<'call, Self::Output>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send;
}

struct CallbackRequest<Function, Return> {
    function: Function,
    origin: HostCallOrigin,
    inputs: crate::runtime::ProfiledRetainedValues<TransferValues>,
    completion: Weak<Mutex<AsyncHostCallbackCompletion<Return>>>,
}

impl<Function, Return> CallbackRequest<Function, Return> {
    fn new(
        function: Function,
        origin: HostCallOrigin,
        inputs: crate::runtime::ProfiledRetainedValues<TransferValues>,
        completion: Weak<Mutex<AsyncHostCallbackCompletion<Return>>>,
    ) -> Self {
        Self {
            function,
            origin,
            inputs,
            completion,
        }
    }
}

impl<Profile, Function, Return> AsyncHostCallbackRequest<Profile>
    for CallbackRequest<Function, Return>
where
    Profile: HostProfile,
    Function: ResumableCallbackTarget<Profile, Output = Return>,
    Return: Send + 'static,
{
    fn service<'call>(
        self: Box<Self>,
        plan: &'call AsyncHostedExecution<Profile>,
        state: &'call mut ResumableState<'_, Profile>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'call>>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
    {
        Box::pin(async move {
            let Self {
                function,
                origin,
                inputs,
                completion,
            } = *self;
            let Some(completion) = completion.upgrade() else {
                return;
            };
            let output = function
                .invoke(plan, state, origin, inputs)
                .await
                .map_err(AsyncHostCallError::nested);
            AsyncHostCallbackCompletion::complete(completion, output);
        })
    }
}

macro_rules! resumable_callback {
    ($function:ty, $target:ty, $output:ty, $run:ident, $map:expr) => {
        impl<Profile: HostProfile> ResumableCallback<Profile> for $function {
            type Output = $output;

            fn request(
                &self,
                origin: HostCallOrigin,
                inputs: TransferInputs,
                completion: Weak<Mutex<AsyncHostCallbackCompletion<Self::Output>>>,
            ) -> Box<dyn AsyncHostCallbackRequest<Profile> + Send> {
                let mut inputs = inputs.into_retained();
                inputs.append_captures(self.captures());
                Box::new(CallbackRequest::new(
                    self.runtime_id(),
                    origin,
                    inputs,
                    completion,
                ))
            }
        }

        impl<Profile: HostProfile> ResumableCallbackTarget<Profile> for $target {
            type Output = $output;

            fn invoke<'call>(
                self,
                plan: &'call AsyncHostedExecution<Profile>,
                state: &'call mut ResumableState<'_, Profile>,
                origin: HostCallOrigin,
                inputs: crate::runtime::ProfiledRetainedValues<TransferValues>,
            ) -> ResumableFuture<'call, Self::Output>
            where
                Profile::RunState: Send,
                Profile::ExternalStores: Send,
            {
                Box::pin(async move {
                    super::$run(plan, state, self, origin, inputs)
                        .await
                        .map($map)
                })
            }
        }
    };
}

resumable_callback!(
    crate::runtime::EvaluatedIntFunction<TransferValues>,
    crate::plan::execution::function::IntFunctionId,
    num_bigint::BigInt,
    run_int,
    std::convert::identity
);
resumable_callback!(
    crate::runtime::EvaluatedFloatFunction<TransferValues>,
    crate::plan::execution::function::FloatFunctionId,
    f64,
    run_float,
    std::convert::identity
);
resumable_callback!(
    crate::runtime::EvaluatedStringFunction<TransferValues>,
    crate::plan::execution::function::StringFunctionId,
    ecow::EcoString,
    run_string,
    std::convert::identity
);
resumable_callback!(
    crate::runtime::EvaluatedBitArrayFunction<TransferValues>,
    crate::plan::execution::function::BitArrayFunctionId,
    crate::BitArrayValue,
    run_bit_array,
    crate::runtime::EvaluatedBitArray::into_value
);
resumable_callback!(
    crate::runtime::EvaluatedUtfCodepointFunction<TransferValues>,
    crate::plan::execution::function::UtfCodepointFunctionId,
    char,
    run_utf_codepoint,
    std::convert::identity
);
resumable_callback!(
    crate::runtime::EvaluatedBoolFunction<TransferValues>,
    crate::plan::execution::function::BoolFunctionId,
    bool,
    run_bool,
    std::convert::identity
);
resumable_callback!(
    crate::runtime::EvaluatedNilFunction<TransferValues>,
    crate::plan::execution::function::NilFunctionId,
    (),
    run_nil,
    std::convert::identity
);
