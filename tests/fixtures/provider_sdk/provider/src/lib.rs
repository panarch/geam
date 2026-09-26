use geam::host::native::{NativeCall, NativeRules};
use geam::provider::advanced::NativeValue;
use geam::provider::{BigInt, EcoString, GleamError, GleamOk, GleamResult, StringValue};
use geam::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
    HostCallableSchema, HostCaptures, HostComponentProfile, HostConstructions, HostCreatedFunction,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomSchema, HostCustomType, HostExternal, HostExternalBinding,
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
    HostExternalStorage, HostExternalStore, HostExternalType, HostFailure, HostFunctionType,
    HostListType, HostOwnedCompletion, HostProvider, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostReturns, HostTypeIndex0, HostTypeIndexNext, HostTypeList,
    HostTypeListEnd, HostTypeParameter,
};
use provider_sdk_example_domain::Catalog;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Component;

#[derive(Default)]
pub struct Stores {
    catalogs: HostExternalStore<Catalog>,
}

#[derive(Debug)]
pub struct RunState {
    prefix: EcoString,
    calls: usize,
}

struct Provider;

struct Prefix;

struct CatalogSchema;

struct CatalogStorage;

struct SummarySchema;

struct SummaryDefinition;

struct SummaryCountField;

struct SummaryItemsField;

type Output = HostTypeParameter<0>;
type Cleanup = HostTypeParameter<1>;
type UnitArguments = HostTypeList<(), HostTypeListEnd>;
type GenericTargets = HostTypeList<(), HostTypeList<Output, HostTypeListEnd>>;
type OutputTarget = HostTypeList<Output, HostTypeListEnd>;

type TransformArguments = HostTypeList<StringValue, HostTypeListEnd>;
type Transform = HostFunctionType<TransformArguments, StringValue>;
type PrefixConstructions = HostTypeList<HostCreatedFunction<Prefix>, HostTypeListEnd>;
type HostCatalog = HostExternalType<CatalogSchema>;
type Summary = HostCustomType<SummarySchema>;
type SummaryConstructor = HostCustomConstructorAt<Summary, HostCustomIndex0, SummaryDefinition>;
type SummaryItems = HostListType<StringValue>;
type SummaryConstructions = HostTypeList<SummaryItems, HostTypeListEnd>;

