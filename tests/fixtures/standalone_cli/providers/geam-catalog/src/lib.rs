use ecow::EcoString;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostComponentProfile,
    HostConstructions, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
    HostCustomFieldListEnd, HostCustomIndex0, HostCustomSchema, HostCustomType, HostExternal,
    HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
    HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostFunctionType,
    HostListType, HostProvider, HostProviderComponent, HostProviderComponentInitialization,
    HostProviderComponentRegistration, HostProviderConfiguration, HostProviderInitializationError,
    HostProviderModule, HostRegistrationError, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use num_bigint::BigInt;
use standalone_catalog_domain::Catalog;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Component;

#[derive(Default)]
pub struct Stores {
    catalogs: HostExternalStore<Catalog>,
}

pub struct RunState {
    prefix: EcoString,
}

struct Provider;
struct CatalogSchema;
struct CatalogStorage;
struct SummarySchema;
struct SummaryDefinition;
struct SummaryCountField;
struct SummaryItemsField;

type TransformArguments = HostTypeList<EcoString, HostTypeListEnd>;
type Transform = HostFunctionType<TransformArguments, EcoString>;
type HostCatalog = HostExternalType<CatalogSchema>;
type Summary = HostCustomType<SummarySchema>;
type SummaryConstructor = HostCustomConstructorAt<Summary, HostCustomIndex0, SummaryDefinition>;
type SummaryItems = HostListType<EcoString>;
type SummaryConstructions = HostTypeList<SummaryItems, HostTypeListEnd>;

impl HostProviderComponent for Component {
    const ID: &'static str = "geam-catalog";
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
        Ok(RunState { prefix })
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("catalog", "catalog")
            .and_then(HostProviderModule::with_external_type::<Provider, CatalogSchema>)
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (), HostCatalog, _>(
                    "new",
                    catalog_new::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_scoped_function::<
                    Provider,
                    (HostCatalog, EcoString, EcoString),
                    HostCatalog,
                    _,
                >("insert", catalog_insert::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Provider,
                    (HostCatalog, Transform),
                    Summary,
                    SummaryConstructions,
                    _,
                >("summarize", summarize::<Profile>)
            })
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

impl HostExternalSchema for CatalogSchema {
    const PACKAGE: &'static str = "catalog";
    const MODULE: &'static str = "catalog";
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
        for (key, value) in value.entries() {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        let entries = value
            .entries()
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
    const PACKAGE: &'static str = "catalog";
    const MODULE: &'static str = "catalog";
    const NAME: &'static str = "Summary";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<SummaryDefinition, HostCustomConstructorListEnd>;
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
    key: EcoString,
    value: EcoString,
) -> Result<HostCallCompletion<'call, HostCatalog>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let value = format!("{}{}", call.state().prefix, value);
    let updated = call.external_payload(catalog).insert(key.as_str(), value);
    let updated = call.create_external(updated);
    Ok(call.return_value(updated))
}

fn summarize<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, Summary>,
    constructions: HostConstructions<'call, SummaryConstructions>,
    catalog: HostExternal<'call, HostCatalog>,
    transform: HostCallable<'call, TransformArguments, EcoString>,
) -> Result<HostCallCompletion<'call, Summary>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let values = call
        .external_payload(catalog)
        .entries()
        .map(|(_, value)| EcoString::from(value.as_str()))
        .collect::<Vec<_>>();
    let mut items = Vec::with_capacity(values.len());
    for value in values {
        items.push(call.invoke(transform, (value, ()))?);
    }
    let count = BigInt::from(items.len());
    let items = call.construct_list(constructions.at::<HostTypeIndex0>(), items);
    Ok(call.return_custom::<SummaryConstructor>((count, (items, ()))))
}
