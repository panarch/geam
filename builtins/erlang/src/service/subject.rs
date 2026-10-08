use super::types::Subject as HostSubject;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostProfile, HostProvider, HostType,
};
use geam_core::provider::advanced::StoredDynamic;
use geam_core::provider::{
    List, MissingListContext, MissingValueContext, ProviderConstructions, ProviderInputValue,
    ProviderListInputCodec, ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
    ProviderMarkerListForms, ProviderNoConstructions, ProviderOutputStorage, ProviderOutputValue,
    ProviderRetainedStorage, ProviderRootOutputValue, ProviderRuntimeValueForms,
    ProviderStaticValueForms, ProviderStoredOwner, ProviderTypedListItemDecoder,
    ProviderTypedValue, ProviderValue, ProviderValueContext, ProviderValueForms,
    ProviderValueListDecoder, Value,
};
use std::convert::Infallible;
use std::marker::PhantomData;

/// An original subject with its exact source message specialization.
/// Passing it through a provider preserves its identity and tag. The macro
/// selects its owned form after the message type and profile are known.
pub struct Subject<Message, Context = MissingValueContext> {
    value: Value<Self, Context>,
    marker: PhantomData<fn() -> Message>,
}

pub(super) type Owned<Message, Host> = Subject<Message, ProviderValueContext<HostSubject<Host>>>;

#[doc(hidden)]
pub struct SubjectRuntime<Profile, Message>(PhantomData<fn() -> (Profile, Message)>);

impl<Profile, Message> ProviderTypedValue<Profile> for Subject<Message>
where
    Profile: HostProfile,
    Message: ProviderTypedValue<Profile>,
{
    type Host = HostSubject<Message::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Message: 'static> ProviderValueForms for Subject<Message> {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = SubjectRuntime<Profile, Message>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = MissingListContext;
    type OwnedListDecoder = MissingListContext;
}

impl<Message: 'static> ProviderMarkerListForms for Subject<Message> {
    type Immediate = List<Self>;
    type Owned = List<Self>;
}

impl<Profile, Message> ProviderRuntimeValueForms<Profile> for SubjectRuntime<Profile, Message>
where
    Profile: HostProfile,
    Message: ProviderTypedValue<Profile> + 'static,
{
    type Host = HostSubject<Message::Host>;
    type Output = Owned<Message, Message::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
    type ImmediateInput = Self::Output;
    type ImmediateListInput = Self::Output;
    type OwnedInput = Self::Output;
    type OwnedListInput = Self::Output;
    type InputRequirements = ProviderNoConstructions;
}

impl<Message, Host: HostType> Owned<Message, Host> {
    pub(super) fn into_host<'call, Profile, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Result<HostCustom<'call, HostSubject<Host>>, HostCallError>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        self.value.into_host(call)
    }
}

impl<Message, Host: HostType> ProviderValue for Owned<Message, Host> {
    type Host = HostSubject<Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Message: 'static, Host: HostType> ProviderValueForms for Owned<Message, Host> {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = SubjectListDecoder<Message, Host>;
    type OwnedListDecoder = SubjectListDecoder<Message, Host>;
}

#[doc(hidden)]
pub struct SubjectListDecoder<Message, Host: HostType>(
    ProviderValueListDecoder<Owned<Message, Host>>,
);

impl<Message, Host: HostType> Clone for SubjectListDecoder<Message, Host> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Message, Host: HostType> ProviderListItemDecoder<Owned<Message, Host>>
    for SubjectListDecoder<Message, Host>
{
    type View = Owned<Message, Host>;
    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Subject {
            value: self.0.decode(value),
            marker: PhantomData,
        }
    }
}

impl<Message, Host: HostType> ProviderTypedListItemDecoder<Owned<Message, Host>>
    for SubjectListDecoder<Message, Host>
{
    type Host = HostSubject<Host>;
}

impl<Message: 'static, Host: HostType> ProviderListInputValue for Owned<Message, Host> {
    type Host = HostSubject<Host>;
    type View = Self;
    type Decoder = SubjectListDecoder<Message, Host>;
}

impl<Profile, Provider, Message: 'static, Host> ProviderListInputCodec<Profile, Provider>
    for Owned<Message, Host>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Host: HostType,
{
    type Requirements = ProviderNoConstructions;
    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder {
        SubjectListDecoder(ProviderValueListDecoder::from_host(call))
    }
}

