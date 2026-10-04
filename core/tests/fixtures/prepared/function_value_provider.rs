use geam_core::embedding::{FunctionDeclaration, HostPreparation, PreparedHostedModule};
use geam_core::provider_support::HostOpaqueFunctionType;
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostFunctionType, HostFunctionValue, HostFunctionValueType,
    HostListType, HostProvider, HostProviderModule, HostProviderSet, HostType, HostTypeIndex0,
    HostTypeList, HostTypeListEnd, HostTypeParameter, HostTypeSequence, ModuleSource,
    PackageSource, StatelessHostProfile, StringValue,
};
use std::marker::PhantomData;

struct Provider;
impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

struct HolderSchema<Field>(PhantomData<Field>);
struct Held<Field>(PhantomData<Field>);
struct GeneralField;
struct OpaqueField;
struct StrictField;
impl<Field: HostCustomField> HostCustomSchema for HolderSchema<Field> {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "function_values";
    const NAME: &'static str = "Holder";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<Held<Field>, HostCustomConstructorListEnd>;
}
impl<Field: HostCustomField> HostCustomConstructorDefinition for Held<Field> {
    const NAME: &'static str = "Held";
    type Fields = HostCustomFieldList<Field, HostCustomFieldListEnd>;
}
impl HostCustomField for GeneralField {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionValueType<
        HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        StringValue,
    >;
}
impl HostCustomField for OpaqueField {
    const LABEL: Option<&'static str> = None;
    type Type = HostOpaqueFunctionType<
        HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        StringValue,
    >;
}
impl HostCustomField for StrictField {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<
        HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        StringValue,
    >;
}

type One<T> = HostTypeList<T, HostTypeListEnd>;
type Function = HostFunctionValueType<One<HostTypeParameter<0>>, StringValue>;
type Holder<Field> = HostCustomType<HolderSchema<Field>, One<HostTypeParameter<0>>>;
type Compound = HostFunctionValueType<
    One<HostListType<HostTypeParameter<0>>>,
    HostListType<HostTypeParameter<0>>,
>;

fn keep<'call, Arguments: HostTypeSequence, Return: HostType>(
    mut call: HostCall<
        'call,
        StatelessHostProfile,
        Provider,
        HostFunctionValueType<Arguments, Return>,
    >,
    proof: HostConstructions<'call, HostTypeListEnd>,
    function: HostFunctionValue<'call, Arguments, Return>,
) -> Result<HostCallCompletion<'call, HostFunctionValueType<Arguments, Return>>, HostCallError> {
    let retained = call.owned_function_value(function, &proof);
    let restored = retained.clone().restore(&mut call)?;
    Ok(call.return_value(restored))
}
fn has_callback<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, bool>,
    proof: HostConstructions<'call, HostTypeListEnd>,
    function: HostFunctionValue<'call, One<HostTypeParameter<0>>, StringValue>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let retained = call.owned_function_value(function, &proof);
    Ok(call.return_value(retained.callable().is_some()))
}
fn keep_holder<'call, Type: HostType>(
    call: HostCall<'call, StatelessHostProfile, Provider, Type>,
    value: Type::Value<'call>,
) -> Result<HostCallCompletion<'call, Type>, HostCallError> {
    Ok(call.return_value(value))
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum FieldRole {
    General,
    Opaque,
    Strict,
}

pub fn hosts(role: FieldRole) -> HostProviderSet {
    let module = HostProviderModule::new("application", "function_values").unwrap()
        .with_scoped_function_and_constructions::<Provider, (Function,), Function, HostTypeListEnd, _>(
            "keep",
            keep::<One<HostTypeParameter<0>>, StringValue>,
        )
        .unwrap()
        .with_scoped_function_and_constructions::<Provider, (Function,), bool, HostTypeListEnd, _>(
            "has_callback",
            has_callback,
        )
        .unwrap()
        .with_scoped_function_and_constructions::<Provider, (Compound,), Compound, HostTypeListEnd, _>(
            "keep_compound",
            keep::<One<HostListType<HostTypeParameter<0>>>, HostListType<HostTypeParameter<0>>>,
        )
        .unwrap();
    let module = match role {
        FieldRole::General => module
            .with_scoped_function::<Provider, (Holder<GeneralField>,), Holder<GeneralField>, _>(
                "keep_holder",
                keep_holder::<Holder<GeneralField>>,
            )
            .unwrap(),
        FieldRole::Opaque => module
            .with_scoped_function::<Provider, (Holder<OpaqueField>,), Holder<OpaqueField>, _>(
                "keep_holder",
                keep_holder::<Holder<OpaqueField>>,
            )
            .unwrap(),
        FieldRole::Strict => module
            .with_scoped_function::<Provider, (Holder<StrictField>,), Holder<StrictField>, _>(
                "keep_holder",
                keep_holder::<Holder<StrictField>>,
            )
            .unwrap(),
    };
    HostProviderSet::from_providers([module]).unwrap()
}

pub fn packages() -> Vec<PackageSource> {
    vec![PackageSource::new(
        "application",
        Vec::<&str>::new(),
        [ModuleSource::new(
            "function_values",
            "src/function_values.gleam",
            include_str!("function_values.gleam"),
        )],
    )]
}

pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "application",
        "function_values",
        packages(),
        hosts(FieldRole::General).into_declarations(),
    )
    .unwrap();
    let mut bindings = HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("main"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), (bool, bool, StringValue)>::new(
            "concrete",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), (bool, bool)>::new("compound"))
        .unwrap();
    bindings.prepare().unwrap()
}
