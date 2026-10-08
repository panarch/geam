use crate::execution::ReferenceId;
use crate::{Component, GleamErlangHostProfile, Reference as HostReference, ReferenceSchema};
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostProvider, HostType,
    HostTypeListEnd,
};
use geam_core::provider::{
    ProviderConstruction, ProviderConstructions, ProviderConvertedStorage,
    ProviderExternalPayloadAccess, ProviderInputValue, ProviderListInputCodec,
    ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
    ProviderNoConstructions, ProviderOutputValue, ProviderOwnedExternal, ProviderRootOutputValue,
    ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};

/// A retained producer-owned reference. Fresh values use the same identity store
/// and equality rules as `gleam/erlang/reference.new`.
#[derive(Clone)]
pub struct Reference {
    value: Representation,
}

#[derive(Clone)]
enum Representation {
    Source(ProviderOwnedExternal<crate::reference::Payload>),
    Identity(ReferenceId),
}

impl Reference {
    pub(super) fn new() -> Self {
        Self {
            value: Representation::Identity(ReferenceId::new()),
        }
    }
}

impl ProviderValue for Reference {
    type Host = HostReference;
    type OutputRequirements = ProviderConstruction<HostReference>;
    type RootRequirements = Self::OutputRequirements;
}

impl ProviderValueForms for Reference {
    type InvocationRequirements = ();
    type Runtime<Profile: geam_core::HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = ReferenceListDecoder;
    type OwnedListDecoder = ReferenceListDecoder;
}

#[doc(hidden)]
pub struct ReferenceListDecoder(ProviderExternalPayloadAccess<crate::reference::Payload>);
impl Clone for ReferenceListDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ProviderListItemDecoder<Reference> for ReferenceListDecoder {
    type View = Reference;
    fn decode(&self, value: ProviderListItemValue<'_>) -> Reference {
        Reference {
            value: Representation::Source(value.into_external(&self.0)),
        }
    }
}

impl ProviderTypedListItemDecoder<Reference> for ReferenceListDecoder {
    type Host = HostReference;
}
impl ProviderListInputValue for Reference {
    type Host = HostReference;
    type View = Self;
    type Decoder = ReferenceListDecoder;
}
impl<Profile: GleamErlangHostProfile, Provider: HostProvider<Profile>>
    ProviderListInputCodec<Profile, Provider> for Reference
{
    type Requirements = ProviderNoConstructions;
    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> ReferenceListDecoder {
        ReferenceListDecoder(
            call.provider_external_payload_access_with::<Component<Profile>, ReferenceSchema>(),
        )
    }
}

impl<Profile, Provider, Return> ProviderInputValue<Profile, Provider, Return> for Reference
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host = HostReference;
    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostExternal<'call, HostReference>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self { value: Representation::Source(call.provider_external_item_with::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(value)) }
    }
}

impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for Reference
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error = HostCallError;
    type Storage = ProviderConvertedStorage;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostExternal<'call, HostReference>, Self::Error> {
        Ok(match self.value {
            Representation::Source(value) => call.provider_external_from_item::<ReferenceSchema, HostTypeListEnd, crate::reference::Payload>(value)?,
            Representation::Identity(id) => call.construct_external_with_binding::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(constructions.token(), crate::reference::Payload::Identity(id)),
        })
    }
}

