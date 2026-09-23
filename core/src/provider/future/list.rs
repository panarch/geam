use super::{
    Decode, Future, FutureProvider, ProviderFutureCodec, ProviderFutureValueContext, decode,
};
use crate::host::{HostCall, HostProfile, HostProvider, HostType, HostWorkProfile};
use crate::provider::{ProviderConstructions, ProviderListItemDecoder, ProviderListItemValue};
use std::marker::PhantomData;

/// Retains a demanded work item with its original result codec and execution.
#[doc(hidden)]
pub struct ProviderFutureListDecoder<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    retention: crate::host::FutureRetention<Profile>,
    values: crate::runtime::ValueRetention,
    decode: Decode<Profile, Host, Output>,
    callable_base: usize,
}

impl<Profile, Host, Output> ProviderFutureListDecoder<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    pub fn from_host_with<'call, Codec, Provider, CallerProvider, Return>(
        call: &HostCall<'call, Profile, CallerProvider, Return>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Profile: HostWorkProfile,
        Provider: HostProvider<Profile>,
        CallerProvider: HostProvider<Profile>,
        Return: HostType,
        Codec: ProviderFutureCodec<Profile, Provider, Host = Host, Output = Output>,
    {
        Self {
            retention: call.future_retention(),
            values: call.value_retention(),
            decode: decode::<Profile, Provider, Codec>,
            callable_base: constructions.host().callable_base(),
        }
    }
}

impl<Profile, Host, Output> Clone for ProviderFutureListDecoder<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    fn clone(&self) -> Self {
        Self {
            retention: self.retention.clone(),
            values: self.values.clone(),
            decode: self.decode,
            callable_base: self.callable_base,
        }
    }
}

impl<Source, Profile, Host, Output> ProviderListItemDecoder<Future<Source>>
    for ProviderFutureListDecoder<Profile, Host, Output>
where
    Profile: HostProfile,
    Host: HostType,
{
    type View = Future<Source, ProviderFutureValueContext<Profile, Host, Output>>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        let (value, lease) = value.into_stored_external(&self.values);
        Future {
            context: ProviderFutureValueContext {
                work: self.retention.bind::<FutureProvider, Host>(value, &lease),
                decode: self.decode,
                callable_base: self.callable_base,
            },
            value: PhantomData,
        }
    }
}

impl<Source, Profile, Host, Output> crate::provider::ProviderTypedListItemDecoder<Future<Source>>
    for ProviderFutureListDecoder<Profile, Host, Output>
where
    Profile: HostWorkProfile,
    Host: HostType,
{
    type Host = crate::host::HostFutureType<Host, crate::host::HostWorkSchema<Profile>>;
}

#[cfg(test)]
mod tests {
    use super::ProviderFutureListDecoder;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
        HostFutureStore, HostList, HostListType, HostOwnedCompletion, HostProfile, HostProvider,
        HostProviderModule, HostProviderSet, HostTypeListEnd, HostWorkProfile,
    };
    use crate::provider::{
        Call, Future, ProviderConstructions, ProviderFutureCodec, ProviderNoConstructions,
    };
    use crate::work_fixture::{WorkComponent, WorkHostType, WorkType};
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;

    struct Profile;
    #[derive(Default)]
    struct State {
        decoded: Vec<BigInt>,
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
    struct Native;
    impl HostProvider<Profile> for Native {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }
    struct Observer;
    impl HostProvider<Profile> for Observer {
        type State = ();
        fn project(state: &mut State) -> &mut () {
            &mut state.unit
        }
    }
    struct Codec;
    impl ProviderFutureCodec<Profile, Native> for Codec {
        type Host = BigInt;
        type Output = BigInt;
        type Requirements = ProviderNoConstructions;
        fn decode<'call>(
            call: &mut HostCall<'call, Profile, Native, ()>,
            value: BigInt,
            _: &ProviderConstructions<'call, ProviderNoConstructions>,
        ) -> BigInt {
            call.state().decoded.push(value.clone());
            value
        }
    }

    fn select<'call>(
        call: HostCall<'call, Profile, Observer, WorkHostType<BigInt>>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        values: HostList<'call, WorkHostType<BigInt>>,
    ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
        let decoder = ProviderFutureListDecoder::from_host_with::<Codec, Native, _, _>(
            &call,
            ProviderConstructions::none(),
        );
        let alias = decoder.clone();
        drop(decoder);
        let values = call.provider_retained_list::<Future<BigInt>, _, _>(values, alias);
        assert_eq!(values.len(), 2);
        let work = values.get(1).unwrap();
        let context = values.__geam_into_context();
        assert_eq!(context.retained().item_reads(), 1);
        drop(context);
        let alias = work.clone();
        Ok(call.return_future(constructions, move |context| {
            Box::pin(async move {
                let mut call = Call::from_future_context(context);
                let first = call.observe(&work).await.unwrap();
                let second = call.observe(&alias).await.unwrap();
                Ok(HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(first + second))
                }))
            })
        }))
    }

    #[test]
    fn demanded_work_survives_its_list_and_call_with_the_original_decoder_and_one_operation() {
        let mut providers = WorkComponent::providers::<Profile>().unwrap();
        providers.push(
            HostProviderModule::new("application", "main")
                .unwrap()
                .with_scoped_function_and_constructions::<
                    Observer,
                    (HostListType<WorkHostType<BigInt>>,),
                    WorkHostType<BigInt>,
                    HostTypeListEnd,
                    _,
                >("select", select)
                .unwrap(),
        );
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
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
                        "main",
                        "main.gleam",
                        r#"
import fixture/work
@external(erlang, "native", "select")
fn select(values: List(work.Work(Int))) -> work.Work(Int)
pub fn main() {
  select([
    work.map(work.ready(100), fn(value) { echo "unused" value }),
    work.map(work.ready(40), fn(value) { echo "selected" value + 2 }),
  ])
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap();
        let (builder, entry) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("main"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let mut state = State::default();
        assert_eq!(
            std::ptr::from_mut(Profile::component_state(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        assert_eq!(
            std::ptr::from_mut(Observer::project(&mut state)),
            std::ptr::addr_of_mut!(state.unit)
        );
        let mut echoes = Vec::new();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echoes, async |scope| {
                let work = scope.call(&entry, ()).await.unwrap();
                assert_eq!(
                    scope.observe(&work).await.unwrap().read(Clone::clone),
                    BigInt::from(84)
                );
                assert_eq!(
                    scope.observe(&work).await.unwrap().read(Clone::clone),
                    BigInt::from(84)
                );
            }),
        )
        .unwrap();
        assert_eq!(state.decoded, [BigInt::from(42), BigInt::from(42)]);
        assert_eq!(
            echoes
                .iter()
                .map(|echo| echo.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["\"selected\""]
        );
    }
}
