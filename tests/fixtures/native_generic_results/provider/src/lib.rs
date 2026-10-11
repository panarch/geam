//! Public native conversion into source parameters, with original retained data.

use geam::gleam_erlang::{Charlist, GleamErlangHostProfile, service};
use geam::host::native::{NativeCall, NativeRules};
use geam::provider::advanced::NativeValue;
use geam::provider::{BigInt, GleamResult, StringValue};
use geam::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostComponentProfile,
    HostFailure, HostList, HostListType, HostProvider, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTupleType, HostTypeIndex0, HostTypeIndexNext, HostTypeList,
    HostTypeListEnd, HostTypeParameter, HostValue,
};

pub struct Component;
type A = HostTypeParameter<0>;
type B = HostTypeParameter<1>;
type One<T> = HostTypeList<T, HostTypeListEnd>;
type Pair<T, U> = HostTupleType<HostTypeList<T, One<U>>>;
type Failure = Pair<A, B>;
type ResultType = GleamResult<StringValue, Failure>;
type Path = GleamResult<Pair<StringValue, GleamResult<StringValue, ()>>, Failure>;
type PathTargets = HostTypeList<Path, HostTypeList<Charlist, One<HostListType<char>>>>;
type CharlistIndex = HostTypeIndexNext<HostTypeIndex0>;
type CharactersIndex = HostTypeIndexNext<CharlistIndex>;

impl HostProviderComponent for Component {
    const ID: &'static str = "native_generic_results_fixture";
    type Stores = ();
    type RunState = ();
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<(), HostProviderInitializationError> {
        if configuration.is_empty() {
            Ok(())
        } else {
            Err(HostProviderInitializationError::for_component::<Self>(
                "configuration must be empty",
            ))
        }
    }
}

impl<Profile: HostComponentProfile<Self>> HostProvider<Profile> for Component {
    type State = ();
    fn project(state: &mut Profile::RunState) -> &mut () {
        Profile::component_state(state)
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new(
            "native_generic_results_fixture",
            "native_generic_results_fixture",
        )
        .and_then(|module| {
            module.with_native_function::<Self, (), A, One<A>, _>(
                "produce",
                NativeRules::default(),
                produce::<Profile>,
            )
        })
        .and_then(|module| {
            module.with_native_function::<Self, (), Failure, One<Failure>, _>(
                "pair",
                NativeRules::default(),
                pair::<Profile>,
            )
        })
        .and_then(|module| {
            module.with_native_function::<Self, (), ResultType, One<ResultType>, _>(
                "native_result",
                NativeRules::default(),
                result::<Profile>,
            )
        })
        .and_then(|module| {
            module.with_resumable_native_function::<Self, (), A, One<A>, _>(
                "delayed",
                NativeRules::default(),
                delayed::<Profile>,
            )
        })
        .and_then(|module| {
            module.with_native_function::<
                Self,
                (HostListType<BigInt>,),
                HostListType<A>,
                One<HostListType<A>>,
                _,
            >("native_list", NativeRules::default(), list::<Profile>)
        })
        .and_then(|module| {
            module.with_scoped_function::<Self, (A,), A, _>("identity", identity::<Profile>)
        })
        .and_then(|module| {
            module.with_scoped_function::<Self, (A,), StringValue, _>("inspect", inspect::<Profile>)
        })
        .and_then(|module| {
            module.with_resumable_native_function::<Self, (StringValue,), Path, PathTargets, _>(
                "get_path_and_query",
                NativeRules::default(),
                path::<Profile>,
            )
        })
        .map(|module| vec![module])
    }
}

fn produce<'call, Profile: HostComponentProfile<Component>>(
    mut call: NativeCall<'call, Profile, Component, A, One<A>>,
) -> Result<HostCallCompletion<'call, A>, HostCallError> {
    let output = call
        .convert::<HostTypeIndex0>(&NativeValue::symbol("payload"))
        .ok_or_else(|| HostFailure::new("payload does not match the registered target"))?;
    Ok(call.finish(output))
}

