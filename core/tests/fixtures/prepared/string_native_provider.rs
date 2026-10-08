use geam_core::embedding::{
    FunctionDeclaration, HostPreparation, PreparedHostedModule, StringValue,
};
use geam_core::execution::{ExecutionUnit, ExitStatus};
use geam_core::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostConstructions,
    HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType, HostFailure,
    HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
    HostTypeListEnd, ModuleSource, PackageSource,
};
use num_bigint::BigInt;
use std::convert::Infallible;

pub struct Profile;

impl HostProfile for Profile {
    type RunState = Audit;
    type ExternalStores = ();
    type ExecutionState = ();
}

impl HostProvider<Profile> for Profile {
    type State = Audit;
    fn project(state: &mut Audit) -> &mut Audit {
        state
    }
}

#[derive(Default)]
pub struct Audit {
    pub events: Vec<(StringValue, bool, BigInt)>,
    pub continued: usize,
    pub stopped: usize,
    pub fail_at: Option<usize>,
    pub cancel_at: Option<usize>,
    pub exit_at: Option<usize>,
    pub unit: Option<ExecutionUnit>,
}

struct DirectionSchema;
struct Before;
struct After;

impl HostCustomSchema for DirectionSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "string_native";
    const NAME: &'static str = "Direction";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        Before,
        HostCustomConstructorList<After, HostCustomConstructorListEnd>,
    >;
}

impl HostCustomConstructorDefinition for Before {
    const NAME: &'static str = "Before";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomConstructorDefinition for After {
    const NAME: &'static str = "After";
    type Fields = HostCustomFieldListEnd;
}

type Direction = HostCustomType<DirectionSchema, HostTypeListEnd>;
type BeforeConstructor = HostCustomConstructorAt<Direction, HostCustomIndex0, Before>;
type AfterConstructor =
    HostCustomConstructorAt<Direction, HostCustomIndexNext<HostCustomIndex0>, After>;

fn observe<'call>(
    call: &mut HostCall<'call, Profile, Profile, StringValue>,
    value: StringValue,
    direction: HostCustom<'call, Direction>,
    index: BigInt,
) -> Result<StringValue, HostCallError> {
    let before = call.custom_fields::<BeforeConstructor>(direction).is_some();
    assert!(before || call.custom_fields::<AfterConstructor>(direction).is_some());
    let unit = call.execution_unit();
    let state = call.state();
    state.events.push((value.clone(), before, index.clone()));
    state.unit = unit;
    let count = state.events.len();
    if state.cancel_at == Some(count) {
        state.unit.as_ref().unwrap().cancel();
    }
    if state.fail_at == Some(count) {
        return Err(HostFailure::new("string native failure").into());
    }
    if state.exit_at == Some(count) {
        return call.exit(ExitStatus::try_from(&BigInt::from(7)).unwrap());
    }
    let suffix = format!("{}{}", if before { "<" } else { ">" }, index);
    Ok(format!("{}{suffix}", value.as_str().unwrap()).into())
}

fn join<'call>(
    mut call: HostCall<'call, Profile, Profile, StringValue>,
    value: StringValue,
    direction: HostCustom<'call, Direction>,
    index: BigInt,
) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
    let result = observe(&mut call, value, direction, index)?;
    Ok(call.return_value(result))
}

fn continuing_join<'call>(
    mut call: HostCall<'call, Profile, Profile, StringValue>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    value: StringValue,
    direction: HostCustom<'call, Direction>,
    index: BigInt,
) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
    let result = observe(&mut call, value, direction, index)?;
    Ok(call.resume(constructions, move |_| {
        Box::pin(async move {
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(result))
            }))
        })
    }))
}

fn later<'call>(
    mut call: HostCall<'call, Profile, Profile, StringValue>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    value: StringValue,
) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
    call.state().continued += 1;
    Ok(call.resume(constructions, move |_| {
        Box::pin(async move {
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn stop<'call>(
    mut call: HostCall<'call, Profile, Profile, StringValue>,
    _value: StringValue,
) -> Result<Infallible, HostCallError> {
    call.state().stopped += 1;
    call.exit(ExitStatus::try_from(&BigInt::from(11)).unwrap())
}

pub fn hosts(continuing: bool) -> HostProviderSet<Profile> {
    let module = HostProviderModule::new("application", "string_native").unwrap();
    let module = if continuing {
        module.with_resumable_function::<Profile, (StringValue, Direction, BigInt), StringValue, HostTypeListEnd, _>("join", continuing_join).unwrap()
    } else {
        module
            .with_scoped_function::<Profile, (StringValue, Direction, BigInt), StringValue, _>(
                "join", join,
            )
            .unwrap()
    };
    HostProviderSet::from_providers([module
        .with_resumable_function::<Profile, (StringValue,), StringValue, HostTypeListEnd, _>(
            "later", later,
        )
        .unwrap()
        .with_scoped_diverging_function::<Profile, (StringValue,), StringValue, _>("stop", stop)
        .unwrap()])
    .unwrap()
}

pub fn packages() -> Vec<PackageSource> {
    vec![PackageSource::new(
        "application",
        Vec::<String>::new(),
        [ModuleSource::new(
            "string_native",
            "src/string_native.gleam",
            include_str!("string_native_calls.gleam"),
        )],
    )]
}

pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "application",
        "string_native",
        packages(),
        hosts(false).into_declarations(),
    )
    .unwrap();
    let mut bindings = HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "ordinary",
        ))
        .unwrap();
    for name in ["tail", "nested_tail", "after_continuing", "after_never"] {
        bindings
            .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                name,
            ))
            .unwrap();
    }
    bindings
        .function(FunctionDeclaration::<
            (StringValue,),
            (StringValue, StringValue),
        >::new("source_caller"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "source_string_caller",
        ))
        .unwrap();
    bindings.prepare().unwrap()
}

#[cfg(test)]
pub fn source(continuing: bool) -> geam_core::embedding::HostedModuleBuilder<Profile> {
    geam_core::embedding::HostedModuleBuilder::new(
        geam_core::compile_typed_host_program(
            "application",
            "string_native",
            packages(),
            hosts(continuing),
        )
        .unwrap(),
    )
    .unwrap()
}
