use super::{MissingValueContext, ProviderValueContext, Value};
use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostProfile, HostProvider, HostType,
};
use crate::provider::advanced::StoredDynamic;
use crate::provider::{
    ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderListInputValue,
    ProviderListItemDecoder, ProviderListItemValue, ProviderNoConstructions, ProviderOutputStorage,
    ProviderOutputValue, ProviderRootOutputValue, ProviderStaticValueForms, ProviderStoredOwner,
    ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};
use std::convert::Infallible;
use std::marker::PhantomData;

type Retained<Type> = Value<Type, ProviderValueContext<<Type as ProviderValue>::Host>>;

impl<Type: ProviderValue> ProviderValue for Value<Type, MissingValueContext> {
    type Host = Type::Host;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Type, Host: HostType> ProviderValue for Value<Type, ProviderValueContext<Host>> {
    type Host = Host;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Type: ProviderValue + 'static, Context> ProviderValueForms for Value<Type, Context>
where
    Self: ProviderValue<Host = Type::Host>,
{
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Retained<Type>;
    type ImmediateInput = Retained<Type>;
    type ImmediateListInput = Value<Type>;
    type OwnedInput = Retained<Type>;
    type OwnedListInput = Value<Type>;
    type ImmediateListDecoder = ProviderValueListDecoder<Type>;
    type OwnedListDecoder = ProviderValueListDecoder<Type>;
}

impl<Profile, Provider, Return, Type> ProviderInputValue<Profile, Provider, Return>
    for Retained<Type>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: ProviderValue,
{
    type Host = Type::Host;
    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self::from_host(call, value)
    }
}

impl<Profile, Provider, Return, Type, Host> ProviderOutputValue<Profile, Provider, Return>
    for Value<Type, ProviderValueContext<Host>>
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
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
        self.into_host(call)
    }
}

/// Storage transfers an existing handle without restoring its represented value.
#[doc(hidden)]
pub struct ProviderRetainedStorage;

impl<Profile, Provider, Return, Type, Host>
    ProviderOutputStorage<Value<Type, ProviderValueContext<Host>>, Profile, Provider, Return>
    for ProviderRetainedStorage
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Error = Infallible;

    fn store<'call, Owner: ProviderStoredOwner>(
        value: Value<Type, ProviderValueContext<Host>>,
        _: &mut HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, ProviderNoConstructions>,
    ) -> Result<StoredDynamic<Owner>, Self::Error> {
        Ok(StoredDynamic::from_runtime_value(value.into_stored()))
    }
}

impl<Profile, Provider, Type> ProviderRootOutputValue<Profile, Provider> for Retained<Type>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Type: ProviderValue,
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

/// Retains a demanded opaque item without granting callable invocation rights.
#[doc(hidden)]
pub struct ProviderValueListDecoder<Type> {
    type_: PhantomData<fn() -> Type>,
}

impl<Type: ProviderValue> ProviderValueListDecoder<Type> {
    #[doc(hidden)]
    pub fn from_host<'call, Profile, Provider, Return>(
        _call: &HostCall<'call, Profile, Provider, Return>,
    ) -> Self
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        Self { type_: PhantomData }
    }
}

impl<Type> Clone for ProviderValueListDecoder<Type> {
    fn clone(&self) -> Self {
        Self { type_: PhantomData }
    }
}

impl<Type: ProviderValue> ProviderListItemDecoder<Value<Type>> for ProviderValueListDecoder<Type> {
    type View = Retained<Type>;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Value::from_stored(value.into_stored())
    }
}

impl<Type: ProviderValue> ProviderTypedListItemDecoder<Value<Type>>
    for ProviderValueListDecoder<Type>
{
    type Host = Type::Host;
}

impl<Type: ProviderValue + 'static> ProviderListInputValue for Value<Type> {
    type Host = Type::Host;
    type View = Retained<Type>;
    type Decoder = ProviderValueListDecoder<Type>;
}

impl<Profile, Provider, Type> ProviderListInputCodec<Profile, Provider> for Value<Type>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Type: ProviderValue + 'static,
{
    type Requirements = ProviderNoConstructions;

