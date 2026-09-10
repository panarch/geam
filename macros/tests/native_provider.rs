#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Inspection, NativeValue, RetainedExternalPayload,
};
use geam_core::provider::{Call, Callback, HostResult, Value};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
    plan_host_program,
};
use num_bigint::BigInt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct State {
    projections: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}

#[geam_macros::provider(package = "native_views", modules = [native], state = State, crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "native_views", crate_path = geam_core)]
mod native {
    use super::{
        Arc, AtomicUsize, BigInt, Call, Callback, EcoString, Equality, Hashing, HostResult,
        Inspection, NativeValue, Ordering, RetainedExternalPayload, State, Value,
    };
    use std::cell::RefCell;

    #[geam_macros::external(name = "Key", retained)]
    struct Key {
        value: RefCell<NativeValue>,
        projections: Arc<AtomicUsize>,
        drops: Arc<AtomicUsize>,
    }

    impl RetainedExternalPayload for Key {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value
                .borrow()
                .source_equal(context, &other.value.borrow())
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.borrow().source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.borrow().inspect(context)
        }

        fn native_view(&self) -> Option<NativeValue> {
            self.projections.fetch_add(1, Ordering::Relaxed);
            Some(self.value.borrow().clone())
        }
    }

    impl Drop for Key {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[geam_macros::external(name = "Envelope", retained)]
    struct Envelope {
        value: NativeValue,
    }

    impl RetainedExternalPayload for Envelope {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.inspect(context)
        }

        fn native_view(&self) -> Option<NativeValue> {
            Some(self.value.clone())
        }
    }

    #[geam_macros::function]
    fn key(#[geam_macros::call] call: &mut Call<State>, name: EcoString) -> Key {
        Key {
            value: RefCell::new(NativeValue::symbol(name)),
            projections: Arc::clone(&call.state().projections),
            drops: Arc::clone(&call.state().drops),
        }
    }

    #[geam_macros::function]
    fn projections(#[geam_macros::call] call: &mut Call<State>) -> BigInt {
        call.state().projections.load(Ordering::Relaxed).into()
    }

    #[geam_macros::function]
    fn wrap<Item>(#[geam_macros::call] call: &mut Call<State>, value: Value<Item>) -> Envelope {
        Envelope {
            value: call.store_dynamic::<_, Envelope>(value).native_view(),
        }
    }

    #[geam_macros::function]
    fn array(#[geam_macros::call] call: &mut Call<State>, values: List<Key>) -> Envelope {
        Envelope {
            value: call.native_tuple(values),
        }
    }

    #[geam_macros::function]
    fn at(value: &Envelope, index: BigInt) -> Result<EcoString, ()> {
        let index = usize::try_from(&index).map_err(|_| ())?;
        value
            .value
            .index(index)
            .and_then(|value| value.as_symbol())
            .ok_or(())
    }

    #[geam_macros::function]
    fn semantics<Left, Right>(
        #[geam_macros::call] call: &mut Call<State>,
        left: Value<Left>,
        right: Value<Right>,
    ) -> (bool, bool) {
        let hashes_match = call.native_source_hash(&left) == call.native_source_hash(&right);
        let left = call.store_dynamic::<_, Envelope>(left).native_view();
        let right = call.store_dynamic::<_, Envelope>(right).native_view();
        (call.native_equal(&left, &right), hashes_match)
    }

    #[geam_macros::function]
    fn inspect<Item>(#[geam_macros::call] call: &mut Call<State>, value: Value<Item>) -> EcoString {
        call.inspect(&value)
    }

    #[geam_macros::function(resumable)]
    async fn reenter(
        #[geam_macros::call] call: &mut Call<State>,
        value: geam_core::provider::advanced::External<Envelope>,
        callback: Callback<fn(EcoString) -> EcoString>,
    ) -> HostResult<EcoString> {
        let value_view = value.with(|value| value.value.clone());
        drop(value);
        call.invoke(
            &callback,
            (value_view
                .index(0)
                .and_then(|value| value.as_symbol())
                .unwrap_or_default(),),
        )
        .await
    }

    #[geam_macros::function]
    fn string(
        #[geam_macros::call] call: &mut Call<State>,
        value: &Envelope,
    ) -> Result<EcoString, ()> {
        call.restore_native::<EcoString>(&value.value).ok_or(())
    }
}

struct Profile;

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &Stores {
        stores
    }
    fn component_state(state: &mut State) -> &mut State {
        state
    }
}

#[test]
fn native_views_keep_lazy_list_storage_send_only_payloads_and_typed_callback_reentry() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let program = compile_typed_host_program(
        "native_views",
        "native_views",
        [PackageSource::new(
            "native_views",
            Vec::<String>::new(),
            [ModuleSource::new(
                "native_views",
                "src/native_views.gleam",
                r#"
pub type Key
pub type Envelope
@external(erlang, "native", "key")
fn key(name: String) -> Key
@external(erlang, "native", "projections")
fn projections() -> Int
@external(erlang, "native", "wrap")
fn wrap(value: a) -> Envelope
@external(erlang, "native", "array")
fn array(values: List(Key)) -> Envelope
@external(erlang, "native", "at")
fn at(value: Envelope, index: Int) -> Result(String, Nil)
@external(erlang, "native", "semantics")
fn semantics(left: a, right: b) -> #(Bool, Bool)
@external(erlang, "native", "inspect")
fn inspect(value: a) -> String
@external(erlang, "native", "reenter")
fn reenter(value: Envelope, callback: fn(String) -> String) -> String
@external(erlang, "native", "string")
fn string(value: Envelope) -> Result(String, Nil)
pub fn main() {
  let first = key("first")
  let second = key("second")
  assert projections() == 0
  let values = array([first, second])
  assert projections() == 0
  assert at(values, 1) == Ok("second")
  assert projections() == 1
  assert at(values, 2) == Error(Nil)
  assert at(values, -1) == Error(Nil)
  assert projections() == 1
  let raw = wrap(#(first, second))
  assert values == raw
  assert semantics(values, raw) == #(True, True)
  assert semantics(values, #(first, second)) == #(True, True)
  assert inspect(values) == "First(Second)"
  assert string(wrap("text")) == Ok("text")
  assert string(wrap(1)) == Error(Nil)
  let suffix = "!"
  assert reenter(values, fn(name) { inspect(key(name <> suffix)) }) == "atom.create(\"first!\")"
  assert reenter(array([]), fn(name) { name }) == ""
  assert at(array([]), 0) == Error(Nil)
  Nil
}
"#,
            )],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(program).unwrap()).unwrap();
    let mut state = State::default();
    let mut echo = Vec::new();
    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut state, &mut echo),
        Ok(geam_core::Value::Nil)
    );
    assert!(echo.is_empty());
    assert_eq!(state.drops.load(Ordering::Relaxed), 3);
}
