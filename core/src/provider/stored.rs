use crate::host::{
    HostCall, HostConstruction, HostExternalPayloadBuilder, HostExternalSchema, HostExternalType,
    HostProfile, HostProvider, HostType, HostTypeAt, HostTypeSequence,
};
use crate::provider::advanced::{Retained, StoredDynamic};
use crate::provider::{ProviderExternalView, ProviderOwnedExternal};
use std::marker::PhantomData;

/// Generated identity for one external declaration that may own retained values.
#[doc(hidden)]
pub trait ProviderStoredOwner: 'static {}

/// Retains one low-level generic argument for a macro-authored payload.
///
/// This bridge exists for built-in providers that still consume the typed-host
/// SDK while a sibling component has moved to the authoring macros.
#[doc(hidden)]
pub fn retain_argument<Profile, Arguments, Owner, Index>(
    builder: &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
    value: <<Arguments as HostTypeAt<Index>>::Type as HostType>::Value<'_>,
) -> Retained<Owner, Index>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence + HostTypeAt<Index>,
    Owner: ProviderStoredOwner,
{
    Retained::from_host_value(builder.store_argument::<Index>(value))
}

/// Retains one low-level value with its exact specialized type.
#[doc(hidden)]
pub fn retain_dynamic<Profile, Arguments, Owner, Type>(
    builder: &mut HostExternalPayloadBuilder<'_, Profile, Arguments>,
    value: Type::Value<'_>,
) -> StoredDynamic<Owner>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence,
    Owner: ProviderStoredOwner,
    Type: HostType,
{
    StoredDynamic::from_host_value(builder.store_dynamic::<Type>(value))
}

/// Retains a transferable payload argument at its construction-sealed type position.
#[doc(hidden)]
pub fn retain_constructed_argument<
    'call,
    Profile,
    Provider,
    Return,
    Schema,
    Arguments,
    Owner,
    Index,
>(
    call: &HostCall<'call, Profile, Provider, Return>,
    _construction: &HostConstruction<'call, HostExternalType<Schema, Arguments>>,
    value: <<Arguments as HostTypeAt<Index>>::Type as HostType>::Value<'call>,
) -> Retained<Owner, Index>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence + HostTypeAt<Index>,
    Owner: ProviderStoredOwner,
{
    Retained::from_runtime_value(call.retain_value::<<Arguments as HostTypeAt<Index>>::Type>(value))
}

/// Retains an existential transferable value under an external construction owner.
#[doc(hidden)]
pub fn retain_constructed_dynamic<
    'call,
    Profile,
    Provider,
    Return,
    Schema,
    Arguments,
    Owner,
    Type,
>(
    call: &HostCall<'call, Profile, Provider, Return>,
    _construction: &HostConstruction<'call, HostExternalType<Schema, Arguments>>,
    value: Type::Value<'call>,
) -> crate::provider::advanced::StoredDynamic<Owner>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence,
    Owner: ProviderStoredOwner,
    Type: HostType,
{
    StoredDynamic::from_runtime_value(call.retain_value::<Type>(value))
}

/// One generic Gleam value retained by a macro-authored external payload.
///
/// Providers create stored values through [`super::Call::store`] and restore
/// fields exposed by a generated external input through [`super::Call::restore`].
/// A stored value is neither cloneable nor copyable and cannot be constructed
/// independently of its generated external owner.
pub struct Stored<Type, Context = MissingStoredContext> {
    context: Context,
    type_: PhantomData<fn() -> Type>,
}

#[doc(hidden)]
pub struct MissingStoredContext;

/// A transferable retained value being assembled into one generated payload.
#[doc(hidden)]
pub struct ProviderStoredOutput<Owner, Index, Host> {
    value: Retained<Owner, Index>,
    host: PhantomData<fn() -> Host>,
}

/// A borrowed transferable retained field selected from one generated input.
#[doc(hidden)]
pub struct ProviderStoredInput<'value, Owner, Index, Host> {
    value: &'value Retained<Owner, Index>,
    host: PhantomData<fn() -> Host>,
}

/// An owned retained field selected before an async provider body runs.
#[doc(hidden)]
pub struct ProviderOwnedStoredInput<Owner, Index, Host> {
    value: Retained<Owner, Index>,
    host: PhantomData<fn() -> Host>,
}

/// A short transferable payload view used by an immediate provider function.
#[doc(hidden)]
pub struct ProviderExternalInputContext<Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    value: ProviderExternalView<Payload>,
    arguments: PhantomData<fn() -> Arguments>,
}

