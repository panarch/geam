use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Index0, Inspection, Retained, RetainedExternalPayload,
};
use geam_core::provider::{Call, Value};
use geam_core::{
    HostComponentProfile, HostExternalTypeSchema, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderModule, HostProviderSet, HostRegistrationError,
    HostedExecution, ModuleSource, PackageSource, Value as RuntimeValue,
    compile_typed_host_program, plan_host_program,
};
use im::Vector;
use num_bigint::BigInt;
use std::rc::Rc;

pub trait RetainedHostProfile: HostProfile + HostComponentProfile<Component> {}

impl<Profile> RetainedHostProfile for Profile where
    Profile: HostProfile + HostComponentProfile<Component>
{
}

fn retained_stores<Profile>(stores: &Profile::ExternalStores) -> &retained_queue::__GeamStores
where
    Profile: RetainedHostProfile,
{
    &<Profile as HostComponentProfile<Component>>::component_stores(stores).retained_queue
}

pub struct Component;

#[derive(Default)]
pub struct Stores {
    retained_queue: retained_queue::__GeamStores,
}

impl HostProviderComponent for Component {
    const ID: &'static str = "retained-queue";
    type Stores = Stores;
    type RunState = ();
}

impl geam_core::__macro_support::ProviderPackage for Component {
    const PACKAGE: &'static str = "retained_queue";
}

impl<Profile> HostProviderComponentRegistration<Profile> for Component
where
    Profile: RetainedHostProfile,
{
    fn providers() -> Result<Vec<HostProviderModule<Profile>>, HostRegistrationError> {
        Ok(vec![retained_queue::__geam_module::<Profile>()?])
    }
}

#[geam_macros::module(
    path = "retained_queue",
    crate_path = geam_core,
    profile = crate::RetainedHostProfile,
    component = crate::Component,
    stores = crate::retained_stores,
)]
mod retained_queue {
    use super::{
        BigInt, Call, EcoString, Equality, Hashing, Index0, Inspection, Rc, Retained,
        RetainedExternalPayload, Value, Vector,
    };

    struct Entry {
        priority: BigInt,
        value: Retained<QueuePayload, Index0>,
    }

    pub struct QueuePayload {
        entries: Vector<Rc<Entry>>,
    }

    impl RetainedExternalPayload for QueuePayload {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.entries.len() == other.entries.len()
                && self
                    .entries
                    .iter()
                    .zip(&other.entries)
                    .all(|(left, right)| {
                        left.priority == right.priority
                            && left.value.source_equal(context, &right.value)
                    })
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            geam_core::__macro_support::external_payload_hash(
                &self
                    .entries
                    .iter()
                    .map(|entry| (&entry.priority, entry.value.source_hash(context)))
                    .collect::<Vec<_>>(),
            )
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            let values = self
                .entries
                .iter()
                .map(|entry| format!("#({}, {})", entry.priority, entry.value.inspect(context)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("PriorityQueue([{values}])").into()
        }
    }