    fn decoder_with<'call, Return: HostType>(
        _call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder {
        ProviderValueListDecoder { type_: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::Retained;
    use crate::host::{HostCall, HostCallCompletion, HostCallError, HostProvider};
    use crate::provider::advanced::ProviderDynamicValue;
    use crate::provider::{
        ProviderConstructions, ProviderInputValue, ProviderOutputValue, ProviderRootOutputValue,
        Value,
    };
    use crate::{
        HostList, HostListType, HostModule, HostProviderModule, HostProviderSet, HostTypeParameter,
        HostValue, HostedExecution, ModuleSource, PackageSource,
    };

    type Profile = crate::host::ExternalTestProfile;
    type Parameter = HostTypeParameter<0>;
    type Opaque = Retained<Parameter>;
    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = ();
        fn project(state: &mut crate::host::ExternalTestRunState) -> &mut () {
            &mut state.provider
        }
    }
    struct Parcel;
    impl crate::HostExternalSchema for Parcel {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Parcel";
        const PARAMETER_COUNT: usize = 0;
    }
    impl crate::HostExternalBinding<Profile, Parcel> for Provider {
        type Storage = Provider;
    }
    impl crate::HostExternalStorage<Profile, Parcel> for Provider {
        type Payload = num_bigint::BigInt;
        fn store(
            stores: &crate::host::ExternalTestStores,
        ) -> &crate::HostExternalStore<Self::Payload> {
            &stores.integers
        }
        fn source_equal(
            _: &crate::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }
        fn source_hash(_: &crate::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }
        fn inspect(
            _: &crate::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> ecow::EcoString {
            format!("Parcel({value})").into()
        }
    }
    type ParcelType = crate::HostExternalType<Parcel>;
    fn parcel<'call>(
        mut call: HostCall<'call, Profile, Provider, ParcelType>,
        value: num_bigint::BigInt,
    ) -> Result<HostCallCompletion<'call, ParcelType>, HostCallError> {
        let value = call.create_external_with_binding::<Provider>(value);
        Ok(call.return_value(value))
    }
    fn hash_parcel<'call>(
        call: HostCall<'call, Profile, Provider, num_bigint::BigInt>,
        value: crate::HostExternal<'call, ParcelType>,
    ) -> Result<HostCallCompletion<'call, num_bigint::BigInt>, HostCallError> {
        let hash = call.source_hash::<ParcelType>(value);
        Ok(call.return_value(hash.into()))
    }
    struct Owner;
    impl crate::provider::ProviderStoredOwner for Owner {}

    fn round_trip<'call>(
        mut call: HostCall<'call, Profile, Provider, Parameter>,
        value: HostValue<'call, Parameter>,
    ) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
        assert_eq!(call.state(), &mut ());
        let proof = ProviderConstructions::none();
        let value = <Opaque as ProviderInputValue<Profile, Provider, Parameter>>::from_host_with(
            &mut call, value, &proof,
        );
        let stored = <Opaque as ProviderDynamicValue<Profile, Provider, Parameter>>::into_stored::<
            Owner,
        >(value, &mut call)
        .unwrap_or_else(|never| match never {});
        let restored = call
            .restore_value::<Parameter>(stored.stored())
            .expect("stored value belongs to this call");
        let value = <Opaque as ProviderInputValue<Profile, Provider, Parameter>>::from_host_with(
            &mut call, restored, &proof,
        );
        let returned = <Opaque as ProviderOutputValue<Profile, Provider, Parameter>>::into_host(
            value, &mut call, &proof,
        )
        .expect("retained value belongs to this call");
        let value = Opaque::from_host(&call, returned);
        <Opaque as ProviderRootOutputValue<Profile, Provider>>::complete(value, call, &proof)
    }

    fn selected<'call>(
        call: HostCall<'call, Profile, Provider, Parameter>,
        values: HostList<'call, Parameter>,
    ) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
        let values = {
            let decoder = super::ProviderValueListDecoder::<Parameter>::from_host(&call);
            call.provider_retained_list::<Value<Parameter>, _, _>(values, decoder.clone())
        };
        assert_eq!(values.len(), 2);
        let value = values.get(1).unwrap();
        let context = values.__geam_into_context();
        assert_eq!(context.retained().item_reads(), 1);
        drop(context);
        <Opaque as ProviderRootOutputValue<Profile, Provider>>::complete(
            value,
            call,
            &ProviderConstructions::none(),
        )
    }

    #[test]
    fn opaque_codecs_keep_values_and_function_identity_after_dropping_list_parents() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Provider, Parcel>()
            .unwrap()
            .with_scoped_function::<Provider, (num_bigint::BigInt,), ParcelType, _>(
                "parcel", parcel,
            )
            .unwrap()
            .with_scoped_function::<Provider, (ParcelType,), num_bigint::BigInt, _>(
                "hash_parcel",
                hash_parcel,
            )
            .unwrap()
            .with_scoped_function::<Provider, (Parameter,), Parameter, _>("round_trip", round_trip)
            .unwrap()
            .with_scoped_function::<Provider, (HostListType<Parameter>,), Parameter, _>(
                "selected", selected,
            )
            .unwrap();
        let hosts =
            HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), [provider]).unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
pub type Message { Message(String, Int) }
pub type Parcel
@external(erlang, "native", "parcel") fn parcel(value: Int) -> Parcel
@external(erlang, "native", "hash_parcel") fn hash_parcel(value: Parcel) -> Int
@external(erlang, "native", "round_trip") fn round_trip(value: item) -> item
@external(erlang, "native", "selected") fn selected(values: List(item)) -> item
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<"한":utf8>>
  assert selected([1, 42]) == 42
  assert selected([1.0, 1.5]) == 1.5
  assert selected(["unused", "retained"]) == "retained"
  assert selected([<<1>>, <<2>>]) == <<2>>
  assert selected([point, point]) == point
  assert selected([False, True])
  assert selected([Nil, Nil]) == Nil
  assert selected([Message("unused", 0), Message("ready", 3)]) == Message("ready", 3)
  assert selected([[0], [1, 2]]) == [1, 2]
  let retained_parcel = selected([parcel(0), parcel(42)])
  assert retained_parcel == parcel(42)
  assert retained_parcel != parcel(0)
  assert hash_parcel(retained_parcel) == hash_parcel(parcel(42))
  echo retained_parcel
  let offset = 7
  let callback = fn(value) { echo value value + offset }
  let retained = selected([fn(value) { panic as "unselected" }, round_trip(callback)])
  #(round_trip(42), round_trip(Message("ready", 3)),
    selected([#(1, ["unused"]), #(2, ["retained"])]),
    retained == callback, retained(10))
}
"#,
                )],
            )],
            hosts,
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echoes = Vec::new();
        let value =
            crate::execution_fixture::run(&mut execution, &mut Default::default(), &mut echoes)
                .unwrap();
        assert_eq!(
            value.inspect().to_string(),
            r#"#(42, Message("ready", 3), #(2, ["retained"]), True, 17)"#
        );
        assert_eq!(
            echoes
                .iter()
                .map(|echo| echo.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["Parcel(42)", "10"]
        );
    }
}
