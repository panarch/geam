use super::CallError;
use super::input::{InputConstructions, ListFamily, ScopedFreshInput, ScopedInputValue};
use super::value::EmbeddingValue;
use super::work::return_::{ScopedReturn, ScopedTake};
use super::work::{ReadValue, ScopeBrand, ScopedOutput, SharedValue, SourceType};
use crate::host::HostProfile;
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryInputConstructions, LibraryListConstructions,
};
use crate::plan::{LibraryValueType, StandardVariant, ValueType};
use crate::runtime::execution::EntryContext;
use crate::runtime::{
    BorrowedValue, EmbeddingCustomInput, EmbeddingEntry, EmbeddingInputStorage,
    EmbeddingInputValue, EmbeddingOutput, EvaluatedExternalValue, RetainedInputs,
};
use std::marker::PhantomData;
use std::sync::Arc;

/// The exact source identity emitted by generated opaque bindings.
///
/// This describes a type, not its private constructors or native storage. Binding
/// checks the complete identity and arguments against the selected Gleam function.
#[doc(hidden)]
pub trait NamedTypeSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;

    fn arguments() -> Vec<ValueType> {
        Vec::new()
    }
}

/// A declaration for retaining a named Gleam custom without exposing its fields.
pub struct CustomType<Schema: NamedTypeSchema>(PhantomData<fn() -> Schema>);

/// A declaration for retaining an external through its existing provider storage.
pub struct ExternalType<Schema: NamedTypeSchema>(PhantomData<fn() -> Schema>);

/// An opaque custom value retained in its original execution domain.
///
/// Cloning shares its private fields. It can be passed back to Gleam in the same
/// domain, including through lists, tuples, Result, Option and Future results.
///
/// Even a parameter-free custom may hide scoped work, so it cannot be moved to
/// another execution lifetime:
///
/// ```compile_fail
/// use geam_core::embedding::{Custom, CustomType, ExecutionScope, Function, NamedTypeSchema};
/// use geam_core::HostProfile;
/// fn foreign<'a, 'b, P: HostProfile, S: NamedTypeSchema>(
///     scope: &ExecutionScope<'a, '_, P>,
///     function: &Function<(CustomType<S>,), CustomType<S>>,
///     value: Custom<'b, S>,
/// ) {
///     let _ = scope.call(function, (value,));
/// }
/// ```
///
/// Containment does not erase the lifetime:
///
/// ```compile_fail
/// use geam_core::embedding::{Completed, Custom, NamedTypeSchema, SharedList};
/// fn escape<'scope, S: NamedTypeSchema>(value: Completed<SharedList<Custom<'scope, S>>>)
///     -> Completed<SharedList<Custom<'static, S>>>
/// {
///     value
/// }
/// ```
pub struct Custom<'scope, Schema: NamedTypeSchema> {
    value: EmbeddingCustomInput,
    context: OpaqueContext<'scope>,
    schema: PhantomData<fn() -> Schema>,
}

/// An opaque external value retained in its original execution domain.
///
/// This handle neither exposes nor clones its Rust payload.
///
/// ```compile_fail
/// use geam_core::embedding::{External, ExternalType, ExecutionScope, Function, NamedTypeSchema};
/// use geam_core::HostProfile;
/// fn foreign<'a, 'b, P: HostProfile, S: NamedTypeSchema>(
///     scope: &ExecutionScope<'a, '_, P>,
///     function: &Function<(ExternalType<S>,), ExternalType<S>>,
///     value: External<'b, S>,
/// ) {
///     let _ = scope.call(function, (value,));
/// }
/// ```
pub struct External<'scope, Schema: NamedTypeSchema> {
    value: EvaluatedExternalValue,
    context: OpaqueContext<'scope>,
    schema: PhantomData<fn() -> Schema>,
}

#[derive(Clone)]
pub(crate) struct OpaqueContext<'scope> {
    _brand: ScopeBrand<'scope>,
    owner: Arc<()>,
}

