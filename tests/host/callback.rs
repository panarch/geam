use geam::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable, HostFailure,
    HostFunctionType, HostLocation, HostModule, HostProfile, HostProvider, HostProviderModule,
    HostProviderSet, HostSpecializationErrorReason, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostValue, HostedExecution, ModuleSource, PackageSource, PanicKind,
    StatelessHostProfile, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::convert::Infallible;

#[path = "callback/family.rs"]
mod family;
#[path = "callback/sealing.rs"]
mod sealing;

type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
type IntCallable = HostFunctionType<IntArguments, BigInt>;
type GenericArgument = HostTypeParameter<0>;
type GenericArguments = HostTypeList<GenericArgument, HostTypeListEnd>;
type GenericCallable = HostFunctionType<GenericArguments, BigInt>;

struct StatelessProvider;

impl HostProvider<StatelessHostProfile> for StatelessProvider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

fn apply<'call>(
    mut call: HostCall<'call, StatelessHostProfile, StatelessProvider, BigInt>,
    function: HostCallable<'call, IntArguments, BigInt>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let returned = call.invoke(function, (value, ()))?;
    Ok(call.return_value(returned))
}

fn forward<'call>(
    call: HostCall<'call, StatelessHostProfile, StatelessProvider, IntCallable>,
    function: HostCallable<'call, IntArguments, BigInt>,
) -> Result<HostCallCompletion<'call, IntCallable>, HostCallError> {
    Ok(call.return_value(function))
}

fn accept_generic_callable<'call>(
    call: HostCall<'call, StatelessHostProfile, StatelessProvider, BigInt>,
    _function: HostCallable<'call, GenericArguments, BigInt>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    Ok(call.return_value(1.into()))
}

fn preserve_generic_value<'call>(
    call: HostCall<'call, StatelessHostProfile, StatelessProvider, GenericArgument>,
    value: HostValue<'call, GenericArgument>,
) -> Result<HostCallCompletion<'call, GenericArgument>, HostCallError> {
    if !call.equal::<GenericArgument>(value, value) {
        return Err(HostFailure::new("opaque function identity changed").into());
    }
    Ok(call.return_value(value))
}

fn stop_with_generic_callable<'call>(
    _call: HostCall<'call, StatelessHostProfile, StatelessProvider, BigInt>,
    _function: HostCallable<'call, GenericArguments, BigInt>,
) -> Result<Infallible, HostCallError> {
    Err(HostFailure::new("symbolic callback should not enter runtime").into())
}

#[test]
fn invokes_and_returns_typed_gleam_callables() {
    let host = HostModule::new("host_support", "host/function")
        .expect("host module should be valid")
        .with_scoped_function::<StatelessProvider, (IntCallable, BigInt), BigInt, _>("apply", apply)
        .expect("callback application should register")
        .with_scoped_function::<StatelessProvider, (IntCallable,), IntCallable, _>(
            "forward", forward,
        )
        .expect("callback pass-through should register");
    let source = r#"
import host/function

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  let forwarded = function.forward(increment)
  #(
    function.apply(increment, 41),
    forwarded(41),
    forwarded == increment,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("host callback source should compile");
    let plan = plan_host_program(typed).expect("host callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("host callback execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Int(42.into()),
            Value::Int(42.into()),
            Value::Bool(true),
        ])),
    );
}

