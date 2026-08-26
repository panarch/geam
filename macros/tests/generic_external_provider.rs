use ecow::EcoString;
use geam_core::provider::{Call, Callback, HostResult, Stored, Value};
use geam_core::{
    HostComponentProfile, HostExternalTypeSchema, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, PlanError, Value as RuntimeValue, ValueType, compile_typed_host_program,
    plan_host_program,
};

#[geam_macros::provider(
    package = "generic_box",
    modules = [generic_box],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "generic_box", crate_path = geam_core)]
mod generic_box {
    use super::{Call, Callback, EcoString, HostResult, Stored, Value};

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    struct Token(EcoString);

    impl Clone for Token {
        fn clone(&self) -> Self {
            panic!("retaining a generic external value must not clone its payload")
        }
    }

    #[geam_macros::external(
        name = "Box",
        parameters = [Item],
        input = BoxInput,
    )]
    pub struct BoxValue<Item> {
        #[geam_macros::stored]
        value: Stored<Item>,
    }

    #[geam_macros::external(
        name = "Pair",
        parameters = [Left, Right],
        input = PairInput,
    )]
    pub struct Pair<Left, Right> {
        #[geam_macros::stored]
        left: Stored<Left>,
        #[geam_macros::stored]
        right: Stored<Right>,
    }

    #[geam_macros::function]
    fn token(value: EcoString) -> Token {
        Token(value)
    }

    #[geam_macros::function]
    fn new<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> BoxValue<Item> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    fn get<Item>(#[geam_macros::call] call: &mut Call<()>, boxed: BoxInput<Item>) -> Value<Item> {
        call.restore(boxed.value())
    }

    #[geam_macros::function]
    fn replace<Old, New>(
        #[geam_macros::call] call: &mut Call<()>,
        _boxed: BoxInput<Old>,
        value: Value<New>,
    ) -> BoxValue<New> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    fn contains<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        boxed: BoxInput<Item>,
        expected: Value<Item>,
    ) -> bool {
        let value = call.restore(boxed.value());
        call.equal(&value, &expected)
    }

    #[geam_macros::function]
    fn map<Input, Output>(
        #[geam_macros::call] call: &mut Call<()>,
        boxed: BoxInput<Input>,
        mapper: Callback<fn(Value<Input>) -> Value<Output>>,
    ) -> HostResult<BoxValue<Output>> {
        let value = call.restore(boxed.value());
        let mapped = call.invoke(mapper, (value,))?;
        Ok(BoxValue {
            value: call.store(mapped),
        })
    }

    #[geam_macros::function]
    fn with_box<Input, Output>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Input>,
        callback: Callback<fn(BoxValue<Input>) -> Value<Output>>,
    ) -> HostResult<Value<Output>> {
        let boxed = BoxValue {
            value: call.store(value),
        };
        call.invoke(callback, (boxed,))
    }

    #[geam_macros::function]
    fn from_box_callback<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        callback: Callback<fn() -> BoxInput<Item>>,
    ) -> HostResult<Value<Item>> {
        let boxed = call.invoke(callback, ())?;
        Ok(call.restore(boxed.value()))
    }

    #[geam_macros::function]
    fn pair<Left, Right>(
        #[geam_macros::call] call: &mut Call<()>,
        left: Value<Left>,
        right: Value<Right>,
    ) -> Pair<Left, Right> {
        Pair {
            left: call.store(left),
            right: call.store(right),
        }
    }

    #[geam_macros::function]
    fn first<Left, Right>(
        #[geam_macros::call] call: &mut Call<()>,
        pair: PairInput<Left, Right>,
    ) -> Value<Left> {
        call.restore(pair.left())
    }

    #[geam_macros::function]
    fn second<Left, Right>(
        #[geam_macros::call] call: &mut Call<()>,
        pair: PairInput<Left, Right>,
    ) -> Value<Right> {
        call.restore(pair.right())
    }

    #[geam_macros::function]
    fn swap<Left, Right>(
        #[geam_macros::call] call: &mut Call<()>,
        pair: PairInput<Left, Right>,
    ) -> Pair<Right, Left> {
        let left = call.restore(pair.left());
        let right = call.restore(pair.right());
        Pair {
            left: call.store(right),
            right: call.store(left),
        }
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const SOURCE: &str = r#"
@external(erlang, "generic_box", "Token")
pub type Token

@external(erlang, "generic_box", "token")
fn token(value: String) -> Token

@external(erlang, "generic_box", "Box")
pub type Box(item)

@external(erlang, "generic_box", "new")
fn new(value: item) -> Box(item)

@external(erlang, "generic_box", "get")
fn get(boxed: Box(item)) -> item

@external(erlang, "generic_box", "replace")
fn replace(boxed: Box(old), value: new) -> Box(new)

@external(erlang, "generic_box", "contains")
fn contains(boxed: Box(item), expected: item) -> Bool

@external(erlang, "generic_box", "map")
fn map(boxed: Box(input), mapper: fn(input) -> output) -> Box(output)

@external(erlang, "generic_box", "with_box")
fn with_box(value: input, callback: fn(Box(input)) -> output) -> output

@external(erlang, "generic_box", "from_box_callback")
fn from_box_callback(callback: fn() -> Box(item)) -> item

@external(erlang, "generic_box", "Pair")
pub type Pair(left, right)

@external(erlang, "generic_box", "pair")
fn pair(left: left, right: right) -> Pair(left, right)

@external(erlang, "generic_box", "first")
fn first(pair: Pair(left, right)) -> left

@external(erlang, "generic_box", "second")
fn second(pair: Pair(left, right)) -> right

@external(erlang, "generic_box", "swap")
fn swap(pair: Pair(left, right)) -> Pair(right, left)

fn increment(value: Int) -> Int {
  value + 1
}

fn unbox_int(value: Box(Int)) -> Int {
  get(value)
}

fn callback_box() -> Box(String) {
  new("callback")
}

pub fn main() {
  let original = new("alpha")
  let replaced = replace(original, 7)
  let mapped = map(replaced, increment)
  let paired = pair("left", 7)
  let swapped = swap(paired)
  let token_box = new(token("opaque"))
  #(
    get(original),
    get(replaced),
    get(mapped),
    contains(original, "alpha"),
    contains(replaced, 7),
    !contains(replaced, 8),
    original == new("alpha"),
    first(paired),
    second(paired),
    first(swapped),
    second(swapped),
    paired == pair("left", 7),
    with_box(9, unbox_int),
    from_box_callback(callback_box),
    contains(token_box, token("opaque")),
  )
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("generic external component should register")
}

