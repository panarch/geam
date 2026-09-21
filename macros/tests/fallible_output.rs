use geam_core::execution::{RunError, TokioHost};
use geam_core::{
    ExecutionError, HostComponentProfile, HostProfile, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
    plan_host_program,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct State {
    effects: Arc<Mutex<Vec<u8>>>,
}

#[geam_macros::provider(package = "fallible_output", state = State, modules = [native], crate_path = geam_core)]
pub struct Component;

mod converted {
    use geam_core::provider::{
        ProviderNoConstructions, ProviderOutputValue, ProviderValue, ProviderValueForms,
    };
    use geam_core::{HostCall, HostCallError, HostFailure, HostProfile, HostProvider, HostType};
    use num_bigint::BigInt;
    use std::sync::{Arc, Mutex};

    pub struct Number {
        pub value: u8,
        pub reject: bool,
        pub effects: Arc<Mutex<Vec<u8>>>,
    }
    impl ProviderValue for Number {
        type Host = BigInt;
        type OutputRequirements = ProviderNoConstructions;
        type RootRequirements = ProviderNoConstructions;
    }
    impl ProviderValueForms for Number {
        type InvocationRequirements = ();
        type ImmediateListDecoder = geam_core::provider::MissingListContext;
        type OwnedListDecoder = geam_core::provider::MissingListContext;
        type Runtime<Profile: geam_core::HostProfile> =
            geam_core::provider::ProviderStaticValueForms<Self>;
        type Output = Self;
        type ImmediateInput = Self;
        type ImmediateListInput = Self;
        type OwnedInput = Self;
        type OwnedListInput = Self;
    }
    impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for Number
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        type Error = HostCallError;
        fn into_host<'call>(
            self,
            _: &mut HostCall<'call, Profile, Provider, Return>,
            _: &geam_core::provider::ProviderConstructions<'call, Self::OutputRequirements>,
        ) -> Result<BigInt, HostCallError> {
            self.effects.lock().unwrap().push(self.value);
            if self.reject {
                Err(HostFailure::new(format!("rejected {}", self.value)).into())
            } else {
                Ok(self.value.into())
            }
        }
    }
}

#[geam_macros::module(path = "fallible_output", crate_path = geam_core)]
mod native {
    use super::{State, converted};
    use geam_core::provider::{BigInt, Call};

    #[geam_macros::custom]
    enum Envelope {
        Value(converted::Number),
    }

    #[geam_macros::function]
    fn values(#[geam_macros::call] call: &Call<State>, fail_at: BigInt) -> Vec<Envelope> {
        (1_u8..=3)
            .map(|value| {
                Envelope::Value(converted::Number {
                    value,
                    reject: fail_at == BigInt::from(value),
                    effects: call.state().effects.clone(),
                })
            })
            .collect()
    }
}

struct Profile;
impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
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
fn nested_custom_output_failure_stops_conversion_and_preserves_the_host_error() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for fail_at in [0, 2] {
        let source = format!(
            r#"
pub type Envelope {{ Value(Int) }}
@external(erlang, "native", "values") fn values(fail_at: Int) -> List(Envelope)
pub fn main() {{ values({fail_at}) }}
"#
        );
        let typed = compile_typed_host_program(
            "fallible_output",
            "fallible_output",
            [PackageSource::new(
                "fallible_output",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "fallible_output",
                    "fallible_output.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers(
                <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut state = State::default();
        let result = runtime.block_on(execution.run_main(&host, &mut state, &mut Vec::new()));
        if fail_at == 0 {
            assert_eq!(
                result.unwrap().inspect().to_string(),
                "[Value(1), Value(2), Value(3)]"
            );
            assert_eq!(*state.effects.lock().unwrap(), [1, 2, 3]);
        } else {
            let Err(RunError::Execution(ExecutionError::Host(error))) = result else {
                panic!("the nested converter must preserve its host failure");
            };
            assert_eq!(error.function().as_str(), "values");
            assert_eq!(error.failure().message().as_str(), "rejected 2");
            assert_eq!(*state.effects.lock().unwrap(), [1, 2]);
        }
    }
}