impl HostProviderComponent for Component {
    const ID: &'static str = "provider-sdk-example";
    type Stores = Stores;
    type RunState = RunState;
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        configuration: &HostProviderConfiguration,
    ) -> Result<Self::RunState, HostProviderInitializationError> {
        let prefix = configuration
            .get("prefix")
            .and_then(|value| value.as_string())
            .cloned()
            .ok_or_else(|| {
                HostProviderInitializationError::for_component::<Self>(
                    "configuration key `prefix` must be a String",
                )
            })?;

        Ok(RunState { prefix, calls: 0 })
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("provider_sdk_example", "provider/sdk")
            .and_then(HostProviderModule::with_external_type::<Provider, CatalogSchema>)
            .and_then(|provider| {
                provider.with_resumable_function::<Provider, (StringValue, Transform), StringValue, HostTypeListEnd, _>(
                    "decorate",
                    decorate::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_callable::<Provider, Prefix, (StringValue,), _>(prefix::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Provider, (StringValue,), Transform, PrefixConstructions, _,
                >("make_transform", make_transform::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (), HostCatalog, _>(
                    "catalog_new",
                    catalog_new::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_scoped_function::<
                    Provider,
                    (HostCatalog, StringValue, StringValue),
                    HostCatalog,
                    _,
                >("catalog_insert", catalog_insert::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (HostCatalog,), BigInt, _>(
                    "catalog_hash",
                    catalog_hash::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_resumable_function::<
                    Provider,
                    (StringValue, Transform),
                    Summary,
                    SummaryConstructions,
                    _,
                >("summarize", summarize::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function::<
                    Provider, (StringValue,), GleamResult<StringValue, BigInt>, _,
                >("non_empty", non_empty::<Profile>)
            })
            .and_then(|provider| provider.with_resumable_native_function::<
                Provider, (HostFunctionType<UnitArguments, Cleanup>, HostFunctionType<UnitArguments, Output>),
                Output, GenericTargets, _,
            >("around", NativeRules::default(), around::<Profile>))
            .and_then(|provider| provider.with_scoped_function::<Provider, (), Output, _>("produce", produce::<Profile>))
            .and_then(|provider| provider.with_resumable_function::<Provider, (), Output, HostTypeListEnd, _>("late_failure", late_failure::<Profile>))
            .and_then(|provider| provider.with_resumable_native_function::<Provider, (), Output, OutputTarget, _>("native_value", NativeRules::default(), native_value::<Profile>))
            .map(|provider| vec![provider])
    }
}

impl<Profile> HostProvider<Profile> for Provider
where
    Profile: HostComponentProfile<Component>,
{
    type State = RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::component_state(state)
    }
}

impl HostCallableSchema for Prefix {
    const PACKAGE: &'static str = "provider_sdk_example";
    const MODULE: &'static str = "provider/sdk";
    const NAME: &'static str = "prefix";
    type Arguments = TransformArguments;
    type Return = StringValue;
    type Captures = TransformArguments;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}

impl HostExternalSchema for CatalogSchema {
    const PACKAGE: &'static str = "provider_sdk_example";
    const MODULE: &'static str = "provider/sdk";
    const NAME: &'static str = "Catalog";
    const PARAMETER_COUNT: usize = 0;
}

impl<Profile> HostExternalStorage<Profile, CatalogSchema> for CatalogStorage
where
    Profile: HostComponentProfile<Component>,
{
    type Payload = Catalog;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::component_stores(stores).catalogs
    }

    fn source_equal(
        _context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left == right
    }

    fn source_hash(_context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        for (key, value) in value.iter() {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        let entries = value
            .iter()
            .map(|(key, value)| format!("#({key:?}, {value:?})"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Catalog([{entries}])").into()
    }
}

impl<Profile> HostExternalBinding<Profile, CatalogSchema> for Provider
where
    Profile: HostComponentProfile<Component>,
{
    type Storage = CatalogStorage;
}

impl HostCustomField for SummaryCountField {
    const LABEL: Option<&'static str> = Some("count");

    type Type = BigInt;
}

impl HostCustomField for SummaryItemsField {
    const LABEL: Option<&'static str> = Some("items");

    type Type = SummaryItems;
}

impl HostCustomConstructorDefinition for SummaryDefinition {
    const NAME: &'static str = "Summary";

    type Fields = HostCustomFieldList<
        SummaryCountField,
        HostCustomFieldList<SummaryItemsField, HostCustomFieldListEnd>,
    >;
}

impl HostCustomSchema for SummarySchema {
    const PACKAGE: &'static str = "provider_sdk_example";
    const MODULE: &'static str = "provider/sdk";
    const NAME: &'static str = "Summary";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<SummaryDefinition, HostCustomConstructorListEnd>;
}

impl RunState {
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn calls(&self) -> usize {
        self.calls
    }
}

fn decorate<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, StringValue>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    value: StringValue,
    transform: HostCallable<'call, TransformArguments, StringValue>,
) -> Result<HostCallContinuation<'call, StringValue>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let decorated = {
        let state = call.state();
        state.calls += 1;
        format!("{}{}", state.prefix, value)
    };
    let transform = call.owned_callable(transform, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let transformed = transform
                .invoke(
                    &context,
                    move |_, _| (decorated.into(), ()),
                    |_, _, value| Ok(value),
                )
                .await?;
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(transformed))
            }))
        })
    }))
}

fn make_transform<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, Transform>,
    constructions: HostConstructions<'call, PrefixConstructions>,
    prefix: StringValue,
) -> Result<HostCallCompletion<'call, Transform>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let function = call.construct_function(constructions.at::<HostTypeIndex0>(), (prefix, ()));
    Ok(call.return_value(function))
}

fn prefix<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, StringValue>,
    captures: HostCaptures<'call, TransformArguments>,
    _: HostConstructions<'call, HostTypeListEnd>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, StringValue>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let (prefix, ()) = call.captures(captures);
    call.state().calls += 1;
    Ok(call.return_value(format!("{prefix}{value}").into()))
}

fn catalog_new<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, HostCatalog>,
) -> Result<HostCallCompletion<'call, HostCatalog>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let catalog = call.create_external(Catalog::default());
    Ok(call.return_value(catalog))
}

fn catalog_insert<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, HostCatalog>,
    catalog: HostExternal<'call, HostCatalog>,
    key: StringValue,
    value: StringValue,
) -> Result<HostCallCompletion<'call, HostCatalog>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let updated = call
        .external_payload(catalog)
        .insert(key.as_str(), value.as_str());
    let updated = call.create_external(updated);
    Ok(call.return_value(updated))
}

fn catalog_hash<'call, Profile>(
    call: HostCall<'call, Profile, Provider, BigInt>,
    catalog: HostExternal<'call, HostCatalog>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let hash = BigInt::from(call.source_hash::<HostCatalog>(catalog));
    Ok(call.return_value(hash))
}