macro_rules! opaque {
    ($type:ident, $value:ident, $runtime:ty, $family:ident, $name:ident, $entries:ident, $take:ident, $read:expr, $returned:expr) => {
        impl<Schema: NamedTypeSchema> EmbeddingValue for $type<Schema> {
            const VARIANT_COUNT: usize = 0;
            const LIST_COUNTS: [usize; 11] = [0; 11];
            const LIST_FAMILY: ListFamily = ListFamily::$family;

            fn library_type() -> LibraryValueType {
                LibraryValueType::$family(crate::plan::$type::new(
                    crate::plan::$name::new(
                        Schema::PACKAGE.into(),
                        Schema::MODULE.into(),
                        Schema::NAME.into(),
                    ),
                    Schema::arguments(),
                ))
            }
            fn collect_variants(_: &mut Vec<StandardVariant>) {}
            fn collect_lists(_: &mut Vec<LibraryValueType>) {}
        }

        impl<Schema: NamedTypeSchema> SourceType for $type<Schema> {
            type Value<'scope> = $value<'scope, Schema>;
        }

        impl<Profile: HostProfile, Schema: NamedTypeSchema> ScopedOutput<Profile>
            for $type<Schema>
        {
            type Retention = ();
            fn retain(_: &Profile::ExternalStores) {}
            fn context<'scope>(
                brand: ScopeBrand<'scope>,
                _: (),
                owner: &Arc<()>,
            ) -> OpaqueContext<'scope> {
                OpaqueContext {
                    _brand: brand,
                    owner: Arc::clone(owner),
                }
            }
        }

        impl<Schema: NamedTypeSchema> Clone for $value<'_, Schema> {
            fn clone(&self) -> Self {
                Self {
                    value: self.value.clone(),
                    context: self.context.clone(),
                    schema: PhantomData,
                }
            }
        }

        impl<'scope, Schema: NamedTypeSchema> ReadValue for $value<'scope, Schema> {
            type View<'value> = Self;
        }

        impl<'scope, Schema: NamedTypeSchema> SharedValue for $value<'scope, Schema> {
            type Context = OpaqueContext<'scope>;
            fn view(value: BorrowedValue<'_>, context: &Self::Context) -> Self {
                Self {
                    value: ($read)(value),
                    context: context.clone(),
                    schema: PhantomData,
                }
            }
        }

        impl<Profile: HostProfile, Schema: NamedTypeSchema> ScopedTake<Profile> for $type<Schema> {
            fn take<'scope>(
                output: &mut EmbeddingOutput,
                context: &<Self::Value<'scope> as SharedValue>::Context,
            ) -> Self::Value<'scope> {
                $value {
                    value: output.$take(),
                    context: context.clone(),
                    schema: PhantomData,
                }
            }
        }

        impl<Profile: HostProfile, Schema: NamedTypeSchema> ScopedReturn<Profile>
            for $type<Schema>
        {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.$entries[slot].inputs()
            }
            async fn call<'scope>(
                execution: &EntryContext<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedInputs,
                context: <Self::Value<'scope> as SharedValue>::Context,
            ) -> Result<Self::Value<'scope>, CallError> {
                let value = entries.$entries[slot]
                    .function()
                    .call(execution, inputs)
                    .await?;
                Ok($value {
                    value: ($returned)(value),
                    context,
                    schema: PhantomData,
                })
            }
        }

        impl<'scope, Schema: NamedTypeSchema>
            ScopedInputValue<$value<'scope, Schema>, ScopeBrand<'scope>> for $type<Schema>
        {
            type ScopedRuntime = $runtime;
            fn owners_match(input: &$value<'scope, Schema>, owner: &Arc<()>) -> bool {
                Arc::ptr_eq(&input.context.owner, owner)
            }
            fn into_runtime(
                input: $value<'scope, Schema>,
                _: &mut InputConstructions<'_>,
                _: &EmbeddingInputStorage,
            ) -> Self::ScopedRuntime {
                input.value
            }
        }

        impl<'scope, Schema: NamedTypeSchema>
            ScopedInputValue<&$value<'scope, Schema>, ScopeBrand<'scope>> for $type<Schema>
        {
            type ScopedRuntime = $runtime;
            fn owners_match(input: &&$value<'scope, Schema>, owner: &Arc<()>) -> bool {
                Arc::ptr_eq(&input.context.owner, owner)
            }
            fn into_runtime(
                input: &$value<'scope, Schema>,
                _: &mut InputConstructions<'_>,
                _: &EmbeddingInputStorage,
            ) -> Self::ScopedRuntime {
                input.value.clone()
            }
        }

        impl<'scope, Schema: NamedTypeSchema>
            ScopedFreshInput<$value<'scope, Schema>, ScopeBrand<'scope>> for $type<Schema>
        {
            fn list_id(
                lists: &LibraryListConstructions,
                index: usize,
            ) -> <Self::ScopedRuntime as EmbeddingInputValue>::ListType {
                lists.$entries[index]
            }
        }

        impl<'scope, Schema: NamedTypeSchema>
            ScopedFreshInput<&$value<'scope, Schema>, ScopeBrand<'scope>> for $type<Schema>
        {
            fn list_id(
                lists: &LibraryListConstructions,
                index: usize,
            ) -> <Self::ScopedRuntime as EmbeddingInputValue>::ListType {
                lists.$entries[index]
            }
        }
    };
}

