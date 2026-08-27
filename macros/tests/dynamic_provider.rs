use ecow::EcoString;
use geam_core::provider::advanced::{
    DynamicKind, Equality, Hashing, Inspection, RetainedExternalPayload, StoredDynamic,
};
use geam_core::provider::{Call, List, Stored, Value};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

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
}

#[geam_macros::module(path = "dynamic_provider", crate_path = geam_core)]
mod dynamic_provider {
    use super::declarations::Token;
    use super::{
        BigInt, Call, DynamicKind, EcoString, Equality, Hashing, Inspection, List,
        RetainedExternalPayload, Stored, StoredDynamic, Value,
    };

    #[geam_macros::external(name = "Dynamic", retained)]
    struct Dynamic {
        value: StoredDynamic<Dynamic>,
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
    fn kind(value: &Dynamic) -> EcoString {
        match value.value.kind() {
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
        let value: StoredDynamic<Dynamic> = call.store_dynamic(value);
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
        let value: StoredDynamic<Dynamic> = call.store_dynamic(value);
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
    fn has_exact_type<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        stored: &Dynamic,
        witness: Value<Item>,
    ) -> bool {
        call.restore_dynamic_value(&stored.value, &witness)
            .is_some()
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
import dynamic_provider/declarations

@external(erlang, "dynamic_provider", "Dynamic")
pub type Dynamic

@external(erlang, "dynamic_provider", "Box")
pub type Box(item)

@external(erlang, "dynamic_provider", "box_value")
fn box_value(value: item) -> Box(item)

@external(erlang, "dynamic_provider", "cast")
fn cast(value: value) -> Dynamic

@external(erlang, "dynamic_provider", "cast_int")
fn cast_int(value: Int) -> Dynamic

@external(erlang, "dynamic_provider", "cast_int_list")
fn cast_int_list(values: List(Int)) -> Dynamic

@external(erlang, "dynamic_provider", "kind")
fn kind(value: Dynamic) -> String

@external(erlang, "dynamic_provider", "restore_int")
fn restore_int(value: Dynamic) -> Result(Int, Nil)

@external(erlang, "dynamic_provider", "restore_int_list_length")
fn restore_int_list_length(value: Dynamic) -> Result(Int, Nil)

@external(erlang, "dynamic_provider", "is_token")
fn is_token(value: Dynamic) -> Bool

@external(erlang, "dynamic_provider", "is_box")
fn is_box(value: Dynamic) -> Bool

@external(erlang, "dynamic_provider", "token_text")
fn token_text(value: Dynamic) -> Result(String, Nil)

@external(erlang, "dynamic_provider", "box_contains_nine")
fn box_contains_nine(value: Dynamic) -> Result(Bool, Nil)

@external(erlang, "dynamic_provider", "boxed_token_text")
fn boxed_token_text(value: Dynamic) -> Result(String, Nil)

@external(erlang, "dynamic_provider", "tuple_size")
fn tuple_size(value: value) -> Int

@external(erlang, "dynamic_provider", "nested_tuple_size")
fn nested_tuple_size(value: value) -> Int

@external(erlang, "dynamic_provider", "same_hash")
fn same_hash(first: value, second: value) -> Bool

@external(erlang, "dynamic_provider", "has_exact_type")
fn has_exact_type(stored: Dynamic, witness: value) -> Bool

@external(erlang, "dynamic_provider", "list_summary")
fn list_summary(values: List(item), expected: item) -> #(Int, Bool)

pub type Marker {
  Marker
}

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  let first = cast(7)
  let equal = cast(7)
  let text = cast("seven")
  let token = declarations.token("opaque")
  let identity_token = declarations.identity_token(token)
  let #(paired_token, pair_flag) = declarations.identity_token_pair(token)
  let token_value = cast(token)
  let boxed = cast(box_value(9))
  let boxed_token = cast(box_value(token))
  let typed_int = cast_int(8)
  let typed_list = cast_int_list([1, 2])
  #(
    kind(first),
    kind(text),
    kind(token_value),
    kind(cast(1.5)),
    kind(cast(<<1>>)),
    kind(cast(codepoint)),
    kind(cast(True)),
    kind(cast(Nil)),
    kind(cast([1])),
    kind(cast(#(1, True))),
    kind(cast(Marker)),
    kind(cast(increment)),
    kind(typed_int),
    kind(typed_list),
    restore_int(first),
    restore_int(text),
    restore_int_list_length(typed_list),
    is_token(token_value),
    !is_token(first),
    is_box(boxed),
    !is_box(token_value),
    token_text(token_value),
    token_text(first),
    box_contains_nine(boxed),
    box_contains_nine(first),
    boxed_token_text(boxed_token),
    pair_flag,
    paired_token == token,
    tuple_size(#(1, "two", True)),
    tuple_size(1),
    nested_tuple_size(#(1, #("two", True))),
    same_hash(first, equal),
    has_exact_type(first, 0),
    !has_exact_type(first, "zero"),
    list_summary([1, 2, 3], 1),
    list_summary([], 1),
    first == equal,
    first,
    token,
    identity_token,
  )
}
"#;

const DECLARATIONS_SOURCE: &str = r#"
@external(erlang, "dynamic_provider", "Token")
pub type Token

@external(erlang, "dynamic_provider", "token")
pub fn token(value: String) -> Token

@external(erlang, "dynamic_provider", "identity_token")
pub fn identity_token(value: Token) -> Token

@external(erlang, "dynamic_provider", "identity_token_pair")
pub fn identity_token_pair(value: Token) -> #(Token, Bool)
"#;

#[test]
fn existential_values_restore_exact_types_and_preserve_source_semantics() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("dynamic provider should register");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("dynamic provider module should be unique");
    let typed = compile_typed_host_program(
        "dynamic_provider",
        "dynamic_provider",
        [PackageSource::new(
            "dynamic_provider",
            Vec::<&str>::new(),
            [
                ModuleSource::new(
                    "dynamic_provider/declarations",
                    "src/dynamic_provider/declarations.gleam",
                    DECLARATIONS_SOURCE,
                ),
                ModuleSource::new("dynamic_provider", "src/dynamic_provider.gleam", SOURCE),
            ],
        )],
        hosts,
    )
    .expect("complete dynamic source should compile");
    let plan = plan_host_program(typed).expect("dynamic provider should link");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("dynamic provider execution should seal");
    let returned = execution
        .run_main(&mut ProfileState { component: () }, &mut Vec::new())
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