impl<Profile, Provider> ProviderRootOutputValue<Profile, Provider> for Reference
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, HostReference>,
        constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, HostReference>, HostCallError> {
        let value = self.into_host(&mut call, constructions)?;
        Ok(call.return_value(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Reference;
    use crate::{Component, GleamErlangProfile, Reference as HostReference, ReferenceSchema};
    use geam_core::host::HostProviderSet;
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostList,
        HostListType, HostProviderModule, HostTypeList, HostTypeListEnd,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use std::sync::{Arc, Mutex};

    type Call<'call> =
        HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, HostReference>;
    type Constructions<'call> =
        HostConstructions<'call, HostTypeList<HostReference, HostTypeListEnd>>;

    type Source<'call> = HostExternal<'call, HostReference>;
    type Items<'call> = HostList<'call, HostReference>;
    type Completion<'call> = Result<HostCallCompletion<'call, HostReference>, HostCallError>;

    #[test]
    fn retained_values_preserve_identity_and_reject_a_previous_execution() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, ReferenceSchema>()
            .unwrap();
        let previous = Arc::new(Mutex::new(None::<Reference>));
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), HostReference, HostTypeList<HostReference, HostTypeListEnd>, _>("make", make)
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (HostReference, HostListType<HostReference>), HostReference, HostTypeList<HostReference, HostTypeListEnd>, _>("round_trip", retaining_round_trip(previous))
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/reference",
                        "producer.gleam",
                        "pub type Reference",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/erlang/reference as p
@external(erlang, "host", "make") fn make() -> p.Reference
@external(erlang, "host", "round_trip") fn round_trip(value: p.Reference, items: List(p.Reference)) -> p.Reference
pub fn main() {
  let value = make()
  value == round_trip(value, [value, value])
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([producer, consumer]).unwrap(),
        ).unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut state = crate::GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo),
            Ok(geam_core::Value::Bool(true))
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo).map_err(|error| error.to_string()),
            Err("host function application::main.round_trip failed: retained value belongs to another execution".into()),
        );
        assert!(echo.is_empty());
    }

    fn retaining_round_trip(
        previous: Arc<Mutex<Option<Reference>>>,
    ) -> impl for<'call> Fn(
        Call<'call>,
        Constructions<'call>,
        Source<'call>,
        Items<'call>,
    ) -> Completion<'call> {
        move |mut call, constructions, original, items| {
            let retained = <Reference>::from_host(&mut call, original);
            let previous = previous.lock().unwrap().replace(retained);
            match previous {
                None => round_trip(call, constructions, original, items),
                Some(previous) => {
                    let constructions = ProviderConstructions::new(&constructions);
                    assert_eq!(
                        previous
                            .clone()
                            .into_host(&mut call, &constructions)
                            .err()
                            .map(|error| error.to_string()),
                        Some("retained value belongs to another execution".into()),
                    );
                    previous.complete(call, &constructions)
                }
            }
        }
    }

    fn make<'call>(call: Call<'call>, constructions: Constructions<'call>) -> Completion<'call> {
        let value = Reference::new();
        value
            .clone()
            .complete(call, &ProviderConstructions::new(&constructions))
    }

    fn round_trip<'call>(
        mut call: Call<'call>,
        constructions: Constructions<'call>,
        original: Source<'call>,
        items: Items<'call>,
    ) -> Completion<'call> {
        let expected_hash = call.source_hash::<HostReference>(original);
        let expected_inspection = call.inspect::<HostReference>(original);
        let retained = <Reference>::from_host(&mut call, original);
        let alias = retained.clone();
        drop(retained);

        let decoder = <Reference as ProviderListInputCodec<
            GleamErlangProfile,
            Component<GleamErlangProfile>,
        >>::decoder_with(&call, &ProviderConstructions::none());
        let alias_decoder = decoder.clone();
        drop(decoder);
        let items = call.provider_retained_list::<Reference, _, _>(items, alias_decoder);
        assert_eq!(items.len(), 2);
        assert!(items.get(2).is_none());
        let first = items.get(0).unwrap();
        let second = items.get(1).unwrap();
        drop(items);
        let constructions = ProviderConstructions::new(&constructions);
        for item in [first, second, alias] {
            let restored = item
                .into_host(&mut call, &constructions)
                .expect("current execution retains the exact producer value");
            assert!(call.equal::<HostReference>(original, restored));
            assert_eq!(call.source_hash::<HostReference>(restored), expected_hash);
            assert_eq!(call.inspect::<HostReference>(restored), expected_inspection);
        }
        <Reference>::from_host(&mut call, original).complete(call, &constructions)
    }
}
