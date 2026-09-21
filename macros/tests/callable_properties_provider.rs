#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};

#[geam_macros::provider(package = "property_provider", modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "property_provider", crate_path = geam_core)]
mod native {
    use geam_core::provider::{BigInt, Call, Callback, Factory, HostResult, StringValue, Value};

    #[geam_macros::custom(input = StartedInput)]
    enum Started<Data> {
        Started(Value<Data>),
        Waiting,
    }

    #[geam_macros::custom(input = StartErrorInput)]
    enum StartError {
        BadChild,
        NotReady(StringValue),
    }

    #[geam_macros::custom(input = PropertyInput)]
    enum Property<Argument, Data> {
        Weight(BigInt),
        Start(Callback<fn(Value<Argument>) -> Result<StartedInput<Data>, StartErrorInput>>),
    }

    #[geam_macros::callable(factory = MapData, await)]
    async fn map_data<Old, New>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::capture] start: Callback<fn() -> Result<StartedInput<Old>, StartErrorInput>>,
        #[geam_macros::capture] mapper: Callback<fn(Value<Old>) -> Value<New>>,
    ) -> HostResult<Result<Started<New>, StartError>> {
        Ok(match call.invoke(&start, ()).await? {
            Ok(StartedInput::Started(value)) => {
                Ok(Started::Started(call.invoke(&mapper, (value,)).await?))
            }
            Ok(StartedInput::Waiting) => Ok(Started::Waiting),
            Err(StartErrorInput::BadChild) => Err(StartError::BadChild),
            Err(StartErrorInput::NotReady(message)) => Err(StartError::NotReady(message)),
        })
    }

    #[geam_macros::function]
    fn mapped<Old, New>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<MapData<Old, New>>,
        start: Callback<fn() -> Result<StartedInput<Old>, StartErrorInput>>,
        mapper: Callback<fn(Value<Old>) -> Value<New>>,
    ) -> HostResult<Callback<fn() -> Result<StartedInput<New>, StartErrorInput>>> {
        call.create(&factory, (start, mapper))
    }

    #[geam_macros::callable(factory = StartChild, await)]
    async fn start_child<Argument, Data>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::capture] properties: geam_core::List<PropertyInput<Argument, Data>>,
        value: Value<Argument>,
    ) -> HostResult<Result<Started<Data>, StartError>> {
        let selected = properties.get(1);
        drop(properties);
        let Some(PropertyInput::Start(callback)) = selected else {
            return Ok(Err(StartError::BadChild));
        };
        Ok(match call.invoke(&callback, (value,)).await? {
            Ok(StartedInput::Started(value)) => Ok(Started::Started(value)),
            Ok(StartedInput::Waiting) => Ok(Started::Waiting),
            Err(StartErrorInput::BadChild) => Err(StartError::BadChild),
            Err(StartErrorInput::NotReady(message)) => Err(StartError::NotReady(message)),
        })
    }

    #[geam_macros::function]
    fn make<Argument, Data>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<StartChild<Argument, Data>>,
        properties: geam_core::List<PropertyInput<Argument, Data>>,
    ) -> HostResult<Callback<fn(Value<Argument>) -> Result<StartedInput<Data>, StartErrorInput>>>
    {
        call.create(&factory, (properties,))
    }

    #[geam_macros::function]
    fn start_properties<Argument, Data>(
        weight: BigInt,
        callback: Callback<fn(Value<Argument>) -> Result<StartedInput<Data>, StartErrorInput>>,
    ) -> Vec<Property<Argument, Data>> {
        vec![Property::Weight(weight), Property::Start(callback)]
    }

    #[geam_macros::function]
    fn spec<Argument, Data>(
        #[geam_macros::call] call: &mut Call<()>,
        #[geam_macros::factory] factory: Factory<StartChild<Argument, Data>>,
        properties: geam_core::List<PropertyInput<Argument, Data>>,
    ) -> HostResult<(
        StringValue,
        StringValue,
        Vec<Callback<fn(Value<Argument>) -> Result<StartedInput<Data>, StartErrorInput>>>,
    )> {
        let callback = call.create(&factory, (properties,))?;
        Ok((
            "child_provider".into(),
            "start".into(),
            vec![callback.clone(), callback],
        ))
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
fn nominal_property_lists_retain_independent_start_arguments_results_and_errors() {
    let source = r#"
pub type Started(data) { Started(data) Waiting }
pub type StartError { BadChild NotReady(String) }
pub type Property(argument, data) {
  Weight(Int)
  Start(fn(argument) -> Result(Started(data), StartError))
}
@external(erlang, "native", "make")
fn make(properties: List(Property(argument, data))) -> fn(argument) -> Result(Started(data), StartError)
@external(erlang, "native", "start_properties")
fn start_properties(weight: Int, callback: fn(argument) -> Result(Started(data), StartError)) -> List(Property(argument, data))
@external(erlang, "native", "spec")
fn spec(properties: List(Property(argument, data))) -> #(String, String, List(fn(argument) -> Result(Started(data), StartError)))
@external(erlang, "native", "mapped")
fn mapped(start: fn() -> Result(Started(old), StartError), mapper: fn(old) -> new) -> fn() -> Result(Started(new), StartError)
pub fn main() {
  let start = make([Weight(3), Start(fn(value: Int) {
    case value {
      0 -> Ok(Waiting)
      1 -> Error(BadChild)
      2 -> Error(NotReady("busy"))
      _ -> { echo value Ok(Started(#("payload", value))) }
    }
  }), Weight(99)])
  let alias = start
  let another = make([Weight(4), Start(fn(value: String) { Ok(Started(value == "ready")) })])
  assert start == alias
  assert start(0) == Ok(Waiting)
  assert start(1) == Error(BadChild)
  assert start(2) == Error(NotReady("busy"))
  assert start(7) == Ok(Started(#("payload", 7)))
  assert alias(8) == Ok(Started(#("payload", 8)))
  assert another("ready") == Ok(Started(True))
  let reject_empty: fn(Int) -> Result(Started(Int), StartError) = make([])
  let reject_weight: fn(Int) -> Result(Started(Int), StartError) = make([Weight(1), Weight(2)])
  assert reject_empty(0) == Error(BadChild)
  assert reject_weight(0) == Error(BadChild)
  let assert #(module, function, [first, second]) = spec(start_properties(5, fn(value: Int) {
    echo value
    Ok(Started(#("native-list", value)))
  }))
  assert module == "child_provider"
  assert function == "start"
  assert first == second
  assert first(9) == Ok(Started(#("native-list", 9)))
  let mapper = fn(value: Int) { echo value #("mapped", value + 1) }
  let success = mapped(fn() { Ok(Started(10)) }, mapper)
  let success_alias = success
  assert success == success_alias
  assert success() == Ok(Started(#("mapped", 11)))
  assert success_alias() == Ok(Started(#("mapped", 11)))
  assert mapped(fn() { Ok(Waiting) }, mapper)() == Ok(Waiting)
  assert mapped(fn() { Error(BadChild) }, mapper)() == Error(BadChild)
  assert mapped(fn() { Error(NotReady("restart")) }, mapper)() == Error(NotReady("restart"))
  assert mapped(fn() { Ok(Started("ready")) }, fn(value) { value == "ready" })() == Ok(Started(True))
  True
}
"#;
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "property_provider",
        "property_provider",
        [PackageSource::new(
            "property_provider",
            Vec::<String>::new(),
            [ModuleSource::new(
                "property_provider",
                "properties.gleam",
                source,
            )],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let mut echoes = Vec::new();
    assert_eq!(
        execution_fixture::run(
            &mut execution,
            &mut (),
            &mut |output: geam_core::EchoOutput| {
                echoes.push(output.value().inspect().to_string());
            }
        ),
        Ok(Value::Bool(true))
    );
    assert_eq!(echoes, ["7", "8", "9", "10", "10"]);
}
