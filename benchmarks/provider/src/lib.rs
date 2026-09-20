use geam::provider::BigInt;
use std::time::Instant;

#[geam::provider(package = "geam_benchmarks", state = RunState, modules = [native])]
pub struct Component;

pub struct RunState {
    origin: Instant,
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl RunState {
    fn timestamp_at(&self, now: Instant) -> BigInt {
        now.duration_since(self.origin).as_nanos().into()
    }
}

#[geam::module(path = "geam_benchmarks/native")]
mod native {
    use super::{BigInt, Instant, RunState};
    use geam::provider::{Call, StringValue, Value};

    #[geam::function]
    fn environment(name: StringValue) -> Result<StringValue, StringValue> {
        std::env::var(name.as_str())
            .map(StringValue::from)
            .map_err(|error| error.to_string().into())
    }

    #[geam::function]
    fn monotonic_ns(#[geam::call] call: &Call<RunState>) -> BigInt {
        call.state().timestamp_at(Instant::now())
    }

    #[geam::function]
    fn consume<Item>(value: Value<Item>) -> Value<Item> {
        std::hint::black_box(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{BigInt, RunState};
    use std::time::{Duration, Instant};

    #[test]
    fn measures_elapsed_nanoseconds_from_the_provider_origin() {
        let origin = Instant::now();
        let state = RunState { origin };
        assert_eq!(state.timestamp_at(origin), BigInt::from(0));
        assert_eq!(
            state.timestamp_at(origin + Duration::from_nanos(123_456_789)),
            BigInt::from(123_456_789)
        );
    }
    #[test]
    fn provider_preserves_typed_values_and_exposes_clock_and_environment() {
        use super::Component;
        use geam::embedding::{FunctionDeclaration, HostedModuleBuilder, StringValue};
        use geam::{
            HostComponentProfile, HostProfile, HostProviderComponent,
            HostProviderComponentRegistration, HostProviderSet, ModuleSource, PackageSource,
            compile_typed_host_program,
        };

        struct Profile;
        impl HostProfile for Profile {
            type RunState = RunState;
            type ExternalStores = <Component as HostProviderComponent>::Stores;
            type ExecutionState = ();
        }
        impl HostComponentProfile<Component> for Profile {
            fn component_stores(
                stores: &Self::ExternalStores,
            ) -> &<Component as HostProviderComponent>::Stores {
                stores
            }
            fn component_state(state: &mut RunState) -> &mut RunState {
                state
            }
        }
        let native = r#"
@external(erlang, "geam_benchmarks_ffi", "environment")
pub fn environment(name: String) -> Result(String, String)
@external(erlang, "geam_benchmarks_ffi", "monotonic_ns")
pub fn monotonic_ns() -> Int
@external(erlang, "geam_benchmarks_ffi", "consume")
pub fn consume(value: a) -> a
"#;
        let source = r#"
import geam_benchmarks/native
pub type Entry { Entry(Int, String) }
pub fn main(present: String, absent: String) {
  let first = native.monotonic_ns()
  let assert 3 = native.consume(3)
  let assert 1.5 = native.consume(1.5)
  let assert True = native.consume(True)
  let assert Nil = native.consume(Nil)
  let assert "hello" = native.consume("hello")
  let assert <<1, 2, 3>> = native.consume(<<1, 2, 3>>)
  let values = [1, 2, 3]
  let assert True = native.consume(values) == values
  let assert #(3, "three") = native.consume(#(3, "three"))
  let assert Entry(4, "four") = native.consume(Entry(4, "four"))
  let assert Ok(7) = native.consume(Ok(7))
  let next = native.consume(fn(value) { value + 1 })
  let assert 10 = next(9)
  #(native.environment(present), native.environment(absent), native.monotonic_ns() >= first)
}
"#;
        let stores = <Component as HostProviderComponent>::Stores::default();
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<Component>>::component_stores(&stores),
            &stores,
        ));
        let providers = HostProviderSet::from_providers(
            <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap(),
        )
        .unwrap();
        let program = compile_typed_host_program(
            "geam_benchmarks",
            "main",
            [PackageSource::new(
                "geam_benchmarks",
                Vec::<&str>::new(),
                [
                    ModuleSource::new("geam_benchmarks/native", "native.gleam", native),
                    ModuleSource::new("main", "main.gleam", source),
                ],
            )],
            providers,
        )
        .unwrap();
        type Environment = Result<StringValue, StringValue>;
        let (bindings, main) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<
                (StringValue, StringValue),
                (Environment, Environment, bool),
            >::new("main"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let executor = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = geam::execution::TokioHost::new(executor.handle().clone());
        let mut state = RunState::default();
        let mut echo = Vec::new();
        let missing = format!("GEAM_BENCH_ABSENT_{}", std::process::id());
        assert!(std::env::var_os(&missing).is_none());
        let result = executor
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    scope
                        .call(
                            &main,
                            (StringValue::from("PATH"), StringValue::from(missing)),
                        )
                        .await
                }),
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            result,
            (
                Ok(StringValue::from(std::env::var("PATH").unwrap())),
                Err(StringValue::from("environment variable not found")),
                true
            )
        );
        assert!(echo.is_empty());
    }
}
