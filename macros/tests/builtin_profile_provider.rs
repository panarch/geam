use geam_core::provider::{Call, HostResult};
use geam_core::{
    HostComponentProfile, HostFailure, HostModule, HostProfile, HostProviderComponent,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value, ValueType,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::marker::PhantomData;

trait CounterSource: 'static {
    fn next(&mut self) -> Result<i64, HostFailure>;
}

trait CounterProfile: HostProfile + HostComponentProfile<Component<Self::Source>> {
    type Source: CounterSource;
}

struct Component<Source = ScriptedSource>(PhantomData<fn() -> Source>);

impl<Source> HostProviderComponent for Component<Source>
where
    Source: CounterSource,
{
    const ID: &'static str = "macro-builtin-profile";
    type Stores = ();
    type RunState = Source;
}

impl<Source> geam_core::__macro_support::ProviderPackage for Component<Source>
where
    Source: CounterSource,
{
    const PACKAGE: &'static str = "profile_provider";
}

#[geam_macros::module(
    path = "profile_provider",
    crate_path = geam_core,
    profile = crate::CounterProfile,
    component = crate::Component<Profile::Source>,
)]
mod profile_provider {
    use super::{BigInt, Call, CounterSource, HostResult};

    #[geam_macros::function(profile = Profile)]
    fn next(#[geam_macros::call] call: &mut Call<Profile::Source>) -> HostResult<BigInt> {
        Ok(call.state_mut().next()?.into())
    }
}

struct ScriptedSource {
    next: i64,
}

impl CounterSource for ScriptedSource {
    fn next(&mut self) -> Result<i64, HostFailure> {
        let value = self.next;
        self.next += 1;
        Ok(value)
    }
}

struct Profile;

impl HostProfile for Profile {
    type RunState = ScriptedSource;
    type ExternalStores = ();
}

impl HostComponentProfile<Component<ScriptedSource>> for Profile {
    fn component_stores(stores: &Self::ExternalStores) -> &() {
        stores
    }

    fn component_state(state: &mut Self::RunState) -> &mut ScriptedSource {
        state
    }
}

impl CounterProfile for Profile {
    type Source = ScriptedSource;
}

const SOURCE: &str = r#"
@external(erlang, "macro_builtin_profile", "next")
fn next() -> Int

pub fn main() {
  next()
}
"#;

#[test]
fn builtin_profile_functions_compile_register_and_project_caller_state() {
    let provider = profile_provider::__geam_module::<Profile>()
        .expect("built-in profile module should register");
    assert_eq!(provider.package(), "profile_provider");
    assert_eq!(provider.module(), "profile_provider");
    let functions = provider.functions().collect::<Vec<_>>();
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name(), "next");
    assert!(functions[0].type_().argument_types().is_empty());
    assert_eq!(functions[0].type_().return_(), &ValueType::Int);

    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), [provider])
        .expect("built-in profile module should be unique");
    let typed = compile_typed_host_program(
        "profile_provider",
        "profile_provider",
        [PackageSource::new(
            "profile_provider",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "profile_provider",
                "src/profile_provider.gleam",
                SOURCE,
            )],
        )],
        hosts,
    )
    .expect("built-in profile source should compile");
    let plan = plan_host_program(typed).expect("built-in profile source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("built-in profile source should seal");
    let mut source = ScriptedSource { next: 4 };

    assert_eq!(
        execution.run_main(&mut source, &mut Vec::new()),
        Ok(Value::Int(BigInt::from(4))),
    );
    assert_eq!(
        execution.run_main(&mut source, &mut Vec::new()),
        Ok(Value::Int(BigInt::from(5))),
    );
}
