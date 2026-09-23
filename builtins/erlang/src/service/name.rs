use crate::{Component, GleamErlangHostProfile, Name as HostName, NameSchema};
use ecow::EcoString;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostProvider, HostType,
    HostTypeList, HostTypeListEnd,
};
use geam_core::provider::{
    ProviderConstruction, ProviderConstructions, ProviderExternalPayloadAccess, ProviderInputValue,
    ProviderListInputCodec, ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
    ProviderNoConstructions, ProviderOutputValue, ProviderOwnedExternal, ProviderRootOutputValue,
    ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};
use std::marker::PhantomData;

/// A producer-owned name with its original phantom message specialization.
/// Cloning retains the same source payload; it never creates or registers a name.
pub struct Name<Message> {
    value: Representation,
    message: PhantomData<fn() -> Message>,
}

#[derive(Clone)]
enum Representation {
    Source(ProviderOwnedExternal<EcoString>),
    Fresh(EcoString),
}

impl<Message> Clone for Name<Message> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            message: PhantomData,
        }
    }
}
impl<Message> Name<Message> {
    pub(super) fn new(name: EcoString) -> Self {
        Self {
            value: Representation::Fresh(name),
            message: PhantomData,
        }
    }

    pub(super) fn with_name<Output>(&self, read: impl FnOnce(&str) -> Output) -> Output {
        match &self.value {
            Representation::Source(value) => value.with(|name| read(name)),
            Representation::Fresh(name) => read(name),
        }
    }
}
impl<Message: ProviderValue> ProviderValue for Name<Message> {
    type Host = HostName<Message::Host>;
    type OutputRequirements = ProviderConstruction<HostName<Message::Host>>;
    type RootRequirements = Self::OutputRequirements;
}
impl<Message: ProviderValue + 'static> ProviderValueForms for Name<Message> {
    type InvocationRequirements = ();
    type Runtime<Profile: geam_core::HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = NameListDecoder<Message>;
    type OwnedListDecoder = NameListDecoder<Message>;
}
#[doc(hidden)]
pub struct NameListDecoder<Message> {
    access: ProviderExternalPayloadAccess<EcoString>,
    message: PhantomData<fn() -> Message>,
}
impl<Message> Clone for NameListDecoder<Message> {
    fn clone(&self) -> Self {
        Self {
            access: self.access.clone(),
            message: PhantomData,
        }
    }
}
impl<Message> ProviderListItemDecoder<Name<Message>> for NameListDecoder<Message> {
    type View = Name<Message>;
    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Name {
            value: Representation::Source(value.into_external(&self.access)),
            message: PhantomData,
        }
    }
}
impl<Message: ProviderValue> ProviderTypedListItemDecoder<Name<Message>>
    for NameListDecoder<Message>
{
    type Host = HostName<Message::Host>;
}
impl<Message: ProviderValue + 'static> ProviderListInputValue for Name<Message> {
    type Host = HostName<Message::Host>;
    type View = Self;
    type Decoder = NameListDecoder<Message>;
}
impl<Profile, Provider, Message> ProviderListInputCodec<Profile, Provider> for Name<Message>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Message: ProviderValue + 'static,
{
    type Requirements = ProviderNoConstructions;
    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder {
        NameListDecoder {
            access: call.provider_external_payload_access_with::<Component<Profile>, NameSchema>(),
            message: PhantomData,
        }
    }
}
impl<Profile, Provider, Return, Message> ProviderInputValue<Profile, Provider, Return>
    for Name<Message>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: ProviderValue,
{
    type Host = HostName<Message::Host>;
    type Requirements = ProviderNoConstructions;
    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostExternal<'call, Self::Host>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self { value: Representation::Source(call.provider_external_item_with::<Component<Profile>, NameSchema, HostTypeList<Message::Host, HostTypeListEnd>>(value)), message: PhantomData }
    }
}
impl<Profile, Provider, Return, Message> ProviderOutputValue<Profile, Provider, Return>
    for Name<Message>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: ProviderValue,
{
    type Error = std::convert::Infallible;
    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostExternal<'call, Self::Host>, Self::Error> {
        Ok(match self.value {
            Representation::Source(value) => call.provider_external_from_item::<NameSchema, HostTypeList<Message::Host, HostTypeListEnd>, EcoString>(value),
            Representation::Fresh(value) => call.construct_external_with_binding::<Component<Profile>, NameSchema, HostTypeList<Message::Host, HostTypeListEnd>>(constructions.token(), value),
        })
    }
}
impl<Profile, Provider, Message> ProviderRootOutputValue<Profile, Provider> for Name<Message>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Message: ProviderValue,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, Self::Host>,
        constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
        let name = self.into_host_infallible(&mut call, constructions);
        Ok(call.return_value(name))
    }
}