/// An owned payload handle used by an async provider function.
#[doc(hidden)]
pub struct ProviderOwnedExternalInputContext<Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    value: ProviderOwnedExternal<Payload>,
    arguments: PhantomData<fn() -> Arguments>,
}

#[doc(hidden)]
pub struct MissingExternalInputContext;

#[doc(hidden)]
pub struct ProviderExternalOutput<Payload> {
    value: Result<Payload, ProviderExternalReturn<Payload>>,
}

/// A typed external identity returned without retaining a payload borrow.
#[doc(hidden)]
pub struct ProviderExternalReturn<Payload> {
    lease: crate::runtime::ExternalPayloadLease,
    payload: PhantomData<fn() -> Payload>,
}

#[doc(hidden)]
pub struct MissingExternalOutputContext;

impl<Type, Owner, Index, Host> Stored<Type, ProviderStoredOutput<Owner, Index, Host>>
where
    Owner: ProviderStoredOwner,
{
    #[doc(hidden)]
    pub fn from_output(value: Retained<Owner, Index>) -> Self {
        Self {
            context: ProviderStoredOutput {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    /// Moves this newly stored value into a transferable persistent payload.
    pub fn into_retained(self) -> Retained<Owner, Index> {
        self.context.value
    }
}

impl<'value, Type, Owner, Index, Host> Stored<Type, ProviderStoredInput<'value, Owner, Index, Host>>
where
    Owner: ProviderStoredOwner,
{
    #[doc(hidden)]
    pub fn from_retained(value: &'value Retained<Owner, Index>) -> Self {
        Self {
            context: ProviderStoredInput {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    pub(crate) fn retained(&self) -> &'value Retained<Owner, Index> {
        self.context.value
    }
}

impl<Type, Owner, Index, Host> Stored<Type, ProviderOwnedStoredInput<Owner, Index, Host>>
where
    Owner: ProviderStoredOwner,
{
    #[doc(hidden)]
    pub fn from_async_retained(value: Retained<Owner, Index>) -> Self {
        Self {
            context: ProviderOwnedStoredInput {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    pub(crate) fn into_async_stored(self) -> Retained<Owner, Index> {
        self.context.value
    }
}

impl<Payload> ProviderExternalOutput<Payload> {
    #[doc(hidden)]
    pub fn new(payload: Payload) -> Self {
        Self { value: Ok(payload) }
    }

    #[doc(hidden)]
    pub fn from_input(value: ProviderExternalReturn<Payload>) -> Self {
        Self { value: Err(value) }
    }

    #[doc(hidden)]
    pub fn into_value(self) -> Result<Payload, ProviderExternalReturn<Payload>> {
        self.value
    }
}

impl<Payload> ProviderExternalReturn<Payload> {
    pub(crate) fn into_lease(self) -> crate::runtime::ExternalPayloadLease {
        self.lease
    }
}

impl<Payload, Arguments> ProviderExternalInputContext<Payload, Arguments>
where
    Payload: Send + 'static,
    Arguments: HostTypeSequence,
{
    #[doc(hidden)]
    pub fn from_host(value: ProviderExternalView<Payload>) -> Self {
        Self {
            value,
            arguments: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn payload(&self) -> &Payload {
        &self.value
    }

    #[doc(hidden)]
    pub fn into_output(self) -> ProviderExternalOutput<Payload> {
        ProviderExternalOutput::from_input(ProviderExternalReturn {
            lease: self.value.into_lease(),
            payload: PhantomData,
        })
    }
}

impl<Payload, Arguments> ProviderOwnedExternalInputContext<Payload, Arguments>
where
    Payload: Send + 'static,
    Arguments: HostTypeSequence,
{
    #[doc(hidden)]
    pub fn from_host(value: ProviderOwnedExternal<Payload>) -> Self {
        Self {
            value,
            arguments: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn with_payload<Output>(&self, read: impl FnOnce(&Payload) -> Output) -> Output {
        self.value.with(read)
    }

    #[doc(hidden)]
    pub fn into_output(self) -> ProviderExternalOutput<Payload> {
        ProviderExternalOutput::from_input(ProviderExternalReturn {
            lease: self.value.into_lease(),
            payload: PhantomData,
        })
    }

    #[doc(hidden)]
    pub fn stored<Type, Owner, Index, Host>(
        &self,
        select: impl for<'payload> FnOnce(&'payload Payload) -> &'payload Retained<Owner, Index>,
    ) -> Stored<Type, ProviderOwnedStoredInput<Owner, Index, Host>>
    where
        Owner: ProviderStoredOwner,
    {
        self.value
            .with(|payload| Stored::from_async_retained(select(payload).clone_retained()))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProviderExternalInputContext, ProviderExternalOutput, ProviderStoredOwner, retain_argument,
        retain_dynamic,
    };
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        CallArguments, HostExternalPayloadBuilder, HostTypeIndex0, HostTypeList, HostTypeListEnd,
    };
    use crate::plan::ValueType;
    use crate::provider::advanced::{DynamicKind, Retained, StoredDynamic};
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    struct Payload;

    impl ProviderStoredOwner for Payload {}

    #[test]
    fn retention_bridges_preserve_the_declared_owner_and_runtime_type() {
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let mut builder =
            HostExternalPayloadBuilder::<TestHostProfile, Arguments>::new(&mut runtime);

        let retained: Retained<Payload, HostTypeIndex0> =
            retain_argument(&mut builder, BigInt::from(7));
        let dynamic: StoredDynamic<Payload> =
            retain_dynamic::<_, Arguments, _, BigInt>(&mut builder, BigInt::from(8));

        assert_eq!(retained.stored().type_(), &ValueType::Int);
        assert_eq!(dynamic.kind(), DynamicKind::Int);
    }

    #[test]
    fn transfer_retention_bridges_keep_exact_values_in_a_source_visible_payload() {
        use crate::host::{
            HostCall, HostCallCompletion, HostConstructions, HostExternal, HostExternalBinding,
            HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
            HostExternalStorage, HostExternalStore, HostExternalType, HostProfile, HostProvider,
            HostProviderModule, HostProviderSet,
        };
        use crate::plan::execution::HostedProgram;
        use crate::plan::{LibraryEntry, LibraryValueType};
        use crate::provider::advanced::{Retained, StoredDynamic};
        use crate::runtime::RetainedInputs;
        use crate::runtime::work::driver::Driver;
        struct Profile;
        struct Provider;
        struct Schema;
        struct Storage;
        struct BridgePayload {
            argument: Retained<Payload, HostTypeIndex0>,
            dynamic: StoredDynamic<Payload>,
        }
        impl HostProfile for Profile {
            type RunState = ();
            type ExternalStores = HostExternalStore<BridgePayload>;
        }
        impl HostProvider<Profile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        impl HostExternalSchema for Schema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Box";
            const PARAMETER_COUNT: usize = 1;
        }
        impl HostExternalBinding<Profile, Schema> for Provider {
            type Storage = Storage;
        }
        impl HostExternalStorage<Profile, Schema> for Storage {
            type Payload = BridgePayload;
            fn store(
                stores: &HostExternalStore<BridgePayload>,
            ) -> &HostExternalStore<BridgePayload> {
                stores
            }
            fn source_equal(
                context: &HostExternalEquality<'_>,
                left: &BridgePayload,
                right: &BridgePayload,
            ) -> bool {
                left.argument.source_equal(context, &right.argument)
                    && left.dynamic.source_equal(context, &right.dynamic)
            }
            fn source_hash(context: &HostExternalHashing<'_>, value: &BridgePayload) -> u64 {
                value.argument.source_hash(context) ^ value.dynamic.source_hash(context)
            }
            fn inspect(
                context: &HostExternalInspection<'_>,
                value: &BridgePayload,
            ) -> ecow::EcoString {
                format!(
                    "Box({}, {})",
                    value.argument.inspect(context),
                    value.dynamic.inspect(context)
                )
                .into()
            }
        }
        type BoxType = HostExternalType<Schema, HostTypeList<BigInt, HostTypeListEnd>>;
        type Constructions = HostTypeList<BoxType, HostTypeListEnd>;
        fn pack<'call>(
            mut call: HostCall<'call, Profile, Provider, BoxType>,
            constructions: HostConstructions<'call, Constructions>,
            value: BigInt,
            other: BigInt,
        ) -> Result<HostCallCompletion<'call, BoxType>, crate::HostCallError> {
            let () = *call.state();
            let construction = constructions.at::<HostTypeIndex0>();
            let argument =
                super::retain_constructed_argument::<_, _, _, _, _, Payload, HostTypeIndex0>(
                    &call,
                    &construction,
                    value,
                );
            let dynamic = super::retain_constructed_dynamic::<_, _, _, _, _, Payload, BigInt>(
                &call,
                &construction,
                other,
            );
            assert_eq!(argument.stored().type_(), &ValueType::Int);
            assert_eq!(dynamic.kind(), DynamicKind::Int);
            let value = call.construct_external_with_binding::<Provider, Schema, HostTypeList<BigInt, HostTypeListEnd>>(
                construction, BridgePayload { argument, dynamic });
            Ok(call.return_value(value))
        }
        fn hash<'call>(
            call: HostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, BoxType>,
        ) -> Result<HostCallCompletion<'call, BigInt>, crate::HostCallError> {
            let value = call.source_hash::<BoxType>(value);
            Ok(call.return_value(value.into()))
        }
        let provider = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_external_type::<Provider, Schema>().expect("schema")
            .with_scoped_function_and_constructions::<Provider, (BigInt, BigInt), BoxType, Constructions, _>("pack", pack).expect("pack")
            .with_scoped_function::<Provider, (BoxType,), BigInt, _>("hash", hash).expect("hash");
        let source = r#"pub type Box(value)
@external(erlang, "native", "pack")
fn pack(value: Int, other: Int) -> Box(Int)
@external(erlang, "native", "hash")
fn hash(value: Box(Int)) -> Int
pub fn run() {
  let first = pack(7, 8)
  let equal = pack(7, 8)
  echo first
  first == equal && first != pack(6, 8) && first != pack(7, 9) && hash(first) == hash(equal)
}
"#;
        let program = crate::frontend::compile_typed_host_program(
            "application",
            "library",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([provider]).expect("providers"),
        )
        .expect("source");
        let plan = crate::planner::plan_host_library_program(program).expect("plan");
        let function = plan
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("entry")
            .gleam_body()
            .expect("source body");
        let entry = LibraryEntry::new(
            function.id(),
            LibraryValueType::Bool,
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries) =
            HostedProgram::from_library_plan(plan, entry, Vec::new()).expect("sealed execution");
        let mut state = ();
        let mut stores = HostExternalStore::default();
        let mut output = Vec::new();
        let mut echo = |value: crate::EchoOutput| output.push(value.to_string());
        let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
        assert!(
            driver
                .run_bool(*entries.bools[0].function(), RetainedInputs::empty())
                .expect("retained payload semantics")
        );
        drop(driver);
        assert_eq!(output, ["src/library.gleam:9\nBox(7, 8)"]);
    }

    struct OwnedPayload {
        value: Cell<usize>,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for OwnedPayload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn fresh_external_outputs_own_non_clone_payloads_in_each_representation() {
        let drops = Arc::new(AtomicUsize::new(0));
        let local = ProviderExternalOutput::<OwnedPayload>::new(OwnedPayload {
            value: Cell::new(41),
            drops: Arc::clone(&drops),
        })
        .into_value();
        assert!(local.is_ok());
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(local);
        assert_eq!(drops.load(Ordering::Relaxed), 1);

        let transferable = ProviderExternalOutput::new(OwnedPayload {
            value: Cell::new(42),
            drops: Arc::clone(&drops),
        });
        std::thread::spawn(move || {
            let value = transferable.into_value();
            assert!(value.is_ok());
            drop(value);
        })
        .join()
        .expect("owned output should transfer");
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn returning_a_transfer_view_releases_the_borrow_but_keeps_exact_payload_ownership() {
        use crate::frontend::compile_typed_host_program;
        use crate::host::{
            HostCall, HostExternal, HostExternalBinding, HostExternalEquality, HostExternalHashing,
            HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
            HostExternalType, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        };
        use crate::{HostCallCompletion, ModuleSource, PackageSource};
        use num_bigint::BigInt;

        struct Profile;
        struct Provider;
        struct Schema;
        struct Storage;
        impl HostProfile for Profile {
            type RunState = Arc<AtomicUsize>;
            type ExternalStores = HostExternalStore<OwnedPayload>;
        }
        impl HostProvider<Profile> for Provider {
            type State = Arc<AtomicUsize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        impl HostExternalSchema for Schema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Owned";
            const PARAMETER_COUNT: usize = 0;
        }
        impl HostExternalBinding<Profile, Schema> for Provider {
            type Storage = Storage;
        }
        impl HostExternalStorage<Profile, Schema> for Storage {
            type Payload = OwnedPayload;
            fn store(stores: &HostExternalStore<OwnedPayload>) -> &HostExternalStore<OwnedPayload> {
                stores
            }
            fn source_equal(
                _: &HostExternalEquality<'_>,
                a: &OwnedPayload,
                b: &OwnedPayload,
            ) -> bool {
                a.value.get() == b.value.get()
            }
            fn source_hash(_: &HostExternalHashing<'_>, value: &OwnedPayload) -> u64 {
                value.value.get() as u64
            }
            fn inspect(_: &HostExternalInspection<'_>, value: &OwnedPayload) -> ecow::EcoString {
                format!("Owned({})", value.value.get()).into()
            }
        }
        fn make<'call>(
            mut call: HostCall<'call, Profile, Provider, HostExternalType<Schema>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Schema>>, crate::HostCallError>
        {
            let drops = Arc::clone(call.state());
            let value = call.create_external_with_binding::<Provider>(OwnedPayload {
                value: Cell::new(42),
                drops,
            });
            Ok(call.return_value(value))
        }
        fn keep<'call>(
            mut call: HostCall<'call, Profile, Provider, HostExternalType<Schema>>,
            value: HostExternal<'call, HostExternalType<Schema>>,
        ) -> Result<HostCallCompletion<'call, HostExternalType<Schema>>, crate::HostCallError>
        {
            let original = call.external_payload(value);
            let address = std::ptr::from_ref(&*original).addr();
            drop(original);
            let view = call.provider_external_view_with::<Provider, Schema, HostTypeListEnd>(value);
            let context = ProviderExternalInputContext::<_, HostTypeListEnd>::from_host(view);
            assert_eq!(context.payload().value.get(), 42);
            let output = context
                .into_output()
                .into_value()
                .err()
                .expect("pass-through retains the original payload");
            let returned = call.provider_external_from_return::<Schema, HostTypeListEnd, _>(output);
            let payload = call.external_payload(returned);
            assert_eq!(std::ptr::from_ref(&*payload).addr(), address);
            drop(payload);
            assert_eq!(call.state().load(Ordering::Relaxed), 0);
            Ok(call.return_value(returned))
        }
        fn hash<'call>(
            call: HostCall<'call, Profile, Provider, BigInt>,
            value: HostExternal<'call, HostExternalType<Schema>>,
        ) -> Result<HostCallCompletion<'call, BigInt>, crate::HostCallError> {
            let hash = call.source_hash::<HostExternalType<Schema>>(value);
            Ok(call.return_value(hash.into()))
        }
        let provider = HostProviderModule::new("application", "library")
            .expect("module")
            .with_external_type::<Provider, Schema>().expect("schema")
            .with_scoped_function::<Provider, (), HostExternalType<Schema>, _>("make", make).expect("make")
            .with_scoped_function::<Provider, (HostExternalType<Schema>,), HostExternalType<Schema>, _>("keep", keep).expect("keep")
            .with_scoped_function::<Provider, (HostExternalType<Schema>,), BigInt, _>("hash", hash).expect("hash");
        let source = r#"pub type Owned
@external(erlang, "native", "make")
fn make() -> Owned
@external(erlang, "native", "keep")
fn keep(value: Owned) -> Owned
@external(erlang, "native", "hash")
fn hash(value: Owned) -> Int
pub fn run() {
  let original = make()
  let returned = keep(original)
  let equal = make()
  echo returned
  original == returned && returned == equal && hash(returned) == hash(equal)
}
"#;
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::<Profile>::from_providers([provider]).expect("providers"),
        )
        .expect("source");
        let library = crate::planner::plan_host_library_program(program).expect("plan");
        let template = library
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("run")
            .signature()
            .id();
        let entry = crate::plan::LibraryEntry::new(
            template,
            crate::plan::LibraryValueType::Bool,
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries) =
            crate::plan::execution::HostedProgram::from_library_plan(library, entry, Vec::new())
                .expect("seal");
        let mut drops = Arc::new(AtomicUsize::new(0));
        let mut stores = HostExternalStore::default();
        let mut output = Vec::new();
        let mut echo = |value: crate::EchoOutput| output.push(value.to_string());
        let mut driver =
            crate::runtime::work::driver::Driver::new(&plan, &mut drops, &mut stores, &mut echo);
        assert!(
            driver
                .run_bool(
                    *entries.bools[0].function(),
                    crate::runtime::RetainedInputs::empty()
                )
                .expect("source call")
        );
        drop(driver);
        assert_eq!(output, ["src/library.gleam:12\nOwned(42)"]);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
}
