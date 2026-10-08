use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostProfile, HostProvider,
    HostRetainedCustomSchema, HostRetainedCustomType, HostType, HostTypeList, HostTypeListEnd,
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

/// An original `gleam/dynamic/decode.Decoder(Item)` retained by its producer SDK.
/// Providers may retain, clone, and return it without describing private fields.
/// Decoding remains a Gleam operation through `decode.run`. Restoration requires
/// the same loaded owner, exact source type, and original live execution.
pub struct Decoder<Item, Context = MissingValueContext> {
    value: Value<Self, Context>,
    marker: PhantomData<fn() -> Item>,
}

#[doc(hidden)]
pub struct DecoderSchema;
impl HostRetainedCustomSchema for DecoderSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dynamic/decode";
    const NAME: &'static str = "Decoder";
    const PARAMETER_COUNT: usize = 1;
}
type HostDecoder<T> = HostRetainedCustomType<DecoderSchema, HostTypeList<T, HostTypeListEnd>>;

type Owned<Item, Host> = Decoder<Item, ProviderValueContext<HostDecoder<Host>>>;

#[doc(hidden)]
pub struct DecoderRuntime<Profile, Item>(PhantomData<fn() -> (Profile, Item)>);

impl<Profile, Item> ProviderTypedValue<Profile> for Decoder<Item>
where
    Profile: HostProfile,
    Item: ProviderTypedValue<Profile>,
{
    type Host = HostDecoder<Item::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Item: 'static> ProviderValueForms for Decoder<Item> {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = DecoderRuntime<Profile, Item>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = MissingListContext;
    type OwnedListDecoder = MissingListContext;
}

impl<Item: 'static> ProviderMarkerListForms for Decoder<Item> {
    type Immediate = List<Self>;
    type Owned = List<Self>;
}

impl<Profile, Item> ProviderRuntimeValueForms<Profile> for DecoderRuntime<Profile, Item>
where
    Profile: HostProfile,
    Item: ProviderTypedValue<Profile> + 'static,
{
    type Host = HostDecoder<Item::Host>;
    type Output = Owned<Item, Item::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
    type ImmediateInput = Self::Output;
    type ImmediateListInput = Self::Output;
    type OwnedInput = Self::Output;
    type OwnedListInput = Self::Output;
    type InputRequirements = ProviderNoConstructions;
}

impl<Item, Host: HostType> Owned<Item, Host> {
    pub(super) fn into_host<'call, Profile, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Result<HostCustom<'call, HostDecoder<Host>>, HostCallError>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        self.value.into_host(call)
    }
}

impl<Item, Host: HostType> ProviderValue for Owned<Item, Host> {
    type Host = HostDecoder<Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Item: 'static, Host: HostType> ProviderValueForms for Owned<Item, Host> {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = DecoderListDecoder<Item, Host>;
    type OwnedListDecoder = DecoderListDecoder<Item, Host>;
}

#[doc(hidden)]
pub struct DecoderListDecoder<Item, Host: HostType>(ProviderValueListDecoder<Owned<Item, Host>>);

impl<Item, Host: HostType> Clone for DecoderListDecoder<Item, Host> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Item, Host: HostType> ProviderListItemDecoder<Owned<Item, Host>>
    for DecoderListDecoder<Item, Host>
{
    type View = Owned<Item, Host>;
    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Decoder {
            value: self.0.decode(value),
            marker: PhantomData,
        }
    }
}

impl<Item, Host: HostType> ProviderTypedListItemDecoder<Owned<Item, Host>>
    for DecoderListDecoder<Item, Host>
{
    type Host = HostDecoder<Host>;
}

impl<Item: 'static, Host: HostType> ProviderListInputValue for Owned<Item, Host> {
    type Host = HostDecoder<Host>;
    type View = Self;
    type Decoder = DecoderListDecoder<Item, Host>;
}

impl<Profile, Provider, Item: 'static, Host> ProviderListInputCodec<Profile, Provider>
    for Owned<Item, Host>
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
        DecoderListDecoder(ProviderValueListDecoder::from_host(call))
    }
}

impl<Profile, Provider, Return, Item, Host> ProviderInputValue<Profile, Provider, Return>
    for Owned<Item, Host>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Host = HostDecoder<Host>;
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