opaque!(
    CustomType,
    Custom,
    EmbeddingCustomInput,
    Custom,
    CustomTypeName,
    customs,
    take_custom,
    |value: BorrowedValue<'_>| value.retained_custom(),
    |mut value: EmbeddingOutput| value.take_custom()
);
opaque!(
    ExternalType,
    External,
    EvaluatedExternalValue,
    External,
    ExternalTypeName,
    externals,
    take_external,
    |value: BorrowedValue<'_>| value.external().clone(),
    std::convert::identity
);

#[cfg(test)]
mod tests {
    use super::{CustomType, ExternalType, NamedTypeSchema};
    use crate::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder, List};
    use crate::execution_fixture::TestHost;
    use crate::host::{HostProfile, HostProviderSet};
    use crate::{ModuleSource, PackageSource};

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = ();
        type ExecutionState = ();
    }

    struct Session;
    impl NamedTypeSchema for Session {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Session";
    }

    #[test]
    fn opaque_session_reentry_preserves_private_fields_and_composes_with_standard_values() {
        use super::ScopedInputValue;
        type SessionType = CustomType<Session>;
        type Package = (SessionType, Result<SessionType, BigInt>, List<SessionType>);
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub opaque type Session {
  Session(saved: Result(Int, String), items: List(Int), callback: fn(Int) -> Int)
}
pub fn start() { Session(Ok(40), [1, 2, 3], fn(value) { value + 2 }) }
pub fn keep(value: Session) { value }
pub fn package(value: #(Session, Result(Session, Int), List(Session))) { value }
pub fn read(value: Session) {
  let Session(saved, items, callback) = value
  let assert Ok(saved) = saved
  let assert [1, 2, 3] = items
  callback(saved)
}
"#,
                )],
            )],
            HostProviderSet::<Profile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let (mut bindings, start) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), SessionType>::new("start"))
            .unwrap();
        let keep = bindings
            .function(FunctionDeclaration::<(SessionType,), SessionType>::new(
                "keep",
            ))
            .unwrap();
        let package = bindings
            .function(FunctionDeclaration::<(Package,), Package>::new("package"))
            .unwrap();
        let read = bindings
            .function(FunctionDeclaration::<(SessionType,), BigInt>::new("read"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                let session = scope.call(&start, ()).await.unwrap();
                assert!(SessionType::owners_match(&session, &session.context.owner));
                assert!(!SessionType::owners_match(
                    &session,
                    &std::sync::Arc::new(())
                ));
                assert!(!SessionType::owners_match(
                    &&session,
                    &std::sync::Arc::new(())
                ));
                let second = scope.call(&start, ()).await.unwrap();
                assert!(!session.value.same_allocation(&second.value));
                let alias = session.clone();
                assert!(session.value.same_allocation(&alias.value));
                let returned = scope.call(&keep, (&alias,)).await.unwrap();
                assert!(session.value.same_allocation(&returned.value));
                assert_eq!(
                    scope.call(&read, (returned,)).await.unwrap(),
                    BigInt::from(42)
                );
                let packed = scope
                    .call(
                        &package,
                        ((
                            &session,
                            Ok::<_, BigInt>(&session),
                            vec![session.clone(), second],
                        ),),
                    )
                    .await
                    .unwrap();
                assert!(packed.0.value.same_allocation(&session.value));
                assert!(
                    packed
                        .1
                        .as_ref()
                        .unwrap()
                        .value
                        .same_allocation(&session.value)
                );
                assert_eq!(packed.2.len(), 2);
                let borrowed_items = scope
                    .call(
                        &package,
                        ((&session, Ok::<_, BigInt>(&session), vec![&session, &alias]),),
                    )
                    .await
                    .unwrap();
                assert!(
                    borrowed_items
                        .2
                        .read_item(1, |item| item.value.same_allocation(&alias.value))
                        .unwrap()
                );
                let item = packed.2.read_item(0, |item| item).unwrap();
                assert!(item.value.same_allocation(&session.value));
                let again = scope
                    .call(
                        &package,
                        ((
                            &packed.0,
                            packed.1.as_ref().map_err(Clone::clone),
                            &packed.2,
                        ),),
                    )
                    .await
                    .unwrap();
                assert!(again.0.value.same_allocation(&session.value));
                assert_eq!(scope.call(&read, (item,)).await.unwrap(), BigInt::from(42));
            }),
        )
        .unwrap();
        assert!(echo.is_empty());
    }

    #[test]
    fn opaque_binding_checks_the_complete_generic_specialization() {
        use crate::embedding::BindingError;
        use crate::plan::{CustomType as SourceCustom, CustomTypeName, FunctionType, ValueType};

        struct NumberSession;
        impl NamedTypeSchema for NumberSession {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Session";
            fn arguments() -> Vec<ValueType> {
                vec![ValueType::Int]
            }
        }
        struct TextSession;
        impl NamedTypeSchema for TextSession {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Session";
            fn arguments() -> Vec<ValueType> {
                vec![ValueType::String]
            }
        }
        struct ForeignPackage;
        impl NamedTypeSchema for ForeignPackage {
            const PACKAGE: &'static str = "foreign";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Session";
            fn arguments() -> Vec<ValueType> {
                vec![ValueType::Int]
            }
        }
        struct ForeignModule;
        impl NamedTypeSchema for ForeignModule {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "foreign";
            const NAME: &'static str = "Session";
            fn arguments() -> Vec<ValueType> {
                vec![ValueType::Int]
            }
        }
        struct OtherType;
        impl NamedTypeSchema for OtherType {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Other";
            fn arguments() -> Vec<ValueType> {
                vec![ValueType::Int]
            }
        }
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub opaque type Session(a) { Session(a) }
pub fn start() { Session(42) }
pub fn keep(value: Session(Int)) { echo "entered" value }
"#,
                )],
            )],
            HostProviderSet::<Profile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let (mut bindings, start) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), CustomType<NumberSession>>::new(
                "start",
            ))
            .unwrap();
        let type_ = |arguments| {
            ValueType::Custom(SourceCustom::new(
                CustomTypeName::new("application".into(), "library".into(), "Session".into()),
                arguments,
            ))
        };
        let number = type_(vec![ValueType::Int]);
        let text = type_(vec![ValueType::String]);
        for (error, package, module, name) in [
            (
                bindings
                    .function(FunctionDeclaration::<
                        (CustomType<ForeignPackage>,),
                        CustomType<ForeignPackage>,
                    >::new("keep"))
                    .err(),
                "foreign",
                "library",
                "Session",
            ),
            (
                bindings
                    .function(FunctionDeclaration::<
                        (CustomType<ForeignModule>,),
                        CustomType<ForeignModule>,
                    >::new("keep"))
                    .err(),
                "application",
                "foreign",
                "Session",
            ),
            (
                bindings
                    .function(FunctionDeclaration::<
                        (CustomType<OtherType>,),
                        CustomType<OtherType>,
                    >::new("keep"))
                    .err(),
                "application",
                "library",
                "Other",
            ),
        ] {
            let expected = ValueType::Custom(SourceCustom::new(
                CustomTypeName::new(package.into(), module.into(), name.into()),
                vec![ValueType::Int],
            ));
            assert_eq!(
                error,
                Some(BindingError::SignatureMismatch {
                    name: "keep".into(),
                    expected: FunctionType::new(vec![expected.clone()], expected),
                    found: FunctionType::new(vec![number.clone()], number.clone()),
                })
            );
        }
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<
                    (CustomType<TextSession>,),
                    CustomType<TextSession>,
                >::new("keep"))
                .err(),
            Some(BindingError::SignatureMismatch {
                name: "keep".into(),
                expected: FunctionType::new(vec![text.clone()], text),
                found: FunctionType::new(vec![number.clone()], number.clone()),
            })
        );
        let no_arguments = type_(vec![]);
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<
                    (CustomType<Session>,),
                    CustomType<Session>,
                >::new("keep"))
                .err(),
            Some(BindingError::SignatureMismatch {
                name: "keep".into(),
                expected: FunctionType::new(vec![no_arguments.clone()], no_arguments),
                found: FunctionType::new(vec![number.clone()], number),
            })
        );
        let keep = bindings
            .function(FunctionDeclaration::<
                (CustomType<NumberSession>,),
                CustomType<NumberSession>,
            >::new("keep"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                let session = scope.call(&start, ()).await.unwrap();
                let returned = scope.call(&keep, (&session,)).await.unwrap();
                assert!(session.value.same_allocation(&returned.value));
            }),
        )
        .unwrap();
        assert_eq!(echo.len(), 1);
    }

    #[test]
    fn hidden_work_and_scoped_completions_keep_the_original_domain_and_graph() {
        use crate::host::{HostComponentProfile, HostFutureStore, HostWorkProfile};
        use crate::work_fixture::{WorkComponent, WorkType};

        struct WorkProfile;
        impl HostProfile for WorkProfile {
            type RunState = ();
            type ExternalStores = HostFutureStore;
            type ExecutionState = ();
        }
        impl HostWorkProfile for WorkProfile {
            type Work = WorkComponent;
        }
        impl HostComponentProfile<WorkComponent> for WorkProfile {
            fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
                stores
            }
            fn component_state(state: &mut ()) -> &mut () {
                state
            }
        }

        let mut state = ();
        assert!(std::ptr::eq(
            WorkProfile::component_state(&mut state),
            &state
        ));

        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
