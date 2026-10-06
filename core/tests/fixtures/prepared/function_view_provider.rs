use geam_core::embedding::{
    BigInt, FunctionDeclaration, HostedModuleBuilder, PreparedHostedModule,
};
use geam_core::host::native::{NativeCall, NativeRules};
use geam_core::provider::advanced::NativeValue;
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
    HostExternalStore, HostFailure, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
    HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue, ModuleSource,
    PackageSource, StringValue,
};

pub struct Profile;
struct Provider;
struct Erased;
struct Storage;
type Source = HostTypeParameter<1>;
type Return = HostTypeParameter<0>;
type One<Type> = HostTypeList<Type, HostTypeListEnd>;

impl HostProfile for Profile {
    type RunState = Vec<StringValue>;
    type ExternalStores = HostExternalStore<NativeValue>;
    type ExecutionState = ();
}

impl HostProvider<Profile> for Provider {
    type State = Vec<StringValue>;
    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

impl HostExternalSchema for Erased {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "function_views";
    const NAME: &'static str = "Erased";
    const PARAMETER_COUNT: usize = 0;
}

impl HostExternalBinding<Profile, Erased> for Provider {
    type Storage = Storage;
}
impl HostExternalStorage<Profile, Erased> for Storage {
    type Payload = NativeValue;
    fn store(stores: &HostExternalStore<NativeValue>) -> &HostExternalStore<NativeValue> {
        stores
    }
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &NativeValue,
        right: &NativeValue,
    ) -> bool {
        left.source_equal(context, right)
    }
    fn source_hash(context: &HostExternalHashing<'_>, value: &NativeValue) -> u64 {
        value.source_hash(context)
    }
    fn inspect(context: &HostExternalInspection<'_>, value: &NativeValue) -> ecow::EcoString {
        value.inspect(context)
    }
    fn native_view(value: &NativeValue) -> Option<NativeValue> {
        Some(value.clone())
    }
}

fn coerce<'call>(
    mut call: NativeCall<'call, Profile, Provider, Return, One<Return>>,
    source: HostValue<'call, Source>,
) -> Result<HostCallCompletion<'call, Return>, HostCallError> {
    let source = call.source::<Source>(source);
    let value = call
        .convert::<HostTypeIndex0>(&source)
        .ok_or_else(|| HostFailure::new("incompatible retained view"))?;
    Ok(call.finish(value))
}

fn tick<'call>(
    mut call: HostCall<'call, Profile, Provider, BigInt>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    call.state().push("source".into());
    Ok(call.return_value(42.into()))
}

pub fn hosts() -> HostProviderSet<Profile> {
    let provider = HostProviderModule::new("application", "function_views")
        .unwrap()
        .with_external_type::<Provider, Erased>()
        .unwrap()
        .with_native_function::<Provider, (Source,), Return, One<Return>, _>(
            "coerce",
            NativeRules::default()
                .retained_views::<One<Source>>()
                .external::<Erased, HostTypeListEnd>(|call, token, value| {
                    Some(call.construct_external(token, value))
                }),
            coerce,
        )
        .unwrap()
        .with_scoped_function::<Provider, (), BigInt, _>("tick", tick)
        .unwrap();
    HostProviderSet::from_providers([provider]).unwrap()
}

pub fn prepare() -> PreparedHostedModule {
    let program = geam_core::compile_typed_host_program(
        "application",
        "function_views",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "function_views",
                "src/function_views.gleam",
                include_str!("function_views.gleam"),
            )],
        )],
        hosts(),
    )
    .unwrap();
    let (mut bindings, _) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("run"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), bool>::new("invalid_input"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), bool>::new("stopped"))
        .unwrap();
    bindings.prepare().unwrap()
}