impl<Profile, Provider, Return, Message, Host> ProviderInputValue<Profile, Provider, Return>
    for Owned<Message, Host>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Host = HostSubject<Host>;
    type Requirements = ProviderNoConstructions;
    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostCustom<'call, Self::Host>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self {
            value: Value::from_host(call, value),
            marker: PhantomData,
        }
    }
}

impl<Profile, Provider, Return, Message, Host> ProviderOutputValue<Profile, Provider, Return>
    for Owned<Message, Host>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Error = HostCallError;
    type Storage = ProviderRetainedStorage;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostCustom<'call, Self::Host>, Self::Error> {
        self.into_host(call)
    }
}

// The SDK wrapper already owns a retained handle; storing it does not restore it.
impl<Profile, Provider, Return, Message, Host>
    ProviderOutputStorage<Owned<Message, Host>, Profile, Provider, Return>
    for ProviderRetainedStorage
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Error = Infallible;

    fn store<'call, Owner: ProviderStoredOwner>(
        value: Owned<Message, Host>,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, ProviderNoConstructions>,
    ) -> Result<StoredDynamic<Owner>, Self::Error> {
        ProviderOutputValue::store_dynamic(value.value, call, constructions)
    }
}

impl<Profile, Provider, Message, Host> ProviderRootOutputValue<Profile, Provider>
    for Owned<Message, Host>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Host: HostType,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, Self::Host>,
        _: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
        let value = self.into_host(&mut call)?;
        Ok(call.return_value(value))
    }
}

impl<Message, Host: HostType> Clone for Owned<Message, Host> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Owned;
    use crate::service::types::{Subject as HostSubject, SubjectSchema};
    use crate::{Component, GleamErlangProfile, Name, NameSchema, PidSchema};
    use geam_core::host::HostProviderSet;
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostCustom, HostList, HostListType,
        HostProviderModule,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue, ProviderStoredOwner,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;
    use std::sync::{Arc, Mutex};

    type Call<'call> =
        HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, HostSubject<BigInt>>;
    type Source<'call> = HostCustom<'call, HostSubject<BigInt>>;
    type Items<'call> = HostList<'call, HostSubject<BigInt>>;
    type Completion<'call> = Result<HostCallCompletion<'call, HostSubject<BigInt>>, HostCallError>;
    struct StorageOwner;
    impl ProviderStoredOwner for StorageOwner {}

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
        let previous = Arc::new(Mutex::new(None::<Owned<BigInt, BigInt>>));
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function::<Component<GleamErlangProfile>, (
                HostSubject<BigInt>,
                HostListType<HostSubject<BigInt>>,
            ), HostSubject<BigInt>, _>("round_trip", retaining_round_trip(previous))
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
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
            HostProviderSet::from_providers([dynamic, process, consumer]).unwrap(),
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
        previous: Arc<Mutex<Option<Owned<BigInt, BigInt>>>>,
    ) -> impl for<'call> Fn(Call<'call>, Source<'call>, Items<'call>) -> Completion<'call> {
        move |mut call, original, items| {
            let retained = Owned::<BigInt, BigInt>::from_host(&mut call, original);
            let previous = previous.lock().unwrap().replace(retained);
            match previous {
                None => round_trip(call, original, items),
                Some(previous) => previous.complete(call, &ProviderConstructions::none()),
            }
        }
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
        original: Source<'call>,
        items: Items<'call>,
    ) -> Completion<'call> {
        let retained = Owned::<BigInt, BigInt>::from_host(&mut call, original);
        let returned = retained.clone();
        drop(retained);
        let constructions = ProviderConstructions::none();
        let stored = returned
            .clone()
            .store_dynamic::<StorageOwner>(&mut call, &constructions)
            .unwrap();
        assert!(call.native_equal(
            &stored.native_view(),
            &call.native_value::<HostSubject<BigInt>>(original),
        ));
        let alias = {
            let decoder = <Owned<BigInt, BigInt> as ProviderListInputCodec<
                GleamErlangProfile,
                Component<GleamErlangProfile>,
            >>::decoder_with(&call, &constructions);
            decoder.clone()
        };
        let items = call.provider_retained_list::<Owned<BigInt, BigInt>, _, _>(items, alias);
        assert_eq!(items.len(), 2);
        assert!(items.get(2).is_none());
        let first = items.get(0).unwrap();
        let second = items.get(1).unwrap();
        drop(items);
        for item in [first, second] {
            let restored = ProviderOutputValue::into_host(item, &mut call, &constructions)
                .expect("current execution retains the exact subject specialization");
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
        returned.complete(call, &constructions)
    }
}