#[test]
fn rejects_only_a_reachable_callback_with_uninhabited_arguments() {
    let host = HostModule::new("host_support", "host/function")
        .expect("host module should be valid")
        .with_scoped_function::<StatelessProvider, (GenericCallable,), BigInt, _>(
            "accept",
            accept_generic_callable,
        )
        .expect("generic callback should register");
    let source = r#"
import host/function

fn generic(_value) {
  1
}

pub fn main() {
  function.accept(generic)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("symbolic callback source should compile");
    let plan = plan_host_program(typed).expect("symbolic callback source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("an invocable symbolic callback should not enter runtime");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/function");
    assert_eq!(error.function(), "accept");
    let HostSpecializationErrorReason::UninhabitedCallbackArguments { callback } = error.reason()
    else {
        panic!("the callback argument should own the sealing reason");
    };
    assert_eq!(callback.argument_types().len(), 1);
    assert!(matches!(
        callback.argument_types()[0],
        geam::ValueType::Parameter(_)
    ));
    assert_eq!(callback.return_(), &geam::ValueType::Int);
}

#[test]
fn seals_callback_arguments_before_a_diverging_host_target() {
    let host = HostModule::new("host_support", "host/function")
        .expect("host module should be valid")
        .with_scoped_diverging_function::<StatelessProvider, (GenericCallable,), BigInt, _>(
            "stop",
            stop_with_generic_callable,
        )
        .expect("diverging generic callback should register");
    let source = r#"
import host/function

fn generic(_value) {
  1
}

pub fn main() {
  function.stop(generic)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("diverging symbolic callback source should compile");
    let plan = plan_host_program(typed).expect("diverging symbolic callback source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("a diverging host must not receive an invocable symbolic callback");
    };

    assert_eq!(error.function(), "stop");
    assert!(matches!(
        error.reason(),
        HostSpecializationErrorReason::UninhabitedCallbackArguments { .. }
    ));
}

#[test]
fn passes_a_symbolic_function_through_an_opaque_generic_host_value() {
    let host = HostModule::new("host_support", "host/function")
        .expect("host module should be valid")
        .with_scoped_function::<StatelessProvider, (GenericArgument,), GenericArgument, _>(
            "preserve",
            preserve_generic_value,
        )
        .expect("opaque generic value should register");
    let source = r#"
import host/function

fn generic(value) {
  value
}

pub fn main() {
  function.preserve(generic)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("opaque symbolic value source should compile");
    let plan = plan_host_program(typed).expect("opaque symbolic value source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("opaque symbolic values should not grant callback invocation");

    assert_eq!(
        execution
            .run_main(&mut (), &mut Vec::new())
            .expect("opaque symbolic value should round-trip")
            .inspect()
            .to_string(),
        "//fn(a) { ... }",
    );
}

struct CallbackProfile;

#[derive(Debug, Default, PartialEq, Eq)]
struct CallbackState {
    outer_calls: usize,
    inner_calls: usize,
    stops: usize,
}

struct OuterProvider;
struct InnerProvider;
struct StopProvider;

impl HostProfile for CallbackProfile {
    type RunState = CallbackState;
}

impl HostProvider<CallbackProfile> for OuterProvider {
    type State = usize;

    fn project(state: &mut CallbackState) -> &mut Self::State {
        &mut state.outer_calls
    }
}

impl HostProvider<CallbackProfile> for InnerProvider {
    type State = usize;

    fn project(state: &mut CallbackState) -> &mut Self::State {
        &mut state.inner_calls
    }
}

impl HostProvider<CallbackProfile> for StopProvider {
    type State = usize;

    fn project(state: &mut CallbackState) -> &mut Self::State {
        &mut state.stops
    }
}

fn apply_with_state<'call>(
    mut call: HostCall<'call, CallbackProfile, OuterProvider, BigInt>,
    function: HostCallable<'call, IntArguments, BigInt>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    *call.state() += 1;
    let returned = call.invoke(function, (value, ()))?;
    Ok(call.return_value(returned))
}

fn increment_with_state<'call>(
    mut call: HostCall<'call, CallbackProfile, InnerProvider, BigInt>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    *call.state() += 1;
    Ok(call.return_value(value + 1))
}

#[test]
fn reenters_gleam_and_a_nested_host_with_independent_run_state() {
    let outer = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<OuterProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            apply_with_state,
        )
        .expect("outer callback should register");
    let inner = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/inner")
        .expect("inner host module should be valid")
        .with_scoped_function::<InnerProvider, (BigInt,), BigInt, _>(
            "increment",
            increment_with_state,
        )
        .expect("inner callback should register");
    let source = r#"
import host/inner
import host/outer

fn bridge(value: Int) {
  echo value as "bridge"
  inner.increment(value)
}

pub fn main() {
  echo 0 as "before"
  let returned = outer.apply(bridge, 41)
  echo returned as "after"
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer, inner]).expect("host modules should be unique"),
    )
    .expect("nested host source should compile");
    let plan = plan_host_program(typed).expect("nested host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("nested host execution should seal");
    let mut first_state = CallbackState::default();
    let mut first_echo = Vec::new();
    let mut second_state = CallbackState::default();
    let mut second_echo = Vec::new();

    assert_eq!(
        execution.run_main(&mut first_state, &mut first_echo),
        Ok(Value::Int(42.into())),
    );
    assert_eq!(
        execution.run_main(&mut first_state, &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
    assert_eq!(
        execution.run_main(&mut second_state, &mut second_echo),
        Ok(Value::Int(42.into())),
    );
    assert_eq!(
        first_state,
        CallbackState {
            outer_calls: 2,
            inner_calls: 2,
            stops: 0,
        },
    );
    assert_eq!(
        second_state,
        CallbackState {
            outer_calls: 1,
            inner_calls: 1,
            stops: 0,
        },
    );
    assert_eq!(first_echo.len(), 3);
    assert_eq!(
        first_echo[0].message().map(|message| message.as_str()),
        Some("before"),
    );
    assert_eq!(first_echo[0].value(), &Value::Int(0.into()));
    assert_eq!(
        first_echo[1].message().map(|message| message.as_str()),
        Some("bridge"),
    );
    assert_eq!(first_echo[1].value(), &Value::Int(41.into()));
    assert_eq!(
        first_echo[2].message().map(|message| message.as_str()),
        Some("after"),
    );
    assert_eq!(first_echo[2].value(), &Value::Int(42.into()));
    assert_eq!(second_echo.len(), 3);
    assert_eq!(
        second_echo
            .iter()
            .map(|output| output.message().map(|message| message.as_str()))
            .collect::<Vec<_>>(),
        [Some("before"), Some("bridge"), Some("after")],
    );
    assert_eq!(second_echo[0].value(), &Value::Int(0.into()));
    assert_eq!(second_echo[1].value(), &Value::Int(41.into()));
    assert_eq!(second_echo[2].value(), &Value::Int(42.into()));
}

fn fail(_: BigInt) -> Result<BigInt, HostFailure> {
    Err(HostFailure::new("inner unavailable"))
}

fn stop_after_callback<'call>(
    mut call: HostCall<'call, CallbackProfile, OuterProvider, BigInt>,
    function: HostCallable<'call, IntArguments, BigInt>,
    value: BigInt,
) -> Result<Infallible, HostCallError> {
    *call.state() += 1;
    let _ = call.invoke(function, (value, ()))?;
    Err(HostFailure::new("callback unexpectedly returned").into())
}

#[test]
fn preserves_the_failed_nested_host_and_its_host_caller_origin() {
    let outer = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<OuterProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            apply_with_state,
        )
        .expect("outer callback should register");
    let inner = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/inner")
        .expect("inner host module should be valid")
        .with_fallible_function("fail", fail)
        .expect("inner failure should register");
    let source = r#"
import host/inner
import host/outer

pub fn main() {
  outer.apply(inner.fail, 1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer, inner]).expect("host modules should be unique"),
    )
    .expect("nested host failure source should compile");
    let plan = plan_host_program(typed).expect("nested host failure source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("nested host failure execution should seal");
    let error = execution
        .run_main(&mut CallbackState::default(), &mut Vec::new())
        .expect_err("inner host should fail");
    let ExecutionError::Host(error) = error else {
        panic!("nested host failure should remain a host error");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/inner");
    assert_eq!(error.function(), "fail");
    assert_eq!(error.failure().message(), "inner unavailable");
    let HostLocation::Host { caller } = error.location() else {
        panic!("direct host re-entry should preserve its host caller");
    };
    assert_eq!(caller.package(), "host_support");
    assert_eq!(caller.module(), "host/outer");
    assert_eq!(caller.function(), "apply");
}

#[test]
fn preserves_a_nested_host_failure_from_a_diverging_outer_host() {
    let outer = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_diverging_function::<OuterProvider, (IntCallable, BigInt), BigInt, _>(
            "stop",
            stop_after_callback,
        )
        .expect("diverging outer callback should register");
    let inner = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/inner")
        .expect("inner host module should be valid")
        .with_fallible_function("fail", fail)
        .expect("inner failure should register");
    let source = r#"
import host/inner
import host/outer

pub fn main() {
  outer.stop(inner.fail, 1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer, inner]).expect("host modules should be unique"),
    )
    .expect("diverging nested host failure source should compile");
    let plan = plan_host_program(typed).expect("diverging nested host failure source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("diverging nested host failure execution should seal");
    let mut state = CallbackState::default();
    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("inner host should fail before the outer host can return");
    let ExecutionError::Host(error) = error else {
        panic!("nested host failure should remain a host error");
    };

    assert_eq!(state.outer_calls, 1);
    assert_eq!(error.module(), "host/inner");
    assert_eq!(error.function(), "fail");
    let HostLocation::Host { caller } = error.location() else {
        panic!("diverging host re-entry should preserve its host caller");
    };
    assert_eq!(caller.module(), "host/outer");
    assert_eq!(caller.function(), "stop");
}

#[test]
fn preserves_a_nested_source_panic_without_host_rewrapping() {
    let outer = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<OuterProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            apply_with_state,
        )
        .expect("outer callback should register");
    let source = r#"
import host/outer

fn stop(_value: Int) -> Int {
  panic as "nested source"
}

pub fn main() {
  outer.apply(stop, 1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer]).expect("host module should be unique"),
    )
    .expect("nested panic source should compile");
    let plan = plan_host_program(typed).expect("nested panic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("nested panic execution should seal");
    let error = execution
        .run_main(&mut CallbackState::default(), &mut Vec::new())
        .expect_err("nested source should panic");
    let ExecutionError::Panic(panic) = error else {
        panic!("nested source panic should not become a host error");
    };

    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(panic.site().module(), "main");
    assert_eq!(panic.site().function(), "stop");
}