import fixture/work
pub opaque type Session { Session(work.Work(Int)) }
pub fn start() {
  Session(work.map(work.ready(40), fn(value) { echo "completed" value + 2 }))
}
pub fn unpack(value: Session) { let Session(work) = value work }
pub fn package(value: Session) { work.ready([value]) }
"#,
                    )],
                ),
            ],
            HostProviderSet::<WorkProfile>::from_providers(WorkComponent::providers().unwrap())
                .unwrap(),
        )
        .unwrap();
        let (mut bindings, start) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), CustomType<Session>>::new("start"))
            .unwrap();
        let unpack = bindings
            .function(FunctionDeclaration::<
                (CustomType<Session>,),
                WorkType<BigInt>,
            >::new("unpack"))
            .unwrap();
        let package = bindings
            .function(FunctionDeclaration::<
                (CustomType<Session>,),
                WorkType<List<CustomType<Session>>>,
            >::new("package"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                let session = scope.call(&start, ()).await.unwrap();
                let packed = scope.call(&package, (&session,)).await.unwrap();
                let result = scope.observe(&packed).await.unwrap();
                let restored = result
                    .read(|values| values.read_item(0, |value| value))
                    .unwrap();
                assert!(session.value.same_allocation(&restored.value));
                drop(result);
                drop(packed);
                let original = scope.call(&unpack, (session,)).await.unwrap();
                let returned = scope.call(&unpack, (restored,)).await.unwrap();
                assert_eq!(
                    scope.observe(&original).await.unwrap().read(Clone::clone),
                    BigInt::from(42)
                );
                assert_eq!(
                    scope.observe(&returned).await.unwrap().read(Clone::clone),
                    BigInt::from(42)
                );
            }),
        )
        .unwrap();
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["library.gleam:5\n\"completed\""]
        );
    }

    struct ResourceProfile;
    impl HostProfile for ResourceProfile {
        type RunState = std::sync::Arc<std::sync::atomic::AtomicUsize>;
        type ExternalStores = crate::HostExternalStore<ResourcePayload>;
        type ExecutionState = ();
    }

    struct Resource;
    impl NamedTypeSchema for Resource {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Resource";
    }
    impl crate::HostExternalSchema for Resource {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Resource";
        const PARAMETER_COUNT: usize = 0;
    }
    struct ResourceProvider;
    impl crate::HostProvider<ResourceProfile> for ResourceProvider {
        type State = std::sync::Arc<std::sync::atomic::AtomicUsize>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }
    impl crate::HostExternalBinding<ResourceProfile, Resource> for ResourceProvider {
        type Storage = Self;
    }
    struct ResourcePayload {
        value: std::cell::Cell<usize>,
        drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }
    impl Drop for ResourcePayload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    impl crate::HostExternalStorage<ResourceProfile, Resource> for ResourceProvider {
        type Payload = ResourcePayload;
        fn store(
            stores: &crate::HostExternalStore<Self::Payload>,
        ) -> &crate::HostExternalStore<Self::Payload> {
            stores
        }
        fn source_equal(
            _: &crate::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left.value.get() == right.value.get()
        }
        fn source_hash(_: &crate::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            value.value.get() as u64
        }
        fn inspect(
            _: &crate::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> crate::embedding::EcoString {
            format!("Resource({})", value.value.get()).into()
        }
    }

    #[test]
    fn external_handles_retain_send_only_payloads_through_private_fields_without_cloning() {
        use super::ScopedInputValue;
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostExternalType, HostProviderModule,
        };
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        fn make(
            mut call: HostCall<'_, ResourceProfile, ResourceProvider, HostExternalType<Resource>>,
        ) -> Result<HostCallCompletion<'_, HostExternalType<Resource>>, HostCallError> {
            let drops = Arc::clone(call.state());
            let resource = call.create_external_with_binding::<ResourceProvider>(ResourcePayload {
                value: std::cell::Cell::new(42),
                drops,
            });
            Ok(call.return_value(resource))
        }
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_external_type::<ResourceProvider, Resource>()
            .unwrap()
            .with_scoped_function::<ResourceProvider, (), HostExternalType<Resource>, _>(
                "make", make,
            )
            .unwrap();
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Resource
pub opaque type Session { Session(Resource) }
@external(erlang, "native", "make")
pub fn make() -> Resource
pub fn keep(value: Resource) { value }
pub fn items(values: List(Resource)) { values }
pub fn wrap(value: Resource) { Session(value) }
pub fn unwrap(value: Session) { let Session(value) = value value }
pub fn inspect(left: Resource, right: Resource) { echo left left == right }
"#,
                )],
            )],
            HostProviderSet::<ResourceProfile>::from_providers([provider]).unwrap(),
        )
        .unwrap();
        type ResourceType = ExternalType<Resource>;
        let (mut bindings, items) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<
                (List<ResourceType>,),
                List<ResourceType>,
            >::new("items"))
            .unwrap();
        let make = bindings
            .function(FunctionDeclaration::<(), ResourceType>::new("make"))
            .unwrap();
        let keep = bindings
            .function(FunctionDeclaration::<(ResourceType,), ResourceType>::new(
                "keep",
            ))
            .unwrap();
        let wrap = bindings
            .function(FunctionDeclaration::<(ResourceType,), CustomType<Session>>::new("wrap"))
            .unwrap();
        let unwrap = bindings
            .function(FunctionDeclaration::<(CustomType<Session>,), ResourceType>::new("unwrap"))
            .unwrap();
        let inspect = bindings
            .function(FunctionDeclaration::<(ResourceType, ResourceType), bool>::new("inspect"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let mut state = Arc::clone(&drops);
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let value = scope.call(&make, ()).await.unwrap();
                assert!(ResourceType::owners_match(&value, &value.context.owner));
                assert!(!ResourceType::owners_match(&value, &Arc::new(())));
                assert!(!ResourceType::owners_match(&&value, &Arc::new(())));
                let alias = value.clone();
                let retained = scope.call(&keep, (&alias,)).await.unwrap();
                let other = scope.call(&make, ()).await.unwrap();
                assert!(scope.call(&inspect, (&value, &other)).await.unwrap());
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                drop(other);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
                let session = scope.call(&wrap, (retained,)).await.unwrap();
                let value = scope.call(&unwrap, (session,)).await.unwrap();
                let borrowed_items = scope.call(&items, (vec![&value, &alias],)).await.unwrap();
                assert_eq!(borrowed_items.len(), 2);
                drop(borrowed_items);
                let values = scope.call(&items, (vec![value, alias],)).await.unwrap();
                let item = values.read_item(0, |item| item).unwrap();
                let again = scope.call(&items, (&values,)).await.unwrap();
                drop(values);
                drop(again);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
                let final_value = scope.call(&keep, (item,)).await.unwrap();
                drop(final_value);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
            }),
        )
        .unwrap();
        // Echo owns its retained diagnostic until the caller releases it.
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["library.gleam:10\nResource(42)"]
        );
        drop(echo);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn resource_fixture_hashes_its_payload_value() {
        use crate::host::{HostExternalHashing, HostExternalStorage, RetainedValueHashing};
        let source_hash = |_: &crate::runtime::RetainedValueRef| 0;
        let raw_hashing = RetainedValueHashing::new(&source_hash);
        let hashing = HostExternalHashing(&raw_hashing);
        let payload = ResourcePayload {
            value: std::cell::Cell::new(42),
            drops: std::sync::Arc::default(),
        };
        assert_eq!(ResourceProvider::source_hash(&hashing, &payload), 42);
    }

    #[test]
    fn opaque_and_compound_returns_preserve_source_failure_before_producing_a_value() {
        let provider = crate::HostProviderModule::new("application", "library")
            .unwrap()
            .with_external_type::<ResourceProvider, Resource>()
            .unwrap();
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Resource
pub opaque type Session { Session(Int) }
pub fn custom() -> Session { panic as "custom stopped" }
pub fn external() -> Resource { panic as "external stopped" }
pub fn compound(stop: Bool) -> #(Session, List(Resource)) {
  case stop {
    True -> panic as "compound stopped"
    False -> #(Session(42), [])
  }
}
pub fn read(session: Session) { let Session(value) = session value }
"#,
                )],
            )],
            HostProviderSet::<ResourceProfile>::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let (mut bindings, custom) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), CustomType<Session>>::new(
                "custom",
            ))
            .unwrap();
        let external = bindings
            .function(FunctionDeclaration::<(), ExternalType<Resource>>::new(
                "external",
            ))
            .unwrap();
        let compound = bindings
            .function(FunctionDeclaration::<
                (bool,),
                (CustomType<Session>, List<ExternalType<Resource>>),
            >::new("compound"))
            .unwrap();
        let read = bindings
            .function(FunctionDeclaration::<(CustomType<Session>,), BigInt>::new(
                "read",
            ))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let mut state = std::sync::Arc::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                assert_eq!(
                    scope.call(&custom, ()).await.err().unwrap().to_string(),
                    "panic: custom stopped"
                );
                assert_eq!(
                    scope.call(&external, ()).await.err().unwrap().to_string(),
                    "panic: external stopped"
                );
                assert_eq!(
                    scope
                        .call(&compound, (true,))
                        .await
                        .err()
                        .unwrap()
                        .to_string(),
                    "panic: compound stopped"
                );
                let (session, resources) = scope.call(&compound, (false,)).await.unwrap();
                assert!(resources.is_empty());
                assert_eq!(scope.call(&read, (&session,)).await.unwrap(), 42.into());
            }),
        )
        .unwrap();
        assert!(echo.is_empty());
    }
}