fn summarize<'call, Profile>(
    call: HostCall<'call, Profile, Provider, Summary>,
    constructions: HostConstructions<'call, SummaryConstructions>,
    value: StringValue,
    transform: HostCallable<'call, TransformArguments, StringValue>,
) -> Result<HostCallContinuation<'call, Summary>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let transform = call.owned_callable(transform, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let item = transform
                .invoke(&context, move |_, _| (value, ()), |_, _, value| Ok(value))
                .await?;
            Ok(HostOwnedCompletion::new(move |mut call, constructions| {
                let items = call.construct_list(constructions.at::<HostTypeIndex0>(), [item]);
                Ok(call.return_custom::<SummaryConstructor>((BigInt::from(1), (items, ()))))
            }))
        })
    }))
}

fn non_empty<'call, Profile>(
    call: HostCall<'call, Profile, Provider, GleamResult<StringValue, BigInt>>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, GleamResult<StringValue, BigInt>>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    if value.as_str().is_empty() {
        Ok(call.return_custom::<GleamError<StringValue, BigInt>>((BigInt::from(0), ())))
    } else {
        Ok(call.return_custom::<GleamOk<StringValue, BigInt>>((value, ())))
    }
}

fn around<'call, Profile>(
    mut call: NativeCall<'call, Profile, Provider, Output, GenericTargets>,
    cleanup: HostCallable<'call, UnitArguments, Cleanup>,
    body: HostCallable<'call, UnitArguments, Output>,
) -> Result<HostCallContinuation<'call, Output>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    call.call().state().calls += 1;
    let cleanup = call.owned_callable::<HostTypeIndex0, _>(cleanup);
    let body = call.owned_callable::<HostTypeIndex0, _>(body);
    Ok(
        call.resume::<HostTypeIndexNext<HostTypeIndex0>>(move |context| {
            Box::pin(async move {
                let result = body.invoke(&context, NativeValue::symbol("nil")).await;
                cleanup.invoke(&context, NativeValue::symbol("nil")).await?;
                result
            })
        }),
    )
}

fn produce<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, Output>,
) -> Result<HostCallCompletion<'call, Output>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    call.state().calls += 1;
    Err(HostFailure::new("producer stopped").into())
}

fn late_failure<'call, Profile>(
    call: HostCall<'call, Profile, Provider, Output>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
) -> Result<HostCallContinuation<'call, Output>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    Ok(call.resume(constructions, |_| {
        Box::pin(async {
            Ok(HostOwnedCompletion::<
                Profile,
                Provider,
                Output,
                HostTypeListEnd,
            >::new(|mut call, _| {
                call.state().calls += 1;
                Err(HostFailure::new("codec stopped").into())
            }))
        })
    }))
}

fn native_value<'call, Profile>(
    mut call: NativeCall<'call, Profile, Provider, Output, OutputTarget>,
) -> Result<HostCallContinuation<'call, Output>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    call.call().state().calls += 1;
    Ok(call.resume::<HostTypeIndex0>(|_| Box::pin(async { Ok(NativeValue::symbol("nil")) })))
}

#[cfg(test)]
mod tests {
    use super::{Component, RunState};
    use geam::provider::EcoString;
    use geam::{
        HostProviderComponentInitialization, HostProviderConfiguration,
        HostProviderInitializationError,
    };
    use std::collections::BTreeMap;

    #[test]
    fn component_initialization_requires_an_explicit_string_prefix() {
        let configuration = HostProviderConfiguration::new(BTreeMap::from([(
            EcoString::from("prefix"),
            EcoString::from("docs:").into(),
        )]));

        let state = Component::initialize(&configuration)
            .expect("string prefix should initialize provider state");
        assert_eq!(state.prefix(), "docs:");
        assert_eq!(state.calls(), 0);

        let error = Component::initialize(&HostProviderConfiguration::empty())
            .expect_err("missing prefix should fail initialization");
        assert_eq!(
            error,
            HostProviderInitializationError::for_component::<Component>(
                "configuration key `prefix` must be a String",
            )
        );

        let wrong_type = HostProviderConfiguration::new(BTreeMap::from([(
            EcoString::from("prefix"),
            true.into(),
        )]));
        assert_eq!(
            Component::initialize(&wrong_type)
                .expect_err("non-string prefix should fail initialization"),
            HostProviderInitializationError::for_component::<Component>(
                "configuration key `prefix` must be a String",
            )
        );
    }

    #[test]
    fn run_state_accessors_report_current_component_state() {
        let state = RunState {
            prefix: "sdk:".into(),
            calls: 3,
        };

        assert_eq!(state.prefix(), "sdk:");
        assert_eq!(state.calls(), 3);
    }
}
