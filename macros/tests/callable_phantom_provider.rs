#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};

#[geam_macros::provider(package = "phantom_provider", modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "phantom_provider", crate_path = geam_core)]
mod native {
    use geam_core::provider::{Call, Callback, Factory, HostResult, Value};

    #[geam_macros::custom(input = SwitchInput)]
    enum Switch<Item> {
        Enabled,
        Disabled,
    }

    #[geam_macros::callable(factory = Choose)]
    fn choose<Item>(
        #[geam_macros::capture] switch: SwitchInput<Item>,
        value: Value<Item>,
    ) -> Result<Value<Item>, bool> {
        match switch {
            SwitchInput::Enabled => Ok(value),
            SwitchInput::Disabled => Err(false),
        }
    }

    #[geam_macros::function]
    fn make<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<Choose<Item>>,
        switch: SwitchInput<Item>,
    ) -> HostResult<Callback<fn(Value<Item>) -> Result<Value<Item>, bool>>> {
        let switch = match switch {
            SwitchInput::Enabled => Switch::Enabled,
            SwitchInput::Disabled => Switch::Disabled,
        };
        call.create(&factory, (switch,))
    }

    #[geam_macros::function]
    fn enabled<Item>() -> Switch<Item> {
        Switch::Enabled
    }

    #[geam_macros::function(await)]
    async fn disabled<Item>() -> Switch<Item> {
        Switch::Disabled
    }

    #[geam_macros::function]
    fn enabled_at<Item>(switches: geam_core::List<SwitchInput<Item>>) -> bool {
        matches!(switches.get(1), Some(SwitchInput::Enabled))
    }
}

struct Profile;
impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = <Component as HostProviderComponent>::Stores;
    type ExecutionState = ();
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn fieldless_phantom_declarations_keep_exhaustive_matches_and_exact_native_specializations() {
    let source = r#"
pub type Switch(item) { Enabled Disabled }
@external(erlang, "native", "make")
fn make(switch: Switch(item)) -> fn(item) -> Result(item, Bool)
@external(erlang, "native", "enabled")
fn enabled() -> Switch(item)
@external(erlang, "native", "disabled")
fn disabled() -> Switch(item)
@external(erlang, "native", "enabled_at")
fn enabled_at(switches: List(Switch(item))) -> Bool
pub fn main() {
  let choose_int: fn(Int) -> Result(Int, Bool) = make(enabled())
  let choose_string: fn(String) -> Result(String, Bool) = make(Enabled)
  let reject: fn(fn(Int) -> Int) -> Result(fn(Int) -> Int, Bool) = make(disabled())
  let switches: List(Switch(String)) = [Disabled, Enabled]
  assert choose_int(7) == Ok(7)
  assert choose_string("kept") == Ok("kept")
  assert reject(fn(value) { value + 1 }) == Error(False)
  assert enabled_at(switches)
  assert !enabled_at([])
  assert !enabled_at([Enabled, Disabled])
  True
}
"#;
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "phantom_provider",
        "phantom_provider",
        [PackageSource::new(
            "phantom_provider",
            Vec::<String>::new(),
            [ModuleSource::new(
                "phantom_provider",
                "phantom.gleam",
                source,
            )],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    assert_eq!(
        execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
        Ok(Value::Bool(true))
    );
}
