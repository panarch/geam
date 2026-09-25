//! An independent manual consumer of typed Dict and canonical Result construction.

use geam::gleam_stdlib::{Component as StdlibComponent, DictOf, GleamStdlibHostProfile, service};
use geam::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
    HostListType, HostProvider, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderModule, HostRegistrationError, HostTupleType, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd,
};
use geam::provider::{BigInt, GleamError, GleamOk, GleamResult, StringValue};

pub struct Component;

type TextDict = DictOf<StringValue, StringValue>;
type TextConstructions = HostTypeList<TextDict, HostTypeListEnd>;
type EntryResult = GleamResult<TextDict, ()>;
type Strings = HostListType<StringValue>;
type Groups = DictOf<BigInt, Strings>;
type GroupConstructions = HostTypeList<Groups, HostTypeList<Strings, HostTypeListEnd>>;
type Dicts = HostListType<TextDict>;
type Nested = HostTupleType<HostTypeList<TextDict, HostTypeList<Dicts, HostTypeListEnd>>>;
type NestedConstructions = HostTypeList<TextDict, HostTypeList<Dicts, HostTypeListEnd>>;
type ListIndex = HostTypeIndexNext<HostTypeIndex0>;

impl HostProviderComponent for Component {
    const ID: &'static str = "dict_service_fixture";
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
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("dict_service_fixture", "dict_service_fixture")
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self, (bool,), TextDict, TextConstructions, _,
                >("entries", entries::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self, (), Groups, GroupConstructions, _,
                >("groups", groups::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self, (), Nested, NestedConstructions, _,
                >("nested", nested::<Profile>)
            })
            .and_then(|provider| {
                provider
                    .with_scoped_function::<Self, (StringValue,), GleamResult<StringValue, ()>, _>(
                        "lookup",
                        lookup::<Profile>,
                    )
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Self, (bool,), EntryResult, TextConstructions, _,
                >("try_entries", try_entries::<Profile>)
            })
            .map(|provider| vec![provider])
    }
}

fn entries<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, TextDict>,
    constructions: HostConstructions<'call, TextConstructions>,
    empty: bool,
) -> Result<HostCallCompletion<'call, TextDict>, HostCallError>
where
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Component>,
{
    let entries = [("LANG", "old"), ("EMPTY", ""), ("LANG", "한국어\0🙂")]
        .into_iter()
        .take(if empty { 0 } else { 3 })
        .map(|(key, item)| (StringValue::from(key), StringValue::from(item)));
    let dict = service::dict_from_entries(&mut call, constructions.at::<HostTypeIndex0>(), entries);
    Ok(call.return_value(dict))
}

fn groups<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, Groups>,
    constructions: HostConstructions<'call, GroupConstructions>,
) -> Result<HostCallCompletion<'call, Groups>, HostCallError>
where
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Component>,
{
    let first = call.construct_list(
        constructions.at::<ListIndex>(),
        [StringValue::from("one"), StringValue::from("하나")],
    );
    let second = call.construct_list(constructions.at::<ListIndex>(), []);
    let dict = service::dict_from_entries(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        [(BigInt::from(1), first), (BigInt::from(2), second)],
    );
    Ok(call.return_value(dict))
}

fn nested<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, Nested>,
    constructions: HostConstructions<'call, NestedConstructions>,
) -> Result<HostCallCompletion<'call, Nested>, HostCallError>
where
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Component>,
{
    let outer = service::dict_from_entries(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        [(StringValue::from("outer"), StringValue::from("ready"))],
    );
    let inner = service::dict_from_entries(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        [(StringValue::from("inner"), StringValue::from("nested"))],
    );
    let empty = service::dict_from_entries(
        &mut call,
        constructions.at::<HostTypeIndex0>(),
        std::iter::empty(),
    );
    let dicts = call.construct_list(constructions.at::<ListIndex>(), [inner, empty]);
    Ok(call.return_tuple((outer, (dicts, ()))))
}

fn lookup<'call, Profile>(
    call: HostCall<'call, Profile, Component, GleamResult<StringValue, ()>>,
    key: StringValue,
) -> Result<HostCallCompletion<'call, GleamResult<StringValue, ()>>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let item = match key.as_str() {
        "LANG" => Some("한국어\0🙂"),
        "EMPTY" => Some(""),
        "MESSAGE" => Some("ready"),
        _ => None,
    };
    match item {
        Some(item) => Ok(call.return_custom::<GleamOk<StringValue, ()>>((item.into(), ()))),
        None => Ok(call.return_custom::<GleamError<StringValue, ()>>(((), ()))),
    }
}

fn try_entries<'call, Profile>(
    mut call: HostCall<'call, Profile, Component, EntryResult>,
    constructions: HostConstructions<'call, TextConstructions>,
    available: bool,
) -> Result<HostCallCompletion<'call, EntryResult>, HostCallError>
where
    Profile: GleamStdlibHostProfile
        + HostComponentProfile<StdlibComponent<Profile::Io>>
        + HostComponentProfile<Component>,
{
    if available {
        let dict = service::dict_from_entries(
            &mut call,
            constructions.at::<HostTypeIndex0>(),
            [(StringValue::from("status"), StringValue::from("ready"))],
        );
        Ok(call.return_custom::<GleamOk<TextDict, ()>>((dict, ())))
    } else {
        Ok(call.return_custom::<GleamError<TextDict, ()>>(((), ())))
    }
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