impl<Profile, Provider, Return, Item, Host> ProviderOutputValue<Profile, Provider, Return>
    for Owned<Item, Host>
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
impl<Profile, Provider, Return, Item, Host>
    ProviderOutputStorage<Owned<Item, Host>, Profile, Provider, Return> for ProviderRetainedStorage
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Host: HostType,
{
    type Error = Infallible;

    fn store<'call, Owner: ProviderStoredOwner>(
        value: Owned<Item, Host>,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, ProviderNoConstructions>,
    ) -> Result<StoredDynamic<Owner>, Self::Error> {
        ProviderOutputValue::store_dynamic(value.value, call, constructions)
    }
}

impl<Profile, Provider, Item, Host> ProviderRootOutputValue<Profile, Provider> for Owned<Item, Host>
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

impl<Item, Host: HostType> Clone for Owned<Item, Host> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DecoderSchema, HostDecoder, Owned};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostCustom, HostList, HostListType,
        HostProfile, HostProvider, HostProviderModule,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue, ProviderStoredOwner,
    };
    use geam_core::{
        ExecutionError, HostConstructions, HostError, HostProviderSet, HostRestoredType,
        HostTypeIndex0, HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource,
        PackageSource, Value,
    };
    use num_bigint::BigInt;

    use geam_core::provider::advanced::{NativeKind, NativeValue};

    struct Profile;
    struct Provider;
    impl ProviderStoredOwner for Provider {}
    #[derive(Default)]
    struct State {
        cached: Option<Owned<BigInt, BigInt>>,
        cached_native: Option<NativeValue>,
        restore_native: bool,
        native_restorations: Vec<bool>,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    fn execution() -> HostedExecution<Profile> {
        let producer = HostProviderModule::new("gleam_stdlib", "gleam/dynamic/decode")
            .unwrap()
            .with_retained_custom_type::<DecoderSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("app", "main").unwrap()
            .with_scoped_function_and_constructions::<Provider, (HostDecoder<BigInt>, HostListType<HostDecoder<BigInt>>), HostDecoder<BigInt>, HostTypeList<HostRestoredType<HostDecoder<BigInt>>, HostTypeListEnd>, _>("keep", keep).unwrap();
        let typed = geam_core::compile_typed_host_program(
            "app",
            "main",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "gleam/dynamic/decode",
                        "decode.gleam",
                        r#"
pub opaque type Decoder(t) { Decoder(function: fn(Int) -> t) }
pub fn shifted(offset: Int) { Decoder(fn(value) { value + offset }) }
pub fn run(value: Int, decoder: Decoder(t)) { decoder.function(value) }
"#,
                    )],
                ),
                PackageSource::new(
                    "app",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/dynamic/decode
@external(erlang, "native", "keep")
fn keep(value: decode.Decoder(Int), values: List(decode.Decoder(Int))) -> decode.Decoder(Int)
pub fn main() {
  let decoder = decode.shifted(40)
  let decoder = keep(decoder, [decoder, decoder])
  decode.run(2, keep(decoder, [decoder, decoder]))
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers([producer, consumer]).unwrap(),
        )
        .unwrap();
        HostedExecution::try_from_module_plan(geam_core::plan_host_program(typed).unwrap()).unwrap()
    }

    fn keep<'call>(
        mut call: HostCall<'call, Profile, Provider, HostDecoder<BigInt>>,
        permissions: HostConstructions<
            'call,
            HostTypeList<HostRestoredType<HostDecoder<BigInt>>, HostTypeListEnd>,
        >,
        original: HostCustom<'call, HostDecoder<BigInt>>,
        values: HostList<'call, HostDecoder<BigInt>>,
    ) -> Result<HostCallCompletion<'call, HostDecoder<BigInt>>, HostCallError> {
        let retained = Owned::<BigInt, BigInt>::from_host(&mut call, original);
        let constructions = ProviderConstructions::none();
        let alias = {
            let list_decoder = <Owned<BigInt, BigInt> as ProviderListInputCodec<
                Profile,
                Provider,
            >>::decoder_with(&call, &constructions);
            list_decoder.clone()
        };
        let values = call.provider_retained_list::<Owned<BigInt, BigInt>, _, _>(values, alias);
        assert_eq!(values.len(), 2);
        assert!(values.get(2).is_none());
        let first = values.get(0).unwrap();
        let second = values.get(1).unwrap();
        drop(values);
        let returned = <Owned<BigInt, BigInt> as ProviderOutputValue<
            Profile,
            Provider,
            HostDecoder<BigInt>,
        >>::into_host(first, &mut call, &constructions)
        .expect("the list item belongs to the current execution");
        assert!(call.equal::<HostDecoder<BigInt>>(original, returned));
        assert_eq!(
            call.source_hash::<HostDecoder<BigInt>>(original),
            call.source_hash::<HostDecoder<BigInt>>(returned)
        );
        let native = call.native_value::<HostDecoder<BigInt>>(original);
        assert_eq!(native.kind(), NativeKind::Opaque);
        assert!(native.index(0).is_none());
        assert!(native.len().is_none());
        let second = second
            .into_host(&mut call)
            .expect("the list alias belongs to the current execution");
        assert!(call.equal::<HostDecoder<BigInt>>(original, second));
        if call.state().restore_native {
            let cached = call.state().cached_native.take();
            call.state().cached_native = Some(native);

            let decoder = match cached {
                Some(cached) => {
                    let restored = call
                        .restore_native(&permissions.at::<HostTypeIndex0>().restoration(), &cached);
                    call.state().native_restorations.push(restored.is_some());
                    let restored = restored.ok_or_else(|| {
                        geam_core::HostFailure::new("retained decoder cannot be restored")
                    })?;
                    Owned::<BigInt, BigInt>::from_host(&mut call, restored)
                }
                None => retained,
            };
            return decoder.complete(call, &constructions);
        }
        let cached = call.state().cached.take();
        call.state().cached = Some(retained.clone());
        let selected = cached.unwrap_or(retained);
        let stored = selected
            .clone()
            .store_dynamic::<Provider>(&mut call, &constructions)
            .unwrap();
        assert_eq!(stored.native_view().kind(), NativeKind::Opaque);
        selected.complete(call, &constructions)
    }

    #[test]
    fn decoder_and_lazy_list_aliases_preserve_captures_without_disclosure() {
        let mut loaded = execution();
        let mut state = State::default();
        assert_eq!(
            crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap(),
            Value::Int(42.into())
        );
        let error =
            crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "keep");
        assert_eq!(
            error.failure().message(),
            "retained value belongs to another execution"
        );
        drop(loaded);
        let error = crate::execution_fixture::run(&mut execution(), &mut state, &mut Vec::new())
            .unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "keep");
        assert_eq!(
            error.failure().message(),
            "retained value belongs to another owner or source type"
        );
    }

    #[test]
    fn native_restoration_does_not_rebind_a_decoder_to_a_fresh_execution() {
        let mut loaded = execution();
        let mut state = State {
            restore_native: true,
            ..State::default()
        };
        assert_eq!(
            crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap(),
            Value::Int(42.into())
        );
        assert_eq!(state.native_restorations, [true]);
        let outcome = crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new());
        assert_eq!(state.native_restorations, [true, false], "{outcome:?}");
        let error = outcome.unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "keep");
        assert_eq!(
            error.failure().message(),
            "retained decoder cannot be restored"
        );
    }

    #[test]
    #[should_panic(expected = "expected a checked host failure")]
    fn host_failure_assertion_rejects_a_source_panic() {
        let typed = geam_core::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() -> Int { panic as \"source failure\" }",
                )],
            )],
            HostProviderSet::<Profile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let mut loaded =
            HostedExecution::try_from_module_plan(geam_core::plan_host_program(typed).unwrap())
                .unwrap();
        let error =
            crate::execution_fixture::run(&mut loaded, &mut State::default(), &mut Vec::new())
                .unwrap_err();
        let _ = expect_host_failure(error);
    }

    fn expect_host_failure(error: ExecutionError) -> Box<HostError> {
        let ExecutionError::Host(error) = error else {
            panic!("expected a checked host failure");
        };
        error
    }
}
