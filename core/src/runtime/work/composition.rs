use super::Shared;
use super::execution::{Completion, SourceWork, WorkContext};
use crate::host::{
    HostCallErrorKind, HostCallRuntime, HostCodecScope, HostFutureCompletion, HostFutureError,
    HostFutureStore, HostProvider, HostScopedValue, HostTokenRuntime, HostType, HostTypeDescriptor,
    HostTypeSequence, HostWorkProfile,
};
use crate::runtime::host::RuntimeHostCall;
use crate::runtime::{HostCallOrigin, StoredRuntimeList};
use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use std::collections::BTreeMap;
use std::future::Future;

impl<Profile: HostWorkProfile> WorkContext<Profile> {
    pub(crate) fn native<Provider, Output, Constructions, Native>(
        &self,
        start: impl FnOnce(super::Dependencies<Completion>) -> Native,
        codec: HostCodecScope,
        origin: HostCallOrigin,
    ) -> SourceWork
    where
        Provider: HostProvider<Profile>,
        Output: HostType,
        Constructions: HostTypeSequence,
        Native: Future<
                Output = Result<
                    HostFutureCompletion<Profile, Provider, Output, Constructions>,
                    HostFutureError,
                >,
            > + Send
            + 'static,
    {
        let context = self.clone();
        self.compose(|dependencies| {
            let native = start(dependencies);
            async move {
                let completion = match native.await {
                    Ok(completion) => Ok(completion),
                    Err(HostFutureError::Host(error)) => Err(error),
                    Err(HostFutureError::Cancelled) => return Err(super::Cancelled),
                    Err(HostFutureError::Execution(error)) => return Ok(Shared::new(Err(error.0))),
                };
                let result = context
                    .with_runtime(move |plan, state| {
                        let output = completion.and_then(|completion| {
                            let mut runtime =
                                RuntimeHostCall::new_codec(plan, state, &codec, origin.clone());
                            completion
                                .complete(&mut runtime)
                                .map(|token| runtime.retain_stored(HostScopedValue::Value(token)))
                        });
                        output.map_err(|error| match error.into_kind() {
                            HostCallErrorKind::Failure(failure) => {
                                crate::ExecutionError::host_failure(
                                    plan,
                                    origin,
                                    codec.function(),
                                    failure,
                                )
                            }
                            HostCallErrorKind::Nested(error) => error,
                        })
                    })
                    .await?;
                Ok(Shared::new(result.map(Shared::new).map_err(Shared::new)))
            }
        })
    }

    pub(crate) fn flatten(
        &self,
        input: SourceWork,
        codec: HostCodecScope,
        origin: HostCallOrigin,
    ) -> SourceWork {
        let context = self.clone();
        self.compose(|dependencies| async move {
            let completion = dependencies.observe(&input).await?;
            let result = completion.read(Clone::clone);
            match result {
                Ok(value) => {
                    let inner = context
                        .with_runtime(move |plan, state| {
                            let mut runtime =
                                RuntimeHostCall::new_codec(plan, state, &codec, origin);
                            let token = value.read(|value| runtime.restore_stored(value));
                            let lease = runtime.external_lease(runtime.external_token(token));
                            crate::host::work_store::<Profile>(runtime.external_stores())
                                .work(&lease)
                        })
                        .await?;
                    dependencies.observe(&inner).await
                }
                Err(error) => Ok(Shared::new(Err(error))),
            }
        })
    }

    pub(crate) fn all(
        &self,
        inputs: StoredRuntimeList,
        store: HostFutureStore,
        list_type: HostTypeDescriptor,
        codec: HostCodecScope,
        origin: HostCallOrigin,
    ) -> SourceWork {
        let context = self.clone();
        self.compose(|dependencies| async move {
            let mut pending = FuturesUnordered::new();
            let mut index = 0;
            while let Some(work) =
                inputs.decode_item(index, |item| store.work(&item.into_external_lease()))
            {
                let position = index;
                let observation = dependencies.observe(&work);
                pending.push(async move { (position, observation.await) });
                index += 1;
            }
            drop(inputs);
            let mut completed = BTreeMap::new();
            while let Some((position, completion)) = pending.next().await {
                let completion = completion?;
                match completion.read(Clone::clone) {
                    Ok(value) => {
                        completed.insert(position, value);
                    }
                    Err(error) => return Ok(Shared::new(Err(error))),
                }
            }
            let result = context
                .with_runtime(move |plan, state| {
                    let mut runtime = RuntimeHostCall::new_codec(plan, state, &codec, origin);
                    let values = completed
                        .into_values()
                        .map(|value| {
                            HostScopedValue::Value(
                                value.read(|value| runtime.restore_stored(value)),
                            )
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice();
                    let output = runtime.build_list(&list_type, values);
                    runtime.retain_stored(HostScopedValue::Value(output))
                })
                .await?;
            Ok(Shared::new(Ok(Shared::new(result))))
        })
    }
}