    #[geam_macros::external(
        name = "PriorityQueue",
        parameters = [Item],
        input = PriorityQueueInput,
        payload = QueuePayload,
        manual,
    )]
    pub struct PriorityQueue<Item>;

    #[geam_macros::function]
    fn empty<Item>() -> PriorityQueue<Item> {
        PriorityQueue::from_payload(QueuePayload {
            entries: Vector::new(),
        })
    }

    #[geam_macros::function]
    fn push<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        queue: PriorityQueueInput<Item>,
        priority: BigInt,
        value: Value<Item>,
    ) -> PriorityQueue<Item> {
        let mut entries = queue.payload().entries.clone();
        entries.push_back(Rc::new(Entry {
            priority,
            value: call.store(value).into_retained(),
        }));
        PriorityQueue::from_payload(QueuePayload { entries })
    }

    #[geam_macros::function]
    fn top_or<Item>(
        #[geam_macros::call] call: &mut Call<()>,
        queue: PriorityQueueInput<Item>,
        fallback: Value<Item>,
    ) -> Value<Item> {
        let Some(index) = queue
            .payload()
            .entries
            .iter()
            .enumerate()
            .max_by_key(|(_, entry)| &entry.priority)
            .map(|(index, _)| index)
        else {
            return fallback;
        };
        let value = queue.stored_item(|payload| &payload.entries[index].value);
        call.restore(value)
    }

    #[geam_macros::function]
    fn size<Item>(queue: PriorityQueueInput<Item>) -> BigInt {
        queue.payload().entries.len().into()
    }

    #[geam_macros::function]
    fn identity<Item>(queue: PriorityQueueInput<Item>) -> PriorityQueue<Item> {
        queue.into_value()
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
@external(erlang, "retained_queue", "PriorityQueue")
pub type PriorityQueue(item)

@external(erlang, "retained_queue", "empty")
fn empty() -> PriorityQueue(item)

@external(erlang, "retained_queue", "push")
fn push(queue: PriorityQueue(item), priority: Int, value: item) -> PriorityQueue(item)

@external(erlang, "retained_queue", "top_or")
fn top_or(queue: PriorityQueue(item), fallback: item) -> item

@external(erlang, "retained_queue", "size")
fn size(queue: PriorityQueue(item)) -> Int

@external(erlang, "retained_queue", "identity")
fn identity(queue: PriorityQueue(item)) -> PriorityQueue(item)

pub fn main() {
  let empty_strings = empty()
  let low = push(empty_strings, 1, "low")
  let high = push(low, 9, "high")
  let equal = push(push(empty(), 1, "low"), 9, "high")
  let numbers = push(empty(), 4, 42)
  let same_high = identity(high)

  #(
    top_or(empty_strings, "empty"),
    top_or(low, "empty"),
    top_or(high, "empty"),
    size(low),
    size(high),
    high == equal,
    top_or(numbers, 0),
    same_high == high,
  )
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("retained external component should register")
}

#[test]
fn retained_external_schema_uses_one_store_for_every_specialization() {
    let providers = providers();
    assert_eq!(providers.len(), 1);
    assert_eq!(
        providers[0].external_types().cloned().collect::<Vec<_>>(),
        [HostExternalTypeSchema::new(
            "retained_queue",
            "retained_queue",
            "PriorityQueue",
            1,
        )],
    );
    assert_eq!(
        providers[0]
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        ["empty", "push", "top_or", "size", "identity"],
    );
    assert_eq!(
        std::mem::size_of::<<Component as HostProviderComponent>::Stores>(),
        std::mem::size_of::<geam_core::HostExternalStore<retained_queue::QueuePayload>>(),
    );
}

#[test]
fn retained_external_payloads_share_old_entries_and_restore_exact_specializations() {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("retained external module should be unique");
    let typed = compile_typed_host_program(
        "retained_queue",
        "retained_queue",
        [PackageSource::new(
            "retained_queue",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "retained_queue",
                "src/retained_queue.gleam",
                SOURCE,
            )],
        )],
        hosts,
    )
    .expect("complete retained external source should compile");
    let plan = plan_host_program(typed).expect("retained external provider should link");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("retained external execution should seal");
    let returned = execution
        .run_main(&mut ProfileState { component: () }, &mut Vec::new())
        .expect("retained external provider should execute");

    assert_eq!(
        returned,
        RuntimeValue::Tuple(vec![
            RuntimeValue::String(EcoString::from("empty")),
            RuntimeValue::String(EcoString::from("low")),
            RuntimeValue::String(EcoString::from("high")),
            RuntimeValue::Int(1.into()),
            RuntimeValue::Int(2.into()),
            RuntimeValue::Bool(true),
            RuntimeValue::Int(42.into()),
            RuntimeValue::Bool(true),
        ]),
    );
}
