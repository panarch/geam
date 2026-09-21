use super::{HostFutureContext, HostFutureType, HostWorkProfile};
use crate::host::{
    HostCall, HostCallError, HostCodecScope, HostExecutionError, HostExternal, HostProfile,
    HostProvider, HostType, HostTypeSequence,
};
use crate::runtime::HostCallOrigin;
use crate::runtime::SharedExecutionError;
use crate::runtime::execution::ExecutionContext;
use crate::runtime::work::execution::SourceWork;
use std::future::Future;
use std::marker::PhantomData;

/// One typed source Future retained by native code.
///
/// Receiving this value does not drive it. Observation uses its original
/// execution endpoint and shares the operation's success or failure. Retaining
/// it cannot restart pending work after that execution has ended.
pub struct HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    work: SourceWork,
    value: crate::runtime::StoredRuntimeValue,
    context: ExecutionContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
    signature: PhantomData<fn(Provider) -> Value>,
}

pub(crate) struct FutureRetention<Profile: HostProfile> {
    store: super::HostFutureStore,
    context: ExecutionContext<Profile>,
    codec: HostCodecScope,
    origin: HostCallOrigin,
}

impl<'call, Profile, Provider, Return> HostCall<'call, Profile, Provider, Return>
where
    Profile: HostWorkProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Retains a Future-valued argument or callback result without observing it.
    pub fn future_value<Value: HostType>(
        &self,
        value: HostExternal<'call, HostFutureType<Value, crate::host::HostWorkSchema<Profile>>>,
    ) -> HostFutureValue<Profile, Provider, Value> {
        let lease = self.runtime.external_lease(value.token);
        HostFutureValue {
            work: crate::host::work_store::<Profile>(self.runtime.external_stores()).work(&lease),
            value: self
                .retain_value::<HostFutureType<Value, crate::host::HostWorkSchema<Profile>>>(value),
            context: self.runtime.execution().with_unit(None),
            codec: self.runtime.codec_scope(),
            origin: self.runtime.origin(),
            signature: PhantomData,
        }
    }

    pub(crate) fn future_retention(&self) -> FutureRetention<Profile> {
        FutureRetention {
            store: crate::host::work_store::<Profile>(self.runtime.external_stores())
                .clone_handle(),
            context: self.runtime.execution().with_unit(None),
            codec: self.runtime.codec_scope(),
            origin: self.runtime.origin(),
        }
    }
}

impl<Profile, Provider, Value> HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    /// Explicitly observes this work while the host drives the native operation.
    ///
    /// Each observer supplies its own typed result decoder. The underlying work
    /// and completion are shared, never replayed for another observation.
    pub fn observe<'request, Constructions, Output, Decode>(
        &'request self,
        context: &'request HostFutureContext<'_, Profile, Provider, Constructions>,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Constructions: HostTypeSequence,
        Output: Send + 'static,
        Decode: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                Value::Value<'call>,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        self.observe_dependencies(&context.dependencies, decode)
    }

    /// Restores the same shared work value in its original live execution.
    ///
    /// This neither observes the work nor creates another operation or payload.
    /// A different execution is rejected before a value token is produced.
    pub fn restore<'call, CallerProvider, Return>(
        &self,
        call: &mut HostCall<'call, Profile, CallerProvider, Return>,
    ) -> Result<
        HostExternal<'call, HostFutureType<Value, crate::host::HostWorkSchema<Profile>>>,
        HostCallError,
    >
    where
        Profile: HostWorkProfile,
        CallerProvider: HostProvider<Profile>,
        Return: HostType,
    {
        self.context
            .for_invocation(&call.runtime.execution())
            .map_err(|_| crate::HostFailure::new("work belongs to another execution"))?;
        Ok(
            call.restore_value::<HostFutureType<Value, crate::host::HostWorkSchema<Profile>>>(
                &self.value,
            ),
        )
    }

    pub(crate) fn into_provider<Other: HostProvider<Profile>>(
        self,
    ) -> HostFutureValue<Profile, Other, Value> {
        HostFutureValue {
            work: self.work,
            value: self.value,
            context: self.context,
            codec: self.codec,
            origin: self.origin,
            signature: PhantomData,
        }
    }

    pub(crate) fn observe_dependencies<'request, Output, Decode>(
        &'request self,
        dependencies: &'request crate::runtime::work::Dependencies<
            crate::runtime::work::execution::Completion,
        >,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Output: Send + 'static,
        Decode: for<'call> FnOnce(
                HostCall<'call, Profile, Provider, ()>,
                Value::Value<'call>,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        let observer = dependencies.observe(&self.work);
        async move {
            let completion = observer.await?;
            let value = completion
                .read(Clone::clone)
                .map_err(|error| HostExecutionError::Execution(SharedExecutionError(error)))?;
            self.context
                .decode_completion(
                    value,
                    self.codec.clone(),
                    self.origin.clone(),
                    move |runtime, token| {
                        let value =
                            crate::host::type_::from_runtime_token::<Value, _>(runtime, token);
                        decode(HostCall::new(runtime), value)
                    },
                )
                .await
        }
    }
}