#[cfg(test)]
mod tests {
    use super::Name;
    use crate::{Component, GleamErlangProfile, Name as HostName, NameSchema};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostList,
        HostListType, HostProviderModule, HostTypeList, HostTypeListEnd,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue,
    };
    use geam_core::{ModuleSource, PackageSource};
    use num_bigint::BigInt;

    type Call<'call> =
        HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, HostName<BigInt>>;
    type Constructions<'call> =
        HostConstructions<'call, HostTypeList<HostName<BigInt>, HostTypeListEnd>>;

    #[test]
    fn retained_values_and_list_decoders_preserve_the_producer_identity() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, NameSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), HostName<BigInt>, HostTypeList<HostName<BigInt>, HostTypeListEnd>, _>("make", make)
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (HostName<BigInt>, HostListType<HostName<BigInt>>), HostName<BigInt>, HostTypeList<HostName<BigInt>, HostTypeListEnd>, _>("round_trip", round_trip)
            .unwrap();
        let result = crate::test_support::run_main(
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/process",
                        "producer.gleam",
                        "pub type Name(a)",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/erlang/process as p
@external(erlang, "host", "make") fn make() -> p.Name(Int)
@external(erlang, "host", "round_trip") fn round_trip(value: p.Name(Int), items: List(p.Name(Int))) -> p.Name(Int)
pub fn main() {
  let value = make()
  value == round_trip(value, [value, value])
}
"#,
                    )],
                ),
            ],
            [producer, consumer],
        );
        assert_eq!(result, geam_core::Value::Bool(true));
    }

    fn make<'call>(
        call: Call<'call>,
        constructions: Constructions<'call>,
    ) -> Result<HostCallCompletion<'call, HostName<BigInt>>, HostCallError> {
        let value = Name::<BigInt>::new("worker".into());
        assert_eq!(value.with_name(str::to_owned), "worker");
        value
            .clone()
            .complete(call, &ProviderConstructions::new(&constructions))
    }

    fn round_trip<'call>(
        mut call: Call<'call>,
        constructions: Constructions<'call>,
        original: HostExternal<'call, HostName<BigInt>>,
        items: HostList<'call, HostName<BigInt>>,
    ) -> Result<HostCallCompletion<'call, HostName<BigInt>>, HostCallError> {
        let expected_hash = call.source_hash::<HostName<BigInt>>(original);
        let expected_inspection = call.inspect::<HostName<BigInt>>(original);
        let retained = <Name<BigInt>>::from_host(&mut call, original);
        let alias = retained.clone();
        drop(retained);
        assert_eq!(alias.with_name(str::to_owned), "worker");
        let decoder = <Name<BigInt> as ProviderListInputCodec<
            GleamErlangProfile,
            Component<GleamErlangProfile>,
        >>::decoder_with(&call, &ProviderConstructions::none());
        let alias_decoder = decoder.clone();
        drop(decoder);
        let items = call.provider_retained_list::<Name<BigInt>, _, _>(items, alias_decoder);
        assert_eq!(items.len(), 2);
        assert!(items.get(2).is_none());
        let first = items.get(0).unwrap();
        let second = items.get(1).unwrap();
        drop(items);
        let constructions = ProviderConstructions::new(&constructions);
        for item in [first, second, alias] {
            let restored = item.into_host_infallible(&mut call, &constructions);
            assert!(call.equal::<HostName<BigInt>>(original, restored));
            assert_eq!(
                call.source_hash::<HostName<BigInt>>(restored),
                expected_hash
            );
            assert_eq!(
                call.inspect::<HostName<BigInt>>(restored),
                expected_inspection
            );
        }
        <Name<BigInt>>::from_host(&mut call, original).complete(call, &constructions)
    }
}