#[test]
fn generic_external_schema_and_functions_preserve_source_parameter_shapes() {
    let providers = providers();
    assert_eq!(providers.len(), 1);
    let external = providers[0].external_types().cloned().collect::<Vec<_>>();
    assert_eq!(
        external,
        [
            HostExternalTypeSchema::new("generic_box", "generic_box", "Token", 0,),
            HostExternalTypeSchema::new("generic_box", "generic_box", "Box", 1,),
            HostExternalTypeSchema::new("generic_box", "generic_box", "Pair", 2,),
        ]
    );
    assert_eq!(
        providers[0]
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        [
            "token",
            "new",
            "get",
            "replace",
            "contains",
            "map",
            "with_box",
            "from_box_callback",
            "pair",
            "first",
            "second",
            "swap",
        ],
    );
}

#[test]
fn generic_external_values_retain_specialized_values_persistently() {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("generic external module should be unique");
    let typed = compile_typed_host_program(
        "generic_box",
        "generic_box",
        [PackageSource::new(
            "generic_box",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "generic_box",
                "src/generic_box.gleam",
                SOURCE,
            )],
        )],
        hosts,
    )
    .expect("complete generic external source should compile");
    let plan = plan_host_program(typed).expect("generic external provider should link");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external execution should seal");
    let returned = execution
        .run_main(&mut ProfileState { component: () }, &mut Vec::new())
        .expect("generic external provider should execute");

    assert_eq!(
        returned,
        RuntimeValue::Tuple(vec![
            RuntimeValue::String(EcoString::from("alpha")),
            RuntimeValue::Int(7.into()),
            RuntimeValue::Int(8.into()),
            RuntimeValue::Bool(true),
            RuntimeValue::Bool(true),
            RuntimeValue::Bool(true),
            RuntimeValue::Bool(true),
            RuntimeValue::String(EcoString::from("left")),
            RuntimeValue::Int(7.into()),
            RuntimeValue::Int(7.into()),
            RuntimeValue::String(EcoString::from("left")),
            RuntimeValue::Bool(true),
            RuntimeValue::Int(9.into()),
            RuntimeValue::String(EcoString::from("callback")),
            RuntimeValue::Bool(true),
        ]),
    );
}

#[test]
fn generic_external_scheme_mismatch_preserves_exact_linkage_context() {
    let mismatched = r#"
@external(erlang, "generic_box", "Box")
pub type Box(item)

@external(erlang, "generic_box", "Token")
pub type Token

@external(erlang, "generic_box", "Pair")
pub type Pair(left, right)

@external(erlang, "generic_box", "get")
fn get(boxed: Box(item)) -> Bool

pub fn main() {
  Nil
}
"#;
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("generic external module should be unique");
    let typed = compile_typed_host_program(
        "generic_box",
        "generic_box",
        [PackageSource::new(
            "generic_box",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "generic_box",
                "src/generic_box.gleam",
                mismatched,
            )],
        )],
        hosts,
    )
    .expect("mismatched generic external source should still compile");
    let error = match plan_host_program(typed) {
        Err(error) => error,
        Ok(_) => panic!("mismatched generic external return should fail during linkage"),
    };
    let (package, module, function, reason) = match error {
        PlanError::HostProviderLink {
            package,
            module,
            function,
            reason,
        } => (package, module, function, reason),
        other => panic!(
            "generic external mismatch should remain a host provider linkage error: {other:?}"
        ),
    };
    assert_eq!(package.as_str(), "generic_box");
    assert_eq!(module.as_str(), "generic_box");
    assert_eq!(function.as_str(), "get");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("generic external mismatch should preserve the exact schemes");
    };
    assert_eq!(expected_scheme.parameters().len(), 1);
    assert_eq!(actual_scheme.parameters().len(), 1);
    assert_eq!(expected_type.argument_types().len(), 1);
    assert_eq!(actual_type.argument_types().len(), 1);
    assert!(matches!(expected_type.return_(), ValueType::Bool));
    let ValueType::Parameter(actual_return) = actual_type.return_() else {
        panic!("generated generic external return should preserve its parameter");
    };
    assert_eq!(actual_return.index(), 0);

    for argument in [
        &expected_type.argument_types()[0],
        &actual_type.argument_types()[0],
    ] {
        let ValueType::External(boxed) = argument else {
            panic!("generic external argument should preserve its nominal type");
        };
        assert_eq!(boxed.type_name().package().as_str(), "generic_box");
        assert_eq!(boxed.type_name().module().as_str(), "generic_box");
        assert_eq!(boxed.type_name().name().as_str(), "Box");
        let [ValueType::Parameter(item)] = boxed.arguments() else {
            panic!("generic external argument should preserve one type parameter");
        };
        assert_eq!(item.index(), 0);
    }
}