fn stop_with_state<'call>(
    mut call: HostCall<'call, CallbackProfile, StopProvider, BigInt>,
    _value: BigInt,
) -> Result<Infallible, HostCallError> {
    *call.state() += 1;
    Err(HostFailure::new("stopped by host").into())
}

#[test]
fn runs_a_stateful_scoped_diverging_provider_with_the_source_return_marker() {
    let host = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/control")
        .expect("host module should be valid")
        .with_scoped_diverging_function::<StopProvider, (BigInt,), BigInt, _>(
            "stop",
            stop_with_state,
        )
        .expect("diverging callback should register");
    let source = r#"
import host/control

pub fn main() {
  control.stop(1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("diverging host source should compile");
    let plan = plan_host_program(typed).expect("diverging host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("diverging execution should seal");
    let mut state = CallbackState::default();
    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("diverging host should fail");
    let ExecutionError::Host(error) = error else {
        panic!("diverging host failure should remain a host error");
    };

    assert_eq!(state.stops, 1);
    assert_eq!(error.module(), "host/control");
    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "stopped by host");
    assert_eq!(error.signature().return_(), &geam::ValueType::Int);
}

#[test]
fn preserves_a_source_provider_failure_and_its_host_caller_origin() {
    let outer = HostModule::<CallbackProfile>::new_for_profile("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<OuterProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            apply_with_state,
        )
        .expect("outer callback should register");
    let stop = HostProviderModule::<CallbackProfile>::new("application", "main")
        .expect("source provider module should be valid")
        .with_scoped_diverging_function::<StopProvider, (BigInt,), BigInt, _>(
            "stop",
            stop_with_state,
        )
        .expect("source provider should register");
    let source = r#"
import host/outer

@external(erlang, "host", "stop")
fn stop(value: Int) -> Int

pub fn main() {
  outer.apply(stop, 1)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers([outer], [stop])
            .expect("host module and source provider should be unique"),
    )
    .expect("source provider callback should compile");
    let plan = plan_host_program(typed).expect("source provider callback should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("source provider callback should seal");
    let mut state = CallbackState::default();
    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("source provider should fail");
    let ExecutionError::Host(error) = error else {
        panic!("source provider failure should remain a host error");
    };

    assert_eq!(
        state,
        CallbackState {
            outer_calls: 1,
            inner_calls: 0,
            stops: 1,
        },
    );
    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "stopped by host");
    let HostLocation::Host { caller } = error.location() else {
        panic!("source provider re-entry should preserve its host caller");
    };
    assert_eq!(caller.package(), "host_support");
    assert_eq!(caller.module(), "host/outer");
    assert_eq!(caller.function(), "apply");
}
