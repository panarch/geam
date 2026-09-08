use ecow::EcoString;
use geam_builtin::FutureComponent;
use geam_builtin::embedding::FutureType;
use geam_core::embedding::{
    BigInt as EmbeddingInt, FunctionDeclaration, HostedModuleBuilder, with_execution_scope,
};
use geam_core::host::HostFutureStore;
use geam_core::provider::advanced::{
    DynamicKind, Equality, Hashing, Index0, Inspection, Retained, RetainedExternalPayload,
    StoredDynamic,
};
use geam_core::provider::{Call, List, Stored, Value};
use geam_core::{
    EchoOutput, EchoSink, HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

#[geam_macros::provider(
    package = "dynamic_provider",
    modules = [declarations, dynamic_provider],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "dynamic_provider/declarations", crate_path = geam_core)]
mod declarations {
    use super::EcoString;
    use geam_core::provider::advanced::External;

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    pub(super) struct Token(pub(super) EcoString);

    #[geam_macros::function]
    fn token(value: EcoString) -> Token {
        Token(value)
    }

    #[geam_macros::function]
    fn identity_token(value: External<Token>) -> External<Token> {
        value
    }

    #[geam_macros::function]
    fn identity_token_pair(value: External<Token>) -> (External<Token>, bool) {
        (value, true)
    }

    #[geam_macros::function]
    async fn identity_token_async(value: External<Token>) -> External<Token> {
        std::future::ready(()).await;
        value
    }

    #[geam_macros::function]
    async fn identity_token_pair_async(value: External<Token>) -> (External<Token>, bool) {
        std::future::ready(()).await;
        (value, true)
    }

    #[geam_macros::function]
    fn first_token(values: geam_core::List<Token>) -> External<Token> {
        values
            .get(0)
            .expect("the test List contains an external token")
    }

    #[geam_macros::function]
    async fn first_token_async(values: geam_core::List<Token>) -> External<Token> {
        std::future::ready(()).await;
        values
            .get(0)
            .expect("the test List contains an external token")
    }
}

#[geam_macros::module(path = "dynamic_provider", crate_path = geam_core)]
mod dynamic_provider {
    use super::declarations::Token;
    use super::{
        BigInt, Call, DynamicKind, EcoString, Equality, Hashing, Index0, Inspection, List,
        Retained, RetainedExternalPayload, Stored, StoredDynamic, Value,
    };

    #[geam_macros::external(name = "Dynamic", retained)]
    struct Dynamic {
        value: StoredDynamic<Dynamic>,
    }

    #[geam_macros::external(name = "Snapshot", retained)]
    struct Snapshot {
        value: Retained<Snapshot, Index0>,
    }

    #[geam_macros::external(name = "Box", parameters = [Item], input = BoxInput)]
    struct BoxValue<Item> {
        #[geam_macros::stored]
        value: Stored<Item>,
    }

