use geam_core::embedding::{FunctionDeclaration, HostPreparation, PreparedHostedModule};
use geam_core::provider_support::HostOpaqueFunctionType;
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
    HostCustomFieldListEnd, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
    HostFunctionType, HostListType, HostProvider, HostProviderModule, HostProviderSet, HostType,
    HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter, ModuleSource, PackageSource,
    StatelessHostProfile, StringValue,
};

struct Provider;
impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

struct HolderSchema;
struct Held;
struct FunctionField;
impl HostCustomSchema for HolderSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "opaque_functions";
    const NAME: &'static str = "Holder";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<Held, HostCustomConstructorListEnd>;
}
impl HostCustomConstructorDefinition for Held {
    const NAME: &'static str = "Held";
    type Fields = HostCustomFieldList<FunctionField, HostCustomFieldListEnd>;
}
impl HostCustomField for FunctionField {
    const LABEL: Option<&'static str> = None;
    type Type = HostOpaqueFunctionType<
        HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        StringValue,
    >;
}

struct CallableHolderSchema;
struct CallableHeld;
struct CallableField;
impl HostCustomSchema for CallableHolderSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "opaque_functions";
    const NAME: &'static str = "Holder";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<CallableHeld, HostCustomConstructorListEnd>;
}
impl HostCustomConstructorDefinition for CallableHeld {
    const NAME: &'static str = "Held";
    type Fields = HostCustomFieldList<CallableField, HostCustomFieldListEnd>;
}
impl HostCustomField for CallableField {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<
        HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        StringValue,
    >;
}

struct CompoundSchema;
struct CompoundHeld;
struct CompoundField;
impl HostCustomSchema for CompoundSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "opaque_functions";
    const NAME: &'static str = "CompoundHolder";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<CompoundHeld, HostCustomConstructorListEnd>;
}
impl HostCustomConstructorDefinition for CompoundHeld {
    const NAME: &'static str = "CompoundHeld";
    type Fields = HostCustomFieldList<CompoundField, HostCustomFieldListEnd>;
}
impl HostCustomField for CompoundField {
    const LABEL: Option<&'static str> = None;
    type Type = HostOpaqueFunctionType<
        HostTypeList<HostListType<HostCustomTypeArgument<HostTypeIndex0>>, HostTypeListEnd>,
        HostListType<HostCustomTypeArgument<HostTypeIndex0>>,
    >;
}

type Holder = HostCustomType<HolderSchema, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;
type CallableHolder =
    HostCustomType<CallableHolderSchema, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;
type CompoundHolder =
    HostCustomType<CompoundSchema, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;

fn keep<'call, Type: HostType>(
    call: HostCall<'call, StatelessHostProfile, Provider, Type>,
    value: Type::Value<'call>,
) -> Result<HostCallCompletion<'call, Type>, HostCallError> {
    Ok(call.return_value(value))
}

pub fn hosts(invocable: bool) -> HostProviderSet {
    let module = HostProviderModule::new("application", "opaque_functions").unwrap();
    let module = if invocable {
        module
            .with_scoped_function::<Provider, (CallableHolder,), CallableHolder, _>(
                "keep",
                keep::<CallableHolder>,
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (Holder,), Holder, _>("keep", keep::<Holder>)
            .unwrap()
    };
    let module = module
        .with_scoped_function::<Provider, (CompoundHolder,), CompoundHolder, _>(
            "keep_compound",
            keep::<CompoundHolder>,
        )
        .unwrap();
    HostProviderSet::from_providers([module]).unwrap()
}

pub fn packages() -> Vec<PackageSource> {
    vec![PackageSource::new(
        "application",
        Vec::<&str>::new(),
        [ModuleSource::new(
            "opaque_functions",
            "src/opaque_functions.gleam",
            include_str!("opaque_functions.gleam"),
        )],
    )]
}

pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "application",
        "opaque_functions",
        packages(),
        hosts(false).into_declarations(),
    )
    .unwrap();
    let mut bindings = HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("main"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), (bool, StringValue)>::new(
            "concrete",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), (bool, bool)>::new("compound"))
        .unwrap();
    bindings.prepare().unwrap()
}
