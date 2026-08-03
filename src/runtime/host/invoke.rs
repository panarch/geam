use super::RuntimeHostCall;
use crate::plan::execution::function::{ExecutionFunctionBody, FunctionBodyOwner};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::ExecutionError;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::{GraphValue, RetainedValues};
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn invoke_value<'run, Profile, Body>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
    origin: HostCallOrigin,
    target: &crate::plan::execution::host::HostFunctionId<Body>,
    inputs: RetainedValues,
) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated>
where
    Profile: crate::HostProfile,
    Body: ExecutionFunctionBody,
    Body::Return: GraphValue,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    let function = plan.host_value_function(target);
    let mut call = RuntimeHostCall::new(plan, state, function, inputs);
    match function.implementation().call(&mut call) {
        Ok(returned) => Ok(call.finish(returned, target.return_())),
        Err(error) => {
            drop(call);
            state.lists_mut().drain_releases();
            Err(host_call_error(plan, origin, function.metadata(), error))
        }
    }
}

pub(in crate::runtime) fn invoke_never<'run, Profile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
    origin: HostCallOrigin,
    target: crate::plan::execution::host::HostNeverFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<std::convert::Infallible>
where
    Profile: crate::HostProfile,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    let function = plan.host_never_function(target);
    let mut call = RuntimeHostCall::new(plan, state, function, inputs);
    match function.implementation().call(&mut call) {
        Ok(never) => match never {},
        Err(error) => {
            drop(call);
            state.lists_mut().drain_releases();
            Err(host_call_error(plan, origin, function.metadata(), error))
        }
    }
}

fn host_call_error<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    origin: HostCallOrigin,
    function: &crate::plan::execution::host::HostedFunctionMetadata,
    error: crate::host::HostCallError,
) -> ExecutionError {
    match error.into_kind() {
        crate::host::HostCallErrorKind::Nested(error) => error,
        crate::host::HostCallErrorKind::Failure(failure) => {
            match origin.into_source_site(function.site()) {
                Ok(site) => ExecutionError::from_host_call(
                    function,
                    site.clone(),
                    plan.source_context_for(site.module()),
                    failure,
                ),
                Err(caller) => ExecutionError::from_host_origin(function, caller, failure),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::host::{ExternalTestProfile, ExternalTestRunState};
    use crate::runtime::profile::external_test::{
        RuntimeCounterCallable, RuntimeCounterProvider, RuntimeCounterSchema, RuntimeHostCounter,
    };
    use crate::{
        HostCall, HostCallError, HostFailure, HostListType, HostModule, HostProviderModule,
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };
    use ecow::EcoString;
    use std::convert::Infallible;

    #[test]
    fn hosted_runtime_routes_diverging_external_return_families() {
        fn stop_counter<'call>(
            _call: HostCall<'call, ExternalTestProfile, RuntimeCounterProvider, RuntimeHostCounter>,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("counter stopped").into())
        }

        fn stop_list<'call>(
            _call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                HostListType<RuntimeHostCounter>,
            >,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("list stopped").into())
        }

        fn stop_function<'call>(
            _call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeCounterCallable,
            >,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("function stopped").into())
        }

        enum ReturnFamily {
            Value,
            List,
            Function,
        }

        let cases = [
            (
                ReturnFamily::Value,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_counter")
fn stop_counter() -> Counter

fn forward() -> Counter {
  stop_counter()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_counter failed: counter stopped",
            ),
            (
                ReturnFamily::List,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_list")
fn stop_list() -> List(Counter)

fn forward() -> List(Counter) {
  stop_list()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_list failed: list stopped",
            ),
            (
                ReturnFamily::Function,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_function")
fn stop_function() -> fn(Int) -> Counter

fn forward() -> fn(Int) -> Counter {
  stop_function()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_function failed: function stopped",
            ),
        ];

        for (family, source, expected_error) in cases {
            let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid");
            let provider = match family {
                ReturnFamily::Value => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        RuntimeHostCounter,
                        _,
                    >(
                        "stop_counter",
                        stop_counter,
                    )
                    .expect("external value target should be valid"),
                ReturnFamily::List => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        HostListType<RuntimeHostCounter>,
                        _,
                    >("stop_list", stop_list)
                    .expect("external list target should be valid"),
                ReturnFamily::Function => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        RuntimeCounterCallable,
                        _,
                    >(
                        "stop_function",
                        stop_function,
                    )
                    .expect("external function target should be valid"),
            };
            let typed = compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new("main", "src/main.gleam", source)],
                )],
                HostProviderSet::with_providers(
                    Vec::<HostModule<ExternalTestProfile>>::new(),
                    [provider],
                )
                .expect("provider module should be unique"),
            )
            .expect("diverging external source should compile");
            let plan = plan_host_program(typed).expect("diverging external source should plan");
            let execution = HostedExecution::try_from_module_plan(plan)
                .expect("diverging external execution should seal");
            let error = execution
                .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
                .expect_err("diverging external target should fail");

            assert_eq!(error.to_string(), expected_error);
        }
    }
}