fn pair<'call, Profile: HostComponentProfile<Component>>(
    mut call: NativeCall<'call, Profile, Component, Failure, One<Failure>>,
) -> Result<HostCallCompletion<'call, Failure>, HostCallError> {
    let integer = call.call().native_values().integer(42.into());
    let native = NativeValue::tuple([NativeValue::symbol("kind"), integer]);
    let output = call
        .convert::<HostTypeIndex0>(&native)
        .ok_or_else(|| HostFailure::new("pair does not match the registered target"))?;
    Ok(call.finish(output))
}

fn result<'call, Profile: HostComponentProfile<Component>>(
    mut call: NativeCall<'call, Profile, Component, ResultType, One<ResultType>>,
) -> Result<HostCallCompletion<'call, ResultType>, HostCallError> {
    let values = call.call().native_values();
    let native = NativeValue::tuple([
        NativeValue::symbol("error"),
        NativeValue::tuple([
            values.string("invalid_uri".into()),
            values.string("input".into()),
        ]),
    ]);
    let output = call
        .convert::<HostTypeIndex0>(&native)
        .ok_or_else(|| HostFailure::new("result does not match the registered target"))?;
    Ok(call.finish(output))
}

fn delayed<'call, Profile: HostComponentProfile<Component>>(
    call: NativeCall<'call, Profile, Component, A, One<A>>,
) -> Result<HostCallContinuation<'call, A>, HostCallError> {
    Ok(call.resume::<HostTypeIndex0>(|_| Box::pin(async { Ok(NativeValue::symbol("delayed")) })))
}

fn list<'call, Profile: HostComponentProfile<Component>>(
    mut call: NativeCall<'call, Profile, Component, HostListType<A>, One<HostListType<A>>>,
    input: HostList<'call, BigInt>,
) -> Result<HostCallCompletion<'call, HostListType<A>>, HostCallError> {
    let native = call.source::<HostListType<BigInt>>(input);
    let output = call
        .convert::<HostTypeIndex0>(&native)
        .ok_or_else(|| HostFailure::new("list does not match the registered target"))?;
    Ok(call.finish(output))
}

fn identity<'call, Profile: HostComponentProfile<Component>>(
    call: HostCall<'call, Profile, Component, A>,
    input: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, A>, HostCallError> {
    Ok(call.return_value(input))
}

fn inspect<'call, Profile: HostComponentProfile<Component>>(
    call: HostCall<'call, Profile, Component, StringValue>,
    input: HostValue<'call, A>,
) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
    let text = call.inspect::<A>(input);
    Ok(call.return_value(text.into()))
}

fn path<'call, Profile>(
    call: NativeCall<'call, Profile, Component, Path, PathTargets>,
    input: StringValue,
) -> Result<HostCallContinuation<'call, Path>, HostCallError>
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Component>,
{
    Ok(call.resume::<HostTypeIndex0>(move |context| {
        Box::pin(async move {
            context
                .with_constructions(move |mut call, targets| {
                    if input == "/[" {
                        let detail = service::charlist_from_string(
                            &mut call,
                            targets.at::<CharlistIndex>(),
                            targets.at::<CharactersIndex>(),
                            "[",
                        );
                        NativeValue::tuple([
                            NativeValue::symbol("error"),
                            NativeValue::tuple([
                                NativeValue::symbol("invalid_uri"),
                                call.native_value::<Charlist>(detail),
                            ]),
                        ])
                    } else {
                        let values = call.native_values();
                        NativeValue::tuple([
                            NativeValue::symbol("ok"),
                            NativeValue::tuple([
                                values.string(input),
                                NativeValue::tuple([
                                    NativeValue::symbol("error"),
                                    NativeValue::symbol("nil"),
                                ]),
                            ]),
                        ])
                    }
                })
                .await
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::Component;
    use geam::{HostProviderComponentInitialization, HostProviderConfiguration};
    use std::collections::BTreeMap;

    #[test]
    fn component_requires_empty_configuration() {
        assert_eq!(
            Component::initialize(&HostProviderConfiguration::empty()),
            Ok(())
        );
        let error = Component::initialize(&HostProviderConfiguration::new(BTreeMap::from([(
            "unexpected".into(),
            true.into(),
        )])))
        .unwrap_err();
        assert_eq!(error.component_id(), "native_generic_results_fixture");
        assert_eq!(error.reason(), "configuration must be empty");
    }
}
