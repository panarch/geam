use ecow::EcoString;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostConstructions,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
    HostExternalStore, HostExternalType, HostListType, HostProvider, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration,
    HostProviderConfiguration, HostProviderInitializationError, HostProviderModule,
    HostRegistrationError, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
};
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Component;

#[derive(Default)]
pub struct Stores {
    patterns: HostExternalStore<CompiledPattern>,
}

struct CompiledPattern {
    source: EcoString,
    regex: Regex,
}

struct Provider;
struct PatternSchema;
struct PatternStorage;
struct CompileErrorSchema;
struct CompileErrorDefinition;
struct CompileErrorMessageField;
struct ResultSchema;
struct OkDefinition;
struct OkField;
struct ErrorDefinition;
struct ErrorField;

type Pattern = HostExternalType<PatternSchema>;
type CompileError = HostCustomType<CompileErrorSchema>;
type CompileErrorConstructor =
    HostCustomConstructorAt<CompileError, HostCustomIndex0, CompileErrorDefinition>;
type PatternResultArguments = HostTypeList<Pattern, HostTypeList<CompileError, HostTypeListEnd>>;
type PatternResult = HostCustomType<ResultSchema, PatternResultArguments>;
type PatternOk = HostCustomConstructorAt<PatternResult, HostCustomIndex0, OkDefinition>;
type PatternError =
    HostCustomConstructorAt<PatternResult, HostCustomIndexNext<HostCustomIndex0>, ErrorDefinition>;
type StringList = HostListType<EcoString>;
type CompileConstructions = HostTypeList<Pattern, HostTypeList<CompileError, HostTypeListEnd>>;
type PatternConstructionIndex = HostTypeIndex0;
type CompileErrorConstructionIndex = HostTypeIndexNext<PatternConstructionIndex>;

impl HostProviderComponent for Component {
    const ID: &'static str = "geam-example-text-pattern";
    type Stores = Stores;
    type RunState = ();
}

impl HostProviderComponentInitialization for Component {
    fn initialize(
        _configuration: &HostProviderConfiguration,
    ) -> Result<Self::RunState, HostProviderInitializationError> {
        Ok(())
    }
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: HostComponentProfile<Self>,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        HostProviderModule::new("example_text_pattern", "example_text_pattern")
            .and_then(HostProviderModule::with_external_type::<Provider, PatternSchema>)
            .and_then(|provider| {
                provider.with_scoped_function_and_constructions::<
                    Provider,
                    (EcoString,),
                    PatternResult,
                    CompileConstructions,
                    _,
                >("compile", compile::<Profile>)
            })
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (Pattern, EcoString), bool, _>(
                    "is_match",
                    is_match::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_scoped_function::<Provider, (Pattern, EcoString), StringList, _>(
                    "find_all",
                    find_all::<Profile>,
                )
            })
            .and_then(|provider| {
                provider.with_scoped_function::<
                    Provider,
                    (Pattern, EcoString, EcoString),
                    EcoString,
                    _,
                >("replace_all", replace_all::<Profile>)
            })
            .map(|provider| vec![provider])
    }
}

impl<Profile> HostProvider<Profile> for Provider
where
    Profile: HostComponentProfile<Component>,
{
    type State = ();

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::component_state(state)
    }
}

impl HostExternalSchema for PatternSchema {
    const PACKAGE: &'static str = "example_text_pattern";
    const MODULE: &'static str = "example_text_pattern";
    const NAME: &'static str = "Pattern";
    const PARAMETER_COUNT: usize = 0;
}

impl<Profile> HostExternalStorage<Profile, PatternSchema> for PatternStorage
where
    Profile: HostComponentProfile<Component>,
{
    type Payload = CompiledPattern;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::component_stores(stores).patterns
    }

    fn source_equal(
        _context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.source == right.source
    }

    fn source_hash(_context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.source.hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("Pattern({:?})", value.source).into()
    }
}

impl<Profile> HostExternalBinding<Profile, PatternSchema> for Provider
where
    Profile: HostComponentProfile<Component>,
{
    type Storage = PatternStorage;
}

impl HostCustomField for CompileErrorMessageField {
    const LABEL: Option<&'static str> = Some("message");
    type Type = EcoString;
}

impl HostCustomConstructorDefinition for CompileErrorDefinition {
    const NAME: &'static str = "CompileError";
    type Fields = HostCustomFieldList<CompileErrorMessageField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for CompileErrorSchema {
    const PACKAGE: &'static str = "example_text_pattern";
    const MODULE: &'static str = "example_text_pattern";
    const NAME: &'static str = "CompileError";
    const PARAMETER_COUNT: usize = 0;
    type Constructors =
        HostCustomConstructorList<CompileErrorDefinition, HostCustomConstructorListEnd>;
}

impl HostCustomField for OkField {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for OkDefinition {
    const NAME: &'static str = "Ok";
    type Fields = HostCustomFieldList<OkField, HostCustomFieldListEnd>;
}

impl HostCustomField for ErrorField {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for ErrorDefinition {
    const NAME: &'static str = "Error";
    type Fields = HostCustomFieldList<ErrorField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for ResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;
    type Constructors = HostCustomConstructorList<
        OkDefinition,
        HostCustomConstructorList<ErrorDefinition, HostCustomConstructorListEnd>,
    >;
}

fn compile<'call, Profile>(
    mut call: HostCall<'call, Profile, Provider, PatternResult>,
    constructions: HostConstructions<'call, CompileConstructions>,
    source: EcoString,
) -> Result<HostCallCompletion<'call, PatternResult>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    match Regex::new(source.as_str()) {
        Ok(regex) => {
            let pattern = call.construct_external(
                constructions.at::<PatternConstructionIndex>(),
                CompiledPattern { source, regex },
            );
            Ok(call.return_custom::<PatternOk>((pattern, ())))
        }
        Err(error) => {
            let error = call.construct_custom::<CompileErrorConstructor>(
                constructions.at::<CompileErrorConstructionIndex>(),
                (EcoString::from(error.to_string()), ()),
            );
            Ok(call.return_custom::<PatternError>((error, ())))
        }
    }
}

fn is_match<'call, Profile>(
    call: HostCall<'call, Profile, Provider, bool>,
    pattern: HostExternal<'call, Pattern>,
    text: EcoString,
) -> Result<HostCallCompletion<'call, bool>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let matches = call.external_payload(pattern).regex.is_match(text.as_str());
    Ok(call.return_value(matches))
}

fn find_all<'call, Profile>(
    call: HostCall<'call, Profile, Provider, StringList>,
    pattern: HostExternal<'call, Pattern>,
    text: EcoString,
) -> Result<HostCallCompletion<'call, StringList>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let matches = call
        .external_payload(pattern)
        .regex
        .find_iter(text.as_str())
        .map(|matched| EcoString::from(matched.as_str()))
        .collect::<Vec<_>>();
    Ok(call.return_list(matches))
}

fn replace_all<'call, Profile>(
    call: HostCall<'call, Profile, Provider, EcoString>,
    pattern: HostExternal<'call, Pattern>,
    text: EcoString,
    replacement: EcoString,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: HostComponentProfile<Component>,
{
    let replaced = call
        .external_payload(pattern)
        .regex
        .replace_all(text.as_str(), replacement.as_str());
    Ok(call.return_value(EcoString::from(replaced.as_ref())))
}
