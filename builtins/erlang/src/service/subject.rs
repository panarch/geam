use super::types::Subject as HostSubject;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostProfile, HostProvider, HostType,
};
use geam_core::provider::{
    ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderListInputValue,
    ProviderListItemDecoder, ProviderListItemValue, ProviderNoConstructions, ProviderOutputValue,
    ProviderRootOutputValue, ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue,
    ProviderValueContext, ProviderValueForms, ProviderValueListDecoder, Value,
};

/// An original subject with its exact source message specialization.
/// Passing it through a provider retains the source custom value and its tag.
pub struct Subject<Message: ProviderValue> {
    value: Value<Self, ProviderValueContext<HostSubject<Message::Host>>>,
}

impl<Message: ProviderValue> Subject<Message> {
    pub(super) fn into_host<'call, Profile, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> HostCustom<'call, HostSubject<Message::Host>>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        self.value.into_host(call)
    }
}

impl<Message: ProviderValue> ProviderValue for Subject<Message> {
    type Host = HostSubject<Message::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Message: ProviderValue + 'static> ProviderValueForms for Subject<Message> {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = SubjectListDecoder<Message>;
    type OwnedListDecoder = SubjectListDecoder<Message>;
}

#[doc(hidden)]
pub struct SubjectListDecoder<Message: ProviderValue>(ProviderValueListDecoder<Subject<Message>>);

impl<Message: ProviderValue> Clone for SubjectListDecoder<Message> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Message: ProviderValue> ProviderListItemDecoder<Subject<Message>>
    for SubjectListDecoder<Message>
{
    type View = Subject<Message>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Subject {
            value: self.0.decode(value),
        }
    }
}

impl<Message: ProviderValue> ProviderTypedListItemDecoder<Subject<Message>>
    for SubjectListDecoder<Message>
{
    type Host = HostSubject<Message::Host>;
}

impl<Message: ProviderValue + 'static> ProviderListInputValue for Subject<Message> {
    type Host = HostSubject<Message::Host>;
    type View = Self;
    type Decoder = SubjectListDecoder<Message>;
}

impl<Profile, Provider, Message> ProviderListInputCodec<Profile, Provider> for Subject<Message>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Message: ProviderValue + 'static,
{
    type Requirements = ProviderNoConstructions;

    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder {
        SubjectListDecoder(
            <Value<Self> as ProviderListInputCodec<Profile, Provider>>::decoder_with(
                call,
                constructions,
            ),
        )
    }
}

impl<Profile, Provider, Return, Message> ProviderInputValue<Profile, Provider, Return>
    for Subject<Message>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: ProviderValue,
{
    type Host = HostSubject<Message::Host>;
    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostCustom<'call, Self::Host>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self {
            value: Value::from_host(call, value),
        }
    }
}

impl<Profile, Provider, Return, Message> ProviderOutputValue<Profile, Provider, Return>
    for Subject<Message>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: ProviderValue,
{
    type Error = std::convert::Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostCustom<'call, Self::Host>, Self::Error> {
        Ok(self.into_host(call))
    }
}

impl<Profile, Provider, Message> ProviderRootOutputValue<Profile, Provider> for Subject<Message>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Message: ProviderValue,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, Self::Host>,
        _: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
        let subject = self.into_host(&mut call);
        Ok(call.return_value(subject))
    }
}

#[cfg(test)]
mod tests {
    use super::Subject;
    use crate::service::types::{Subject as HostSubject, SubjectSchema};
    use crate::{Component, GleamErlangProfile, Name, NameSchema, PidSchema};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostCustom, HostList, HostListType,
        HostProviderModule,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue,
    };
    use geam_core::{ModuleSource, PackageSource};
    use num_bigint::BigInt;

    #[test]
    fn retained_subject_and_cloned_list_decoder_preserve_the_exact_specialization() {
        let process = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_shared_custom_type::<SubjectSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>()
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, NameSchema>()
            .unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (), Name<BigInt>, _>(
                "name", name,
            )
            .unwrap();
        let dynamic = HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, geam_stdlib::provider_support::DynamicSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (HostSubject<BigInt>, HostListType<HostSubject<BigInt>>), HostSubject<BigInt>, _>("round_trip", round_trip)
            .unwrap();
        let result = crate::test_support::run_main(
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/dynamic",
                        "dynamic.gleam",
                        "pub type Dynamic",
                    )],
                ),
                PackageSource::new(
                    "gleam_erlang",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "gleam/erlang/process",
                        "process.gleam",
                        r#"
import gleam/dynamic.{type Dynamic}
pub type Pid
pub type Name(a)
pub opaque type Subject(message) { Subject(owner: Pid, tag: Dynamic) NamedSubject(name: Name(message)) }
@external(erlang, "host", "name") fn name() -> Name(Int)
pub fn make() -> Subject(Int) { NamedSubject(name()) }
"#,
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
@external(erlang, "host", "round_trip")
fn round_trip(subject: p.Subject(Int), items: List(p.Subject(Int))) -> p.Subject(Int)
pub fn main() {
  let subject = p.make()
  subject == round_trip(subject, [subject, subject])
}
"#,
                    )],
                ),
            ],
            [dynamic, process, consumer],
        );
        assert_eq!(result, geam_core::Value::Bool(true));
    }

    fn name<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, Name<BigInt>>,
    ) -> Result<HostCallCompletion<'call, Name<BigInt>>, HostCallError> {
        let value = call.create_external("worker".into());
        Ok(call.return_value(value))
    }

    fn round_trip<'call>(
        mut call: HostCall<
            'call,
            GleamErlangProfile,
            Component<GleamErlangProfile>,
            HostSubject<BigInt>,
        >,
        original: HostCustom<'call, HostSubject<BigInt>>,
        items: HostList<'call, HostSubject<BigInt>>,
    ) -> Result<HostCallCompletion<'call, HostSubject<BigInt>>, HostCallError> {
        let retained = Subject::<BigInt>::from_host(&mut call, original);
        let constructions = ProviderConstructions::none();
        let decoder = <Subject<BigInt> as ProviderListInputCodec<
            GleamErlangProfile,
            Component<GleamErlangProfile>,
        >>::decoder_with(&call, &constructions);
        let alias = decoder.clone();
        drop(decoder);
        let items = call.provider_retained_list::<Subject<BigInt>, _, _>(items, alias);
        assert_eq!(items.len(), 2);
        assert!(items.get(2).is_none());
        let first = items.get(0).unwrap();
        let second = items.get(1).unwrap();
        drop(items);
        for item in [first, second] {
            let restored = item.into_host_infallible(&mut call, &constructions);
            assert!(call.equal::<HostSubject<BigInt>>(original, restored));
            assert_eq!(
                call.source_hash::<HostSubject<BigInt>>(restored),
                call.source_hash::<HostSubject<BigInt>>(original)
            );
            assert_eq!(
                call.inspect::<HostSubject<BigInt>>(restored),
                "NamedSubject(name: Worker)"
            );
        }
        retained.complete(call, &constructions)
    }
}