impl<Profile, Provider, Value> Clone for HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            value: self.value.clone_retained(),
            context: self.context.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
            signature: PhantomData,
        }
    }
}

impl<Profile: HostProfile> FutureRetention<Profile> {
    pub(crate) fn bind<Provider: HostProvider<Profile>, Value: HostType>(
        &self,
        value: crate::runtime::StoredRuntimeValue,
        lease: &crate::runtime::ExternalPayloadLease,
    ) -> HostFutureValue<Profile, Provider, Value> {
        HostFutureValue {
            work: self.store.work(lease),
            value,
            context: self.context.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
            signature: PhantomData,
        }
    }
}

impl<Profile: HostProfile> Clone for FutureRetention<Profile> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone_handle(),
            context: self.context.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostFutureValue;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternal,
        HostFutureStore, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostWorkProfile,
    };
    use crate::work_fixture::{WorkComponent, WorkHostType, WorkType};
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;
    use std::sync::{Arc, Mutex};

    struct Profile;
    struct Native;
    #[derive(Default)]
    struct State {
        saved: Arc<Mutex<Option<HostFutureValue<Profile, Native, BigInt>>>>,
        unit: (),
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut State) -> &mut () {
            &mut state.unit
        }
    }
    impl HostProvider<Profile> for Native {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    fn save<'call>(
        mut call: HostCall<'call, Profile, Native, ()>,
        work: HostExternal<'call, WorkHostType<BigInt>>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let work = call.future_value(work);
        let alias = work.clone();
        assert_eq!(work.work.identity(), alias.work.identity());
        *call.state().saved.lock().unwrap() = Some(alias);
        Ok(call.return_value(()))
    }

    fn restore<'call>(
        mut call: HostCall<'call, Profile, Native, WorkHostType<BigInt>>,
    ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
        let retained = call.state().saved.lock().unwrap().as_ref().unwrap().clone();
        let work = retained.restore(&mut call)?;
        Ok(call.return_value(work))
    }

    fn program() -> HostedModuleBuilder<Profile> {
        let mut providers = WorkComponent::providers::<Profile>().unwrap();
        providers.push(
            HostProviderModule::new("application", "library")
                .unwrap()
                .with_scoped_function::<Native, (WorkHostType<BigInt>,), (), _>("save", save)
                .unwrap()
                .with_scoped_function::<Native, (), WorkHostType<BigInt>, _>("restore", restore)
                .unwrap(),
        );
        HostedModuleBuilder::new(
            crate::compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "work.gleam",
                            WorkComponent::SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new(
                            "library",
                            "library.gleam",
                            r#"
import fixture/work
@external(erlang, "native", "save") fn save(value: work.Work(Int)) -> Nil
@external(erlang, "native", "restore") pub fn restore() -> work.Work(Int)
pub fn roundtrip() {
  let original = work.map(work.ready(40), fn(value) { echo "observed" value + 2 })
  save(original)
  let restored = restore()
  assert restored == original
  restored
}
"#,
                        )],
                    ),
                ],
                HostProviderSet::from_providers(providers).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn restored_work_shares_operation_and_rejects_live_foreign_and_closed_executions() {
        const REJECTED: &str =
            "host function application::library.restore failed: work belongs to another execution";
        let (mut bindings, roundtrip) = program()
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new(
                "roundtrip",
            ))
            .unwrap();
        let restore = bindings
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("restore"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let (foreign, foreign_restore) = program()
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("restore"))
            .unwrap();
        let mut foreign = foreign.seal().unwrap();
        let mut state = State::default();
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<WorkComponent>>::component_state(&mut state),
            &state.unit
        ));
        let mut foreign_state = State {
            saved: Arc::clone(&state.saved),
            unit: (),
        };
        let mut echoes = Vec::new();
        let mut foreign_echoes = Vec::new();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echoes, async |scope| {
                let work = scope.call(&roundtrip, ()).await.unwrap();
                foreign
                    .with_execution(
                        &host,
                        &mut foreign_state,
                        &mut foreign_echoes,
                        async |other| {
                            assert_eq!(
                                other
                                    .call(&foreign_restore, ())
                                    .await
                                    .map(|_| ())
                                    .unwrap_err()
                                    .to_string(),
                                REJECTED
                            );
                        },
                    )
                    .await
                    .unwrap();
                let again = scope.call(&restore, ()).await.unwrap();
                assert_eq!(
                    scope.observe(&work).await.unwrap().read(Clone::clone),
                    BigInt::from(42)
                );
                assert_eq!(
                    scope.observe(&again).await.unwrap().read(Clone::clone),
                    BigInt::from(42)
                );
            }),
        )
        .unwrap();
        assert_eq!(echoes.len(), 1);
        assert_eq!(echoes[0].value().inspect().to_string(), "\"observed\"");
        assert!(foreign_echoes.is_empty());
        host.block_on(
            module.with_execution(&host, &mut state, &mut echoes, async |scope| {
                assert_eq!(
                    scope
                        .call(&restore, ())
                        .await
                        .map(|_| ())
                        .unwrap_err()
                        .to_string(),
                    REJECTED
                );
            }),
        )
        .unwrap();
        assert_eq!(echoes.len(), 1);
    }
}