    impl RetainedExternalPayload for Dynamic {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.inspect(context)
        }
    }

    impl RetainedExternalPayload for Snapshot {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            format!("Snapshot({})", self.value.inspect(context)).into()
        }
    }

    #[geam_macros::function]
    fn cast<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> Dynamic {
        Dynamic {
            value: call.store_dynamic(value),
        }
    }

    #[geam_macros::function]
    fn cast_int(#[geam_macros::call] call: &mut Call<()>, value: BigInt) -> Dynamic {
        Dynamic {
            value: call.store_dynamic(value),
        }
    }

    #[geam_macros::function]
    fn cast_int_list(#[geam_macros::call] call: &mut Call<()>, values: List<BigInt>) -> Dynamic {
        Dynamic {
            value: call.store_dynamic(values),
        }
    }

    #[geam_macros::function]
    fn box_value<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Item>,
    ) -> BoxValue<Item> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    fn snapshot<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> Snapshot {
        Snapshot {
            value: call.store(value).into_retained(),
        }
    }

    #[geam_macros::function]
    fn kind(value: &Dynamic) -> EcoString {
        kind_name(value.value.kind())
    }

    #[geam_macros::function]
    async fn kind_async(value: &Dynamic) -> EcoString {
        std::future::ready(()).await;
        value.with(|value| kind_name(value.value.kind()))
    }

    fn kind_name(kind: DynamicKind) -> EcoString {
        match kind {
            DynamicKind::Int => "Int",
            DynamicKind::Float => "Float",
            DynamicKind::String => "String",
            DynamicKind::BitArray => "BitArray",
            DynamicKind::UtfCodepoint => "UtfCodepoint",
            DynamicKind::Bool => "Bool",
            DynamicKind::Nil => "Nil",
            DynamicKind::List => "List",
            DynamicKind::Tuple => "Tuple",
            DynamicKind::Custom => "Custom",
            DynamicKind::External => "External",
            DynamicKind::Function => "Function",
        }
        .into()
    }

    #[geam_macros::function]
    fn restore_int(
        #[geam_macros::call] call: &mut Call<()>,
        value: &Dynamic,
    ) -> Result<BigInt, ()> {
        call.restore_dynamic::<BigInt, Dynamic>(&value.value)
            .ok_or(())
    }

    #[geam_macros::function]
    fn restore_int_list_length(
        #[geam_macros::call] call: &mut Call<()>,
        value: &Dynamic,
    ) -> Result<BigInt, ()> {
        call.restore_dynamic::<List<BigInt>, Dynamic>(&value.value)
            .map(|values| values.len().into())
            .ok_or(())
    }

    #[geam_macros::function]
    fn is_token(value: &Dynamic) -> bool {
        value.value.is_external::<Token>()
    }

    #[geam_macros::function]
    fn is_box(value: &Dynamic) -> bool {
        value.value.is_external::<BoxValue<BigInt>>()
    }

    #[geam_macros::function]
    fn token_text(
        #[geam_macros::call] call: &mut Call<()>,
        value: &Dynamic,
    ) -> Result<EcoString, ()> {
        let token = call
            .restore_dynamic::<Token, Dynamic>(&value.value)
            .ok_or(())?;
        Ok(token.0.clone())
    }

    #[geam_macros::function]
    fn box_contains_nine(
        #[geam_macros::call] call: &mut Call<()>,
        value: &Dynamic,
    ) -> Result<bool, ()> {
        let boxed = call
            .restore_dynamic::<BoxValue<BigInt>, Dynamic>(&value.value)
            .ok_or(())?;
        let value = call.restore(boxed.value());
        Ok(call.inspect(&value) == "9")
    }

    #[geam_macros::function]
    fn boxed_token_text(
        #[geam_macros::call] call: &mut Call<()>,
        value: &Dynamic,
    ) -> Result<EcoString, ()> {
        let boxed = call
            .restore_dynamic::<BoxValue<Token>, Dynamic>(&value.value)
            .ok_or(())?;
        let token = call.restore(boxed.value());
        Ok(call.external_payload(token).0.clone())
    }

    #[geam_macros::function]
    fn tuple_size<Item>(#[geam_macros::call] call: &mut Call<()>, value: Value<Item>) -> BigInt {
        let value = call.store_dynamic::<_, Dynamic>(value);
        value
            .into_tuple_items()
            .map(|items| BigInt::from(items.len()))
            .unwrap_or_default()
    }

    #[geam_macros::function]
    fn nested_tuple_size<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Item>,
    ) -> BigInt {
        let value = call.store_dynamic::<_, Dynamic>(value);
        let Ok(items) = value.into_tuple_items() else {
            return BigInt::default();
        };
        items
            .into_vec()
            .into_iter()
            .nth(1)
            .and_then(|value| value.into_tuple_items().ok())
            .map(|items| BigInt::from(items.len()))
            .unwrap_or_default()
    }

    #[geam_macros::function]
    fn same_hash<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        first: Value<Item>,
        second: Value<Item>,
    ) -> bool {
        call.source_hash(&first) == call.source_hash(&second)
    }

    #[geam_macros::function]
    fn inspect_value<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        value: Value<Item>,
    ) -> EcoString {
        call.inspect(&value)
    }

    #[geam_macros::function]
    fn has_exact_type<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        stored: &Dynamic,
        witness: Value<Item>,
    ) -> bool {
        call.restore_dynamic_value(&stored.value, &witness)
            .is_some()
    }

    #[geam_macros::function]
    fn list_length<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        values: Value<geam_core::List<Item>>,
    ) -> BigInt {
        call.list_len(&values).into()
    }

    #[geam_macros::function]
    fn list_summary<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        values: Value<geam_core::List<Item>>,
        expected: Value<Item>,
    ) -> (BigInt, bool) {
        let matches = call
            .list_get::<_, Item, _>(&values, 0)
            .is_some_and(|value| call.equal(&value, &expected));
        (call.list_len(&values).into(), matches)
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
    future: HostFutureStore,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
    future: (),
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}

