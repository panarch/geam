//! An independent manual consumer of the producer-owned Charlist construction API.

use geam::gleam_erlang::{Charlist, GleamErlangHostProfile, service};
use geam::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
    HostListType, HostProvider, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderModule, HostRegistrationError, HostTupleType, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd,
};
use geam::provider::{BigInt, StringValue};

pub struct Component;

type Constructions = HostTypeList<Charlist, HostTypeList<HostListType<char>, HostTypeListEnd>>;
type Header = HostTupleType<HostTypeList<Charlist, HostTypeList<Charlist, HostTypeListEnd>>>;
type Headers = HostListType<Header>;
type Status = HostTupleType<
    HostTypeList<Charlist, HostTypeList<BigInt, HostTypeList<Charlist, HostTypeListEnd>>>,
>;
type Metadata = HostTupleType<HostTypeList<Status, HostTypeList<Headers, HostTypeListEnd>>>;
type MetadataConstructions = HostTypeList<
    Charlist,
    HostTypeList<
        HostListType<char>,
        HostTypeList<Header, HostTypeList<Headers, HostTypeList<Status, HostTypeListEnd>>>,
    >,
>;
type CharactersIndex = HostTypeIndexNext<HostTypeIndex0>;
type HeaderIndex = HostTypeIndexNext<CharactersIndex>;
type HeadersIndex = HostTypeIndexNext<HeaderIndex>;
type StatusIndex = HostTypeIndexNext<HeadersIndex>;

impl HostProviderComponent for Component {
    const ID: &'static str = "charlist_service_fixture";
    type Stores = ();
    type RunState = ();
}

impl<Profile: HostComponentProfile<Self>> HostProvider<Profile> for Component {
    type State = ();

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::component_state(state)
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("charlist_service_fixture", "charlist_service_fixture")
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self,
                    (StringValue,),
                    Charlist,
                    Constructions,
                    _,
                >("from_text", from_text::<Profile>)
            })
            .and_then(|provider| {
                provider
                    .with_scoped_function_and_constructions::<Self, (), Header, Constructions, _>(
                        "header",
                        header::<Profile>,
                    )
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self,
                    (),
                    Metadata,
                    MetadataConstructions,
                    _,
                >("metadata", metadata::<Profile>)
            })
            .map(|provider| vec![provider])
    }
}

fn from_text<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, Charlist>,
    constructions: HostConstructions<'call, Constructions>,
    text: StringValue,
) -> Result<HostCallCompletion<'call, Charlist>, HostCallError>
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Component>,
{
    let value = service::charlist_from_string(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        constructions.at::<CharactersIndex>(),
        &text,
    );
    Ok(call.return_value(value))
}

fn header<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, Header>,
    constructions: HostConstructions<'call, Constructions>,
) -> Result<HostCallCompletion<'call, Header>, HostCallError>
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Component>,
{
    let name = service::charlist_from_string(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        constructions.at::<CharactersIndex>(),
        "user-agent",
    );
    let value = service::charlist_from_string(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        constructions.at::<CharactersIndex>(),
        "geam-charlist-fixture/1.0",
    );
    Ok(call.return_tuple((name, (value, ()))))
}

fn metadata<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, Metadata>,
    constructions: HostConstructions<'call, MetadataConstructions>,
) -> Result<HostCallCompletion<'call, Metadata>, HostCallError>
where
    Profile: GleamErlangHostProfile + HostComponentProfile<Component>,
{
    let version = service::charlist_from_string(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        constructions.at::<CharactersIndex>(),
        "HTTP/1.1",
    );
    let reason = service::charlist_from_string(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        constructions.at::<CharactersIndex>(),
        "OK",
    );
    let status = call.construct_tuple(
        constructions.at::<StatusIndex>(),
        (version, (BigInt::from(200), (reason, ()))),
    );
    let headers = [("x-empty", ""), ("x-unicode", "\0Aé🙂")].map(|(name, value)| {
        let name = service::charlist_from_string(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            constructions.at::<CharactersIndex>(),
            name,
        );
        let value = service::charlist_from_string(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            constructions.at::<CharactersIndex>(),
            value,
        );
        call.construct_tuple(constructions.at::<HeaderIndex>(), (name, (value, ())))
    });
    let headers = call.construct_list(constructions.at::<HeadersIndex>(), headers);
    Ok(call.return_tuple((status, (headers, ()))))
}

#[cfg(test)]
mod tests {
    use super::Component;
    use geam::host::{HostComponentProfile, HostProfile, HostProvider};

    struct Profile;

    impl HostProfile for Profile {
        type RunState = (u8, ());
        type ExternalStores = ();
        type ExecutionState = ();
    }

    impl HostComponentProfile<Component> for Profile {
        fn component_stores(stores: &()) -> &() {
            stores
        }

        fn component_state(state: &mut (u8, ())) -> &mut () {
            &mut state.1
        }
    }

    #[test]
    fn component_projects_the_profile_selected_state_slot() {
        let stores = ();
        assert!(std::ptr::eq(Profile::component_stores(&stores), &stores));
        let mut state = (42, ());
        let expected = &raw mut state.1;
        let actual = <Component as HostProvider<Profile>>::project(&mut state);
        assert!(std::ptr::eq(actual, expected));
        assert_eq!(state.0, 42);
    }
}
