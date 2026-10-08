mod codec;
mod type_;
pub use codec::{ProviderRetainedStorage, ProviderValueListDecoder};

use crate::host::{HostCall, HostRetainedValue};
use crate::runtime::StoredRuntimeValue;
use crate::{HostProfile, HostProvider, HostType};
use std::marker::PhantomData;

/// An opaque handle for one statically declared source type.
///
/// Provider macros use this type for generic source values and opaque function
/// pass-through. It does not expose the concrete runtime family or materialize
/// the represented value. Use an active [`super::Call`] for source equality,
/// hashing, and inspection.
pub struct Value<Type, Context = MissingValueContext> {
    context: Context,
    type_: PhantomData<fn() -> Type>,
}

#[doc(hidden)]
pub struct MissingValueContext;

/// Owned retained value used by a transferable provider invocation.
#[doc(hidden)]
pub struct ProviderValueContext<Host>
where
    Host: HostType,
{
    value: StoredRuntimeValue,
    host: PhantomData<fn() -> Host>,
}

impl<Type, Host> Value<Type, ProviderValueContext<Host>>
where
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_host<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: Host::Value<'call>,
    ) -> Self
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        Self {
            context: ProviderValueContext {
                value: call.retain_value::<Host>(value),
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host<'call, Profile, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Result<Host::Value<'call>, crate::HostCallError>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        call.restore_value::<Host>(&self.context.value)
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue {
        &self.context.value
    }

    #[doc(hidden)]
    pub fn from_retained(value: HostRetainedValue<Host>) -> Self {
        Self::from_stored(value.value)
    }

    #[doc(hidden)]
    pub fn into_retained(self) -> HostRetainedValue<Host> {
        HostRetainedValue {
            value: self.into_stored(),
            type_: PhantomData,
        }
    }

    pub(crate) fn from_stored(value: StoredRuntimeValue) -> Self {
        Self {
            context: ProviderValueContext {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    pub(crate) fn into_stored(self) -> StoredRuntimeValue {
        self.context.value
    }
}

impl<Type, Host: HostType> Clone for Value<Type, ProviderValueContext<Host>> {
    fn clone(&self) -> Self {
        Self::from_stored(self.context.value.clone_retained())
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderValueContext, Value};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostProfile, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeParameter, HostValue,
    };
    use crate::provider::{ProviderConstructions, ProviderRootOutputValue};
    use crate::runtime::{BorrowedValue, StoredRuntimeValue};
    use crate::{ExecutionError, HostedExecution, ModuleSource, PackageSource};

    type Parameter = HostTypeParameter<0>;
    type Scoped = Value<Parameter, ProviderValueContext<Parameter>>;
    type Pure = Value<Parameter, ProviderValueContext<Parameter>>;

    struct Profile;
    struct Provider;
    #[derive(Default)]
    struct State {
        scoped: Option<Scoped>,
        pure: Option<Pure>,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    fn scoped<'call>(
        mut call: HostCall<'call, Profile, Provider, Parameter>,
        value: HostValue<'call, Parameter>,
    ) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
        let current = Scoped::from_host(&call, value);
        let retained = call
            .state()
            .scoped
            .replace(current.clone())
            .unwrap_or(current);
        <Scoped as ProviderRootOutputValue<Profile, Provider>>::complete(
            retained,
            call,
            &ProviderConstructions::none(),
        )
    }

    fn pure<'call>(
        mut call: HostCall<'call, Profile, Provider, Parameter>,
        value: HostValue<'call, Parameter>,
    ) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
        let current = Pure::from_host(&call, value);
        let retained = call.state().pure.take().unwrap_or(current);
        call.state().pure = Some(retained.clone());
        <Pure as ProviderRootOutputValue<Profile, Provider>>::complete(
            retained,
            call,
            &ProviderConstructions::none(),
        )
    }

    fn execution(source: &str) -> HostedExecution<Profile> {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function::<Provider, (Parameter,), Parameter, _>("scoped", scoped)
            .unwrap()
            .with_scoped_function::<Provider, (Parameter,), Parameter, _>("pure", pure)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap()).unwrap()
    }

    #[test]
    fn root_completion_rejects_the_wrong_specialization_in_the_original_execution() {
        let mut loaded = execution(
            r#"
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
@external(erlang, "native", "pure") fn pure(value: a) -> a
pub fn main() { #(scoped(42), scoped("different source type")) }
"#,
        );
        let error =
            crate::execution_fixture::run(&mut loaded, &mut State::default(), &mut Vec::new())
                .unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "scoped");
        assert_eq!(
            error.failure().message(),
            "retained value belongs to another owner or source type"
        );
    }

    #[test]
    fn root_completion_rejects_nested_scoped_values_after_their_original_execution() {
        for source in [
            r#"
pub opaque type Secret { Secret(fn(Int) -> Int) }
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
@external(erlang, "native", "pure") fn pure(value: a) -> a
pub fn main() {
  let assert Secret(callback) = scoped(Secret(fn(value) { value + 1 }))
  callback(41)
}
"#,
            r#"
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
@external(erlang, "native", "pure") fn pure(value: a) -> a
pub fn main() {
  let #(callback, _) = scoped(#(fn(value: Int) { value + 1 }, 42))
  callback(41)
}
"#,
            r#"
pub type Box(a) { Box(a) }
pub opaque type Secret { Secret(fn(Int) -> Int) }
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
@external(erlang, "native", "pure") fn pure(value: a) -> a
pub fn main() {
  let assert Box([Secret(callback)]) = scoped(Box([Secret(fn(value) { value + 1 })]))
  callback(41)
}
"#,
        ] {
            let mut loaded = execution(source);
            let mut state = State::default();
            assert_eq!(
                crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap(),
                crate::Value::Int(42.into())
            );
            let error = crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new())
                .unwrap_err();
            let error = expect_host_failure(error);
            assert_eq!(error.function(), "scoped");
            assert_eq!(
                error.failure().message(),
                "retained value belongs to another execution"
            );
        }
    }

    #[test]
    fn opaque_work_cannot_be_rebound_or_observed_after_the_original_execution() {
        use crate::host::{HostComponentProfile, HostFutureStore, HostWorkProfile};
        use crate::work_fixture::WorkComponent;

        struct WorkProfile;
        struct WorkProvider;
        impl HostProfile for WorkProfile {
            type RunState = (State, ());
            type ExternalStores = HostFutureStore;
            type ExecutionState = ();
        }
        impl HostWorkProfile for WorkProfile {
            type Work = WorkComponent;
        }
        impl HostComponentProfile<WorkComponent> for WorkProfile {
            fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
                stores
            }
            fn component_state(state: &mut (State, ())) -> &mut () {
                &mut state.1
            }
        }
        impl HostProvider<WorkProfile> for WorkProvider {
            type State = State;
            fn project(state: &mut (State, ())) -> &mut State {
                &mut state.0
            }
        }
        fn keep<'call>(
            mut call: HostCall<'call, WorkProfile, WorkProvider, Parameter>,
            value: HostValue<'call, Parameter>,
        ) -> Result<HostCallCompletion<'call, Parameter>, HostCallError> {
            let current = Scoped::from_host(&call, value);
            let retained = call.state().scoped.take().unwrap_or(current);
            let value = retained.clone().into_host(&mut call)?;
            call.state().scoped = Some(retained);
            Ok(call.return_value(value))
        }

        let mut providers = WorkComponent::providers::<WorkProfile>().unwrap();
        providers.push(
            HostProviderModule::new("application", "main")
                .unwrap()
                .with_scoped_function::<WorkProvider, (Parameter,), Parameter, _>("keep", keep)
                .unwrap(),
        );
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import fixture/work
pub opaque type Hidden { Hidden(work.Work(Int)) }
@external(erlang, "native", "keep") fn keep(value: a) -> a
pub fn main() {
  let original = work.map(work.ready(40), fn(value) { echo "observed" value + 2 })
  let assert Hidden(retained) = keep(Hidden(original))
  assert retained == original
  Nil
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap();
        let mut loaded =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut state = (State::default(), ());
        assert!(std::ptr::eq(
            <WorkProfile as HostComponentProfile<WorkComponent>>::component_state(&mut state),
            &state.1,
        ));
        let mut echoes = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut loaded, &mut state, &mut echoes).unwrap(),
            crate::Value::Nil,
        );
        assert!(echoes.is_empty(), "transfer must not observe pending work");
        let error =
            crate::execution_fixture::run(&mut loaded, &mut state, &mut echoes).unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "keep");
        assert_eq!(
            error.failure().message(),
            "retained value belongs to another execution"
        );
        assert!(
            echoes.is_empty(),
            "rejection must not reactivate the old work"
        );
    }

    #[test]
    fn static_retention_preserves_pure_opaque_values_across_executions() {
        let mut loaded = execution(
            r#"
pub opaque type Session { Session(Int) }
@external(erlang, "native", "pure") fn pure(value: a) -> a
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
pub fn main() { let assert Session(value) = pure(Session(42)) value }
"#,
        );
        let mut state = State::default();
        for _ in 0..2 {
            assert_eq!(
                crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap(),
                crate::Value::Int(42.into())
            );
        }
        drop(loaded);
        let retained = state.pure.take().unwrap().into_stored();
        assert_eq!(
            BorrowedValue::from_stored(&retained)
                .custom_field(0)
                .int()
                .bigint()
                .into_owned(),
            num_bigint::BigInt::from(42)
        );
    }

    #[test]
    fn pure_retention_cannot_relax_the_original_execution_of_a_callback() {
        let mut loaded = execution(
            r#"
@external(erlang, "native", "pure") fn pure(value: a) -> a
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
pub fn main() { pure(fn(value: Int) { value + 1 })(41) }
"#,
        );
        let mut state = State::default();
        assert_eq!(
            crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
        let error =
            crate::execution_fixture::run(&mut loaded, &mut state, &mut Vec::new()).unwrap_err();
        let error = expect_host_failure(error);
        assert_eq!(error.function(), "pure");
        assert_eq!(
            error.failure().message(),
            "retained value belongs to another execution"
        );
    }

    fn expect_host_failure(error: ExecutionError) -> Box<crate::HostError> {
        match error {
            ExecutionError::Host(error) => error,
            error => panic!("expected a checked host failure, got {error}"),
        }
    }

    #[test]
    #[should_panic(expected = "expected a checked host failure")]
    fn host_failure_assertion_rejects_a_source_panic() {
        let mut loaded = execution(
            r#"
@external(erlang, "native", "pure") fn pure(value: a) -> a
@external(erlang, "native", "scoped") fn scoped(value: a) -> a
pub fn main() -> Int { panic as "source failure" }
"#,
        );
        let error =
            crate::execution_fixture::run(&mut loaded, &mut State::default(), &mut Vec::new())
                .unwrap_err();
        expect_host_failure(error);
    }

    #[test]
    fn provider_value_preserves_its_owned_value_through_retention() {
        type Parameter = HostTypeParameter<0>;
        let value = Value::<Parameter, ProviderValueContext<Parameter>>::from_stored(
            StoredRuntimeValue::test_int(42.into()),
        );
        assert_eq!(
            BorrowedValue::from_stored(value.stored()).int(),
            &num_bigint::BigInt::from(42)
        );
        assert_eq!(
            BorrowedValue::from_stored(&value.into_stored()).int(),
            &num_bigint::BigInt::from(42)
        );
    }
}