impl HostComponentProfile<FutureComponent> for Profile {
    fn component_stores(stores: &ProfileStores) -> &HostFutureStore {
        &stores.future
    }

    fn component_state(state: &mut ProfileState) -> &mut () {
        &mut state.future
    }
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

struct AsyncProfile;

#[derive(Default)]
struct FutureStores {
    provider: <Component as HostProviderComponent>::Stores,
    future: HostFutureStore,
}

#[derive(Default)]
struct AsyncEcho(Vec<String>);

impl EchoSink for AsyncEcho {
    fn emit(&mut self, output: EchoOutput) {
        self.0.push(output.value().inspect().to_string());
    }
}

impl HostProfile for AsyncProfile {
    type RunState = <Component as HostProviderComponent>::RunState;
    type ExternalStores = FutureStores;
}

impl HostComponentProfile<Component> for AsyncProfile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.provider
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        state
    }
}

impl geam_core::host::HostWorkProfile for AsyncProfile {
    type Work = FutureComponent;
}
impl HostComponentProfile<FutureComponent> for AsyncProfile {
    fn component_stores(stores: &FutureStores) -> &HostFutureStore {
        &stores.future
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

const FUTURE_SOURCE: &str = concat!(
    "import geam/future\n",
    include_str!("fixtures/future_dynamic/values.gleam"),
    r#"
@external(erlang, "dynamic_provider", "kind_async")
fn kind_async(value: Dynamic) -> future.Future(String)

pub fn async_flow(value: Int) {
  let dynamic = cast(value)
  let before = kind(dynamic)
  use during <- future.then(kind_async(dynamic))
  let after = restore_int(dynamic)
  let token = declarations.token("async")
  use identity <- future.then(declarations.identity_token_async(token))
  let assert True = identity == token
  use #(paired, pair_flag) <- future.then(declarations.identity_token_pair_async(token))
  let assert True = pair_flag && paired == token
  use first <- future.map(declarations.first_token_async([token]))
  let assert True = first == token
  #(before, during, after)
}
"#,
);
const FUTURE_DECLARATIONS: &str = concat!(
    "import geam/future\n",
    include_str!("fixtures/future_dynamic/declarations.gleam"),
    r#"
@external(erlang, "dynamic_provider", "identity_token_async")
pub fn identity_token_async(value: Token) -> future.Future(Token)
@external(erlang, "dynamic_provider", "identity_token_pair_async")
pub fn identity_token_pair_async(value: Token) -> future.Future(#(Token, Bool))
@external(erlang, "dynamic_provider", "first_token_async")
pub fn first_token_async(values: List(Token)) -> future.Future(Token)
"#,
);

#[test]
fn existential_values_restore_exact_types_and_preserve_source_semantics() {
    let mut providers = FutureComponent::providers().expect("Future component");
    providers.extend(
        <Component as HostProviderComponentRegistration<Profile>>::providers()
            .expect("dynamic provider should register"),
    );
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("dynamic provider module should be unique");
    let typed = compile_typed_host_program(
        "dynamic_provider",
        "dynamic_provider",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "dynamic_provider",
                ["geam"],
                [
                    ModuleSource::new(
                        "dynamic_provider/declarations",
                        "src/dynamic_provider/declarations.gleam",
                        FUTURE_DECLARATIONS,
                    ),
                    ModuleSource::new(
                        "dynamic_provider",
                        "src/dynamic_provider.gleam",
                        FUTURE_SOURCE,
                    ),
                ],
            ),
        ],
        hosts,
    )
    .expect("complete dynamic source should compile");
    let plan = plan_host_program(typed).expect("dynamic provider should link");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("dynamic provider execution should seal");
    let returned = execution
        .run_main(
            &mut ProfileState {
                component: (),
                future: (),
            },
            &mut Vec::new(),
        )
        .expect("dynamic provider should execute");

    assert_eq!(
        returned.inspect().to_string(),
        r#"#("Int", "String", "External", "Float", "BitArray", "UtfCodepoint", "Bool", "Nil", "List", "Tuple", "Custom", "Function", "Int", "List", Ok(7), Error(Nil), Ok(2), True, True, True, True, Ok("opaque"), Error(Nil), Ok(True), Error(Nil), Ok("opaque"), True, True, 3, 0, 2, True, True, True, #(3, True), #(0, False), True, 7, Token(<opaque>), Token(<opaque>))"#,
    );
    let geam_core::Value::Tuple(values) = returned else {
        panic!("dynamic main should return its inspected tuple");
    };
    let [
        ..,
        geam_core::Value::External(original),
        geam_core::Value::External(identity),
    ] = values.as_slice()
    else {
        panic!("dynamic main should preserve both Token values");
    };
    assert_eq!(original.identity(), identity.identity());
}

#[test]
fn retained_dynamic_values_cross_pending_execution_without_changing_identity() {
    const APPLICATION: &str = r#"
import dynamic_provider

pub fn flow(value: Int) { dynamic_provider.async_flow(value) }
pub fn direct() { dynamic_provider.transfer_flow() }
"#;

    let mut providers = FutureComponent::providers().expect("Future component");
    providers.extend(
        <Component as HostProviderComponentRegistration<AsyncProfile>>::providers()
            .expect("dynamic provider should register for transferable execution"),
    );
    let hosts = HostProviderSet::from_providers(providers)
        .expect("dynamic provider module should be unique");
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../../builtins/geam/gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "dynamic_provider",
                ["geam"],
                [
                    ModuleSource::new(
                        "dynamic_provider/declarations",
                        "src/dynamic_provider/declarations.gleam",
                        FUTURE_DECLARATIONS,
                    ),
                    ModuleSource::new(
                        "dynamic_provider",
                        "src/dynamic_provider.gleam",
                        FUTURE_SOURCE,
                    ),
                ],
            ),
            PackageSource::new(
                "application",
                ["dynamic_provider"],
                [ModuleSource::new("main", "src/main.gleam", APPLICATION)],
            ),
        ],
        hosts,
    )
    .expect("dynamic async source should compile");
    let (mut bindings, direct) = HostedModuleBuilder::new(typed)
        .expect("dynamic async source should plan")
        .function(FunctionDeclaration::<
            (),
            (
                bool,
                bool,
                bool,
                bool,
                bool,
                bool,
                (bool, bool, bool, bool, bool, bool, bool),
            ),
        >::new("direct"))
        .expect("dynamic direct flow should bind");
    let flow = bindings
        .function(FunctionDeclaration::<
            (EmbeddingInt,),
            FutureType<(EcoString, EcoString, Result<EmbeddingInt, ()>)>,
        >::new("flow"))
        .expect("dynamic flow should bind");
    let mut module = bindings.seal().expect("dynamic flow should seal");
    let mut echo = AsyncEcho::default();
    let mut state = ();
    let returned = poll_ready(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        assert_eq!(
            scope.call(&direct, ()).expect("direct retained values"),
            (
                true,
                true,
                true,
                true,
                true,
                true,
                (true, true, true, true, true, true, true)
            )
        );
        let work = scope
            .call(&flow, (7.into(),))
            .expect("construct retained work");
        scope.observe(&work).await.expect("dynamic completion")
    }));
    returned.read(|(before, during, result)| {
        assert_eq!(before, "Int");
        assert_eq!(during, "Int");
        assert_eq!(result, Ok(&EmbeddingInt::from(7)));
    });
    assert_eq!(echo.0, ["Snapshot(11)"]);
}

fn poll_ready<Output>(future: impl Future<Output = Output>) -> Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    for _ in 0..16 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("future did not become ready")
}
